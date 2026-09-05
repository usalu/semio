use super::*;
use super::super::{close_surface_reconcile_handback_one, release_surface_reconcile_handback, reserve_surface_reconcile_handback, take_surface_reconcile_terminal, SurfaceReconcileHandbackReservation, SurfaceReconcileJob, SurfaceReconciler, SurfaceReconcileTerminal, SURFACE_RECONCILE_HANDBACKS, SURFACE_RECONCILE_HANDBACK_SLOTS};

struct HandbackReservations(Vec<SurfaceReconcileHandbackReservation>);

impl Drop for HandbackReservations {
    fn drop(&mut self) {
        for reservation in self.0.drain(..) { release_surface_reconcile_handback(reservation); }
    }
}

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap() }
fn built_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../🧬️contract/♻️retirement/🌲️built/🧫️fixture/🔣️.json")).unwrap() }

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
fn runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads() {
    let fixture = fixture();
    let components: serde_json::Value = serde_json::from_str(include_str!("../../../../🧬️contract/♻️retirement/🌳️typed/🧩️components.json")).unwrap();
    let foreign: ui_contract::UiValue = serde_json::from_value(fixture["foreign"].clone()).unwrap();
    for grant in fixture["grants"].as_array().unwrap() {
        for row in components["cases"].as_array().unwrap() {
            let mut cursor = SurfaceTreeRetireCursor::default();
            let mut source = Some(populated(serde_json::from_value(row["component"].clone()).unwrap()));
            assert!(cursor.try_begin_node(&mut source)); assert!(source.is_none());
            assert_eq!(cursor.close_step(0, 4096).unwrap(), Default::default());
            assert_eq!(cursor.close_step(1, 0).unwrap(), Default::default());
            let mut blocked = Some(crate::ComponentTree { root: text_node("blocked", "🌊") });
            let mut blocked_node = Some(text_node("blocked-node", "🌊"));
            let mut blocked_held = Some((Some(9), text_node("blocked-held", "🌊")));
            let source_identity = blocked.as_ref().unwrap() as *const crate::ComponentTree;
            let node_identity = blocked_node.as_ref().unwrap() as *const crate::TreeNode;
            let held_identity = blocked_held.as_ref().unwrap() as *const (Option<usize>, crate::TreeNode);
            let owner_identity = cursor.owner.as_ref().unwrap() as *const ui_contract::BuiltTreeRetirement;
            assert!(!cursor.try_begin_tree(&mut blocked));
            assert!(!cursor.try_begin_node(&mut blocked_node));
            assert!(!cursor.try_begin_held(&mut blocked_held));
            assert_eq!(blocked.as_ref().unwrap() as *const crate::ComponentTree, source_identity);
            assert_eq!(blocked_node.as_ref().unwrap() as *const crate::TreeNode, node_identity);
            assert_eq!(blocked_held.as_ref().unwrap() as *const (Option<usize>, crate::TreeNode), held_identity);
            assert_eq!(cursor.owner.as_ref().unwrap() as *const ui_contract::BuiltTreeRetirement, owner_identity);
            assert_eq!(close(&mut cursor, grant.as_u64().unwrap() as usize), row["bytes"].as_u64().unwrap() as usize + built_fixture()["extraPayloadBytes"].as_u64().unwrap() as usize);
            assert!(cursor.try_begin_tree(&mut blocked)); assert!(blocked.is_none());
            assert_eq!(close(&mut cursor, 1), "blocked🌊".len());
            assert!(cursor.try_begin_node(&mut blocked_node)); assert!(blocked_node.is_none());
            assert_eq!(close(&mut cursor, 1), "blocked-node🌊".len());
            assert!(cursor.try_begin_held(&mut blocked_held)); assert!(blocked_held.is_none());
            assert_eq!(close(&mut cursor, 1), "blocked-held🌊".len());
            assert_eq!(serde_json::to_value(&foreign).unwrap(), fixture["foreign"]);
        }
    }
    let mut foreign = ui_contract::UiValueRetirement::new(foreign);
    for _ in 0..10_000 { if foreign.close_step(1, 4096).unwrap().complete { break; } }
    assert!(foreign.terminal_is_empty());
    eprintln!("[DEBUG] runtime tree exact owner:18 components*3 grants; occupied node/tree/held sources preserved; bytes matched; foreign untouched");
}

#[test]
fn runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission() {
    let fixture = fixture();
    let pages = fixture["pages"].as_u64().unwrap() as usize;
    for maintenance in [false, true] {
        let generation = 91_401 + u64::from(maintenance);
        let mut terminal = SurfaceReconcileTerminal::try_from_reconciler(SurfaceReconciler::new("retire"), generation).unwrap();
        let state = terminal.state.as_mut().unwrap();
        let mut source = Some(crate::ComponentTree { root: chain(pages) });
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
    eprintln!("[DEBUG] runtime partial tree:384 pages preserved by exact handback; take+close and queued maintenance restored full admission");
}

#[test]
fn runtime_tree_retirement_rejected_close_preserves_source_until_handback_admission() {
    let generation = 91_403;
    let mut reservations = HandbackReservations(Vec::new());
    while let Some(reservation) = reserve_surface_reconcile_handback(generation) { reservations.0.push(reservation); }
    assert_eq!(reservations.0.len(), SURFACE_RECONCILE_HANDBACK_SLOTS);
    let node = populated(ui_contract::Component::Separator(ui_contract::SeparatorProps {}));
    let mut rejected = SurfaceReconcileJob::try_new(SurfaceReconciler::new("retire"), crate::ComponentTree { root: node }, generation).unwrap_err();
    let source = rejected.state.as_ref().unwrap().source.as_ref().unwrap() as *const crate::ComponentTree;
    let fault = rejected.state.as_ref().unwrap().fault.clone();
    assert!(!rejected.close_step());
    assert_eq!(rejected.state.as_ref().unwrap().source.as_ref().unwrap() as *const crate::ComponentTree, source);
    assert_eq!(rejected.state.as_ref().unwrap().fault, fault);
    assert!(rejected.state.as_ref().unwrap().retire_tree.is_empty());
    release_surface_reconcile_handback(reservations.0.pop().unwrap());
    for _ in 0..100_000 { if rejected.close_step() { break; } }
    assert!(rejected.terminal_is_empty());
    eprintln!("[DEBUG] runtime rejected close:full handback admission preserved exact typed source+fault; one released slot allowed bounded terminal closure");
}
