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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
