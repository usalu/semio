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
use crate::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3EdmPreamble, Ifc2x3Snapshot, STDIO_IFC2X3_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_contract::part21::{parse_part21, write_part21_with, Part21Preamble, Part21WriteOptions};
use std::fmt::Write as _;

//#region 🔖️Codec
/// 📐️ The IFC2X3 FILE_SCHEMA name a conforming Part-21 file must declare.
pub const IFC2X3_SCHEMA_NAME: &str = "IFC2X3";

/// 📥️ Decodes IFC2X3 SPF bytes into an [`Ifc2x3Snapshot`]. Real standard-specific validation
/// beyond generic Part-21 parsing: rejects any file whose `FILE_SCHEMA` doesn't declare
/// `IFC2X3` (so this decoder never silently accepts an IFC4 or plain STEP AP214 file).
pub fn decode_ifc2x3(bytes: &[u8]) -> Result<Ifc2x3Snapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("ifc2x3: not valid utf-8: {e}"))?;
    let document = parse_part21(text).map_err(|e| format!("ifc2x3 parse: {e}"))?;
    let declares_ifc2x3 = document.header.file_schema.iter().any(|v| v.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some(IFC2X3_SCHEMA_NAME))));
    if !declares_ifc2x3 {
        return Err(format!("ifc2x3: FILE_SCHEMA does not declare {IFC2X3_SCHEMA_NAME}"));
    }
    Ok(Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: parse_edm_preamble(text) })
}

/// 📤️ Regenerates valid IFC2X3 SPF bytes from a snapshot. Losslessness is `write_part21`'s job
/// (shared with `step`/`4`); this function's only own contribution is the byte encoding.
pub fn encode_ifc2x3(snapshot: &Ifc2x3Snapshot) -> Result<Vec<u8>, String> {
    crate::standards::v2x3::subsets::base::schema::snapshot::validate_ifc2x3_snapshot(snapshot)?;
    let options = Part21WriteOptions { line_ending: "\r\n", blank_after_header: snapshot.edm_preamble.is_some(), blank_before_data: true, blank_before_terminator: true, space_after_instance_equals: true };
    Ok(write_part21_with(&snapshot.document, options, snapshot.edm_preamble.as_ref()).into_bytes())
}
//#endregion 🔖️Codec

//#region 🏭️EdmPreamble
fn parse_edm_preamble(text: &str) -> Option<Ifc2x3EdmPreamble> {
    let lines = text.lines().map(|line| line.trim_end_matches('\r')).collect::<Vec<_>>();
    let start = lines.iter().position(|line| *line == "/******************************************************************************************")?;
    let end = lines[start + 1..].iter().position(|line| *line == "******************************************************************************************/")? + start + 1;
    let value = |label: &str| {
        let prefix = format!("* {label}");
        lines[start + 1..end].iter().find_map(|line| line.strip_prefix(&prefix).map(str::trim_start)).map(str::to_string)
    };
    Some(Ifc2x3EdmPreamble {
        producer: value("STEP Physical File produced by:")?,
        module: value("Module:")?,
        creation_date: value("Creation date:")?,
        host: value("Host:")?,
        database: value("Database:")?,
        database_version: value("Database version:")?,
        database_creation_date: value("Database creation date:")?,
        schema: value("Schema:")?,
        model: value("Model:")?,
        model_creation_date: value("Model creation date:")?,
        header_model: value("Header model:")?,
        header_model_creation_date: value("Header model creation date:")?,
        user: value("EDMuser:")?,
        group: value("EDMgroup:")?,
        license: value("License ID and type:")?,
        options: value("EDMstepFileFactory options:")?,
    })
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
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use crate::standards::v2x3::subsets::base::io::Ifc2x3Analyzer;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::standards::v2x3::subsets::base::io::Ifc2x3Composer as Ifc2x3RawAnyComposer;
    use crate::standards::v2x3::subsets::cobie::io::Ifc2x3CobieComposer;
    use crate::standards::v2x3::subsets::cv20::io::Ifc2x3Cv20Composer;
    use crate::standards::v2x3::subsets::sav::io::Ifc2x3SavComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Ifc2x3RawAnyComposer>(), composer_entry_of::<Ifc2x3Cv20Composer>(), composer_entry_of::<Ifc2x3SavComposer>(), composer_entry_of::<Ifc2x3CobieComposer>()]).as_slice()
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
    use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;
    use crate::standards::v2x3::subsets::base::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
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
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_ifc2x3_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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
    fn sniff_text(body: &str) -> IoConfidence {
        let trimmed = body.trim_start();
        if trimmed.starts_with("ISO-10303-21") {
            if trimmed.contains("IFC2X3") {
                IoConfidence::High
            } else {
                IoConfidence::Medium
            }
        } else {
            IoConfidence::Low
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    pub struct Ifc2x3AnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3AnalyzerAnalysis {
        type Parts = Ifc2x3Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
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
                    Err(_) => IoConfidence::Low,
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Ifc2x3Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match if text.trim_start().starts_with("ISO-10303-21") {
                        crate::standards::v2x3::engine::decode_ifc2x3(text.as_bytes()).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))
                    } else {
                        <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| crate::standards::v2x3::engine::decode_ifc2x3(bytes).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))) {
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec Ifc2x3BuilderFacets {
        construction: Ifc2x3BuilderConstruction,
        analysis: Ifc2x3AnalyzerAnalysis,
        composition: crate::standards::v2x3::subsets::base::io::derived_composition::Ifc2x3ComposerComposition,
    }
    builder: Ifc2x3Builder,
    analyzer: Ifc2x3Analyzer,
    composer: Ifc2x3Composer,
);
