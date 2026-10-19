//! 🧩️ Facets of the `stdio.gltf` 2.0 `any` subset: the derived construction (builder), analysis (`.gltf`/`.glb`/native
//! sniffing through the byte reader) and composition, the composer registry and the generated facet types.
//! They sit above the model, the mutations, the inferences and the byte reader, so they live at the subset root.

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2_0::subsets::any::GltfAnalyzer;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct GltfComposerComposition;

    impl ArtifactComposition for GltfComposerComposition {
        type Snapshot = GltfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_JSON, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_JSON || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "GltfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = GltfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "GltfComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::standards::v2_0::subsets::any::GltfComposer as GltfRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<GltfRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
pub mod derived_construction {
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfComponentType;
    #[cfg(test)] use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfDecodedAccessor;
    use semio_s_artifact_stdio_gltf_mutations::standards::v2_0::subsets::any::schema::mutations::GltfMutation;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAccessor;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfBuffer;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfBufferView;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfJson;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfMaterial;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfMesh;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfNode;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfPrimitive;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfScene;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::GltfDiff;
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.gltf` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct GltfBuilderConstruction {
        snapshot: GltfSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for GltfBuilderConstruction {
        type Snapshot = GltfSnapshot;
        type Mutation = GltfMutation;
        type Diff = GltfDiff;
        fn empty() -> Self {
            Self { snapshot: GltfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<GltfSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<GltfSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = protocol::Mutation::diff(&mutation, &self.snapshot);
            if outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error) {
                self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.gltf.mutation-rejected", semio_framework_diagnostic::TextSpan::at(1, 1), format!("{:?}", outcome.messages())));
            }
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => {
                    self.snapshot = snapshot;
                }
                Err(error) => {
                    self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.gltf.mutation-rejected", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string()));
                }
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
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
        pub fn new(component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize) -> Self {
            Self { buffer_view: None, byte_offset: 0, component_type, accessor_type, count, normalized: false, min: None, max: None }
        }
        pub fn with_buffer_view(mut self, buffer_view: usize, byte_offset: usize) -> Self {
            self.buffer_view = Some(buffer_view);
            self.byte_offset = byte_offset;
            self
        }
        pub fn with_normalized(mut self, normalized: bool) -> Self {
            self.normalized = normalized;
            self
        }
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
        pub fn set_asset_version(&mut self, version: &str) -> &mut Self {
            self.snapshot.document.asset.version = version.to_string();
            self
        }

        /// 📦️ Appends a buffer, storing its real bytes on the snapshot's `buffers` (index-aligned with
        /// `document.buffers`) and recording `byteLength` in the document. Returns the new index.
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
        pub fn add_buffer_view(&mut self, buffer: usize, byte_offset: usize, byte_length: usize, byte_stride: Option<usize>, target: Option<u64>) -> usize {
            let idx = self.snapshot.document.buffer_views.len();
            self.snapshot.document.buffer_views.push(GltfBufferView { buffer, byte_offset, byte_length, byte_stride, target, name: None, extensions: None, extras: None });
            idx
        }

        /// 🔢️ Appends an `accessor` from a typed [`GltfAccessorSpec`]. Returns the new index.
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
        pub fn add_material(&mut self, material: GltfMaterial) -> usize {
            let idx = self.snapshot.document.materials.len();
            self.snapshot.document.materials.push(material);
            idx
        }

        /// 🕸️ Appends an empty `mesh` (primitives added via [`Self::add_mesh_primitive`]). Returns the
        /// new index.
        pub fn add_mesh(&mut self) -> usize {
            let idx = self.snapshot.document.meshes.len();
            self.snapshot.document.meshes.push(GltfMesh::default());
            idx
        }

        /// 🔺️ Appends a primitive to `meshes[mesh]` -- `attributes` are `(semantic, accessor index)`
        /// pairs (e.g. `("POSITION", 0)`), `indices`/`material` are optional accessor/material
        /// indices, `mode` is the primitive topology (defaults to `4` TRIANGLES per spec when unset).
        pub fn add_mesh_primitive(&mut self, mesh: usize, attributes: &[(&str, usize)], indices: Option<usize>, material: Option<usize>, mode: Option<u64>) {
            let primitive = GltfPrimitive { attributes: attributes.iter().map(|(name, idx)| ((*name).to_string(), *idx)).collect(), indices, material, mode, targets: Vec::new(), extensions: None, extras: None };
            let mesh_entry = self.snapshot.document.meshes.get_mut(mesh).expect("mesh index out of range -- call add_mesh first");
            mesh_entry.primitives.push(primitive);
        }

        /// 🧍️ Appends a `node`, optionally referencing a mesh. Returns the new index.
        pub fn add_node(&mut self, mesh: Option<usize>) -> usize {
            let idx = self.snapshot.document.nodes.len();
            self.snapshot.document.nodes.push(GltfNode { mesh, ..GltfNode::default() });
            idx
        }

        /// 🎬️ Appends a `scene` referencing `nodes` (root node indices), with an optional passthrough
        /// `extensions` object (real documents sometimes carry a declared-but-empty `{}` here).
        /// Returns the new index.
        pub fn add_scene(&mut self, nodes: Vec<usize>, extensions: Option<GltfJson>) -> usize {
            let idx = self.snapshot.document.scenes.len();
            self.snapshot.document.scenes.push(GltfScene { nodes, name: None, extensions, extras: None });
            idx
        }

        /// 🎬️ Sets the document's default `scene` index.
        pub fn set_default_scene(&mut self, scene: usize) -> &mut Self {
            self.snapshot.document.scene = Some(scene);
            self
        }

        /// 🧩️ Sets `extensionsUsed` (declared, not necessarily applied -- mirrors real-world documents
        /// that declare an extension namespace without every element using it).
        pub fn set_extensions_used(&mut self, names: Vec<String>) -> &mut Self {
            self.snapshot.document.extensions_used = names;
            self
        }

        /// 📸️ Peeks the in-progress document -- used by tests/callers that need to inspect state
        /// mid-construction without consuming the builder via `build()`.
        pub fn document(&self) -> &semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfDocument {
            &self.snapshot.document
        }

        /// 📦️ Peeks the in-progress resolved buffer bytes.
        pub fn buffers(&self) -> &[Vec<u8>] {
            &self.snapshot.buffers
        }
    }
    //#endregion 🔖️DocumentConstructors

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_construction::*;
pub mod derived_analysis {
    use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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
        match semio_framework_pack_json::parse(trimmed, semio_framework_pack_json::JsonMemberPolicy::Reject) {
            Ok(value) => value.get("asset").and_then(|a| a.get("version")).and_then(|v| v.as_str()).is_some(),
            Err(_) => false,
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.gltf` (2.0/♾️any) sources.
    pub struct GltfAnalyzerAnalysis;

    impl ArtifactAnalysis for GltfAnalyzerAnalysis {
        type Parts = GltfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if looks_like_glb(bytes) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if looks_like_gltf_json(text) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Medium
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = GltfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => {
                        let result = if looks_like_gltf_json(text) { semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::parse_gltf_document(text.trim().as_bytes()) } else { <GltfSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string()) };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = semio_framework_plugin::io::Confidence::Low;
                                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                    AnalyzeSource::Binary(bytes) => {
                        let result = if looks_like_glb(bytes) { semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::decode_glb(bytes) } else { <GltfSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string()) };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = semio_framework_plugin::io::Confidence::Low;
                                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err));
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
    include!("🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
semio_framework_plugin::derive_artifact_facets!(
    pub spec GltfBuilderFacets {
        construction: GltfBuilderConstruction,
        analysis: GltfAnalyzerAnalysis,
        composition: GltfComposerComposition,
    }
    builder: GltfBuilder,
    analyzer: GltfAnalyzer,
    composer: GltfComposer,
);

//#region 🌉️DocumentMutationBridge
/// 🌉️ Mutation laws over real `.gltf`/`.glb` documents: read the bytes, apply the row's mutation (and its inverse)
/// through the central applier, write the result back in the document's own source form.
fn gltf_bridge_read(document: &[u8]) -> Result<semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot, String> {
    if document.starts_with(b"glTF") { semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::decode_glb(document) } else { semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::parse_gltf_document(document) }
}

fn gltf_bridge_write(snapshot: &semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot) -> Result<Vec<u8>, String> {
    match snapshot.source_form {
        semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSourceForm::Glb => semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::encode_glb(snapshot),
        semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSourceForm::Json => Ok(semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::serialize_gltf_document(snapshot)),
    }
}

pub fn gltf_mutated_document(document: &[u8], kind: &str, params_json: &str) -> Result<Vec<u8>, String> {
    let mutation = semio_s_artifact_stdio_gltf_mutations::standards::v2_0::subsets::any::io::text::mutations::gltf_row_mutation(kind, params_json)?;
    let snapshot = gltf_bridge_read(document)?;
    let mutated = semio_s_artifact_stdio_gltf_mutations::standards::v2_0::subsets::any::io::text::mutations::gltf_bridge_apply(kind, "mutation", &mutation, &snapshot)?;
    gltf_bridge_write(&mutated)
}

pub fn gltf_inverse_restored_document(document: &[u8], kind: &str, params_json: &str) -> Result<Vec<u8>, String> {
    let mutation = semio_s_artifact_stdio_gltf_mutations::standards::v2_0::subsets::any::io::text::mutations::gltf_row_mutation(kind, params_json)?;
    let base = gltf_bridge_read(document)?;
    let mut restored = semio_s_artifact_stdio_gltf_mutations::standards::v2_0::subsets::any::io::text::mutations::gltf_bridge_apply(kind, "mutation", &mutation, &base)?;
    for step in <semio_s_artifact_stdio_gltf_mutations::standards::v2_0::subsets::any::schema::mutations::GltfMutation as protocol::Mutation<semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot>>::inverse(&mutation, &base).map_err(semio_framework_value::ValueError::into_message)?.into_iter().rev() {
        restored = semio_s_artifact_stdio_gltf_mutations::standards::v2_0::subsets::any::io::text::mutations::gltf_bridge_apply(kind, "inverse", &step, &restored)?;
    }
    gltf_bridge_write(&restored)
}
//#endregion 🌉️DocumentMutationBridge
