//! 🪪️ Builds identity from verified catalog facts without granting inference execution authority.

use std::sync::Arc;

use super::{
    schema::{InferenceBindingIdentityV1, InferenceIdentityV1, InferenceParentDialectV1, InferenceRequestV1, GIS_GRANTED_MODE, GIS_SERVICE_ID, INPUT_MAX_BYTES},
    sha256, InferenceErrorV1, InferenceOperationControlV1, InferencePrivateBytesV1,
};
use crate::artifact_authority::{
    trusted_catalog::{VerifiedDocumentOpenSelectionV1, VerifiedTrustedCatalog},
    TrustedArtifactCatalog, TrustedArtifactIdentity,
};
use crate::directory::model::AuthSessionRecord;
use directory::os_directory::{descriptor_digest_v1, hex_lower, ArtifactFrontier, DocumentDescriptor, DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1, DocumentScope};
use semio_framework::ContributedInferenceMetadata;
use semio_framework_plugin::ArtifactInferenceService;
use serde::{Deserialize, Serialize};

const GIS_MAP_BINDING_DOMAIN: &[u8] = b"semio.hub.gis-map-frozen-binding/v1\0";
const GIS_MAP_NATIVE_EXECUTABLE: &str = "semio_s_plugin_gis::gis_map_inference_service";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GisMapFrozenPackageV1 {
    plugin_id: String,
    package_id: String,
    version: String,
    component_sha256: String,
    component_blake3: String,
    descriptor_byte_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GisMapFrozenArtifactV1 {
    kind: String,
    schema: String,
    pack_schema_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GisMapFrozenDialectV1 {
    artifact_kind: String,
    standard: String,
    subset: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GisMapFrozenSurfaceV1 {
    surface_id: String,
    app_id: String,
    window_kind_id: String,
    role: String,
    renderer_target: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GisMapFrozenGrantV1 {
    read: bool,
    write: bool,
    observe: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GisMapFrozenServiceV1 {
    owner: String,
    contributor: String,
    artifact_kind: String,
    artifact_schema: String,
    artifact_schema_version: u32,
    document_schema: String,
    document_schema_version: u32,
    inference_schema: String,
    inference_schema_version: u32,
    algorithm_version: u32,
    policy_version: u32,
    depends_on: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GisMapFrozenBindingProjectionV1 {
    catalog_generation_id: String,
    package: GisMapFrozenPackageV1,
    artifact: GisMapFrozenArtifactV1,
    parent_dialect: GisMapFrozenDialectV1,
    surface: GisMapFrozenSurfaceV1,
    grant: GisMapFrozenGrantV1,
    service: GisMapFrozenServiceV1,
    native_executable: String,
}

/// 🧊️ One process-lifetime GIS Map editor selection pinned to its verified catalog and exact native executable.
pub struct VerifiedGisMapArtifactBindingV1 {
    catalog: Arc<VerifiedTrustedCatalog>,
    selection: VerifiedDocumentOpenSelectionV1,
    projection: GisMapFrozenBindingProjectionV1,
    service: ArtifactInferenceService,
    digest: String,
}

impl VerifiedGisMapArtifactBindingV1 {
    /// 🔐️ Returns the canonical digest covering every frozen catalog, surface, grant, service, and executable fact.
    pub fn digest(&self) -> &str {
        &self.digest
    }

    /// 🎯 Returns the retained catalog selection; it never derives authority from public plan bytes.
    pub fn selection(&self) -> &VerifiedDocumentOpenSelectionV1 {
        &self.selection
    }

    /// 💡️ Returns the exact non-capturing service admitted with this binding.
    pub fn service(&self) -> ArtifactInferenceService {
        self.service
    }

    /// 🗂️ Retains the immutable catalog generation for the binding's complete lifetime.
    pub fn catalog(&self) -> &Arc<VerifiedTrustedCatalog> {
        &self.catalog
    }

    /// 🪪️ Projects every frozen executable fact into the immutable identity carried by each job.
    pub fn identity(&self) -> InferenceBindingIdentityV1 {
        InferenceBindingIdentityV1 {
            digest: self.digest.clone(),
            catalog_generation_id: self.projection.catalog_generation_id.clone(),
            package_id: self.projection.package.package_id.clone(),
            package_version: self.projection.package.version.clone(),
            component_sha256: self.projection.package.component_sha256.clone(),
            component_blake3: self.projection.package.component_blake3.clone(),
            artifact_kind: self.projection.artifact.kind.clone(),
            document_schema: self.projection.artifact.schema.clone(),
            parent_dialect: InferenceParentDialectV1 {
                artifact_kind: self.projection.parent_dialect.artifact_kind.clone(),
                standard: self.projection.parent_dialect.standard.clone(),
                subset: self.projection.parent_dialect.subset.clone(),
            },
            surface_id: self.projection.surface.surface_id.clone(),
            granted_mode: GIS_GRANTED_MODE.to_owned(),
            service_id: self.projection.service.inference_schema.clone(),
            service_version: self.projection.service.inference_schema_version,
            algorithm_version: self.projection.service.algorithm_version,
        }
    }
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_gis_map_binding_projection(projection: &GisMapFrozenBindingProjectionV1, native: ArtifactInferenceService) -> Result<(), InferenceErrorV1> {
    let metadata = native.metadata();
    let expected = semio_s_plugin_gis::artifacts::gismap::gis_map_inference_service();
    if !valid_digest(&projection.catalog_generation_id)
        || [&projection.package.component_sha256, &projection.package.component_blake3, &projection.package.descriptor_byte_sha256, &projection.artifact.pack_schema_hash].iter().any(|value| !valid_digest(value))
        || projection.package.plugin_id != "gis"
        || projection.package.package_id != "semio:gis"
        || projection.package.version.is_empty()
        || projection.artifact.kind != "s.gis.gismap"
        || projection.artifact.schema != "gis.map"
        || projection.parent_dialect.artifact_kind != projection.artifact.kind
        || projection.parent_dialect.standard != "1"
        || projection.parent_dialect.subset != "*"
        || projection.surface.surface_id != "s.gis.gismap@1/*#editor"
        || projection.surface.app_id != projection.surface.surface_id
        || projection.surface.window_kind_id != "gis2d-main"
        || projection.surface.role != "editor"
        || !matches!(projection.surface.renderer_target.as_str(), "react" | "wgpu" | "wasm")
        || !projection.grant.read
        || !projection.grant.write
        || !projection.grant.observe
        || projection.service.owner != "gis"
        || projection.service.contributor != "gis"
        || projection.service.artifact_kind != projection.artifact.kind
        || projection.service.artifact_schema != "s.gis.gismap"
        || projection.service.document_schema != projection.artifact.schema
        || projection.service.inference_schema != GIS_SERVICE_ID
        || !projection.service.depends_on.is_empty()
        || [projection.service.artifact_schema_version, projection.service.document_schema_version, projection.service.inference_schema_version, projection.service.algorithm_version, projection.service.policy_version] != [1; 5]
        || metadata.owner != projection.service.owner
        || metadata.artifact_kind != projection.service.artifact_kind
        || metadata.artifact_schema != projection.service.artifact_schema
        || metadata.artifact_schema_version != projection.service.artifact_schema_version
        || metadata.document_schema != projection.service.document_schema
        || metadata.document_schema_version != projection.service.document_schema_version
        || metadata.inference_schema != projection.service.inference_schema
        || metadata.inference_schema_version != projection.service.inference_schema_version
        || metadata.algorithm_version != projection.service.algorithm_version
        || metadata.policy_version != projection.service.policy_version
        || projection.native_executable != GIS_MAP_NATIVE_EXECUTABLE
        || native.executable_identity() != expected.executable_identity()
    {
        return Err(InferenceErrorV1::Denied);
    }
    Ok(())
}

fn gis_map_binding_digest(projection: &GisMapFrozenBindingProjectionV1) -> Result<String, InferenceErrorV1> {
    let mut bytes = GIS_MAP_BINDING_DOMAIN.to_vec();
    bytes.extend(serde_json::to_vec(projection).map_err(|_| InferenceErrorV1::Invalid)?);
    Ok(sha256(&bytes))
}

fn verified_gis_map_binding_with_service(catalog: Arc<VerifiedTrustedCatalog>, native: ArtifactInferenceService) -> Result<Option<Arc<VerifiedGisMapArtifactBindingV1>>, InferenceErrorV1> {
    let selection = catalog.selected_document_open().ok_or(InferenceErrorV1::Denied)?;
    if selection.package.plugin_id != "gis" || selection.package.package_id != "semio:gis" || selection.artifact.kind != "s.gis.gismap" {
        return Ok(None);
    }
    if selection.surface.role != DocumentOpenSurfaceRoleV1::Editor || !selection.grant.write {
        return Ok(None);
    }
    let mut packages = catalog.packages().iter().filter(|package| package.plugin_id() == selection.package.plugin_id && package.package_ref().package.0 == selection.package.package_id);
    let package = packages.next().ok_or(InferenceErrorV1::Denied)?;
    if packages.next().is_some()
        || package.version() != selection.package.version
        || hex_lower(package.component_sha256()) != selection.package.component_sha256
        || hex_lower(&package.package_ref().hash.0) != selection.package.component_blake3
        || hex_lower(package.descriptor_sha256()) != selection.package.descriptor_byte_sha256
        || package.descriptor().package_id != selection.package.package_id
        || package.descriptor().manifest.version != selection.package.version
    {
        return Err(InferenceErrorV1::Denied);
    }
    let mut services = package.descriptor().contributions.inference_services.iter().filter(|service| service.inference_schema == GIS_SERVICE_ID);
    let declared = services.next().ok_or(InferenceErrorV1::Denied)?;
    if services.next().is_some() {
        return Err(InferenceErrorV1::Denied);
    }
    let renderer_target = match selection.surface.renderer_target {
        DocumentOpenRendererTargetV1::React => "react",
        DocumentOpenRendererTargetV1::Wgpu => "wgpu",
        DocumentOpenRendererTargetV1::Wasm => "wasm",
    };
    let projection = GisMapFrozenBindingProjectionV1 {
        catalog_generation_id: catalog.generation_id().to_owned(),
        package: GisMapFrozenPackageV1 {
            plugin_id: selection.package.plugin_id.clone(),
            package_id: selection.package.package_id.clone(),
            version: selection.package.version.clone(),
            component_sha256: selection.package.component_sha256.clone(),
            component_blake3: selection.package.component_blake3.clone(),
            descriptor_byte_sha256: selection.package.descriptor_byte_sha256.clone(),
        },
        artifact: GisMapFrozenArtifactV1 { kind: selection.artifact.kind.clone(), schema: selection.artifact.schema.clone(), pack_schema_hash: selection.artifact.pack_schema_hash.clone() },
        parent_dialect: GisMapFrozenDialectV1 { artifact_kind: selection.parent_dialect.artifact_kind.clone(), standard: selection.parent_dialect.standard.clone(), subset: selection.parent_dialect.subset.clone() },
        surface: GisMapFrozenSurfaceV1 {
            surface_id: selection.surface.surface_id.clone(),
            app_id: selection.surface.app_id.clone(),
            window_kind_id: selection.surface.window_kind_id.clone(),
            role: "editor".to_owned(),
            renderer_target: renderer_target.to_owned(),
        },
        grant: GisMapFrozenGrantV1 { read: selection.grant.read, write: selection.grant.write, observe: selection.grant.observe },
        service: GisMapFrozenServiceV1 {
            owner: declared.owner.clone(),
            contributor: declared.contributor.clone(),
            artifact_kind: declared.artifact_kind.clone(),
            artifact_schema: declared.artifact_schema.clone(),
            artifact_schema_version: declared.artifact_schema_version,
            document_schema: declared.document_schema.clone(),
            document_schema_version: declared.document_schema_version,
            inference_schema: declared.inference_schema.clone(),
            inference_schema_version: declared.inference_schema_version,
            algorithm_version: declared.algorithm_version,
            policy_version: declared.policy_version,
            depends_on: declared.depends_on.clone(),
        },
        native_executable: GIS_MAP_NATIVE_EXECUTABLE.to_owned(),
    };
    validate_gis_map_binding_projection(&projection, native)?;
    let digest = gis_map_binding_digest(&projection)?;
    let selection = selection.clone();
    Ok(Some(Arc::new(VerifiedGisMapArtifactBindingV1 { catalog, selection, projection, service: native, digest })))
}

/// 🧊️ Freezes the verified profile's GIS Map editor choice and literal native inference executable before readiness publication.
pub fn verified_gis_map_binding(catalog: Arc<VerifiedTrustedCatalog>) -> Result<Option<Arc<VerifiedGisMapArtifactBindingV1>>, InferenceErrorV1> {
    verified_gis_map_binding_with_service(catalog, semio_s_plugin_gis::artifacts::gismap::gis_map_inference_service())
}

struct PackageProjection<'a> {
    plugin_id: &'a str,
    package_id: &'a str,
    version: &'a str,
    component_sha256: &'a str,
    services: &'a [ContributedInferenceMetadata],
}

fn exact_projection<'a>(scope: &DocumentScope, descriptor: &DocumentDescriptor, package: &PackageProjection<'a>) -> Result<&'a ContributedInferenceMetadata, InferenceErrorV1> {
    if scope.space_id != descriptor.space_id
        || scope.document_id != descriptor.document_id
        || descriptor.artifact_kind != "s.gis.gismap"
        || descriptor.artifact_schema != "gis.map"
        || descriptor.owner.plugin_id != "gis"
        || descriptor.owner.package_id != "semio:gis"
        || package.plugin_id != descriptor.owner.plugin_id
        || package.package_id != descriptor.owner.package_id
        || package.version != descriptor.owner.version
        || package.component_sha256 != descriptor.owner.package_hash
    {
        return Err(InferenceErrorV1::Denied);
    }
    if package.services.len() > 64 {
        return Err(InferenceErrorV1::Bounds);
    }
    let mut services = package.services.iter().filter(|service| service.inference_schema == GIS_SERVICE_ID);
    let service = services.next().ok_or(InferenceErrorV1::Denied)?;
    if services.next().is_some()
        || service.owner != "gis"
        || service.contributor != "gis"
        || service.artifact_kind != descriptor.artifact_kind
        || service.artifact_schema != "s.gis.gismap"
        || service.document_schema != descriptor.artifact_schema
        || !service.depends_on.is_empty()
        || [service.artifact_schema_version, service.document_schema_version, service.inference_schema_version, service.algorithm_version, service.policy_version] != [1; 5]
    {
        return Err(InferenceErrorV1::Denied);
    }
    Ok(service)
}

pub(crate) struct InferenceIdentitySourceV1<'a> {
    pub request: InferenceRequestV1,
    pub scope: &'a DocumentScope,
    pub descriptor: &'a DocumentDescriptor,
    pub session: &'a AuthSessionRecord,
    pub frontier: &'a ArtifactFrontier,
    pub materialized_input: &'a InferencePrivateBytesV1,
    pub now_ms: i64,
}

/// 🪪️ Derives the whole immutable job identity from the frozen binding; no client bytes contribute.
pub(crate) async fn identity_from_frozen_binding(
    binding: &VerifiedGisMapArtifactBindingV1, source: InferenceIdentitySourceV1<'_>, control: &InferenceOperationControlV1,
) -> Result<InferenceIdentityV1, InferenceErrorV1> {
    let catalog = binding.catalog().as_ref();
    control.checkpoint(0)?;
    source.request.validate()?;
    if source.now_ms < 0
        || source.session.revoked_at.is_some()
        || source.session.expires_at <= source.now_ms
        || source.frontier.document_id != source.scope.document_id
        || source.materialized_input.as_slice().is_empty()
        || source.materialized_input.as_slice().len() > INPUT_MAX_BYTES
    {
        return Err(InferenceErrorV1::Denied);
    }
    let descriptor_digest = descriptor_digest_v1(source.descriptor).map_err(|_| InferenceErrorV1::Invalid)?;
    let mut packages = catalog.packages().iter().filter(|package| package.plugin_id() == source.descriptor.owner.plugin_id);
    let package = packages.next().ok_or(InferenceErrorV1::Denied)?;
    if packages.next().is_some() {
        return Err(InferenceErrorV1::Denied);
    }
    let package_hash = hex_lower(package.component_sha256());
    let projection = PackageProjection { plugin_id: package.plugin_id(), package_id: &package.descriptor().package_id, version: package.version(), component_sha256: &package_hash, services: &package.descriptor().contributions.inference_services };
    exact_projection(source.scope, source.descriptor, &projection)?;
    let frozen = binding.identity();
    if frozen.package_id != source.descriptor.owner.package_id || frozen.package_version != source.descriptor.owner.version || frozen.component_sha256 != package_hash || frozen.artifact_kind != source.descriptor.artifact_kind || frozen.document_schema != source.descriptor.artifact_schema {
        return Err(InferenceErrorV1::Denied);
    }
    control.checkpoint(1)?;
    catalog.resolve(&TrustedArtifactIdentity::from_descriptor(source.descriptor)).await.map_err(|_| InferenceErrorV1::Denied)?;
    control.checkpoint(2)?;
    let identity = InferenceIdentityV1 {
        request: source.request,
        user_id: source.session.user_id.clone(),
        session_id: source.session.id.clone(),
        authorization_generation: source.session.authorization_generation,
        space_id: source.scope.space_id.clone(),
        document_id: source.scope.document_id.clone(),
        descriptor_digest: hex_lower(&descriptor_digest.0),
        binding: frozen,
        head_ordinal: source.frontier.head_edit_ordinal,
        head_edit_id: source.frontier.head_edit_id.clone(),
        last_commit_seq: source.frontier.last_commit_seq,
        chain_hash: hex_lower(&source.frontier.chain_hash.0),
        input_hash: sha256(source.materialized_input.as_slice()),
    };
    identity.validate()?;
    control.checkpoint(3)?;
    Ok(identity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gis_map_verified_binding_freezes_catalog_selection_and_native_executable() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🗺️gis-map-frozen-binding-v1/🔣️.json")).unwrap();
        let projection: GisMapFrozenBindingProjectionV1 = serde_json::from_value(fixture["binding"].clone()).unwrap();
        let native = semio_s_plugin_gis::artifacts::gismap::gis_map_inference_service();
        assert_eq!(validate_gis_map_binding_projection(&projection, native), Ok(()));
        assert_eq!(gis_map_binding_digest(&projection).unwrap(), fixture["expectedDigest"]);
        for hostile in fixture["hostile"].as_array().unwrap() {
            let mut candidate = fixture["binding"].clone();
            let path = hostile["path"].as_array().unwrap();
            let mut at = &mut candidate;
            for segment in &path[..path.len() - 1] {
                at = &mut at[segment.as_str().unwrap()];
            }
            at[path.last().unwrap().as_str().unwrap()] = hostile["value"].clone();
            let admitted = serde_json::from_value::<GisMapFrozenBindingProjectionV1>(candidate)
                .ok()
                .is_some_and(|candidate| validate_gis_map_binding_projection(&candidate, native).is_ok() && gis_map_binding_digest(&candidate).ok().as_deref() == fixture["expectedDigest"].as_str());
            assert_eq!(admitted, hostile["accepted"], "{}", hostile["name"]);
        }

        fn reject(_request: &semio_framework_plugin::ArtifactInferenceExecutionRequest<'_>) -> Result<semio_framework_plugin::ArtifactInferenceExecution, semio_framework_plugin::ArtifactInferenceExecutionError> {
            Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("test.reject", "wrong executable"))
        }
        let substituted = ArtifactInferenceService::new(native.metadata(), reject);
        assert_eq!(validate_gis_map_binding_projection(&projection, substituted), Err(InferenceErrorV1::Denied));
    }

    #[test]
    fn inference_catalog_projection_requires_exact_scope_package_and_declared_service() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🪪️inference-catalog-selection-v1/🔣️.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let mut row = fixture.clone();
            let path = case["path"].as_array().unwrap();
            if !path.is_empty() {
                let mut at = &mut row;
                for segment in &path[..path.len() - 1] {
                    at = if let Some(index) = segment.as_u64() { &mut at[index as usize] } else { &mut at[segment.as_str().unwrap()] };
                }
                at[path.last().unwrap().as_str().unwrap()] = case["value"].clone();
            }
            let scope = DocumentScope::new(row["scope"]["spaceId"].as_str().unwrap(), row["scope"]["documentId"].as_str().unwrap());
            let descriptor: DocumentDescriptor = directory::os_pack::json::from_json_str(&row["descriptor"].to_string()).unwrap();
            let services: Vec<ContributedInferenceMetadata> = serde_json::from_value(row["services"].clone()).unwrap();
            let package = &row["package"];
            let projection = PackageProjection {
                plugin_id: package["pluginId"].as_str().unwrap(),
                package_id: package["packageId"].as_str().unwrap(),
                version: package["version"].as_str().unwrap(),
                component_sha256: package["componentSha256"].as_str().unwrap(),
                services: &services,
            };
            assert_eq!(exact_projection(&scope, &descriptor, &projection).is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
        }
    }
}
