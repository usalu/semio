//! ️tests for app example `🎬️demo-session`.

#[semio_framework_async_macros::async_test]
async fn cmd_asset_is_nonempty_demo_script() {
    let text = include_str!("../🖼️assets/🎮️demo.cmd.semio");
    assert!(text.len() > 64, "cmd fixture must carry real payload");
    assert!(text.contains("setActiveExample"), "demo session must drive an example load");
}
