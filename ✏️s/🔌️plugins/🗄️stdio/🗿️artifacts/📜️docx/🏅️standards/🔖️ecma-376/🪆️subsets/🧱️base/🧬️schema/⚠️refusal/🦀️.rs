//! ⚠️ Owned Docx document and package refusals.
//#region 🔖️Error
/// ⚠️ Typed docx decode/encode failure — a package this engine cannot honestly interpret is
/// never fabricated into a partial/empty document.
#[derive(Clone, Debug, PartialEq)]
pub enum DocxError {
    Opc(semio_s_artifact_stdio_zip::opc::OpcError),
    Ownership(semio_framework_value::ValueError),
    MissingMainDocumentRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for DocxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "docx: {e}"),
            Self::Ownership(detail) => write!(f, "docx: retained ownership: {}",detail.message),
            Self::MissingMainDocumentRelationship => write!(f, "docx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "docx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "docx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "docx: {detail}"),
        }
    }
}

impl std::error::Error for DocxError {}
/// 🪢️ Package and ownership layers keep their own kind; every document-structure refusal is invalid input.
impl From<DocxError> for semio_framework_value::ValueError {
    fn from(error: DocxError) -> Self {
        let kind = match &error { DocxError::Opc(error) => error.refusal_kind(), DocxError::Ownership(error) => error.kind, _ => semio_framework_value::ValueRefusalKind::InvalidValue };
        Self::new(kind, error.to_string())
    }
}

impl From<semio_s_artifact_stdio_zip::opc::OpcError> for DocxError {
    fn from(e: semio_s_artifact_stdio_zip::opc::OpcError) -> Self {
        Self::Opc(e)
    }
}

impl From<semio_framework_value::ValueError> for DocxError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::Ownership(error)
    }
}
impl DocxError {
    /// 🧭️ Retains actual owned causes while classifying authored package validation failures.
    pub fn into_value_error(self) -> semio_framework_value::ValueError {
        match self {
            Self::Ownership(error) => error,
            Self::Opc(error) => error.into_value_error(),
            error @ (Self::MissingMainDocumentRelationship | Self::MissingPart(_) | Self::Xml { .. } | Self::Malformed(_)) => semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()),
        }
    }
}
//#endregion 🔖️Error