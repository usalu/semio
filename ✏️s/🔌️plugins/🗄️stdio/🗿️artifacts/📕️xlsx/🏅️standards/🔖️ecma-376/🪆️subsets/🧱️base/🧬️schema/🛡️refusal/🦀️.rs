//! 🛡️ SpreadsheetML structural and physical refusal vocabulary.

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

