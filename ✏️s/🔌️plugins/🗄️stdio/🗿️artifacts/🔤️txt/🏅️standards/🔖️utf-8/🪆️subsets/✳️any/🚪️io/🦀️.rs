//! 🚪️ IO stdio.txt (utf-8/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_utf_8::subsets::any::io::TxtAnalyzer;
    use crate::TxtSnapshot;
    use {semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct TxtComposerComposition;

    impl ArtifactComposition for TxtComposerComposition {
        type Snapshot = TxtSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "TxtComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = TxtAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "TxtComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Register
#[cfg(not(target_arch = "wasm32"))]
use crate::TxtDiff;
use crate::{TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

/// 🗂️ Registers codecs and the artifact schema descriptor. One of stdio's 10 protected
/// imperative plugin-root calls (`crate::engine::register()` in
/// `🗄️stdio/🦀️.rs`) — left callable at that exact path via a pure re-export
/// (`standards::v_utf_8::engine::register`), body unchanged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    crate::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    semio_framework_plugin::io::register_native_snapshot_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.txt", standard: semio_framework_artifact_reference::StandardId("utf-8"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::of::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA)).expect("static Stdio registration must be available and conflict-free");
}

/// 📇️ P2-P3: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) — real,
/// non-fabricated calls (unlike json/csv, `TxtSnapshot`/`TxtDiff` DO carry genuine derived
/// `RecordSpec` constructors: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]` emit
/// `__dsl_spec`/`__dsl_diff_spec` respectively, see `../🧬️schema/📸️snapshot` and `🔺️diff`'s own
/// doc comments). Covers both the document's own schema id and its `"<doc>#diff"` diff schema
/// id, per design ruling B-R4. `#[cfg]`-gated to match `os_dsl::registry`'s own
/// `#[cfg(not(target_arch = "wasm32"))]` (📇️registry/🦀️.rs) -- the registry simply does
/// not exist as a compiled item on `wasm32`.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.txt", TxtSnapshot::__dsl_spec));
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.txt#diff", TxtDiff::__dsl_spec));
}

#[cfg(target_arch = "wasm32")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) — 5-role
/// `LanguageSpec` set (Document/Ops/Diff/Pack/Spr), following `note`'s exemplar pattern exactly
/// (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/⚙️engine/🦀️.rs`), same as
/// the sibling `stdio.csv`/`stdio.json` P2 pilots.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.txt",
        extension: Some("txt"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(crate::standards::v_utf_8::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_utf_8::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v_utf_8::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_utf_8::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.txt"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.txt.op",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Ops,
        grammar: Some(crate::standards::v_utf_8::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_utf_8::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v_utf_8::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_utf_8::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.txt.op"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.txt.diff",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Diff,
        grammar: Some(crate::standards::v_utf_8::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v_utf_8::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v_utf_8::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_utf_8::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.txt.diff"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.txt.pack",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v_utf_8::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_utf_8::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.txt.pack"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.txt.spr",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v_utf_8::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v_utf_8::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.txt.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.txt`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_schema() {
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::schema::txt_artifact_schema_descriptor()).expect("schema descriptor publication");
}

/// 💡️ Registers `s.stdio.txt.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v_utf_8::subsets::any::schema::inferences::txt_artifact_inference_descriptor()).expect("schema descriptor publication");
}
//#endregion 🔖️Register

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_utf_8::subsets::any::io::TxtComposer as TxtRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TxtRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🔖️IoDeclaration
/// 🚪️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot):
/// `io() -> IoDeclaration` for `standard utf-8 / subset any` — design.md §2/§3. **Carrier law**:
/// `s.stdio.txt@utf-8/*` IS `CARRIER_TEXT` (`semio_framework::io_schema::CARRIER_TEXT`), so its
/// own native `Text` `IoPayload` already equals what `io_identify`/`io_route` treat as "the raw
/// file" — zero foreign `IoEntry` rows are needed on this side (mirrors `💾️binary`'s `io()`; see
/// that file's doc comment for the full reasoning, incl. why the old self-referential identity
/// leaves + `derived_composition`'s binary-dependency read stay in place this pass).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA),
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
    use crate::{TxtDiff, TxtMutation, TxtSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.txt` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct TxtBuilderConstruction {
        snapshot: TxtSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for TxtBuilderConstruction {
        type Snapshot = TxtSnapshot;
        type Mutation = TxtMutation;
        type Diff = TxtDiff;
        fn empty() -> Self {
            Self { snapshot: TxtSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<TxtSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_txt_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <TxtDiff as protocol::MutationDiff<TxtSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::TxtSnapshot;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.txt` parts.
    #[derive(Clone, Debug, Default)]
    pub struct TxtParts {
        pub snapshot: Option<TxtSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.txt` (utf-8/✳️any) sources.
    pub struct TxtAnalyzerAnalysis;

    /// 🔍 `stdio.txt` accepts anything that is real, valid UTF-8 — a `Text` source is
    /// trivially valid by construction (`High`); a `Binary` source is inspected for actual
    /// UTF-8 validity and the presence of NUL bytes (the standard "probably not text"
    /// signal binary sniffers use).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn classify_bytes(bytes: &[u8]) -> IoConfidence {
        match std::str::from_utf8(bytes) {
            Ok(_) if !bytes.contains(&0) => IoConfidence::High,
            Ok(_) => IoConfidence::Medium,
            Err(_) => IoConfidence::Low,
        }
    }

    impl ArtifactAnalysis for TxtAnalyzerAnalysis {
        type Parts = TxtParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(_) => IoConfidence::High,
                AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                    Ok((_, inner)) => classify_bytes(&inner),
                    Err(_) => classify_bytes(bytes),
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = TxtParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <TxtSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <TxtSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

    //#region 🧪️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec TxtBuilderFacets {
        construction: TxtBuilderConstruction,
        analysis: TxtAnalyzerAnalysis,
        composition: crate::standards::v_utf_8::subsets::any::io::derived_composition::TxtComposerComposition,
    }
    builder: TxtBuilder,
    analyzer: TxtAnalyzer,
    composer: TxtComposer,
);
