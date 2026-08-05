//! 🗂️ Block 3D play app commands — document-tree multi-selection plus vortex select/hover. All
//! config-only: they emit `config_operations`, never document operations.

pub mod set_selection {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigOperation};
    use crate::artifacts::block3d::op::Block3dOperation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setSelection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigOperation::SetSelection { ids: payload.ids.clone() }]))
    }
}

pub mod select_vortex {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigOperation};
    use crate::artifacts::block3d::op::Block3dOperation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selectVortex")]
    pub struct SelectVortex {
        pub full_id: String,
        pub merge: bool,
    }

    pub fn handle(payload: &SelectVortex, _doc: &DocumentView<'_, Block3dDefinition>, cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        let local = payload.full_id.split_once(':').map(|(_, tail)| tail).unwrap_or(payload.full_id.as_str());
        let id = format!("vortex:{local}");
        let mut ids = if payload.merge { cfg.projection.selected_ids.clone() } else { Vec::new() };
        if !ids.contains(&id) {
            ids.push(id);
        }
        Ok(Emit::config(vec![Block3dConfigOperation::SetSelection { ids }]))
    }
}

pub mod hover_vortex {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigOperation};
    use crate::artifacts::block3d::op::Block3dOperation;
    use crate::artifacts::block3d::Block3dDefinition;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "hoverVortex")]
    pub struct HoverVortex {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub full_id: Option<String>,
    }

    pub fn handle(payload: &HoverVortex, _doc: &DocumentView<'_, Block3dDefinition>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dOperation, Block3dConfigOperation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigOperation::SetHoveredVortexFullId { full_id: payload.full_id.clone() }]))
    }
}
