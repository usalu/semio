//! 🧬️ LasArtifact schema — full artifact state.

use crate::artifacts::las::LasSnapshot;
use schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las")]
pub struct LasArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header: crate::artifacts::las::schema::snapshot::LasHeader,
    #[state(artifact)]
    #[value(default)]
    pub vlrs: Vec<crate::artifacts::las::schema::snapshot::LasVlr>,
    #[state(artifact)]
    #[value(default)]
    pub points: Vec<crate::artifacts::las::schema::snapshot::LasPoint>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LasArtifact {
    fn default() -> Self {
        Self::from_snapshot(LasSnapshot::default())
    }
}

impl LasArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> LasSnapshot {
        LasSnapshot { schema: self.schema.clone(), header: self.header.clone(), vlrs: self.vlrs.clone(), points: self.points.clone() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: LasSnapshot) -> Self {
        Self { schema: snapshot.schema, header: snapshot.header, vlrs: snapshot.vlrs, points: snapshot.points }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: LasSnapshot) {
        self.schema = snapshot.schema;
        self.header = snapshot.header;
        self.vlrs = snapshot.vlrs;
        self.points = snapshot.points;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn las_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.las",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
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
    use crate::artifacts::las::{LasDiff, LasMutation, LasSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.las` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct LasBuilderConstruction {
        snapshot: LasSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for LasBuilderConstruction {
        type Snapshot = LasSnapshot;
        type Mutation = LasMutation;
        type Diff = LasDiff;
        fn empty() -> Self {
            Self { snapshot: LasSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<LasSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<LasSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::las::schema::mutations::apply_las_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <LasDiff as protocol::MutationDiff<LasSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::artifacts::las::LasSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.las` parts.
    #[derive(Clone, Debug, Default)]
    pub struct LasParts {
        pub snapshot: Option<LasSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.las` (1.0/✳️header) sources.
    pub struct LasAnalyzerAnalysis;

    impl ArtifactAnalysis for LasAnalyzerAnalysis {
        type Parts = LasParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG: [u8; 4] = *b"LASF";
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 4 && bytes[0..4] == SIG {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.las's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 4 bytes to sniff the real signature.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(8).collect();
                    if hex.len() < 8 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 4];
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
            let mut parts = LasParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <LasSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <LasSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec LasBuilderFacets {
        construction: LasBuilderConstruction,
        analysis: LasAnalyzerAnalysis,
        composition: super::super::io::derived_composition::LasComposerComposition,
    }
    builder: LasBuilder,
    analyzer: LasAnalyzer,
    composer: LasComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_las_snapshot() -> LasSnapshot {
    LasSnapshot::default()
}

/// 📄️ The demo `stdio.las` document — a small but genuinely representative point-data-format-0
/// snapshot (one VLR, two points), matching the companion real-format fixture assets
/// (`📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio`, both regenerated
/// from this snapshot's real `print_dsl`/`encode_pack` output). Single source of truth for those
/// fixtures, asserted equal by `conformance_laws::fixture_honesty_law`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_las_snapshot() -> LasSnapshot {
    use crate::artifacts::las::schema::snapshot::{LasHeader, LasPoint, LasVlr};
    use crate::artifacts::las::{LasSnapshot, STDIO_LAS_DOCUMENT_SCHEMA};
    LasSnapshot {
        schema: STDIO_LAS_DOCUMENT_SCHEMA.into(),
        header: LasHeader {
            version_major: 1,
            version_minor: 2,
            system_identifier: "SEMIO".into(),
            generating_software: "semio-las-engine".into(),
            creation_day_of_year: 100,
            creation_year: 2026,
            offset_to_point_data: 289, // structural: 227 (fixed header) + 54 (vlr header) + 8 (vlr data)
            number_of_vlrs: 1,
            point_data_format_id: 0,
            point_data_record_length: 20,
            number_of_point_records: 2,
            points_by_return: [2, 0, 0, 0, 0],
            x_scale: 0.01,
            y_scale: 0.01,
            z_scale: 0.01,
            x_offset: 0.0,
            y_offset: 0.0,
            z_offset: 0.0,
            max_x: 200.0,
            min_x: 100.0,
            max_y: 0.0,
            min_y: -50.0,
            max_z: 11.0,
            min_z: 10.0,
            ..LasHeader::default()
        },
        vlrs: vec![LasVlr { user_id: "LASF_Projection".into(), record_id: 34735, description: "GeoKeyDirectoryTag".into(), data: vec![1, 0, 1, 0, 0, 0, 3, 0] }],
        points: vec![
            LasPoint {
                x: 100.0,
                y: -50.0,
                z: 10.0,
                intensity: 100,
                return_number: 1,
                number_of_returns: 1,
                scan_direction_flag: false,
                edge_of_flight_line: false,
                classification: 2,
                scan_angle_rank: -5,
                user_data: 0,
                point_source_id: 1000,
                gps_time: None,
                rgb: None,
            },
            LasPoint {
                x: 101.23,
                y: -49.5,
                z: 10.01,
                intensity: 110,
                return_number: 2,
                number_of_returns: 2,
                scan_direction_flag: true,
                edge_of_flight_line: true,
                classification: 4,
                scan_angle_rank: 3,
                user_data: 1,
                point_source_id: 1001,
                gps_time: None,
                rgb: None,
            },
        ],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 📇️ `dsl::registry::register_schema_spec` is deliberately NOT called here — `LasSnapshot`/
/// `LasHeader`/`LasVlr`/`LasPoint`/`LasDiff` are all fully hand-rolled (no `#[derive(dsl::
/// DslRecord)]`/`DslDiff` anywhere in this tree). No `fn() -> RecordSpec` exists to register
/// under `"stdio.las"`/`"stdio.las#diff"` — filed as a `mechanism_gaps` entry
/// (`register-schema-spec-needs-recordspec`) rather than fabricating an unrelated spec. Kept
/// reachable as `crate::artifacts::las::engine::register_schema_specs` (the plugin root's
/// `.setup(...)` call, ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W6 g4) — `dsl::registry::
/// register_schema_spec` is a separate registry no `ArtifactDeclaration` field covers.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}
//#endregion 🔖️Register
