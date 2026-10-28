//! 🪻️ Staged slice of the `tool-run` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! ⏯️ The mounted instance's tool-run turns (ledger driving, tick folding and finalize publication) and the
//! [`ToolRunExtension`] that registers them with the instance through [`InstanceExtension`].
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_tool_run::ToolRunEntry;
use semio_framework_plugin_tool_run::ToolRunJobHandle;
use semio_framework_plugin_tool_run::ToolRunJobSlot;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::Mutation;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework::Fault;
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework_ui_locale::LocalizedLabel;
use crate::jobs_components::ToolOperations;
use semio_framework_ui_contract::*;
use semio_framework_plugin_tool_run::*;
use std::sync::Arc;
use semio_framework_plugin_decl_app::tool_run_contract::ToolRunJobRequest;
use semio_framework_os_kernel::{MemberFactory, ReferenceSpaceMember};
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPurpose;

impl<A: ArtifactApp> ToolOperations<A> {
    /// 🧵️ Builds the run's job for `purpose`: the app's retargetable job when it builds one, else its plain job.
    pub fn build_tool_run_job_slot<M: ReferenceSpaceMember + MemberFactory + 'static>(&self, stores: &InstanceStores<A>, child_composition: &ChildComposition<M>, entry: &ToolRunEntry<A>, purpose: ToolRunJobPurpose) -> Result<Option<ToolRunJobSlot<A::Config>>, Fault> {
        let request = || ToolRunJobRequest::<A> {
            receiving: &entry.definition.receiving,
            tool_id: &entry.tool_id,
            definition: &entry.definition,
            purpose,
            identity: entry.identity,
            snapshot: match purpose {
                ToolRunJobPurpose::Run => Arc::clone(&entry.base),
                ToolRunJobPurpose::Revalidate => stores.store.snapshot_owner(),
            },
            config: stores.config_store.snapshot_owner(),
            window_id: entry.window.as_ref().map(|(window_id, _)| window_id.as_str()),
            window_config: entry.window.as_ref().and_then(|(window_id, window_kind_id)| stores.window_config_store.snapshot(window_kind_id, window_id)),
            checkpoint: entry.checkpoint.as_deref(),
            provisional: &entry.provisional,
            instance_owner: self.instance_operation_owner.clone(),
            port: entry.port.clone(),
            host: self.live_runtime_instance_id.map(semio_framework_plugin_reactor_tasks::tasks_extension_continuation::host_for_instance),
            trace_keys: entry.trace_keys.clone(),
            entity_marks: &entry.entity_marks,
            children: ChildContentView::clone(&child_composition.child_content_root),
            member_ops: entry.member.as_ref().map_or(&[], |member| member.ops.as_slice()),
        };
        let generation = entry.identity.generation;
        if let Some(job) = A::build_retargetable_tool_run_job(request())? {
            return Ok(Some(ToolRunJobSlot::new(ToolRunJobHandle::Retargetable(job), purpose, generation, entry.definition.receiving)));
        }
        Ok(A::build_tool_run_job(request())?.map(|job| ToolRunJobSlot::new(ToolRunJobHandle::Plain(job), purpose, generation, entry.definition.receiving)))
    }

    /// 🏷️ The history row label of a transaction a finalized run published (`<appId>#<toolId>` of a declared tool
    /// run): the tool's own label in every locale, so the row reads the same after a reload.
    pub(crate) fn tool_run_transaction_label(&self, app_id: &str, transaction: &protocol::TransactionRef) -> Option<LocalizedLabel> {
        let tool_id = transaction.tool.strip_prefix(app_id)?.strip_prefix('#')?;
        self.registry.tool_run(tool_id).map(|(label, _)| label.clone())
    }
}
