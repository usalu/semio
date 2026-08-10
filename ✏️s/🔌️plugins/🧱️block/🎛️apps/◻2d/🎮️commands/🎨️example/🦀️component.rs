//! 🎨️ Block 2D play app commands — load a bundled example fixture, or replace the whole document from
//! externally-edited text (the DSL editor's `edit` action).

//#region 🔖️ExampleIds
pub const BLOCK2D_EXAMPLE_LEFT: &str = "hexagonal-cut-concrete-forest-left";
pub const BLOCK2D_EXAMPLE_RIGHT: &str = "hexagonal-cut-concrete-forest-right";
//#endregion 🔖️ExampleIds

pub mod set_active_example {
    use super::{BLOCK2D_EXAMPLE_LEFT, BLOCK2D_EXAMPLE_RIGHT};
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
    use crate::artifacts::block2d::op::Block2dMutation;
    use crate::artifacts::block2d::Block2dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setActiveExample")]
    pub struct SetActiveExample {
        pub id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
        let example = match payload.id.as_str() {
            BLOCK2D_EXAMPLE_LEFT => crate::artifacts::block2d::dsl::parse_dsl(crate::artifacts::block2d::dsl::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
            BLOCK2D_EXAMPLE_RIGHT => crate::artifacts::block2d::dsl::parse_dsl(crate::artifacts::block2d::dsl::BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT).ok(),
            _ => None,
        };
        match example {
            Some(document) => Ok(Emit::mutations(vec![Block2dMutation::SetSnapshot { snapshot: document }])),
            None => Ok(Emit::default()),
        }
    }
}

pub mod edit {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
    use crate::artifacts::block2d::op::Block2dMutation;
    use crate::artifacts::block2d::Block2dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "edit")]
    pub struct Edit {
        pub text: String,
    }

    pub fn handle(payload: &Edit, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
        match serde_json::from_str::<Block2dSnapshot>(&payload.text) {
            Ok(document) if &document != doc.snapshot => Ok(Emit::mutations(vec![Block2dMutation::SetSnapshot { snapshot: document }])),
            _ => Ok(Emit::default()),
        }
    }
}
