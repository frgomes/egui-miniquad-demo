use android_activity::{AndroidApp, MainEvent, PollEvent};
use std::time::Duration; // Needed for the poll_events timeout

use log::{info, warn, error, LevelFilter};
use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, filter, registry};
use tracing_logcat::{LogcatMakeWriter, LogcatTag};

// This is the entry point function called by the android-activity glue layer.
// #[no_mangle] is essential to ensure the function name is not changed by the compiler,
// as the Android system/glue layer expects to find a function with this exact name.
#[cfg(any(target_os = "android"))]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    // Initialize the tracing-log tracer *first*.
    // This captures any `log!` messages emitted by dependencies.
    // This effectively replaces `android_logger::init_once`.
    // The `tracing-log` crate will set the global `log` logger.

    use tracing::instrument::WithSubscriber;
    LogTracer::builder().init().expect("Failed to set log tracer");
    log::info!("Log tracer initialized."); // This message will be captured by tracing-log

    // 1. Create a writer that directs output to Android's logcat.
    // LogcatTag::Target uses the tracing target as the logcat tag.
    // LogcatTag::Fixed("MyAppTag".to_string()) would use a fixed tag.
    let logcat_writer = LogcatMakeWriter::new(LogcatTag::Target)
        .expect("Failed to create logcat writer");

    // 2. Build the tracing subscriber using the fmt (formatting) module.
    let subscriber = fmt::Subscriber::builder()
        // Set the writer for the formatter to our logcat writer.
        .with_writer(logcat_writer)
        // Optional: Set a maximum level. Events below this level will be discarded early.
        // Level::INFO, Level::DEBUG, Level::TRACE are common during development.
        .with_max_level(Level::INFO)
        // Optional: Disable ANSI colors, as logcat doesn't typically support them well.
        .with_ansi(false)
        // Optional: Customize the event format.
        // logcat adds timestamps, so `without_time()` is often desired.
        // You might want to include the target and level.
        .event_format(fmt::format().with_target(true).with_level(true).compact())
        // Finish building the subscriber.
        .finish();

    // 3. Install the subscriber as the global default.
    // `try_init` returns a Result, which is safer than `init` if there's a chance
    // a subscriber might already be installed (e.g., in tests), but `init` panics
    // if it fails, which might be desired in a main application entry point.
    // Use `init().expect("Failed to set global tracing subscriber");` if panicking is okay.
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");

    tracing::info!("Tracing subscriber initialized successfully!");
    tracing::info!("Entering android_main - Application started.");

    // In a real application (like a game), you would typically set up your
    // graphics context (e.g., miniquad) here or after certain events like InitWindow.
    // Your main game loop would likely run within or alongside the event loop below.

    let mut app_is_running = true;
    let mut activity_has_window = false; // Track if the window surface is ready

    // This is the main event loop. The AndroidApp polls for events from the OS.
    while app_is_running {
        // Poll for events. The timeout determines how long to wait if no events are pending.
        // - Some(Duration::from_millis(0)) for non-blocking polling (useful if you render every frame regardless of events)
        // - Some(...) for polling with a timeout
        // - None for blocking until an event occurs (useful for event-driven apps, less common for games)
        // Using a short timeout or 0 keeps the loop active for rendering.
        let poll_timeout = if activity_has_window {
            // If the window is ready, poll quickly to render frames
            Some(Duration::from_millis(0))
        } else {
            // If no window, wait for a bit longer for InitWindow event
             Some(Duration::from_millis(100))
        };

        app.poll_events(poll_timeout, |event| {
            // This closure is called for each event received.
            match event {
                PollEvent::Wake => {
                    // Sent to wake up the event loop, usually when an event has been queued.
                    // info!("Event loop woken up.");
                }
                PollEvent::Timeout => {
                    // Sent when the poll_events call times out without receiving other events.
                    // info!("Poll timeout.");
                }
                PollEvent::Main(main_event) => {
                    // These are the main lifecycle and input events.
                    match main_event {
                        MainEvent::InitWindow { .. } => {
                            tracing::info!("MainEvent::InitWindow - Window surface created or changed.");
                            // The graphics context (like miniquad) should be initialized here,
                            // as the rendering surface is now available.
                            activity_has_window = true;
                            // In a miniquad app, miniquad::start might be called here or just after the loop.
                            // Often, you call miniquad::start unconditionally after the loop setup,
                            // and its backend handles the window availability based on this event.
                        }
                        //FIXME MainEvent::Terminate => {
                        //FIXME     tracing::info!("MainEvent::Terminate - Application is terminating.");
                        //FIXME     // Clean up resources here.
                        //FIXME     app_is_running = false; // Exit the main loop
                        //FIXME }
                        //FIXME MainEvent::Resume => {
                        //FIXME     tracing::info!("MainEvent::Resume - Application is resuming.");
                        //FIXME     // Resume game loop, audio, etc.
                        //FIXME }
                        //FIXME MainEvent::Pause => {
                        //FIXME     tracing::info!("MainEvent::Pause - Application is pausing.");
                        //FIXME     // Pause game loop, audio, save state, etc.
                        //FIXME }
                        //FIXME MainEvent::Input { .. } => {
                        //FIXME     // info!("MainEvent::Input - Received input event.");
                        //FIXME     // Input events like touch, key presses are delivered here.
                        //FIXME     // These would typically be processed by your input handling system
                        //FIXME     // (e.g., within miniquad's event handling if it's integrated).
                        //FIXME }
                        //FIXME MainEvent::DestroyWindow => {
                        //FIXME     tracing::info!("MainEvent::DestroyWindow - Window surface is being destroyed.");
                        //FIXME     // Clean up graphics resources associated with the window.
                        //FIXME     activity_has_window = false;
                        //FIXME }
                        //FIXME MainEvent::SaveInstanceState => {
                        //FIXME     tracing::info!("MainEvent::SaveInstanceState - Save application state.");
                        //FIXME     // Save any transient state that needs to be restored later.
                        //FIXME }
                        //FIXME MainEvent::ReceiveConfig { .. } => {
                        //FIXME     tracing::info!("MainEvent::ReceiveConfig - Configuration changed (e.g., orientation).");
                        //FIXME     // React to configuration changes if needed.
                        //FIXME }
                        //FIXME MainEvent::LowMemory => {
                        //FIXME     warn!("MainEvent::LowMemory - System is low on memory.");
                        //FIXME      // Free up some memory if possible.
                        //FIXME }
                        _ => {
                             // Handle other less common events if necessary
                             tracing::info!("Other MainEvent:{:?}", main_event);
                        }
                    }
                }
                //FIXME // PollEvent can have other variants depending on features, e.g., NativeEvent
                //FIXME PollEvent::NativeEvent { .. } => {
                //FIXME     tracing::info!("Received native event");
                //FIXME }
                _ => {
                    // Handle other less common events if necessary
                    tracing::info!("Other event: {:?}", event);
                }
            }
        });

        // --- Render Frame ---
        // If the window surface is ready, render a frame.
        // In a miniquad app, this is where you would call your rendering logic,
        // usually managed by miniquad's internal loop which is started by miniquad::start.
        // The miniquad::start function typically runs its *own* loop internally
        // and handles drawing and event polling based on the Android events received
        // by the glue layer.
        // So, in a typical miniquad setup, the loop above might primarily manage
        // lifecycle events and call `miniquad::start` once the window is ready,
        // and then `miniquad::start` takes over the main frame loop.

        // Example (Conceptual - not actual miniquad drawing loop):
        if activity_has_window {
            // Perform rendering using miniquad
            // This is usually done by miniquad's internal loop after miniquad::start is called
            quad_main()
        }
    }

    tracing::info!("Exiting android_main - Application stopped.");
}




#[unsafe(no_mangle)]
pub fn quad_main() {
    egui_miniquad_demo::worker::start();
}
