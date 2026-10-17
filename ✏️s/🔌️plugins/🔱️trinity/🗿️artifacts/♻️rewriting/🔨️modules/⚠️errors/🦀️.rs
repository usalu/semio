//! ⚠️ Rewriting errors: the `TrinityRewritingError` of the rewriting engine with its `Display`, `Error` and `From` conversions.

//#region ⚠️ Errors
/// ⚠️ Trinity rewriting-engine errors.
#[derive(Debug)]
pub enum TrinityRewritingError {
    /// 🧩️ Trinity graph snapshot load/validation/mutation failure.
    Graph(semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::errors::TrinityRamError),
    /// 🧭️ VCS store/dispatch failure.
    Vcs(vcs::VcsError),
    /// 🧬️ JSON (de)serialization failure.
    Json(semio_framework_value::ValueError),
    /// 🔤️ Jack query parse/execute failure (the shared `🫀️core` jack-query kernel's own API is not
    /// yet expressed as an owned error type).
    Jack(String),
    /// 🎨️ Canvas theme merge failure (`infinite_board_port_directed`'s own API is not yet
    /// expressed as an owned error type).
    CanvasTheme(String),
}

impl std::fmt::Display for TrinityRewritingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Graph(error) => write!(formatter, "{error}"),
            Self::Vcs(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Jack(message) | Self::CanvasTheme(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for TrinityRewritingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Graph(error) => std::error::Error::source(error),
            Self::Vcs(error) => std::error::Error::source(error),
            Self::Json(error) => std::error::Error::source(error),
            Self::Jack(_) | Self::CanvasTheme(_) => None,
        }
    }
}

impl From<semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::errors::TrinityRamError> for TrinityRewritingError {
    fn from(error: semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::errors::TrinityRamError) -> Self {
        Self::Graph(error)
    }
}

impl From<vcs::VcsError> for TrinityRewritingError {
    fn from(error: vcs::VcsError) -> Self {
        Self::Vcs(error)
    }
}

impl From<semio_framework_value::ValueError> for TrinityRewritingError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::Json(error)
    }
}
//#endregion ⚠️ Errors
