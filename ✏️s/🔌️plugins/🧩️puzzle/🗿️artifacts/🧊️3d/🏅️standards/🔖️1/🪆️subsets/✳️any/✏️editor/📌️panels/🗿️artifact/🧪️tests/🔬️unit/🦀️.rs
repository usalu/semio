use std::collections::BTreeMap;
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
    drop(page);
    drain_retired_ui_owners();
}

/// 🏢️ The flagship example's OBJECT ROWS, not just its four sections: wave B44 §6.2 read
/// `outliner entityRows=0` in the browser on this document and could establish no selection through it
/// at all, so "the page was admitted" was green while the panel presented nothing selectable. Every
/// materialised object row must carry the `interactionSelect` activation binding a click needs, and the
/// truncated section must name what it left out with a continuation the next page can be reached by.
#[test]
fn the_nakagin_outliner_page_carries_selectable_object_rows() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    drain_retired_ui_owners();
    let fixture = crate::editor::puzzle3d::nakagin_fixture();
    assert!(fixture.objects.len() >= 100, "the Nakagin fixture must be the large document: {} objects", fixture.objects.len());
    let page = super::render(&fixture, native).expect("the Nakagin outliner page must be admitted");
    let mut rows = Vec::new();
    walk(&page, &mut rows);
    let object_ids: std::collections::HashSet<&str> = fixture.objects.iter().map(|object| object.id.as_str()).collect();
    let object_rows: Vec<_> = rows.iter().filter(|(key, _, _, _)| object_ids.contains(key.as_str())).collect();
    eprintln!("[DEBUG] b46.outliner nakagin objects={} nodes={} objectRows={} continuations={}", fixture.objects.len(), rows.len(), object_rows.len(), rows.iter().filter(|(key, _, _, _)| key.ends_with(".more")).count());
    assert!(!object_rows.is_empty(), "the Nakagin outliner presented no object row at all: {rows:?}");
    for (key, _, bindings, row_actions) in &object_rows {
        assert_eq!(*bindings, 1, "object row {key} lost its interactionSelect activation binding");
        assert_eq!(*row_actions, 2, "object row {key} lost its hide/lock row actions");
    }
    let more = rows.iter().find(|(key, _, _, _)| key == &format!("{}.objects.more", super::ROOT)).expect("the truncated objects section must close with a continuation row");
    assert!(more.2 > 0, "the Nakagin continuation row must advance setPanelPage: {more:?}");
    drop(page);
    drain_retired_ui_owners();
}

/// 🔁️ One row action's `setSelectionFlag` args, flattened to `(flag, value)` — the two entries the
/// reducer reads (`🎮️commands/🔖️set-selection-flag/🦀️.rs`).
fn flag_binding(row_action: &semio_framework_ui_contract::RowAction) -> (String, bool) {
    let Some(semio_framework_plugin::UiValue::Map(map)) = row_action.action.args.as_ref() else {
        panic!("a hide/lock row action must carry setSelectionFlag args");
    };
    let (mut flag, mut value) = (String::new(), None);
    let mut cursor = map.iter();
    while let Some((key, entry)) = cursor.advance() {
        match (key.as_str(), entry) {
            ("flag", semio_framework_plugin::UiValue::Text(text)) => flag = text.as_str().to_string(),
            ("value", semio_framework_plugin::UiValue::Bool(bit)) => value = Some(*bit),
            _ => {}
        }
    }
    (flag, value.expect("a hide/lock row action must carry an explicit value"))
}

/// 🔁️ Every hide/lock row action of one built page, keyed by its row.
fn flag_bindings(node: &super::BuiltNode, rows: &mut Vec<(String, String, bool)>) {
    if let semio_framework_ui_contract::Component::TreeItem(props) = &node.component {
        for row_action in props.row_actions.iter() {
            let (flag, value) = flag_binding(row_action);
            rows.push((node.key.as_str().to_string(), flag, value));
        }
    }
    for child in node.children.iter() {
        flag_bindings(child, rows);
    }
}

/// 🙈️ The outliner's inline hide/lock toggles must ASK FOR THE INVERSE of the row's current flag —
/// `📓️2026-09-09-user-feature-checklist.md` §17/summary #8: `flag_args` hardcoded `value: true`, so
/// "Show"/"Unlock" re-sent the state the row was already in and an outliner-hidden object could never
/// be un-hidden from the row that hid it. Asserted for all three row kinds that carry the toggles
/// (object / reference / target volume) in both states, so a regression in any one of them fails.
#[test]
fn outliner_hide_and_lock_rows_dispatch_the_inverse_of_the_current_flag() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    for flagged in [false, true] {
        drain_retired_ui_owners();
        let mut fixture = empty_fixture();
        fixture.objects.push(crate::editor::puzzle3d::Puzzle3dObject {
            id: "object-1".into(),
            label: None,
            object_kind: Some("Object".into()),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: None,
            mesh_url: None,
            vortices: Vec::new(),
            hidden: flagged,
            locked: flagged,
            reveal_index: None,
        });
        fixture.references.push(crate::editor::puzzle3d::Puzzle3dReference {
            id: "reference-1".into(),
            source: crate::editor::puzzle3d::Puzzle3dReferenceSource { url: "/reference/plan.png".into(), media_kind: Some("image".into()) },
            hidden: flagged,
            locked: flagged,
            ..Default::default()
        });
        fixture.target_volumes.push(crate::editor::puzzle3d::Puzzle3dTargetVolume { id: "volume-1".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, hidden: flagged, locked: flagged });
        let page = super::render(&fixture, native).expect("a three-row outliner page must be admitted");
        let mut rows = Vec::new();
        flag_bindings(&page, &mut rows);
        assert_eq!(rows.len(), 6, "one hide plus one lock row action per object/reference/target-volume row: {rows:?}");
        for (key, flag, value) in &rows {
            assert_eq!(*value, !flagged, "row {key}'s {flag} toggle must ask for {} while the row is {flagged}", !flagged);
        }
        for expected in ["object-1", "reference-1", "volume-1"] {
            assert_eq!(rows.iter().filter(|(key, _, _)| key == expected).count(), 2, "row {expected} lost a hide/lock action: {rows:?}");
        }
        drop(page);
    }
    drain_retired_ui_owners();
}

/// 🔁️ The checklist's own QA, closed as a loop: hide an object through the outliner's inline row
/// action, re-render, then UN-hide it through the SAME row — for `hidden` and for `locked`. The
/// sibling law above pins one render's args; this one pins that feeding those args to the reducer the
/// row names (`setSelectionFlag`'s explicit `{entity, ids}` path, i.e.
/// [`apply_puzzle3d_selection_flag`]) and re-rendering yields the OPPOSITE request, so the second
/// click undoes the first. With the old hardcoded `value: true` the row asked for `true` on both
/// passes and the object could never come back.
///
/// [`apply_puzzle3d_selection_flag`]: crate::editor::puzzle3d::apply_puzzle3d_selection_flag
#[test]
fn an_outliner_flag_row_undoes_itself_on_the_second_click() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    let mut fixture = empty_fixture();
    fixture.objects.push(crate::editor::puzzle3d::Puzzle3dObject {
        id: "object-1".into(),
        label: None,
        object_kind: Some("Object".into()),
        origin: [0.0, 0.0, 0.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
        reveal_index: None,
    });
    let requested = |fixture: &crate::editor::puzzle3d::Puzzle3dFixture, flag: &str| {
        drain_retired_ui_owners();
        let page = super::render(fixture, native).expect("a one-row outliner page must be admitted");
        let mut rows = Vec::new();
        flag_bindings(&page, &mut rows);
        let value = rows.iter().find(|(key, rendered, _)| key == "object-1" && rendered == flag).map(|(_, _, value)| *value).unwrap_or_else(|| panic!("the outliner row must offer a {flag} toggle: {rows:?}"));
        drop(page);
        value
    };
    let state = |fixture: &crate::editor::puzzle3d::Puzzle3dFixture, flag: &str| {
        let object = fixture.objects.first().expect("the one object survives every flag write");
        if flag == "locked" {
            object.locked
        } else {
            object.hidden
        }
    };
    for flag in ["hidden", "locked"] {
        for expected in [true, false] {
            let asked = requested(&fixture, flag);
            assert_eq!(asked, expected, "the outliner's {flag} row must ask for {expected} while the object is {}", !expected);
            crate::editor::puzzle3d::apply_puzzle3d_selection_flag(&mut fixture, "object", &["object-1".to_string()], flag, asked);
            assert_eq!(state(&fixture, flag), expected, "clicking the outliner's own {flag} row must reach {expected}");
        }
    }
    drain_retired_ui_owners();
}

/// 📄 Wave U: the objects-section `+N` row is a `setPanelPage` control. Page 1 starts at the next
/// object; the last page drops the continuation.
#[test]
fn pressing_the_outliner_continuation_reveals_the_next_page() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    let rows_per = super::SECTION_ROWS;
    let objects = rows_per.saturating_mul(2).saturating_add(3);
    let section = "puzzle3d-play-document.objects";
    drain_retired_ui_owners();
    let fixture = scaled_fixture(objects, 0);
    let first = super::render_from(&fixture, native, &BTreeMap::new()).expect("page 0");
    let mut rows = Vec::new();
    walk(&first, &mut rows);
    let _more = rows.iter().find(|(key, _, bindings, _)| key.ends_with(".more") && key.contains("objects") && *bindings > 0).expect("page 0 must close the objects section with a setPanelPage +N");
    assert!(rows.iter().any(|(key, _, _, _)| key == "object-0"), "page 0 starts at object-0: {rows:?}");
    assert!(!rows.iter().any(|(key, _, _, _)| *key == format!("object-{rows_per}")), "page 0 must not already show the next-page head");
    drop(first);
    drain_retired_ui_owners();
    let mut pages = BTreeMap::new();
    pages.insert(section.to_string(), 1);
    let second = super::render_from(&fixture, native, &pages).expect("page 1");
    rows.clear();
    walk(&second, &mut rows);
    assert!(rows.iter().any(|(key, _, _, _)| *key == format!("object-{rows_per}")), "page 1 must start at object-{rows_per}: {rows:?}");
    assert!(!rows.iter().any(|(key, _, _, _)| key == "object-0"), "page 1 must not keep page 0's head");
    drop(second);
    drain_retired_ui_owners();
    let last = (objects - 1) / rows_per;
    pages.insert(section.to_string(), last as u32);
    let tail = super::render_from(&fixture, native, &pages).expect("last page");
    rows.clear();
    walk(&tail, &mut rows);
    assert_eq!(rows.iter().filter(|(key, _, _, _)| key.ends_with(".more") && key.contains("objects")).count(), 0, "the last objects page must drop +N: {rows:?}");
    drop(tail);
    drain_retired_ui_owners();
}


/// 🧱 Wave W-X §6: Nakagin's host artifact panel must stay inside the 16-node reconcile envelope.
/// 18 presented nodes credited 8.42 MiB and refused `framework.panel.artifact` after the switch.
#[test]
fn nakagin_artifact_panel_fits_reconcile_node_envelope() {
    let _page = page_guard();
    let native = labels_for(semio_framework_plugin::Terminology::Native);
    drain_retired_ui_owners();
    let fixture = crate::editor::puzzle3d::nakagin_fixture();
    let page = super::render(&fixture, native).expect("the Nakagin outliner page must be admitted");
    fn count(node: &super::BuiltNode) -> usize {
        1 + node.children.iter().map(count).sum::<usize>()
    }
    let nodes = count(&page);
    assert!(nodes <= super::PANEL_RECONCILE_NODE_BUDGET, "Nakagin artifact panel presented {nodes} nodes over the host reconcile envelope");
    eprintln!("[DEBUG] Nakagin artifact panel presented {nodes} nodes (budget {})", super::PANEL_RECONCILE_NODE_BUDGET);
    drop(page);
    drain_retired_ui_owners();
}
