//! 🧲️ Note play app commands — snap-to-grid enabled/spacing. Document-mutating.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSnapEnabled
pub mod set_snap_enabled {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-snap-enabled")]
    pub struct SetSnapEnabled {
        pub value: Option<bool>,
    }

    pub fn handle(payload: &SetSnapEnabled, doc: &DocumentView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let next = payload.value.unwrap_or(!doc.snapshot.snap_enabled.unwrap_or(false));
        Ok(Emit::mutations(vec![NoteMutation::SetSnapEnabled { enabled: Some(next) }]))
    }
}
//#endregion 🔖️SetSnapEnabled

//#region 🔖️SetSnapGridSpacing
pub mod set_snap_grid_spacing {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-snap-grid-spacing")]
    pub struct SetSnapGridSpacing {
        pub value: f64,
    }

    pub fn handle(payload: &SetSnapGridSpacing, _doc: &DocumentView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![NoteMutation::SetSnapGridSpacing { spacing: Some(payload.value.max(1.0)) }]))
    }
}
//#endregion 🔖️SetSnapGridSpacing
