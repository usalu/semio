//! ⚠️ Owned Pptx document and package refusals.
//#region 🔖️Error
/// ⚠️ Typed pptx decode/encode failure — a package this engine cannot honestly interpret is
/// never fabricated into a partial/empty presentation.
#[derive(Clone, Debug, PartialEq)]
pub enum PptxError {
    Opc(semio_s_artifact_stdio_zip::opc::OpcError),
    Ownership(semio_framework_value::ValueError),
    MissingPresentationRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for PptxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "pptx: {e}"),
            Self::Ownership(e) => write!(f, "pptx: {}",e.message),
            Self::MissingPresentationRelationship => write!(f, "pptx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "pptx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "pptx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "pptx: {detail}"),
        }
    }
}

impl std::error::Error for PptxError {}
/// 🪢️ Package and ownership layers keep their own kind; every document-structure refusal is invalid input.
impl From<PptxError> for semio_framework_value::ValueError {
    fn from(error: PptxError) -> Self {
        let kind = match &error { PptxError::Opc(error) => error.refusal_kind(), PptxError::Ownership(error) => error.kind, _ => semio_framework_value::ValueRefusalKind::InvalidValue };
        Self::new(kind, error.to_string())
    }
}

impl From<semio_s_artifact_stdio_zip::opc::OpcError> for PptxError {
    fn from(e: semio_s_artifact_stdio_zip::opc::OpcError) -> Self {
        Self::Opc(e)
    }
}

//#endregion 🔖️Error
impl From<semio_framework_value::ValueError> for PptxError{fn from(error:semio_framework_value::ValueError)->Self{Self::Ownership(error)}}
