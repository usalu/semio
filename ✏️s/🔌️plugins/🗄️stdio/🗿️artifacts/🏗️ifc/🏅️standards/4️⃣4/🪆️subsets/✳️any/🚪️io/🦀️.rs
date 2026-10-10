//! 🚪️ IO stdio.ifc (4/✳️any) — registration now flows through 🎹️composer::register
//! (called once from 🔌️plugin/🔧️setup via ⚙️engine::register), not per-leaf register(). ifc's
//! own imperative registration is left alone (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
//! STATE-MACHINES explicit instruction — `ArtifactDeclaration` has exactly one `.schema()`/
//! `.document_codec()` slot and cannot hold both `4`'s and `2x3`'s independent descriptors/codecs
//! at once); only physically dissolved out of `⚙️engine`.
//#region 🔖️Submodules
/// 🏛️ Spatial structure + placement matrices + property sets, built on the shared
/// `semio_s_artifact_stdio_contract::part21` generic graph — never persisted itself. Reused externally by
/// `🧿️semio`'s own IFC4 deserializer, so this stays reachable at the same `engine::spatial` path
/// through the `engine` barrel shim.
#[path = "🏛️spatial/🦀️.rs"]
pub mod spatial;
//#endregion 🔖️Submodules

//#region 🔖️Codec
/// 📐️ The IFC4 (ADD2 TC1) `FILE_SCHEMA` name a conforming Part-21 file must declare.
pub const IFC4_SCHEMA_NAME: &str = "IFC4";

fn declares_ifc4(document: &semio_s_artifact_stdio_contract::part21::Part21Document) -> bool {
    document.header.file_schema.iter().any(|value| value.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some(IFC4_SCHEMA_NAME))))
}

/// 📥️ Decodes IFC4 SPF bytes into their Part-21 document: standard-specific validation beyond generic Part-21 parsing, a file whose `FILE_SCHEMA` does not declare `IFC4` (an IFC2X3 or AP214 file) is refused.
pub fn decode_ifc4_document(bytes: &[u8]) -> Result<semio_s_artifact_stdio_contract::part21::Part21Document, String> {
    let text = std::str::from_utf8(bytes).map_err(|error| format!("ifc4: not valid utf-8: {error}"))?;
    let document = semio_s_artifact_stdio_contract::part21::parse_part21(text).map_err(|error| format!("ifc4 parse: {error}"))?;
    if !declares_ifc4(&document) {
        return Err(format!("ifc4: FILE_SCHEMA does not declare {IFC4_SCHEMA_NAME}"));
    }
    Ok(document)
}

/// 📤️ Regenerates valid IFC4 SPF bytes from a document: the header must declare `IFC4` and every instance id must be unique (losslessness is `write_part21`'s job, shared with `step` and `2x3`).
pub fn encode_ifc4_document(document: &semio_s_artifact_stdio_contract::part21::Part21Document) -> Result<Vec<u8>, String> {
    if !declares_ifc4(document) {
        return Err(format!("ifc4: FILE_SCHEMA does not declare {IFC4_SCHEMA_NAME}"));
    }
    let mut ids = std::collections::BTreeSet::new();
    if let Some(instance) = document.instances.iter().find(|instance| !ids.insert(instance.id)) {
        return Err(format!("ifc4: the instance #{} is written twice", instance.id));
    }
    Ok(semio_s_artifact_stdio_contract::part21::write_part21(document).into_bytes())
}
//#endregion 🔖️Codec

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v4::subsets::any::io::IfcAnalyzer;
    use crate::IfcSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct IfcComposerComposition;

    impl ArtifactComposition for IfcComposerComposition {
        type Snapshot = IfcSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "IfcComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = IfcAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "IfcComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::standards::v4::subsets::any::io::IfcComposer as IfcRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<IfcRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{IfcDiff, IfcMutation, IfcSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.ifc` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct IfcBuilderConstruction {
        snapshot: IfcSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for IfcBuilderConstruction {
        type Snapshot = IfcSnapshot;
        type Mutation = IfcMutation;
        type Diff = IfcDiff;
        fn empty() -> Self {
            Self { snapshot: IfcSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<IfcSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<IfcSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (next, diff) = store::apply_outcome(&self.snapshot, protocol::Mutation::diff(&mutation, &self.snapshot));
            self.snapshot = next;
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
    use crate::IfcSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.ifc` parts.
    #[derive(Clone, Debug, Default)]
    pub struct IfcParts {
        pub snapshot: Option<IfcSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.ifc` (4/✳️any) sources.
    pub struct IfcAnalyzerAnalysis;

    impl ArtifactAnalysis for IfcAnalyzerAnalysis {
        type Parts = IfcParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("4"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = IfcParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <IfcSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <IfcSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec IfcBuilderFacets {
        construction: IfcBuilderConstruction,
        analysis: IfcAnalyzerAnalysis,
        composition: crate::standards::v4::subsets::any::io::derived_composition::IfcComposerComposition,
    }
    builder: IfcBuilder,
    analyzer: IfcAnalyzer,
    composer: IfcComposer,
);

use crate::standards::v4::subsets::any::schema::*;
use crate::{IfcSnapshot, IfcMutation, STDIO_IFC_DOCUMENT_SCHEMA};
//#region 🔖️Register
/// 🗂️ **Deliberately left imperative and callable** (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-
/// APP-STATE-MACHINES, per the ticket's own explicit instruction: "leave ifc's registration
/// alone" — see the artifact root `🦀️.rs`'s own doc comment for why `ArtifactDeclaration`
/// structurally cannot hold both `4`'s and `2x3`'s independent descriptors/codecs at once). Only
/// physically dissolved out of `⚙️engine`; reached as `crate::standards::v4::
/// engine::register()` through the `engine` barrel shim below, which is exactly the path
/// `🦀️.rs`'s root `ifc::engine::register()` override calls explicitly (alongside `v2x3::
/// engine::register()`) — and, since the root shim's `pub use super::standards::v4::engine::*;`
/// glob otherwise re-exports this standard, also the plugin root's own `crate::
/// engine::register()` entry point before that override's `fn register()` shadows it.
///
/// Registers codecs and the artifact schema descriptor.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    crate::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("4"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::ArtifactCodec::bare::<IfcSnapshot, IfcMutation>(STDIO_IFC_DOCUMENT_SCHEMA)).expect("static Stdio registration must be available and conflict-free");
}

/// 📌️ P2-FG1: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's
/// json exemplar — `stdio.ifc`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s
/// `protocol` slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no
/// dedicated "diff binary" role even though `🔺️diff/💾️binary/📡️.protocol.semio` is a
/// real, conformance-tested file — its binary form is exercised directly by `protocol_walk_law`
/// below, just not wired through a 6th `LanguageRole`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc",
        extension: Some("ifc"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(crate::standards::v4::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v4::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.op",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Ops,
        grammar: Some(crate::standards::v4::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v4::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.op"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.diff",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Diff,
        grammar: Some(crate::standards::v4::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v4::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.diff"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.pack",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v4::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.pack"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.spr",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v4::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.spr"),
    });
}

/// 📌️ P2-FG1: `dsl::registry::register_schema_spec` is intentionally NOT called here — `IfcValue`
/// (a genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
/// `IfcSnapshot`/`IfcDiff` at all (real `cargo check` confirmed, see `🔺️diff/🦀️.rs`'s own
/// doc comment) — filed as the `register-schema-spec-needs-recordspec` mechanism gap rather than
/// fabricating an unrelated spec, per the recipe's own instruction.
/// 📌️ Registers schema leaves for `s.stdio.ifc`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_schema() {
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(ifc_artifact_schema_descriptor()).expect("schema descriptor publication");
}

/// 💡️ Registers `s.stdio.ifc.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v4::subsets::any::schema::inferences::ifc_artifact_inference_descriptor()).expect("schema descriptor publication");
}
//#endregion 🔖️Register
