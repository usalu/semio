
use super::*;
use semio_framework::kernel::{ActorInstanceLifetime, ActorUiPatchReceipt};

fn pack<T: serde::Serialize>(value: &T) -> Vec<u8> {
    let value = serde_json::to_value(value).expect("neutral patch payload serializes");
    store::pack_rt::encode_wire_value(&semio_framework::DslValue::from(&value))
}

fn leaf() -> ui::UiNodeRecord {
    ui::UiNodeRecord {
        id: UiNodeId(1),
        key: ui::UiText::try_from_str("map-root").expect("bounded key"),
        component: ui::Component::Separator(ui::SeparatorProps {}),
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: Default::default(),
    }
}

fn patch_ops() -> Vec<wit_ui::PatchOp> {
    let mut node_children = UiNodeChildren::default();
    node_children.try_push(UiNodeId(2)).expect("one child");
    vec![
        wit_ui::PatchOp::Upsert(wit_ui::PatchUpsert { node: pack(&leaf()) }),
        wit_ui::PatchOp::SetComponent(wit_ui::PatchSetComponent { node: 1, component: pack(&ui::Component::Separator(ui::SeparatorProps {})) }),
        wit_ui::PatchOp::SetLayout(wit_ui::PatchSetLayout { node: 1, layout: pack(&ui::LayoutSpec::default()) }),
        wit_ui::PatchOp::SetActivity(wit_ui::PatchSetActivity { node: 1, activity: pack(&serde_json::json!({ "activity": ui::Activity::default(), "disabled": true })) }),
        wit_ui::PatchOp::SetChildren(wit_ui::PatchSetChildren { node: 1, children: vec![2] }),
        wit_ui::PatchOp::SetStyle(wit_ui::PatchSetStyle { node: 1, style: pack(&ui::StyleSpec::default()) }),
        wit_ui::PatchOp::SetAccessibility(wit_ui::PatchSetAccessibility { node: 1, accessibility: pack(&ui::AccessibilitySpec::default()) }),
        wit_ui::PatchOp::SetBindings(wit_ui::PatchSetBindings { node: 1, bindings: pack(&UiNodeBindings::default()) }),
        wit_ui::PatchOp::SetMenu(wit_ui::PatchSetMenu { node: 1, menu: pack(&Option::<ui::MenuRef>::None) }),
        wit_ui::PatchOp::Remove(2),
        wit_ui::PatchOp::SetRoot(1),
    ]
}

fn wit_patch(instance: u32) -> wit_ui::UiPatch {
    wit_ui::UiPatch { surface: wit_ui::SurfaceRef { instance, surface: "map.main".to_string() }, revision: 9, base_revision: 8, ops: patch_ops() }
}

fn receipt() -> ActorUiPatchReceipt {
    ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { activation_generation: 41, instance_id: 7, guest_lifetime: 3 }, patch_sequence: 1 }
}

fn expected() -> UiPatch {
    let mut children = UiNodeChildren::default();
    children.try_push(UiNodeId(2)).expect("one child");
    let mut ops = UiPatchOps::default();
    for op in [
        UiPatchOp::Upsert(leaf()),
        UiPatchOp::SetComponent { id: UiNodeId(1), component: ui::Component::Separator(ui::SeparatorProps {}) },
        UiPatchOp::SetLayout { id: UiNodeId(1), layout: Default::default() },
        UiPatchOp::SetActivity { id: UiNodeId(1), activity: Default::default(), disabled: true },
        UiPatchOp::SetChildren { id: UiNodeId(1), children },
        UiPatchOp::SetStyle { id: UiNodeId(1), style: Default::default() },
        UiPatchOp::SetAccessibility { id: UiNodeId(1), accessibility: Default::default() },
        UiPatchOp::SetBindings { id: UiNodeId(1), bindings: Default::default() },
        UiPatchOp::SetMenu { id: UiNodeId(1), menu: None },
        UiPatchOp::Remove { id: UiNodeId(2) },
        UiPatchOp::SetRoot { id: UiNodeId(1) },
    ] {
        ops.try_push(op).expect("bounded operation corpus");
    }
    UiPatch { surface: ui::SurfaceId::try_from("7:map.main").expect("bounded surface"), base_revision: ui::UiRevision(8), revision: ui::UiRevision(9), ops }
}

#[test]
fn every_wit_patch_variant_moves_into_one_exact_kernel_owner() {
    let owner = wit_ui_patches_to_kernel(7, 2_097_152, Vec::new(), vec![wit_patch(7)], Some(receipt())).expect("canonical patch");
    let actual = owner.iter().next().expect("one patch");
    assert_eq!(actual, &expected());
    assert_eq!(serde_json::to_value(actual).unwrap(), serde_json::to_value(expected()).unwrap());
}

#[test]
fn emitted_and_returned_channels_are_atomic_bounded_and_drained_once() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixture/🔣️.json")).expect("neutral fixture");
    assert_eq!(fixture["operationKinds"].as_array().unwrap().len(), 11);
    for row in fixture["cases"].as_array().unwrap() {
        let emitted = (0..row["emitted"].as_u64().unwrap()).map(|_| wit_patch(7)).collect();
        let returned = (0..row["returned"].as_u64().unwrap()).map(|_| wit_patch(7)).collect();
        let paired = row["receipt"].as_bool().unwrap().then_some(receipt());
        let result = wit_ui_patches_to_kernel(7, fixture["maximumWireBytes"].as_u64().unwrap() as u32, emitted, returned, paired);
        assert_eq!(result.is_ok(), row["expected"] == "accepted", "{}", row["id"]);
    }
    let empty = wit_ui_patches_to_kernel(7, 2_097_152, Vec::new(), Vec::new(), None).expect("second empty drain");
    assert!(empty.is_empty());
}

#[test]
fn target_budget_receipt_and_malformed_pack_refuse_before_publication() {
    assert!(wit_ui_patches_to_kernel(8, 2_097_152, Vec::new(), vec![wit_patch(7)], Some(receipt())).is_err());
    let wrong_receipt = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { instance_id: 8, ..receipt().lifetime }, ..receipt() };
    assert!(wit_ui_patches_to_kernel(7, 2_097_152, Vec::new(), vec![wit_patch(7)], Some(wrong_receipt)).is_err());
    assert!(wit_ui_patches_to_kernel(7, 1, Vec::new(), vec![wit_patch(7)], Some(receipt())).is_err());
    assert!(wit_ui_patches_to_kernel(7, 2_097_152, Vec::new(), vec![wit_patch(7)], None).is_err());
    let malformed = wit_ui::UiPatch { surface: wit_ui::SurfaceRef { instance: 7, surface: "map.main".to_string() }, revision: 9, base_revision: 8, ops: vec![wit_ui::PatchOp::Upsert(wit_ui::PatchUpsert { node: vec![255] })] };
    assert!(wit_ui_patches_to_kernel(7, 2_097_152, Vec::new(), vec![malformed], Some(receipt())).is_err());
}
