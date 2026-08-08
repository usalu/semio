//! 🎨️ Block 5D play app commands — load a bundled example fixture, or replace the whole document from
//! externally-edited text (the DSL editor's `edit` action).

//#region 🔖️ExampleIds
pub const BLOCK5D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
pub const BLOCK5D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
//#endregion 🔖️ExampleIds

pub mod set_active_example {
    use super::{BLOCK5D_EXAMPLE_CAPSULE, BLOCK5D_EXAMPLE_FOREST_LEFT};
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigMutation};
    use crate::artifacts::block5d::op::Block5dMutation;
    use crate::artifacts::block5d::Block5dSnapshot;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setActiveExample")]
    pub struct SetActiveExample {
        pub id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
        let example = match payload.id.as_str() {
            BLOCK5D_EXAMPLE_FOREST_LEFT => crate::artifacts::block5d::dsl::parse_dsl(crate::artifacts::block5d::dsl::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
            BLOCK5D_EXAMPLE_CAPSULE => crate::artifacts::block5d::dsl::parse_dsl(crate::artifacts::block5d::dsl::BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
            _ => None,
        };
        match example {
            Some(document) => Ok(Emit::mutations(vec![Block5dMutation::SetSnapshot { snapshot: document }])),
            None => Ok(Emit::default()),
        }
    }
}

pub mod edit {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigMutation};
    use crate::artifacts::block5d::op::Block5dMutation;
    use crate::artifacts::block5d::Block5dSnapshot;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "edit")]
    pub struct Edit {
        pub text: String,
    }

    pub fn handle(payload: &Edit, doc: &DocumentView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
        match serde_json::from_str::<Block5dSnapshot>(&payload.text) {
            Ok(document) if &document != doc.snapshot => Ok(Emit::mutations(vec![Block5dMutation::SetSnapshot { snapshot: document }])),
            _ => Ok(Emit::default()),
        }
    }
}
