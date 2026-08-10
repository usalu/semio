//! 🪟️ Block 3D play app commands — per-window view state: active inspector representation, visible
//! representations, arrangement, spacing, active utility. All config-only.

pub mod set_active_representation {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setActiveRepresentation")]
    pub struct SetActiveRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub representation_id: Option<String>,
    }

    pub fn handle(payload: &SetActiveRepresentation, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetActiveRepresentation { representation_id: payload.representation_id.clone() }]))
    }
}

pub mod set_window_representations {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setWindowRepresentations")]
    pub struct SetWindowRepresentations {
        pub window_id: String,
        pub representation_ids: Vec<String>,
    }

    pub fn handle(payload: &SetWindowRepresentations, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetWindowRepresentations { window_id: payload.window_id.clone(), representation_ids: payload.representation_ids.clone() }]))
    }
}

pub mod toggle_window_representation {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggleWindowRepresentation")]
    pub struct ToggleWindowRepresentation {
        pub window_id: String,
        pub representation_id: String,
        pub visible: bool,
    }

    pub fn handle(payload: &ToggleWindowRepresentation, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::ToggleWindowRepresentation { window_id: payload.window_id.clone(), representation_id: payload.representation_id.clone(), visible: payload.visible }]))
    }
}

pub mod set_window_arrangement {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setWindowArrangement")]
    pub struct SetWindowArrangement {
        pub window_id: String,
        pub arrangement: String,
    }

    pub fn handle(payload: &SetWindowArrangement, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetWindowArrangement { window_id: payload.window_id.clone(), arrangement: payload.arrangement.clone() }]))
    }
}

pub mod set_window_spacing {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setWindowSpacing")]
    pub struct SetWindowSpacing {
        pub window_id: String,
        pub spacing: f64,
    }

    pub fn handle(payload: &SetWindowSpacing, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetWindowSpacing { window_id: payload.window_id.clone(), spacing: payload.spacing }]))
    }
}

pub mod set_active_utility {
    use crate::apps::block3d::config::{Block3dConfig, Block3dConfigMutation};
    use crate::artifacts::block3d::op::Block3dMutation;
    use crate::artifacts::block3d::Block3dSnapshot;
    use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "setActiveUtility")]
    pub struct SetActiveUtility {
        pub window_id: String,
        pub utility_id: String,
    }

    pub fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Block3dConfigMutation::SetActiveUtility { window_id: payload.window_id.clone(), utility_id: payload.utility_id.clone() }]))
    }
}
