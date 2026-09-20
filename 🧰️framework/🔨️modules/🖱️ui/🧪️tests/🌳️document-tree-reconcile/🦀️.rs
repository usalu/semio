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
use crate::wgpu::engine::{ui_document_ingress_generation, Ui, UiDocumentIngressFault, UiDocumentIngressStatus, UiFrameStep, UiLayoutStep};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::tree::UiTree;
use ui_contract::{SurfaceId, UiDocumentAssembly, UiDocumentAssemblyIdentity, UiDocumentLease, UiDocumentLeaseHeader, UiRevision};

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

/// 🎬️ A synthesized Select option is not itself a published record, so it dispatches through the
/// descriptor copied from its Select owner. That descriptor must retain the binding's action scope;
/// the document controller identifies the live app session and is deliberately different here.
#[test]
fn a_synthesized_select_option_keeps_the_published_binding_scope() {
    let law = law();
    let action_law = &law["actionScope"];
    let select_record = record(&action_law["selectRecord"]);
    let document = document_from(&law, &[], 7, 3, Some(&action_law["selectRecord"]));
    let projected = ui_node_from_record(
        &document,
        &select_record,
        law["document"]["surface"].as_str().expect("fixture surface"),
        action_law["documentController"].as_str().expect("fixture document controller"),
    );
    let UiNode::Select(select) = projected else { panic!("fixture Select projects as a retained Select") };
    assert_eq!(select.on_change.controller_id, action_law["bindingController"].as_str().expect("binding controller"));

    let chosen = action_law["chosenItem"].as_str().expect("chosen item");
    let item = select.items.iter().find(|item| item.value == chosen).expect("chosen Select item");
    let UiNode::Button(option) = select_item_row(&select, item) else { panic!("a Select option synthesizes a Button row") };
    let expected = &action_law["expectedAction"];
    assert_eq!(option.action.controller_id, expected["controllerId"].as_str().expect("expected controller"));
    assert_eq!(option.action.action, expected["action"].as_str().expect("expected action"));
    assert_eq!(
        option.action.args,
        Some(DslValue::Object(vec![
            ("windowId".into(), DslValue::String("framework.settings.general".into())),
            ("value".into(), DslValue::String(chosen.into())),
        ])),
        "the synthesized row preserves authored arguments and adds the chosen value",
    );
}

//#region 🪪️IngressGeneration
/// 📃️ Builds one real [`UiDocumentLease`] the way the BROWSER producer does — stepped
/// `open_into`/`place_one`/`finish_into` — so the laws below can drive the engine's own page ingress
/// instead of the testkit's `Ui::publish_document` shortcut. That shortcut is exactly why the defect
/// these laws pin survived every earlier reconcile law: it writes the tree directly and never
/// consults `document_status`/`begin_document`, the two rules a producer's generation has to satisfy.
fn lease_from(law: &serde_json::Value, ids: &[u64], generation: u64, revision: u64, extra: Option<&serde_json::Value>) -> UiDocumentLease {
    let mut present: Vec<u64> = ids.to_vec();
    if let Some(extra) = extra {
        present.push(extra["id"].as_u64().expect("fixture addition id"));
    }
    let mut records: Vec<UiNodeRecord> = Vec::new();
    for node in law["document"]["nodes"].as_array().expect("fixture nodes") {
        let id = node["id"].as_u64().expect("fixture node id");
        if !present.contains(&id) {
            continue;
        }
        let mut node = node.clone();
        if let Some(children) = node["children"].as_array().cloned() {
            let mut kept: Vec<serde_json::Value> = children.into_iter().filter(|child| present.contains(&child.as_u64().unwrap_or(u64::MAX))).collect();
            if let Some(extra) = extra.filter(|_| id == 1) {
                kept.push(serde_json::Value::from(extra["id"].as_u64().expect("fixture addition id")));
            }
            node["children"] = serde_json::Value::Array(kept);
        }
        records.push(record(&node));
    }
    if let Some(extra) = extra {
        records.push(record(extra));
    }
    let mut owner = UiDocumentAssembly::default();
    let mut surface = Some(SurfaceId::try_from(law["document"]["surface"].as_str().expect("fixture surface")).expect("fixture surface id"));
    let identity =
        UiDocumentAssemblyIdentity { generation, revision: UiRevision(revision), root: Some(UiNodeId(law["document"]["root"].as_u64().expect("fixture root"))), layout_epoch: law["document"]["layoutEpoch"].as_u64().expect("fixture layout epoch") };
    for _ in 0..4096 {
        if owner.open_into(&mut surface, identity, 1, 32768).expect("assembly open admits").progressed && surface.is_none() {
            break;
        }
    }
    assert!(surface.is_none(), "the assembly claimed the exact surface identity");
    for source in records {
        let mut source = Some(source);
        for _ in 0..4096 {
            owner.place_one(&mut source, 1, 32768).expect("record placement admits");
            if source.is_none() {
                break;
            }
        }
        assert!(source.is_none(), "every fixture record is placed within its own budget");
    }
    let mut lease = None;
    for _ in 0..4096 {
        if owner.finish_into(&mut lease, UiRevision(revision), 1, 32768).expect("assembly finish admits").complete {
            break;
        }
    }
    lease.expect("the assembly published a lease")
}

/// 📥️ The shell's own ingress ladder in one call — `document_status` → `begin_document` →
/// `read_node_page`/`apply_document_page` → `finish_document` → `step_document_reconcile`, the exact
/// order `render_ui_document_step` walks. Answers whether the document was ADMITTED at all.
fn ingest(ui: &mut Ui, window_id: &str, lease: &UiDocumentLease) -> bool {
    let header = lease.header().expect("the lease publishes its header");
    let generation = header.generation;
    let mut admitted = false;
    for _ in 0..65_536 {
        let mut sequence = 0;
        let mut cx = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(generation),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(4096, u64::MAX),
            semio_framework_job::CancelToken::root_now(),
            test_clock,
            &mut sequence,
        );
        match ui.document_status(window_id, generation) {
            UiDocumentIngressStatus::Vacant => {
                if ui.begin_document(window_id, header.clone(), &mut cx).is_err() {
                    return admitted;
                }
                admitted = true;
            }
            // 🚦️ `ValidationPending` is the stepped validator asking for another opportunity, never a
            // refusal — `render_ui_document_step` retries on exactly this set.
            UiDocumentIngressStatus::Pending { next_page, node_count } if next_page == node_count => match ui.finish_document(window_id, generation, &mut cx) {
                Ok(()) | Err(UiDocumentIngressFault::ValidationPending | UiDocumentIngressFault::InterruptedClose | UiDocumentIngressFault::Cancelled | UiDocumentIngressFault::Deadline) => {}
                Err(fault) => panic!("a fully paged document finishes: {fault:?}"),
            },
            UiDocumentIngressStatus::Pending { next_page, .. } => {
                let page = lease.read_node_page(next_page).expect("the lease reads its own page").expect("the page exists");
                ui.apply_document_page(window_id, page, &mut cx).expect("a fixture page applies");
            }
            UiDocumentIngressStatus::Published => {
                if matches!(ui.step_document_reconcile(window_id, "generation3d", &mut cx), UiDocumentReconcileStep::Complete) {
                    return admitted;
                }
            }
        }
    }
    panic!("the ingress ladder did not terminate inside its own budget");
}

/// 🗝️ Every key the engine's arena currently paints for `window_id`.
fn arena_keys(ui: &Ui, window_id: &str) -> Vec<String> {
    ui.tree(window_id).map(tree_order).unwrap_or_default()
}

/// ♻️ Returns one lease's arena slot. `UI_DOCUMENT_LEASE_SLOTS` is a FIXED process-wide table shared
/// by every law in this binary, so a law that holds two leases at once is answered `ArenaFull` rather
/// than measuring anything — each document is ingested and then given straight back.
fn retire_lease(mut lease: UiDocumentLease) {
    for _ in 0..100_000 {
        if lease.close_read_step_with_grant(1, 32768).expect("exact retirement authority").complete {
            return;
        }
    }
    panic!("document lease did not retire");
}

/// 📥️ Assembles one document, drives it through the engine's ingress ladder and returns the slot —
/// the whole "one refresh publishes one document" cycle a shell frame performs.
fn ingest_once(ui: &mut Ui, window_id: &str, law: &serde_json::Value, ids: &[u64], generation: u64, revision: u64, extra: Option<&serde_json::Value>) -> bool {
    let lease = lease_from(law, ids, generation, revision, extra);
    let admitted = ingest(ui, window_id, &lease);
    retire_lease(lease);
    admitted
}

/// ⚖️ LAW: a surface's SECOND document reaches the arena, and a producer that mints ONE CONSTANT
/// generation for every document never gets a second one in at all.
///
/// 🩸️ This is the defect behind "`Add Generation` on 6118 settles and nothing appears". The browser
/// producer minted `u64::from(instance_id)` — the plugin instance id, a session constant identical
/// for every surface — so the first document each surface ingested published at generation 1, and
/// from then on `document_status` answered `Published` for every later document of that surface: the
/// ingress phase short-circuited to the reconcile, which re-reconciled the SAME tree, forever. The
/// guest re-rendered correctly and the bridge projected the new tree correctly (measured on 6118: the
/// generations body moved from 5 nodes/rev 1 to 6 nodes/rev 2), and not one of those nodes could
/// reach the screen (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-generation-publication-2026-09-13.md`).
#[test]
fn a_surfaces_second_document_reaches_the_arena_and_a_constant_generation_never_does() {
    let law = law();
    let ids: Vec<u64> = law["document"]["nodes"].as_array().expect("fixture nodes").iter().map(|node| node["id"].as_u64().expect("fixture id")).collect();
    let first_revision = law["document"]["revision"].as_u64().expect("fixture revision");
    let second_revision = law["secondGeneration"]["revision"].as_u64().expect("fixture second revision");
    let addition = law["secondGeneration"]["addedNode"].clone();
    let added_key = addition["key"].as_str().expect("fixture addition key").to_string();
    let removed: Vec<u64> = law["secondGeneration"]["removedIds"].as_array().expect("removed ids").iter().map(|id| id.as_u64().expect("removed id")).collect();
    let survivors: Vec<u64> = ids.iter().copied().filter(|id| !removed.contains(id)).collect();

    let minted_first = ui_document_ingress_generation(None, first_revision);
    let minted_second = ui_document_ingress_generation(Some((first_revision, minted_first)), second_revision);
    assert!(minted_second > minted_first, "a republished surface must mint a STRICTLY greater ingress generation");

    let mut ui = Ui::new();
    assert!(ingest_once(&mut ui, "second-document", &law, &ids, minted_first, first_revision, None), "the first document is admitted");
    assert!(!arena_keys(&ui, "second-document").contains(&added_key), "the first document does not carry the addition");

    assert!(ingest_once(&mut ui, "second-document", &law, &survivors, minted_second, second_revision, Some(&addition)), "the second document is admitted");
    assert!(arena_keys(&ui, "second-document").contains(&added_key), "the SECOND document's new node must reach the paintable arena: {:?}", arena_keys(&ui, "second-document"));
    assert_eq!(arena_keys(&ui, "second-document").len(), law["secondGeneration"]["expected"]["arenaNodeCount"].as_u64().expect("count") as usize);

    // 🩸️ …and the producer the browser actually shipped: the same constant for both documents.
    let constant = law["ingressGeneration"]["constantProducerIsRefused"]["generations"].as_array().expect("constant generations").iter().map(|value| value.as_u64().expect("generation")).collect::<Vec<_>>();
    assert!(constant.windows(2).all(|pair| pair[0] == pair[1]), "the shipped producer's generations were all equal — that is the defect");
    let mut refused = Ui::new();
    assert!(ingest_once(&mut refused, "constant", &law, &ids, constant[0], first_revision, None), "its first document is admitted");
    assert!(!ingest_once(&mut refused, "constant", &law, &survivors, constant[1], second_revision, Some(&addition)), "a repeat generation is never ADMITTED — `document_status` answers Published and the ingress is skipped");
    assert!(!arena_keys(&refused, "constant").contains(&added_key), "…so the arena keeps the first tree, which is exactly what 6118 painted");
    eprintln!("[DEBUG] ingress generations minted={minted_first}/{minted_second} constant={constant:?}");
}

/// ⚖️ LAW: the ingress-generation rule itself — held still while a surface's own revision holds
/// still (so an unchanged surface pays no ingress), moved forward whenever that revision moves, and
/// never moved BACKWARD even when the revision does (a retired surface reopened under a fresh owner
/// restarts at revision 1 and must still be admitted). Shared fixture with the TypeScript twin.
#[test]
fn the_ingress_generation_holds_still_while_a_surface_does_and_never_moves_backward() {
    let law = law();
    let mut previous: Option<u64> = None;
    for case in law["ingressGeneration"]["cases"].as_array().expect("ingress generation cases") {
        let minted = case["minted"].as_array().map(|pair| (pair[0].as_u64().expect("minted revision"), pair[1].as_u64().expect("minted generation")));
        let revision = case["revision"].as_u64().expect("case revision");
        let generation = ui_document_ingress_generation(minted, revision);
        assert_eq!(generation, case["generation"].as_u64().expect("case generation"), "case {}", case["label"].as_str().unwrap_or_default());
        assert!(generation > 0, "a zero generation is refused by `UiDocumentTree::new` outright");
        if let Some(previous) = previous {
            assert!(generation >= previous, "the rule never moves a surface's generation backward");
        }
        previous = Some(generation);
    }
}
//#endregion 🪪️IngressGeneration

//#region 🎬️IntentStamping
// 🎬️ LAW: mounting a record stamps the arena node with the dispatch contract its gestures fire
// through — the surface/revision/node/nodeKey address plus every `(trigger, ActionId)` it binds, the
// exact inputs React's `UiDocumentStore::buildIntent` reads off its own store and record
// (`📃️UiDocumentStore/🟦️.tsx`). Without it the wgpu target had nothing but a stringly
// `ActionDescriptor` and could neither drop a stale gesture nor order two of them.

fn button_record(id: u64, key: &str, action: &str, version: u64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "key": key,
        "component": { "type": "button", "icon": "circle-dot", "label": "Go" },
        "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": {},
        "bindings": [{ "trigger": "activate", "action": { "scope": "ctrl", "name": action, "version": version }, "args": { "windowId": "w1" } }],
        "children": []
    })
}

fn extension_record(id: u64, key: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "key": key,
        "component": { "type": "extension", "extension": "body.inspector", "props": null },
        "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": {},
        "children": []
    })
}

/// 🌳️ A one-record-deep document rooted at `root`, so a law can pin projection without the whole
/// fixture tree — `document_from`'s `extra` slot only ever appends a LEAF.
fn tiny_document(surface: &str, revision: u64, root: &serde_json::Value, child: Option<&serde_json::Value>) -> UiDocumentTree {
    let header = UiDocumentLeaseHeader {
        generation: 11,
        surface: SurfaceId::try_from(surface).expect("surface id"),
        revision: UiRevision(revision),
        root: UiNodeId(root["id"].as_u64().expect("root id")),
        layout_epoch: 1,
        node_count: 1 + usize::from(child.is_some()),
    };
    let mut document = UiDocumentTree::new(header).expect("header admits");
    document.try_upsert_record(record(root)).expect("root admits");
    if let Some(child) = child {
        document.try_upsert_record(record(child)).expect("child admits");
    }
    document
}

/// 🧹️ Hands every record and arena slot back before the test ends. The `UiNodeRecord` table draws on
/// a PROCESS-WIDE fixed arena, so a law that merely drops its tree leaves those pages held until the
/// allocator notices — and a neighbouring test running in parallel meets `ArenaFull` instead of its
/// own law. Retiring explicitly is the same rule the runtime itself follows.
fn retire(mut tree: UiTree) {
    while !tree.close_document_binding_step() {}
    if let Some(mut document) = tree.take_document() {
        while !document.close_step() {}
    }
}

fn container_record(id: u64, key: &str, children: &[u64]) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "key": key,
        "component": { "type": "container", "role": "plain" },
        "layout": { "kind": "stack", "axis": "vertical", "gap": "md", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "grow": true, "wrap": false },
        "style": {},
        "activity": "idle",
        "accessibility": {},
        "children": children
    })
}

#[test]
fn mounting_a_record_stamps_its_whole_dispatch_contract_onto_the_arena_node() {
    let mut tree = UiTree::new();
    let mut cursor = UiDocumentReconcileCursor::default();
    let root = container_record(0, "panel", &[1]);
    let button = button_record(1, "run", "runTool", 1);
    tree.publish_document(tiny_document("note.play.navigator", 5, &root, Some(&button)));
    cursor.rearm(11);
    for _ in 0..4096 {
        match tree.step_document_reconcile(&mut cursor, "note.play.navigator", "note") {
            UiDocumentReconcileStep::Pending => {}
            terminal => {
                assert_eq!(terminal, UiDocumentReconcileStep::Complete);
                break;
            }
        }
    }

    let node = tree.document_node(UiNodeId(1)).and_then(|node| tree.node(node)).expect("the button mounted");
    let intent = node.intent.as_ref().expect("a published record always carries its dispatch contract");
    assert_eq!(intent.address.surface, "note.play.navigator");
    assert_eq!(intent.address.revision, 5, "the address carries the revision the gesture will be judged stale against");
    assert_eq!(intent.address.node, 1);
    assert_eq!(intent.address.node_key, "run", "the key survives id churn from an intervening reconciliation — that is why it travels alongside the id");
    assert!(intent.binds(ui_contract::Trigger::Activate));
    assert!(!intent.binds(ui_contract::Trigger::Delta), "binding PRESENCE is answerable — this is what gates a stepper's relative path");
    let action = intent.action_for(ui_contract::Trigger::Activate).expect("the bound action id");
    assert_eq!(action.scope.as_str(), "ctrl", "the versioned ActionId survives whole; the legacy descriptor kept only its name");
    assert_eq!(action.name.as_str(), "runTool");
    assert_eq!(action.version, 1);
    retire(tree);
}

#[test]
fn a_new_revision_restamps_every_surviving_node_so_its_own_gestures_never_read_stale() {
    let mut tree = UiTree::new();
    let mut cursor = UiDocumentReconcileCursor::default();
    let root = container_record(0, "panel", &[1]);
    let button = button_record(1, "run", "runTool", 1);
    let drive = |tree: &mut UiTree, cursor: &mut UiDocumentReconcileCursor| {
        cursor.rearm(11);
        for _ in 0..4096 {
            match tree.step_document_reconcile(cursor, "note.play.navigator", "note") {
                UiDocumentReconcileStep::Pending => {}
                terminal => return terminal,
            }
        }
        panic!("reconcile did not terminate");
    };
    tree.publish_document(tiny_document("note.play.navigator", 5, &root, Some(&button)));
    assert_eq!(drive(&mut tree, &mut cursor), UiDocumentReconcileStep::Complete);
    let slot = tree.document_node(UiNodeId(1)).expect("mounted");

    tree.publish_document(tiny_document("note.play.navigator", 9, &root, Some(&button)));
    assert_eq!(drive(&mut tree, &mut cursor), UiDocumentReconcileStep::Complete);

    assert_eq!(tree.document_node(UiNodeId(1)), Some(slot), "the identity survives the revision");
    let intent = tree.node(slot).and_then(|node| node.intent.as_ref()).expect("still stamped");
    assert_eq!(intent.address.revision, 9, "a node left at its old revision would start refusing its own live gestures as stale");
    retire(tree);
}

#[test]
fn an_extension_slot_carries_its_publishers_plugin_id_instead_of_an_empty_string() {
    let mut tree = UiTree::new();
    let mut cursor = UiDocumentReconcileCursor::default();
    let root = container_record(0, "panel", &[1]);
    let extension = extension_record(1, "inspector-slot");
    tree.publish_document(tiny_document("note.play.navigator", 2, &root, Some(&extension)));
    cursor.rearm(11);
    for _ in 0..4096 {
        match tree.step_document_reconcile(&mut cursor, "note.play.navigator", "note") {
            UiDocumentReconcileStep::Pending => {}
            terminal => {
                assert_eq!(terminal, UiDocumentReconcileStep::Complete);
                break;
            }
        }
    }

    let node = tree.document_node(UiNodeId(1)).and_then(|node| tree.node(node)).expect("the extension mounted");
    let crate::wgpu::component::ui::UiNode::ExternalSlot(slot) = &node.spec.0 else { panic!("an extension projects to an external slot, got {:?}", node.spec.0) };
    assert_eq!(slot.plugin_id, "note", "a surface id's first segment IS its owning plugin — an empty id would compose empty ids into everything built from it");
    assert_eq!(slot.app_id, "note");
    assert_eq!(slot.body_key, "body.inspector");
    retire(tree);
}
//#endregion 🎬️IntentStamping

//#region 🗺️EngineSurfaceRootDocument
/// 🗺️ The document shape gis2d publishes: ONE record, and that record IS the engine surface
/// (`componentScene#gis2d-main`, `nodes=1`). Every other wgpu playground wraps its surface in at
/// least one container, so this shape had no law at all.
fn engine_surface_root_record(id: u64, key: &str, kind: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "key": key,
        // 📐️ A root engine surface owns the viewport in both axes. `LeafLayout` requires both
        // sizing fields on the wire; the renderer contract has no compatibility defaults.
        "layout": { "kind": "leaf", "width": "fill", "height": "fill" },
        "component": { "type": "surface", "kind": kind, "docSchema": "tiled-map@1", "doc": { "bytes": [] } },
        "style": {},
        "activity": "idle",
        "accessibility": {},
        "children": [],
    })
}

/// 🚦️ The wgpu shell's OWN per-window loop, bounded: layout first, paint only when this window's own
/// root is clean, and a `Pending` that finds the root dirty again goes back to layout — the exact
/// ladder `render_ui_document_step` walks (`🗣️Interpreter/🎯️targets/🧊️wgpu`'s `UiDocumentFramePhase`).
/// Answers the number of opportunities the window spent, or `None` if it never terminated.
fn drive_window_to_painted(ui: &mut Ui, window_id: &str, viewport: crate::wgpu::geometry::Rect, atlas: &mut FontAtlas, budget: usize) -> Option<usize> {
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut target = crate::wgpu::draw::DrawList::default();
    for spent in 0..budget {
        if ui.layout_is_dirty(window_id) {
            ui.request_layout(window_id);
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), test_clock, &mut preview_sequence);
            let _ = ui.step_layouts(&pool, atlas, &mut cx);
            continue;
        }
        match ui.frame_into_step::<NoSceneHost>(window_id, viewport, atlas, None, None, &mut target) {
            UiFrameStep::Ready | UiFrameStep::Missing => return Some(spent),
            UiFrameStep::Pending => {}
            UiFrameStep::Fault => panic!("the window's retained paint faulted at {}", ui.paint_stall_census(window_id)),
        }
    }
    None
}

/// 🗺️ LAW: a document whose ROOT is the engine surface itself reaches a painted frame.
///
/// 🩸️ What this pins: `FrameBuildPhase::Chrome` holds the WHOLE frame — navbar, dock, footer, every
/// window — until the shell's chrome walk answers `true`, and that walk cannot leave a window whose
/// retained document never reports itself complete. gis2d on wgpu booted `data-semio-os-ready`, laid
/// `componentScene#gis2d-main` out at the full `1433.6 × 813.6` viewport, fetched its first map tile
/// — and presented a uniformly empty canvas, with no fault, no paint-stall notice and a chrome
/// ledger stuck at `generation: 0` (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w14a-*`).
#[test]
fn a_document_whose_root_is_the_engine_surface_reaches_a_painted_frame() {
    let mut ui = Ui::new();
    let root = engine_surface_root_record(0, "gis2d.play.composite", "tiled-map");
    assert!(ui.publish_document("gis2d-main", tiny_document("gis2d-main", 1, &root, None)));
    let mut cx_sequence = 0;
    let mut cx = semio_framework_job::StepContext::new(
        semio_framework_job::allocate_operation_id(),
        semio_framework_job::Generation(11),
        semio_framework_job::StepBudget::new(4096, u64::MAX),
        semio_framework_job::CancelToken::root_now(),
        test_clock,
        &mut cx_sequence,
    );
    assert_eq!(ui.step_document_reconcile("gis2d-main", "gis", &mut cx), UiDocumentReconcileStep::Complete);
    assert!(ui.tree("gis2d-main").and_then(|tree| tree.root).is_some(), "the single surface record mounts as the arena root");

    let mut atlas = FontAtlas::builtin();
    let viewport = crate::wgpu::geometry::Rect { x: 0.0, y: 0.0, w: 1433.6, h: 813.6 };
    let spent = drive_window_to_painted(&mut ui, "gis2d-main", viewport, &mut atlas, 1 << 16);
    assert!(spent.is_some(), "the window never finished one paint in 65 536 opportunities — the shell's chrome walk can never leave it: {}", ui.paint_stall_census("gis2d-main"));

    let rect = ui.tree("gis2d-main").and_then(|tree| tree.root.and_then(|root| tree.accepted_layout(root))).expect("the root solved a layout");
    assert!(rect.width > 1000.0 && rect.height > 600.0, "an engine-surface ROOT fills the viewport it is given, got {rect:?}");
}
//#endregion 🗺️EngineSurfaceRootDocument
