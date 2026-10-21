//! 🌱️ Intrinsic values are canonical single-field Pack records with explicit physical framing.

use semio_framework_pack_error::PackRefusal;
use semio_framework_pack_record::DecodeOptions;
use semio_framework_value::DslValue;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

/// 📦️ The caller declares the physical framing instead of retrying a different decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrinsicFormat { Body, Document }

#[path="🎮️retained/🦀️.rs"]
mod retained;
pub use retained::{RetainedIntrinsicBody,RetainedIntrinsicDocument,RetainedIntrinsicInput,RetainedIntrinsicStep,RetainedIntrinsicOriginalCall};

/// 🛬️ Moves the exact intrinsic value out of one complete input under cumulative caller control.
pub fn decode(bytes: &[u8], format: IntrinsicFormat, options: &DecodeOptions, control: &mut NativeDecodeControl<'_>) -> Result<DslValue, PackRefusal> {
    match format {
        IntrinsicFormat::Body => semio_framework_pack_record::intrinsic::decode_body(bytes, options, control),
        IntrinsicFormat::Document => semio_framework_pack_record::intrinsic::decode_document(bytes, options, control),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🎮️retained/🧪️tests/🦀️.rs"]
mod retained_tests;
