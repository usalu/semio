//! 🎸️ Staged slice of the `tool-machine` band of the mounted artifact instance: its `instance-stores` members (WP6 step S0 of the instance family).
//!
//! 🛠️ The instance's tool-machine turns: gesture, scrub and typing dispatch.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use store::Mutation;
use store::MutationDiff;
use protocol::OpBinary;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use std::future::Future;
use crate::stores_components::InstanceStores;
use store::{MemberFactory, SpaceMember};
use semio_framework_ui_contract::*;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::*;

//#region 🔖️Driver
impl<A: ArtifactApp> InstanceStores<A> {
    /// 🧪️ The committed generations of the config lanes — the app config, and window `window_id` of kind `window_kind_id` once
    /// its partition exists — so a law counts the config edits a press published.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn config_generations(&self, window_kind_id: &str, window_id: &str) -> (u64, Option<u64>) {
        (self.config_store.generation(), self.window_config_store.snapshot(window_kind_id, window_id).map(|snapshot| snapshot.generation()))
    }

    /// 🧪️ The `TransactionRef` each edit of the log carries, oldest first (`None`: an edit no tool transaction authored) — a
    /// law counts edits and reads which run or press authored each.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn edit_transactions(&self) -> Vec<Option<protocol::TransactionRef>> {
        self.store.envelope().vcs.edits.iter().map(|edit| edit.mutation_meta.first().and_then(|meta| meta.transaction.clone())).collect()
    }
}
//#endregion 🔖️Driver
