//! 💾️ Media exchange of the procedural module: the solid export and import handlers and the command schema. See [`super`].

use super::*;

//#region 🔖️MediaExport
/// 📤️ Handles `Command::ExportSolid`: re-evaluates the active host_snapshot, exports every preview geometry handle through `flow` brep geometry session's STEP/OBJ/STL kernel codecs (GLB bridges through mesh tessellation), and stashes the JSON result on `params.__solidExport` for the host shell to read back.
pub(super) fn handle_export_solid(owner: &mut ModuleGeometryOwner, payload: &mut ModuleRenderPayload, format: &str) {
    let slug = if payload.example_id.is_empty() { "hexagonal-mushroom-column" } else { payload.example_id.as_str() };
    let Some(snapshot_json) = learning_program_json(slug) else {
        return;
    };
    let host_snapshot: FlowHostSnapshot = semio_framework_pack_json::from_json_str(snapshot_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| FlowHostSnapshot::default());
    let params = params_as_json(&payload.params);
    let mut handles = evaluated_preview_geometry_handles(owner, &host_snapshot, &params);
    if let Ok(imported) = imported_geometry_handles(owner, &params) {
        for handle in imported { if !handles.contains(&handle) { handles.push(handle); } }
    }
    host_snapshot.retire_cold();
    owner.retain_current(&handles);
    let result_json =
        if handles.is_empty() { semio_framework_pack_json::json!({ "error": "no procedural solid geometry to export" }) } else { parse_json(&owner.session.export_solid_json(&handles, format, SOLID_EXPORT_DEFLECTION), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(semio_framework_pack_json::json!({ "error": "export failed" })) };
    let mut object = params_as_json(&payload.params);
    let Some(map) = object.as_object_mut() else {
        return;
    };
    map.insert("__solidExport", result_json);
    payload.params = json_to_dsl_value(&object);
}

/// 📥️ Handles `Command::ImportSolid`: imports `data` (UTF-8 text for STEP/OBJ, base64 for STL/GLB) as `format` through `flow` brep geometry session's in-process kernel (GLB bridges through mesh tessellation into an OBJ ingestion) and persists the replayable interchange source on `params.__solidImport`.
pub(super) fn handle_import_solid(owner: &mut ModuleGeometryOwner, payload: &mut ModuleRenderPayload, format: &str, data: &str) {
    let source = semio_framework_pack_json::json!({ "format": format, "data": data, "tolerance": SOLID_IMPORT_TOLERANCE });
    let params = semio_framework_pack_json::json!({ "__solidImport": source });
    let result_json = if data.is_empty() { semio_framework_pack_json::json!({ "error": "no import data provided" }) } else {
        match imported_geometry_handles(owner, &params) {
            Ok(_) => source,
            Err(_) => semio_framework_pack_json::json!({ "error": "import failed" }),
        }
    };
    let mut object = params_as_json(&payload.params);
    let Some(map) = object.as_object_mut() else {
        return;
    };
    map.insert("__solidImport", result_json);
    payload.params = json_to_dsl_value(&object);
}

pub(super) fn imported_geometry_handles(owner: &mut ModuleGeometryOwner, params: &Value) -> Result<Vec<String>, Fault> {
    let Some(source) = params.get("__solidImport").filter(|source| source.get("error").is_none()) else {
        owner.retire_import_cache();
        return Ok(Vec::new());
    };
    let source_json = json_to_string(source);
    if let Some(cache) = &owner.imported {
        if cache.source == source_json { return Ok(cache.handles.clone()); }
    }
    owner.retire_import_cache();
    let format = source.get("format").and_then(Value::as_str).ok_or_else(|| Fault::from("playbook.module.procedural.import-source-format"))?;
    let data = source.get("data").and_then(Value::as_str).ok_or_else(|| Fault::from("playbook.module.procedural.import-source-data"))?;
    let tolerance = source.get("tolerance").and_then(Value::as_f64).ok_or_else(|| Fault::from("playbook.module.procedural.import-source-tolerance"))?;
    let result = parse_json(&owner.session.import_solid_json(format, data, tolerance), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| Fault::from("playbook.module.procedural.import-source-invalid"))?;
    if result.get("error").is_some() { return Err(Fault::from("playbook.module.procedural.import-source-failed")); }
    let values = result.get("handles").and_then(Value::as_array).ok_or_else(|| Fault::from("playbook.module.procedural.import-handles-missing"))?;
    let handles: Vec<String> = values.iter().map(|value| value.as_str().filter(|handle| is_brep_geometry_handle(handle)).map(str::to_owned).ok_or_else(|| Fault::from("playbook.module.procedural.import-handle-invalid"))).collect::<Result<_, _>>()?;
    owner.retire_import_cache();
    owner.imported = Some(ModuleImportedGeometry { source: source_json, handles: handles.clone() });
    Ok(handles)
}
//#endregion 🔖️Params

//#region 🔖️Command
/// 🎯️ B1: this module's `ArtifactApp::Command` — the SOLE dispatch surface for the solid
/// import/export behavior previously routed through the deleted stringly-typed `handle_action`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
pub enum Command {
    #[dsl(key = "export-solid")]
    ExportSolid { format: String },
    #[dsl(key = "import-solid")]
    ImportSolid { format: String, data: String },
}

/// 🎯️ Handcrafted OpBinary (P6) — `DslOps` emits `DslVariants` only.
impl protocol::OpBinary for Command {
    /// 🎯️ The two manifest ids this command schema owns. `app_commands!` fills this from its own rows
    /// via `ToolCommandCatalog`; a HAND-WRITTEN `OpBinary` silently keeps the trait's
    /// `&["typed-command"]` fallback instead. `tool_job_registration` joins these ids against the
    /// generated `Migrated` declarations and the live concrete factories, and a join that matches
    /// nothing is a `panic!` in app construction — which in a guest is a `wasm trap: unreachable`, so
    /// the whole app fails to spawn. Measured 2026-09-21 on the rebuilt guest: `tool proof catalog
    /// must exactly join migrated generated declarations to live concrete factories` (ticket
    /// 26/09/18 S10 §3.2).
    const TOOL_JOB_IDS: &'static [&'static str] = MODULE_RETAINED_TOOL_IDS;

    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command
