//! 🪐️ Staged slice of the `envelope-ladder` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The envelope-decode frontier of one hosted app: worker jobs, returned field decoders and completed records
//! retire one owner per turn under the caller's unchanged grant and exact per-axis quotes.
use super::*;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close}};

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    /// 🪹️ No live decode, returned field decoder, or completed record remains.
    pub(crate) fn envelope_decode_terminal_is_empty(&self) -> bool {
        self.envelope_decode_jobs.is_empty()
            && self.envelope_field_decoder_retirements.is_empty()
            && self.envelope_completed_record_retirements.is_empty()
            && self.envelope_field_decoders.terminal_is_empty()
            && self.envelope_completed_records.terminal_is_empty()
    }
}
