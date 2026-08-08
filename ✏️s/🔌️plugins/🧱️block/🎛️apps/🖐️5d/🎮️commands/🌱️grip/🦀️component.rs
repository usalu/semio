//! 🌱️ Block 5D play app commands — add/remove a rim-grip template.

pub mod add_grip {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigMutation};
    use crate::artifacts::block5d::op::Block5dMutation;
    use crate::artifacts::block5d::{Block5dDefinition, Block5dGripTemplate};
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addGrip")]
    pub struct AddGrip {}

    pub fn handle(_payload: &AddGrip, doc: &DocumentView<'_, Block5dDefinition>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
        let Some(grip_kind_id) = doc.projection.grip_kinds.first().map(|kind| kind.id.clone()) else {
            return Ok(Emit::default());
        };
        let id = crate::artifacts::block5d::engine::next_id(doc.projection.grips.iter().map(|grip| grip.id.as_str()), "grip-");
        let grip = Block5dGripTemplate { id, grip_kind: grip_kind_id, angle: 0.0, radius_2d: 0.36, position: [0.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius_3d: 0.36 };
        Ok(Emit::mutations(vec![Block5dMutation::SetGrip { index: doc.projection.grips.len(), grip }]))
    }
}

pub mod remove_grip {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigMutation};
    use crate::artifacts::block5d::op::Block5dMutation;
    use crate::artifacts::block5d::Block5dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeGrip")]
    pub struct RemoveGrip {
        pub id: String,
    }

    pub fn handle(payload: &RemoveGrip, _doc: &DocumentView<'_, Block5dDefinition>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![Block5dMutation::RemoveGrip { id: payload.id.clone() }]))
    }
}
