//! 🩹️ Exact reverse WIT-to-kernel UI patch ownership bridge shared by both native actor hosts.

use super::wit_ui;
use semio_framework::kernel::{ActorUiPatchReceipt, UiTurnPatches, UI_TURN_PATCHES_MAXIMUM};
use semio_framework_ui_contract::{
    self as ui, UiNodeBindings, UiNodeChildren, UiNodeId, UiPatch, UiPatchOp, UiPatchOps,
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ActivityPatchPayload {
    activity: ui::Activity,
    disabled: bool,
}

fn checked_add(total: &mut usize, value: usize) -> Result<(), &'static str> {
    *total = total.checked_add(value).ok_or("ui patch wire length overflow")?;
    Ok(())
}

fn patch_wire_bytes(patch: &wit_ui::UiPatch) -> Result<usize, &'static str> {
    if patch.ops.len() > ui::UI_DOCUMENT_PATCH_OPS {
        return Err("ui patch operation capacity exceeded");
    }
    let mut bytes = 32usize;
    checked_add(&mut bytes, patch.surface.surface.len())?;
    for op in &patch.ops {
        checked_add(&mut bytes, 16)?;
        match op {
            wit_ui::PatchOp::Upsert(value) => checked_add(&mut bytes, value.node.len())?,
            wit_ui::PatchOp::SetComponent(value) => checked_add(&mut bytes, value.component.len())?,
            wit_ui::PatchOp::SetLayout(value) => checked_add(&mut bytes, value.layout.len())?,
            wit_ui::PatchOp::SetActivity(value) => checked_add(&mut bytes, value.activity.len())?,
            wit_ui::PatchOp::SetChildren(value) => checked_add(&mut bytes, value.children.len().checked_mul(8).ok_or("ui patch child length overflow")?)?,
            wit_ui::PatchOp::SetStyle(value) => checked_add(&mut bytes, value.style.len())?,
            wit_ui::PatchOp::SetAccessibility(value) => checked_add(&mut bytes, value.accessibility.len())?,
            wit_ui::PatchOp::SetBindings(value) => checked_add(&mut bytes, value.bindings.len())?,
            wit_ui::PatchOp::SetMenu(value) => checked_add(&mut bytes, value.menu.len())?,
            wit_ui::PatchOp::Remove(_) | wit_ui::PatchOp::SetRoot(_) => {}
        }
    }
    Ok(bytes)
}

fn decode_pack<T: serde::de::DeserializeOwned>(bytes: &[u8], field: &'static str) -> Result<T, String> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| format!("{field}: {error}"))?;
    if store::pack_rt::encode_wire_value(&value) != bytes {
        return Err(format!("{field}: non-canonical pack"));
    }
    serde_json::from_value(serde_json::Value::from(&value)).map_err(|error| format!("{field}: {error}"))
}

fn children(values: Vec<u64>) -> Result<UiNodeChildren, String> {
    let mut children = UiNodeChildren::default();
    for value in values {
        children.try_push(UiNodeId(value)).map_err(|_| "ui patch children capacity exceeded".to_string())?;
    }
    Ok(children)
}

fn operation(value: wit_ui::PatchOp) -> Result<UiPatchOp, String> {
    Ok(match value {
        wit_ui::PatchOp::Upsert(value) => UiPatchOp::Upsert(decode_pack(&value.node, "ui patch upsert node")?),
        wit_ui::PatchOp::SetComponent(value) => UiPatchOp::SetComponent { id: UiNodeId(value.node), component: decode_pack(&value.component, "ui patch component")? },
        wit_ui::PatchOp::SetLayout(value) => UiPatchOp::SetLayout { id: UiNodeId(value.node), layout: decode_pack(&value.layout, "ui patch layout")? },
        wit_ui::PatchOp::SetActivity(value) => {
            let payload: ActivityPatchPayload = decode_pack(&value.activity, "ui patch activity")?;
            UiPatchOp::SetActivity { id: UiNodeId(value.node), activity: payload.activity, disabled: payload.disabled }
        }
        wit_ui::PatchOp::SetChildren(value) => UiPatchOp::SetChildren { id: UiNodeId(value.node), children: children(value.children)? },
        wit_ui::PatchOp::SetStyle(value) => UiPatchOp::SetStyle { id: UiNodeId(value.node), style: decode_pack(&value.style, "ui patch style")? },
        wit_ui::PatchOp::SetAccessibility(value) => UiPatchOp::SetAccessibility { id: UiNodeId(value.node), accessibility: decode_pack(&value.accessibility, "ui patch accessibility")? },
        wit_ui::PatchOp::SetBindings(value) => UiPatchOp::SetBindings { id: UiNodeId(value.node), bindings: decode_pack::<UiNodeBindings>(&value.bindings, "ui patch bindings")? },
        wit_ui::PatchOp::SetMenu(value) => UiPatchOp::SetMenu { id: UiNodeId(value.node), menu: decode_pack(&value.menu, "ui patch menu")? },
        wit_ui::PatchOp::Remove(id) => UiPatchOp::Remove { id: UiNodeId(id) },
        wit_ui::PatchOp::SetRoot(id) => UiPatchOp::SetRoot { id: UiNodeId(id) },
    })
}

fn patch(value: wit_ui::UiPatch, expected_instance: u32) -> Result<UiPatch, String> {
    if value.surface.instance != expected_instance || value.surface.surface.is_empty() {
        return Err("ui patch surface target mismatch".to_string());
    }
    let surface = ui::SurfaceId::try_from(format!("{}:{}", value.surface.instance, value.surface.surface)).map_err(|_| "ui patch surface identity invalid".to_string())?;
    let mut ops = UiPatchOps::default();
    for value in value.ops {
        let op = operation(value)?;
        ops.try_push(op).map_err(|_| "ui patch operation capacity exceeded".to_string())?;
    }
    Ok(UiPatch { surface, base_revision: ui::UiRevision(value.base_revision), revision: ui::UiRevision(value.revision), ops })
}

/// 🧷️ Moves exactly one emitted-first WIT patch into the retained kernel turn owner.
pub(super) fn wit_ui_patches_to_kernel(
    expected_instance: u32,
    max_patch_bytes: u32,
    emitted: Vec<wit_ui::UiPatch>,
    returned: Vec<wit_ui::UiPatch>,
    receipt: Option<ActorUiPatchReceipt>,
) -> Result<UiTurnPatches, String> {
    let count = emitted.len().checked_add(returned.len()).ok_or_else(|| "ui patch count overflow".to_string())?;
    ActorUiPatchReceipt::validate_pairing(receipt, count).map_err(|error| error.to_string())?;
    if receipt.is_some_and(|receipt| receipt.lifetime.instance_id != expected_instance) {
        return Err("ui patch receipt instance mismatch".to_string());
    }
    if count > UI_TURN_PATCHES_MAXIMUM {
        return Err("ui patch turn capacity exceeded".to_string());
    }
    let admitted = usize::try_from(max_patch_bytes).map_err(|_| "ui patch byte budget invalid".to_string())?;
    let measured = emitted.iter().chain(&returned).try_fold(0usize, |total, value| {
        let value = patch_wire_bytes(value)?;
        total.checked_add(value).ok_or("ui patch wire length overflow")
    })?;
    if measured > admitted {
        return Err("ui patch byte budget exceeded".to_string());
    }
    let Some(value) = emitted.into_iter().chain(returned).next() else { return Ok(UiTurnPatches::default()) };
    let patch = patch(value, expected_instance)?;
    let mut owner = UiTurnPatches::default();
    owner.try_push_ui_patch(patch).map_err(|_| "ui patch retained owner admission refused".to_string())?;
    Ok(owner)
}

#[cfg(test)]
mod tests {
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
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).expect("neutral fixture");
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
        let malformed = wit_ui::UiPatch {
            surface: wit_ui::SurfaceRef { instance: 7, surface: "map.main".to_string() },
            revision: 9,
            base_revision: 8,
            ops: vec![wit_ui::PatchOp::Upsert(wit_ui::PatchUpsert { node: vec![255] })],
        };
        assert!(wit_ui_patches_to_kernel(7, 2_097_152, Vec::new(), vec![malformed], Some(receipt())).is_err());
    }
}
