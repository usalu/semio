
use super::*;
use crate::editor::puzzle5d::testkit::*;

#[test]
fn renders_the_world_scene() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("world-3d"));
}
