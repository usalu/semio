//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
    use crate::standards::v1::subsets::mesh::schema::SemioMeshAnalyzer;
    #[cfg(feature = "conversion-mesh")]
    use semio_framework_plugin::{deserializer_entry_of, register_composer_entries, serializer_entry_of, ComposerEntry};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    //#region 🔖️IoBridgeImports
    // 🌉️ W4 (mesh↔{gltf,stl,obj,ply,las}) io leaves — real trait impls registered below.
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::export::serializers::artifacts::las::v1_0::any::SemioMeshToLas;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::export::serializers::artifacts::obj::v3_0::any::SemioMeshToObj;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::export::serializers::artifacts::ply::v1_0::any::SemioMeshToPly;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::export::serializers::artifacts::png::v1_2::any::SemioMeshToPng;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly;
    #[cfg(feature = "conversion-mesh")]
    use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl;
    //#endregion 🔖️IoBridgeImports

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("mesh") };

    //#region 🔖️Composer
    pub struct SemioMeshComposerComposition;

    impl ArtifactComposition for SemioMeshComposerComposition {
        type Snapshot = SemioMeshSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SemioMeshComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioMeshAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioMeshComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
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
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioMeshSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioMeshSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_mesh_referential_invariants(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_mesh.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioMeshValidator: payload did not decode as a SemioMeshSnapshot".to_string())],
            }
        }
    }

    /// 🔗 Real cross-collection referential check, shared by the registered validator above and its
    /// own direct unit tests below.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_mesh_referential_invariants(snapshot: &SemioMeshSnapshot) -> Vec<dsl::Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut seen_mesh_ids = std::collections::HashSet::new();
        for mesh in &snapshot.meshes {
            if !seen_mesh_ids.insert(mesh.id.as_str()) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_mesh.duplicate-mesh-id", dsl::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate mesh id {:?}", mesh.id)));
            }
            let mut seen_primitive_ids = std::collections::HashSet::new();
            for primitive in &mesh.primitives {
                if !seen_primitive_ids.insert(primitive.id.as_str()) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_mesh.duplicate-primitive-id", dsl::TextSpan::at(1, 1), format!("SemioMeshValidator: mesh {:?} has duplicate primitive id {:?}", mesh.id, primitive.id)));
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
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_mesh.duplicate-material-id", dsl::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate material id {:?}", material.id)));
            }
        }

        let mut seen_texture_ids = std::collections::HashSet::new();
        for texture in &snapshot.textures {
            if !seen_texture_ids.insert(texture.id.as_str()) {
                diagnostics.push(dsl::Diagnostic::error("stdio.semio_mesh.duplicate-texture-id", dsl::TextSpan::at(1, 1), format!("SemioMeshValidator: duplicate texture id {:?}", texture.id)));
            }
        }

        diagnostics
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioMeshValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, and SubsetValidator. Called from
    /// this artifact's standard-level `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::mesh::schema::semio_mesh_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioMeshSnapshot, crate::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation>(crate::standards::v1::subsets::mesh::schema::snapshot::STDIO_SEMIOMESH_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-mesh")]
        register_composer_entries(io_bridge_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.mesh.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::mesh::schema::inferences::semio_mesh_artifact_inference_descriptor());
    }

    //#region 🔖️IoBridgeEntries
    /// 🌉️ The 5 (format) x 2 (direction) real semio↔format bridges (gltf/stl/obj/ply/las). Each
    /// `deserializer_entry_of`/`serializer_entry_of` row is single-read (`reads: &[FROM]`) and,
    /// via `register_composer_entries`'s own bidirectional insert (one entry -> BOTH
    /// "mesh imports from format" and "format exports to mesh" IoKeys, see that fn's doc comment),
    /// the 10 rows below give all 20 IoKeys (5 formats x 2 directions x 2 perspectives) without
    /// hand-writing each perspective separately.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-mesh")]
    fn io_bridge_entries() -> &'static [ComposerEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    deserializer_entry_of::<SemioMeshFromGltf>(),
                    serializer_entry_of::<SemioMeshToGltf>(),
                    deserializer_entry_of::<SemioMeshFromStl>(),
                    serializer_entry_of::<SemioMeshToStl>(),
                    serializer_entry_of::<SemioMeshToPng>(),
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
    #[cfg(all(test, feature = "conversion-mesh"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧾️Encoding
/// 🧾️ A standard file format a mesh is written as through this subset's own export leaves.
#[cfg(feature = "conversion-mesh")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemioMeshFormat {
    Stl,
    Obj,
    Ply,
    Gltf,
    Las,
    Dwg,
    Png,
}

/// 🧾️ The file bytes of `mesh` in `format` — the one call every domain artifact that projects into a
/// mesh makes, so each standard format has exactly one writer in the repository.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "conversion-mesh")]
pub fn encode_mesh(mesh: &crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot, format: SemioMeshFormat) -> Result<Vec<u8>, String> {
    use crate::standards::v1::subsets::mesh::io::export::serializers::artifacts::{dwg::v_ac1024::any::SemioMeshToDwg, gltf::v2_0::any::SemioMeshToGltf, las::v1_0::any::SemioMeshToLas, obj::v3_0::any::SemioMeshToObj, ply::v1_0::any::SemioMeshToPly, png::v1_2::any::SemioMeshToPng, stl::v_ascii::any::SemioMeshToStl};
    use semio_framework_plugin::{resolve_ready, ArtifactSerializer};
    match format {
        SemioMeshFormat::Stl => Ok(semio_s_artifact_stdio_stl::engine::encode_stl_ascii(&resolve_ready(SemioMeshToStl::serialize(mesh)).map_err(|e| e.to_string())?).into_bytes()),
        SemioMeshFormat::Obj => Ok(semio_s_artifact_stdio_obj::engine::encode_obj(&resolve_ready(SemioMeshToObj::serialize(mesh)).map_err(|e| e.to_string())?).into_bytes()),
        SemioMeshFormat::Ply => semio_s_artifact_stdio_ply::engine::encode_ply(&resolve_ready(SemioMeshToPly::serialize(mesh)).map_err(|e| e.to_string())?),
        SemioMeshFormat::Gltf => Ok(semio_s_artifact_stdio_gltf::engine::serialize_gltf_document(&resolve_ready(SemioMeshToGltf::serialize(mesh)).map_err(|e| e.to_string())?)),
        SemioMeshFormat::Las => semio_s_artifact_stdio_las::engine::encode_las(&resolve_ready(SemioMeshToLas::serialize(mesh)).map_err(|e| e.to_string())?),
        SemioMeshFormat::Dwg => semio_s_artifact_stdio_dwg::engine::dwg_to_bytes(&resolve_ready(SemioMeshToDwg::serialize(mesh)).map_err(|e| e.to_string())?.drawing.to_native()?),
        SemioMeshFormat::Png => semio_s_artifact_stdio_png::io::encode_png(&resolve_ready(SemioMeshToPng::serialize(mesh)).map_err(|e| e.to_string())?),
    }
}

/// 📥️ `bytes` in `format` read into a mesh through this subset's own import leaves (png has none: a
/// picture of a mesh carries no geometry).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(feature = "conversion-mesh")]
pub fn decode_mesh(bytes: &[u8], format: SemioMeshFormat) -> Result<crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot, String> {
    use crate::standards::v1::subsets::mesh::io::import::deserializers::artifacts::{dwg::v_ac1024::any::SemioMeshFromDwg, gltf::v2_0::any::SemioMeshFromGltf, las::v1_0::any::SemioMeshFromLas, obj::v3_0::any::SemioMeshFromObj, ply::v1_0::any::SemioMeshFromPly, stl::v_ascii::any::SemioMeshFromStl};
    use semio_framework_plugin::{resolve_ready, ArtifactDeserializer};
    let text = || std::str::from_utf8(bytes).map_err(|e| e.to_string());
    match format {
        SemioMeshFormat::Stl => resolve_ready(SemioMeshFromStl::deserialize(&semio_s_artifact_stdio_stl::io::decode_stl_auto(bytes)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Obj => resolve_ready(SemioMeshFromObj::deserialize(&semio_s_artifact_stdio_obj::io::decode_obj(text()?)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Ply => resolve_ready(SemioMeshFromPly::deserialize(&semio_s_artifact_stdio_ply::io::decode_ply(bytes)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Gltf => {
            let gltf = if bytes.starts_with(b"glTF") { semio_s_artifact_stdio_gltf::io::decode_glb(bytes)? } else { semio_s_artifact_stdio_gltf::io::parse_gltf_document(bytes)? };
            resolve_ready(SemioMeshFromGltf::deserialize(&gltf)).map_err(|e| e.to_string())
        }
        SemioMeshFormat::Las => resolve_ready(SemioMeshFromLas::deserialize(&semio_s_artifact_stdio_las::io::decode_las(bytes)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Dwg => resolve_ready(SemioMeshFromDwg::deserialize(&semio_s_artifact_stdio_dwg::schema::snapshot::decode_dwg(bytes)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Png => Err("semio/mesh←png: a picture of a mesh carries no geometry".into()),
    }
}
//#endregion 🧾️Encoding
