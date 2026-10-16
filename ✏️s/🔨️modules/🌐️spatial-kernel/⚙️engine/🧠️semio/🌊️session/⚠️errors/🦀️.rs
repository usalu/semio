//! ⚠️ The brep module error type of the media import and export bridges.

use super::*;

// #region ⚠️ Errors
/// 🧯️ Internal error type for the brep module's media import/export bridging helpers (`export_solid_json`/`import_solid_json` still surface it flattened to JSON `{"error"}` strings, matching prior behaviour byte-for-byte).
#[derive(Debug)]
pub enum BrepModuleError {
    OwnerBusy,
    Kernel(semio_framework_3d_brep_engine::BrepError),
    Codec(EvalError),
    Mesh(String),
    UnsupportedExportFormat(String),
    UnsupportedImportFormat(String),
    InvalidArgs(String),
    UnknownMethod(String),
}

impl std::fmt::Display for BrepModuleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OwnerBusy => formatter.write_str("brep kernel owner busy"),
            Self::Kernel(error) => std::fmt::Display::fmt(error, formatter),
            Self::Codec(error) => std::fmt::Display::fmt(error, formatter),
            Self::Mesh(detail) => formatter.write_str(detail),
            Self::UnsupportedExportFormat(format) => write!(formatter, "unsupported solid export format: {format}"),
            Self::UnsupportedImportFormat(format) => write!(formatter, "unsupported solid import format: {format}"),
            Self::InvalidArgs(detail) => write!(formatter, "invalid brep_invoke args: {detail}"),
            Self::UnknownMethod(method) => write!(formatter, "unknown brep_invoke method: {method}"),
        }
    }
}

impl std::error::Error for BrepModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Kernel(error) => Some(error),
            Self::Codec(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_3d_brep_engine::BrepError> for BrepModuleError {
    fn from(error: semio_framework_3d_brep_engine::BrepError) -> Self {
        Self::Kernel(error)
    }
}

impl From<EvalError> for BrepModuleError {
    fn from(error: EvalError) -> Self {
        Self::Codec(error)
    }
}
// #endregion ⚠️ Errors
