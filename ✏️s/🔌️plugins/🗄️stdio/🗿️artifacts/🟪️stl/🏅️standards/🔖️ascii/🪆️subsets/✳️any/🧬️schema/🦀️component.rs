//! 🧬️ StlArtifact schema — full artifact state.

use crate::artifacts::stl::schema::snapshot::StlTriangle;
use crate::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.stl` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.stl")]
pub struct StlArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub solid_name: String,
    #[state(artifact)]
    #[serde(default)]
    pub triangles: Vec<StlTriangle>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for StlArtifact {
    fn default() -> Self {
        Self::from_snapshot(StlSnapshot::default())
    }
}

impl StlArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> StlSnapshot {
        StlSnapshot { schema: self.schema.clone(), solid_name: self.solid_name.clone(), triangles: self.triangles.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: StlSnapshot) -> Self {
        Self { schema: snapshot.schema, solid_name: snapshot.solid_name, triangles: snapshot.triangles }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: StlSnapshot) {
        self.schema = snapshot.schema;
        self.solid_name = snapshot.solid_name;
        self.triangles = snapshot.triangles;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.stl`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn stl_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.stl",
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
            rust: include_str!("🧬️mutations/🦀️.rs"),
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
    use crate::artifacts::stl::{StlDiff, StlMutation, StlSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.stl` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct StlBuilderConstruction {
        snapshot: StlSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for StlBuilderConstruction {
        type Snapshot = StlSnapshot;
        type Mutation = StlMutation;
        type Diff = StlDiff;
        fn empty() -> Self {
            Self { snapshot: StlSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<StlSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<StlSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::stl::schema::mutations::apply_stl_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <StlDiff as protocol::MutationDiff<StlSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::stl::StlSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.stl` parts.
    #[derive(Clone, Debug, Default)]
    pub struct StlParts {
        pub snapshot: Option<StlSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.stl` (ascii/✳️any) sources.
    pub struct StlAnalyzerAnalysis;

    /// 🔍 ASCII STL starts with a `solid` keyword and a real body has `facet`/`vertex`
    /// structure; binary STL has no fixed magic, so a plausible triangle-count framing
    /// (`84 + count*50 == len`) is the best available signal.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_stl(bytes: &[u8]) -> IoConfidence {
        if bytes.starts_with(b"solid") {
            if let Ok(text) = std::str::from_utf8(bytes) {
                if text.contains("facet") && text.contains("vertex") && text.contains("endsolid") {
                    return IoConfidence::High;
                }
            }
            return IoConfidence::Medium;
        }
        if bytes.len() >= 84 {
            let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
            if 84 + count * 50 == bytes.len() {
                return IoConfidence::High;
            }
        }
        IoConfidence::Low
    }

    impl ArtifactAnalysis for StlAnalyzerAnalysis {
        type Parts = StlParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    looks_like_stl(body.as_bytes())
                }
                AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                    Ok((_, inner)) => looks_like_stl(&inner),
                    Err(_) => looks_like_stl(bytes),
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = StlParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <StlSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <StlSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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

        #[semio_framework_async_macros::async_test]
        async fn sniff_real_ascii_stl_is_high() {
            let text = "solid mesh\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\nendsolid mesh\n";
            assert_eq!(StlAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[semio_framework_async_macros::async_test]
        async fn sniff_unrelated_text_is_low() {
            assert_eq!(StlAnalyzerAnalysis::sniff(&AnalyzeSource::Text("not an stl file")), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec StlBuilderFacets {
        construction: StlBuilderConstruction,
        analysis: StlAnalyzerAnalysis,
        composition: super::super::io::derived_composition::StlComposerComposition,
    }
    builder: StlBuilder,
    analyzer: StlAnalyzer,
    composer: StlComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::artifacts::stl::engine::empty_stl_snapshot` through the `engine` barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_stl_snapshot() -> StlSnapshot {
    StlSnapshot::default()
}

/// 📄️ FG1: the demo `stdio.stl` document — a non-degenerate, non-empty `solid_name` plus two
/// distinct-normal triangles, matching the companion real-format fixture assets
/// (`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`, both literally this
/// snapshot's `print_dsl`/`encode_pack` output, asserted equal by `conformance_laws::
/// fixture_honesty_law`). Deliberately avoids the empty-`solid_name` degenerate case: the
/// grammar's `LINE` raw-span terminal captures rest-of-physical-line starting at the NEXT real
/// token after `"solid"`/`"endsolid"` — when the name is empty, that next token is on a LATER
/// line (whitespace/newlines are lexer trivia), which would swallow that later line's content as
/// if it were the name. This is a real, narrow edge of the `LINE` primitive itself (shared,
/// framework-level — `📖️grammar/🦀️component.rs`'s `match_raw_span`), not a bug in this artifact;
/// every other pilot's own demo/fixture picks similarly avoid degenerate corners their grammar's
/// primitives don't cleanly cover (same "model realistically" convention this ticket's recipe
/// documents throughout, not a `mechanism_gaps` entry of its own since it's a strict subset of the
/// already-documented `protocol-prim-ref-recursion`-adjacent raw-span family).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_stl_snapshot() -> StlSnapshot {
    StlSnapshot {
        schema: STDIO_STL_DOCUMENT_SCHEMA.into(),
        solid_name: "demo".into(),
        triangles: vec![StlTriangle { normal: [0.0, 0.0, 1.0], vertices: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] }, StlTriangle { normal: [0.0, 0.0, -1.0], vertices: [[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]] }],
    }
}
//#endregion 🔖️DocumentHelpers
