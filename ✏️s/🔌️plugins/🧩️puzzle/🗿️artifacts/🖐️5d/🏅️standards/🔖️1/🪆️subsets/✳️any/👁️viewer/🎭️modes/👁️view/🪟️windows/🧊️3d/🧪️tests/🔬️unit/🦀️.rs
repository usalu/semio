use crate::Puzzle5dSnapshot;
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dPart;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::document_records::Puzzle5dScale;
#[allow(unused_imports)]
use semio_framework_plugin::app::{MeshView, MeshWindowKit, WindowKit};
#[allow(unused_imports)]
use semio_framework_plugin::{world3d_mesh_id_from_url, world3d_meshes_json_from_urls, world3d_selection_json, WindowKindDefinition};
use crate::viewer::puzzle5d::modes::view::windows::world3d::{instances_json};

use crate::viewer::puzzle5d::modes::view::windows::world3d::*;

#[test]
fn definition_uses_the_frozen_mesh_window_kit_id() {
    let def = definition();
    assert_eq!(def.id, "framework.window.mesh");
    assert_eq!(def.id, BODY_KEY);
}

#[test]
fn render_produces_a_scene_node_for_the_default_document() {
    let document = Puzzle5dSnapshot::default();
    let _node = render(&document);
}

#[test]
fn render_places_one_instance_per_part() {
    let document = Puzzle5dSnapshot { parts: vec![Puzzle5dPart { id: "p1".into(), ..Default::default() }], ..Default::default() };
    assert!(instances_json(&document).contains("\"p1\""));
}
