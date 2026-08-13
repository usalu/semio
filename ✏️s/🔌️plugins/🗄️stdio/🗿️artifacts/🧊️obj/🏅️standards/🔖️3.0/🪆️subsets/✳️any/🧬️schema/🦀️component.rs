//! 🧬️ ObjArtifact schema — full artifact state, mirrors `ObjSnapshot` field-for-field.

use crate::artifacts::obj::ObjSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.obj` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj")]
pub struct ObjArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<crate::artifacts::obj::schema::snapshot::ObjVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub texcoords: Vec<crate::artifacts::obj::schema::snapshot::ObjTexCoord>,
    #[state(persistent)]
    #[serde(default)]
    pub normals: Vec<crate::artifacts::obj::schema::snapshot::ObjNormal>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<crate::artifacts::obj::schema::snapshot::ObjFace>,
    #[state(persistent)]
    #[serde(default)]
    pub groups: Vec<crate::artifacts::obj::schema::snapshot::ObjGroup>,
    #[state(persistent)]
    #[serde(default)]
    pub objects: Vec<crate::artifacts::obj::schema::snapshot::ObjObject>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtllib: Option<String>,
    #[state(persistent)]
    #[serde(default)]
    pub usemtl: Vec<crate::artifacts::obj::schema::snapshot::ObjUsemtlRange>,
    #[state(persistent)]
    #[serde(default)]
    pub smoothing_groups: Vec<crate::artifacts::obj::schema::snapshot::ObjSmoothingRange>,
    #[state(persistent)]
    #[serde(default)]
    pub unknown_statements: Vec<crate::artifacts::obj::schema::snapshot::ObjUnknownStatement>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ObjArtifact {
    fn default() -> Self {
        Self::from_snapshot(ObjSnapshot::default())
    }
}

impl ObjArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> ObjSnapshot {
        ObjSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),
            texcoords: self.texcoords.clone(),
            normals: self.normals.clone(),
            faces: self.faces.clone(),
            groups: self.groups.clone(),
            objects: self.objects.clone(),
            mtllib: self.mtllib.clone(),
            usemtl: self.usemtl.clone(),
            smoothing_groups: self.smoothing_groups.clone(),
            unknown_statements: self.unknown_statements.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: ObjSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            vertices: snapshot.vertices,
            texcoords: snapshot.texcoords,
            normals: snapshot.normals,
            faces: snapshot.faces,
            groups: snapshot.groups,
            objects: snapshot.objects,
            mtllib: snapshot.mtllib,
            usemtl: snapshot.usemtl,
            smoothing_groups: snapshot.smoothing_groups,
            unknown_statements: snapshot.unknown_statements,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: ObjSnapshot) {
        self.schema = snapshot.schema;
        self.vertices = snapshot.vertices;
        self.texcoords = snapshot.texcoords;
        self.normals = snapshot.normals;
        self.faces = snapshot.faces;
        self.groups = snapshot.groups;
        self.objects = snapshot.objects;
        self.mtllib = snapshot.mtllib;
        self.usemtl = snapshot.usemtl;
        self.smoothing_groups = snapshot.smoothing_groups;
        self.unknown_statements = snapshot.unknown_statements;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.obj`.
pub fn obj_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.obj",
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
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::obj::{ObjDiff, ObjMutation, ObjSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.obj` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct ObjBuilderConstruction {
        snapshot: ObjSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ObjBuilderConstruction {
        type Snapshot = ObjSnapshot;
        type Mutation = ObjMutation;
        type Diff = ObjDiff;
        fn empty() -> Self {
            Self { snapshot: ObjSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ObjSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ObjSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::obj::schema::mutations::apply_obj_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <ObjDiff as protocol::MutationDiff<ObjSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::obj::ObjSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.obj` parts.
    #[derive(Clone, Debug, Default)]
    pub struct ObjParts {
        pub snapshot: Option<ObjSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Sniff
    /// 🔍 OBJ has no magic byte signature (it's plain text) — sniff by scanning the first
    /// ~200 non-blank lines for real Wavefront keyword shapes (`v `/`f ` are the strong
    /// signal; `vt`/`vn`/`o`/`g`/`usemtl`/`s`/`mtllib` are weaker corroborating signals).
    fn looks_like_obj(text: &str) -> IoConfidence {
        let mut vertex_lines = 0u32;
        let mut face_lines = 0u32;
        let mut other_tokens = 0u32;
        for line in text.lines().take(200) {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            match line.split_whitespace().next() {
                Some("v") => vertex_lines += 1,
                Some("f") => face_lines += 1,
                Some("vt") | Some("vn") | Some("o") | Some("g") | Some("usemtl") | Some("s") | Some("mtllib") => other_tokens += 1,
                _ => {}
            }
        }
        if vertex_lines > 0 && face_lines > 0 {
            IoConfidence::High
        } else if vertex_lines > 0 || face_lines > 0 || other_tokens > 0 {
            IoConfidence::Medium
        } else {
            IoConfidence::Low
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.obj` (3.0/✳️any) sources.
    pub struct ObjAnalyzerAnalysis;

    impl ArtifactAnalysis for ObjAnalyzerAnalysis {
        type Parts = ObjParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    looks_like_obj(body)
                }
                AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                    Ok((_, inner)) => match String::from_utf8(inner) {
                        Ok(text) => looks_like_obj(&text),
                        Err(_) => IoConfidence::Low,
                    },
                    Err(_) => match std::str::from_utf8(bytes) {
                        Ok(text) => looks_like_obj(text),
                        Err(_) => IoConfidence::Low,
                    },
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ObjParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ObjSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.text",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <ObjSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.binary",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
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

        #[test]
        fn sniff_real_obj_text_is_high() {
            let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
            assert_eq!(ObjAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[test]
        fn sniff_unrelated_text_is_low() {
            let text = "{\"not\": \"an obj file at all\"}";
            assert_eq!(ObjAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec ObjBuilderFacets {
        construction: derived_construction::ObjBuilderConstruction,
        analysis: derived_analysis::ObjAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ObjComposerComposition,
    }
    builder: ObjBuilder,
    analyzer: ObjAnalyzer,
    composer: ObjComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::artifacts::obj::engine::empty_obj_snapshot` through the `engine` barrel shim.
pub fn empty_obj_snapshot() -> ObjSnapshot {
    ObjSnapshot::default()
}

/// 📄️ Raw demo Wavefront OBJ text — a two-triangle quad split across two named groups/materials,
/// matching `📚️examples/🎬️demo/🖼️assets/🧊️example.obj` verbatim (this module's own single source
/// of truth for the demo fixture — exercises every statement kind: `mtllib`, `v`/`vt`/`vn`
/// (incl. `w`-omitted forms), `f` (`v/vt/vn` triangles), `o`, `g`, `usemtl`, `s` (both numeric and
/// `off`), a `#` comment, and one genuinely-unrecognized keyword line retained via
/// `unknown_statements`).
const DEMO_OBJ_TEXT: &str = "# stdio.obj demo -- a two-triangle quad split across two named groups/materials\n\
mtllib demo.mtl\n\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
o Quad\ng Front\nusemtl Red\ns 1\n\
f 1/1/1 2/2/1 3/3/1\n\
f 1/1/1 3/3/1 4/4/1\n\
g Back\nusemtl Blue\ns off\n\
f 3/3/1 2/2/1 1/1/1\n\
# trailing note retained verbatim\n\
weird_directive foo bar\n";

/// 📄️ The `demo` example snapshot, parsed once from [`DEMO_OBJ_TEXT`] and stabilized to the
/// SECOND-generation decode/encode fixed point this module's own doc comment documents
/// (`unknown_statements[].line_index` renumbers into the trailer on the first re-encode) — so
/// `print_dsl(demo_obj_snapshot())` is genuinely stable, matching `🗣️example.dsl.semio`'s own
/// `fixture_honesty_law` requirement exactly. Same pattern `stdio.txt`'s own
/// `demo_txt_snapshot()`/`stdio.csv`'s own `demo_csv_snapshot()` establish.
pub fn demo_obj_snapshot() -> ObjSnapshot {
    let gen1 = crate::artifacts::obj::engine::decode_obj(DEMO_OBJ_TEXT).unwrap_or_else(|_| empty_obj_snapshot());
    let gen2_text = crate::artifacts::obj::engine::encode_obj(&gen1);
    crate::artifacts::obj::engine::decode_obj(&gen2_text).unwrap_or(gen1)
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️RegisterSchemaSpecs
/// 📇️ P2-FG1: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) — a
/// real, non-fabricated call: `ObjSnapshot` genuinely derives `#[derive(dsl::DslRecord)]`
/// (`📸️snapshot/🦀️component.rs`), so `__dsl_spec` is a real generated constructor. `ObjDiff` is
/// hand-rolled (§3b tri-state blocker — `Option<Option<T>>` fields — see `🔺️diff/🦀️component.rs`'s
/// own module doc comment), NOT `#[derive(dsl::DslDiff)]`, so it has no `__dsl_diff_spec` to
/// register under `"stdio.obj#diff"` — filed as a `mechanism_gaps` entry rather than fabricating a
/// `RecordSpec` that would diverge from the real hand-rolled diff codec (same treatment
/// gif89a/svg's own hand-rolled diffs get). `#[cfg]`-gated to match `os_dsl::registry`'s own
/// `#[cfg(not(target_arch = "wasm32"))]`. Dissolved out of `⚙️engine` (ticket 26/08/12/
/// ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — one of the ten deliberate imperative
/// `engine::register()`-family calls left in place at the stdio plugin root's own
/// `.setup(crate::artifacts::obj::engine::register_schema_specs)`, reached through the `engine`
/// barrel shim.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.obj", ObjSnapshot::__dsl_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}
//#endregion 🔖️RegisterSchemaSpecs
