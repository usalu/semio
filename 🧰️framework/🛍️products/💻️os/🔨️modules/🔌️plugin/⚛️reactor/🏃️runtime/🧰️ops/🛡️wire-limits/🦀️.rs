//! 🛡️ Public action wire limits, envelope validation and addressed action admission.

use super::*;
// 🚫️async: E1-adjacent pure lookup consumed by `with_instances_mut`'s sync callback
// (`FnOnce(&mut Vec<AppInstance>) -> Result<R, Fault>`, itself required sync by the
// single-threaded `RefCell::borrow_mut` reentrancy guard) — see R9.
pub(super) fn find_instance<'a, PA: PluginApp>(list: &'a RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>, instance_id: u32) -> Result<HostInstanceGuard<'a, PA>, Fault> {
    let cell = list.get(instance_id).ok_or_else(|| plugin_internal_fault(format!("unknown instance: {instance_id}")))?;
    cell.host_instance().ok_or_else(|| plugin_internal_fault(format!("instance busy or poisoned: {instance_id}")))
}
pub(super) fn runtime_instance_cell<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32) -> Result<std::sync::Arc<RuntimeAppCell<PA>>, Fault> {
    runtime.instances.try_borrow().map_err(|_| plugin_internal_fault("runtime instance authority is busy"))?.get(instance_id).cloned().ok_or_else(|| plugin_internal_fault(format!("unknown instance: {instance_id}")))
}
/// 📏️ The ONE public invocation envelope, read off the language-neutral contract
/// (`🛂️manifest/🎛️public-invocation/🧬️schema/🔣️.json`). `MAX_PUBLIC_ACTION_STRING_BYTES` is
/// checked BEFORE the addressed tool's own `max_raw_wire_bytes`, so no tool contract can widen
/// it. `setContributions` is the host pack crossing: it keeps the body cap and skips the per-string
/// cap so one scoped contributions pack can land without 4 KiB paging
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const MAX_PUBLIC_ACTION_BODY_BYTES: usize = semio_framework::PUBLIC_INVOCATION_BODY_BYTES;
const MAX_PUBLIC_ACTION_STRING_BYTES: usize = semio_framework::PUBLIC_INVOCATION_STRING_BYTES;
const MAX_PUBLIC_ACTION_DEPTH: usize = semio_framework::PUBLIC_INVOCATION_DEPTH;
const HOST_CONTRIBUTIONS_PACK_COMMAND_ID: &str = "setContributions";
fn dff_interactive_wire_limit(owner: crate::tool_contract::ToolOwnerWitness, runtime_controller_id: &str, addressed_controller_id: &str, command_id: &str, app_contracts: &[crate::tool_contract::ArtifactToolPublicContract]) -> Result<Option<usize>, Fault> {
    let registration = app_contracts.iter().find(|registration| registration.controller_id == addressed_controller_id && registration.tool_id == command_id);
    match registration {
        Some(registration) if runtime_controller_id == addressed_controller_id && registration.owner == owner && !registration.schema_id.is_empty() => Ok(Some(registration.max_raw_wire_bytes)),
        Some(_) => Err(plugin_internal_fault("addressed app-owned tool is not owned by the live app type/controller")),
        None => Ok(None),
    }
}
fn dff_invocation_wire_limit(body: &str, identifier_key: &str, label: &str, owner: crate::tool_contract::ToolOwnerWitness, runtime_controller_id: &str, app_contracts: &[crate::tool_contract::ArtifactToolPublicContract]) -> Result<Option<usize>, Fault> {
    let bytes = body.as_bytes();
    let mut cursor = 0usize;
    let mut depth = 0usize;
    let mut address_depth = None;
    let mut pending_address_start = None;
    let mut identifier_seen = false;
    let mut controller_seen = false;
    let mut selected_identifier = None;
    let mut selected_controller = None;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'{' => {
                depth = depth.saturating_add(1);
                if pending_address_start == Some(cursor) {
                    address_depth = Some(depth);
                    pending_address_start = None;
                }
                cursor += 1;
            }
            b'}' => {
                if address_depth == Some(depth) {
                    address_depth = None;
                }
                depth = depth.saturating_sub(1);
                cursor += 1;
            }
            b'"' => {
                let key_start = cursor + 1;
                cursor = key_start;
                let mut escaped = false;
                while cursor < bytes.len() && bytes[cursor] != b'"' {
                    if bytes[cursor] == b'\\' {
                        escaped = true;
                        cursor = cursor.saturating_add(1);
                    }
                    cursor = cursor.saturating_add(1);
                }
                if cursor >= bytes.len() {
                    return Err(plugin_internal_fault(format!("{label} contains an unterminated string")));
                }
                let key_end = cursor;
                cursor += 1;
                let mut value_cursor = cursor;
                while value_cursor < bytes.len() && bytes[value_cursor].is_ascii_whitespace() {
                    value_cursor += 1;
                }
                if bytes.get(value_cursor) != Some(&b':') {
                    continue;
                }
                if escaped && (depth == 1 || address_depth.is_some_and(|start| depth >= start)) {
                    return Err(plugin_internal_fault(format!("{label} address keys must be plain bounded identifiers")));
                }
                value_cursor += 1;
                while value_cursor < bytes.len() && bytes[value_cursor].is_ascii_whitespace() {
                    value_cursor += 1;
                }
                let key = &body[key_start..key_end];
                if depth == 1 && key == "address" {
                    if bytes.get(value_cursor) != Some(&b'{') {
                        return Err(plugin_internal_fault(format!("{label} address must be an object")));
                    }
                    pending_address_start = Some(value_cursor);
                } else if address_depth == Some(depth) && key == identifier_key {
                    if identifier_seen {
                        return Err(plugin_internal_fault(format!("{label} address contains a duplicate {identifier_key}")));
                    }
                    identifier_seen = true;
                    if bytes.get(value_cursor) != Some(&b'"') {
                        return Err(plugin_internal_fault(format!("{identifier_key} must be a plain bounded string")));
                    }
                    let value_start = value_cursor + 1;
                    value_cursor = value_start;
                    let mut value_escaped = false;
                    while value_cursor < bytes.len() && bytes[value_cursor] != b'"' {
                        if bytes[value_cursor] == b'\\' {
                            value_escaped = true;
                            value_cursor = value_cursor.saturating_add(1);
                        }
                        value_cursor = value_cursor.saturating_add(1);
                    }
                    if value_cursor >= bytes.len() || value_escaped {
                        return Err(plugin_internal_fault(format!("{identifier_key} must be an unescaped bounded identifier")));
                    }
                    selected_identifier = Some(&body[value_start..value_cursor]);
                } else if address_depth.is_some_and(|start| depth >= start) && key == "appId" {
                    if controller_seen {
                        return Err(plugin_internal_fault(format!("{label} address contains a duplicate appId")));
                    }
                    controller_seen = true;
                    if bytes.get(value_cursor) != Some(&b'"') {
                        return Err(plugin_internal_fault("appId must be a plain bounded string"));
                    }
                    let value_start = value_cursor + 1;
                    value_cursor = value_start;
                    let mut value_escaped = false;
                    while value_cursor < bytes.len() && bytes[value_cursor] != b'"' {
                        if bytes[value_cursor] == b'\\' {
                            value_escaped = true;
                            value_cursor = value_cursor.saturating_add(1);
                        }
                        value_cursor = value_cursor.saturating_add(1);
                    }
                    if value_cursor >= bytes.len() || value_escaped {
                        return Err(plugin_internal_fault("appId must be an unescaped bounded identifier"));
                    }
                    selected_controller = Some(&body[value_start..value_cursor]);
                }
            }
            _ => cursor += 1,
        }
    }
    match selected_controller.zip(selected_identifier) {
        Some((addressed_controller_id, command_id)) => dff_interactive_wire_limit(owner, runtime_controller_id, addressed_controller_id, command_id, app_contracts),
        None => Ok(None),
    }
}
fn public_address_command_id(body: &str) -> Option<String> {
    serde_json::from_str::<Value>(body).ok()?.get("address")?.get("commandId")?.as_str().map(str::to_owned)
}
fn validate_public_json_envelope(body: &str, label: &str) -> Result<(), Fault> {
    validate_public_json_envelope_with(body, label, true)
}
fn validate_public_command_json(body: &str) -> Result<(), Fault> {
    let cap_strings = public_address_command_id(body).as_deref() != Some(HOST_CONTRIBUTIONS_PACK_COMMAND_ID);
    validate_public_json_envelope_with(body, "command", cap_strings)
}
fn validate_public_json_envelope_with(body: &str, label: &str, cap_strings: bool) -> Result<(), Fault> {
    if body.len() > MAX_PUBLIC_ACTION_BODY_BYTES {
        return Err(plugin_internal_fault(format!("{label} exceeds the bounded public action body")));
    }
    let mut depth = 0usize;
    let mut string_bytes = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for byte in body.bytes() {
        if in_string {
            if escaped {
                escaped = false;
                string_bytes = string_bytes.saturating_add(1);
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            } else {
                string_bytes = string_bytes.saturating_add(1);
            }
            if cap_strings && string_bytes > MAX_PUBLIC_ACTION_STRING_BYTES {
                return Err(plugin_internal_fault(format!("{label} contains an oversized string")));
            }
            continue;
        }
        match byte {
            b'"' => {
                in_string = true;
                string_bytes = 0;
            }
            b'{' | b'[' => {
                depth = depth.saturating_add(1);
                if depth > MAX_PUBLIC_ACTION_DEPTH {
                    return Err(plugin_internal_fault(format!("{label} exceeds the bounded nesting depth")));
                }
            }
            b'}' | b']' if depth == 0 => return Err(plugin_internal_fault(format!("{label} closes outside its envelope"))),
            b'}' | b']' => depth -= 1,
            _ => {}
        }
    }
    if in_string || escaped || depth != 0 {
        return Err(plugin_internal_fault(format!("{label} is structurally incomplete")));
    }
    Ok(())
}
fn validate_public_action_envelope(body: &str, owner: crate::tool_contract::ToolOwnerWitness, runtime_controller_id: &str, app_contracts: &[crate::tool_contract::ArtifactToolPublicContract]) -> Result<(), Fault> {
    validate_public_json_envelope(body, "action")?;
    if let Some(limit) = dff_invocation_wire_limit(body, "actionId", "action", owner, runtime_controller_id, app_contracts)? {
        if body.len() > limit {
            return Err(plugin_internal_fault("interactive Draw/Flow/Forms action exceeds its command-specific wire bound"));
        }
    }
    Ok(())
}
fn validate_public_command_envelope(body: &str, owner: crate::tool_contract::ToolOwnerWitness, runtime_controller_id: &str, app_contracts: &[crate::tool_contract::ArtifactToolPublicContract]) -> Result<(), Fault> {
    validate_public_command_json(body)?;
    if let Some(limit) = dff_invocation_wire_limit(body, "commandId", "command", owner, runtime_controller_id, app_contracts)? {
        if body.len() > limit {
            return Err(plugin_internal_fault("interactive Draw/Flow/Forms command exceeds its command-specific wire bound"));
        }
    }
    Ok(())
}
#[cfg(test)]
include!("../../../../🧪️tests/🔬️plugin-runtime-dff-public-action-admission/🦀️.rs");
fn debug_action_dispatch_branch(action_id: &str) -> &'static str {
    if !crate::reserved_jobs::is_framework_reserved_action_id(action_id) {
        "catalog"
    } else if matches!(action_id, "copy" | "cut" | "paste") {
        "clipboard-instack"
    } else if crate::tool_jobs::is_tool_run_action_id(action_id) || crate::app_contract::is_time_travel_action_id(action_id) || action_id == crate::app_views::CANCEL_TYPED_OPERATION_ACTION_ID || action_id == semio_framework::HOST_EVENT_ACTION_ID {
        "host-driven"
    } else {
        "spawn-admit"
    }
}
