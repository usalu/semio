
use super::*;
use crate::editor::puzzle5d::testkit::*;

#[test]
fn catalogue_tree_lists_all_four_kind_sections() {
    let mut app = app();
    let rendered = render_body(&mut app, BODY_KEY);
    for section in ["puzzle5d-play-kinds.parts", "puzzle5d-play-kinds.grips", "puzzle5d-play-kinds.fasteners", "puzzle5d-play-kinds.ropes"] {
        assert!(rendered.contains(section), "catalogue must carry {section}");
    }
}
