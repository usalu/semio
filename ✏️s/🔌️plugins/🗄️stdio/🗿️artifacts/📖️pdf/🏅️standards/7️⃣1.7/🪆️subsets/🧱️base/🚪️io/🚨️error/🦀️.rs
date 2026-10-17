//! 🚨️ The typed engine error every PDF codec layer shares, its Value refusal mapping and the `PResult` alias.

//#region 🔖️Error
/// 🚨 Typed engine error — never silent fabrication. `Unsupported` names a feature this codec
/// refuses to guess at (e.g. an encryption revision it does not implement).
#[derive(Clone, Debug, PartialEq)]
pub enum PdfEngineError {
    NotPdf,
    Unsupported(String),
    Malformed(String),
}

impl std::fmt::Display for PdfEngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfEngineError::NotPdf => write!(f, "pdf: not a PDF file (missing %PDF- magic)"),
            PdfEngineError::Unsupported(s) => write!(f, "pdf: unsupported: {s}"),
            PdfEngineError::Malformed(s) => write!(f, "pdf: malformed: {s}"),
        }
    }
}
impl std::error::Error for PdfEngineError {}
/// 🧭️ Carries the engine's own unsupported/malformed/not-PDF authority into the canonical Value refusal kinds.
impl From<PdfEngineError> for semio_framework_value::ValueError {
    fn from(error: PdfEngineError) -> Self {
        let kind = match &error { PdfEngineError::Unsupported(_) => semio_framework_value::ValueRefusalKind::UnsupportedOwner, PdfEngineError::NotPdf | PdfEngineError::Malformed(_) => semio_framework_value::ValueRefusalKind::InvalidValue };
        Self::new(kind, error.to_string())
    }
}

pub type PResult<T> = Result<T, PdfEngineError>;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn malformed<T>(msg: impl Into<String>) -> PResult<T> {
    Err(PdfEngineError::Malformed(msg.into()))
}
//#endregion 🔖️Error
