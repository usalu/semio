//! ⚙️ Imperative app engine — the app's own stateful host over the artifact's pure `ImperativeSnapshot`.
//! Relocated from the deleted artifact-tree `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): an artifact is a schema + io, never an
//! engine; behaviour belongs to the app that edits it. `ImperativeHost` owns `&mut self` execution
//! state (`registry`, `next_serial`) — the textbook D5 Behavioral case — and `imperative_io()` returns
//! `AppIo`, this app's typed media surface. `default_snapshot()` stayed at `🧬️schema` (pure, no app
//! type in its signature, and still needed by the artifact's own mutation/diff tests).

use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, Path, PathRef, Registry, Step};
use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, Executor, RunResult};

//#region ⚠️ Errors
/// 🚨️ Imperative core's fallible operations.
#[derive(Debug, thiserror::Error)]
pub enum ImperativeCoreError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("unsupported schema: {0}")]
    UnsupportedSchema(String),
    #[error("missing owner")]
    MissingOwner,
    #[error("missing slot")]
    MissingSlot,
    #[error("unknown owner step: {0}")]
    UnknownOwnerStep(String),
    #[error("unknown step: {0}")]
    UnknownStep(String),
}
//#endregion ⚠️ Errors

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifacts::imperative::artifact_kind()` already declares (`computation.imperative`, reused
/// verbatim as this port's `kind_id`), plus one extra output port: `result:out`, the imperative path's
/// last `run` scope as a generic data value (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub async fn imperative_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::artifacts::imperative::IMPERATIVE_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Imperative },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: Some("computation.imperative".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "computation.imperative".into(), name: "Imperative".into(), dimension: "graph".into(), component_kind: "imperative".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Host
/// 🎛️ Native imperative path host. `path`/`seed` are the LIVE working representation this host
/// mutates directly (matches `📓️wave4-reports/flow-report.md`'s working-scene pattern); `document`
/// is kept in sync via [`Self::sync_document`] after every mutating call so its `flow`/`text`
/// composed-child handles always reflect the current `path`/`seed` (ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `ImperativeSnapshot` no longer carries `path`/
/// `seed` inline). `document` stays `pub` for API parity with the pre-migration shape; `path`/`seed`
/// are the ones every method here actually reads/writes.
pub struct ImperativeHost {
    pub document: ImperativeSnapshot,
    path: Path,
    seed: std::collections::BTreeMap<String, neural_engine::Value>,
    registry: Registry,
    next_serial: u64,
}

impl Default for ImperativeHost {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::imperative::schema::default_snapshot())
    }
}

impl ImperativeHost {
    /// 🌱 Reads the live `path`/`seed` off `document`'s working scene (the cache, keyed by the
    /// snapshot's own `flow`/`text` handles — see `ImperativeWorkingScene`'s doc comment for the
    /// staleness gap this inherits in a fresh process with an unseeded cache).
    pub async fn from_snapshot(document: ImperativeSnapshot) -> Self {
        crate::artifacts::imperative::standards::v1::subsets::any::io::bootstrap_imperative_runtime();
        let scene = crate::artifacts::imperative::imperative_working_scene(&document);
        Self { document, path: scene.path, seed: scene.seed, registry: imperative_module_registry(), next_serial: 100 }
    }

    pub async fn load_json(json: &str) -> Result<Self, ImperativeCoreError> {
        let document: ImperativeSnapshot = serde_json::from_str(json)?;
        if document.schema != "imperative.document" {
            return Err(ImperativeCoreError::UnsupportedSchema(document.schema));
        }
        Ok(Self::from_snapshot(document))
    }

    pub async fn to_json(&self) -> Result<String, ImperativeCoreError> {
        Ok(serde_json::to_string(&self.document)?)
    }

    pub async fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    /// 🔄 Re-mints `document.flow` from the live `path` (mint+cache, never persisted elsewhere) —
    /// called after every mutating method so `document` never drifts from `path`. `seed` never
    /// changes through this host's own methods, so `document.text` is left as-is.
    async fn sync_document(&mut self) {
        self.document.flow = crate::artifacts::imperative::imperative_flow_child_handle_and_cache(&self.path);
    }

    async fn resolve_path_mut<'a>(&'a mut self, path_ref: &PathRef) -> Result<&'a mut Path, ImperativeCoreError> {
        if path_ref.owner.is_none() && path_ref.slot.is_none() {
            return Ok(&mut self.path);
        }
        let owner = path_ref.owner.as_ref().ok_or(ImperativeCoreError::MissingOwner)?;
        let slot = path_ref.slot.as_ref().ok_or(ImperativeCoreError::MissingSlot)?;
        let owner_step = self.path.steps.iter_mut().find(|step| step.id == *owner).ok_or_else(|| ImperativeCoreError::UnknownOwnerStep(owner.clone()))?;
        Ok(owner_step.bodies.entry(slot.clone()).or_insert_with(Path::new))
    }

    pub async fn add_step(&mut self, kind: &str, index: Option<usize>) -> String {
        self.add_step_at(&PathRef::default(), kind, index).expect("root PathRef always resolves — resolve_path_mut only fails for a non-default owner/slot")
    }

    pub async fn add_step_at(&mut self, path_ref: &PathRef, kind: &str, index: Option<usize>) -> Result<String, ImperativeCoreError> {
        self.next_serial += 1;
        let id = format!("step-{}", self.next_serial);
        let step = Step { id: id.clone(), kind: kind.into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() };
        let path = self.resolve_path_mut(path_ref)?;
        let insert_at = index.unwrap_or(path.steps.len()).min(path.steps.len());
        path.steps.insert(insert_at, step);
        self.sync_document();
        Ok(id)
    }

    pub async fn remove_step(&mut self, id: &str) -> bool {
        self.remove_step_at(&PathRef::default(), id)
    }

    pub async fn remove_step_at(&mut self, path_ref: &PathRef, id: &str) -> bool {
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

    pub async fn move_step(&mut self, id: &str, new_index: usize) -> bool {
        self.move_step_at(&PathRef::default(), id, new_index)
    }

    pub async fn move_step_at(&mut self, path_ref: &PathRef, id: &str, new_index: usize) -> bool {
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

    pub async fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        self.set_step_params_at(&PathRef::default(), id, json)
    }

    pub async fn set_step_params_at(&mut self, path_ref: &PathRef, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        let params: Dictionary = serde_json::from_str(json)?;
        let path = self.resolve_path_mut(path_ref)?;
        let Some(step) = path.steps.iter_mut().find(|step| step.id == id) else {
            return Err(ImperativeCoreError::UnknownStep(id.into()));
        };
        step.params = params;
        self.sync_document();
        Ok(())
    }

    pub async fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.path, &crate::artifacts::imperative::seed_dictionary(&self.seed))
    }

    pub async fn compile_text(&self) -> String {
        compile_to_text(&self.path)
    }
}
//#endregion 🔖️Host

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn imperative_io_declares_result_out_reusing_the_computation_imperative_kind() {
        let io = imperative_io();
        assert_eq!(io.document_schema, "imperative.document/v1");
        assert_eq!(io.artifact.id, "computation.imperative");
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "result:out");
        assert_eq!(port.kind_id.as_deref(), Some("computation.imperative"));
        assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert!(!port.required);
    }

    #[semio_framework_async_macros::async_test]
    async fn host_runs_default_snapshot() {
        let host = ImperativeHost::default();
        let result = host.run();
        assert_eq!(result.effects.len(), 2);
        assert!(result.effects.iter().all(|entry| entry.error.is_none()));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_adds_nested_step_in_control_body() {
        let mut host = ImperativeHost::default();
        let owner = host.add_step("control.if", None);
        let path_ref = PathRef { owner: Some(owner.clone()), slot: Some("then".into()) };
        let nested = host.add_step_at(&path_ref, "log.print", None).expect("add nested");
        assert_eq!(nested, "step-102");
        let owner_step = host.path.steps.iter().find(|step| step.id == owner).expect("owner");
        assert_eq!(owner_step.bodies.get("then").map(|path| path.steps.len()), Some(1));
    }

    #[semio_framework_async_macros::async_test]
    async fn imperative_core_error_messages() {
        assert_eq!(ImperativeCoreError::MissingOwner.to_string(), "missing owner");
        assert_eq!(ImperativeCoreError::MissingSlot.to_string(), "missing slot");
        assert_eq!(ImperativeCoreError::UnsupportedSchema("bad.schema".into()).to_string(), "unsupported schema: bad.schema");
        assert_eq!(ImperativeCoreError::UnknownOwnerStep("step-9".into()).to_string(), "unknown owner step: step-9");
        assert_eq!(ImperativeCoreError::UnknownStep("step-9".into()).to_string(), "unknown step: step-9");
    }

    #[semio_framework_async_macros::async_test]
    async fn host_load_json_rejects_unsupported_schema() {
        let json = r#"{"schema":"not.imperative","flow":{"childId":"f","target":{"artifactId":"f","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"flow"}}},"text":{"childId":"t","target":{"artifactId":"t","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"text"}}}}"#;
        assert!(matches!(ImperativeHost::load_json(json), Err(ImperativeCoreError::UnsupportedSchema(schema)) if schema == "not.imperative"));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_load_json_rejects_invalid_json() {
        assert!(matches!(ImperativeHost::load_json("not json"), Err(ImperativeCoreError::Json(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_load_json_and_to_json_round_trip() {
        let json = ImperativeHost::default().to_json().expect("serializes");
        let host = ImperativeHost::load_json(&json).expect("parses back");
        assert_eq!(host.to_json().expect("serializes again"), json);
    }

    #[semio_framework_async_macros::async_test]
    async fn host_catalogue_json_is_nonempty() {
        assert!(!ImperativeHost::default().catalogue_json().is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn host_add_step_at_reports_missing_owner_and_slot() {
        let mut host = ImperativeHost::default();
        let missing_owner = PathRef { owner: None, slot: Some("then".into()) };
        assert!(matches!(host.add_step_at(&missing_owner, "log.print", None), Err(ImperativeCoreError::MissingOwner)));
        let missing_slot = PathRef { owner: Some("step-1".into()), slot: None };
        assert!(matches!(host.add_step_at(&missing_slot, "log.print", None), Err(ImperativeCoreError::MissingSlot)));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_add_step_at_reports_unknown_owner_step() {
        let mut host = ImperativeHost::default();
        let path_ref = PathRef { owner: Some("does-not-exist".into()), slot: Some("then".into()) };
        assert!(matches!(host.add_step_at(&path_ref, "log.print", None), Err(ImperativeCoreError::UnknownOwnerStep(owner)) if owner == "does-not-exist"));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_add_step_clamps_out_of_range_index() {
        let mut host = ImperativeHost::default();
        let before = host.path.steps.len();
        let id = host.add_step("log.print", Some(9999));
        assert_eq!(host.path.steps.last().map(|step| &step.id), Some(&id));
        assert_eq!(host.path.steps.len(), before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn host_remove_step_false_for_unresolvable_path_ref_and_unknown_id() {
        let mut host = ImperativeHost::default();
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(!host.remove_step_at(&bad_path_ref, "step-1"));
        assert!(!host.remove_step("does-not-exist"));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_remove_step_true_when_removed() {
        let mut host = ImperativeHost::default();
        assert!(host.remove_step("step-1"));
        assert!(host.path.steps.iter().all(|step| step.id != "step-1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_move_step_false_for_unresolvable_path_ref_and_unknown_id() {
        let mut host = ImperativeHost::default();
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(!host.move_step_at(&bad_path_ref, "step-1", 0));
        assert!(!host.move_step("does-not-exist", 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_move_step_true_and_reorders() {
        let mut host = ImperativeHost::default();
        assert!(host.move_step("step-2", 0));
        assert_eq!(host.path.steps[0].id, "step-2");
    }

    #[semio_framework_async_macros::async_test]
    async fn host_set_step_params_at_rejects_invalid_json_and_unknown_step() {
        let mut host = ImperativeHost::default();
        assert!(matches!(host.set_step_params_json("step-1", "not json"), Err(ImperativeCoreError::Json(_))));
        assert!(matches!(host.set_step_params_json("does-not-exist", "{}"), Err(ImperativeCoreError::UnknownStep(id)) if id == "does-not-exist"));
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(matches!(host.set_step_params_at(&bad_path_ref, "step-1", "{}"), Err(ImperativeCoreError::UnknownOwnerStep(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_set_step_params_updates_existing_step() {
        use neural_engine::{Atom, Value};
        let mut host = ImperativeHost::default();
        host.set_step_params_json("step-2", r#"{"message":"updated"}"#).expect("sets params");
        let step = host.path.steps.iter().find(|step| step.id == "step-2").expect("step-2 exists");
        assert_eq!(step.params.get("message"), Some(&Value::Atom(Atom::String("updated".into()))));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_compile_text_contains_step_kinds() {
        let host = ImperativeHost::default();
        let compiled = host.compile_text();
        assert!(compiled.contains("state.set"));
        assert!(compiled.contains("log.print"));
    }
}
//#endregion 🧪️Tests
