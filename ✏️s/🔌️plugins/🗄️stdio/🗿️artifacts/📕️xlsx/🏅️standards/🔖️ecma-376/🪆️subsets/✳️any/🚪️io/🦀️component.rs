//! 🚪️ IO stdio.xlsx (ecma-376/✳️any) — registration flows through `xlsx::declaration()`
//! (`🗄️stdio/🗿️artifacts/📕️xlsx/🦀️component.rs`), not a side-effecting `register()`; `⚙️engine`
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
    Opc(crate::artifacts::zip::opc::OpcError),
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

impl From<crate::artifacts::zip::opc::OpcError> for XlsxError {
    fn from(e: crate::artifacts::zip::opc::OpcError) -> Self {
        Self::Opc(e)
    }
}
//#endregion 🔖️Error

//#region 🔖️Constants
pub const SML_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
pub const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
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
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3's `✳️strict` subset). Recognized here
/// (decode/sniff, additively, alongside the Transitional URI above) so a genuinely Strict-shaped
/// package can be decoded at all -- without this, `decode_xlsx` would reject every real Strict
/// document with `MissingWorkbookRelationship` before the `✳️strict` subset analyzer ever ran.
pub const REL_TYPE_OFFICE_DOCUMENT_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";
/// 🏅️ Strict's `sharedStrings` relationship TYPE, same rationale as above -- without recognizing
/// it, any Strict document using shared strings would hard-fail decode with an out-of-range
/// shared-string index (the shared-strings part would never be found).
pub const REL_TYPE_SHARED_STRINGS_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings";

pub async fn attr(name: &str, value: &str) -> crate::artifacts::xml::schema::snapshot::XmlAttr {
    crate::artifacts::xml::schema::snapshot::XmlAttr { name: name.into(), value: value.into() }
}

pub async fn attr_val<'a>(attrs: &'a [crate::artifacts::xml::schema::snapshot::XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}
//#endregion 🔖️Constants

//#region 🔖️ColumnLetters
/// 🔤️ 0-indexed column number -> spreadsheet column letters (`0 -> "A"`, `25 -> "Z"`, `26 -> "AA"`).
pub async fn column_letter(mut index: u32) -> String {
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
pub async fn column_index(letters: &str) -> Option<u32> {
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
pub async fn column_letters_of(reference: &str) -> &str {
    reference.trim_end_matches(|c: char| c.is_ascii_digit())
}
//#endregion 🔖️ColumnLetters

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::XlsxAnalyzer;
    use crate::artifacts::xlsx::XlsxSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    pub struct XlsxComposerComposition;

    impl ArtifactComposition for XlsxComposerComposition {
        type Snapshot = XlsxSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP, DEP_XML]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_ZIP || s.dialect == DEP_XML)
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
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::XlsxComposer as XlsxRawAnyComposer;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::schema::XlsxStrictComposer;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::XlsxTransitionalComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub async fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<XlsxRawAnyComposer>(), composer_entry_of::<XlsxStrictComposer>(), composer_entry_of::<XlsxTransitionalComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
