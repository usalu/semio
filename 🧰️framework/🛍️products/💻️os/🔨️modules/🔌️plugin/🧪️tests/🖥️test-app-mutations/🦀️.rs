//#region 🧪️TestAppMutationFixtures
#[path = "../🖥️test-app-mutations-config/🦀️.rs"]
pub mod config;
pub(crate) use config::{ChangeTestConfigSelection, TestConfig, TestConfigMutation};
#[path = "../🖥️test-app-mutations-document/🦀️.rs"]
pub mod document;
pub(crate) use document::{SetCount, SetLabel, TestMutation, TestSnapshot};
//#endregion 🧪️TestAppMutationFixtures
