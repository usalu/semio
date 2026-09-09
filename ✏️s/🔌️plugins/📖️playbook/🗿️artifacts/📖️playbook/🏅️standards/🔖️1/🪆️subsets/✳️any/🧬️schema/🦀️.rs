//! 🧬️ Playbook artifact schema — every field of the artifact with its state class.

use crate::{PlaybookDocumentChild, PlaybookFlowChild, PLAYBOOK_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ playbook document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.document")]
    pub document: PlaybookDocumentChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub flow: PlaybookFlowChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PlaybookArtifact {
    fn default() -> Self {
        let snapshot = crate::PlaybookSnapshot::default();
        Self { schema: PLAYBOOK_DOCUMENT_SCHEMA.into(), id: "playbook".into(), version: "1".into(), title: None, document: snapshot.document, flow: snapshot.flow }
    }
}

impl PlaybookArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::PlaybookSnapshot {
        crate::PlaybookSnapshot { schema: self.schema.clone(), id: self.id.clone(), version: self.version.clone(), title: self.title.clone(), document: self.document.clone(), flow: self.flow.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::PlaybookSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, version: snapshot.version, title: snapshot.title, document: snapshot.document, flow: snapshot.flow, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::PlaybookSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.version = snapshot.version;
        self.title = snapshot.title;
        self.document = snapshot.document;
        self.flow = snapshot.flow;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `document`/`flow` are `store::ArtifactChild<S>` composed-artifact
/// handles, which speak `serde` (framework-internal, unaffected by this ticket) rather than
/// `ToValue`/`FromValue` directly — bridged per-field through the pre-existing
/// `to_dsl_value`/`from_dsl_value` seam (`🌱️value/🔀️serde`) instead of widening the derive macro to
/// understand child-slot handles. See the fan-out playbook's "composed artifact fields" trap.
impl ::semio_framework_os_kernel::ToValue for PlaybookArtifact {
    fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
        ::semio_framework_os_kernel::DslValue::object([
            ("schema".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.schema)),
            ("id".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.id)),
            ("version".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.version)),
            ("title".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.title)),
            ("document".to_string(), ::semio_framework_os_kernel::to_dsl_value(&self.document).expect("ArtifactChild serializes")),
            ("flow".to_string(), ::semio_framework_os_kernel::to_dsl_value(&self.flow).expect("ArtifactChild serializes")),
        ])
    }
}
impl ::semio_framework_os_kernel::FromValue for PlaybookArtifact {
    fn from_value(value: ::semio_framework_os_kernel::DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ::semio_framework_os_kernel::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: ::semio_framework_os_kernel::FromValue::from_value(field("schema")?)?,
            id: ::semio_framework_os_kernel::FromValue::from_value(field("id")?)?,
            version: ::semio_framework_os_kernel::FromValue::from_value(field("version")?)?,
            title: ::semio_framework_os_kernel::FromValue::from_value(field("title")?)?,
            document: ::semio_framework_os_kernel::from_dsl_value(field("document")?).map_err(::semio_framework_os_kernel::ValueError::new)?,
            flow: ::semio_framework_os_kernel::from_dsl_value(field("flow")?).map_err(::semio_framework_os_kernel::ValueError::new)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.playbook.playbook` — twenty handcrafted schema leaves.
pub fn playbook_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.playbook.playbook",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
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
    use crate::{PlaybookDiff, PlaybookMutation, PlaybookSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct PlaybookBuilderConstruction {
        snapshot: PlaybookSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PlaybookBuilderConstruction {
        type Snapshot = PlaybookSnapshot;
        type Mutation = PlaybookMutation;
        type Diff = PlaybookDiff;
        fn empty() -> Self {
            Self { snapshot: PlaybookSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PlaybookSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(dsl::Diagnostic::error("mutation.apply", dsl::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <PlaybookDiff as protocol::MutationDiff<PlaybookSnapshot>>::apply(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
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
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::PlaybookSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct PlaybookParts {
        pub snapshot: Option<PlaybookSnapshot>,
    }

    pub struct PlaybookAnalyzerAnalysis;

    impl ArtifactAnalysis for PlaybookAnalyzerAnalysis {
        type Parts = PlaybookParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.playbook.playbook", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PlaybookParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PlaybookSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🧱️ A blank block of the requested kind — every optional field defaulted, ready to be edited.
/// Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — pure over `PlaybookBlock`, no app-runtime parameter.
pub fn default_block(id: String, kind: &str) -> crate::PlaybookBlock {
    crate::PlaybookBlock {
        id,
        label: kind.into(),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️document-helpers/🦀️.rs"]
mod document_helpers_tests;
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PlaybookBuilderFacets {
        construction: PlaybookBuilderConstruction,
        analysis: PlaybookAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PlaybookComposerComposition,
    }
    builder: PlaybookBuilder,
    analyzer: PlaybookAnalyzer,
    composer: PlaybookComposer,
);
//#endregion 🧬️DerivedArtifactFacets
