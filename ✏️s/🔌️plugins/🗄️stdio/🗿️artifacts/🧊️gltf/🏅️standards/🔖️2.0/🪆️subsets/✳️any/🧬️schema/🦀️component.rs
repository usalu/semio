//! 🧬️ GltfArtifact schema — full artifact state.

use crate::artifacts::gltf::schema::snapshot::{GltfAccessor, GltfBuffer, GltfBufferView, GltfDocument, GltfJson, GltfMesh, GltfPrimitive, GltfSourceForm};
use crate::artifacts::gltf::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf")]
pub struct GltfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub document: GltfDocument,
    #[state(artifact)]
    #[serde(default)]
    pub buffers: Vec<Vec<u8>>,
    #[state(artifact)]
    #[serde(default)]
    pub source_form: GltfSourceForm,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for GltfArtifact {
    fn default() -> Self {
        Self::from_snapshot(GltfSnapshot::default())
    }
}

impl GltfArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> GltfSnapshot {
        GltfSnapshot { schema: self.schema.clone(), document: self.document.clone(), buffers: self.buffers.clone(), source_form: self.source_form }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: GltfSnapshot) -> Self {
        Self { schema: snapshot.schema, document: snapshot.document, buffers: snapshot.buffers, source_form: snapshot.source_form }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: GltfSnapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
        self.buffers = snapshot.buffers;
        self.source_form = snapshot.source_form;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gltf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.gltf",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("../🔨️modules/🧭️mutation-dispatch/🦀️component.rs"),
            typescript: include_str!("../🔨️modules/🧭️mutation-dispatch/🟦️component.ts"),
            graphql: include_str!("../🔨️modules/🧭️mutation-dispatch/🔗️component.graphql"),
            json_schema: include_str!("../🔨️modules/🧭️mutation-dispatch/🔣️component.json"),
            proto: include_str!("../🔨️modules/🧭️mutation-dispatch/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
    use crate::artifacts::gltf::schema::modules::mutation_dispatch::GltfMutation;
    use crate::artifacts::gltf::schema::snapshot::{GltfAccessor, GltfBuffer, GltfBufferView, GltfJson, GltfMaterial, GltfMesh, GltfNode, GltfPrimitive, GltfScene};
    use crate::artifacts::gltf::{GltfMutationDiff, GltfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.gltf` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct GltfBuilderConstruction {
        snapshot: GltfSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for GltfBuilderConstruction {
        type Snapshot = GltfSnapshot;
        type Mutation = GltfMutation;
        type Diff = GltfMutationDiff;
        async fn empty() -> Self {
            Self { snapshot: GltfSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<GltfSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<GltfSnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = protocol::Mutation::diff(&mutation, &self.snapshot).await;
            if outcome.worst_level().await.is_some_and(|level| level >= dsl::Severity::Error) {
                self.diagnostics.push(dsl::Diagnostic::error("stdio.gltf.mutation-rejected", dsl::TextSpan::at(1, 1), format!("{:?}", outcome.messages().await)));
            }
            match outcome.diff().await.try_apply(&self.snapshot) {
                Ok(snapshot) => {
                    self.snapshot = snapshot;
                }
                Err(error) => {
                    self.diagnostics.push(dsl::Diagnostic::error("stdio.gltf.mutation-rejected", dsl::TextSpan::at(1, 1), error.to_string()));
                }
            }
            (self, outcome)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <GltfMutationDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&diff, &self.snapshot).await?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️AccessorSpec
    /// 📐️ Everything `add_accessor` needs beyond the buffer-view-and-offset plumbing common to every
    /// accessor. Built via `new` + chained `with_*` setters (values, not consuming-`Self` document
    /// mutation -- this is a plain value type, not the builder itself).
    #[derive(Clone, Debug)]
    pub struct GltfAccessorSpec {
        pub buffer_view: Option<usize>,
        pub byte_offset: usize,
        pub component_type: GltfComponentType,
        pub accessor_type: GltfAccessorType,
        pub count: usize,
        pub normalized: bool,
        pub min: Option<Vec<f64>>,
        pub max: Option<Vec<f64>>,
    }

    impl GltfAccessorSpec {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize) -> Self {
            Self { buffer_view: None, byte_offset: 0, component_type, accessor_type, count, normalized: false, min: None, max: None }
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_buffer_view(mut self, buffer_view: usize, byte_offset: usize) -> Self {
            self.buffer_view = Some(buffer_view);
            self.byte_offset = byte_offset;
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_normalized(mut self, normalized: bool) -> Self {
            self.normalized = normalized;
            self
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_min_max(mut self, min: Vec<f64>, max: Vec<f64>) -> Self {
            self.min = Some(min);
            self.max = Some(max);
            self
        }
    }
    //#endregion 🔖️AccessorSpec

    //#region 🔖️DocumentConstructors
    impl GltfBuilderConstruction {
        /// 🌱 Sets `asset.version` (the one glTF-mandatory field).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_asset_version(&mut self, version: &str) -> &mut Self {
            self.snapshot.document.asset.version = version.to_string();
            self
        }

        /// 📦️ Appends a buffer, storing its real bytes on the snapshot's `buffers` (index-aligned with
        /// `document.buffers`) and recording `byteLength` in the document. Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_buffer(&mut self, bytes: Vec<u8>) -> usize {
            let byte_length = bytes.len();
            let idx = self.snapshot.buffers.len();
            self.snapshot.buffers.push(bytes);
            self.snapshot.document.buffers.push(GltfBuffer { byte_length, uri: None, name: None, extensions: None, extras: None });
            idx
        }

        /// 🪟️ Appends a `bufferView` (buffer index, byte offset/length, optional `byteStride` for
        /// interleaved data, optional `target` -- 34962 `ARRAY_BUFFER` / 34963 `ELEMENT_ARRAY_BUFFER`).
        /// Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_buffer_view(&mut self, buffer: usize, byte_offset: usize, byte_length: usize, byte_stride: Option<usize>, target: Option<u64>) -> usize {
            let idx = self.snapshot.document.buffer_views.len();
            self.snapshot.document.buffer_views.push(GltfBufferView { buffer, byte_offset, byte_length, byte_stride, target, name: None, extensions: None, extras: None });
            idx
        }

        /// 🔢️ Appends an `accessor` from a typed [`GltfAccessorSpec`]. Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_accessor(&mut self, spec: GltfAccessorSpec) -> usize {
            let idx = self.snapshot.document.accessors.len();
            self.snapshot.document.accessors.push(GltfAccessor {
                buffer_view: spec.buffer_view,
                byte_offset: spec.byte_offset,
                component_type: spec.component_type,
                normalized: spec.normalized,
                count: spec.count,
                kind: spec.accessor_type,
                max: spec.max,
                min: spec.min,
                sparse: None,
                name: None,
                extensions: None,
                extras: None,
            });
            idx
        }

        /// 🎨️ Appends a fully typed `material`. Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_material(&mut self, material: GltfMaterial) -> usize {
            let idx = self.snapshot.document.materials.len();
            self.snapshot.document.materials.push(material);
            idx
        }

        /// 🕸️ Appends an empty `mesh` (primitives added via [`Self::add_mesh_primitive`]). Returns the
        /// new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_mesh(&mut self) -> usize {
            let idx = self.snapshot.document.meshes.len();
            self.snapshot.document.meshes.push(GltfMesh::default());
            idx
        }

        /// 🔺️ Appends a primitive to `meshes[mesh]` -- `attributes` are `(semantic, accessor index)`
        /// pairs (e.g. `("POSITION", 0)`), `indices`/`material` are optional accessor/material
        /// indices, `mode` is the primitive topology (defaults to `4` TRIANGLES per spec when unset).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_mesh_primitive(&mut self, mesh: usize, attributes: &[(&str, usize)], indices: Option<usize>, material: Option<usize>, mode: Option<u64>) {
            let primitive = GltfPrimitive { attributes: attributes.iter().map(|(name, idx)| ((*name).to_string(), *idx)).collect(), indices, material, mode, targets: Vec::new(), extensions: None, extras: None };
            let mesh_entry = self.snapshot.document.meshes.get_mut(mesh).expect("mesh index out of range -- call add_mesh first");
            mesh_entry.primitives.push(primitive);
        }

        /// 🧍️ Appends a `node`, optionally referencing a mesh. Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_node(&mut self, mesh: Option<usize>) -> usize {
            let idx = self.snapshot.document.nodes.len();
            self.snapshot.document.nodes.push(GltfNode { mesh, ..GltfNode::default() });
            idx
        }

        /// 🎬️ Appends a `scene` referencing `nodes` (root node indices), with an optional passthrough
        /// `extensions` object (real documents sometimes carry a declared-but-empty `{}` here).
        /// Returns the new index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_scene(&mut self, nodes: Vec<usize>, extensions: Option<GltfJson>) -> usize {
            let idx = self.snapshot.document.scenes.len();
            self.snapshot.document.scenes.push(GltfScene { nodes, name: None, extensions, extras: None });
            idx
        }

        /// 🎬️ Sets the document's default `scene` index.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_default_scene(&mut self, scene: usize) -> &mut Self {
            self.snapshot.document.scene = Some(scene);
            self
        }

        /// 🧩️ Sets `extensionsUsed` (declared, not necessarily applied -- mirrors real-world documents
        /// that declare an extension namespace without every element using it).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_extensions_used(&mut self, names: Vec<String>) -> &mut Self {
            self.snapshot.document.extensions_used = names;
            self
        }

        /// 📸️ Peeks the in-progress document -- used by tests/callers that need to inspect state
        /// mid-construction without consuming the builder via `build()`.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn document(&self) -> &crate::artifacts::gltf::schema::snapshot::GltfDocument {
            &self.snapshot.document
        }

        /// 📦️ Peeks the in-progress resolved buffer bytes.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn buffers(&self) -> &[Vec<u8>] {
            &self.snapshot.buffers
        }
    }
    //#endregion 🔖️DocumentConstructors

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn typed_constructors_build_a_decodable_triangle() {
            let mut b = GltfBuilderConstruction::empty();
            b.await.set_asset_version("2.0");
            let mut bytes = Vec::new();
            let verts: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
            for v in verts {
                for c in v {
                    bytes.extend_from_slice(&c.to_le_bytes());
                }
            }
            let buf = b.await.add_buffer(bytes);
            let bv = b.await.add_buffer_view(buf, 0, 36, None, Some(34962));
            let acc = b.await.add_accessor(GltfAccessorSpec::new(GltfComponentType::Float, GltfAccessorType::Vec3, 3).with_buffer_view(bv, 0).with_min_max(vec![0.0, 0.0, 0.0], vec![1.0, 1.0, 0.0]));
            let mat = b.await.add_material(GltfMaterial { pbr_metallic_roughness: Some(crate::artifacts::gltf::schema::snapshot::GltfPbrMetallicRoughness { base_color_factor: [1.0, 0.0, 0.0, 1.0], ..Default::default() }), ..Default::default() });
            let mesh = b.await.add_mesh();
            b.await.add_mesh_primitive(mesh, &[("POSITION", acc)], None, Some(mat), None);
            let node = b.await.add_node(Some(mesh));
            let scene = b.await.add_scene(vec![node], None);
            b.await.set_default_scene(scene);
            let snapshot = b.build().expect("build");

            assert_eq!(snapshot.document.asset.version, "2.0");
            let decoded = crate::artifacts::gltf::engine::decode_accessor(&snapshot.document, &snapshot.buffers, acc).expect("decode");
            assert_eq!(decoded.components, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::gltf::GltfSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.gltf` parts.
    #[derive(Clone, Debug, Default)]
    pub struct GltfParts {
        pub snapshot: Option<GltfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Sniff
    /// 👃️ `.glb` binary container magic: `glTF` + little-endian version `2`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_glb(bytes: &[u8]) -> bool {
        bytes.len() >= 12 && &bytes[0..4] == b"glTF" && u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) == 2
    }

    /// 👃️ `.gltf` JSON text: a JSON object whose top-level `asset` object carries a `version` string
    /// -- the one field glTF 2.0 §3.9 makes universally mandatory, so this is a real (if cheap) probe
    /// rather than a content-blind guess.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_gltf_json(text: &str) -> bool {
        let trimmed = text.trim_start();
        if !trimmed.starts_with('{') {
            return false;
        }
        match serde_json::from_str::<serde_json::Value>(trimmed) {
            Ok(value) => value.get("asset").and_then(|a| a.get("version")).and_then(|v| v.as_str()).is_some(),
            Err(_) => false,
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.gltf` (2.0/✳️any) sources.
    pub struct GltfAnalyzerAnalysis;

    impl ArtifactAnalysis for GltfAnalyzerAnalysis {
        type Parts = GltfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if looks_like_glb(bytes) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if looks_like_gltf_json(text) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Medium
                    }
                }
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = GltfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => {
                        // A genuine `.gltf` JSON document parses directly through the real codec; only
                        // fall back to the SemioEnvelope-wrapped `ArtifactDsl` preamble form (used by
                        // this crate's own internal store round-trips) when the text isn't bare JSON.
                        let result = if looks_like_gltf_json(text) { crate::artifacts::gltf::engine::parse_gltf_document(text.trim().as_bytes()) } else { <GltfSnapshot as store::ArtifactDsl>::parse_dsl(text).await.map_err(|e| e.to_string()) };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = IoConfidence::Low;
                                diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                    AnalyzeSource::Binary(bytes) => {
                        // A genuine raw `.glb` container decodes directly through the real codec; only
                        // fall back to the SemioEnvelope-wrapped `ArtifactPack` form (this crate's own
                        // internal store round-trip encoding) when the bytes aren't a `.glb` container.
                        let result = if looks_like_glb(bytes) { crate::artifacts::gltf::engine::decode_glb(bytes) } else { <GltfSnapshot as store::ArtifactPack>::decode_pack(bytes).await.map_err(|e| e.to_string()) };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = IoConfidence::Low;
                                diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn sniff_recognizes_glb_magic() {
            let mut bytes = vec![b'g', b'l', b'T', b'F'];
            bytes.extend_from_slice(&2u32.to_le_bytes());
            bytes.extend_from_slice(&[0u8; 4]);
            assert_eq!(GltfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
            assert_eq!(GltfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(b"not a glb")), IoConfidence::Low);
        }

        #[semio_framework_async_macros::async_test]
        async fn sniff_recognizes_gltf_json() {
            assert_eq!(GltfAnalyzerAnalysis::sniff(&AnalyzeSource::Text(r#"{"asset":{"version":"2.0"}}"#)), IoConfidence::High);
            assert_eq!(GltfAnalyzerAnalysis::sniff(&AnalyzeSource::Text("not json")), IoConfidence::Medium);
        }

        #[semio_framework_async_macros::async_test]
        async fn analyze_decodes_real_gltf_json_text_directly() {
            let text = r#"{"asset":{"version":"2.0"},"scenes":[]}"#;
            let analysis = GltfAnalyzerAnalysis::analyze(&[AnalyzeSource::Text(text)]);
            assert_eq!(analysis.await.confidence, IoConfidence::High);
            let snap = analysis.await.parts.snapshot.expect("snapshot");
            assert_eq!(snap.document.asset.version, "2.0");
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec GltfBuilderFacets {
        construction: GltfBuilderConstruction,
        analysis: GltfAnalyzerAnalysis,
        composition: super::super::io::derived_composition::GltfComposerComposition,
    }
    builder: GltfBuilder,
    analyzer: GltfAnalyzer,
    composer: GltfComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::artifacts::gltf::engine::empty_gltf_snapshot` through the `engine` barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_gltf_snapshot() -> GltfSnapshot {
    GltfSnapshot::default()
}

/// 🌳️ P2-FG3: a genuinely non-trivial persisted snapshot (one scene/node/mesh/accessor/material/
/// buffer PLUS one of every WEAK collection item — bufferView/texture/image/sampler/skin/
/// animation/camera — and populated `extensionsUsed`/`extras`) — used by this artifact's own
/// conformance-law tests AND by the shipped `.dsl.semio`/`.pack.semio` example fixtures (never a
/// bare fake like the pre-FG3 `{"hello":"stdio.gltf","n":1}` stub, `fixture_honesty_law`'s own
/// mandate). Mirrors `demo_json_snapshot`'s own role in json's pilot report.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_gltf_snapshot() -> GltfSnapshot {
    use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
    use crate::artifacts::gltf::schema::snapshot::{
        GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAsset, GltfCamera, GltfCameraProjection, GltfImage, GltfInterpolation, GltfMaterial, GltfNode, GltfPbrMetallicRoughness, GltfPerspective, GltfSampler,
        GltfScene, GltfSkin, GltfTexture,
    };

    let document = GltfDocument {
        asset: GltfAsset { version: "2.0".into(), generator: Some("semio".into()), ..GltfAsset::default() },
        scene: Some(0),
        scenes: vec![GltfScene { nodes: vec![0], name: Some("root-scene".into()), ..GltfScene::default() }],
        nodes: vec![GltfNode { mesh: Some(0), camera: Some(0), name: Some("root-node".into()), ..GltfNode::default() }],
        meshes: vec![GltfMesh { primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(), 0)], indices: None, material: Some(0), mode: Some(4), targets: Vec::new(), extensions: None, extras: None }], ..GltfMesh::default() }],
        accessors: vec![GltfAccessor {
            buffer_view: Some(0),
            byte_offset: 0,
            component_type: GltfComponentType::Float,
            normalized: false,
            count: 3,
            kind: GltfAccessorType::Vec3,
            max: Some(vec![1.0, 1.0, 1.0]),
            min: Some(vec![0.0, 0.0, 0.0]),
            sparse: None,
            name: Some("positions".into()),
            extensions: None,
            extras: None,
        }],
        buffer_views: vec![GltfBufferView { buffer: 0, byte_offset: 0, byte_length: 36, byte_stride: None, target: Some(34962), name: None, extensions: None, extras: None }],
        // A real `data:` URI (not `None`) -- `None` would round-trip LOSSY through the TEXT
        // facet specifically (`serialize_gltf_document` embeds any uri-less buffer as a data URI
        // on print, so re-parsing would fabricate a `Some(..)` that never matches this demo's own
        // `None`, a real asymmetry discovered by `fixture_honesty_law` -- setting a genuine data
        // URI up front keeps BOTH the text (`parse_gltf_document`) and GLB (`decode_glb`, which
        // never embeds when a buffer already declares a `uri`) facets byte-for-byte lossless.
        buffers: vec![GltfBuffer { byte_length: 36, uri: Some(crate::artifacts::gltf::engine::encode_data_uri("application/octet-stream", &[0u8; 36])), name: Some("geometry".into()), extensions: None, extras: None }],
        materials: vec![GltfMaterial {
            name: Some("triangle-material".into()),
            pbr_metallic_roughness: Some(GltfPbrMetallicRoughness { base_color_factor: [1.0, 0.0, 0.0, 1.0], metallic_factor: 0.0, roughness_factor: 0.8, ..GltfPbrMetallicRoughness::default() }),
            ..GltfMaterial::default()
        }],
        textures: vec![GltfTexture { sampler: Some(0), source: Some(0), name: None, extensions: None, extras: None }],
        images: vec![GltfImage { uri: Some("texture.png".into()), ..GltfImage::default() }],
        samplers: vec![GltfSampler::default()],
        skins: vec![GltfSkin { joints: vec![0], name: Some("root-skin".into()), ..GltfSkin::default() }],
        animations: vec![GltfAnimation {
            channels: vec![GltfAnimationChannel { sampler: 0, target: GltfAnimationChannelTarget { node: Some(0), path: GltfAnimationPath::Translation, extensions: None, extras: None }, extensions: None, extras: None }],
            samplers: vec![crate::artifacts::gltf::schema::snapshot::GltfAnimationSampler { input: 0, interpolation: GltfInterpolation::Linear, output: 0, extensions: None, extras: None }],
            name: Some("spin".into()),
            extensions: None,
            extras: None,
        }],
        cameras: vec![GltfCamera {
            projection: GltfCameraProjection::Perspective(GltfPerspective { aspect_ratio: Some(1.777), yfov: 0.8, zfar: Some(100.0), znear: 0.1, extensions: None, extras: None }),
            name: Some("main-camera".into()),
            extensions: None,
            extras: None,
        }],
        extensions_used: vec!["KHR_materials_unlit".into()],
        extensions_required: Vec::new(),
        extensions: None,
        extras: Some(GltfJson::Object(vec![("generator-note".into(), GltfJson::String("fg3 demo fixture".into()))])),
    };
    GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![vec![0u8; 36]], source_form: GltfSourceForm::Json }
}
//#endregion 🔖️DocumentHelpers
