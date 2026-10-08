//! 🚪️ IO stdio.binary (raw/✳️any) — leaves are typed `ArtifactSerializer`/`ArtifactDeserializer`
//! impls; the 🎹️composer at this subset assembles them into its `ComposerEntry`. This facet root
//! no longer self-registers (nothing to register -- see `🎹️composer::register` at the artifact
//! level, called once from `🔌️plugin/🔧️setup`).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_raw::subsets::any::io::BinaryAnalyzer;
    use crate::BinarySnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct BinaryComposerComposition;

    impl ArtifactComposition for BinaryComposerComposition {
        type Snapshot = BinarySnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            // 🌱 Terminal format: composes from its own native text/binary representation only.
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "BinaryComposerComposition: no source in dialect stdio.binary/raw/*".into(), diagnostics: Vec::new() });
            }
            let analysis = BinaryAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "BinaryComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES) — pure `ComposerEntry` aggregation, no engine needed. NOTE: always reach this via a
/// fully-qualified path (`standards::v_raw::subsets::any::io::io_registry::entries()`) — the
/// artifact root's OWN `io_registry` (`🗿️artifacts/💾️binary/🦀️.rs`) shadows this name with
/// a DIFFERENT return type (`&'static [&'static ComposerEntry]` vs this module's
/// `&'static [ComposerEntry]`); a bare `io_registry::entries()` silently rebinds to the wrong one.
pub mod io_registry {
    use crate::standards::v_raw::subsets::any::io::BinaryComposer as BinaryRawAnyComposer;
    use semio_framework_plugin::composer_entry_of;
    use semio_framework_plugin::io::ComposerEntry;
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    /// 🎹️ Every composer entry this standard can serve.
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<BinaryRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🔖️Register
/// 🗂️ Registers codecs, the artifact schema descriptor and every composer entry imperatively, outside any plugin assembly —
/// the twin of [`crate::declaration`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    semio_framework_plugin::io::register_composer_entries(io_registry::entries()).expect("static Stdio registration must be available and conflict-free");
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.binary", standard: semio_framework_artifact_reference::StandardId("raw"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::of::<crate::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot, crate::standards::v_raw::subsets::any::schema::mutations::BinaryMutation>(crate::STDIO_BINARY_DOCUMENT_SCHEMA))
        .expect("static Stdio registration must be available and conflict-free");
}

/// 📇️ P2-P3 follow-up fix: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion
/// API) — genuinely callable here (`BinarySnapshot` derives `semio_framework_dsl_record_derive::DslRecord`, `BinaryDiff` derives
/// `dsl::DslDiff`, so both `__dsl_spec`/`__dsl_diff_spec` exist), same 2-call shape as
/// `txt::register_schema_specs` (`🔤️txt/…/🚪️io/🦀️.rs`). Per-mutation-variant specs are
/// NOT registered here, same as txt — `register_schema_spec` registers one spec under one schema id,
/// and there is no single canonical id for a Mutation enum's N independently-shaped variants; that
/// is the genuine scope boundary, not "this facet has too many specs to register any of them."
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.binary", crate::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot::__dsl_spec));
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.binary#diff", crate::standards::v_raw::subsets::any::schema::diff::BinaryDiff::__dsl_spec));
}

#[cfg(target_arch = "wasm32")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}

/// 📌️ P2-P3: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per note's/json's
/// exemplar pattern -- `stdio.binary`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`.
/// `diff`'s `protocol` slot stays `None` matching the exemplar's own shape exactly (the role
/// scheme has no dedicated "diff binary" role even though `🔺️diff/💾️binary/📡️component.protocol.
/// semio` is a real, conformance-tested file -- its binary form is exercised directly by
/// `protocol_walk_law` (`💡️inferences/🦀️.rs`), just not wired through a 6th `LanguageRole`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    use crate::standards::v_raw::subsets::any::schema;
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.binary",
        extension: Some("bin"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(crate::standards::v_raw::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_raw::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v_raw::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_raw::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.binary"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.binary.op",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Ops,
        grammar: Some(crate::standards::v_raw::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_raw::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v_raw::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_raw::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.binary.op"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.binary.diff",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Diff,
        grammar: Some(crate::standards::v_raw::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_raw::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: semio_framework_dsl::passthrough_hooks("stdio.binary.diff"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.binary.pack",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v_raw::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_raw::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.binary.pack"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.binary.spr",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v_raw::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_raw::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.binary.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.binary`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_schema() {
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v_raw::subsets::any::schema::binary_artifact_schema_descriptor()).expect("schema descriptor publication");
}

/// 💡️ Registers `s.stdio.binary.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema` above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v_raw::subsets::any::schema::inferences::binary_artifact_inference_descriptor()).expect("schema descriptor publication");
}
//#endregion 🔖️Register

//#region 🔖️IoDeclaration
/// 🚪️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot):
/// `io() -> IoDeclaration` for `standard raw / subset any` — design.md §2/§3. **Carrier law**:
/// `s.stdio.binary@raw/*` IS `CARRIER_BINARY` (`semio_framework::io_schema::CARRIER_BINARY`), so
/// its own native `Binary` `IoPayload` already equals what `io_identify`/`io_route` treat as "the
/// raw file" — zero foreign `IoEntry` rows are needed on this side. Every OTHER artifact that
/// wants to accept raw bytes registers its OWN `Deserializer<Snapshot>` with `FROM:
/// CARRIER_BINARY` on ITS side, not here (this is why the old self-referential identity
/// `ArtifactDeserializer`/`ArtifactSerializer` leaves at `🚪️io/📥️import/…/🗿️artifacts/💾️binary/…`/
/// `📤️export/…` are provably redundant under the new mechanism — kept in place this pass only
/// because deleting them requires the crate to build cleanly for verification, which it
/// currently cannot (see `📓️w2-p-report.md` `## verification`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::{BinaryMutation, BinarySnapshot};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    IoDeclaration {
        native: NativeCodecs {
            // 🧭️ Grammar/protocol registration through `LanguagePair`'s `&'static dsl::LanguageSpec`
            // slots is legal to leave `None` (the type's own doc: plain-codec subsets are a real,
            // supported shape) — deferred here, not lost: the underlying `ArtifactDsl`/
            // `ArtifactPack` codecs these would point at are unchanged and independently tested.
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<BinarySnapshot, BinaryMutation>(crate::STDIO_BINARY_DOCUMENT_SCHEMA.to_string()),
        },
        entries: &[],
    }
}
//#endregion 🔖️IoDeclaration

//#region 🧪️CarrierLaw
#[cfg(test)]
#[path = "🧪️tests/🔬️carrier-law/🦀️.rs"]
mod carrier_law;
//#endregion 🧪️CarrierLaw

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{BinaryDiff, BinaryMutation, BinarySnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.binary` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct BinaryBuilderConstruction {
        snapshot: BinarySnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for BinaryBuilderConstruction {
        type Snapshot = BinarySnapshot;
        type Mutation = BinaryMutation;
        type Diff = BinaryDiff;
        fn empty() -> Self {
            Self { snapshot: BinarySnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<BinarySnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_binary_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
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

pub mod derived_analysis {
    use crate::BinarySnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.binary` parts.
    #[derive(Clone, Debug, Default)]
    pub struct BinaryParts {
        pub snapshot: Option<BinarySnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.binary` (raw/✳️any) sources.
    pub struct BinaryAnalyzerAnalysis;

    impl ArtifactAnalysis for BinaryAnalyzerAnalysis {
        type Parts = BinaryParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            // 👃️ Any byte sequence is a valid stdio.binary payload -- terminal format, always High.
            semio_framework_plugin::io::Confidence::High
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = BinaryParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <BinarySnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <BinarySnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
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

semio_framework_plugin::derive_artifact_facets!(
    pub spec BinaryBuilderFacets {
        construction: BinaryBuilderConstruction,
        analysis: BinaryAnalyzerAnalysis,
        composition: crate::standards::v_raw::subsets::any::io::derived_composition::BinaryComposerComposition,
    }
    builder: BinaryBuilder,
    analyzer: BinaryAnalyzer,
    composer: BinaryComposer,
);
