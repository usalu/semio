//! 🔗️ Block 2D play app commands — add/remove a handle-kind compatibility rule.

pub mod add_compatibility_rule {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
    use crate::artifacts::block2d::op::Block2dMutation;
    use crate::artifacts::block2d::Block2dSnapshot;
    use crate::BlockCompatibilityRule;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "addCompatibilityRule")]
    pub struct AddCompatibilityRule {
        pub source: String,
        pub target: String,
    }

    pub fn handle(payload: &AddCompatibilityRule, doc: &DocumentView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
        if payload.source.is_empty() || payload.target.is_empty() {
            return Ok(Emit::default());
        }
        let id = crate::artifacts::block2d::engine::next_id(doc.snapshot.compatibility.iter().map(|rule| rule.id.as_str()), "compat-");
        let rule = BlockCompatibilityRule { id, source: payload.source.clone(), target: payload.target.clone(), bidirectional: true };
        Ok(Emit::mutations(vec![Block2dMutation::SetCompatibilityRule { index: doc.snapshot.compatibility.len(), rule }]))
    }
}

pub mod remove_compatibility_rule {
    use crate::apps::block2d::config::{Block2dConfig, Block2dConfigMutation};
    use crate::artifacts::block2d::op::Block2dMutation;
    use crate::artifacts::block2d::Block2dSnapshot;
    use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "removeCompatibilityRule")]
    pub struct RemoveCompatibilityRule {
        pub id: String,
    }

    pub fn handle(payload: &RemoveCompatibilityRule, _doc: &DocumentView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
        Ok(Emit::mutations(vec![Block2dMutation::RemoveCompatibilityRule { id: payload.id.clone() }]))
    }
}
