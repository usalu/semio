//! 🧬️ ZipArtifact schema — full artifact state.

use crate::artifacts::zip::schema::snapshot::ZipEntry;
use crate::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;

//#region Artifact
/// 🧬️ Full `stdio.zip` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip")]
pub struct ZipArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub entries: Vec<ZipEntry>,
    #[state(artifact)]
    #[value(default)]
    pub comment: String,
}
//#endregion Artifact

//#region Conversions
impl Default for ZipArtifact {
    fn default() -> Self {
        Self::from_snapshot(ZipSnapshot::default())
    }
}

impl ZipArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> ZipSnapshot {
        ZipSnapshot { schema: self.schema.clone(), entries: self.entries.clone(), comment: self.comment.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: ZipSnapshot) -> Self {
        Self { schema: snapshot.schema, entries: snapshot.entries, comment: snapshot.comment }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: ZipSnapshot) {
        self.schema = snapshot.schema;
        self.entries = snapshot.entries;
        self.comment = snapshot.comment;
    }
}
//#endregion Conversions

//#region 🔖️DocumentHelpers
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — mirrors `png`'s own `empty_png_snapshot`/`demo_png_snapshot` placement beside the
/// artifact struct.
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_zip_snapshot() -> ZipSnapshot {
    ZipSnapshot::default()
}

/// 📦️ Demo logical ZIP document with two decompressed semantic members and an archive comment.
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law` in
/// `💡️inferences/🦀️.rs`) and for this artifact's own `protocol_walk_law` (walked against
/// the REAL `📸️snapshot/💾️binary/📡️.protocol.semio` — needs at least one central-directory
/// entry for the `repeat`/`backward`/`jump` construct to have real bytes to walk).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_zip_snapshot() -> ZipSnapshot {
    ZipSnapshot {
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: vec![ZipEntry { name: "readme.txt".into(), data: b"hello from stdio.zip".to_vec() }, ZipEntry { name: "data/poem.txt".into(), data: b"deflate this small poem, it should compress reasonably well well well".to_vec() }],
        comment: "demo archive comment".into(),
    }
}
//#endregion 🔖️DocumentHelpers

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.zip`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn zip_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.zip",
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
//#endregion Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::zip::schema::snapshot::ZipEntry;
    use crate::artifacts::zip::{ZipDiff, ZipMutation, ZipSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.zip` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct ZipBuilderConstruction {
        snapshot: ZipSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ZipBuilderConstruction {
        type Snapshot = ZipSnapshot;
        type Mutation = ZipMutation;
        type Diff = ZipDiff;
        fn empty() -> Self {
            Self { snapshot: ZipSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::zip::schema::mutations::apply_zip_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <ZipDiff as protocol::MutationDiff<ZipSnapshot>>::apply(&diff, &self.snapshot)?;
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

    //#region 🔖️TypedConstructors
    impl ZipBuilderConstruction {
        /// ➕️ Adds a logical member; native compression is deterministic serializer policy.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_stored_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            self.snapshot.entries.push(ZipEntry { name: name.into(), data });
            self
        }

        /// ➕️ Adds a logical member; native compression is deterministic serializer policy.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_deflate_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            self.snapshot.entries.push(ZipEntry { name: name.into(), data });
            self
        }

        /// ➕️ Adds a fully-specified member (metadata-faithful construction path).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_entry(mut self, entry: ZipEntry) -> Self {
            self.snapshot.entries.push(entry);
            self
        }

        /// 💬️ Sets the archive-level (EOCD) comment.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
            self.snapshot.comment = comment.into();
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::zip::ZipSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.zip` parts.
    #[derive(Clone, Debug, Default)]
    pub struct ZipParts {
        pub snapshot: Option<ZipSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.zip` (2.0/✳️base) sources.
    pub struct ZipAnalyzerAnalysis;

    impl ArtifactAnalysis for ZipAnalyzerAnalysis {
        type Parts = ZipParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: inspects the argument's bytes (magic + a well-formed EOCD), never a
            // constant. `AnalyzeSource::Text` is the hex-envelope DSL form, not raw container bytes,
            // so it can't be magic-sniffed the same way — treated as low confidence here (the DSL
            // envelope preamble, not this sniff, is what actually recognizes it).
            use crate::artifacts::zip::standards::v2_0::subsets::any::io::{sniff_zip_bytes, SniffConfidence};
            match source {
                AnalyzeSource::Binary(bytes) => match sniff_zip_bytes(bytes) {
                    SniffConfidence::High => IoConfidence::High,
                    SniffConfidence::Medium => IoConfidence::Medium,
                    SniffConfidence::Low => IoConfidence::Low,
                },
                AnalyzeSource::Text(_) => IoConfidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ZipParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ZipSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => {
                        let result = if matches!(crate::artifacts::zip::standards::v2_0::subsets::any::io::sniff_zip_bytes(bytes), crate::artifacts::zip::standards::v2_0::subsets::any::io::SniffConfidence::High) {
                            crate::artifacts::zip::standards::v2_0::subsets::any::io::decode_zip(bytes).map_err(|err| err.to_string())
                        } else {
                            <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|err| err.to_string())
                        };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = IoConfidence::Low;
                                diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err));
                            }
                        }
                    }
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
    pub spec ZipBuilderFacets {
        construction: ZipBuilderConstruction,
        analysis: ZipAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ZipComposerComposition,
    }
    builder: ZipBuilder,
    analyzer: ZipAnalyzer,
    composer: ZipComposer,
);
//#endregion 🧬️DerivedArtifactFacets
