//! 🚪️ IO stdio.json (rfc8259/🌍️geojson) — the base subset's JSON codec composes the snapshot, the
//! RFC 7946 gate decides whether it may carry the geojson dialect stamp, and the registered
//! `SubsetValidator` re-runs that gate on the wire payload. Registration flows through the standard's
//! composer aggregator and the artifact root's `subset_validators`, as for 🛜️i-json.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use crate::standards::v_rfc8259::subsets::base::schema::JsonComposer as JsonAnyComposer;
    use crate::standards::v_rfc8259::subsets::geojson::schema::check_geojson_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator};

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
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
