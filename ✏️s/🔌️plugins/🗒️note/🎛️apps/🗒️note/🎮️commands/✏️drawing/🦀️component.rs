//! ✏️ Note play app commands — pencil width / eraser radius. Document-mutating.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetPencilWidth
pub mod set_pencil_width {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-pencil-width")]
    pub struct SetPencilWidth {
        pub value: f64,
    }

    pub fn handle(payload: &SetPencilWidth, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![NoteMutation::SetPencilWidth { width: Some(payload.value.clamp(1.0, 24.0)) }]))
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

    pub fn handle(payload: &SetEraserRadius, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![NoteMutation::SetEraserRadius { radius: Some(payload.value.clamp(4.0, 48.0)) }]))
    }
}
//#endregion 🔖️SetEraserRadius
