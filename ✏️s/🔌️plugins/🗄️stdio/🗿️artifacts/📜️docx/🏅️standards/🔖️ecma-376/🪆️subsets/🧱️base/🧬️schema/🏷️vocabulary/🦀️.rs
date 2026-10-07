//! 🏷️ Docx namespaces, relationships and logical XML names.
//#region 🔖️Constants
/// 🏅️ ISO/IEC 29500-1 Strict's officeDocument relationship type (`📏️strict`'s
/// `STRICT_REL_BASE`/`officeDocument`) — decode must recognize this alongside the transitional
/// `REL_TYPE_OFFICE_DOCUMENT`, since this `✳️any`-level decoder is shared by every subset
/// including `📏️strict`, which legitimately never uses the transitional relationship type.
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
/// stamped Strict (`📏️strict`'s `set-relationship-base`/`set-snapshot`) carries THIS type on its
/// styles relationship and never the transitional one, so a writer that recognizes only
/// [`REL_TYPE_STYLES`] concludes the package has no styles relationship and appends a second,
/// transitional-typed one beside the strict one it just failed to see — real package corruption,
/// caught by `📏️mutate-docx-ecma-376-strict`'s differential rows the moment that case first ran a
/// subject half.
pub const STRICT_REL_TYPE_STYLES: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/styles";
