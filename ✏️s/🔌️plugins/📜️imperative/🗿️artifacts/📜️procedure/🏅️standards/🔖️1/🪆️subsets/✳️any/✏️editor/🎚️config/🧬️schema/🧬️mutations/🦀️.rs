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

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = ImperativeConfig, diff = ImperativeConfig, schema = "imperative.config")]
pub enum ImperativeConfigMutation {
    #[dsl(key = "replace-config")]
    ReplaceConfig(ReplaceConfig),
    #[dsl(key = "set-run-output")]
    SetRunOutput(SetRunOutput),
    #[dsl(key = "set-contributions")]
    SetContributions(SetContributions),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
