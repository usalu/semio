//! ⚙️ Imperative app engine — the app's own stateful host over the artifact's pure `ProcedureSnapshot`.
//! Relocated from the deleted artifact-tree `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): an artifact is a schema + io, never an
//! engine; behaviour belongs to the app that edits it. `ImperativeHost` owns `&mut self` execution
//! state (`registry`, `next_serial`) — the textbook D5 Behavioral case — and `imperative_io()` returns
//! `AppIo`, this app's typed media surface. `default_snapshot()` stayed at `🧬️schema` (pure, no app
//! type in its signature, and still needed by the artifact's own mutation/diff tests).

use crate::{Dictionary, ProcedureSnapshot, Path, PathRef, Registry, Step};
use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, Executor, RunResult};

//#region ⚠️ Errors
/// 🚨️ Imperative core's fallible operations.
#[derive(Debug)]
pub enum ImperativeCoreError {
    Json(dsl::ValueError),
    UnsupportedSchema(String),
    MissingOwner,
    MissingSlot,
    UnknownOwnerStep(String),
    UnknownStep(String),
}

impl std::fmt::Display for ImperativeCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::UnsupportedSchema(schema) => write!(formatter, "unsupported schema: {schema}"),
            Self::MissingOwner => formatter.write_str("missing owner"),
            Self::MissingSlot => formatter.write_str("missing slot"),
            Self::UnknownOwnerStep(step) => write!(formatter, "unknown owner step: {step}"),
            Self::UnknownStep(step) => write!(formatter, "unknown step: {step}"),
        }
    }
}

impl std::error::Error for ImperativeCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            _ => None,
        }
    }
}

impl From<dsl::ValueError> for ImperativeCoreError {
    fn from(error: dsl::ValueError) -> Self {
        Self::Json(error)
    }
}
//#endregion ⚠️ Errors

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifact_kind()` already declares (`computation.procedure`, reused
/// verbatim as this port's `kind_id`), plus one extra output port: `result:out`, the imperative path's
/// last `run` scope as a generic data value (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub fn imperative_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::PROCEDURE_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Procedure },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: Some("computation.procedure".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "computation.procedure".into(), name: "Procedure".into(), dimension: "graph".into(), component_kind: "procedure".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Host
/// 🎛️ Native imperative path host. `path`/`seed` are the LIVE working representation this host
/// mutates directly (matches `📓️wave4-reports/flow-report.md`'s working-scene pattern); `document`
/// is kept in sync via [`Self::sync_document`] after every mutating call so its `flow`/`text`
/// composed-child handles always reflect the current `path`/`seed` (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `ProcedureSnapshot` no longer carries `path`/
/// `seed` inline). `document` stays `pub` for API parity with the pre-migration shape; `path`/`seed`
/// are the ones every method here actually reads/writes.
pub struct ImperativeHost {
    pub document: ProcedureSnapshot,
    path: Path,
    seed: std::collections::BTreeMap<String, neural_engine::Value>,
    registry: Registry,
    next_serial: u64,
}

impl Default for ImperativeHost {
    fn default() -> Self {
        Self::from_snapshot(crate::schema::default_snapshot())
    }
}

impl ImperativeHost {
    /// 🌱 Reads the live `path`/`seed` off `document`'s working scene (the cache, keyed by the
    /// snapshot's own `flow`/`text` handles — see `ProcedureWorkingScene`'s doc comment for the
    /// staleness gap this inherits in a fresh process with an unseeded cache).
    pub fn from_snapshot(document: ProcedureSnapshot) -> Self {
        crate::standards::v1::subsets::any::io::bootstrap_imperative_runtime();
        let scene = crate::procedure_working_scene(&document);
        Self { document, path: scene.path, seed: scene.seed, registry: imperative_module_registry(), next_serial: 100 }
    }

    pub fn load_json(json: &str) -> Result<Self, ImperativeCoreError> {
        let document: ProcedureSnapshot = dsl::os_pack::json::from_json_str(json)?;
        if document.schema != "procedure.document" {
            return Err(ImperativeCoreError::UnsupportedSchema(document.schema));
        }
        Ok(Self::from_snapshot(document))
    }

    pub fn to_json(&self) -> Result<String, ImperativeCoreError> {
        Ok(dsl::os_pack::json::to_json_string(&self.document))
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    /// 🔄 Re-mints `document.flow` from the live `path` (mint+cache, never persisted elsewhere) —
    /// called after every mutating method so `document` never drifts from `path`. `seed` never
    /// changes through this host's own methods, so `document.text` is left as-is.
    fn sync_document(&mut self) {
        self.document.flow = crate::procedure_flow_child_with_owner(&self.path);
    }

    fn resolve_path_mut<'a>(&'a mut self, path_ref: &PathRef) -> Result<&'a mut Path, ImperativeCoreError> {
        if path_ref.owner.is_none() && path_ref.slot.is_none() {
            return Ok(&mut self.path);
        }
        let owner = path_ref.owner.as_ref().ok_or(ImperativeCoreError::MissingOwner)?;
        let slot = path_ref.slot.as_ref().ok_or(ImperativeCoreError::MissingSlot)?;
        let owner_step = self.path.steps.iter_mut().find(|step| step.id == *owner).ok_or_else(|| ImperativeCoreError::UnknownOwnerStep(owner.clone()))?;
        Ok(owner_step.bodies.entry(slot.clone()).or_insert_with(Path::new))
    }

    pub fn add_step(&mut self, kind: &str, index: Option<usize>) -> String {
        self.add_step_at(&PathRef::default(), kind, index).expect("root PathRef always resolves — resolve_path_mut only fails for a non-default owner/slot")
    }

    pub fn add_step_at(&mut self, path_ref: &PathRef, kind: &str, index: Option<usize>) -> Result<String, ImperativeCoreError> {
        self.next_serial += 1;
        let id = format!("step-{}", self.next_serial);
        let step = Step { id: id.clone(), kind: kind.into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() };
        let path = self.resolve_path_mut(path_ref)?;
        let insert_at = index.unwrap_or(path.steps.len()).min(path.steps.len());
        path.steps.insert(insert_at, step);
        self.sync_document();
        Ok(id)
    }

    pub fn remove_step(&mut self, id: &str) -> bool {
        self.remove_step_at(&PathRef::default(), id)
    }

    pub fn remove_step_at(&mut self, path_ref: &PathRef, id: &str) -> bool {
        let path = match self.resolve_path_mut(path_ref) {
            Ok(path) => path,
            Err(_) => return false,
        };
        let before = path.steps.len();
        path.steps.retain(|step| step.id != id);
        let changed = path.steps.len() != before;
        if changed {
            self.sync_document();
        }
        changed
    }

    pub fn move_step(&mut self, id: &str, new_index: usize) -> bool {
        self.move_step_at(&PathRef::default(), id, new_index)
    }

    pub fn move_step_at(&mut self, path_ref: &PathRef, id: &str, new_index: usize) -> bool {
        let path = match self.resolve_path_mut(path_ref) {
            Ok(path) => path,
            Err(_) => return false,
        };
        let Some(current) = path.steps.iter().position(|step| step.id == id) else {
            return false;
        };
        let step = path.steps.remove(current);
        let insert_at = new_index.min(path.steps.len());
        path.steps.insert(insert_at, step);
        self.sync_document();
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        self.set_step_params_at(&PathRef::default(), id, json)
    }

    pub fn set_step_params_at(&mut self, path_ref: &PathRef, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        let params: Dictionary = dsl::os_pack::json::from_json_str(json)?;
        let path = self.resolve_path_mut(path_ref)?;
        let Some(step) = path.steps.iter_mut().find(|step| step.id == id) else {
            return Err(ImperativeCoreError::UnknownStep(id.into()));
        };
        step.params = params;
        self.sync_document();
        Ok(())
    }

    pub fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.path, &crate::seed_dictionary(&self.seed))
    }

    pub fn compile_text(&self) -> String {
        compile_to_text(&self.path)
    }
}
//#endregion 🔖️Host

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
