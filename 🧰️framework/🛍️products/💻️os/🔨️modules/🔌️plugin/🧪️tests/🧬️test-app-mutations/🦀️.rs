//#region 🧪️TestAppMutationFixtures
#[path = "🎚️config/🦀️.rs"] pub mod config;
pub(crate) use config::{ChangeTestConfigSelection, TestConfig, TestConfigDiff, TestConfigMutation};
#[path = "🧬️document/🦀️.rs"] pub mod document;
pub(crate) use document::{SetCount, SetLabel, TestDiff, TestMutation, TestSnapshot};
//#endregion 🧪️TestAppMutationFixtures
