//! 🧬️ Imperative configuration mutation collection.

use super::*;
#[path = "📸️replace-config/🦀️.rs"]
mod replace_config;
pub use replace_config::ReplaceConfig;
#[path = "📤️set-run-output/🦀️.rs"]
mod set_run_output;
pub use set_run_output::SetRunOutput;
#[path = "🧩️set-contributions/🦀️.rs"]
mod set_contributions;
pub use set_contributions::SetContributions;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = ImperativeConfig, diff = ImperativeConfigDiff, schema = "imperative.config")]
pub enum ImperativeConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-run-output")]
    SetRunOutput(SetRunOutput),
    #[dsl(key = "set-contributions")]
    SetContributions(SetContributions),
}

impl store::snapshot_clone_preparation::ConfigApplyMutation<ImperativeConfig> for ImperativeConfigMutation {
    fn exchange(self, post: &mut ImperativeConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::ReplaceConfig(ReplaceConfig { config }) => Self::ReplaceConfig(ReplaceConfig { config: std::mem::replace(post, config) }),
            Self::SetRunOutput(SetRunOutput { json }) => Self::SetRunOutput(SetRunOutput { json: std::mem::replace(&mut post.run_output_json, json) }),
            Self::SetContributions(SetContributions { json }) => Self::SetContributions(SetContributions { json: std::mem::replace(&mut post.contributions_json, json) }),
        })
    }

    fn payload_bytes(&self) -> usize {
        match self {
            Self::ReplaceConfig(ReplaceConfig { config }) => config.run_output_json.len() + config.contributions_json.len(),
            Self::SetRunOutput(SetRunOutput { json }) | Self::SetContributions(SetContributions { json }) => json.len(),
        }
    }
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
