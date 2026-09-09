use super::*;
use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::terminology::puzzle3d_labels;
use crate::editor::puzzle3d::{nakagin_fixture, Puzzle3dScene, PUZZLE3D_DEFAULT_UTILITY};

#[test]
fn kinds_tree_object_drag_data_carries_object_kind_and_mesh_url() {
    let _page = super::super::document::PANEL_PAGE_GUARD.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let envelope = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let node = render(&envelope, labels).expect("catalogue tree");
    assert!(matches!(node.component, semio_framework_ui_contract::Component::Tree(_)));
    let objects = node.children.iter().find(|section| section.key.as_str() == "puzzle3d-play-kinds.objects").expect("objects section");
    let draggable = objects
        .children
        .iter()
        .find_map(|item| match &item.component {
            semio_framework_ui_contract::Component::TreeItem(props) if props.draggable == Some(true) => Some(props),
            _ => None,
        })
        .expect("draggable object kind");
    let drag_data = draggable.drag_data.as_ref().expect("drag data");
    let encoded = drag_data.iter().find(|(mime, _)| mime.as_str() == PUZZLE3D_CATALOGUE_DRAG_MIME).map(|(_, value)| value.as_str()).expect("catalogue mime");
    let payload: Value = json::parse(encoded).expect("drag payload json");
    assert!(payload.get("objectKind").and_then(Value::as_str).is_some(), "drag payload must carry objectKind");
    assert!(payload.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some(), "drag payload must carry meshUrl for preview");
}

/// 🧾️ Paging law: a catalog declaring more kinds than a built node admits children still renders. Every
/// section stays inside `UI_BUILT_CHILDREN_MAX`, each truncated one carries its `+N` continuation row, and
/// the nested vortex templates of an over-wide object kind page the same way.
#[test]
fn the_catalogue_pages_an_over_wide_kind_catalog_without_exceeding_the_fixed_page() {
    let _page = super::super::document::PANEL_PAGE_GUARD.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let kinds: Vec<Value> = (0..200)
        .map(|index| {
            let vortices: Vec<Value> = (0..40).map(|slot| json!({ "vortexKind": format!("edge-{slot}") })).collect();
            json!({ "id": format!("kind-{index}"), "label": format!("Kind {index}"), "meshUrl": "mesh://kind", "vortices": Value::from(vortices) })
        })
        .collect();
    let mut fixture = crate::editor::puzzle3d::empty_fixture();
    fixture.meta.kind_catalogs = Some(json::to_dsl_value(&json!({ "objects": Value::from(kinds) })));
    let envelope = Puzzle3dScene { fixture, runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let node = render(&envelope, labels).expect("an over-wide catalogue must still be admitted");
    let objects = node.children.iter().find(|section| section.key.as_str() == "puzzle3d-play-kinds.objects").expect("objects section");
    assert!(objects.children.len() <= semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX, "objects section declares {} children", objects.children.len());
    assert!(objects.children.iter().any(|row| row.key.as_str().ends_with(".more")), "a truncated catalogue section must carry a continuation row");
    for row in objects.children.iter() {
        assert!(row.children.len() <= semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX, "kind row {} declares {} children", row.key.as_str(), row.children.len());
    }
    eprintln!("[DEBUG] catalogue page kinds=200 objects-section-children={}", objects.children.len());
}
