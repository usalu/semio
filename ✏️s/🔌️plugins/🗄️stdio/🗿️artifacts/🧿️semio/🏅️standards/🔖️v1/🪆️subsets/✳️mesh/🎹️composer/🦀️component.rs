//! 🎹️ SemioMeshComposer (s.stdio.semio/v1/mesh) — analyzer-only compose (decodes the subset's
//! own JSON-pack payload). W4 adds real cross-format compose sources once semio↔format
//! import/export leaves land (gltf/stl/obj/ply/las).

use semio_framework_plugin::{
    ArtifactComposer, ArtifactAnalyzer as _, AnalyzeSource, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId,
    SubsetValidator, SubsetValidatorEntry, register_subset_validator, subset_validator_entry_of,
    ComposerEntry, deserializer_entry_of, serializer_entry_of, register_composer_entries,
};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::analyzer::SemioMeshAnalyzer;
//#region 🔖️IoBridgeImports
// 🌉️ W4 (mesh↔{gltf,stl,obj,ply,las}) io leaves — real trait impls registered below.
use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::ply::v1_0::any::SemioMeshToPly;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas;
use crate::artifacts::semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::las::v1_0::any::SemioMeshToLas;
//#endregion 🔖️IoBridgeImports

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

//#region 🔖️Composer
pub struct SemioMeshComposer;

impl ArtifactComposer for SemioMeshComposer {
    type Snapshot = SemioMeshSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] { &[DIALECT] }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "SemioMeshComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = SemioMeshAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "SemioMeshComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
//#endregion 🔖️Composer

//#region 🔖️SubsetValidator
/// 🛡️ Decodes the payload as this subset's own `SemioMeshSnapshot`, then checks real
/// referential invariants across its own collections: every `primitive.material_id` (when
/// `Some`) must resolve to a real entry in `materials`, and mesh/primitive/material/texture ids
/// must be unique within their own collection (dangling refs + duplicate keys are the two
/// invariant classes the master plan calls out for subset validators).
pub struct SemioMeshValidator;

impl SubsetValidator for SemioMeshValidator {
    const DIALECT: Dialect = DIALECT;
    fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload {
            IoPayload::Binary(bytes) => <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
            IoPayload::Text(text) => <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
        };
        match decoded {
            Some(snapshot) => check_mesh_referential_invariants(&snapshot),
            None => vec![dsl::Diagnostic::error(
                "stdio.semio_mesh.validate-decode-failed",
                dsl::TextSpan::at(1, 1),
                "SemioMeshValidator: payload did not decode as a SemioMeshSnapshot".to_string(),
            )],
        }
    }
}

/// 🔗 Real cross-collection referential check, shared by the registered validator above and its
/// own direct unit tests below.
pub fn check_mesh_referential_invariants(snapshot: &SemioMeshSnapshot) -> Vec<dsl::Diagnostic> {
    let mut diagnostics = Vec::new();

    let mut seen_mesh_ids = std::collections::HashSet::new();
    for mesh in &snapshot.meshes {
        if !seen_mesh_ids.insert(mesh.id.as_str()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_mesh.duplicate-mesh-id",
                dsl::TextSpan::at(1, 1),
                format!("SemioMeshValidator: duplicate mesh id {:?}", mesh.id),
            ));
        }
        let mut seen_primitive_ids = std::collections::HashSet::new();
        for primitive in &mesh.primitives {
            if !seen_primitive_ids.insert(primitive.id.as_str()) {
                diagnostics.push(dsl::Diagnostic::error(
                    "stdio.semio_mesh.duplicate-primitive-id",
                    dsl::TextSpan::at(1, 1),
                    format!("SemioMeshValidator: mesh {:?} has duplicate primitive id {:?}", mesh.id, primitive.id),
                ));
            }
            if let Some(material_id) = &primitive.material_id {
                if !snapshot.materials.iter().any(|m| &m.id == material_id) {
                    diagnostics.push(dsl::Diagnostic::error(
                        "stdio.semio_mesh.dangling-material-ref",
                        dsl::TextSpan::at(1, 1),
                        format!("SemioMeshValidator: mesh {:?} primitive {:?} references missing material {:?}", mesh.id, primitive.id, material_id),
                    ));
                }
            }
        }
    }

    let mut seen_material_ids = std::collections::HashSet::new();
    for material in &snapshot.materials {
        if !seen_material_ids.insert(material.id.as_str()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_mesh.duplicate-material-id",
                dsl::TextSpan::at(1, 1),
                format!("SemioMeshValidator: duplicate material id {:?}", material.id),
            ));
        }
    }

    let mut seen_texture_ids = std::collections::HashSet::new();
    for texture in &snapshot.textures {
        if !seen_texture_ids.insert(texture.id.as_str()) {
            diagnostics.push(dsl::Diagnostic::error(
                "stdio.semio_mesh.duplicate-texture-id",
                dsl::TextSpan::at(1, 1),
                format!("SemioMeshValidator: duplicate texture id {:?}", texture.id),
            ));
        }
    }

    diagnostics
}

static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
fn validator_entry() -> &'static SubsetValidatorEntry { VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioMeshValidator>) }
//#endregion 🔖️SubsetValidator

//#region 🔖️Register
/// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
/// this artifact's standard-level `engine::register()`.
pub fn register() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::mesh::schema::semio_mesh_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<SemioMeshSnapshot, crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation>(crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::STDIO_SEMIOMESH_DOCUMENT_SCHEMA));
    register_subset_validator(validator_entry());
    register_composer_entries(io_bridge_entries());
}

//#region 🔖️IoBridgeEntries
/// 🌉️ The 5 (format) x 2 (direction) real semio↔format bridges (gltf/stl/obj/ply/las). Each
/// `deserializer_entry_of`/`serializer_entry_of` row is single-read (`reads: &[FROM]`) and,
/// via `register_composer_entries`'s own bidirectional insert (one entry -> BOTH
/// "mesh imports from format" and "format exports to mesh" IoKeys, see that fn's doc comment),
/// the 10 rows below give all 20 IoKeys (5 formats x 2 directions x 2 perspectives) without
/// hand-writing each perspective separately.
fn io_bridge_entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                deserializer_entry_of::<SemioMeshFromGltf>(),
                serializer_entry_of::<SemioMeshToGltf>(),
                deserializer_entry_of::<SemioMeshFromStl>(),
                serializer_entry_of::<SemioMeshToStl>(),
                deserializer_entry_of::<SemioMeshFromObj>(),
                serializer_entry_of::<SemioMeshToObj>(),
                deserializer_entry_of::<SemioMeshFromPly>(),
                serializer_entry_of::<SemioMeshToPly>(),
                deserializer_entry_of::<SemioMeshFromLas>(),
                serializer_entry_of::<SemioMeshToLas>(),
            ]
        })
        .as_slice()
}
//#endregion 🔖️IoBridgeEntries
//#endregion 🔖️Register

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioPrimitive};

    #[test]
    fn clean_snapshot_has_no_diagnostics() {
        let snapshot = SemioMeshSnapshot {
            meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), material_id: Some("mat1".into()), ..Default::default() }] }],
            materials: vec![SemioMaterial { id: "mat1".into(), ..Default::default() }],
            ..Default::default()
        };
        assert!(check_mesh_referential_invariants(&snapshot).is_empty());
    }

    #[test]
    fn dangling_material_ref_is_flagged() {
        let snapshot = SemioMeshSnapshot {
            meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), material_id: Some("missing".into()), ..Default::default() }] }],
            ..Default::default()
        };
        let diagnostics = check_mesh_referential_invariants(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_mesh.dangling-material-ref"), "got {diagnostics:?}");
    }

    #[test]
    fn duplicate_ids_are_flagged_per_collection() {
        let snapshot = SemioMeshSnapshot {
            meshes: vec![SemioMesh { id: "dup".into(), primitives: vec![] }, SemioMesh { id: "dup".into(), primitives: vec![] }],
            materials: vec![SemioMaterial { id: "dup".into(), ..Default::default() }, SemioMaterial { id: "dup".into(), ..Default::default() }],
            textures: vec![],
            ..Default::default()
        };
        let diagnostics = check_mesh_referential_invariants(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_mesh.duplicate-mesh-id"), "got {diagnostics:?}");
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_mesh.duplicate-material-id"), "got {diagnostics:?}");
    }

    #[test]
    fn validator_decodes_and_runs_the_referential_checks_end_to_end() {
        let snapshot = SemioMeshSnapshot {
            meshes: vec![SemioMesh { id: "m1".into(), primitives: vec![SemioPrimitive { id: "p1".into(), material_id: Some("missing".into()), ..Default::default() }] }],
            ..Default::default()
        };
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let diagnostics = SemioMeshValidator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_mesh.dangling-material-ref"), "got {diagnostics:?}");
    }
}
//#endregion 🔖️Tests
