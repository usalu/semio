//! 🚪️ IO stdio.txt (utf-8/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register().
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::TxtAnalyzer;
    use crate::artifacts::txt::TxtSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

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
use crate::artifacts::txt::{TxtDiff, TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

/// 🗂️ Registers codecs and the artifact schema descriptor. One of stdio's 10 protected
/// imperative plugin-root calls (`crate::artifacts::txt::engine::register()` in
/// `🗄️stdio/🦀️component.rs`) — left callable at that exact path via a pure re-export
/// (`standards::v_utf_8::engine::register`), body unchanged.
pub fn register() {
    crate::artifacts::txt::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA));
}

/// 📇️ P2-P3: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) — real,
/// non-fabricated calls (unlike json/csv, `TxtSnapshot`/`TxtDiff` DO carry genuine derived
/// `RecordSpec` constructors: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]` emit
/// `__dsl_spec`/`__dsl_diff_spec` respectively, see `../🧬️schema/📸️snapshot` and `🔺️diff`'s own
/// doc comments). Covers both the document's own schema id and its `"<doc>#diff"` diff schema
/// id, per design ruling B-R4. `#[cfg]`-gated to match `os_dsl::registry`'s own
/// `#[cfg(not(target_arch = "wasm32"))]` (📇️registry/🦀️component.rs) -- the registry simply does
/// not exist as a compiled item on `wasm32`.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.txt", TxtSnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.txt#diff", TxtDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) — 5-role
/// `LanguageSpec` set (Document/Ops/Diff/Pack/Spr), following `note`'s exemplar pattern exactly
/// (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`), same as
/// the sibling `stdio.csv`/`stdio.json` P2 pilots.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt",
        extension: Some("txt"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::txt::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::txt::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::txt::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::txt::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::txt::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::txt::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::txt::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::diff::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.txt`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::txt::schema::txt_artifact_schema_descriptor());
}

/// 💡️ Registers `s.stdio.txt.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::inferences::txt_artifact_inference_descriptor());
}
//#endregion 🔖️Register

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::TxtComposer as TxtRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TxtRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
