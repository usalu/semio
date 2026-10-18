//! ⚠️ DAG error and port side of the DAG model: the DAG error with its display and conversions, the port side and the computation header rows. See [`super`].

//#region ⚠️ Errors
/// 🚨️ Crate-local error for DAG snapshot parsing, layout, and host-state mutation.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum DagError {
    SnapshotRootNotObject,
    SchemaMismatch,
    NodesMissing,
    UnknownAlignMode(String),
    UnknownWidget(String),
    CanvasTheme(String),
    GridFactorOutOfRange,
    /// 🌉️ Holds the formatted message rather than a codec error type directly — `pack::json`'s
    /// `JsonError` (syntax) and the value derive's `ValueError` (shape) both fold into this,
    /// matching the Display text `serde_json::Error` used to produce.
    Json(String),
}

impl std::fmt::Display for DagError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SnapshotRootNotObject => formatter.write_str("snapshot root must be object"),
            Self::SchemaMismatch => formatter.write_str("schema must be dag.hostDocument"),
            Self::NodesMissing => formatter.write_str("nodes array missing"),
            Self::CanvasTheme(message) => formatter.write_str(message),
            Self::UnknownAlignMode(mode) => write!(formatter, "unknown align mode: {mode}"),
            Self::UnknownWidget(widget) => write!(formatter, "unknown widget: {widget}"),
            Self::GridFactorOutOfRange => formatter.write_str("gridFactor must be finite and in (0, 1e6]"),
            Self::Json(error) => formatter.write_str(error),
        }
    }
}

impl std::error::Error for DagError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<semio_framework_pack_json::JsonError> for DagError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<semio_framework_value::ValueError> for DagError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::Json(error.to_string())
    }
}

//#endregion ⚠️ Errors

// #region 🔖️PortSide
/// ↔ Which side of a computation node a variadic insert targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagPortSide {
    Input,
    Output,
}

impl DagPortSide {
    /// 🔤️ The side as the `insertPort` row names it: `input` or `output`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Output => "output",
        }
    }
}

// #endregion 🔖️PortSide

/// 📛️ Reserved title row above computation IO channels.
pub const DAG_COMPUTATION_HEADER_ROWS: usize = 0;
