//! ️tests for app example `🎬️demo-session`.

#[test]
fn cmd_asset_is_nonempty_demo_script() {
    let text = include_str!("../🖼️assets/🎮️.cmd.semio");
    assert!(text.len() > 64, "cmd fixture must carry real payload");
    assert!(text.contains("setActiveExample"), "demo session must drive an example load");
}
