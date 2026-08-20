//! 🚪️ Binary entry point for `semio-framework-plugin-describe`; all logic lives in the
//! `semio_framework_plugin_describe` godfile (`📦️glue.rs`).
// 🚫️async: E3 `fn main` + R4 allow-list clause 1 — a binary's entry point IS the executor bridge.
// `run` became `async fn` under the universal-async decree; this is the one place in the crate that
// drives it to completion.
fn main() {
    std::process::exit(semio_framework_async::block_on(semio_framework_plugin_describe::run(std::env::args().skip(1).collect())));
}
