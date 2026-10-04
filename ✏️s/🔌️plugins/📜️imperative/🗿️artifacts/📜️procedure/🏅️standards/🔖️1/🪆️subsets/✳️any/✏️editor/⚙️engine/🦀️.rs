//! ⚙️ Imperative app engine — the app's own stateful host over the artifact's pure `ProcedureSnapshot`.
//! Relocated from the deleted artifact-tree `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): an artifact is a schema + io, never an
//! engine; behaviour belongs to the app that edits it. `ImperativeHost` owns `&mut self` execution
//! state (`registry`, `next_serial`) — the textbook D5 Behavioral case — and `imperative_io()` returns
//! `AppIo`, this app's typed media surface. `default_snapshot()` stayed at `🧬️schema` (pure, no app
//! type in its signature, and still needed by the artifact's own mutation/diff tests).

use crate::{Dictionary, Path, PathRef, ProcedureScene, Registry, Step};
use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, Executor, RunResult};

//#region ⚠️ Errors
/// 🚨️ Imperative core's fallible operations.
#[derive(Debug)]
pub enum ImperativeCoreError {
    Json(semio_framework_value::ValueError),
    MissingOwner,
    MissingSlot,
    UnknownOwnerStep(String),
    UnknownStep(String),
}

impl std::fmt::Display for ImperativeCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
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

impl From<semio_framework_value::ValueError> for ImperativeCoreError {
    fn from(error: semio_framework_value::ValueError) -> Self {
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
        artifact_schema: crate::PROCEDURE_DOCUMENT_SCHEMA.into(),
        artifact_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Procedure },
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
/// 🎛️ Native imperative program host over the composed scene (design §20.15): the program and seed it runs and edits
/// are the scene's own copies, never a document's local owner.
pub struct ImperativeHost {
    path: Path,
    seed: std::collections::BTreeMap<String, neural_engine::Value>,
    registry: Registry,
    next_serial: u64,
}

impl Default for ImperativeHost {
    fn default() -> Self {
        Self::from_scene(&ProcedureScene::default())
    }
}

/// 🧊️ The host is the cold boundary of its seed's neural values (steps retire their own params on drop).
impl Drop for ImperativeHost {
    fn drop(&mut self) {
        neural_engine::ColdRetire::retire_cold(std::mem::take(&mut self.seed));
    }
}

impl ImperativeHost {
    /// 🌱 A host over a copy of the scene's program and seed.
    pub fn from_scene(scene: &ProcedureScene) -> Self {
        crate::standards::v1::subsets::any::io::bootstrap_imperative_runtime();
        Self { path: scene.path.clone(), seed: scene.seed.clone(), registry: imperative_module_registry(), next_serial: 100 }
    }

    /// 🕸️ The program this host holds.
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
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
        path.steps.len() != before
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
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        self.set_step_params_at(&PathRef::default(), id, json)
    }

    pub fn set_step_params_at(&mut self, path_ref: &PathRef, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        let params: Dictionary = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        let path = self.resolve_path_mut(path_ref)?;
        let Some(step) = path.steps.iter_mut().find(|step| step.id == id) else {
            return Err(ImperativeCoreError::UnknownStep(id.into()));
        };
        // 🧊️ Retire the DISPLACED dictionary rather than dropping it in place — see `Step`'s own
        // cold-boundary note in `imperative_engine`.
        neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut step.params, params));
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
