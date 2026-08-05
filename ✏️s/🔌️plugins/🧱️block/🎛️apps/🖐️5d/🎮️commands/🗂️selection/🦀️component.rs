//! 🗂️ Block 5D play app command — multi-selection in the document tree. Config-only: it emits
//! `config_operations`, never document operations.

pub mod set_selection {
    use crate::apps::block5d::config::{Block5dConfig, Block5dConfigOperation};
    use crate::artifacts::block5d::op::Block5dOperation;
    use crate::artifacts::block5d::Block5dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setSelection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, Block5dDefinition>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dOperation, Block5dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Block5dConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}
