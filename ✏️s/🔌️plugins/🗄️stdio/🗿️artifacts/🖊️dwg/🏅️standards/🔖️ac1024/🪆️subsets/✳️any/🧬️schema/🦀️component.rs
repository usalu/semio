//! 🧬️ DwgArtifact schema — full artifact state.

use crate::artifacts::dwg::DwgSnapshot;
use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::{DwgDecodeStatus, DwgSection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[serde(default)]
    pub maintenance_version: u8,
    #[state(artifact)]
    #[serde(default)]
    pub codepage: u16,
    #[state(artifact)]
    #[serde(default)]
    pub bytes: Vec<u8>,
    #[state(artifact)]
    #[serde(default)]
    pub section_names: Vec<String>,
    #[state(artifact)]
    #[serde(default)]
    pub sections: Vec<DwgSection>,
    #[state(artifact)]
    #[serde(default)]
    pub decode_status: DwgDecodeStatus,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DwgArtifact {
    fn default() -> Self {
        Self::from_snapshot(DwgSnapshot::default())
    }
}

impl DwgArtifact {
    pub fn to_snapshot(&self) -> DwgSnapshot {
        DwgSnapshot {
            schema: self.schema.clone(),
            version: self.version.clone(),
            maintenance_version: self.maintenance_version,
            codepage: self.codepage,
            bytes: self.bytes.clone(),
            section_names: self.section_names.clone(),
            sections: self.sections.clone(),
            decode_status: self.decode_status,
        }
    }

    pub fn from_snapshot(snapshot: DwgSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            version: snapshot.version,
            maintenance_version: snapshot.maintenance_version,
            codepage: snapshot.codepage,
            bytes: snapshot.bytes,
            section_names: snapshot.section_names,
            sections: snapshot.sections,
            decode_status: snapshot.decode_status,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: DwgSnapshot) {
        self.schema = snapshot.schema;
        self.version = snapshot.version;
        self.maintenance_version = snapshot.maintenance_version;
        self.codepage = snapshot.codepage;
        self.bytes = snapshot.bytes;
        self.section_names = snapshot.section_names;
        self.sections = snapshot.sections;
        self.decode_status = snapshot.decode_status;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn dwg_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.dwg",
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
    use crate::artifacts::dwg::{DwgDiff, DwgMutation, DwgSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.dwg` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DwgBuilderConstruction {
        snapshot: DwgSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for DwgBuilderConstruction {
        type Snapshot = DwgSnapshot;
        type Mutation = DwgMutation;
        type Diff = DwgDiff;
        fn empty() -> Self {
            Self { snapshot: DwgSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DwgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DwgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::dwg::schema::mutations::apply_dwg_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <DwgDiff as protocol::MutationDiff<DwgSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::dwg::DwgSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.dwg` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DwgParts {
        pub snapshot: Option<DwgSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.dwg` (ac1024/✳️any) sources.
    pub struct DwgAnalyzerAnalysis;

    impl ArtifactAnalysis for DwgAnalyzerAnalysis {
        type Parts = DwgParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DwgParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DwgSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <DwgSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DwgBuilderFacets {
        construction: derived_construction::DwgBuilderConstruction,
        analysis: derived_analysis::DwgAnalyzerAnalysis,
        composition: super::super::io::derived_composition::DwgComposerComposition,
    }
    builder: DwgBuilder,
    analyzer: DwgAnalyzer,
    composer: DwgComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::artifacts::dwg::standards::v_ac1024::engine::empty_dwg_snapshot` through the `engine`
/// barrel shim, and (via the root `crate::artifacts::dwg::engine` shim, ac1024-only) as
/// `crate::artifacts::dwg::engine::empty_dwg_snapshot` too.
pub fn empty_dwg_snapshot() -> DwgSnapshot {
    DwgSnapshot::default()
}

/// 📄️ The demo `stdio.dwg` (ac1024, the CANONICAL standard per S-6/Decision #5) document —
/// decodes the real, committed 22-byte AC1024 stub (`📚️examples/🎬️demo/🖼️assets/🖊️example.dwg`)
/// via this standard's own real `decode_dwg`. The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `conformance_laws::fixture_honesty_law`, now in `../🚪️io/🦀️component.rs`).
pub fn demo_dwg_snapshot() -> DwgSnapshot {
    let stub = b"AC1024\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    crate::artifacts::dwg::schema::snapshot::decode_dwg(stub).expect("decode ac1024 demo stub")
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️RegisterSchemaSpecs
/// 📇️ `DwgSnapshot`/`DwgDiff` (ac1024) both derive real `dsl::DslRecord`/`dsl::DslDiff` —
/// genuinely callable, same 2-call shape as `stdio.binary`/`stdio.txt`'s own
/// `register_schema_specs`. Per-mutation-variant specs are NOT registered here — no single
/// canonical id exists for a `Mutation` enum's N independently-shaped variants (same documented
/// scope boundary every other pilot's own `register_schema_specs` observes). Dissolved out of
/// `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — one of the ten
/// deliberate imperative `engine::register()`-family calls left in place at the stdio plugin
/// root's own `.setup(crate::artifacts::dwg::engine::register_schema_specs)`, reached through the
/// root `engine` shim (ac1024-only) and this standard's own `engine` barrel shim.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.dwg", crate::artifacts::dwg::schema::snapshot::DwgSnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.dwg#diff", crate::artifacts::dwg::schema::diff::DwgDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}
//#endregion 🔖️RegisterSchemaSpecs
