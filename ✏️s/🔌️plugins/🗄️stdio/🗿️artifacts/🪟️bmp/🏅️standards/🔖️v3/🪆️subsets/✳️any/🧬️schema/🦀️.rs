//! 🧬️ BmpArtifact schema — full artifact state.

use crate::BmpSnapshot;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub bytes: Vec<u8>,
}

impl Default for BmpArtifact {
    fn default() -> Self {
        Self::from_snapshot(BmpSnapshot::default())
    }
}

impl BmpArtifact {
    pub fn to_snapshot(&self) -> BmpSnapshot {
        BmpSnapshot { schema: self.schema.clone(), bytes: self.bytes.clone() }
    }

    pub fn from_snapshot(snapshot: BmpSnapshot) -> Self {
        Self { schema: snapshot.schema, bytes: snapshot.bytes }
    }

    pub fn set_snapshot(&mut self, snapshot: BmpSnapshot) {
        self.schema = snapshot.schema;
        self.bytes = snapshot.bytes;
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.bmp`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bmp_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.bmp",
        artifact: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::{BmpDiff, BmpMutation, BmpSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.bmp` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct BmpBuilderConstruction {
        snapshot: BmpSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for BmpBuilderConstruction {
        type Snapshot = BmpSnapshot;
        type Mutation = BmpMutation;
        type Diff = BmpDiff;
        fn empty() -> Self {
            Self { snapshot: BmpSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<BmpSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<BmpSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_bmp_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <BmpDiff as protocol::MutationDiff<BmpSnapshot>>::apply(&diff, &self.snapshot)?;
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
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::BmpSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.bmp` parts.
    #[derive(Clone, Debug, Default)]
    pub struct BmpParts {
        pub snapshot: Option<BmpSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.bmp` (v3/✳️any) sources.
    pub struct BmpAnalyzerAnalysis;

    impl ArtifactAnalysis for BmpAnalyzerAnalysis {
        type Parts = BmpParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG: [u8; 2] = *b"BM";
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 2 && bytes[0..2] == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.bmp's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 2 bytes to sniff the real signature.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(4).collect();
                    if hex.len() < 4 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 2];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return IoConfidence::Low,
                        }
                    }
                    if decoded == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = BmpParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <BmpSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <BmpSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec BmpBuilderFacets {
        construction: BmpBuilderConstruction,
        analysis: BmpAnalyzerAnalysis,
        composition: super::super::io::derived_composition::BmpComposerComposition,
    }
    builder: BmpBuilder,
    analyzer: BmpAnalyzer,
    composer: BmpComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
// 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// `empty_bmp_snapshot`/`demo_bmp_snapshot` relocated here verbatim (pure helpers over the
// document type, destination rule 5); `BmpEngine` (zero construction sites) deleted outright;
// the real codec (`encode_bmp`/`decode_bmp` + every pure format algorithm) + the protected
// `register()` cluster (`crate::engine::register()` is one of stdio's 10
// deliberate imperative plugin-root calls — untouched, reached via this standard's own inline
// `engine` barrel) + `io_registry` all moved to `../🚪️io`; tests moved beside what they now test.
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_bmp_snapshot() -> BmpSnapshot {
    BmpSnapshot::default()
}

/// 🎬 P2-FG2: canonical demo snapshot — the same value the real `.dsl.semio`/`.pack.semio`
/// fixtures under `📚️examples/🎬️demo/🖼️assets/` are genuine `print_dsl`/`encode_pack` output
/// of (regenerated this wave via a real `encode_bmp`/`print_dsl`/`encode_pack` call, replacing
/// the pre-existing fake "hello" placeholder text). 4x2 24-bit `BI_RGB`, bottom-up, 8 distinct
/// non-solid RGBA pixels (`row_bytes(4, 24) == 12`, already a multiple of 4, so this fixture
/// does NOT exercise row padding — `gradient_checkerboard_24bit_round_trip`'s own 6-wide fixture
/// in `../🚪️io`'s own tests already covers that) — `header_size`/`planes`/`bits_per_pixel`/
/// `compression` are exactly what `encode_bmp` always hardcodes (40/1/24/0, see its own
/// `EncodeScopeNote`), so this snapshot is safe against `encode_bmp`'s own canonicalization (any
/// other value here would silently "self-correct" on the first decode and break
/// `fixture_honesty_law`'s `parse_dsl(fixture) == demo()` identity). No palette (bpp=24 has none).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_bmp_snapshot() -> BmpSnapshot {
    BmpSnapshot { schema: crate::STDIO_BMP_DOCUMENT_SCHEMA.into(), bytes: crate::io::demo_bmp_bytes() }
}
//#endregion 🔖️DocumentHelpers
