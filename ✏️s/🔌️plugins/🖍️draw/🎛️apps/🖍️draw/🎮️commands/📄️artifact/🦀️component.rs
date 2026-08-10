//! 📄️ Draw play app commands — whole-document load/replace vocabulary (constitutional: was `ui`'s
//! `ContentOperations` region, document-level rows).

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::apps::draw::DRAW_PLAY_EXAMPLE_DEFAULT_ID;
use crate::artifacts::draw::engine::{default_draw_document, semio_draw_example_document};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetSnapshot
pub mod set_snapshot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-snapshot")]
    pub struct SetSnapshot {
        #[dsl(block)]
        pub snapshot: DrawSnapshot,
    }

    pub fn handle(payload: &SetSnapshot, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut crate::apps::draw::commands::canvas::DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![DrawMutation::SetSnapshot { snapshot: payload.snapshot.clone() }]))
    }
}
//#endregion 🔖️SetSnapshot

//#region 🔖️CommitDocument
pub mod commit_document {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "commit-document")]
    pub struct CommitDocument {
        #[dsl(block)]
        pub snapshot: DrawSnapshot,
    }

    pub fn handle(payload: &CommitDocument, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut crate::apps::draw::commands::canvas::DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![DrawMutation::SetSnapshot { snapshot: payload.snapshot.clone() }]))
    }
}
//#endregion 🔖️CommitDocument

//#region 🔖️SetFixtureJson
pub mod set_fixture_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "fixture-json")]
    pub struct SetFixtureJson {
        pub json: String,
    }

    /// 🌡 Parsed as JSON (falling back to a no-op when it isn't valid or doesn't carry the draw schema)
    /// — mirrors every other plugin's fixture-injection command.
    pub fn handle(payload: &SetFixtureJson, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut crate::apps::draw::commands::canvas::DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        if payload.json.contains(DRAW_DOCUMENT_SCHEMA) {
            if let Ok(snapshot) = serde_json::from_str::<DrawSnapshot>(&payload.json) {
                return Ok(Emit::mutations(vec![DrawMutation::SetSnapshot { snapshot }]));
            }
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️SetFixtureJson

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut crate::apps::draw::commands::canvas::DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
        let next = if payload.example_id.is_empty() {
            Some(default_draw_document("empty", None))
        } else if payload.example_id == DRAW_PLAY_EXAMPLE_DEFAULT_ID {
            Some(semio_draw_example_document())
        } else {
            None
        };
        match next {
            Some(snapshot) => Ok(Emit {
                artifact_mutations: vec![DrawMutation::SetSnapshot { snapshot }],
                config_mutations: vec![DrawConfigMutation::SetSelection { ids: Vec::new() }],
                ..Default::default()
            }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetActiveExample
