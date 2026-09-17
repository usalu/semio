use super::*;
use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::terminology::puzzle3d_labels;
use crate::editor::puzzle3d::{nakagin_fixture, Puzzle3dScene, PUZZLE3D_DEFAULT_UTILITY};
use semio_framework_plugin::{TreeWindowRequest, ViewModel};

const RETIREMENT_DRAIN_STEPS: usize = 4096;

fn drain() {
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if ui::close_built_node_page_one() {
            break;
        }
    }
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if ui::close_ui_value_page_one() {
            break;
        }
    }
}

fn labels() -> &'static Puzzle3dLabels {
    puzzle3d_labels(&ViewModel::default()).expect("admitted host axis")
}

fn scene(fixture: crate::editor::puzzle3d::Puzzle3dFixture) -> Puzzle3dScene {
    Puzzle3dScene { fixture, runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() }
}

fn section_key(suffix: &str) -> String {
    format!("{ROOT}.{suffix}")
}

/// 🪟️ One host window request for this body — a scroll, an expand or a collapse in the React shell.
fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: BODY_KEY.to_string(), node_key: node_key.to_string(), open, offset, rows }
}

fn hosted(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

/// 🛍️ The catalogue with its object-kind section opened wide — what a reader scrolled to the top of
/// the panel sees.
fn objects_open() -> ViewModel {
    hosted(vec![request(&format!("{ROOT}.objects"), Some(true), 0, 128)])
}

fn built(envelope: &Puzzle3dScene, view: &ViewModel) -> BuiltNode {
    drain();
    render(envelope, labels(), &TreeWindows::for_body(view, BODY_KEY)).expect("catalogue tree")
}

fn json_of(envelope: &Puzzle3dScene, view: &ViewModel) -> String {
    let node = built(envelope, view);
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: node }).expect("catalogue projection");
    drain();
    json
}

fn tree_of(json: &str) -> serde_json::Value {
    serde_json::from_str(json).expect("the catalogue projection is JSON")
}

fn node_at<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| node_at(child, key))
}

fn child_keys(node: &serde_json::Value) -> Vec<String> {
    node["children"].as_array().map(|children| children.iter().filter_map(|child| child["key"].as_str().map(str::to_owned)).collect()).unwrap_or_default()
}

fn window_of(node: &serde_json::Value) -> (usize, usize) {
    let window = &node["component"]["window"];
    let total = window["total"].as_u64().unwrap_or_else(|| panic!("a container stamps its window: {node}"));
    (total as usize, window["offset"].as_u64().unwrap_or(0) as usize)
}

/// 🧫️ A catalog of `kinds` object kinds, each declaring `templates` rim-vortex templates.
fn wide_catalog(kinds: usize, templates: usize) -> crate::editor::puzzle3d::Puzzle3dFixture {
    let entries: Vec<Value> = (0..kinds)
        .map(|index| {
            let vortices: Vec<Value> = (0..templates).map(|slot| json!({ "vortexKind": format!("edge-{slot}") })).collect();
            json!({ "id": format!("kind-{index}"), "label": format!("Kind {index}"), "meshUrl": "mesh://kind", "vortices": Value::from(vortices) })
        })
        .collect();
    let mut fixture = crate::editor::puzzle3d::empty_fixture();
    fixture.meta.kind_catalogs = Some(json::to_dsl_value(&json!({ "objects": Value::from(entries) })));
    fixture
}

#[test]
fn kinds_tree_object_drag_data_carries_object_kind_and_mesh_url() {
    let envelope = scene(nakagin_fixture());
    let node = built(&envelope, &objects_open());
    assert!(matches!(node.component, ui::Component::Tree(_)));
    let objects = node.children.iter().find(|section| section.key.as_str() == "puzzle3d-play-kinds.objects").expect("objects section");
    let draggable = objects
        .children
        .iter()
        .find_map(|item| match &item.component {
            ui::Component::TreeItem(props) if props.draggable == Some(true) => Some(props),
            _ => None,
        })
        .expect("draggable object kind");
    let drag_data = draggable.drag_data.as_ref().expect("drag data");
    let encoded = drag_data.iter().find(|(mime, _)| mime.as_str() == PUZZLE3D_CATALOGUE_DRAG_MIME).map(|(_, value)| value.as_str()).expect("catalogue mime");
    let payload: Value = json::parse(encoded).expect("drag payload json");
    assert!(payload.get("objectKind").and_then(Value::as_str).is_some(), "drag payload must carry objectKind");
    assert!(payload.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some(), "drag payload must carry meshUrl for preview");
    drop(node);
    drain();
}

//#region 🪟️WindowLaws
/// 🪟️ A catalog an order of magnitude past one viewport stamps its FULL extent and materialises only
/// the host's slice. The old build truncated at `UI_BUILT_CHILDREN_MAX` and closed the section with a
/// `+N` whose `setPanelPage` cursor was never read back (`📓️audit-app-panels-a.md` §4a) — the rows past
/// the first page were unreachable. Now the section reports all of them and the host scrolls.
#[test]
fn an_over_wide_catalog_stamps_its_total_and_materialises_only_its_window() {
    let envelope = scene(wide_catalog(200, 40));
    let view = hosted(vec![request(&section_key("objects"), Some(true), 0, 10)]);
    let json = json_of(&envelope, &view);
    let tree = tree_of(&json);
    let objects = node_at(&tree, &section_key("objects")).expect("objects section");
    assert_eq!(window_of(objects), (200, 0), "the section reports the WHOLE catalog: {objects}");
    let expected: Vec<String> = (0..10).map(|index| format!("kind-{index}")).collect();
    assert_eq!(child_keys(objects), expected, "exactly the ten rows the host asked for, keyed by kind id: {objects}");
    for row in objects["children"].as_array().expect("rows") {
        assert_eq!(window_of(row).0, 40, "each kind row stamps its own rim-vortex template total: {row}");
        assert!(child_keys(row).is_empty(), "a folded kind row builds no template: {row}");
    }
    assert!(!json.contains(".more"), "no continuation row survives anywhere: {json}");
    assert!(!json.contains("\"+"), "and no `+N` label either: {json}");
}

/// 🪟️ A closed section costs one stamped `total` and nothing else.
#[test]
fn a_closed_catalogue_section_stamps_its_total_and_materialises_no_child() {
    let envelope = scene(wide_catalog(50, 2));
    let view = hosted(vec![request(&section_key("objects"), Some(false), 0, 64)]);
    let tree = tree_of(&json_of(&envelope, &view));
    let objects = node_at(&tree, &section_key("objects")).expect("objects section");
    assert_eq!(window_of(objects), (50, 0), "a closed section still reports everything it owns: {objects}");
    assert!(child_keys(objects).is_empty(), "and materialises none of it: {objects}");
}

/// 🪟️ Scrolling a catalog section, and expanding ONE kind's rim-vortex templates, both resolve to an
/// exact slice keyed by the entry's own id.
#[test]
fn a_tree_window_request_materialises_exactly_its_slice_of_the_catalog() {
    let envelope = scene(wide_catalog(120, 12));
    let view = hosted(vec![request(&section_key("objects"), Some(true), 90, 4), request(&format!("{}{}kind-91", section_key("objects"), ui::TREE_WINDOW_PATH_SEPARATOR), Some(true), 3, 3)]);
    let tree = tree_of(&json_of(&envelope, &view));
    let objects = node_at(&tree, &section_key("objects")).expect("objects section");
    assert_eq!(window_of(objects), (120, 90), "the section reports all 120 kinds and where the slice starts: {objects}");
    assert_eq!(child_keys(objects), vec!["kind-90", "kind-91", "kind-92", "kind-93"], "exactly entries [90, 94) are built: {objects}");
    let row = node_at(&tree, "kind-91").expect("the opened kind row");
    assert_eq!(window_of(row), (12, 3), "the kind row reports all twelve templates and the slice start: {row}");
    assert_eq!(child_keys(row), vec!["puzzle3d-kind-vortex.3.edge-3", "puzzle3d-kind-vortex.4.edge-4", "puzzle3d-kind-vortex.5.edge-5"], "exactly entries [3, 6) are built: {row}");
}
//#endregion 🪟️WindowLaws

/// 🛍️ Wave B26: the catalogue row's OWN add gesture. Battery #48 measured
/// `catalogue-add-object-kind before=1 after=1` while `catalogue-drag-drop` — a different route into the
/// same command — passed, so this pins the half the drop never exercises: every object-kind row must
/// declare an `activate` binding addressed at `addObjectKind` and carrying that kind's own id as args.
/// The row is also EXPANDABLE (its rim-vortex templates are its children), which is exactly the shape a
/// tree renderer is most tempted to treat as a fold toggle and nothing else, so the binding has to be on
/// the row itself rather than on a leaf underneath it.
#[test]
fn every_object_kind_row_binds_activate_to_add_object_kind_with_its_own_kind_id() {
    let envelope = scene(nakagin_fixture());
    let node = built(&envelope, &objects_open());
    let objects = node.children.iter().find(|section| section.key.as_str() == "puzzle3d-play-kinds.objects").expect("objects section");
    let rows: Vec<_> = objects.children.iter().collect();
    assert!(!rows.is_empty(), "the nakagin catalogue declares object kinds");
    for row in rows {
        let binding = row
            .bindings
            .iter()
            .find(|binding| matches!(binding.trigger, Trigger::Activate))
            .unwrap_or_else(|| panic!("object kind row {} declares no activate binding: {:?}", row.key.as_str(), row.bindings));
        assert_eq!(binding.action.name.as_str(), "addObjectKind", "the row's activate binding must address addObjectKind");
        let args = binding.args.as_ref().unwrap_or_else(|| panic!("object kind row {} binds addObjectKind with no args", row.key.as_str()));
        let ui::UiValue::Map(entries) = args else { panic!("object kind row {} args are not a map: {args:?}", row.key.as_str()) };
        let kind = entries
            .iter()
            .find(|(key, _)| key.as_str() == "objectKind")
            .map(|(_, value)| value)
            .unwrap_or_else(|| panic!("object kind row {} args carry no objectKind", row.key.as_str()));
        let ui::UiValue::Text(text) = kind else { panic!("objectKind arg is not text: {kind:?}") };
        assert_eq!(text.as_str(), row.key.as_str(), "the row must ask for the kind it renders");
        eprintln!("[DEBUG] catalogue row activate row={} kind={}", row.key.as_str(), text.as_str());
    }
    drop(node);
    drain();
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
    let envelope = scene(nakagin_fixture());
    drain();
    let node = render(&envelope, labels(), &TreeWindows::unhosted()).expect("catalogue tree");
    let open_state = |key: &str| {
        let section = node.children.iter().find(|section| section.key.as_str() == key).unwrap_or_else(|| panic!("section {key}"));
        let ui::Component::TreeSection(props) = &section.component else { panic!("section {key} is not a tree section") };
        props.default_open
    };
    assert_eq!(open_state("puzzle3d-play-kinds.objects"), Some(true), "the placeable object kinds must be laid out when the panel opens");
    for folded in ["puzzle3d-play-kinds.vortices", "puzzle3d-play-kinds.cables", "puzzle3d-play-kinds.attractions"] {
        assert_ne!(open_state(folded), Some(true), "{folded} is a template catalog and stays folded, so the first row under the panel is an object kind");
    }
    let objects = node.children.iter().find(|section| section.key.as_str() == "puzzle3d-play-kinds.objects").expect("objects section");
    assert!(!objects.children.is_empty(), "an opened objects section must carry at least one kind row on the first paint");
    eprintln!("[DEBUG] catalogue default-open objects={:?} rows={}", open_state("puzzle3d-play-kinds.objects"), objects.children.len());
    drop(node);
    drain();
}

/// 🌲️ The DEFAULT document's catalogue, which every law above misses: they all read
/// `nakagin_fixture()`, while the aggregator boots on `concrete-forest` (`initial_snapshot` /
/// `create_puzzle3d_app`), so an empty objects catalog THERE is the one a user actually sees. Pins
/// that the shipped fixture declares object kinds and that each one names a `/mesh/` representation
/// url which resolves through the SAME index the world mesh lane publishes — never a `meshUrl` key,
/// which these compose-shaped rows do not carry.
#[test]
fn the_default_concrete_forest_catalogue_declares_kinds_with_resolvable_mesh_urls() {
    let fixture = crate::editor::puzzle3d::default_fixture();
    let entries = crate::editor::puzzle3d::puzzle3d_catalog_entries(&fixture, "objects");
    assert!(!entries.is_empty(), "the default document's catalogue must declare object kinds, else the Catalogue panel opens empty");
    let index = crate::editor::puzzle3d::Puzzle3dKindMeshIndex::of(&fixture.meta);
    let lane = crate::editor::puzzle3d::collect_mesh_urls(&fixture);
    for entry in entries {
        let kind_id = entry.get("id").and_then(dsl::DslValue::as_str).expect("every catalogue row names a kind").to_string();
        let url = entry
            .get("representations")
            .and_then(dsl::DslValue::as_array)
            .into_iter()
            .flatten()
            .filter_map(|representation| representation.get("url").and_then(dsl::DslValue::as_str))
            .find(|url| !url.is_empty())
            .unwrap_or_else(|| panic!("catalogue row {kind_id} declares no representations[].url"))
            .to_string();
        assert!(url.starts_with("/mesh/"), "catalogue row {kind_id} must name a served mesh route, observed {url}");
        let probe = crate::editor::puzzle3d::Puzzle3dObject {
            id: format!("probe-{kind_id}"),
            label: None,
            object_kind: Some(kind_id.clone()),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: Vec::new(),
            hidden: false,
            locked: false,
        };
        assert_eq!(index.resolve(&probe), Some(url.as_str()), "an instance of {kind_id} must resolve its kind's representation url");
        assert!(lane.iter().any(|published| published == &url), "{kind_id}'s mesh {url} must reach the world mesh lane, else its candidates read mesh-unavailable");
        eprintln!("[DEBUG] concrete-forest catalogue kind={kind_id} url={url}");
    }
}

/// 🛍️ The default document's catalogue as the panel RENDERS it: one row per catalogued kind — the
/// window the host opens spans the whole catalog, so nothing is dropped — every row draggable with its
/// own kind id and a non-empty `meshUrl` in the drag payload (the key `World3dHost`'s catalogue-drop
/// preview parses).
#[test]
fn the_default_concrete_forest_catalogue_renders_a_draggable_row_for_every_kind() {
    let fixture = crate::editor::puzzle3d::default_fixture();
    let declared = crate::editor::puzzle3d::puzzle3d_catalog_entries(&fixture, "objects").len();
    let envelope = scene(fixture);
    let node = built(&envelope, &objects_open());
    let objects = node.children.iter().find(|section| section.key.as_str() == "puzzle3d-play-kinds.objects").expect("objects section");
    let rows: Vec<_> = objects.children.iter().collect();
    assert_eq!(rows.len(), declared, "every catalogued kind of the default document must get its own row");
    for row in rows {
        let ui::Component::TreeItem(props) = &row.component else { panic!("catalogue row {} is not a tree item", row.key.as_str()) };
        assert_eq!(props.draggable, Some(true), "catalogue row {} must be draggable into the viewport", row.key.as_str());
        assert!(row.bindings.len() <= 2, "catalogue row {} declares {} bindings, over the two a panel row admits", row.key.as_str(), row.bindings.len());
        let drag_data = props.drag_data.as_ref().unwrap_or_else(|| panic!("catalogue row {} carries no drag data", row.key.as_str()));
        let encoded = drag_data.iter().find(|(mime, _)| mime.as_str() == PUZZLE3D_CATALOGUE_DRAG_MIME).map(|(_, value)| value.as_str()).expect("catalogue mime");
        let payload: Value = json::parse(encoded).expect("drag payload json");
        assert_eq!(payload.get("objectKind").and_then(Value::as_str), Some(row.key.as_str()), "the drag payload must name the kind its row renders");
        assert!(payload.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some(), "catalogue row {} carries no meshUrl for the drop preview", row.key.as_str());
        eprintln!("[DEBUG] concrete-forest catalogue row={} payload={encoded}", row.key.as_str());
    }
    drop(node);
    drain();
}
