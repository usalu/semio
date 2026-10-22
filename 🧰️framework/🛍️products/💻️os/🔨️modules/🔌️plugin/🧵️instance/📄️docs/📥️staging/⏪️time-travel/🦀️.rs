//! 🦠️ Staged slice of the `time-travel` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! ⏪️ The mounted instance's time-travel turns (history editing sessions, replay and supersession) and the
//! [`TimeTravelExtension`] that registers them with the instance through [`InstanceExtension`].
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use store::Mutation;
use protocol::OpText;
use crate::docs_components::EnvelopeJobs;
use std::future::Future;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;
use semio_framework_plugin_time_travel::*;
use semio_framework_plugin_document_media::ActiveDocumentArchiveLoad;
use semio_framework_os_kernel::{MemberFactory, ReferenceSpaceMember};

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// 🛬️ The whole-document load (document archive operation) still running on this instance, if any
    /// (`📓️api-stepped-document-load.md`).
    pub fn live_document_load(&self) -> Option<u64> {
        let mut live = None;
        self.document_archive_loads.each_id(|operation| {
            if live.is_none() && self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::loading) {
                live = Some(operation);
            }
        });
        live
    }
}
