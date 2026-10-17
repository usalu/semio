use crate::standards::v1_1::subsets::tiny::schema::snapshot::SvgSnapshot;
#[allow(unused_imports)]
use crate::standards::v1_1::subsets::base::io::text::snapshot::write_svg_xml;
#[allow(unused_imports)]
use semio_framework_plugin::{app::ImageView, app::ImageWindowKit};
#[allow(unused_imports)]
use semio_framework_plugin::{BuiltNode, WindowKindDefinition, WindowKit};
use crate::viewer::svg_tiny::modes::view::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = SvgSnapshot::default();
    let _node = render(&document);
}
