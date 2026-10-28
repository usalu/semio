//! 🚪️ IO stdio.ifc.2x3 (2x3/🧱️base) — registration flows through 🎹️composer::register /
//! `engine::register` (now `schema::register`, reached through the `engine` barrel shim —
//! ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES leaves ifc's own imperative
//! registration alone per that ticket's explicit instruction; only physically dissolved out of
//! `⚙️engine`), not per-leaf register().
//!
//! 📐️ IFC2X3 is buildingSMART Coordination View 2.0-era IFC, ISO/PAS 16739:2005 schema,
//! physically-encoded identically to `📐️step`'s AP214 (`FILE_SCHEMA(('IFC2X3'))` in place of
//! `FILE_SCHEMA(('AUTOMOTIVE_DESIGN'))`). Reuses `semio_s_artifact_stdio_contract::part21`'s tokenizer/writer
//! functions directly — PARSING-CODE reuse; what's NOT reused is `Part21Document`'s type IDENTITY
//! as this standard's snapshot type.
use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3EdmPreamble;
use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_contract::part21::{parse_part21_with_comments, write_part21_with, Part21Preamble, Part21WriteOptions};
use std::fmt::Write as _;
#[path="⚖️boundary/🦀️.rs"]
mod exchange_boundary;
pub use exchange_boundary::{validate_ifc2x3_exchange,validate_ifc2x3_exchange_with};

//#region 🔖️Codec
/// 📐️ The IFC2X3 FILE_SCHEMA name a conforming Part-21 file must declare.
pub const IFC2X3_SCHEMA_NAME: &str = "IFC2X3";

/// 📥️ Decodes IFC2X3 SPF bytes into an [`Ifc2x3Snapshot`]. Real standard-specific validation
/// beyond generic Part-21 parsing: rejects any file whose `FILE_SCHEMA` doesn't declare
/// `IFC2X3` (so this decoder never silently accepts an IFC4 or plain STEP AP214 file).
pub fn decode_ifc2x3(bytes: &[u8]) -> Result<Ifc2x3Snapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("ifc2x3: not valid utf-8: {e}"))?;
    let mut edm_preamble=Ok(None);
    let document=parse_part21_with_comments(text,&mut |start,end|{if edm_preamble.is_err(){return;}match parse_edm_preamble(&text[start..end]){Ok(Some(value))=>{if matches!(edm_preamble,Ok(Some(_))){edm_preamble=Err("ifc2x3: duplicate EDM preamble".into());}else{edm_preamble=Ok(Some(value));}},Ok(None)=>{},Err(error)=>edm_preamble=Err(error)}}).map_err(|e|format!("ifc2x3 parse: {e}"))?;
    let snapshot=Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: edm_preamble? };
    validate_ifc2x3_exchange(&snapshot)?;
    Ok(snapshot)
}

/// 📤️ Regenerates valid IFC2X3 SPF bytes from a snapshot. Losslessness is `write_part21`'s job
/// (shared with `step`/`4`); this function's only own contribution is the byte encoding.
pub fn encode_ifc2x3(snapshot: &Ifc2x3Snapshot) -> Result<Vec<u8>, String> {
    validate_ifc2x3_exchange(snapshot)?;
    let options = Part21WriteOptions { line_ending: "\r\n", blank_after_header: snapshot.edm_preamble.is_some(), blank_before_data: true, blank_before_terminator: true, space_after_instance_equals: true };
    Ok(write_part21_with(&snapshot.document, options, snapshot.edm_preamble.as_ref()).into_bytes())
}
//#endregion 🔖️Codec

//#region 🏭️EdmPreamble
fn parse_edm_preamble(text:&str)->Result<Option<Ifc2x3EdmPreamble>,String>{
    const OPEN:&str="/******************************************************************************************";const CLOSE:&str="******************************************************************************************/";
    let mut lines=text.lines();if lines.next()!=Some(OPEN){return Ok(None);}
    let labels=["* STEP Physical File produced by:","* Module:","* Creation date:","* Host:","* Database:","* Database version:","* Database creation date:","* Schema:","* Model:","* Model creation date:","* Header model:","* Header model creation date:","* EDMuser:","* EDMgroup:","* License ID and type:","* EDMstepFileFactory options:"];
    let mut values:[Option<String>;16]=std::array::from_fn(|_|None);let mut closed=false;
    for line in lines{if line==CLOSE{closed=true;continue;}for(index,label)in labels.iter().enumerate(){if let Some(value)=line.strip_prefix(*label){if values[index].is_some(){return Err("ifc2x3: duplicate EDM metadata field".into());}values[index]=Some(value.trim_start_matches(|c:char|c.is_whitespace()||c=='\u{feff}').into());break;}}}
    if values[0].is_none(){return Ok(None);}if !closed||values.iter().any(Option::is_none){return Err("ifc2x3: incomplete EDM metadata".into());}
    Ok(Some(Ifc2x3EdmPreamble{producer:values[0].take().unwrap(),module:values[1].take().unwrap(),creation_date:values[2].take().unwrap(),host:values[3].take().unwrap(),database:values[4].take().unwrap(),database_version:values[5].take().unwrap(),database_creation_date:values[6].take().unwrap(),schema:values[7].take().unwrap(),model:values[8].take().unwrap(),model_creation_date:values[9].take().unwrap(),header_model:values[10].take().unwrap(),header_model_creation_date:values[11].take().unwrap(),user:values[12].take().unwrap(),group:values[13].take().unwrap(),license:values[14].take().unwrap(),options:values[15].take().unwrap()}))
}

impl Part21Preamble for Ifc2x3EdmPreamble {
    fn write_preamble(&self, out: &mut String, line_ending: &str) {
        out.push_str("/******************************************************************************************");
        out.push_str(line_ending);
        for (label, value) in [
            ("STEP Physical File produced by:", self.producer.as_str()),
            ("Module:", self.module.as_str()),
            ("Creation date:", self.creation_date.as_str()),
            ("Host:", self.host.as_str()),
            ("Database:", self.database.as_str()),
            ("Database version:", self.database_version.as_str()),
            ("Database creation date:", self.database_creation_date.as_str()),
            ("Schema:", self.schema.as_str()),
            ("Model:", self.model.as_str()),
            ("Model creation date:", self.model_creation_date.as_str()),
            ("Header model:", self.header_model.as_str()),
            ("Header model creation date:", self.header_model_creation_date.as_str()),
            ("EDMuser:", self.user.as_str()),
            ("EDMgroup:", self.group.as_str()),
            ("License ID and type:", self.license.as_str()),
            ("EDMstepFileFactory options:", self.options.as_str()),
        ] {
            write!(out, "* {label:<31} {value}{line_ending}").expect("String write");
        }
        out.push_str("******************************************************************************************/");
        out.push_str(line_ending);
    }
}
//#endregion 🏭️EdmPreamble

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::base::io::Ifc2x3Analyzer;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Ifc2x3ComposerComposition;

    impl ArtifactComposition for Ifc2x3ComposerComposition {
        type Snapshot = Ifc2x3Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "Ifc2x3ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = Ifc2x3Analyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "Ifc2x3ComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
#[path="⚖️boundary/🧪️tests/🦀️.rs"]
mod exchange_boundary_tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::standards::v2x3::subsets::base::io::Ifc2x3Composer as Ifc2x3RawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Ifc2x3RawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry




pub mod derived_construction {
    use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
    use semio_s_artifact_stdio_ifc_2x3_base_mutations::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation;
    use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct Ifc2x3BuilderConstruction {
        snapshot: Ifc2x3Snapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Ifc2x3BuilderConstruction {
        type Snapshot = Ifc2x3Snapshot;
        type Mutation = Ifc2x3Mutation;
        type Diff = Ifc2x3Diff;
        fn empty() -> Self {
            Self { snapshot: Ifc2x3Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (next, diff) = store::os_store::replay::apply_outcome(&self.snapshot, protocol::ColdMutation::diff(&mutation, &self.snapshot));
            self.snapshot = next;
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff_cold(&diff, &self.snapshot)?;
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
    use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.ifc.2x3` parts.
    #[derive(Clone, Debug, Default)]
    pub struct Ifc2x3Parts {
        pub snapshot: Option<Ifc2x3Snapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Sniff
    /// 🔍️ Real, honest confidence probe: `High` when the text/bytes look like a Part-21 envelope AND
    /// declare `IFC2X3` in `FILE_SCHEMA`; `Medium` for a Part-21 envelope of an unknown schema (could
    /// still decode -- IFC2X3 is layered on the same generic tokenizer); `Low` otherwise.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sniff_text(body: &str) -> semio_framework_plugin::io::Confidence {
        let trimmed = body.trim_start();
        if trimmed.starts_with("ISO-10303-21") {
            if trimmed.contains("IFC2X3") {
                semio_framework_plugin::io::Confidence::High
            } else {
                semio_framework_plugin::io::Confidence::Medium
            }
        } else {
            semio_framework_plugin::io::Confidence::Low
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    pub struct Ifc2x3AnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3AnalyzerAnalysis {
        type Parts = Ifc2x3Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    sniff_text(body)
                }
                AnalyzeSource::Binary(bytes) => match std::str::from_utf8(bytes) {
                    Ok(text) => sniff_text(text),
                    Err(_) => semio_framework_plugin::io::Confidence::Low,
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Ifc2x3Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match if text.trim_start().starts_with("ISO-10303-21") {
                        crate::standards::v2x3::subsets::base::io::decode_ifc2x3(text.as_bytes()).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))
                    } else {
                        <Ifc2x3Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Ifc2x3Snapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).or_else(|_| crate::standards::v2x3::subsets::base::io::decode_ifc2x3(bytes).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))) {
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

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec Ifc2x3BuilderFacets {
        construction: Ifc2x3BuilderConstruction,
        analysis: Ifc2x3AnalyzerAnalysis,
        composition: Ifc2x3ComposerComposition,
    }
    builder: Ifc2x3Builder,
    analyzer: Ifc2x3Analyzer,
    composer: Ifc2x3Composer,
);

use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::*;
use semio_s_artifact_stdio_ifc_2x3_base_descriptor::standards::v2x3::subsets::base::schema::descriptor::ifc2x3_artifact_schema_descriptor;
//#region 🔖️Register
/// 🗂️ **Deliberately left imperative and callable** (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-
/// APP-STATE-MACHINES, per the ticket's own explicit instruction: "leave ifc's registration
/// alone" — `ArtifactDeclaration` has exactly one `.schema()`/`.document_codec()` slot and
/// cannot hold both `4`'s and `2x3`'s independent descriptors/codecs at once, see the artifact
/// root `🦀️.rs`'s own doc comment). Only physically dissolved out of `⚙️engine`; reached
/// as `crate::standards::v2x3::subsets::base::io::register()` through the `engine` barrel
/// shim, which is exactly the path `🦀️.rs`'s root `ifc::engine::register()` override calls
/// explicitly (alongside `v4::engine::register()`).
///
/// Registers this standard's schema descriptor, document codec, 5-role `LanguageSpec`s, and (via
/// each real subset's own composer) its `SubsetValidator`s. Does NOT call the artifact-level
/// `ifc::composer::register()` (that union is already invoked once from `4`'s own
/// `engine::register()`, extended by this ticket to also union `v2x3::composer::entries()` —
/// calling it a second time here would be a redundant registration, same reasoning gif's
/// `89a::engine::register` doc comment gives).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(ifc2x3_artifact_schema_descriptor()).expect("schema descriptor publication");
    register_artifact_inferences();
    register_pilot_languages();
    semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("*") }, store::bare_artifact_codec::<Ifc2x3Snapshot, mutations::Ifc2x3Mutation>(STDIO_IFC2X3_DOCUMENT_SCHEMA))
        .expect("static Stdio registration must be available and conflict-free");
}

/// 💡️ Registers `s.stdio.ifc.2x3.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to the schema descriptor registration above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(inferences::ifc2x3_artifact_inference_descriptor()).expect("schema descriptor publication");
}

/// 📌️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: 5-role
/// `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's json exemplar —
/// `stdio.ifc.2x3`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol`
/// slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no dedicated
/// "diff binary" role even though `🔺️diff/💾️binary/📡️.protocol.semio` is a real,
/// conformance-tested file — its binary form is exercised directly by `protocol_walk_law` below,
/// just not wired through a 6th `LanguageRole`), same precedent `4`'s own
/// `register_pilot_languages` established.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    #[cfg(test)] use semio_s_artifact_stdio_ifc_2x3_base_model::standards::v2x3::subsets::base::schema::{diff, snapshot};
#[cfg(test)] use semio_s_artifact_stdio_ifc_2x3_base_mutations::standards::v2x3::subsets::base::schema::mutations;
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3",
        extension: Some("ifc"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.op",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Ops,
        grammar: Some(text::mutations::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(text::mutations::COMPONENT_GRAMMAR_PATH),
        protocol: Some(binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.op"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.diff",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Diff,
        grammar: Some(text::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(text::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.diff"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.pack",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.pack"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.spr",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.spr"),
    });
}

// 📌️ `dsl::registry::register_schema_spec` is intentionally NOT called here — `Part21Value` (a
// genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
// `Ifc2x3Snapshot`/`Ifc2x3Diff` at all (same `register-schema-spec-needs-recordspec` mechanism gap
// `4`'s own `IfcSnapshot`/`IfcDiff` doc comment documents for the isomorphic shape) — filed as a
// `mechanism_gaps` entry rather than fabricating an unrelated spec.
//#endregion 🔖️Register
