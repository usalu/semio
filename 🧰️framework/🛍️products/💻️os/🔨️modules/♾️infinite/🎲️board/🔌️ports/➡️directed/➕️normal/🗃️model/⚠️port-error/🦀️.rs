//! ⚠️ Port error of the board model: the normal port error with its display, error and from-JSON conversions. See [`super`].

//#region ⚠️ Errors
/// ⚠️ Errors from board host theme/catalog/layout JSON mutators and manifest validation.
#[derive(Debug)]
pub enum NormalPortError {
    Json(serde_json::Error),
    ExternalLinkPreviewJson(serde_json::Error),
    BrushSessionJson(serde_json::Error),
    DropPreviewJson(serde_json::Error),
    Theme(String),
    GridFactorOutOfRange,
    CompatNotArray,
    RowNotObject(&'static str),
    CompatSourceMissing,
    CompatTargetMissing,
    InvalidCompatSpecificity(String),
    LegacyLabelField(&'static str),
    KindCatalogsRootNotObject,
    IdMissing(&'static str),
    HandleKindColorMissing,
    InvalidHandleKindColor(String),
    NodeKindHandleKindMissing,
    NodeKindHandleAngleMissing,
    EdgeTipRowInvalid(String),
    CatalogMissingKind(&'static str, String),
    DropPreviewInvalid,
    InvalidHandleColor(String, String),
    EventCredits,
    PlacementNamingJson(serde_json::Error),
    PlacementNamingEmpty,
}

impl std::fmt::Display for NormalPortError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::ExternalLinkPreviewJson(error) => write!(formatter, "setLinkSessionJson: {error}"),
            Self::BrushSessionJson(error) => write!(formatter, "setBrushSessionJson: {error}"),
            Self::DropPreviewJson(error) => write!(formatter, "setDropPreviewJson: {error}"),
            Self::Theme(message) => formatter.write_str(message),
            Self::GridFactorOutOfRange => formatter.write_str("gridFactor must be finite and in (0, 1e6]"),
            Self::CompatNotArray => formatter.write_str("expected JSON array of compatibility objects"),
            Self::RowNotObject(row) => write!(formatter, "{row} row must be object"),
            Self::CompatSourceMissing => formatter.write_str("compat row missing string source"),
            Self::CompatTargetMissing => formatter.write_str("compat row missing string target"),
            Self::InvalidCompatSpecificity(value) => write!(formatter, "compat specificity must be general|node|edge|handle|wire|vortex, got {value:?}"),
            Self::LegacyLabelField(row) => write!(formatter, "{row} kind row must use name, not legacy label"),
            Self::KindCatalogsRootNotObject => formatter.write_str("kind catalogs root must be object"),
            Self::IdMissing(row) => write!(formatter, "{row} id missing"),
            Self::HandleKindColorMissing => formatter.write_str("handle kind color missing"),
            Self::InvalidHandleKindColor(color) => write!(formatter, "invalid handle kind color {color:?}"),
            Self::NodeKindHandleKindMissing => formatter.write_str("node kind handle handleKind missing"),
            Self::NodeKindHandleAngleMissing => formatter.write_str("node kind handle angle missing"),
            Self::EdgeTipRowInvalid(row) => write!(formatter, "edge tip row {row:?} invalid"),
            Self::CatalogMissingKind(catalog, kind) => write!(formatter, "catalog missing {catalog} kind {kind:?}"),
            Self::DropPreviewInvalid => formatter.write_str("setDropPreviewJson: preview payload missing nodeKind, screen/world point, or size"),
            Self::InvalidHandleColor(handle, color) => write!(formatter, "invalid color on handle {handle}: {color:?}"),
            Self::EventCredits => formatter.write_str("board event credits exhausted before descriptor publication"),
            Self::PlacementNamingJson(error) => write!(formatter, "setPlacementNamingJson: {error}"),
            Self::PlacementNamingEmpty => formatter.write_str("setPlacementNamingJson: brushNodePrefix and brushEdgePrefix must be non-empty"),
        }
    }
}

impl std::error::Error for NormalPortError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            Self::ExternalLinkPreviewJson(error) | Self::BrushSessionJson(error) | Self::DropPreviewJson(error) | Self::PlacementNamingJson(error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for NormalPortError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
//#endregion ⚠️ Errors
