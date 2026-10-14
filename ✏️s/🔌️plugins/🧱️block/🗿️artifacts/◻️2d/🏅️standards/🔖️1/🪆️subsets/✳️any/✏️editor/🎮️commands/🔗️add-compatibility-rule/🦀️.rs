//! 🔗️ 🔗️ Block 2D play app commands command — `add-compatibility-rule`.

use semio_s_artifact_block_2d_mutations::Block2dMutation;
use semio_s_artifact_block_2d_model::Block2dSnapshot;
use crate::config::{Block2dConfig, Block2dConfigMutation};
use semio_s_plugin_block::BlockCompatibilityRule;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "addCompatibilityRule")]
pub struct AddCompatibilityRule {
    pub source: String,
    pub target: String,
}

pub fn handle(payload: &AddCompatibilityRule, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    if payload.source.is_empty() || payload.target.is_empty() {
        return Ok(Emit::default());
    }
    let id = semio_s_artifact_block_2d_schema::next_id(doc.snapshot.compatibility.iter().map(|rule| rule.id.as_str()), "compat-");
    let rule = BlockCompatibilityRule { id, source: payload.source.clone(), target: payload.target.clone(), bidirectional: true };
    Ok(Emit::mutations(vec![semio_s_artifact_block_2d_mut_meta::add_compatibility_rule(rule)]))
}
