//! ✏️ Note play app commands — pencil width / eraser radius. Document-mutating.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetPencilWidth
pub mod set_pencil_width {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-pencil-width")]
    pub struct SetPencilWidth {
        pub value: f64,
    }

    pub fn handle(payload: &SetPencilWidth, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_pencil_width(Some(payload.value.clamp(1.0, 24.0)))]))
    }
}
//#endregion 🔖️SetPencilWidth

//#region 🔖️SetEraserRadius
pub mod set_eraser_radius {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-eraser-radius")]
    pub struct SetEraserRadius {
        pub value: f64,
    }

    pub fn handle(payload: &SetEraserRadius, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_eraser_radius(Some(payload.value.clamp(4.0, 48.0)))]))
    }
}
//#endregion 🔖️SetEraserRadius
