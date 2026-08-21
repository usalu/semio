//! 🧬️ DwgArtifact schema — full artifact state.

use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::{
    DwgApplicationHistory, DwgApplicationInfo, DwgAuxiliaryHeader, DwgClass, DwgDependency, DwgHeaderVariables, DwgIndexedPreview, DwgLogicalDrawing, DwgRevisionHistory, DwgSummaryInfo, DwgTemplate,
};
use crate::artifacts::dwg::DwgSnapshot;
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
    pub drawing: DwgLogicalDrawing,
    #[state(artifact)]
    #[serde(default)]
    pub header: DwgHeaderVariables,
    #[state(artifact)]
    #[serde(default)]
    pub classes: Vec<DwgClass>,
    #[state(artifact)]
    #[serde(default)]
    pub dependencies: Vec<DwgDependency>,
    #[state(artifact)]
    #[serde(default)]
    pub summary: DwgSummaryInfo,
    #[state(artifact)]
    #[serde(default)]
    pub application: DwgApplicationInfo,
    #[state(artifact)]
    #[serde(default)]
    pub template: DwgTemplate,
    #[state(artifact)]
    #[serde(default)]
    pub auxiliary_header: DwgAuxiliaryHeader,
    #[state(artifact)]
    #[serde(default)]
    pub revision_history: DwgRevisionHistory,
    #[state(artifact)]
    #[serde(default)]
    pub preview: DwgIndexedPreview,
    #[state(artifact)]
    #[serde(default)]
    pub application_history: DwgApplicationHistory,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DwgArtifact {
    fn default() -> Self {
        Self::from_snapshot(DwgSnapshot::default())
    }
}

impl DwgArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> DwgSnapshot {
        DwgSnapshot {
            schema: self.schema.clone(),
            version: self.version.clone(),
            maintenance_version: self.maintenance_version,
            codepage: self.codepage,
            drawing: self.drawing.clone(),
            header: self.header.clone(),
            classes: self.classes.clone(),
            dependencies: self.dependencies.clone(),
            summary: self.summary.clone(),
            application: self.application.clone(),
            template: self.template.clone(),
            auxiliary_header: self.auxiliary_header.clone(),
            revision_history: self.revision_history.clone(),
            preview: self.preview.clone(),
            application_history: self.application_history.clone(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: DwgSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            version: snapshot.version,
            maintenance_version: snapshot.maintenance_version,
            codepage: snapshot.codepage,
            drawing: snapshot.drawing,
            header: snapshot.header,
            classes: snapshot.classes,
            dependencies: snapshot.dependencies,
            summary: snapshot.summary,
            application: snapshot.application,
            template: snapshot.template,
            auxiliary_header: snapshot.auxiliary_header,
            revision_history: snapshot.revision_history,
            preview: snapshot.preview,
            application_history: snapshot.application_history,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: DwgSnapshot) {
        self.schema = snapshot.schema;
        self.version = snapshot.version;
        self.maintenance_version = snapshot.maintenance_version;
        self.codepage = snapshot.codepage;
        self.drawing = snapshot.drawing;
        self.header = snapshot.header;
        self.classes = snapshot.classes;
        self.dependencies = snapshot.dependencies;
        self.summary = snapshot.summary;
        self.application = snapshot.application;
        self.template = snapshot.template;
        self.auxiliary_header = snapshot.auxiliary_header;
        self.revision_history = snapshot.revision_history;
        self.preview = snapshot.preview;
        self.application_history = snapshot.application_history;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
    use crate::artifacts::dwg::{DwgDiff, DwgMutation, DwgSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

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
        async fn empty() -> Self {
            Self { snapshot: DwgSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DwgSnapshot as store::ArtifactDsl>::parse_dsl(text)?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DwgSnapshot as store::ArtifactPack>::decode_pack(bytes)?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::dwg::schema::mutations::apply_dwg_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <DwgDiff as protocol::MutationDiff<DwgSnapshot>>::apply(&diff, &self.snapshot)?;
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
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::dwg::DwgSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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

        async fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DwgParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DwgSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <DwgSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DwgBuilderFacets {
        construction: DwgBuilderConstruction,
        analysis: DwgAnalyzerAnalysis,
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_dwg_snapshot() -> DwgSnapshot {
    DwgSnapshot::default()
}

/// 📄️ The minimal logical `stdio.dwg` AC1024 demonstration document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_dwg_snapshot() -> DwgSnapshot {
    DwgSnapshot { version: "AC1024".into(), maintenance_version: 2, codepage: 30, ..Default::default() }
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.dwg", DwgSnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.dwg#diff", crate::artifacts::dwg::schema::diff::DwgDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}
//#endregion 🔖️RegisterSchemaSpecs
