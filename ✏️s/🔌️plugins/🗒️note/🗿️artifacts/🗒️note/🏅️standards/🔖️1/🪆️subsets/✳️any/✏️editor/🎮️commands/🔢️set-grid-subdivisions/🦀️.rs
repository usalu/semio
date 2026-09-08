//! 🔲️ 🔲️ Note play app commands command — `set-grid-subdivisions`.

use crate::op::NoteMutation;
use crate::NoteSnapshot;
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-grid-subdivisions")]
pub struct SetGridSubdivisions {
    pub value: f64,
}

pub fn handle(payload: &SetGridSubdivisions, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::schema::mutations::change_grid_subdivisions(Some(payload.value.round().clamp(1.0, 16.0)))]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::note::commands::set_grid_opacity;
    use crate::editor::note::testkit::{dispatch, note_app};
    use crate::editor::note::NoteCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_grid_subdivisions_and_opacity_clamp() {
        let mut app = note_app().await;
        dispatch(&mut app, NoteCommand::SetGridSubdivisions(SetGridSubdivisions { value: 40.0 })).await;
        assert_eq!(app.snapshot().expect("snapshot").grid_subdivisions, Some(16.0));

        dispatch(&mut app, NoteCommand::SetGridOpacity(set_grid_opacity::SetGridOpacity { value: 5.0 })).await;
        assert_eq!(app.snapshot().expect("snapshot").grid_opacity, Some(1.0));
    }
}
//#endregion 🧪️Tests
