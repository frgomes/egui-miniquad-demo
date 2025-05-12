#[cfg(any(target_os="linux", target_os="macos", target_os="windows", target_arch="wasm32"))]
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Log to stdout (if you run with `RUST_LOG=debug`).
        tracing_subscriber::fmt::init();
    }

    egui_miniquad_demo::worker::start();
}
