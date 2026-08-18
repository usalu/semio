//! 🚪️ Binary entry point for `semio-framework-plugin-describe`; all logic lives in the
//! `semio_framework_plugin_describe` godfile (`📦️glue.rs`).
fn main() {
    std::process::exit(semio_framework_plugin_describe::run(std::env::args().skip(1).collect()));
}
