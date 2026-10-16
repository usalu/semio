use super::*;

struct HandbackReservations(Vec<SurfaceReconcileHandbackReservation>);
impl Drop for HandbackReservations {
    fn drop(&mut self) {
        for reservation in self.0.drain(..) { release_surface_reconcile_handback(reservation); }
    }
}
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn built_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../../🧬️contract/🏗️builder/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap() }
fn text_node(key: &str, value: &str) -> crate::TreeNode {
    ui_contract::BuiltNode::try_new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from(value).unwrap(), emphasize: None, data_attributes: Default::default() })).unwrap()
}
fn populated(component: ui_contract::Component) -> crate::TreeNode {
    let fixture = built_fixture();
    let mut node = crate::TreeNode::try_new("K", component).unwrap();
    node.bindings.try_push(serde_json::from_value(fixture["binding"].clone()).unwrap()).unwrap();
    node.menu = Some(serde_json::from_value(fixture["menu"].clone()).unwrap());
    node.children.try_push(text_node("C", "child")).unwrap();
    node.rejected_children.try_push(text_node("R", "rejected")).unwrap();
    node
}
fn close(cursor: &mut SurfaceTreeRetireCursor, grant: usize) -> usize {
    let mut bytes = 0;
    for _ in 0..1_000_000 {
        let step = cursor.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        bytes += step.released_bytes;
        if step.complete { assert!(cursor.is_empty()); return bytes; }
        if !step.progressed { std::thread::yield_now(); }
    }
    panic!("runtime exact tree owner did not reach terminal");
}
fn chain(pages: usize) -> crate::TreeNode {
    let mut node = text_node("K", "V");
    for index in 0..pages {
        let mut parent = text_node("K", "V");
        let children = if index % 2 == 0 { &mut parent.children } else { &mut parent.rejected_children };
        children.try_push(node).unwrap();
        node = parent;
    }
    node
}
#[test]
fn runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let fixture = fixture();
    let pages = fixture["pages"].as_u64().unwrap() as usize;
    for maintenance in [false, true] {
        let generation = 91_401 + u64::from(maintenance);
        let mut terminal = SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::from_text("retire"), generation).unwrap();
        let state = terminal.state.as_mut().unwrap();
        let mut source = Some(crate::ComponentTree::new(chain(pages)));
        assert!(state.retire_tree.try_begin_tree(&mut source));
        assert!(!state.retire_tree.step());
        let key = terminal.handback_key().unwrap();
        drop(terminal);
        assert!(!SURFACE_RECONCILE_HANDBACKS.lock().unwrap().slots[key.slot].state.as_ref().unwrap().retire_tree.is_empty());
        if maintenance {
            for _ in 0..100_000 { if close_surface_reconcile_handback_one().unwrap() { break; } }
            assert!(SURFACE_RECONCILE_HANDBACKS.lock().unwrap().slots[key.slot].state.is_none());
        } else {
            let mut terminal = take_surface_reconcile_terminal(key).unwrap().unwrap();
            for _ in 0..100_000 { if terminal.close_step() { break; } }
            assert!(terminal.terminal_is_empty());
        }
        let mut source = Some(chain(pages));
        let mut cursor = SurfaceTreeRetireCursor::default();
        assert!(cursor.try_begin_node(&mut source));
        assert_eq!(close(&mut cursor, 1), (pages + 1) * 2);
    }
}
#[test]
fn runtime_tree_retirement_rejected_close_preserves_source_until_handback_admission() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let generation = 91_403;
    let mut reservations = HandbackReservations(Vec::new());
    while let Some(reservation) = reserve_surface_reconcile_handback(generation) { reservations.0.push(reservation); }
    assert_eq!(reservations.0.len(), SURFACE_RECONCILE_HANDBACK_SLOTS);
    let node = populated(ui_contract::Component::Separator(ui_contract::SeparatorProps {}));
    let mut rejected = SurfaceReconcileJob::try_new(SurfaceReconciler::from_text("retire"), crate::ComponentTree::new(node), generation).unwrap_err();
    let source = rejected.state.as_ref().unwrap().source.as_ref().unwrap() as *const crate::ComponentTree;
    let fault = rejected.state.as_ref().unwrap().fault.clone();
    assert!(!rejected.close_step());
    assert_eq!(rejected.state.as_ref().unwrap().source.as_ref().unwrap() as *const crate::ComponentTree, source);
    assert_eq!(rejected.state.as_ref().unwrap().fault, fault);
    assert!(rejected.state.as_ref().unwrap().retire_tree.is_empty());
    release_surface_reconcile_handback(reservations.0.pop().unwrap());
    for _ in 0..100_000 { if rejected.close_step() { break; } }
    assert!(rejected.terminal_is_empty());
}
