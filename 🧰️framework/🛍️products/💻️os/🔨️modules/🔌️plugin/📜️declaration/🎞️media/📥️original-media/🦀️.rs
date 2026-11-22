//! 📥️ Receives a picked original file under the exact mounted command Source.
#[path = "🎟️loan.rs"]
pub(crate) mod ingress;
#[path = "📥️receiver.rs"]
pub(crate) mod receiver;
#[path = "🌉️fixed-page.rs"]
pub(crate) mod bridge;
#[path = "📥️command-owner.rs"]
pub(crate) mod command_owner;
#[path = "🛂️guest.rs"]
pub(crate) mod guest;
#[path = "🪪️natural-source-gate.rs"]
pub mod natural_source_gate;
#[path = "📥️natural-source.rs"]
pub mod natural_source;
#[path = "🧾️picker.rs"]
pub(crate) mod raw_picker;
#[path = "📦️capsule.rs"]
pub(crate) mod raw_picker_pack;

pub use command_owner::OriginalMediaFormatInput;
pub use guest::{OriginalGuestMediaInput, OriginalMediaPickerIntent};
pub use ingress::{OriginalMediaLoan, ORIGINAL_MEDIA_PAGE_BYTES};
pub use raw_picker::{Ack, Control, FormatReceipt, FormatResult, Phase, RawPicker, ResultCode, Transport};
pub use raw_picker_pack::{DecodedLoan, picker_args};
pub const RAW_MEDIA_BYTES: usize = semio_framework_replication_command_ingress::COMMAND_MAXIMUM_BYTES;
pub type RawMediaTarget = [u64; 8];
pub type RawMediaPicker = RawPicker<RawMediaTarget, RAW_MEDIA_BYTES>;
pub type RawMediaInput = OriginalGuestMediaInput<RawMediaTarget, RAW_MEDIA_BYTES>;
pub type RawMediaIntent = OriginalMediaPickerIntent<RawMediaTarget>;
#[path = "🧭️runtime.rs"]
pub mod runtime;
#[path = "📨️issue.rs"]
pub mod issue;
pub use semio_framework_replication_command_ingress::ORIGINAL_MEDIA_COMMAND_KIND;
