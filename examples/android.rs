#[cfg_attr(target_os = "android", ndk_glue::main(ndk_glue = "::miniquad::sapp_android"))]
fn main() {
    egui_miniquad_demo::worker::run();
}
