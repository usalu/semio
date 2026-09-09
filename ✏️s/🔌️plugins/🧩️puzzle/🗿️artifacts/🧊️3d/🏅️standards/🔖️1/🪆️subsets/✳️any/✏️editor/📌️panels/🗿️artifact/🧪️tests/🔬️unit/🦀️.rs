use crate::editor::puzzle3d::terminology::{puzzle3d_labels, Puzzle3dLabels};
use crate::editor::puzzle3d::{default_fixture, empty_fixture, with_puzzle3d_app, PUZZLE3D_DOCUMENT_TREE_BUILDS};

const RETIREMENT_DRAIN_STEPS: usize = 4096;

fn builds() -> u32 {
    PUZZLE3D_DOCUMENT_TREE_BUILDS.with(std::cell::Cell::get)
}

fn labels_for(terminology: semio_framework_plugin::Terminology) -> &'static Puzzle3dLabels {
    puzzle3d_labels(&semio_framework_plugin::ViewModel { terminology, ..Default::default() }).expect("admitted host axis")
}

/// ♻️ Stands in for the reactor's own one-page-per-turn retirement pump, which no unit test has: a
/// released tree hands its child backings and its `UiValue` pages to the retirement authorities, and
/// only closing them returns the process-wide admission credit the next build needs.
fn drain_retired_ui_owners() {
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if super::ui::close_built_node_page_one() {
            break;
        }
    }
    for _ in 0..RETIREMENT_DRAIN_STEPS {
        if super::ui::close_ui_value_page_one() {
            break;
        }
    }
}

/// 🔒️ Holds [`super::PANEL_PAGE_GUARD`] for the body of one law, so the page it measures is the only page
/// competing for the process-global arena while it runs.
fn page_guard() -> std::sync::MutexGuard<'static, ()> {
    super::PANEL_PAGE_GUARD.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// 🧾️ The memo laws, in one serialized body because the admission credit they consume is process-wide:
/// an unchanged fixture and label set is served by `BuiltNode::credited_clone` with no rebuild, while a
/// terminology switch and a fixture switch each miss — the key carries both axes, so a locale or
/// terminology change can never keep serving the previous language's rows.
#[test]
fn the_outliner_memo_serves_one_key_and_rebuilds_on_a_fixture_or_label_switch() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    let reuse = labels_for(semio_framework_plugin::Terminology::Reuse);
    let empty = empty_fixture();
    with_puzzle3d_app(|app| {
        drain_retired_ui_owners();
        PUZZLE3D_DOCUMENT_TREE_BUILDS.with(|counter| counter.set(0));
        let cold = app.document_tree_cached(&empty, native).expect("cold outliner build");
        assert_eq!(builds(), 1, "the cold read must build exactly once");
        let warm = app.document_tree_cached(&empty, native).expect("memoized outliner read");
        assert_eq!(builds(), 1, "the warm read must not rebuild");
        assert_eq!(cold.key.as_str(), warm.key.as_str());
        assert_eq!(cold.children.len(), warm.children.len());
        for (retained, aliased) in cold.children.iter().zip(warm.children.iter()) {
            assert_eq!(retained.key.as_str(), aliased.key.as_str());
            assert_eq!(retained.children.len(), aliased.children.len());
        }
        drop((cold, warm));
        drain_retired_ui_owners();
        app.document_tree_cached(&empty, reuse).expect("terminology switch rebuild");
        assert_eq!(builds(), 2, "a different label set must rebuild");
        drain_retired_ui_owners();
        let populated = default_fixture();
        app.document_tree_cached(&populated, reuse).expect("fixture switch rebuild");
        assert_eq!(builds(), 3, "a different fixture must rebuild");
        app.document_tree_cached(&populated, reuse).expect("memoized outliner read");
        assert_eq!(builds(), 3, "the newest key stays memoized");
    });
    drain_retired_ui_owners();
}

/// 🏗️ A synthetic document of `objects` objects carrying `vortices` vortices each — the shape the flagship
/// Nakagin fixture has (180 objects, ≈2 vortices per object) and the shape a `DOCUMENT_OBJECT_SLOTS`-scale
/// document has, without depending on either asset's contents.
fn scaled_fixture(objects: usize, vortices: usize) -> crate::editor::puzzle3d::Puzzle3dFixture {
    let mut fixture = empty_fixture();
    fixture.objects = (0..objects)
        .map(|index| crate::editor::puzzle3d::Puzzle3dObject {
            id: format!("object-{index}"),
            label: None,
            object_kind: Some("capsule".into()),
            origin: [f64::from(index as u32), 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: (0..vortices).map(|slot| crate::editor::puzzle3d::Puzzle3dVortex { id: format!("vortex-{slot}"), vortex_kind: Some("edge".into()), ..Default::default() }).collect(),
            hidden: false,
            locked: false,
            reveal_index: None,
        })
        .collect();
    fixture
}

/// 🧾️ Every row of one built page, depth-first, paired with the number of children it declares.
fn walk(node: &super::BuiltNode, rows: &mut Vec<(String, usize, usize, usize)>) {
    let row_actions = match &node.component {
        semio_framework_ui_contract::Component::TreeItem(props) => props.row_actions.len(),
        _ => 0,
    };
    rows.push((node.key.as_str().to_string(), node.children.len(), node.bindings.len(), row_actions));
    for child in node.children.iter() {
        walk(child, rows);
    }
}

/// 🧾️ The paging laws. A Nakagin-scale document and a `DOCUMENT_OBJECT_SLOTS`-scale one both render: no
/// `ui.fixed-capacity` fault, no node wider than the built-children contract, no more interactive rows than
/// the page ceiling, every truncated section closed by a `+N` continuation row, and every materialised
/// object row still carrying its activation binding plus both `setSelectionFlag` row actions. The counts
/// are asserted as bounds rather than equalities on purpose: the admission credit a page spends is
/// process-wide, so a sibling test holding part of it must shorten this page, never fault it.
#[test]
fn the_outliner_pages_a_document_scale_fixture_without_exceeding_the_fixed_page() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    for (objects, vortices) in [(180usize, 2usize), (1200, 2), (1200, 0)] {
        drain_retired_ui_owners();
        let fixture = scaled_fixture(objects, vortices);
        let page = super::render(&fixture, native).expect("a document-scale outliner page must be admitted");
        let mut rows = Vec::new();
        walk(&page, &mut rows);
        let interactive = rows.iter().filter(|(_, _, bindings, _)| *bindings > 0).count();
        let continuations = rows.iter().filter(|(key, _, _, _)| key.ends_with(".more")).count();
        assert!(interactive <= semio_framework_ui_contract::UI_VALUE_PAGE_ROWS, "{objects} objects materialised {interactive} interactive rows over the page ceiling");
        assert!(interactive >= 1, "{objects} objects materialised no row at all");
        for (key, children, _, _) in &rows {
            assert!(*children <= semio_framework_ui_contract::UI_BUILT_CHILDREN_MAX, "row {key} declares {children} children");
        }
        let object_rows: Vec<_> = rows.iter().filter(|(key, _, _, row_actions)| key.starts_with("object-") && *row_actions > 0).collect();
        assert!(!object_rows.is_empty(), "{objects} objects materialised no object row");
        assert!(object_rows.len() <= super::SECTION_ROWS, "{objects} objects materialised {} object rows", object_rows.len());
        for (key, _, bindings, row_actions) in &object_rows {
            assert_eq!(*bindings, 1, "object row {key} lost its interactionSelect activation binding");
            assert_eq!(*row_actions, 2, "object row {key} lost its hide/lock row actions");
        }
        assert!(continuations >= 1, "{objects} objects truncated the objects section yet emitted no continuation row");
        eprintln!("[DEBUG] outliner page objects={objects} vortices={vortices} interactive={interactive} continuations={continuations} object-rows={}", object_rows.len());
        drop(page);
    }
    drain_retired_ui_owners();
}

/// 🪺️ A document the page can hold keeps its nested rows: the reservation that stops one wide parent from
/// eating its section only bites under pressure, so five objects still carry all ten vortex children and
/// nothing is truncated.
#[test]
fn a_document_that_fits_the_page_keeps_every_nested_vortex_row() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    drain_retired_ui_owners();
    let fixture = scaled_fixture(5, 2);
    let page = super::render(&fixture, native).expect("a small outliner page must be admitted");
    let mut rows = Vec::new();
    walk(&page, &mut rows);
    let interactive = rows.iter().filter(|(_, _, bindings, _)| *bindings > 0).count();
    assert_eq!(interactive, 15, "a document that fits keeps every object row AND every vortex child");
    assert_eq!(rows.iter().filter(|(key, _, _, _)| key.ends_with(".more")).count(), 0, "a document that fits truncates nothing");
    eprintln!("[DEBUG] outliner page objects=5 vortices=2 interactive={interactive}");
    drop(page);
    drain_retired_ui_owners();
}

/// 🗼️ The flagship example itself: `nakagin_fixture()` used to fail `document::render` cold with
/// `ui.fixed-capacity` (`📓️2026-09-09-wave-G2-locale-and-tree-memo.md` §5.1). It must render.
#[test]
fn the_outliner_renders_the_nakagin_example() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    drain_retired_ui_owners();
    let fixture = crate::editor::puzzle3d::nakagin_fixture();
    let page = super::render(&fixture, native).expect("the Nakagin outliner page must be admitted");
    let mut rows = Vec::new();
    walk(&page, &mut rows);
    assert_eq!(page.children.len(), 4, "the outliner keeps its four document sections");
    eprintln!("[DEBUG] nakagin outliner objects={} rows={}", fixture.objects.len(), rows.len());
    drop(page);
    drain_retired_ui_owners();
}
