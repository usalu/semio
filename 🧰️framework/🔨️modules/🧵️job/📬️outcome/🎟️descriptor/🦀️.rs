//! 🎟️ Context admissions keep capability minting local while pure descriptor ownership lives with payloads.
use semio_framework_job_payload as payload;
pub use payload::JobOutcomeKind;
pub type JobOutcomeDescriptor=payload::OriginalOutcomeDescriptor<super::JobOutcomeAdmission>;
pub type JobOutcomeDescriptorSlot=payload::OriginalOutcomeDescriptorSlot<super::JobOutcomeAdmission>;
pub type JobOutcomeView<'a>=payload::OriginalOutcomeView<'a,super::JobOutcomeAdmission>;
