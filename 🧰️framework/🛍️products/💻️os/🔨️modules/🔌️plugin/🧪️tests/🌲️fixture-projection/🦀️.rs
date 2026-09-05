//! 🧪️ Exact fixture ownership remains retained across observation success, rejection and panic.

use semio_framework_ui_contract::{BuiltNode, BuiltTreeRetirement, Component, TextProps, UiValue, UiValueRetirement, UI_BUILT_CHILD_RETIRE_SLOTS};

fn node() -> BuiltNode {
    BuiltNode::try_new("K", Component::Text(TextProps { value: "V".try_into().unwrap(), emphasize: None, data_attributes: None })).unwrap()
}

fn close(owner: &mut BuiltTreeRetirement) {
    for _ in 0..1_000_000 { if owner.close_step(1, 4096).unwrap().complete { break; } }
    assert!(owner.terminal_is_empty());
}

#[test]
fn fixture_projection_retires_exact_tree_before_return_error_or_panic() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    assert_eq!(fixture["reservedPages"], UI_BUILT_CHILD_RETIRE_SLOTS);
    let payloads: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/🧫️fixture/🔣️.json")).unwrap();
    let foreign: UiValue = serde_json::from_value(fixture["foreign"].clone()).unwrap();
    for mode in fixture["cases"].as_array().unwrap() {
        let mode = mode.as_str().unwrap();
        let mut root = node();
        for index in 0..UI_BUILT_CHILD_RETIRE_SLOTS {
            let mut parent = node();
            if index % 2 == 0 { parent.children.try_push(root).unwrap(); } else { parent.rejected_children.try_push(root).unwrap(); }
            root = parent;
        }
        root.bindings.try_push(serde_json::from_value(payloads["binding"].clone()).unwrap()).unwrap();
        root.menu = Some(serde_json::from_value(payloads["menu"].clone()).unwrap());
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| super::testkit::observe_and_retire_fixture_tree(super::ComponentTree { root }, |_| -> Result<&str, &str> {
            match mode { "success" => Ok("observed"), "rejection" => Err("rejected"), "panic" => panic!("fixture projection panic"), _ => unreachable!() }
        })));
        match mode {
            "success" => assert_eq!(outcome.unwrap(), Ok("observed")),
            "rejection" => assert_eq!(outcome.unwrap(), Err("rejected")),
            "panic" => assert_eq!(outcome.unwrap_err().downcast_ref::<&str>(), Some(&"fixture projection panic")),
            _ => unreachable!(),
        }
        assert_eq!(serde_json::to_value(&foreign).unwrap(), fixture["foreign"]);
        let mut probe = node();
        for _ in 0..UI_BUILT_CHILD_RETIRE_SLOTS { let mut parent = node(); parent.children.try_push(probe).unwrap(); probe = parent; }
        close(&mut BuiltTreeRetirement::new(probe));
    }
    let mut foreign = UiValueRetirement::new(foreign);
    for _ in 0..10_000 { if foreign.close_step(1, 4096).unwrap().complete { break; } }
    assert!(foreign.terminal_is_empty());
    eprintln!("[DEBUG] fixture projection:success+rejection+panic returned after384 exact pages retired; full admission revalidated,foreign retained");
}
