//! 🗂️ Block 2D play app command — multi-selection in the document tree. Config-only: it emits
//! `config_operations`, never document operations.

pub mod set_selection {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigOperation};
    use crate::artifacts::block2d::op::Block2dOperation;
    use crate::artifacts::block2d::Block2dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setSelection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, Block2dDefinition>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dOperation, Block2dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Block2dConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}
