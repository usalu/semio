//! 🧪️ bridge tests (🔬️unit): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::WorldSubElementPaint;
use semio_framework_os_infinite_world_components::{face_overlay_color, WorldMeshBuffers};
use semio_framework_os_infinite_world_bridge::{component_overlay_color, srgb_byte_to_linear, world3d_apply_scalar_field_colors, world3d_scalar_field_fits, world3d_sub_element_paint_native, world3d_tone_rgba};
use semio_framework_test_fixtures::fixture_text;
use std::collections::HashSet;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_ui_theme::{Rgba, Theme};
use serde_json::json;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

//#region 🔖️Modelling
#[allow(non_snake_case)]
fn MODELLING_FIXTURE() -> &'static str {
    fixture_text!("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/📏️world3d-modelling/🔣️.json")
}

fn modelling_fixture() -> serde_json::Value {
    serde_json::from_str(MODELLING_FIXTURE()).expect("world3d modelling fixture")
}

fn tetrahedron() -> WorldMeshBuffers {
    let mut data = WorldMeshBuffers::default();
    data.positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    data.indices = vec![0, 1, 2, 0, 1, 3, 0, 2, 3, 1, 2, 3];
    data
}

fn field_named(name: &str, values: serde_json::Value) -> semio_framework_ui_declaration::ui::World3dScalarField {
    let entry = modelling_fixture()["valid"].as_array().unwrap().iter().find(|entry| entry["name"] == name).unwrap().clone();
    let mut value = entry["value"].clone();
    value["values"] = values;
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture scalar field")
}

#[test]
fn a_vertex_scalar_field_paints_the_canonical_linear_vertex_colours_and_a_wrong_count_leaves_the_mesh_alone() {
    let field = field_named("scalar-vertex-inferno", serde_json::json!([0.0, 0.5, 1.0, null]));
    let mut data = tetrahedron();
    assert!(world3d_apply_scalar_field(&mut data, &field));
    assert_eq!(data.colors.len(), 16);
    let to_linear = |hex: &str| -> [f32; 3] { [1, 3, 5].map(|at| srgb_byte_to_linear(u8::from_str_radix(&hex[at..at + 2], 16).unwrap())) };
    let fixture = modelling_fixture();
    let middle = fixture["ramps"]["inferno"]["samples"].as_array().unwrap().iter().find(|sample| sample["t"] == 0.5).unwrap()["hex"].as_str().unwrap().to_string();
    for (vertex, hex) in ["#000004".to_string(), middle, "#fcffa4".to_string(), fixture["noDataHex"].as_str().unwrap().to_string()].iter().enumerate() {
        let want = to_linear(hex);
        for axis in 0..3 {
            assert!((data.colors[vertex * 4 + axis] - want[axis]).abs() < 1e-6, "vertex {vertex} axis {axis}");
        }
        assert_eq!(data.colors[vertex * 4 + 3], 1.0);
    }
    let mut untouched = tetrahedron();
    assert!(!world3d_apply_scalar_field(&mut untouched, &field_named("scalar-vertex-inferno", serde_json::json!([0.0, 1.0]))));
    assert!(untouched.colors.is_empty() && untouched.attributes.is_empty());
}

#[test]
fn a_face_scalar_field_becomes_one_constant_colour_attribute_per_triangle_and_replaces_authored_colours() {
    let field = field_named("scalar-face-coolwarm", serde_json::json!([10.0, 12.0, 18.0, 20.0]));
    let mut data = tetrahedron();
    data.colors = vec![1.0; 16];
    data.attributes.insert("authored".into(), semio_framework::MeshAttribute { domain: semio_framework::MeshAttributeDomain::Vertex, semantic: semio_framework::MeshAttributeSemantic::Color, interpolation: semio_framework::MeshAttributeInterpolation::Linear, values: Vec::new(), indices: None });
    assert!(world3d_apply_scalar_field(&mut data, &field));
    assert!(data.colors.is_empty());
    assert!(!data.attributes.contains_key("authored"), "an authored colour channel never fights the analysis colours");
    let attribute = &data.attributes["scalarField"];
    assert_eq!((attribute.domain, attribute.semantic, attribute.interpolation), (semio_framework::MeshAttributeDomain::Face, semio_framework::MeshAttributeSemantic::Color, semio_framework::MeshAttributeInterpolation::Constant));
    assert_eq!(attribute.values.len(), 4);
}
//#endregion 🔖️Modelling

#[test]
fn highlight_tokens_recolour_exactly_their_granularity_through_the_live_theme() {
    let highlight: semio_framework_ui_declaration::ui::World3dHighlight = serde_json::from_value(serde_json::json!({
        "face": {"hover": "info", "selected": "secondary"},
        "edge": {"hover": "danger"},
        "vertex": {"selected": "success", "widthPx": 8}
    }))
    .unwrap();
    let mut custom = semio_framework_ui_theme::Theme::default();
    custom.celebrate[1] = semio_framework_ui_theme::Rgba::new(0.2, 0.4, 0.6, 1.0);
    custom.error = semio_framework_ui_theme::Rgba::new(0.9, 0.1, 0.1, 1.0);
    custom.success = semio_framework_ui_theme::Rgba::new(0.1, 0.9, 0.1, 1.0);
    for theme in [semio_framework_ui_theme::Theme::default(), custom] {
        let paint = world3d_sub_element_paint_native(&theme, Some(&highlight));
        assert_eq!(paint.face_select, Some([theme.celebrate[1].r, theme.celebrate[1].g, theme.celebrate[1].b, 1.0]));
        assert_eq!(paint.edge_hover, Some([theme.error.r, theme.error.g, theme.error.b, 1.0]));
        assert_eq!(paint.vertex_select, Some([theme.success.r, theme.success.g, theme.success.b, 1.0]));
        assert_eq!((paint.edge_select, paint.vertex_hover, paint.vertex_mark_px), (None, None, Some(8.0)), "an absent token keeps the built-in palette; the vertex width becomes the mark size");
        let selected: HashSet<String> = ["1".to_string()].into();
        let none = HashSet::new();
        assert_eq!(component_overlay_color("1", &selected, &none, &None, (paint.face_select, paint.face_hover)).map(|(color, _)| color), paint.face_select);
        assert_eq!(component_overlay_color("1", &selected, &none, &None, (paint.edge_select, paint.edge_hover)).map(|(color, _)| color), Some([0.35, 0.75, 1.0, 1.0]), "the edge selection token is absent: the built-in palette stays");
        let hovered = Some("2".to_string());
        assert_eq!(component_overlay_color("2", &none, &none, &hovered, (paint.edge_select, paint.edge_hover)).map(|(color, _)| color), Some([theme.error.r, theme.error.g, theme.error.b, 0.9]));
        assert_eq!(face_overlay_color(&paint, 2), [theme.celebrate[1].r, theme.celebrate[1].g, theme.celebrate[1].b, 0.62]);
    }
    assert_eq!(world3d_sub_element_paint_native(&semio_framework_ui_theme::Theme::default(), None), WorldSubElementPaint::default(), "no highlight leaves every default alone");
    assert_eq!(face_overlay_color(&WorldSubElementPaint::default(), 1), [0.35, 0.75, 1.0, 0.48]);
}

#[test]
fn every_tone_resolves_to_an_opaque_theme_colour() {
    let theme = semio_framework_ui_theme::Theme::default();
    for tone in semio_framework_ui_declaration::ui::World3dTone::ALL {
        let color = world3d_tone_rgba(&theme, *tone);
        assert_eq!(color[3], 1.0, "{tone:?}");
        assert!(color[..3].iter().all(|channel| (0.0..=1.0).contains(channel)));
    }
    assert_eq!(world3d_tone_rgba(&theme, semio_framework_ui_declaration::ui::World3dTone::Neutral), [theme.text.r, theme.text.g, theme.text.b, 1.0]);
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}

fn world3d_apply_scalar_field(data:&mut WorldMeshBuffers,field:&semio_framework_ui_declaration::ui::World3dScalarField)->bool{
    if !world3d_scalar_field_fits(data,field){return false;}
    for slot in 0..data.attributes.slot_count(){let progress=data.attributes.retire_slot_if(slot,|_,attribute|attribute.semantic!=semio_framework::MeshAttributeSemantic::Color,world_original_fixture_grant()).unwrap();assert_eq!(progress.copied_items,1);}
    world3d_apply_scalar_field_colors(data,field);true
}
