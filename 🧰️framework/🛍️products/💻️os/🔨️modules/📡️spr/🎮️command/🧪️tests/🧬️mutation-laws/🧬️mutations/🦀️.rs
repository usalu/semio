use super::{Counter, CounterDiff};
use crate::os_spr::{OpText, OpBinary, ProtocolError};
use serde::{Deserialize, Serialize};

#[path = "➕️add-counter/🦀️.rs"] mod add_counter;
pub use add_counter::AddCounter;
#[path = "✌️add-counter-twice/🦀️.rs"] mod add_counter_twice;
pub use add_counter_twice::AddCounterTwice;
#[path = "4️⃣add-counter-four-times/🦀️.rs"] mod add_counter_four_times;
pub use add_counter_four_times::AddCounterFourTimes;
#[path = "🌐️add-counter-then-notify-foreign/🦀️.rs"] mod add_counter_then_notify_foreign;
pub use add_counter_then_notify_foreign::AddCounterThenNotifyForeign;
#[path = "🔢️add-counter-sequence/🦀️.rs"] mod add_counter_sequence;
pub use add_counter_sequence::AddCounterSequence;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::Mutations, dsl_derive::DslOps, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = Counter, diff = CounterDiff, schema = "command.test.counter")]
pub enum CounterMutation {
    AddCounter(AddCounter),
    AddCounterTwice(AddCounterTwice),
    AddCounterFourTimes(AddCounterFourTimes),
    AddCounterThenNotifyForeign(AddCounterThenNotifyForeign),
    AddCounterSequence(AddCounterSequence),
}

impl OpText for CounterMutation {
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
        for (keyword, spec_fn) in <Self as crate::os_dsl::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(&keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec = variants.iter().find(|(name, _)| name == &keyword).map(|(_, spec)| spec()).expect("owned variant schema");
        crate::os_dsl::print(&record, &spec, crate::os_dsl::JoinMode::Inline)
    }
}

impl OpBinary for CounterMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(name, _)| name == &keyword).ok_or_else(|| ProtocolError::Malformed { what: "op variant", offset: 0, detail: "unknown owned variant".into() })?;
        let spec = (variants[ordinal].1)();
        let body = crate::os_pack::encode_record_body(&spec, &record, &crate::os_pack::EncodeOptions::default()).map_err(ProtocolError::from)?;
        let mut bytes = vec![1];
        crate::os_pack::write_varint_u64(&mut bytes, u64::try_from(ordinal).map_err(|_| ProtocolError::Malformed { what: "op variant", offset: 1, detail: "variant index exceeds u64".into() })?);
        bytes.extend(body);
        Ok(bytes)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let mut reader = crate::os_pack::ByteReader::new(bytes);
        if reader.read_u8()? != 1 { return Err(ProtocolError::Malformed { what: "op format", offset: 0, detail: "unsupported counter op format".into() }); }
        let ordinal = usize::try_from(reader.read_varint_u64()?).map_err(|_| ProtocolError::Malformed { what: "op variant", offset: 1, detail: "variant index exceeds platform width".into() })?;
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal).ok_or_else(|| ProtocolError::Malformed { what: "op variant", offset: 1, detail: "variant index out of range".into() })?;
        let (record, _) = crate::os_pack::decode_record_body(&bytes[reader.position()..], &spec_fn(), &crate::os_pack::DecodeOptions::default()).map_err(ProtocolError::from)?;
        <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| ProtocolError::Malformed { what: "op record", offset: 2, detail: error.to_string() })
    }
}
