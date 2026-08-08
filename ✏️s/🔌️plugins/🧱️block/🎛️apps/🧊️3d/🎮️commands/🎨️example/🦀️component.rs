//! 🎨️ Block 3D play app commands — load a bundled example fixture, or replace the whole document from
//! externally-edited text (the DSL editor's `edit` action).

//#region 🔖️ExampleIds
pub const BLOCK3D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
pub const BLOCK3D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
//#endregion 🔖️ExampleIds

pub mod set_active_example {
    use super::{BLOCK3D_EXAMPLE_CAPSULE, BLOCK3D_EXAMPLE_FOREST_LEFT};
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setActiveExample")]
    pub struct SetActiveExample {
        pub id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        let example = match payload.id.as_str() {
            BLOCK3D_EXAMPLE_CAPSULE => crate::artifacts::block3d::dsl::parse_dsl(crate::artifacts::block3d::dsl::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
            BLOCK3D_EXAMPLE_FOREST_LEFT => crate::artifacts::block3d::dsl::parse_dsl(crate::artifacts::block3d::dsl::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
            _ => None,
        };
        match example {
            Some(document) => Ok(Emit::mutations(vec![Block3dMutation::SetDocument { document }])),
            None => Ok(Emit::default()),
        }
    }
}

pub mod edit {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "edit")]
    pub struct Edit {
        pub text: String,
    }

    pub fn handle(payload: &Edit, doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        match serde_json::from_str::<Block3dDefinition>(&payload.text) {
            Ok(document) if &document != doc.projection => Ok(Emit::mutations(vec![Block3dMutation::SetDocument { document }])),
            _ => Ok(Emit::default()),
        }
    }
}
