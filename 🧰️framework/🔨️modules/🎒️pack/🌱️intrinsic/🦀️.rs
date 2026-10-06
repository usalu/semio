//! 🌱️ Intrinsic values are canonical single-field Pack records with explicit physical framing.
use crate::{PackRefusal, record::DecodeOptions};
use semio_framework_dsl_record::{FieldSpec, FieldValue, RecordLayout, RecordSpec, Shape};
use semio_framework_value::{DslValue, NativeDecodeControl, ValueRefusalKind};

/// 📦️ The caller declares the physical framing instead of retrying a different decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrinsicFormat { Body, Document }

const FIELD_ID: u16 = 1;

/// 🛬️ Moves the exact intrinsic value out of one complete input under cumulative caller control.
pub fn decode(bytes: &[u8], format: IntrinsicFormat, options: &DecodeOptions, control: &mut NativeDecodeControl<'_>) -> Result<DslValue, PackRefusal> {
    match format {
        IntrinsicFormat::Body => crate::record::decode_value_record_body_exact_controlled(bytes, FIELD_ID, options, control),
        IntrinsicFormat::Document => {
            let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(FIELD_ID, "value", Shape::Value)]);
            let (mut record, report) = crate::record::decode_document_controlled(bytes, &spec, options, control)?;
            if !report.unknown_field_ids.is_empty() || record.fields.len() != 1 {
                return Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "intrinsic document", offset: 0, detail: "expected exactly one declared Value field".into() });
            }
            match record.fields.remove(&FIELD_ID) {
                Some(FieldValue::Value(value)) => Ok(value),
                _ => Err(PackRefusal::Malformed { kind: ValueRefusalKind::InvalidValue, what: "intrinsic document", offset: 0, detail: "declared intrinsic field is absent".into() }),
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
