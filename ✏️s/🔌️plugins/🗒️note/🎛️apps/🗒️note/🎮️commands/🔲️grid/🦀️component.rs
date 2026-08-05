//! 🔲️ Note play app commands — grid visibility/spacing/subdivisions/opacity. Document-mutating.

use crate::apps::note::config::{NoteConfig, NoteConfigOperation};
use crate::artifacts::note::op::NoteOperation;
use crate::artifacts::note::NoteDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetGridVisible
pub mod set_grid_visible {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-grid-visible")]
    pub struct SetGridVisible {
        pub value: Option<bool>,
    }

    pub fn handle(payload: &SetGridVisible, doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        let next = payload.value.unwrap_or(!doc.projection.grid_visible.unwrap_or(true));
        Ok(Emit::operations(vec![NoteOperation::SetGridVisible { visible: Some(next) }]))
    }
}
//#endregion 🔖️SetGridVisible

//#region 🔖️SetGridSpacing
pub mod set_grid_spacing {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-grid-spacing")]
    pub struct SetGridSpacing {
        pub value: f64,
    }

    pub fn handle(payload: &SetGridSpacing, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        Ok(Emit::operations(vec![NoteOperation::SetGridSpacing { spacing: Some(payload.value.max(4.0)) }]))
    }
}
//#endregion 🔖️SetGridSpacing

//#region 🔖️SetGridSubdivisions
pub mod set_grid_subdivisions {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-grid-subdivisions")]
    pub struct SetGridSubdivisions {
        pub value: f64,
    }

    pub fn handle(payload: &SetGridSubdivisions, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        Ok(Emit::operations(vec![NoteOperation::SetGridSubdivisions { value: Some(payload.value.round().clamp(1.0, 16.0)) }]))
    }
}
//#endregion 🔖️SetGridSubdivisions

//#region 🔖️SetGridOpacity
pub mod set_grid_opacity {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-grid-opacity")]
    pub struct SetGridOpacity {
        pub value: f64,
    }

    pub fn handle(payload: &SetGridOpacity, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        Ok(Emit::operations(vec![NoteOperation::SetGridOpacity { opacity: Some(payload.value.clamp(0.05, 1.0)) }]))
    }
}
//#endregion 🔖️SetGridOpacity

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app};
    use crate::apps::note::NoteCommand;

    #[test]
    fn set_grid_subdivisions_and_opacity_clamp() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::SetGridSubdivisions(set_grid_subdivisions::SetGridSubdivisions { value: 40.0 }));
        assert_eq!(app.projection().expect("projection").grid_subdivisions, Some(16.0));

        dispatch(&mut app, NoteCommand::SetGridOpacity(set_grid_opacity::SetGridOpacity { value: 5.0 }));
        assert_eq!(app.projection().expect("projection").grid_opacity, Some(1.0));
    }
}
//#endregion 🧪️Tests
