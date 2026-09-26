use super::*;

#[cfg(test)]
#[path = "../../📏️ownership/🧪️tests/📏️ownership/🦀️.rs"]
mod ownership;

#[cfg(test)]
#[path = "../../📃️document/🧪️tests/📃️document/🦀️.rs"]
mod canonical_document_tests;

#[cfg(test)]
#[path = "../../📤️output/🧪️tests/📤️output/🦀️.rs"]
mod output_pool_tests;

//#region 🔖️Fixtures
fn ui_text(value: &str) -> ui_contract::UiText {
    ui_contract::UiText::try_from_str(value).expect("bounded fixture text")
}

fn leaf(key: &str) -> crate::TreeNode {
    crate::TreeNode::try_new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {})).expect("bounded fixture node")
}

fn text(key: &str, value: &str) -> crate::TreeNode {
    crate::TreeNode::try_new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from(value).expect("bounded fixture label"), emphasize: None, data_attributes: None })).expect("bounded fixture node")
}

fn container(key: &str, children: Vec<crate::TreeNode>) -> crate::TreeNode {
    let node =
        crate::TreeNode::try_new(key, ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
            .expect("bounded fixture node");
    node.try_with_children(children).unwrap_or_else(|_| panic!("bounded fixture children"))
}

fn tree(root: crate::TreeNode) -> crate::ComponentTree {
    crate::ComponentTree::new(root)
}

fn ui_list(values: impl IntoIterator<Item = ui_contract::UiValue>) -> ui_contract::UiList {
    let mut builder = ui_contract::UiListBuilder::try_new().expect("fixed list builder");
    for value in values {
        builder.push(value).expect("fixed list page");
    }
    builder.finish()
}

fn ui_map(entries: impl IntoIterator<Item = (String, ui_contract::UiValue)>) -> ui_contract::UiMap {
    let mut builder = ui_contract::UiMapBuilder::try_new().expect("fixed map builder");
    for (key, value) in entries {
        builder.push(key, value).expect("ascending fixed map page");
    }
    builder.finish()
}

fn styled(node: crate::TreeNode, tone: ui_contract::Tone) -> crate::TreeNode {
    crate::TreeNode { style: ui_contract::StyleSpec { tone, ..Default::default() }, ..node }
}

fn with_shortcut(node: crate::TreeNode, shortcut: &str) -> crate::TreeNode {
    crate::TreeNode { accessibility: ui_contract::AccessibilitySpec { shortcut: Some(ui_text(shortcut)), ..Default::default() }, ..node }
}

fn with_binding(mut node: crate::TreeNode, scope: &str, name: &str) -> crate::TreeNode {
    node.bindings.try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::try_v1(scope, name).expect("bounded fixture action"), args: None, capability: None }).expect("bounded fixture bindings");
    node
}

fn with_menu(node: crate::TreeNode, menu_id: &str) -> crate::TreeNode {
    crate::TreeNode { menu: Some(ui_contract::MenuRef { id: ui_text(menu_id), args: None }), ..node }
}

fn id_of(snapshot: &ui_contract::UiSnapshot, key: &str) -> ui_contract::UiNodeId {
    snapshot.nodes.iter().find(|record| record.key.as_str() == key).unwrap_or_else(|| panic!("no node keyed {key:?} in snapshot")).id
}

fn assert_snapshot_matches_state(snapshot: &ui_contract::UiSnapshot, state: &ui_contract::UiSnapshotState) {
    assert_eq!(snapshot.revision, state.revision);
    assert_eq!(Some(snapshot.root), state.root);
    assert_eq!(snapshot.nodes.len(), state.nodes.len(), "snapshot/state node-count mismatch");
    for record in &snapshot.nodes {
        assert_eq!(state.nodes.get(&record.id), Some(record), "record {:?} diverges between snapshot and applied state", record.id);
    }
}

fn reconcile_resumable(current: &SurfaceReconciler, component_tree: crate::ComponentTree) -> (SurfaceReconciler, Option<SurfaceReconcileReadyPatch>, usize) {
    let mut cursor = SurfaceReconcileCursor::new(component_tree, current);
    let mut yields = 0;
    loop {
        match cursor.step(current) {
            SurfaceReconcileStep::Yield { .. } => yields += 1,
            SurfaceReconcileStep::Complete { mut reconciler, patch } => {
                let mut output = None;
                while !reconciler.seal_step(cursor.usage, &mut output, patch.is_some()).unwrap() {}
                let ready = patch.map(|patch| SurfaceReconcileReadyPatch { generation: cursor.assembly_generation, patch: pending_surface_patch(Some(patch)), credit: output, handback: reserve_surface_reconcile_handback(cursor.assembly_generation) });
                while !cursor.retire_one() {}
                return (reconciler, ready, yields);
            }
            SurfaceReconcileStep::Fault(fault) => panic!("unexpected reconcile fault: {fault:?}"),
        }
    }
}
//#endregion 🔖️Fixtures

//#region ⏭️ResumableCursor
#[test]
fn instance_lifetime_published_patch_close_retains_exact_handback_until_terminal() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🎭️actor/🚪️lifetime/🧫️fixtures/🔣️.json")).unwrap();
    let mut reservation = SurfaceReconcileReservation::try_new(8_971).expect("real published owner reservation");
    let metadata =
        pending_surface_patch(Some(ui_contract::UiPatch { surface: ui_contract::SurfaceId::try_from("7:retained").unwrap(), base_revision: ui_contract::UiRevision(0), revision: ui_contract::UiRevision(1), ops: ui_contract::UiPatchOps::default() }));
    let mut owner = SurfaceReconcilePublishedPatch { generation: 8_971, metadata, revision: ui_contract::UiRevision(1), credit: reservation.credit.take(), handback: reservation.handback.take() };
    let key = owner.handback.as_ref().unwrap().key;
    let mut remaining = Vec::new();
    let mut complete = Vec::new();
    let mut prior = vec!["metadata", "credit", "handback"];
    let mut turns = 0;
    let mut bytes = 0;
    assert_eq!(fixture["publishedClose"]["granularity"], "owner-transition");
    for _ in 0..100_000 {
        let step = owner.close_step_with_grant(1, 1).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= 1);
        turns += 1;
        bytes += step.released_bytes;
        let fields: Vec<_> = [("metadata", !owner.metadata.terminal_is_empty()), ("credit", owner.credit.is_some()), ("handback", owner.handback.is_some())].into_iter().filter_map(|(name, retained)| retained.then_some(name)).collect();
        if fields != prior || step.complete {
            prior = fields.clone();
            remaining.push(fields);
            complete.push(step.complete);
        }
        if step.complete {
            break;
        }
    }
    assert_eq!(bytes, "7:retained".len());
    assert_eq!(serde_json::to_value(&remaining).unwrap(), fixture["publishedClose"]["remaining"]);
    assert_eq!(serde_json::to_value(complete).unwrap(), fixture["publishedClose"]["complete"]);
    assert!(owner.terminal_is_empty());
    eprintln!("[DEBUG] published-close owner-transitions={} physical-turns={turns} semantic-bytes={bytes} grant=1", remaining.len());
    let registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap();
    let slot = &registry.slots[key.slot];
    assert!(slot.epoch != key.epoch || (!slot.reserved && slot.state.is_none()), "the exact handback was released, not queued and forgotten");
    drop(registry);
    drop(owner);
}

#[test]
fn fixed_runtime_owners_keep_bounded_state_off_the_stack() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    eprintln!("[DEBUG] canonical-owner-layout reconciler={} cursor={} retained={}", size_of::<SurfaceReconciler>(), size_of::<SurfaceReconcileCursor>(), size_of::<SurfaceReconcileRetained>());
    assert!(size_of::<SurfaceReconciler>() <= 1_024);
    assert!(size_of::<SurfaceReconcileCursor>() <= 48 * 1_024);
    assert!(size_of::<SurfaceReconcileRetained>() <= 64 * 1_024);
    assert!(size_of::<crate::TreeNode>() <= 8 * 1_024);
    assert!(size_of::<ui_contract::UiNodeRecord>() <= 8 * 1_024);
}

#[test]
fn resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let component_tree = tree(container("root", vec![text("a", "hello"), container("b", vec![leaf("x"), leaf("y")])]));
    let mut direct = SurfaceReconciler::new("s");
    let expected_patch = direct.reconcile(&component_tree).expect("initial direct patch");

    let current = SurfaceReconciler::new("s");
    let (resumed, mut actual_patch, yields) = reconcile_resumable(&current, component_tree);

    assert_eq!(actual_patch.as_ref().and_then(|patch| patch.patch.get()), Some(&expected_patch));
    if let Some(patch) = actual_patch.as_mut() {
        while !patch.close_step() {}
    }
    let actual = resumed.snapshot();
    let expected = direct.snapshot();
    assert_eq!(actual.revision, expected.revision);
    assert_eq!(actual.root, expected.root);
    assert_eq!(actual.nodes.len(), expected.nodes.len());
    assert!(expected.nodes.iter().all(|record| actual.nodes.iter().any(|candidate| candidate == record)));
    assert!(yields >= 15, "five nodes must cross traversal, identity, and diff cursors");
}

#[test]
fn abandoned_large_tree_cursor_leaves_the_retained_shadow_and_revision_unchanged() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut current = SurfaceReconciler::new("s");
    current.reconcile(&tree(container("root", vec![leaf("baseline")]))).expect("baseline");
    let before = current.snapshot();
    let children = (0..30).map(|index| leaf(&format!("item-{index}"))).collect();
    let mut cursor = SurfaceReconcileCursor::new(tree(container("root", children)), &current);

    assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
    drop(cursor);

    assert_eq!(current.snapshot(), before, "cancellation or supersession must discard only candidate state");
}

#[test]
fn every_large_tree_cursor_slice_stays_below_eight_milliseconds() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    use std::time::{Duration, Instant};

    let children = (0..30).map(|index| leaf(&format!("item-{index}"))).collect();
    let current = SurfaceReconciler::new("s");
    let mut cursor = SurfaceReconcileCursor::new(tree(container("root", children)), &current);
    let mut yields = 0;
    loop {
        let started = Instant::now();
        let step = cursor.step(&current);
        let elapsed = started.elapsed();
        assert!(elapsed < Duration::from_millis(8), "one node cursor slice took {elapsed:?}");
        match step {
            SurfaceReconcileStep::Yield { nodes, .. } => {
                assert!(nodes <= 1);
                yields += 1;
            }
            SurfaceReconcileStep::Complete { mut reconciler, patch } => {
                let mut output = None;
                loop {
                    let started = Instant::now();
                    let complete = reconciler.seal_step(cursor.usage, &mut output, patch.is_some()).unwrap();
                    assert!(started.elapsed() < Duration::from_millis(8), "canonical seal exceeds one callback contract");
                    if complete {
                        break;
                    }
                }
                assert_eq!(reconciler.snapshot().nodes.len(), 31);
                assert!(patch.is_some());
                let mut ready = SurfaceReconcileReadyPatch { generation: cursor.assembly_generation, patch: pending_surface_patch(patch), credit: output, handback: reserve_surface_reconcile_handback(cursor.assembly_generation) };
                while !ready.close_step() {}
                while !cursor.retire_one() {}
                while !reconciler.retire_one() {}
                break;
            }
            SurfaceReconcileStep::Fault(fault) => panic!("unexpected reconcile fault: {fault:?}"),
        }
    }
    assert!(yields >= 93, "every presented node crosses three independent cursor phases");
}

#[test]
fn identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let surface = "s".repeat(SurfaceReconcileLimits::default().max_identifier_bytes + 1);
    let tree = tree(leaf("exact"));
    let mut rejected = match SurfaceReconcileJob::try_new(SurfaceReconciler::new(surface), tree, 71) {
        Ok(_) => panic!("identifier + 1 must reject"),
        Err(rejected) => rejected,
    };
    let (_, returned) = rejected.take_sources().expect("exact rejected owners");
    assert_eq!(returned.root.key.as_str(), "exact");
    while !rejected.close_step() {}
    assert!(rejected.terminal_is_empty());
}

#[test]
fn semantic_aggregate_quota_faults_before_key_or_record_clone() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut data_attributes = ui_contract::UiFixedMap::default();
    data_attributes.try_push(ui_text("semantic"), ui_text("payload")).expect("bounded fixture attribute");
    let node = crate::TreeNode::try_new("exact", ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(ui_text("value")), emphasize: None, data_attributes: Some(data_attributes) })).expect("bounded fixture node");
    let current = SurfaceReconciler::new("s");
    let limits = SurfaceReconcileLimits { max_bytes: SURFACE_RECONCILE_PAGE_BYTES, ..Default::default() };
    let mut cursor = SurfaceReconcileCursor::new_with_limits(tree(node), &current, limits);
    let mut fault = None;
    for _ in 0..4_096 {
        if let SurfaceReconcileStep::Fault(found) = cursor.step(&current) {
            fault = Some(found);
            break;
        }
    }
    assert!(matches!(fault, Some(SurfaceReconcileFault::Credits { .. })));
    let retained = cursor.held_node.as_ref().expect("exact unmaterialized node remains retained");
    assert_eq!(retained.1.key.as_str(), "exact");
    assert!(cursor.flat.is_empty());
    assert!(cursor.seen.is_empty());
    assert!(cursor.new_ordinals.is_empty());
    assert!(cursor.ops.is_empty());
}

#[test]
fn opaque_surface_document_uses_aggregate_credits_instead_of_scalar_page() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let payload = vec![7; ui_contract::UI_FIXED_BYTES];
    let props = ui_contract::SurfaceProps {
        kind: ui_contract::SurfaceKind::NodeGraph,
        doc_schema: ui_text("node-graph@1"),
        doc: ui_contract::SurfaceDoc { bytes: ui_contract::UiFixedBytes::try_from_vec(payload.clone()).expect("fixed surface payload") },
        bindings: Default::default(),
    };
    let node = crate::TreeNode::try_new("surface", ui_contract::Component::Surface(props)).expect("bounded surface node");
    let current = SurfaceReconciler::new("s");
    let (reconciled, mut patch, _) = reconcile_resumable(&current, tree(node));

    assert!(patch.is_some(), "the opaque surface publishes through the same transactional patch path");
    if let Some(patch) = patch.as_mut() {
        while !patch.close_step() {}
    }
    let snapshot = reconciled.snapshot();
    let ui_contract::Component::Surface(actual) = &snapshot.nodes[0].component else { panic!("surface component") };
    assert_eq!(actual.doc.bytes.as_slice(), payload);
    let json = serde_json::to_value(&snapshot).expect("third-party snapshot serialization");
    assert_eq!(json["nodes"][0]["component"]["doc"]["bytes"].as_array().map(Vec::len), Some(ui_contract::UI_FIXED_BYTES));
}

#[test]
fn semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    fn expired_now() -> Option<u64> {
        Some(10)
    }
    let generation = 7_001;
    {
        let mut zero = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("zero")), generation).expect("admitted");
        let mut sequence = 0;
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(0, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        assert_eq!(zero.drive_one(&mut context), SurfaceReconcileJobStep::MoreWork);
        let cursor = zero.state.as_ref().and_then(|state| state.cursor.as_ref()).expect("cursor retained");
        assert!(cursor.pending_root.is_some());
        assert!(cursor.held_node.is_none());
    }
    super::drain_surface_reconcile_registry_until_idle();

    let generation = 7_002;
    let mut expired = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("deadline")), generation).expect("admitted");
    let mut sequence = 0;
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::allocate_operation_id(),
        semio_framework_job::Generation(generation),
        semio_framework_job::StepBudget::new(1, 10),
        semio_framework_job::root_cancel_token(),
        expired_now,
        &mut sequence,
    );
    assert_eq!(expired.drive_one(&mut context), SurfaceReconcileJobStep::MoreWork);
    assert!(expired.state.as_ref().and_then(|state| state.cursor.as_ref()).is_some_and(|cursor| cursor.pending_root.is_some() && cursor.held_node.is_none()));
}

#[test]
fn semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let wide = (0..128).map(|index| ui_contract::UiValue::Text(ui_contract::UiText::try_from_string(format!("value-{index}")).expect("bounded fixture text"))).collect::<Vec<_>>();
    let node = crate::TreeNode::try_new("wide", ui_contract::Component::Extension(ui_contract::ExtensionProps { extension: ui_text("fixture"), props: ui_contract::UiValue::List(ui_list(wide)) })).expect("bounded fixture node");
    let current = SurfaceReconciler::new("s");
    let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
    for _ in 0..32 {
        assert!(matches!(cursor.step(&current), SurfaceReconcileStep::Yield { .. }));
    }
    assert!(cursor.flat.is_empty(), "a wide value cannot complete census in one grant");

    let mut deep = ui_contract::UiValue::Null;
    for _ in 0..=SURFACE_RECONCILE_VALUE_DEPTH {
        deep = ui_contract::UiValue::List(ui_list([deep]));
    }
    let node = crate::TreeNode::try_new("deep", ui_contract::Component::Extension(ui_contract::ExtensionProps { extension: ui_text("fixture"), props: deep })).expect("bounded fixture node");
    let mut cursor = SurfaceReconcileCursor::new(tree(node), &current);
    let mut fault = None;
    for _ in 0..8_192 {
        if let SurfaceReconcileStep::Fault(found) = cursor.step(&current) {
            fault = Some(found);
            break;
        }
    }
    assert!(matches!(fault, Some(SurfaceReconcileFault::ValueDepth { .. })));
    assert!(cursor.flat.is_empty());
}

#[test]
fn retained_map_page_advances_each_key_once_without_rewalking_prior_entries() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let value = ui_contract::UiValue::Map(ui_map([("a".to_owned(), ui_contract::UiValue::Null), ("b".to_owned(), ui_contract::UiValue::Null), ("c".to_owned(), ui_contract::UiValue::Null)]));
    let mut cursor = SurfaceSemanticCensusCursor::default();
    cursor.push_value(&value).expect("fixed value depth");
    let mut steps = 0;
    while cursor.depth > 0 {
        cursor.value_step().expect("retained map cursor progress");
        steps += 1;
        assert!(steps <= 11, "three entries must never rewalk prior pages");
    }
    assert_eq!(steps, 11);
}

#[test]
fn allocate_inspect_admit_retains_exact_vector_backing_on_cap_plus_one_without_partial_item_mutation() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut owner = Vec::<u64>::new();
    let mut usage = SurfaceReconcileUsage::default();
    let limits = SurfaceReconcileLimits { max_nodes: 0, max_items: 0, max_bytes: 0, max_identifier_bytes: 0 };
    let fault = admit_vec_backing(&mut owner, &mut usage, limits).expect_err("first backing slot exceeds zero cap");
    assert!(owner.capacity() >= 1, "allocate-inspect retains the exact allocated page on refusal");
    assert!(owner.is_empty(), "no logical item mutates before page admission");
    assert!(matches!(fault, SurfaceReconcileFault::Credits { .. }));

    let actual_capacity = owner.capacity();
    let exact = SurfaceReconcileLimits { max_nodes: 0, max_items: actual_capacity, max_bytes: actual_capacity * size_of::<u64>(), max_identifier_bytes: 0 };
    let mut admitted = Vec::<u64>::new();
    let mut usage = SurfaceReconcileUsage::default();
    admit_vec_backing(&mut admitted, &mut usage, exact).expect("actual inspected cap admits");
    admitted.push(7);
    assert_eq!(admitted, [7]);
}

#[test]
fn persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let generation = 7_003;
    let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("credit")), generation).expect("admitted");
    let mut sequence = 0;
    for _ in 0..4_096 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, u64::MAX),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        if job.drive_one(&mut context) == SurfaceReconcileJobStep::Ready {
            break;
        }
    }
    let (reconciler, ready_patch) = match job.take_ready() {
        Ok(ready) => ready,
        Err(_) => panic!("ready owner"),
    };
    let retained_credit = reconciler.document.as_ref().expect("take_ready retains original canonical root").try_read().unwrap().resident_limits();
    assert!(retained_credit.items < SurfaceReconcileLimits::default().max_items, "ready reconciliation returns unused aggregate item capacity");
    assert!(retained_credit.bytes < SurfaceReconcileLimits::default().max_bytes, "ready reconciliation returns unused aggregate byte capacity");
    let mut ready_patch = ready_patch.expect("initial reconciliation publishes a patch");
    let patch_credit = ready_patch.credit.as_ref().expect("ready patch shares the retained credit");
    assert_eq!((patch_credit.limits().items, patch_credit.limits().bytes), (retained_credit.items, retained_credit.bytes));
    while !ready_patch.close_step() {}
    let mut terminal = SurfaceReconcileTerminal::try_from_reconciler(reconciler, generation).expect("pre-admitted terminal handback");
    assert!(!terminal.close_step());
    while !terminal.terminal_is_empty() {
        terminal.close_step();
    }
}

#[test]
fn public_drop_handback_is_lossless_at_terminal_cap_and_plus_one() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let first = 80_000;
    let mut keys = Vec::with_capacity(SURFACE_RECONCILE_HANDBACK_SLOTS);
    for offset in 0..SURFACE_RECONCILE_HANDBACK_SLOTS {
        let terminal = SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new(format!("drop-{offset}")), first + offset as u64).expect("public cap admits");
        keys.push(terminal.handback_key().expect("fixed registry key"));
        drop(terminal);
    }
    let overflow = SurfaceReconciler::new("overflow-owner");
    let returned = match SurfaceReconcileTerminal::try_from_reconciler(overflow, first + SURFACE_RECONCILE_HANDBACK_SLOTS as u64) {
        Ok(_) => panic!("public cap + 1 must refuse"),
        Err(returned) => returned,
    };
    assert_eq!(returned.surface().0.as_str(), "overflow-owner");
    for key in keys {
        let mut terminal = take_surface_reconcile_terminal(key).expect("valid handback registry").expect("every fixed handback owner remains O(1) recoverable");
        while !terminal.terminal_is_empty() {
            terminal.close_step();
        }
    }
}

#[test]
fn stale_cancel_and_drop_handoff_preserve_public_terminal_ownership() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let generation = 8_001;
    let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("exact")), generation).expect("admitted");
    let mut sequence = 0;
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::allocate_operation_id(),
        semio_framework_job::Generation(generation + 1),
        semio_framework_job::StepBudget::new(1, u64::MAX),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
    );
    assert_eq!(job.drive_one(&mut context), SurfaceReconcileJobStep::Fault);
    let handback_key = job.handback_key().expect("fault retains fixed public handback reservation");
    drop(job);
    let mut terminal = take_surface_reconcile_terminal(handback_key).expect("valid handback registry").expect("drop handback is observable in O(1)");
    assert!(matches!(terminal.fault(), Some(SurfaceReconcileFault::StaleGeneration { .. })));
    for _ in 0..32 {
        if terminal.close_step() && terminal.terminal_is_empty() {
            break;
        }
    }
    assert!(terminal.terminal_is_empty());

    let generation = 8_002;
    let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("s"), tree(leaf("cancel")), generation).expect("admitted");
    let cancel = semio_framework_job::root_cancel_token();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut context =
        semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_us, &mut sequence);
    assert_eq!(job.drive_one(&mut context), SurfaceReconcileJobStep::Fault);
    let mut terminal = job.into_terminal();
    assert!(matches!(terminal.fault(), Some(SurfaceReconcileFault::Cancelled)));
    for _ in 0..32 {
        if terminal.close_step() && terminal.terminal_is_empty() {
            break;
        }
    }
    assert!(terminal.terminal_is_empty());
}
//#endregion ⏭️ResumableCursor

//#region 🔖️FirstReconcileAndIdempotence
#[test]
fn first_reconcile_emits_set_root_and_one_upsert_per_node_then_is_idempotent() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    let component_tree = tree(container("root", vec![leaf("a"), leaf("b")]));

    let patch = reconciler.reconcile(&component_tree).expect("first reconcile must emit a patch");
    assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
    assert_eq!(patch.revision, ui_contract::UiRevision(1));
    assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(_))).count(), 3);
    assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::SetRoot { .. })).count(), 1);

    assert!(reconciler.reconcile(&component_tree).is_none(), "an unchanged tree must emit no patch");
}
//#endregion 🔖️FirstReconcileAndIdempotence

//#region 🔖️TargetedOps
#[test]
fn changing_one_leaf_text_emits_exactly_one_op_naming_exactly_that_node() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![text("a", "hello"), leaf("b")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![text("a", "world"), leaf("b")]))).expect("a changed leaf must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetComponent { id, component } => {
            assert_eq!(*id, target_id);
            assert_eq!(component, &ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from("world").expect("bounded fixture label"), emphasize: None, data_attributes: None }));
        }
        other => panic!("expected SetComponent (not Upsert), got {other:?}"),
    }
}

#[test]
fn reordering_siblings_preserves_every_id_and_emits_only_set_children() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("b"), leaf("c")]))).unwrap();
    let before = reconciler.snapshot();
    let (a_id, b_id, c_id) = (id_of(&before, "a"), id_of(&before, "b"), id_of(&before, "c"));

    let patch = reconciler.reconcile(&tree(container("root", vec![leaf("c"), leaf("a"), leaf("b")]))).expect("a reorder must emit a patch");
    assert_eq!(patch.ops.len(), 1);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetChildren { children, .. } => assert_eq!(children.iter().copied().collect::<Vec<_>>(), vec![c_id, a_id, b_id]),
        other => panic!("expected SetChildren, got {other:?}"),
    }

    let after = reconciler.snapshot();
    assert_eq!(id_of(&after, "a"), a_id);
    assert_eq!(id_of(&after, "b"), b_id);
    assert_eq!(id_of(&after, "c"), c_id);
}

#[test]
fn inserting_a_middle_sibling_preserves_the_others_ids() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("c")]))).unwrap();
    let before = reconciler.snapshot();
    let (a_id, c_id) = (id_of(&before, "a"), id_of(&before, "c"));

    let patch = reconciler.reconcile(&tree(container("root", vec![leaf("a"), leaf("b"), leaf("c")]))).expect("an insertion must emit a patch");
    assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::Upsert(record) if record.key.as_str() == "b")));

    let after = reconciler.snapshot();
    assert_eq!(id_of(&after, "a"), a_id);
    assert_eq!(id_of(&after, "c"), c_id);
}

#[test]
fn changed_component_with_unchanged_layout_emits_set_component_not_upsert() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();

    let patch = reconciler.reconcile(&tree(container("root", vec![text("a", "now text")]))).expect("a component change must emit a patch");
    assert_eq!(patch.ops.len(), 1);
    assert!(matches!(patch.ops[0], ui_contract::UiPatchOp::SetComponent { .. }), "expected SetComponent, got {:?}", patch.ops[0]);
}

/// 🎨️ The finding this packet exists to fix: a style-only change on a leaf with everything else
/// unchanged must emit exactly one `SetStyle`, never a whole-node `Upsert`.
#[test]
fn changing_only_style_emits_exactly_one_set_style_not_upsert() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![styled(leaf("a"), ui_contract::Tone::Danger)]))).expect("a style change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetStyle { id, style } => {
            assert_eq!(*id, target_id);
            assert_eq!(style.tone, ui_contract::Tone::Danger);
        }
        other => panic!("expected SetStyle (not Upsert), got {other:?}"),
    }
}

#[test]
fn changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![with_shortcut(leaf("a"), "Ctrl+S")]))).expect("an accessibility change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetAccessibility { id, accessibility } => {
            assert_eq!(*id, target_id);
            assert_eq!(accessibility.shortcut.as_deref(), Some("Ctrl+S"));
        }
        other => panic!("expected SetAccessibility (not Upsert), got {other:?}"),
    }
}

#[test]
fn changing_only_bindings_emits_exactly_one_set_bindings_not_upsert() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![with_binding(leaf("a"), "scope", "name")]))).expect("a bindings change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetBindings { id, bindings } => {
            assert_eq!(*id, target_id);
            assert_eq!(bindings.len(), 1);
        }
        other => panic!("expected SetBindings (not Upsert), got {other:?}"),
    }
}

#[test]
fn changing_only_menu_emits_exactly_one_set_menu_not_upsert() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let patch = reconciler.reconcile(&tree(container("root", vec![with_menu(leaf("a"), "menu")]))).expect("a menu change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "exactly one op expected, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::SetMenu { id, menu } => {
            assert_eq!(*id, target_id);
            assert_eq!(menu.as_ref().map(|menu| menu.id.as_str()), Some("menu"));
        }
        other => panic!("expected SetMenu (not Upsert), got {other:?}"),
    }
}

/// 💰️ Once several groups change at once, [`SurfaceReconciler::estimate_bytes`] weighs a single
/// `Upsert` against the pile of targeted ops it would replace — here five groups change on a leaf
/// whose new component/accessibility still carry no real text, so the targeted ops' fixed per-op
/// overhead alone outweighs one full-record `Upsert`, and `Upsert` wins.
#[test]
fn changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let target_id = id_of(&reconciler.snapshot(), "a");

    let mut changed =
        crate::TreeNode::try_new("a", ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
            .expect("bounded fixture node");
    changed.style = ui_contract::StyleSpec { tone: ui_contract::Tone::Danger, ..Default::default() };
    changed.activity = ui_contract::Activity::Loading;
    changed.disabled = true;
    changed.accessibility = ui_contract::AccessibilitySpec { hidden: true, ..Default::default() };

    let patch = reconciler.reconcile(&tree(container("root", vec![changed]))).expect("a multi-group change must emit a patch");
    assert_eq!(patch.ops.len(), 1, "expected one Upsert to beat several targeted ops, got {:?}", patch.ops);
    match &patch.ops[0] {
        ui_contract::UiPatchOp::Upsert(record) => assert_eq!(record.id, target_id),
        other => panic!("expected Upsert, got {other:?}"),
    }
}
//#endregion 🔖️TargetedOps

//#region 🔖️Removal
#[test]
fn removing_a_subtree_emits_one_remove_and_leaves_no_orphan_in_retained() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![container("mid", vec![leaf("x"), leaf("y")]), leaf("z")]))).unwrap();
    let mid_id = id_of(&reconciler.snapshot(), "mid");

    let patch = reconciler.reconcile(&tree(container("root", vec![leaf("z")]))).expect("a removal must emit a patch");
    let removes: Vec<_> = patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Remove { .. })).collect();
    assert_eq!(removes.len(), 1);
    assert!(matches!(removes[0], ui_contract::UiPatchOp::Remove { id } if *id == mid_id));

    let after = reconciler.snapshot();
    assert!(!after.nodes.iter().any(|record| matches!(record.key.as_str(), "mid" | "x" | "y")), "removed subtree must leave no orphan");
    assert_eq!(after.nodes.len(), 2, "only root and z should remain");
}

#[test]
fn ids_are_never_reused_after_removal() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let removed_id = id_of(&reconciler.snapshot(), "a");

    reconciler.reconcile(&tree(container("root", vec![]))).unwrap();
    reconciler.reconcile(&tree(container("root", vec![leaf("a")]))).unwrap();
    let reinserted_id = id_of(&reconciler.snapshot(), "a");

    assert_ne!(reinserted_id, removed_id, "a fresh node at a previously-used key must never reuse a removed id");
}
//#endregion 🔖️Removal

//#region 🔖️Rejection
#[test]
fn mark_rejected_then_reconcile_emits_a_full_resend() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    let component_tree = tree(container("root", vec![leaf("a"), leaf("b")]));
    reconciler.reconcile(&component_tree).unwrap();
    assert!(reconciler.reconcile(&component_tree).is_none());

    reconciler.mark_rejected();
    let patch = reconciler.reconcile(&component_tree).expect("resend after rejection must emit a patch");
    assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
    assert_eq!(patch.ops.iter().filter(|op| matches!(op, ui_contract::UiPatchOp::Upsert(_))).count(), 3);
    assert!(patch.ops.iter().any(|op| matches!(op, ui_contract::UiPatchOp::SetRoot { .. })));
}
//#endregion 🔖️Rejection

//#region 🔖️DuplicateKeys
#[test]
fn duplicate_sibling_keys_are_reported_even_when_component_tree_new_is_bypassed() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    let mut children = ui_contract::BuiltChildren::default();
    children.try_push(leaf("a")).expect("bounded fixture child");
    children.try_push(leaf("a")).expect("bounded fixture child");
    let root = crate::TreeNode { children, ..leaf("root") };
    let component_tree = crate::ComponentTree { root };

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| reconciler.reconcile(&component_tree)));
    assert!(result.is_err(), "a duplicate sibling key must panic, not silently shadow");
}
//#endregion 🔖️DuplicateKeys

//#region 🔖️RoundTripProperty
/// 🔁️ The property that matters most: every patch this reconciler ever emits must apply cleanly
/// through the contract's own [`ui_contract::apply_patch`], and doing so must reproduce
/// [`SurfaceReconciler::snapshot`] exactly. Exercised across a sequence of trees that each mutate a
/// different axis (reorder, insert, remove, text change, nested restructure, collapse-to-one-child,
/// style-only, accessibility-only, bindings-only, menu-only, and a multi-group change that should
/// fall back to `Upsert`) so this one test would catch a producer/consumer disagreement in any of
/// them — including the four field-targeted ops this packet adds.
#[test]
fn round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let mut reconciler = SurfaceReconciler::new("s");
    let mut receiver_state = ui_contract::UiSnapshotState::new(ui_contract::SurfaceId::try_from("s").expect("bounded fixture surface"));
    let limits = ui_contract::UiDocumentLimits::default();

    let frames = vec![
        tree(container("root", vec![leaf("a"), leaf("b")])),
        tree(container("root", vec![leaf("b"), leaf("a"), text("c", "hi")])),
        tree(container("root", vec![text("c", "bye"), container("mid", vec![leaf("d")])])),
        tree(container("root", vec![container("mid", vec![leaf("d"), leaf("e")])])),
        tree(container("root", vec![leaf("solo")])),
        tree(container("root", vec![styled(leaf("solo"), ui_contract::Tone::Primary)])),
        tree(container("root", vec![with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K")])),
        tree(container("root", vec![with_binding(with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K"), "scope", "name")])),
        tree(container("root", vec![with_menu(with_binding(with_shortcut(styled(leaf("solo"), ui_contract::Tone::Primary), "Ctrl+K"), "scope", "name"), "menu")])),
        tree(container("root", vec![leaf("solo"), leaf("solo2")])),
    ];

    let mut generation = 1u64;
    for component_tree in &frames {
        if let Some(patch) = reconciler.reconcile(component_tree) {
            let mut producer = ui_contract::UiPatchApplyProducer::try_new(receiver_state, patch, limits, generation).expect("every emitted patch must enter the retained contract producer");
            loop {
                match producer.drive_one(generation, false, false) {
                    ui_contract::UiPatchApplyStep::MoreWork => {}
                    ui_contract::UiPatchApplyStep::Ready => break,
                    ui_contract::UiPatchApplyStep::Rejected => panic!("every emitted patch must apply cleanly against the contract producer: {:?}", producer.rejection()),
                }
            }
            let mut outcome = producer.take_ready().unwrap_or_else(|_| panic!("ready producer must transfer its exact outcome"));
            while !outcome.close_step() {}
            receiver_state = outcome.take_state().unwrap_or_else(|_| panic!("closed outcome must return the exact new state"));
            generation = generation.checked_add(1).expect("bounded fixture generation");
        }
        assert_snapshot_matches_state(&reconciler.snapshot(), &receiver_state);
    }
}
//#endregion 🔖️RoundTripProperty

//#region 🔖️ResidentAdmissionArithmetic

/// ⚖️ Every slot the resident SLOT ledger offers must also fit inside its BYTE aggregate, so a surface
/// is refused only when the declared admission limit [`SURFACE_RECONCILE_ADMISSION_SLOTS`] is actually
/// reached — never because the FLOOR PRICE of one reservation outgrew the budget while sixty-three
/// slots stood empty.
///
/// 🐛️ ticket 26/09/09/PROCEDURAL-3D-END-TO-END lane `react-example-switch-regression`: the React
/// generation3d editor ran with the aggregate at `33548883` of `33554432` bytes — 99.98 % — from boot,
/// because every mounted surface was charged a floor that already contained a whole static document
/// arena slot. The fifth example switch asked the flow window to grow to `2517394` bytes, the ledger
/// had `5549` left, and `SurfaceReconcileFault::ResidentCredit` retired `window:procedural-main` for
/// good: fourteen further switches published the same stale graph
/// (`🗑️generated/example-switch/results.json`). A full aggregate is deliberately a terminal fault
/// rather than a retry — wave B56's defer loop is worse — so the aggregate must simply never be full
/// while slots remain.
#[test]
fn resident_aggregate_admits_every_reconcile_slot() {
    let fixed = ui_contract::UiResidentPermit::fixed_backing_bytes().expect("resident ledger");
    let floor = SURFACE_RECONCILE_FLOOR_BYTES;
    let open = ui_contract::UiDocumentAssembly::required_open_bytes();
    let flat = size_of::<FlatPresentedNode>();
    let document = ui_contract::UI_RESIDENT_DOCUMENT_BYTES;
    let admitted = SURFACE_RECONCILE_AGGREGATE_BYTES / document.max(1);
    eprintln!(
        "[DEBUG] resident-admission aggregate={SURFACE_RECONCILE_AGGREGATE_BYTES}B fixed={fixed}B contract={}B floor={floor}B open={open}B page={}B flat={flat}B nodes={} ceiling={SURFACE_RECONCILE_SURFACE_BYTES}B document={document}B admitted={admitted} slots={SURFACE_RECONCILE_ADMISSION_SLOTS}",
        ui_contract::UiResidentPermit::contract_backing_bytes(),
        SURFACE_RECONCILE_PAGE_BYTES,
        SurfaceReconcileLimits::default().max_nodes,
    );
    assert!(
        admitted >= SURFACE_RECONCILE_ADMISSION_SLOTS,
        "the byte aggregate must fund every slot the slot ledger admits at one full document each: {admitted} of {SURFACE_RECONCILE_ADMISSION_SLOTS} slots fit at {document}B, against {SURFACE_RECONCILE_AGGREGATE_BYTES}B aggregate",
    );
    assert!(
        SURFACE_RECONCILE_AGGREGATE_BYTES.saturating_sub(fixed) > SURFACE_RECONCILE_ADMISSION_SLOTS.saturating_mul(floor),
        "the fixed backing must never eat the floor of every slot: {fixed}B backing and {floor}B floors against {SURFACE_RECONCILE_AGGREGATE_BYTES}B",
    );
}

//#endregion 🔖️ResidentAdmissionArithmetic


//#region 🪟️TreeWindowReconcileSizeLaw

/// 🪟️ Sections one virtualised panel tree declares (objects / references / target volumes /
/// attractions — the puzzle3d artifact panel whose stale `PANEL_RECONCILE_NODE_BUDGET = 16` this law
/// replaces).
const TREE_WINDOW_LAW_SECTIONS: usize = 4;

/// 🪟️ Rows one section's `TreeWindow.total` declares: one full built child list
/// ([`ui_contract::UI_BUILT_CHILDREN_MAX`]).
const TREE_WINDOW_LAW_SECTION_TOTAL: usize = ui_contract::UI_BUILT_CHILDREN_MAX;

/// 🪟️ The per-section slice one presented document could carry if the RECORD arena were the only
/// bound: a surface document is [`ui_contract::UI_DOCUMENT_NODES`] records wide IN TOTAL
/// (`UiSnapshotNodes`), and the tree root plus one record per section are spent before a single row
/// is materialised, so `(128 − 1 − 4) / 4 = 30` — never the 128 a `BuiltChildren` list can hold. The
/// law below starts here and walks DOWN until admission actually accepts the page.
const TREE_WINDOW_LAW_PAGE_ROWS: usize = (ui_contract::UI_DOCUMENT_NODES - 1 - TREE_WINDOW_LAW_SECTIONS) / TREE_WINDOW_LAW_SECTIONS;

/// 🕹️ Rows per section that carry their own `Activate` binding with a four-entry argument map,
/// mirroring an app row that still owns an action (catalogue `addWidget`, workshop install …) rather
/// than picking through the tree-level interaction domain. Pick rows cost no arena at all.
const TREE_WINDOW_LAW_BOUND_ROWS: usize = 31;

fn tree_window_law_row(section: usize, index: usize) -> crate::TreeNode {
    let key = format!("s{section}.row-{index}");
    let props = ui_contract::TreeItemProps {
        label: ui_contract::Label::try_from(key.as_str()).expect("bounded fixture label"),
        description: None,
        icon: Some(ui_text("layers")),
        default_open: None,
        draggable: None,
        drag_data: None,
        dimmed: None,
        window: None,
        granularity: Some(ui_text("object")),
        row_actions: ui_contract::UiFixedList::default(),
    };
    let mut node = crate::TreeNode::try_new(&key, ui_contract::Component::TreeItem(props)).expect("bounded fixture row");
    if index < TREE_WINDOW_LAW_BOUND_ROWS {
        if let Some(args) = tree_window_law_args(&key) {
            node.bindings
                .try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::try_v1("puzzle", "addWidget").expect("bounded fixture action"), args: Some(args), capability: None })
                .expect("bounded fixture bindings");
        }
    }
    node
}

/// 🌱️ A four-entry argument map, or `None` when the shared `UiValue` arena refuses one — a refused
/// row shortens the window, it never faults, so this law measures whatever admission actually allowed.
fn tree_window_law_args(key: &str) -> Option<ui_contract::UiValue> {
    let mut builder = ui_contract::UiMapBuilder::try_new()?;
    for (name, value) in [("id", key), ("kind", "assembly"), ("mode", "insert"), ("origin", "panel")] {
        builder.push(name.to_string(), ui_contract::UiValue::Text(ui_contract::UiText::try_from_str(value)?)).ok()?;
    }
    Some(ui_contract::UiValue::Map(builder.finish()))
}

/// 🪟️ One presented document of the paged tree: every section stamps its FULL `total`, materialises
/// only `[offset, offset + rows)`, and never mints a continuation row.
fn tree_window_law_document(offset: usize, rows_per_section: usize) -> crate::ComponentTree {
    let mut sections = Vec::with_capacity(TREE_WINDOW_LAW_SECTIONS);
    for section in 0..TREE_WINDOW_LAW_SECTIONS {
        let rows: Vec<crate::TreeNode> = (offset..TREE_WINDOW_LAW_SECTION_TOTAL.min(offset + rows_per_section)).map(|index| tree_window_law_row(section, index)).collect();
        let props = ui_contract::TreeSectionProps {
            label: Some(ui_contract::Label::try_from(format!("Section {section}").as_str()).expect("bounded fixture label")),
            default_open: Some(true),
            window: Some(ui_contract::TreeWindow { row_extent: Default::default(), total: u32::try_from(TREE_WINDOW_LAW_SECTION_TOTAL).expect("bounded fixture total"), offset: u32::try_from(offset).expect("bounded fixture offset") }),
        };
        let node = crate::TreeNode::try_new(format!("section-{section}"), ui_contract::Component::TreeSection(props)).expect("bounded fixture section");
        sections.push(node.try_with_children(rows).unwrap_or_else(|_| panic!("bounded fixture section children")));
    }
    let root = crate::TreeNode::try_new("artifact-tree", ui_contract::Component::Tree(ui_contract::TreeProps { presentation: Default::default(), interaction_domain: Some(ui_text("object")) })).expect("bounded fixture tree");
    tree(root.try_with_children(sections).unwrap_or_else(|_| panic!("bounded fixture tree children")))
}

/// 📏️ Drives one whole resumable reconciliation and hands back either the credits it consumed or the
/// exact refusal, releasing the cursor's own reservations either way.
fn tree_window_law_reconcile(current: &SurfaceReconciler, component_tree: crate::ComponentTree) -> Result<(SurfaceReconciler, SurfaceReconcileUsage), SurfaceReconcileFault> {
    let mut cursor = SurfaceReconcileCursor::new(component_tree, current);
    loop {
        match cursor.step(current) {
            SurfaceReconcileStep::Yield { .. } => {}
            SurfaceReconcileStep::Complete { mut reconciler, patch } => {
                let usage = cursor.usage;
                let mut output = None;
                while !reconciler.seal_step(usage, &mut output, patch.is_some()).unwrap() {}
                if let Some(patch) = patch {
                    let mut ready = SurfaceReconcileReadyPatch { generation: cursor.assembly_generation, patch: pending_surface_patch(Some(patch)), credit: output, handback: reserve_surface_reconcile_handback(cursor.assembly_generation) };
                    while !ready.close_step() {}
                }
                while !cursor.retire_one() {}
                return Ok((reconciler, usage));
            }
            SurfaceReconcileStep::Fault(fault) => {
                while !cursor.retire_one() {}
                drop(cursor);
                super::drain_surface_reconcile_registry_until_idle();
                return Err(fault);
            }
        }
    }
}

/// 📏️ The reconcile SIZE law, replacing `PANEL_RECONCILE_NODE_BUDGET = 16` and the inherited
/// "~0.46 MiB per presented node" docstring it carried (puzzle3d artifact panel, 2026-09-10) — a
/// number 📓️audit-surface-budgets.md §3 showed was never a measured host-side
/// `size_of::<FlatPresentedNode>()`, and which predates the page-exact `UiFixedList` reservation fix
/// of 2026-09-13.
///
/// What this pins, for ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §7:
/// 1. The real per-node host cost is `size_of::<FlatPresentedNode>()` plus that node's measured
///    semantic bytes, both printed, so nobody has to inherit a number again.
/// 2. A four-section tree whose sections each declare a full
///    [`ui_contract::UI_BUILT_CHILDREN_MAX`]-row `TreeWindow` reconciles far under
///    [`SURFACE_RECONCILE_SURFACE_BYTES`]. **Bytes are not what bounds a virtualised panel.**
/// 3. What bounds it is the ITEM ledger (`SurfaceReconcileLimits::max_items`), reached long before
///    either the byte ceiling or the [`ui_contract::UI_DOCUMENT_NODES`] record arena whenever rows
///    carry their own argument maps — which is exactly why §5's `.interaction_domain()` moves pick
///    rows off per-row bindings. Every refusal is asserted to be an item/node refusal, never a byte
///    one, and the window is shortened rather than faulted — the `ui.fixed-capacity` rule of §5.
/// 4. The whole 4 × [`TREE_WINDOW_LAW_SECTION_TOTAL`] document streams as a sequence of pages, every
///    one of them measured, the way scrolling actually requests them.
#[test]
fn four_section_full_window_tree_reconciles_under_the_surface_byte_ceiling() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let flat = size_of::<FlatPresentedNode>();
    let limits = SurfaceReconcileLimits::default();
    let mut current = SurfaceReconciler::new("panel:artifact-tree");
    let mut peak = SurfaceReconcileUsage::default();
    let mut total_bytes = 0usize;
    let mut materialised = 0usize;
    let mut pages = 0usize;
    let mut rows_per_section = TREE_WINDOW_LAW_PAGE_ROWS;
    let mut offset = 0usize;
    while offset < TREE_WINDOW_LAW_SECTION_TOTAL {
        let (reconciled, usage) = loop {
            match tree_window_law_reconcile(&current, tree_window_law_document(offset, rows_per_section)) {
                Ok(admitted) => break admitted,
                Err(SurfaceReconcileFault::Credits { usage, limits }) => {
                    eprintln!("[DEBUG] flat-presented-node bytes={flat} refused rows_per_section={rows_per_section} nodes={}/{} items={}/{} surface_bytes={}/{}", usage.nodes, limits.max_nodes, usage.items, limits.max_items, usage.bytes, limits.max_bytes);
                    assert!(
                        usage.bytes <= limits.max_bytes,
                        "no window page may ever be refused for BYTES — the 8 MiB surface ceiling is not the binding constraint: {} of {}B",
                        usage.bytes,
                        limits.max_bytes,
                    );
                    rows_per_section = rows_per_section.checked_sub(1).expect("admission must accept some non-empty window");
                    assert!(rows_per_section > 0, "admission must accept at least one row per section");
                }
                Err(fault) => panic!("unexpected reconcile fault: {fault:?}"),
            }
        };
        let rows = usage.nodes.saturating_sub(1 + TREE_WINDOW_LAW_SECTIONS);
        eprintln!(
            "[DEBUG] flat-presented-node bytes={flat} rows={rows} nodes={} offset={offset} rows_per_section={rows_per_section} items={} surface_bytes={} per_node_bytes={} ceiling={SURFACE_RECONCILE_SURFACE_BYTES}",
            usage.nodes,
            usage.items,
            usage.bytes,
            usage.bytes / usage.nodes.max(1),
        );
        assert!(usage.nodes <= limits.max_nodes, "one presented document must stay inside the {}-record document arena: {} nodes", limits.max_nodes, usage.nodes);
        assert!(
            usage.bytes < SURFACE_RECONCILE_SURFACE_BYTES,
            "a {TREE_WINDOW_LAW_SECTIONS}-section window page must reconcile under the per-surface ceiling: {} of {SURFACE_RECONCILE_SURFACE_BYTES}B",
            usage.bytes,
        );
        if usage.bytes > peak.bytes {
            peak = usage;
        }
        total_bytes = total_bytes.saturating_add(usage.bytes);
        materialised = materialised.saturating_add(rows);
        pages += 1;
        offset += rows_per_section;
        let mut retired = reconciled;
        while !retired.retire_one() {}
        current = retired;
    }
    eprintln!(
        "[DEBUG] flat-presented-node bytes={flat} rows={materialised} pages={pages} declared_rows={} peak_surface_bytes={} peak_nodes={} peak_items={} total_surface_bytes={total_bytes} ceiling={SURFACE_RECONCILE_SURFACE_BYTES} document_nodes={} max_items={}",
        TREE_WINDOW_LAW_SECTIONS * TREE_WINDOW_LAW_SECTION_TOTAL,
        peak.bytes,
        peak.nodes,
        peak.items,
        ui_contract::UI_DOCUMENT_NODES,
        limits.max_items,
    );
    assert_eq!(materialised, TREE_WINDOW_LAW_SECTIONS * TREE_WINDOW_LAW_SECTION_TOTAL, "every declared row of every section must be materialised by exactly one page");
    assert!(peak.bytes < SURFACE_RECONCILE_SURFACE_BYTES, "the heaviest window page must still fit one surface: {} of {SURFACE_RECONCILE_SURFACE_BYTES}B", peak.bytes);
    assert!(
        peak.bytes / peak.nodes.max(1) < SURFACE_RECONCILE_SURFACE_BYTES / ui_contract::UI_DOCUMENT_NODES,
        "per-node cost must leave a full document arena affordable: {}B per node against {}B",
        peak.bytes / peak.nodes.max(1),
        SURFACE_RECONCILE_SURFACE_BYTES / ui_contract::UI_DOCUMENT_NODES,
    );
}

/// 📊️ One Home-shaped table row: six cells, five row actions and a row activation, each binding carrying its own
/// one-entry `spaceId` argument map — the heaviest row an app ships today.
fn census_law_table_row(index: usize) -> crate::TreeNode {
    census_law_table_row_shaped(index, true)
}

/// 📊️ A light table row: two cells and one argument-free row action, no row activation.
fn census_law_light_table_row(index: usize) -> crate::TreeNode {
    census_law_table_row_shaped(index, false)
}

fn census_law_table_row_shaped(index: usize, heavy: bool) -> crate::TreeNode {
    let key = format!("space:{index:04}");
    let binding = |name: &str| {
        let mut args = ui_contract::UiMapBuilder::try_new().expect("bounded fixture args");
        args.push("spaceId".to_string(), ui_contract::UiValue::Text(ui_text(&key))).expect("bounded fixture arg");
        ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::try_v1("s.space.home", name).expect("bounded fixture action"), args: Some(ui_contract::UiValue::Map(args.finish())), capability: None }
    };
    let name = format!("Studio {index}");
    let all_cells = [name.as_str(), "Atelier", "Private", "1", "2026-09-25 23:05", "Hub"];
    let mut cells = ui_contract::UiFixedList::default();
    for cell in if heavy { &all_cells[..] } else { &all_cells[..2] } {
        cells.try_push(ui_text(cell)).expect("bounded fixture cell");
    }
    let all_actions = [("folder-open", "openSpace"), ("pencil", "renameSpace"), ("link", "shareSpace"), ("trash-2", "deleteSpace"), ("users", "manageSpace")];
    let mut row_actions = ui_contract::UiFixedList::default();
    for (icon, name) in if heavy { &all_actions[..] } else { &all_actions[..1] } {
        let action = if heavy { binding(name) } else { ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::try_v1("s.space.home", name).expect("bounded fixture action"), args: None, capability: None } };
        row_actions.try_push(ui_contract::RowAction { icon: ui_text(icon), label: Some(ui_contract::Label::try_from(*name).expect("bounded fixture label")), action, placement: Default::default() }).expect("bounded fixture row action");
    }
    let mut node = crate::TreeNode::try_new(&key, ui_contract::Component::TableRow(ui_contract::TableRowProps { cells, row_actions })).expect("bounded fixture row");
    if heavy {
        node.bindings.try_push(binding("openSpace")).expect("bounded fixture activation");
    }
    node
}

fn census_law_table_document(rows: usize, row: fn(usize) -> crate::TreeNode) -> crate::ComponentTree {
    let mut columns = ui_contract::UiFixedList::default();
    for column in ["Name", "Kind", "Visibility", "Members", "Updated", "Origin"] {
        columns.try_push(ui_contract::Label::try_from(column).expect("bounded fixture column")).expect("bounded fixture columns");
    }
    let props = ui_contract::TableProps { label: ui_contract::Label::try_from("Studios").expect("bounded fixture label"), columns, actions_label: Some(ui_contract::Label::try_from("Actions").expect("bounded fixture label")), window: Some(ui_contract::TreeWindow { row_extent: Default::default(), total: 500, offset: 0 }) };
    let table = crate::TreeNode::try_new("framework.window.table", ui_contract::Component::Table(props)).expect("bounded fixture table");
    tree(table.try_with_children((0..rows).map(row).collect::<Vec<_>>()).unwrap_or_else(|_| panic!("bounded fixture table rows")))
}

/// 📏️ [`surface_subtree_items`] prices a document at least what its fresh reconciliation consumes (the one extra
/// item is the reconciler's root seed) and at most twice that, for tree pick rows, tree rows bound with a
/// four-entry argument map, light table rows and Home-shaped table rows, and their containers alone.
#[test]
fn subtree_item_census_bounds_the_items_a_fresh_reconciliation_consumes() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let trees = [0, 1, 8, 16].into_iter().map(|rows| (format!("tree {rows}"), tree_window_law_document(0, rows)));
    let heavy = [0, 1, 8, 28].into_iter().map(|rows| (format!("heavy table {rows}"), census_law_table_document(rows, census_law_table_row)));
    let light = [1, 60].into_iter().map(|rows| (format!("light table {rows}"), census_law_table_document(rows, census_law_light_table_row)));
    for (name, document) in trees.chain(heavy).chain(light) {
        let priced = surface_subtree_items(&document.root).expect("bounded fixture census");
        let (reconciled, usage) = tree_window_law_reconcile(&SurfaceReconciler::new("panel:census"), document).expect("bounded fixture reconciles");
        let consumed = usage.items - 1;
        assert!(priced >= consumed, "{name}: priced {priced} under the {consumed} items the reconciliation consumed ({usage:?})");
        assert!(priced <= consumed * 2, "{name}: priced {priced} more than twice the {consumed} items the reconciliation consumed");
        let mut retired = reconciled;
        while !retired.retire_one() {}
    }
}

//#endregion 🪟️TreeWindowReconcileSizeLaw
