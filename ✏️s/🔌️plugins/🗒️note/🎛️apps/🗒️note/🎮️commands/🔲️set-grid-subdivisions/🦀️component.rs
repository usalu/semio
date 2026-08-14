//! 🔲️ 🔲️ Note play app commands command — `set-grid-subdivisions`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::NoteSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-grid-subdivisions")]
pub struct SetGridSubdivisions {
    pub value: f64,
}

pub fn handle(payload: &SetGridSubdivisions, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::apps::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![crate::artifacts::note::schema::mutations::change_grid_subdivisions(Some(payload.value.round().clamp(1.0, 16.0)))]))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::commands::set_grid_opacity;
    use crate::apps::note::testkit::{dispatch, note_app};
    use crate::apps::note::NoteCommand;

    #[test]
    fn set_grid_subdivisions_and_opacity_clamp() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::SetGridSubdivisions(SetGridSubdivisions { value: 40.0 }));
        assert_eq!(app.snapshot().expect("snapshot").grid_subdivisions, Some(16.0));

        dispatch(&mut app, NoteCommand::SetGridOpacity(set_grid_opacity::SetGridOpacity { value: 5.0 }));
        assert_eq!(app.snapshot().expect("snapshot").grid_opacity, Some(1.0));
    }
}
//#endregion 🧪️Tests
