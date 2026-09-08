
use super::*;
use crate::editor::puzzle3d::config::{Puzzle3dConfig, Puzzle3dRuntime};
use crate::editor::puzzle3d::terminology::puzzle3d_labels;
use crate::editor::puzzle3d::{PUZZLE3D_DEFAULT_UTILITY, Puzzle3dScene, nakagin_fixture};

#[test]
fn kinds_tree_object_drag_data_carries_object_kind_and_mesh_url() {
    let envelope = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default());
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
