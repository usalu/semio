//! 🚪️ IO stdio.json (rfc8259/🌍️geojson) — the base subset's JSON codec composes the snapshot, the
//! RFC 7946 gate decides whether it may carry the geojson dialect stamp, and the registered
//! `SubsetValidator` re-runs that gate on the wire payload. Registration flows through the standard's
//! composer aggregator and the artifact root's `subset_validators`, as for 🛜️i-json.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use crate::standards::v_rfc8259::subsets::base::io::JsonComposer as JsonAnyComposer;
    use crate::standards::v_rfc8259::subsets::geojson::schema::check_geojson_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator};

    const DIALECT_GEOJSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("geojson") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    /// 🎹️ Composes through the base JSON composer, then stamps `rfc8259/geojson` only when RFC 7946
    /// conformance holds; soft findings (a GJ2008 `crs`, left-handed rings) travel as diagnostics.
    pub struct JsonGeoJsonComposerComposition;

    impl ArtifactComposition for JsonGeoJsonComposerComposition {
        type Snapshot = JsonSnapshot;
        const WRITES: Dialect = DIALECT_GEOJSON;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_GEOJSON, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = JsonAnyComposer::compose(sources)?;
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = check_geojson_conformance(&inner.snapshot).into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let message = format!("RFC 7946 GeoJSON conformance violated: {}", hard.iter().map(|d| d.message.as_str()).collect::<Vec<_>>().join("; "));
                return Err(ComposeError { message, diagnostics: hard.into_iter().chain(soft).collect() });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }

    /// 🛡️ The registered `SubsetValidator` for `rfc8259/geojson`.
    pub struct JsonGeoJsonValidator;

    impl SubsetValidator for JsonGeoJsonValidator {
        const DIALECT: Dialect = DIALECT_GEOJSON;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}")),
                IoPayload::Text(text) => <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string()),
            };
            match decoded {
                Ok(snapshot) => check_geojson_conformance(&snapshot),
                Err(error) => vec![Diagnostic {
                    code: FaultCode::new("stdio.json.geojson.validate-decode-failed"),
                    severity: Severity::Error,
                    span: TextSpan::at(1, 1),
                    message: format!("the geojson payload is not a JsonSnapshot: {error}"),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path="."]
pub mod sqlite {
    #[path="🪶️sqlite/📸️snapshot/🦀️.rs"]
    pub mod snapshot;
}

pub mod derived_construction {
    use crate::standards::v_rfc8259::subsets::geojson::schema::check_geojson_conformance;
    use crate::standards::v_rfc8259::subsets::base::schema::diff::JsonDiff;
    use crate::standards::v_rfc8259::subsets::base::schema::mutations::JsonMutation;
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    /// 🏗️ The base subset's mutation vocabulary over the shared `JsonSnapshot`; only the build gate is
    /// this subset's own: a snapshot that is not RFC 7946 GeoJSON never builds.
    #[derive(Clone, Debug, Default)]
    pub struct JsonGeoJsonBuilderConstruction {
        snapshot: JsonSnapshot,
    }

    impl ArtifactBuilder for JsonGeoJsonBuilderConstruction {
        type Snapshot = JsonSnapshot;
        type Mutation = JsonMutation;
        type Diff = JsonDiff;

        fn empty() -> Self {
            Self { snapshot: JsonSnapshot::from_value(serde_json::json!({ "type": "FeatureCollection", "features": [] })) }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self { snapshot: <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text)? })
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes)? })
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = crate::schema::mutations::apply_json_mutation(&mut self.snapshot, &mutation);
            (self, outcome)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }

        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            let hard: Vec<semio_framework_diagnostic::Diagnostic> = check_geojson_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v_rfc8259::subsets::geojson::schema::check_geojson_conformance;
    use crate::standards::v_rfc8259::subsets::base::io::JsonAnalyzer as JsonAnyAnalyzer;
    use crate::standards::v_rfc8259::subsets::base::io::JsonParts;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("geojson") };

    /// 🧐️ The base analyzer's real parse, with RFC 7946 conformance folded on top.
    pub struct JsonGeoJsonAnalyzerAnalysis;

    impl ArtifactAnalysis for JsonGeoJsonAnalyzerAnalysis {
        type Parts = JsonParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            JsonAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = JsonAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_geojson_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
                    confidence = semio_framework_plugin::io::Confidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec JsonGeoJsonBuilderFacets {
        construction: JsonGeoJsonBuilderConstruction,
        analysis: JsonGeoJsonAnalyzerAnalysis,
        composition: crate::standards::v_rfc8259::subsets::geojson::io::derived_composition::JsonGeoJsonComposerComposition,
    }
    builder: JsonGeoJsonBuilder,
    analyzer: JsonGeoJsonAnalyzer,
    composer: JsonGeoJsonComposer,
);
