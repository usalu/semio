//! 🪥️ Staged slice of the `command-routes` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🧷️ Command routes of the mounted artifact instance: framework-reserved routes, action dispatch and command dispatch.
use semio_framework::{ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};
use semio_framework_plugin_document_media::ActiveDocumentArchiveLoad;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_command::DOCUMENT_LOADING_CODE;
use semio_framework_plugin_decl_emission::Emit;
use crate::docs_components::EnvelopeJobs;
use std::future::Future;
use semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// ⏳️ The `document.loading` refusal of `verb` while a whole-document load still runs (`📓️api-stepped-document-load.md`
    /// §3, gap N17): every verb but a view or interaction verb waits — nothing reads or edits the document being replaced;
    /// hover and pick read only the selection (audit W2A-13) — except `historyEditCancelReplay`, which cancels the load;
    /// `None` while no load runs.
    pub fn document_loading_refusal(&self, tools: &ToolOperations<A>, verb: &str, admitted_by_extension: bool) -> Option<Fault> {
        let mut loading = false;
        self.document_archive_loads.each_id(|operation| loading |= self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::loading));
        let exempt = matches!(tools.declared_action_kind(verb), Some(ActionKind::View | ActionKind::Interaction)) || INTERACTION_ACTION_IDS.contains(&verb) || admitted_by_extension;
        (loading && !exempt).then(|| Fault::new(FaultOrigin::Framework, FaultCode::new(DOCUMENT_LOADING_CODE), format!("'{verb}' waits: the document is still loading")))
    }
}
