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

/// 🛍️ Wave B26: the catalogue row's OWN add gesture. Battery #48 measured
/// `catalogue-add-object-kind before=1 after=1` while `catalogue-drag-drop` — a different route into the
/// same command — passed, so this pins the half the drop never exercises: every object-kind row must
/// declare an `activate` binding addressed at `addObjectKind` and carrying that kind's own id as args.
/// The row is also EXPANDABLE (its rim-vortex templates are its children), which is exactly the shape a
/// tree renderer is most tempted to treat as a fold toggle and nothing else, so the binding has to be on
/// the row itself rather than on a leaf underneath it.
#[test]
fn every_object_kind_row_binds_activate_to_add_object_kind_with_its_own_kind_id() {
    let _page = super::super::document::PANEL_PAGE_GUARD.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let envelope = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let node = render(&envelope, labels).expect("catalogue tree");
    let objects = node.children.iter().find(|section| section.key.as_str() == "puzzle3d-play-kinds.objects").expect("objects section");
    let rows: Vec<_> = objects.children.iter().filter(|row| !row.key.as_str().ends_with(".more")).collect();
    assert!(!rows.is_empty(), "the nakagin catalogue declares object kinds");
    for row in rows {
        let binding = row
            .bindings
            .iter()
            .find(|binding| matches!(binding.trigger, semio_framework_ui_contract::Trigger::Activate))
            .unwrap_or_else(|| panic!("object kind row {} declares no activate binding: {:?}", row.key.as_str(), row.bindings));
        assert_eq!(binding.action.name.as_str(), "addObjectKind", "the row's activate binding must address addObjectKind");
        let args = binding.args.as_ref().unwrap_or_else(|| panic!("object kind row {} binds addObjectKind with no args", row.key.as_str()));
        let semio_framework_ui_contract::UiValue::Map(entries) = args else { panic!("object kind row {} args are not a map: {args:?}", row.key.as_str()) };
        let kind = entries
            .iter()
            .find(|(key, _)| key.as_str() == "objectKind")
            .map(|(_, value)| value)
            .unwrap_or_else(|| panic!("object kind row {} args carry no objectKind", row.key.as_str()));
        let semio_framework_ui_contract::UiValue::Text(text) = kind else { panic!("objectKind arg is not text: {kind:?}") };
        assert_eq!(text.as_str(), row.key.as_str(), "the row must ask for the kind it renders");
        eprintln!("[DEBUG] catalogue row activate row={} kind={}", row.key.as_str(), text.as_str());
    }
}

/// 🛍️ Wave B27: the catalogue must OPEN on the section whose rows a press can place.
///
/// Every section declared `default_open: false`, so the Catalogue panel rendered four empty headers.
/// A folded row is `display:none` — it still answers `querySelectorAll`, but its box is `{0,0,0,0}`, so
/// the battery's `catalogue-add-object-kind` hit-tested the viewport ORIGIN (inside the navbar) and read
/// "the row is covered by chrome" for three waves while the real state was "no row is laid out at all".
/// The outliner's own primary section (`📌️panels/🗿️artifact/🦀️.rs`) has always opened this way; this
/// pins the same rule here, and pins that the secondary catalogs stay folded so the first `treeitem`
/// under the panel is an object kind rather than a vortex template.
#[test]
fn the_catalogue_opens_its_object_kinds_and_folds_the_template_catalogs() {
    let _page = super::super::document::PANEL_PAGE_GUARD.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let envelope = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let node = render(&envelope, labels).expect("catalogue tree");
    let open_state = |key: &str| {
        let section = node.children.iter().find(|section| section.key.as_str() == key).unwrap_or_else(|| panic!("section {key}"));
        let semio_framework_ui_contract::Component::TreeSection(props) = &section.component else { panic!("section {key} is not a tree section") };
        props.default_open
    };
    assert_eq!(open_state("puzzle3d-play-kinds.objects"), Some(true), "the placeable object kinds must be laid out when the panel opens");
    for folded in ["puzzle3d-play-kinds.vortices", "puzzle3d-play-kinds.cables", "puzzle3d-play-kinds.attractions"] {
        assert_ne!(open_state(folded), Some(true), "{folded} is a template catalog and stays folded, so the first row under the panel is an object kind");
    }
    let objects = node.children.iter().find(|section| section.key.as_str() == "puzzle3d-play-kinds.objects").expect("objects section");
    assert!(objects.children.iter().any(|row| !row.key.as_str().ends_with(".more")), "an opened objects section must carry at least one kind row");
    eprintln!("[DEBUG] catalogue default-open objects={:?} rows={}", open_state("puzzle3d-play-kinds.objects"), objects.children.len());
}
