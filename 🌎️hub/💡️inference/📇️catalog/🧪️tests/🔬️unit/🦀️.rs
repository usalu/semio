
use super::*;

#[test]
fn gis_map_verified_binding_freezes_catalog_selection_and_native_executable() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json")).unwrap();
    let projection: GisMapFrozenBindingV1 = serde_json::from_value(fixture["binding"].clone()).unwrap();
    let native = semio_s_artifact_gis_gismap::gis_map_inference_service();
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
        let admitted = serde_json::from_value::<GisMapFrozenBindingV1>(candidate)
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️fixtures/🎯️inference-catalog-selection-v1/🔣️.json")).unwrap();
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
