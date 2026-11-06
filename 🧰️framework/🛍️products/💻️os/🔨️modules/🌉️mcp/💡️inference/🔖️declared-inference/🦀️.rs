//! 🔖️ Declared inference: the wire-shaped declared service record, descriptor and installed contribution lookups and the workspace and artifact rosters. See [`super`].

use super::*;

//#region 🔖️DeclaredInference
/// 💡️ One declared inference service, wire-shaped 1:1 from `semio_framework::ContributedInferenceMetadata`
/// — the exact static fields a plugin's own `🔣️.json` carries, never a live guest call.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DeclaredInference {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub contributor: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// 📜️ The inference's own published payload contract, straight off the committed descriptor —
    /// what a client must SEND and what it gets back. `inference_list` and `capabilities_describe`
    /// carry it, so "what do I put in `payload`?" is answerable without reading plugin source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<semio_framework::InferencePayloadContract>,
}

impl DeclaredInference {
    /// 🧭️ The plugin whose guest actually answers this row — the `contributor` (which equals
    /// `owner` for an owner-authored inference, and names the contributing plugin for a row
    /// contributed onto a dependency's artifact kind, exactly the id
    /// `ArtifactInferenceRouter::register_plugin` is called with). Falls back to `owner` only for a
    /// descriptor old enough to have shipped without the field at all.
    pub fn route_plugin_id(&self) -> &str {
        if self.contributor.is_empty() {
            &self.owner
        } else {
            &self.contributor
        }
    }
}

impl From<&semio_framework::ContributedInferenceMetadata> for DeclaredInference {
    fn from(metadata: &semio_framework::ContributedInferenceMetadata) -> Self {
        Self {
            owner: metadata.owner.clone(),
            artifact_kind: metadata.artifact_kind.clone(),
            artifact_schema: metadata.artifact_schema.clone(),
            artifact_schema_version: metadata.artifact_schema_version,
            inference_schema: metadata.inference_schema.clone(),
            inference_schema_version: metadata.inference_schema_version,
            algorithm_version: metadata.algorithm_version,
            policy_version: metadata.policy_version,
            contributor: metadata.contributor.clone(),
            depends_on: metadata.depends_on.clone(),
            payload: metadata.payload.clone(),
        }
    }
}

/// 💡️ `descriptor.contributions.inference_services` (owner-authored) plus every
/// `artifact_contributions[].inferences` entry (contributed onto a dependency's kind) — the same
/// chain `🏃️run/🦀️.rs`'s `register_plugin` builds for `ArtifactInferenceRouter`.
fn declared_inferences_from_descriptor(descriptor: &semio_framework::PackageDescriptor) -> Vec<DeclaredInference> {
    descriptor.contributions.inference_services.iter().chain(descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter())).map(DeclaredInference::from).collect()
}

/// 🗂️ Every artifact kind the packages in `descriptors` OWN — each app's own dialect kind, plus
/// every plugin-level `artifact_kinds` row (a library package with zero apps declares its kinds
/// there and nowhere else). This is the set an extension's contribution has to land on to be
/// reachable from this workspace at all.
fn owned_artifact_kinds(descriptors: &[semio_framework::PackageDescriptor]) -> std::collections::BTreeSet<String> {
    descriptors.iter().flat_map(|descriptor| descriptor.manifest.apps.iter().map(|app| app.dialect.artifact_kind.clone()).chain(descriptor.manifest.artifact_kinds.iter().map(|kind| kind.id.clone()))).collect()
}

/// 🧩️ The installed packages that CONTRIBUTE an inference onto an artifact kind `primary` already [Design notes](README.md#source-notes-79).
fn contributed_inference_descriptors(primary: &[semio_framework::PackageDescriptor], installed: Vec<semio_framework::PackageDescriptor>) -> Vec<semio_framework::PackageDescriptor> {
    let owned = owned_artifact_kinds(primary);
    let present: std::collections::BTreeSet<&str> = primary.iter().map(|descriptor| descriptor.manifest.plugin_id.as_str()).collect();
    let mut extra: Vec<semio_framework::PackageDescriptor> = installed
        .into_iter()
        .filter(|descriptor| {
            !present.contains(descriptor.manifest.plugin_id.as_str()) && descriptor.contributions.artifact_contributions.iter().any(|contribution| !contribution.inferences.is_empty() && owned.contains(&contribution.artifact_kind))
        })
        .collect();
    extra.sort_by(|left, right| left.manifest.plugin_id.cmp(&right.manifest.plugin_id));
    extra
}

/// 🧩️ Folder mode's installed package set, for [`contributed_inference_descriptors`] — the same
/// `registry::discover_descriptors` scan `build_catalog` already runs, over the SAME generated
/// registry, which lists extension packages alongside plugin ones. A hub workspace never reaches
/// this: its `discovery_descriptors` is the authenticated catalog's whole selection set, extension
/// packages included, so the hub arm is already complete and must never consult a local registry.
fn installed_descriptors_for_contributions(workspace: &HeadlessWorkspace) -> Result<Vec<semio_framework::PackageDescriptor>, GatewayError> {
    #[cfg(test)]
    if let Some(descriptors)=&workspace.test_discovery {return Ok(descriptors.clone());}
    match workspace.origin() {
        WorkspaceOrigin::Hub { .. } => Ok(Vec::new()),
        WorkspaceOrigin::Folder { .. } => discover_descriptors(&find_repo_root()?),
    }
}

/// 💡️ Real, static, plugin-agnostic discovery: the UNION of every registered plugin's own declared [Design notes](README.md#source-notes-123).
pub fn declared_inferences_for_workspace(workspace: &HeadlessWorkspace) -> Result<Vec<DeclaredInference>, GatewayError> {
    let descriptors = workspace.discovery_descriptors()?;
    if descriptors.is_empty() {
        return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "no plugin-owned capability is registered in this workspace's catalog — nothing to read a declared inference roster from").retryable());
    }
    let contributed = contributed_inference_descriptors(&descriptors, installed_descriptors_for_contributions(workspace)?);
    let mut roster = Vec::new();
    for descriptor in descriptors.iter().chain(contributed.iter()) {
        roster.extend(declared_inferences_from_descriptor(descriptor));
    }
    Ok(roster)
}

/// 💡️ Resolves `artifact_id`'s real schema by delegating to the SAME `semio://artifact/{id}/schema` [Design notes](README.md#source-notes-148).
fn resolve_artifact_schema(workspace: &HeadlessWorkspace, artifact_id: &str) -> Result<String, GatewayError> {
    let contents = workspace.read_resource(&format!("semio://artifact/{artifact_id}/schema"))?;
    let body: serde_json::Value = contents.first().and_then(|content| content.text.as_deref()).and_then(|text| serde_json::from_str(text).ok()).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("`{artifact_id}` schema resource returned no decodable body")))?;
    body.get("schema").and_then(serde_json::Value::as_str).map(str::to_string).ok_or_else(|| GatewayError::new(GatewayErrorCode::Internal, format!("`{artifact_id}` schema resource body carries no `schema` field")))
}

/// 💡️ Discovery for one artifact id: `PROBE_SCHEMA` (this crate's own synthetic probe schema, real [Design notes](README.md#source-notes-161).
pub fn declared_inferences_for_artifact(workspace: &HeadlessWorkspace, artifact_id: &str) -> Result<(String, Vec<DeclaredInference>), GatewayError> {
    if resolve_artifact_schema(workspace, artifact_id)? == PROBE_SCHEMA {
        return Ok((PROBE_SCHEMA.to_string(), Vec::new()));
    }
    let artifact_kind = document_artifact_kind(workspace, artifact_id)?;
    let matches = declared_inferences_for_workspace(workspace)?.into_iter().filter(|item| item.artifact_kind == artifact_kind).collect();
    Ok((artifact_kind, matches))
}
//#endregion 🔖️DeclaredInference
