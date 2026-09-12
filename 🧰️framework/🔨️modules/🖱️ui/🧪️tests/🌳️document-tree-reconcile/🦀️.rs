//! 🌳️ LAW: a published `UiDocumentTree` becomes a paintable arena, and painting it emits draw work.
//!
//! `UiTree::publish_document` stored the document in a field whose only readers were a generation
//! comparison and its own retirement — nothing ever walked its `UiNodeRecord`s into the arena, so
//! `UiTree::root` stayed `None`, `Ui::frame_into_step` answered `Missing` on its first line, and this
//! target's window bodies had never painted a single quad (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
//! `📓️wgpu-blank-paint-2026-09-12.md` §5 — the arena's ONLY writer, `Ui::apply_tree`, is
//! `cfg(test/testkit)`).
//!
//! This file pins the missing edge against the LIVE arena, with the neutral oracle
//! `🖱️ui/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json` as the shared fixture (its TypeScript twin is
//! `📺️renderer/🧑‍🎨engine/🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts`).

use super::*;
use crate::wgpu::engine::{Ui, UiLayoutStep};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::tree::UiTree;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiRevision};

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🌳️document-tree-reconcile/🔣️.json")).expect("document tree reconcile fixture")
}

fn record(value: &serde_json::Value) -> UiNodeRecord {
    serde_json::from_value(value.clone()).expect("fixture record deserializes against the contract")
}

fn document_from(law: &serde_json::Value, ids: &[u64], generation: u64, revision: u64, extra: Option<&serde_json::Value>) -> UiDocumentTree {
    let source = &law["document"];
    let header = UiDocumentLeaseHeader {
        generation,
        surface: SurfaceId::try_from(source["surface"].as_str().expect("fixture surface")).expect("fixture surface id"),
        revision: UiRevision(revision),
        root: UiNodeId(source["root"].as_u64().expect("fixture root")),
        layout_epoch: source["layoutEpoch"].as_u64().expect("fixture layout epoch"),
        node_count: ids.len() + usize::from(extra.is_some()),
    };
    let mut document = UiDocumentTree::new(header).expect("fixture header admits");
    for node in source["nodes"].as_array().expect("fixture nodes") {
        let id = node["id"].as_u64().expect("fixture node id");
        if !ids.contains(&id) {
            continue;
        }
        let mut node = node.clone();
        if let Some(children) = node["children"].as_array().cloned() {
            node["children"] = serde_json::Value::Array(children.into_iter().filter(|child| ids.contains(&child.as_u64().unwrap_or(u64::MAX))).collect());
        }
        document.try_upsert_record(record(&node)).expect("fixture record admits");
    }
    if let Some(extra) = extra {
        document.try_upsert_record(record(extra)).expect("fixture addition admits");
    }
    document
}

fn reconcile(tree: &mut UiTree, cursor: &mut UiDocumentReconcileCursor, generation: u64) -> UiDocumentReconcileStep {
    cursor.rearm(generation);
    for _ in 0..4096 {
        match tree.step_document_reconcile(cursor, "procedural-main", "generation3d") {
            UiDocumentReconcileStep::Pending => {}
            terminal => return terminal,
        }
    }
    panic!("document reconcile did not terminate inside its own node budget");
}

fn tree_order(tree: &UiTree) -> Vec<String> {
    let mut order = Vec::new();
    let Some(root) = tree.root else { return order };
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let Some(node) = tree.node(id) else { continue };
        order.push(match &node.key {
            NodeKey::Explicit(key) => key.clone(),
            NodeKey::Positional(variant, ordinal) => format!("#{variant}:{ordinal}"),
        });
        let children: Vec<_> = tree.children(id).collect();
        for child in children.into_iter().rev() {
            stack.push(child);
        }
    }
    order
}

fn test_clock() -> Option<u64> {
    Some(0)
}

/// 🎬️ A scene host that paints nothing — this law measures the DOCUMENT's own quads and glyphs, so
/// the surface leaf deliberately falls through to `paint`'s own placeholder chrome instead of
/// borrowing a host's fill. Passed as the type parameter only; every call site hands `None`.
struct NoSceneHost;

impl crate::wgpu::scene_slots::SceneHost for NoSceneHost {
    fn paint_slot_step(
        &mut self,
        _slot: &crate::wgpu::scene_slots::SceneSlot<'_>,
        cursor: &mut crate::wgpu::scene_slots::ScenePaintCursor,
        _draw: &mut crate::wgpu::draw::DrawList,
        _atlas: &mut FontAtlas,
        _icons: Option<&crate::wgpu::draw::IconAtlas>,
    ) -> crate::wgpu::scene_slots::ScenePaintStep {
        cursor.finish()
    }
}

fn drive_layout(ui: &mut Ui, window_id: &str, width: f32, height: f32, atlas: &mut FontAtlas) {
    ui.set_viewport(window_id, width, height);
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let mut preview_sequence = 0;
    for _ in 0..200_000 {
        let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
        if matches!(ui.step_layouts(&pool, atlas, &mut cx), UiLayoutStep::Idle) {
            return;
        }
    }
    panic!("layout did not settle");
}

/// 🌳️ The headline: the published document mounts, its surface consumes its own lane subtree, and the
/// arena it produces paints real instances — the exact chain that answered `Missing` before.
#[test]
fn a_published_document_mounts_its_records_and_the_arena_it_produces_paints() {
    let law = law();
    let ids: Vec<u64> = law["document"]["nodes"].as_array().expect("fixture nodes").iter().map(|node| node["id"].as_u64().expect("fixture id")).collect();
    let document = document_from(&law, &ids, law["document"]["generation"].as_u64().expect("generation"), law["document"]["revision"].as_u64().expect("revision"), None);

    let mut tree = UiTree::new();
    assert!(tree.root.is_none(), "an unreconciled tree has no paintable root — the defect this law pins");
    tree.publish_document(document);
    assert!(tree.root.is_none(), "publishing alone must not mount anything; the reconcile is a separate, budgeted pass");

    let mut cursor = UiDocumentReconcileCursor::default();
    assert_eq!(reconcile(&mut tree, &mut cursor, law["document"]["generation"].as_u64().expect("generation")), UiDocumentReconcileStep::Complete);

    let expected_keys: Vec<String> = law["expected"]["mountedKeysInTreeOrder"].as_array().expect("expected keys").iter().map(|key| key.as_str().expect("key").to_string()).collect();
    assert_eq!(tree_order(&tree), expected_keys, "every mounted record, in document order, and nothing under the surface");
    assert_eq!(tree_order(&tree).len(), law["expected"]["arenaNodeCount"].as_u64().expect("count") as usize);

    for skipped in law["expected"]["skippedIds"].as_array().expect("skipped ids") {
        assert!(tree.document_node(UiNodeId(skipped.as_u64().expect("skipped id"))).is_none(), "a surface's lane carrier must never become a paintable node");
    }

    let surface_id = UiNodeId(law["expected"]["surfaceNode"]["id"].as_u64().expect("surface id"));
    let surface_node = tree.document_node(surface_id).and_then(|node| tree.node(node)).expect("the surface record mounted");
    let UiNode::ComponentScene(scene) = &surface_node.spec.0 else { panic!("a Component::Surface record must project onto a ComponentScene node") };
    assert_eq!(scene.component_kind, SurfaceKind::NodeGraph);
    assert_eq!(scene.surface_id, law["expected"]["surfaceNode"]["surfaceId"].as_str().expect("surface"), "surfaceId is the OWNING DOCUMENT's surface — React's surfaceHostIdentityV1");
    assert_eq!(scene.pane_id.as_deref(), law["expected"]["surfaceNode"]["paneId"].as_str(), "paneId carries the program's own authored surface id");
    assert_eq!(scene.controller_id, law["expected"]["surfaceNode"]["controllerId"].as_str().expect("controller"));
    assert!(scene.node_graph.is_none(), "an empty doc payload renders the host's own empty viewport rather than refusing the record");

    // 🖌️ …and the arena that came out of it actually paints.
    let mut ui = Ui::new();
    let document = document_from(&law, &ids, law["document"]["generation"].as_u64().expect("generation"), law["document"]["revision"].as_u64().expect("revision"), None);
    assert!(ui.publish_document("procedural-main", document));
    let mut cx_sequence = 0;
    let mut cx = semio_framework_job::StepContext::new(
        semio_framework_job::allocate_operation_id(),
        semio_framework_job::Generation(law["document"]["generation"].as_u64().expect("generation")),
        semio_framework_job::StepBudget::new(4096, u64::MAX),
        semio_framework_job::CancelToken::root_now(),
        test_clock,
        &mut cx_sequence,
    );
    assert_eq!(ui.step_document_reconcile("procedural-main", "generation3d", &mut cx), UiDocumentReconcileStep::Complete);
    assert!(ui.tree("procedural-main").and_then(|tree| tree.root).is_some(), "the engine's own window tree now has a root");

    let mut atlas = FontAtlas::builtin();
    drive_layout(&mut ui, "procedural-main", 1200.0, 800.0, &mut atlas);
    let draw = ui.frame::<NoSceneHost>("procedural-main", 1200.0, 800.0, &mut atlas, None, None).expect("a mounted document paints");
    let layers = draw.layers.iter().filter(|layer| !layer.ui_instances.is_empty() || !layer.vector_vertices.is_empty() || !layer.raster_instances.is_empty()).count();
    let instances: usize = draw.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    let glyphs: usize = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.params[2] == crate::wgpu::draw::KIND_GLYPH).count();
    assert!(layers >= law["expected"]["paint"]["drawCallsAtLeast"].as_u64().expect("draw calls") as usize, "expected at least one non-empty draw layer, got {layers}");
    assert!(instances >= law["expected"]["paint"]["uiInstancesAtLeast"].as_u64().expect("instances") as usize, "expected painted quads, got {instances}");
    assert!(glyphs >= law["expected"]["paint"]["glyphInstancesAtLeast"].as_u64().expect("glyphs") as usize, "expected shaped glyphs, got {glyphs}");
}

/// 🪪️ Identity across generations: a record that survives keeps its arena slot (and therefore its
/// widget state), a record that leaves frees exactly its own slot, and the new order is the document's.
#[test]
fn a_second_generation_preserves_surviving_identities_and_retires_only_what_left() {
    let law = law();
    let ids: Vec<u64> = law["document"]["nodes"].as_array().expect("fixture nodes").iter().map(|node| node["id"].as_u64().expect("fixture id")).collect();
    let mut tree = UiTree::new();
    let mut cursor = UiDocumentReconcileCursor::default();
    tree.publish_document(document_from(&law, &ids, 7, 3, None));
    assert_eq!(reconcile(&mut tree, &mut cursor, 7), UiDocumentReconcileStep::Complete);

    let before: Vec<(u64, Option<NodeId>)> = law["secondGeneration"]["expected"]["preservedIds"]
        .as_array()
        .expect("preserved ids")
        .iter()
        .map(|id| {
            let id = id.as_u64().expect("preserved id");
            (id, tree.document_node(UiNodeId(id)))
        })
        .collect();
    assert!(before.iter().all(|(_, node)| node.is_some()), "every id the second generation keeps mounted in the first");

    let removed: Vec<u64> = law["secondGeneration"]["removedIds"].as_array().expect("removed ids").iter().map(|id| id.as_u64().expect("removed id")).collect();
    let retired_node = tree.document_node(UiNodeId(removed[0])).expect("the retired record was mounted");
    let survivors: Vec<u64> = ids.iter().copied().filter(|id| !removed.contains(id)).collect();
    let addition = law["secondGeneration"]["addedNode"].clone();
    let mut second = document_from(&law, &survivors, 8, 4, Some(&addition));
    let parent = UiNodeId(1);
    let added = UiNodeId(addition["id"].as_u64().expect("added id"));
    second.try_reparent(added, parent).expect("the addition joins the outline column");
    tree.publish_document(second);
    assert_eq!(reconcile(&mut tree, &mut cursor, 8), UiDocumentReconcileStep::Complete);

    for (id, node) in &before {
        assert_eq!(tree.document_node(UiNodeId(*id)), *node, "record {id} survived the generation and must keep its exact arena slot");
    }
    assert!(tree.document_node(UiNodeId(removed[0])).is_none(), "a record that left the document loses its binding");
    assert!(!tree.contains(retired_node), "…and its arena slot is freed, not leaked");
    assert_eq!(tree_order(&tree).len(), law["secondGeneration"]["expected"]["arenaNodeCount"].as_u64().expect("count") as usize);
    assert!(tree.document_node(added).is_some(), "the new record mounted");
}

/// 🧹️ Retirement: dropping the document frees every arena slot it minted, one step at a time.
#[test]
fn retiring_a_document_frees_every_node_it_mounted_one_step_at_a_time() {
    let law = law();
    let ids: Vec<u64> = law["document"]["nodes"].as_array().expect("fixture nodes").iter().map(|node| node["id"].as_u64().expect("fixture id")).collect();
    let mut tree = UiTree::new();
    let mut cursor = UiDocumentReconcileCursor::default();
    tree.publish_document(document_from(&law, &ids, 7, 3, None));
    assert_eq!(reconcile(&mut tree, &mut cursor, 7), UiDocumentReconcileStep::Complete);
    let mounted = law["expected"]["arenaNodeCount"].as_u64().expect("count") as usize;

    let mut steps = 0;
    while !tree.close_document_binding_step() {
        steps += 1;
        assert!(steps <= mounted, "retirement must free exactly the nodes it mounted, one per step");
    }
    assert_eq!(steps, mounted);
    assert!(tree.root.is_none(), "a fully retired document leaves no paintable root behind");
}
