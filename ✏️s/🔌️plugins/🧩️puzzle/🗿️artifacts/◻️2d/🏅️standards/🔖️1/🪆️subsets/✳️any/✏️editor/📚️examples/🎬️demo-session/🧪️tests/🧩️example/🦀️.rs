//! 🧪️tests for app example `🎬️demo-session`.

use semio_framework_test_fixtures::fixture_text;

#[test]
fn cmd_asset_is_nonempty_demo_script() {
    let text = fixture_text!("../../../../✏️editor/📚️examples/🎬️demo-session/🖼️assets/🎮️.cmd.semio");
    assert!(text.len() > 64, "cmd snapshot must carry real payload");
    assert!(text.contains("setActiveExample"), "demo session must drive an example load");
}
