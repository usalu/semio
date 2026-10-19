//! 🧾️ Run context: the run sink that keeps a run's mutated documents and configs apart from its source.
//!
//! Holds the `🔖️RunContext` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️RunContext
/// ✍️ Write-only sink for everything one `SpaceRunner::run` call produces — the ONLY thing allowed to
/// persist bytes for a node's post-import document/config state or accumulate the run's own
/// `semio_framework_artifact_workflow_run::RunArtifact`. Never reads the source `documents`/`configs` maps `run()` takes, and
/// `run()` never writes through anything OTHER than this — the structural half of "non-destructive":
/// `node_artifacts`/`node_configs` are keyed by node id, a distinct key space from the source
/// `artifact_ref`/`config_ref` strings, so a caller (e.g. `bin.rs`) cannot alias a run-owned write
/// path onto a source artifact path by construction.
///
/// 🔒️ `record` is the ONLY way this crate ever mutates a `semio_framework_artifact_workflow_run::RunArtifact` — it always goes
/// through `semio_framework_artifact_workflow_run::apply_run_operation_checked` (never the raw `semio_framework_artifact_workflow_run::apply_run_operation`),
/// so an operation emitted after `Seal` is refused here with its outcome messages (`RunError::MutationRefused`), not silently
/// applied. `SpaceRunner::run` calls `record` for every `NodeStarted`/`NodeFinished`; callers own
/// `Start` (before `run`) and `Seal` (after), since those two carry run-identity/collection-ref fields
/// `SpaceRunner` itself has no business knowing about.
pub struct RunSink {
    pub document: semio_framework_artifact_workflow_run::RunArtifact,
    /// 🧾️ Every operation `record` has successfully applied, in order — `bin.rs` replays this exact
    /// sequence through a real `store::ArtifactStore` to produce persistable pack+spr bytes for
    /// `SpaceBundle::write_run_document` (the same "build an envelope, `Apply`, `snapshot_pack`"
    /// pattern every other document in this codebase persists through).
    pub mutations: Vec<RunMutation>,
    pub node_artifacts: BTreeMap<String, (Vec<u8>, Vec<u8>)>,
    pub node_configs: BTreeMap<String, (Vec<u8>, Vec<u8>)>,
}

impl RunSink {
    pub fn new(document: semio_framework_artifact_workflow_run::RunArtifact) -> Self {
        Self { document, mutations: Vec::new(), node_artifacts: BTreeMap::new(), node_configs: BTreeMap::new() }
    }

    pub async fn record(&mut self, operation: RunMutation) -> Result<(), RunError> {
        self.document = semio_framework_artifact_workflow_run_admission::apply_run_operation_checked(&self.document, operation.clone()).await.map_err(RunError::MutationRefused)?;
        self.mutations.push(operation);
        Ok(())
    }

    pub fn write_node_artifact(&mut self, node_id: &str, pack: Vec<u8>, spr: Vec<u8>) {
        self.node_artifacts.insert(node_id.to_string(), (pack, spr));
    }

    pub fn write_node_config(&mut self, node_id: &str, pack: Vec<u8>, spr: Vec<u8>) {
        self.node_configs.insert(node_id.to_string(), (pack, spr));
    }
}

/// 🎛️ Deterministic bytes for one node's bound `RunParameterValue` overlay (sorted by `field_path`) —
/// folded into that node's `config_fingerprint` so `--param` overrides correctly dirty/cache-key a
/// node even though the underlying config pack+spr bytes sent to the app are untouched (this crate is
/// generic over arbitrary apps' config schemas — it has no per-app `ConfigSpec` to safely patch an
/// opaque config pack's fields by `field_path` itself; that requires `semio_framework::ConfigSpec`,
/// which lives one layer up, at the plugin-manifest layer this crate doesn't depend on). Delivering an
/// override into the actual bytes the app receives is deferred — see this wave's final report.
fn node_parameter_overlay_bytes(node_id: &str, bindings: &[WorkflowParameterBinding], parameter_values: &[RunParameterValue]) -> Vec<u8> {
    let mut pairs: Vec<(&str, &str)> =
        bindings.iter().filter(|binding| binding.node_id == node_id).filter_map(|binding| parameter_values.iter().find(|value| value.parameter_id == binding.parameter_id).map(|value| (binding.field_path.as_str(), value.value.as_str()))).collect();
    pairs.sort_unstable();
    let mut bytes = Vec::new();
    for (field_path, value) in pairs {
        bytes.extend_from_slice(field_path.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(value.as_bytes());
        bytes.push(0);
    }
    bytes
}

/// 🔑️ `document`/`config` fingerprints for one node — `config_fingerprint` folds in the node's bound
/// parameter overlay (see `node_parameter_overlay_bytes`) so a `--param` override changes the
/// fingerprint even when the underlying config bytes don't.
pub(super) fn node_fingerprints(node_id: &str, document_spr: &[u8], config_spr: &[u8], bindings: &[WorkflowParameterBinding], parameter_values: &[RunParameterValue]) -> (String, String) {
    let document_fingerprint = framework_hash::hash_bytes(document_spr);
    let overlay = node_parameter_overlay_bytes(node_id, bindings, parameter_values);
    let config_fingerprint = if overlay.is_empty() { framework_hash::hash_bytes(config_spr) } else { framework_hash::hash_bytes(&[config_spr, &overlay].concat()) };
    (document_fingerprint, config_fingerprint)
}
//#endregion 🔖️RunContext
