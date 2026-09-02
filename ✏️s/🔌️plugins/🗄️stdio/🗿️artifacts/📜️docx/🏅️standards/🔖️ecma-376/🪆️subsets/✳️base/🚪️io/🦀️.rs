//! 🚪️ IO stdio.docx (ecma-376/✳️any) — registration flows through `docx::declaration()`
//! (`🗄️stdio/🗿️artifacts/📜️docx/🦀️.rs`), not a side-effecting `register()`; `⚙️engine`
//! dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — its orphaned
//! `register()`/`register_artifact_inferences()`/`register_pilot_languages()` (zero callers,
//! superseded by `declaration()`) deleted outright; `DocxError` + shared OPC/XML constants below
//! (used by both `📥️import/🧩️deserializers` and `📤️export/🧵️serializers`); `io_registry` moved
//! here from `⚙️engine`, live (`docx::declaration()`'s `.composers(...)` and this artifact's own
//! root `io_registry` both reach it).
//#region 🔖️Error
/// ⚠️ Typed docx decode/encode failure — a package this engine cannot honestly interpret is
/// never fabricated into a partial/empty document.
#[derive(Clone, Debug, PartialEq)]
pub enum DocxError {
    Opc(crate::artifacts::zip::opc::OpcError),
    MissingMainDocumentRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for DocxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "docx: {e}"),
            Self::MissingMainDocumentRelationship => write!(f, "docx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "docx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "docx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "docx: {detail}"),
        }
    }
}

impl std::error::Error for DocxError {}

impl From<crate::artifacts::zip::opc::OpcError> for DocxError {
    fn from(e: crate::artifacts::zip::opc::OpcError) -> Self {
        Self::Opc(e)
    }
}
//#endregion 🔖️Error

//#region 🔖️Constants
/// 🏅️ ISO/IEC 29500-1 Strict's officeDocument relationship type (`✳️strict`'s
/// `STRICT_REL_BASE`/`officeDocument`) — decode must recognize this alongside the transitional
/// `REL_TYPE_OFFICE_DOCUMENT`, since this `✳️any`-level decoder is shared by every subset
/// including `✳️strict`, which legitimately never uses the transitional relationship type.
pub const STRICT_REL_TYPE_OFFICE_DOCUMENT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";
pub const W_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
pub const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
pub const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
pub const MAIN_DOCUMENT_PART: &str = "word/document.xml";
pub const STYLES_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml";
pub const STYLES_PART: &str = "word/styles.xml";
/// 🧭️ The styles relationship's `Target`, RELATIVE TO ITS OWNER'S DIRECTORY (`word/`) per OPC
/// §9.3 -- NOT `STYLES_PART` verbatim, which is package-root-relative and would resolve (via
/// `resolve_relationship_target("word/document.xml", "word/styles.xml")`) to the wrong path
/// `word/word/styles.xml`. This is the OPC module's own documented "#1 relative-target gotcha".
pub const STYLES_REL_TARGET: &str = "styles.xml";
pub const REL_TYPE_STYLES: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
/// 🏅️ ISO/IEC 29500-1 Strict's styles relationship type — the exact counterpart of
/// [`STRICT_REL_TYPE_OFFICE_DOCUMENT`], and needed for the same reason. A package that has been
/// stamped Strict (`✳️strict`'s `set-relationship-base`/`set-snapshot`) carries THIS type on its
/// styles relationship and never the transitional one, so a writer that recognizes only
/// [`REL_TYPE_STYLES`] concludes the package has no styles relationship and appends a second,
/// transitional-typed one beside the strict one it just failed to see — real package corruption,
/// caught by `mutate-docx-ecma-376-strict`'s differential rows the moment that case first ran a
/// subject half.
pub const STRICT_REL_TYPE_STYLES: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/styles";
//#endregion 🔖️Constants

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::docx::standards::v_ecma_376::subsets::base::schema::DocxAnalyzer;
    use crate::artifacts::docx::DocxSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_XML: Dialect = Dialect { artifact_kind: "s.stdio.xml", standard: StandardId("1.0"), subset: SubsetId("*") };

    pub struct DocxComposerComposition;

    impl ArtifactComposition for DocxComposerComposition {
        type Snapshot = DocxSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP, DEP_XML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
                return Err(ComposeError { message: "DocxComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = DocxAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "DocxComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::docx::standards::v_ecma_376::subsets::base::schema::DocxComposer as DocxRawAnyComposer;
    use crate::artifacts::docx::standards::v_ecma_376::subsets::strict::schema::DocxStrictComposer;
    use crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::schema::DocxTransitionalComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<DocxRawAnyComposer>(), composer_entry_of::<DocxStrictComposer>(), composer_entry_of::<DocxTransitionalComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
