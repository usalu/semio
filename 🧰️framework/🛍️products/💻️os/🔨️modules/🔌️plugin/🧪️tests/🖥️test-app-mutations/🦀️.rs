//#region 🧪️TestAppMutationFixtures
#[path = "🎚️config/🦀️.rs"]
pub mod config;
pub(crate) use config::{ChangeTestConfigSelection, TestConfig, TestConfigMutation};
#[path = "🧬️document/🦀️.rs"]
pub mod document;
pub(crate) use document::{SetCount, SetLabel, TestMutation, TestSnapshot};
//#endregion 🧪️TestAppMutationFixtures
