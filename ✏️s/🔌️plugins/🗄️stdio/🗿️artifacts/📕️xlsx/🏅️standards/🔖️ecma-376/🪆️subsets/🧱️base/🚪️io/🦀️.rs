//! 🚪️ IO stdio.xlsx (ecma-376/🧱️base) — registration flows through `xlsx::declaration()`
//! (`🗄️stdio/🗿️artifacts/📕️xlsx/🦀️.rs`), not a side-effecting `register()`; `⚙️engine`
//! dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — `XlsxEngine` (zero
//! construction sites) deleted outright; its orphaned `register()`/`register_artifact_inferences()`/
//! `register_pilot_languages()` (zero callers, superseded by `declaration()`) deleted outright too;
//! `XlsxError` + shared OPC/XML constants + the `column_letter`/`column_index` pure helpers below
//! (used by both `📥️import/🧩️deserializers` and `📤️export/🧵️serializers`); `io_registry` moved
//! here from `⚙️engine`, live (`xlsx::declaration()`'s `.composers(...)` and this artifact's own
//! root `io_registry` both reach it).
//#region 🔖️Error
/// ⚠️ Typed xlsx decode/encode failure — a workbook this engine cannot honestly interpret
/// (dangling relationship, out-of-range shared-string index, non-numeric numeric cell, …) is
/// never fabricated into a partial/empty workbook.
#[derive(Clone, Debug, PartialEq)]
pub enum XlsxError {
    Opc(semio_s_artifact_stdio_zip::opc::OpcError),
    MissingWorkbookRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for XlsxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "xlsx: {e}"),
            Self::MissingWorkbookRelationship => write!(f, "xlsx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "xlsx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "xlsx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "xlsx: {detail}"),
        }
    }
}

impl std::error::Error for XlsxError {}
/// 🪢️ Package and ownership layers keep their own kind; every document-structure refusal is invalid input.
impl From<XlsxError> for semio_framework_value::ValueError {
    fn from(error: XlsxError) -> Self {
        let kind = match &error { XlsxError::Opc(error) => error.refusal_kind(), _ => semio_framework_value::ValueRefusalKind::InvalidValue };
        Self::new(kind, error.to_string())
    }
}

impl From<semio_s_artifact_stdio_zip::opc::OpcError> for XlsxError {
    fn from(e: semio_s_artifact_stdio_zip::opc::OpcError) -> Self {
        Self::Opc(e)
    }
}
//#endregion 🔖️Error

//#region 🔖️Constants
pub const SML_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
pub const SML_NS_STRICT: &str = "http://purl.oclc.org/ooxml/spreadsheetml/main";
pub const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const R_NS_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
pub const WORKBOOK_PART: &str = "xl/workbook.xml";
pub const SHARED_STRINGS_PART: &str = "xl/sharedStrings.xml";
pub const WORKBOOK_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
pub const WORKSHEET_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
pub const SHARED_STRINGS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";
pub const REL_TYPE_WORKSHEET: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
pub const REL_TYPE_SHARED_STRINGS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";
/// 🏅️ ISO/IEC 29500-1 Strict's officeDocument relationship TYPE for the package-root -> workbook
/// pointer (Strict's Annex replaces every `schemas.openxmlformats.org` relationship-type URI with
/// a `purl.oclc.org/ooxml` equivalent, not just the content markup namespaces -- ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3's `🔒️strict` subset). Recognized here
/// (decode/sniff, additively, alongside the Transitional URI above) so a genuinely Strict-shaped
/// package can be decoded at all -- without this, `decode_xlsx` would reject every real Strict
/// document with `MissingWorkbookRelationship` before the `🔒️strict` subset analyzer ever ran.
pub const REL_TYPE_OFFICE_DOCUMENT_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";
/// 🏅️ Strict's `sharedStrings` relationship TYPE, same rationale as above -- without recognizing
/// it, any Strict document using shared strings would hard-fail decode with an out-of-range
/// shared-string index (the shared-strings part would never be found).
pub const REL_TYPE_SHARED_STRINGS_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings";
/// 🏅️ Strict's `worksheet` relationship TYPE — the workbook-owned pointer that gives a part its worksheet role in a Strict
/// package, same rationale as the two Strict relationship types above.
pub const REL_TYPE_WORKSHEET_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attr(name: &str, value: &str) -> semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr {
    semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr { name: name.into(), value: value.into() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn attr_val<'a>(attrs: &'a [semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

/// 🧭️ Extends an inherited namespace scope with declarations authored on one XML element.
pub fn namespace_scope(parent: &[(String, String)], node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode) -> Vec<(String, String)> {
    let mut scope = parent.to_vec();
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = node else { return scope };
    for attribute in attrs {
        let prefix = if attribute.name == "xmlns" { Some("") } else { attribute.name.strip_prefix("xmlns:") };
        if let Some(prefix) = prefix {
            if let Some(binding) = scope.iter_mut().find(|(bound, _)| bound == prefix) {
                binding.1.clone_from(&attribute.value);
            } else {
                scope.push((prefix.into(), attribute.value.clone()));
            }
        }
    }
    scope
}

/// 🧭️ Resolves an element qualified name; the default namespace applies to unprefixed elements.
pub fn expanded_element_name(name: &str, scope: &[(String, String)]) -> Result<(String, String), String> {
    let (prefix, local) = name.split_once(':').unwrap_or(("", name));
    let namespace = if prefix.is_empty() {
        scope.iter().rev().find(|(bound, _)| bound.is_empty()).map_or("", |(_, value)| value.as_str())
    } else {
        scope.iter().rev().find(|(bound, _)| bound == prefix).map(|(_, value)| value.as_str()).ok_or_else(|| format!("unbound XML namespace prefix in element {name}"))?
    };
    Ok((namespace.into(), local.into()))
}

/// 🧭️ Resolves an attribute qualified name; the default namespace never applies to attributes.
pub fn expanded_attribute_name(name: &str, scope: &[(String, String)]) -> Result<(String, String), String> {
    let Some((prefix, local)) = name.split_once(':') else { return Ok((String::new(), name.into())) };
    let namespace =
        if prefix == "xml" { "http://www.w3.org/XML/1998/namespace" } else { scope.iter().rev().find(|(bound, _)| bound == prefix).map(|(_, value)| value.as_str()).ok_or_else(|| format!("unbound XML namespace prefix in attribute {name}"))? };
    Ok((namespace.into(), local.into()))
}

/// 🧭️ Matches an element by exact namespace URI and local name.
pub fn element_matches(node: &semio_s_artifact_stdio_xml::schema::snapshot::XmlNode, scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<bool, String> {
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name, .. } = node else { return Ok(false) };
    let (namespace, name) = expanded_element_name(name, scope)?;
    Ok(name == local && namespaces.contains(&namespace.as_str()))
}

/// 🧭️ Reads an attribute by exact namespace URI and local name.
pub fn attribute_value<'a>(node: &'a semio_s_artifact_stdio_xml::schema::snapshot::XmlNode, scope: &[(String, String)], namespaces: &[&str], local: &str) -> Result<Option<&'a str>, String> {
    let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { attrs, .. } = node else { return Ok(None) };
    for attribute in attrs {
        let (namespace, name) = expanded_attribute_name(&attribute.name, scope)?;
        if name == local && namespaces.contains(&namespace.as_str()) {
            return Ok(Some(attribute.value.as_str()));
        }
    }
    Ok(None)
}
//#endregion 🔖️Constants

//#region 🔖️ColumnLetters
/// 🔤️ 0-indexed column number -> spreadsheet column letters (`0 -> "A"`, `25 -> "Z"`, `26 -> "AA"`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn column_letter(mut index: u32) -> String {
    let mut letters = Vec::new();
    loop {
        letters.push((b'A' + (index % 26) as u8) as char);
        if index < 26 {
            break;
        }
        index = index / 26 - 1;
    }
    letters.iter().rev().collect()
}

/// 🔤️ Inverse of `column_letter`: spreadsheet column letters -> 0-indexed column number
/// (`"A" -> 0`, `"Z" -> 25`, `"AA" -> 26`). `None` on empty or non-alphabetic input.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn column_index(letters: &str) -> Option<u32> {
    if letters.is_empty() || !letters.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let mut idx: u64 = 0;
    for c in letters.chars() {
        idx = idx * 26 + (c.to_ascii_uppercase() as u64 - 'A' as u64 + 1);
    }
    Some((idx - 1) as u32)
}

/// 🔤️ Splits an A1-style cell reference (`"B2"`) into its column-letter prefix (`"B"`) — only the
/// column part is needed by the decoder, since row is already known from the enclosing `<row r>`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn column_letters_of(reference: &str) -> &str {
    reference.trim_end_matches(|c: char| c.is_ascii_digit())
}
//#endregion 🔖️ColumnLetters

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::XlsxAnalyzer;
    use crate::XlsxSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    pub struct XlsxComposerComposition;

    impl ArtifactComposition for XlsxComposerComposition {
        type Snapshot = XlsxSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_ZIP)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "XlsxComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = XlsxAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "XlsxComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_ecma_376::subsets::base::io::XlsxComposer as XlsxRawAnyComposer;
    use crate::standards::v_ecma_376::subsets::strict::io::XlsxStrictComposer;
    use crate::standards::v_ecma_376::subsets::transitional::io::XlsxTransitionalComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<XlsxRawAnyComposer>(), composer_entry_of::<XlsxStrictComposer>(), composer_entry_of::<XlsxTransitionalComposer>()]).as_slice()
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
    use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet};
    use crate::{XlsxDiff, XlsxMutation, XlsxSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.xlsx` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct XlsxBuilderConstruction {
        snapshot: XlsxSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for XlsxBuilderConstruction {
        type Snapshot = XlsxSnapshot;
        type Mutation = XlsxMutation;
        type Diff = XlsxDiff;
        fn empty() -> Self {
            Self { snapshot: XlsxSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_xlsx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <XlsxDiff as protocol::MutationDiff<XlsxSnapshot>>::apply(&diff, &self.snapshot)?;
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

    //#region 🔖️TypedConstructors
    /// 🧱️ Typed content constructors — build a workbook from sheets and rows of cell values,
    /// auto-assigning `(row, col)` coordinates left-to-right (`col` 0-based).
    impl XlsxBuilderConstruction {
        /// ➕️ Appends a new (initially empty) sheet and makes it the active sheet for `add_row`.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_sheet(mut self, name: impl Into<String>) -> Self {
            let mut workbook = self.snapshot.project_workbook().unwrap_or_default();
            workbook.sheets.push(XlsxSheet { name: name.into(), cells: Vec::new() });
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(workbook);
            self
        }

        /// ➕️ Appends a row of values to the active sheet (the most recently added one), assigning
        /// `(row: index, col: 0..)` coordinates left-to-right.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_row(mut self, index: u32, values: Vec<XlsxCellValue>) -> Self {
            let mut workbook = self.snapshot.project_workbook().unwrap_or_default();
            if let Some(sheet) = workbook.sheets.last_mut() {
                sheet.cells.extend(values.into_iter().enumerate().map(|(col, value)| XlsxCell { row: index, col: col as u32, value }));
            }
            self.snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(workbook);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::XlsxSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.xlsx` parts.
    #[derive(Clone, Debug, Default)]
    pub struct XlsxParts {
        pub snapshot: Option<XlsxSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.xlsx` (ecma-376/🧱️base) sources.
    pub struct XlsxAnalyzerAnalysis;

    impl ArtifactAnalysis for XlsxAnalyzerAnalysis {
        type Parts = XlsxParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
            // `xl/` — disambiguates from docx/pptx, which share the same zip magic and OPC shape.
            match source {
                AnalyzeSource::Binary(bytes) if crate::standards::v_ecma_376::subsets::base::io::import::deserializers::sniff_xlsx_bytes(bytes) => IoConfidence::High,
                AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => IoConfidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = XlsxParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <XlsxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec XlsxBuilderFacets {
        construction: XlsxBuilderConstruction,
        analysis: XlsxAnalyzerAnalysis,
        composition: crate::standards::v_ecma_376::subsets::base::io::derived_composition::XlsxComposerComposition,
    }
    builder: XlsxBuilder,
    analyzer: XlsxAnalyzer,
    composer: XlsxComposer,
);
