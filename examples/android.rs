use android_activity::{AndroidApp, MainEvent, PollEvent};
use std::time::Duration; // Needed for the poll_events timeout

use tracing_android::layer;
use tracing_log::LogTracer;
use tracing::{Level, info, warn, error, debug, trace};
use tracing_subscriber::{registry, prelude::*};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};


// This is the entry point function called by the android-activity glue layer.
// #[no_mangle] is essential to ensure the function name is not changed by the compiler,
// as the Android system/glue layer expects to find a function with this exact name.
#[cfg(any(target_os = "android"))]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    LogTracer::builder().init().expect("Failed to set log tracer");
    log::info!("Log tracer initialized by tracing-log."); // This message goes through the new tracer
    let subscriber = registry::Registry::default()
        .with(EnvFilter::from_default_env()
              .add_directive(LevelFilter::INFO.into()));
    tracing::info!("Android application started.");

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
                        MainEvent::TerminateWindow { .. } => {
                            tracing::info!("MainEvent::TerminateWindow - Application is terminating.");
                            // Clean up resources here.
                            app_is_running = false; // Exit the main loop
                        }
                        MainEvent::Resume { .. } => {
                            tracing::info!("MainEvent::Resume - Application is resuming.");
                            // Resume game loop, audio, etc.
                        }
                        MainEvent::Pause { .. } => {
                            tracing::info!("MainEvent::Pause - Application is pausing.");
                            // Pause game loop, audio, save state, etc.
                        }
                        MainEvent::InputAvailable { .. } => {
                            tracing::info!("MainEvent::InputAvailable - Received input event.");
                            // Input events like touch, key presses are delivered here.
                            // These would typically be processed by your input handling system
                            // (e.g., within miniquad's event handling if it's integrated).
                        }
                        MainEvent::Destroy { .. } => {
                            tracing::info!("MainEvent::Destroy - Window surface is being destroyed.");
                            // Clean up graphics resources associated with the window.
                            activity_has_window = false;
                        }
                        MainEvent::SaveState { .. } => {
                            tracing::info!("MainEvent::SaveState - Save application state.");
                            // Save any transient state that needs to be restored later.
                        }
                        MainEvent::ConfigChanged { .. } => {
                            tracing::info!("MainEvent::ConfigChanged - Configuration changed (e.g., orientation).");
                            // React to configuration changes if needed.
                        }
                        MainEvent::LowMemory { .. } => {
                            warn!("MainEvent::LowMemory - System is low on memory.");
                             // Free up some memory if possible.
                        }
                        MainEvent::WindowResized { .. } => {
                            tracing::info!("MainEvent::WindowResized.");
                        }
                        MainEvent::RedrawNeeded { .. } => {
                            tracing::info!("MainEvent::RedrawNeeded.");
                        }
                        MainEvent::ContentRectChanged { .. } => {
                            tracing::info!("MainEvent::ContentRectChanged.");
                        }
                        MainEvent::GainedFocus { .. } => {
                            tracing::info!("MainEvent::GainedFocus.");
                        }
                        MainEvent::LostFocus { .. } => {
                            tracing::info!("MainEvent::LostFocus.");
                        }
                        MainEvent::Start { .. } => {
                            tracing::info!("MainEvent::Start - Start application.");
                        }
                        MainEvent::Stop { .. } => {
                            tracing::info!("MainEvent::Stop - Stop application.");
                        }
                        MainEvent::InsetsChanged { .. } => {
                            tracing::info!("MainEvent::InsetsChanged.");
                        }
                        _ => {
                            // Handle other less common events if necessary
                            tracing::info!("Other MainEvent:{:?}", main_event);
                        }
                    }
                }
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
            //XXX quad_main()
        }
    }

    tracing::info!("Exiting android_main - Application stopped.");
}




#[unsafe(no_mangle)]
pub fn quad_main() {
    //XXX egui_miniquad_demo::worker::start();
}
