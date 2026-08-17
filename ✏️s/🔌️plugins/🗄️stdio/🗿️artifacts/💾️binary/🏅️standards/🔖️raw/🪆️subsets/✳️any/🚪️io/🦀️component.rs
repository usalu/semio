//! 🚪️ IO stdio.binary (raw/✳️any) — leaves are typed `ArtifactSerializer`/`ArtifactDeserializer`
//! impls; the 🎹️composer at this subset assembles them into its `ComposerEntry`. This facet root
//! no longer self-registers (nothing to register -- see `🎹️composer::register` at the artifact
//! level, called once from `🔌️plugin/🔧️setup`).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::binary::standards::v_raw::subsets::any::schema::BinaryAnalyzer;
    use crate::artifacts::binary::BinarySnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

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
/// artifact root's OWN `io_registry` (`🗿️artifacts/💾️binary/🦀️component.rs`) shadows this name with
/// a DIFFERENT return type (`&'static [&'static ComposerEntry]` vs this module's
/// `&'static [ComposerEntry]`); a bare `io_registry::entries()` silently rebinds to the wrong one.
pub mod io_registry {
    use crate::artifacts::binary::standards::v_raw::subsets::any::schema::BinaryComposer as BinaryRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    /// 🎹️ Every composer entry this standard can serve.
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<BinaryRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🔖️Register
/// 🗂️ Registers codecs, the artifact schema descriptor, and every composer entry — dissolved out
/// of the former `⚙️engine::register()` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-
/// MACHINES). `binary` is one of stdio's 10 deliberate imperative-`register()` artifacts (never
/// converted to the `ArtifactDeclaration` builder pattern, per `crate::plugin()`'s own call —
/// unchanged in call order/behavior, only the function's file moved with the deleted directory);
/// left reachable at its old `crate::artifacts::binary::engine::register()` path via a pure
/// re-export shim in `📦️glue.rs`.
pub fn register() {
    let _ = semio_framework_plugin::register_composer_entries(io_registry::entries());
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    let _ = store::register_document_codec(store::ArtifactCodec::of::<crate::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot, crate::artifacts::binary::standards::v_raw::subsets::any::schema::mutations::BinaryMutation>(
        crate::artifacts::binary::STDIO_BINARY_DOCUMENT_SCHEMA,
    ));
}

/// 📇️ P2-P3 follow-up fix: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion
/// API) — genuinely callable here (`BinarySnapshot` derives `dsl::DslRecord`, `BinaryDiff` derives
/// `dsl::DslDiff`, so both `__dsl_spec`/`__dsl_diff_spec` exist), same 2-call shape as
/// `txt::register_schema_specs` (`📄txt/…/⚙️engine/🦀️component.rs`). Per-mutation-variant specs are
/// NOT registered here, same as txt — `register_schema_spec` registers one spec under one schema id,
/// and there is no single canonical id for a Mutation enum's N independently-shaped variants; that
/// is the genuine scope boundary, not "this facet has too many specs to register any of them."
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.binary", crate::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.binary#diff", crate::artifacts::binary::standards::v_raw::subsets::any::schema::diff::BinaryDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}

/// 📌️ P2-P3: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per note's/json's
/// exemplar pattern -- `stdio.binary`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`.
/// `diff`'s `protocol` slot stays `None` matching the exemplar's own shape exactly (the role
/// scheme has no dedicated "diff binary" role even though `🔺️diff/💾️binary/📡️component.protocol.
/// semio` is a real, conformance-tested file -- its binary form is exercised directly by
/// `protocol_walk_law` (`💡️inferences/🦀️component.rs`), just not wired through a 6th `LanguageRole`).
pub fn register_pilot_languages() {
    use crate::artifacts::binary::standards::v_raw::subsets::any::schema;
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.binary.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.binary`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::binary::standards::v_raw::subsets::any::schema::binary_artifact_schema_descriptor());
}

/// 💡️ Registers `s.stdio.binary.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema` above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING P2/S3+S4).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::binary::standards::v_raw::subsets::any::schema::inferences::binary_artifact_inference_descriptor());
}
//#endregion 🔖️Register
