//! 🪐️ Staged slice of the `envelope-ladder` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The envelope-decode frontier of one hosted app: worker jobs, returned field decoders and completed records
//! retire one owner per turn under the caller's unchanged grant and exact per-axis quotes.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use crate::docs_components::EnvelopeJobs;
use semio_framework::FaultFrom;
use std::future::Future;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// 🪹️ No live decode, returned field decoder, or completed record remains.
    pub fn envelope_decode_terminal_is_empty(&self) -> bool {
        self.envelope_decode_jobs.is_empty()
            && self.envelope_field_decoder_retirements.is_empty()
            && self.envelope_completed_record_retirements.is_empty()
            && self.envelope_field_decoders.terminal_is_empty()
            && self.envelope_completed_records.terminal_is_empty()
    }
}
