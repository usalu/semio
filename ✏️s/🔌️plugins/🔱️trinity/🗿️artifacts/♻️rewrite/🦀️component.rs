//! ♻️ `trinity.rewrite.rule` artifact — document entities (constitutional: general).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

use crate::artifacts::jack::PropertyValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region ⚠️ Errors
/// ⚠️ Trinity rewrite-engine errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityRewriteError {
    /// 🧩️ Trinity graph fixture load/validation/mutation failure.
    #[error(transparent)]
    Graph(#[from] crate::artifacts::jack::TrinityRamError),
    /// 🧭️ VCS store/dispatch failure.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
    /// 🧬️ JSON (de)serialization failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// 🔤️ Jack query parse/execute failure (the shared `🫀️core` jack-query kernel's own API is not
    /// yet thiserror-migrated).
    #[error("{0}")]
    Jack(String),
    /// 📐️ Force-directed layout failure (`infinite_board_port_directed`'s own API is not yet
    /// thiserror-migrated).
    #[error("{0}")]
    Layout(String),
    /// 🎨️ Canvas theme merge failure (`infinite_board_port_directed`'s own API is not yet
    /// thiserror-migrated).
    #[error("{0}")]
    CanvasTheme(String),
    #[error("force layout fixture missing nodes")]
    ForceLayoutFixtureMissingNodes,
}
//#endregion ⚠️ Errors

//#region 🔖️Types
/// 📍️ Local `{x, y}` twin for a bare `(f64, f64)` tuple — the DSL engine's `DslField` binding has no
/// impl for raw Rust tuples (only named `DslRecord`/`DslScalar` types can bind), so `rule_layout`'s
/// value type is this named record instead, with `From`/`Into` conversions at this crate's own
/// remaining `(f64, f64)` call sites (tests only — no production logic reads `rule_layout` today).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LayoutPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for LayoutPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<LayoutPoint> for (f64, f64) {
    fn from(point: LayoutPoint) -> Self {
        (point.x, point.y)
    }
}

/// 📸️ Persisted rewrite snapshot — defined in `snapshot::schema`.
pub use super::snapshot::schema::RewriteSnapshot;

pub const REWRITE_RULE_SCHEMA: &str = "trinity.rewrite.rule";
//#endregion 🔖️Types

// 📜️ `RewriteSnapshot`/`RewriteRuleMutation` derive their `store::DocumentDsl`/`protocol::OpText`
// impls directly (see `#[derive(dsl::DslRecord)]` above and `#[derive(dsl::DslEnum)]` in `🔧️op`) —
// every field already binds through the `dsl::` engine with no foreign types, so no hand-written
// parser/printer or twin type is needed anywhere in this artifact (unlike `jack`'s `JackSnapshot`).

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Text × Document per owner-table (`text.♻️rewrite`).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.♻️rewrite".into(),
        name: "Trinity Rewrite Rule".into(),
        source_format: REWRITE_RULE_SCHEMA.into(),
        component_kind: "trinity".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: REWRITE_RULE_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
