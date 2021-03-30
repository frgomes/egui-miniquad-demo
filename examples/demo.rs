#[cfg(any(target_os="linux", target_os="macos", target_os="windows", target_os = "android", target_arch="wasm32"))]
fn main() {
    egui_miniquad_demo::worker::run();
}
