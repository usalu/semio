//#region 🚪️InstanceLifecycleWire
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "🩹️patch/🦀️.rs"]
mod patch_receipt;
pub use patch_receipt::{ActorUiPatchReceipt, ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES};

pub const ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES: usize = 44;
pub(crate) const REQUEST_SEQUENCE_MAXIMUM: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(crate = "::protocol::value", rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceLifetime {
    #[cfg_attr(test, serde(with = "decimal_generation"))]
    #[value(with = "decimal_generation")]
    pub activation_generation: u64,
    pub instance_id: u32,
    #[cfg_attr(test, serde(with = "decimal_generation"))]
    #[value(with = "decimal_generation")]
    pub guest_lifetime: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(crate = "::protocol::value", rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceOpenRequest {
    #[cfg_attr(test, serde(with = "decimal_generation"))]
    #[value(with = "decimal_generation")]
    pub activation_generation: u64,
    pub instance_id: u32,
    #[cfg_attr(test, serde(with = "request_sequence"))]
    #[value(with = "request_sequence")]
    pub request_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(crate = "::protocol::value", rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceCloseRequest {
    pub lifetime: ActorInstanceLifetime,
    #[cfg_attr(test, serde(with = "request_sequence"))]
    #[value(with = "request_sequence")]
    pub request_sequence: u64,
}

// 🖐️ Hand-written `ToValue`/`FromValue` below (not derived): `with` on an enum variant's own
// named field is deliberately unsupported by `#[derive(ToValue, FromValue)]` (a `compile_error!`
// naming the field) — `request_sequence`/`close_generation` here are exactly that case.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields))]
pub enum ActorInstanceLifecycleReceipt {
    Captured {
        lifetime: ActorInstanceLifetime,
        #[cfg_attr(test, serde(with = "request_sequence"))]
        request_sequence: u64,
    },
    Accepted {
        lifetime: ActorInstanceLifetime,
        #[cfg_attr(test, serde(with = "request_sequence"))]
        request_sequence: u64,
        #[cfg_attr(test, serde(with = "decimal_generation"))]
        close_generation: u64,
    },
    Retired {
        lifetime: ActorInstanceLifetime,
        #[cfg_attr(test, serde(with = "request_sequence"))]
        request_sequence: u64,
        #[cfg_attr(test, serde(with = "decimal_generation"))]
        close_generation: u64,
    },
}

impl ::protocol::value::ToValue for ActorInstanceLifecycleReceipt {
    fn to_value(&self) -> ::protocol::value::DslValue {
        use ::protocol::value::DslValue;
        let (kind, lifetime, request_sequence, close_generation) = match self {
            Self::Captured { lifetime, request_sequence } => ("captured", lifetime, request_sequence, None),
            Self::Accepted { lifetime, request_sequence, close_generation } => ("accepted", lifetime, request_sequence, Some(close_generation)),
            Self::Retired { lifetime, request_sequence, close_generation } => ("retired", lifetime, request_sequence, Some(close_generation)),
        };
        let mut entries = vec![("kind".to_string(), DslValue::String(kind.to_string())), ("lifetime".to_string(), ::protocol::value::ToValue::to_value(lifetime)), ("requestSequence".to_string(), request_sequence::to_value(request_sequence))];
        if let Some(close_generation) = close_generation {
            entries.push(("closeGeneration".to_string(), decimal_generation::to_value(close_generation)));
        }
        DslValue::object(entries)
    }
}

impl ::protocol::value::FromValue for ActorInstanceLifecycleReceipt {
    fn from_value(value: ::protocol::value::DslValue) -> Result<Self, ::protocol::value::ValueError> {
        use ::protocol::value::{DslValue, ValueError};
        let DslValue::Object(entries) = value else {
            return Err(ValueError::new("expected object"));
        };
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = get("kind").and_then(|v| v.as_str().map(str::to_string)).ok_or_else(|| ValueError::new("missing field `kind`"))?;
        let known: &[&str] = match kind.as_str() {
            "captured" => &["kind", "lifetime", "requestSequence"],
            "accepted" | "retired" => &["kind", "lifetime", "requestSequence", "closeGeneration"],
            _ => return Err(ValueError::new(format!("unknown variant `{kind}`"))),
        };
        if let Some((unknown, _)) = entries.iter().find(|(k, _)| !known.contains(&k.as_str())) {
            return Err(ValueError::new(format!("unknown field `{unknown}`")));
        }
        let lifetime: ActorInstanceLifetime = ::protocol::value::FromValue::from_value(get("lifetime").ok_or_else(|| ValueError::new("missing field `lifetime`"))?).map_err(|error: ValueError| error.under("lifetime"))?;
        let request_sequence = request_sequence::from_value(get("requestSequence").ok_or_else(|| ValueError::new("missing field `requestSequence`"))?).map_err(|error| error.under("requestSequence"))?;
        match kind.as_str() {
            "captured" => Ok(Self::Captured { lifetime, request_sequence }),
            _ => {
                let close_generation = decimal_generation::from_value(get("closeGeneration").ok_or_else(|| ValueError::new("missing field `closeGeneration`"))?).map_err(|error| error.under("closeGeneration"))?;
                if kind == "accepted" {
                    Ok(Self::Accepted { lifetime, request_sequence, close_generation })
                } else {
                    Ok(Self::Retired { lifetime, request_sequence, close_generation })
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(crate = "::protocol::value", rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorInstanceLifecycleAck {
    pub receipt: ActorInstanceLifecycleReceipt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorInstanceLifecycleWire {
    Open(ActorInstanceOpenRequest),
    Close(ActorInstanceCloseRequest),
    Receipt(ActorInstanceLifecycleReceipt),
    Ack(ActorInstanceLifecycleAck),
}

impl ActorInstanceLifetime {
    pub fn is_valid(self) -> bool {
        self.activation_generation != 0 && self.guest_lifetime != 0
    }
}

impl ActorInstanceOpenRequest {
    pub fn is_valid(self) -> bool {
        self.activation_generation != 0 && valid_request(self.request_sequence)
    }
}

impl ActorInstanceCloseRequest {
    pub fn is_valid(self) -> bool {
        self.lifetime.is_valid() && valid_request(self.request_sequence)
    }
}

impl ActorInstanceLifecycleReceipt {
    pub fn lifetime(self) -> ActorInstanceLifetime {
        match self {
            Self::Captured { lifetime, .. } | Self::Accepted { lifetime, .. } | Self::Retired { lifetime, .. } => lifetime,
        }
    }

    pub fn request_sequence(self) -> u64 {
        match self {
            Self::Captured { request_sequence, .. } | Self::Accepted { request_sequence, .. } | Self::Retired { request_sequence, .. } => request_sequence,
        }
    }

    pub fn close_generation(self) -> Option<u64> {
        match self {
            Self::Captured { .. } => None,
            Self::Accepted { close_generation, .. } | Self::Retired { close_generation, .. } => Some(close_generation),
        }
    }

    pub fn is_valid(self) -> bool {
        self.lifetime().is_valid() && valid_request(self.request_sequence()) && self.close_generation() != Some(0)
    }

    fn tag(self) -> u8 {
        match self {
            Self::Captured { .. } => 1,
            Self::Accepted { .. } => 3,
            Self::Retired { .. } => 4,
        }
    }
}

impl ActorInstanceLifecycleWire {
    /// 📤️ Encodes the complete fixed authority; invalid input leaves the caller's output untouched.
    pub fn encode(&self, output: &mut [u8; ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES]) -> Result<usize, &'static str> {
        let (tag, activation, instance, guest, request, close) = match *self {
            Self::Open(open) => {
                if !open.is_valid() {
                    return Err("actor-lifecycle.invalid-authority");
                }
                (0, open.activation_generation, open.instance_id, None, open.request_sequence, None)
            }
            Self::Close(close) => {
                if !close.is_valid() {
                    return Err("actor-lifecycle.invalid-authority");
                }
                (2, close.lifetime.activation_generation, close.lifetime.instance_id, Some(close.lifetime.guest_lifetime), close.request_sequence, None)
            }
            Self::Receipt(receipt) | Self::Ack(ActorInstanceLifecycleAck { receipt }) => {
                if !receipt.is_valid() {
                    return Err("actor-lifecycle.invalid-authority");
                }
                let tag = if matches!(self, Self::Ack(_)) {
                    if receipt.tag() == 1 {
                        5
                    } else {
                        receipt.tag() + 3
                    }
                } else {
                    receipt.tag()
                };
                let lifetime = receipt.lifetime();
                (tag, lifetime.activation_generation, lifetime.instance_id, Some(lifetime.guest_lifetime), receipt.request_sequence(), receipt.close_generation())
            }
        };
        output[0] = tag;
        let mut offset = 1;
        for mut value in [Some(activation), Some(u64::from(instance)), guest, Some(request), close].into_iter().flatten() {
            loop {
                let byte = (value & 127) as u8;
                value >>= 7;
                output[offset] = byte | if value == 0 { 0 } else { 128 };
                offset += 1;
                if value == 0 {
                    break;
                }
            }
        }
        Ok(offset)
    }

    /// 📥️ Decodes canonical unsigned LEB128 without allocating, coercing, or accepting old authority shapes.
    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() || bytes.len() > ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES {
            return Err("actor-lifecycle.envelope");
        }
        let tag = bytes[0];
        if tag > 7 {
            return Err("actor-lifecycle.tag");
        }
        let mut offset = 1;
        let activation_generation = read_unsigned(bytes, &mut offset, u64::MAX, true)?;
        let instance_id = read_unsigned(bytes, &mut offset, u64::from(u32::MAX), false)? as u32;
        let guest_lifetime = if tag == 0 { None } else { Some(read_unsigned(bytes, &mut offset, u64::MAX, true)?) };
        let request_sequence = read_unsigned(bytes, &mut offset, REQUEST_SEQUENCE_MAXIMUM, true)?;
        let value = if tag == 0 {
            Self::Open(ActorInstanceOpenRequest { activation_generation, instance_id, request_sequence })
        } else {
            let lifetime = ActorInstanceLifetime { activation_generation, instance_id, guest_lifetime: guest_lifetime.expect("non-open tag owns its parsed guest lifetime") };
            if tag == 2 {
                Self::Close(ActorInstanceCloseRequest { lifetime, request_sequence })
            } else {
                let receipt = if tag == 1 || tag == 5 {
                    ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence }
                } else {
                    let close_generation = read_unsigned(bytes, &mut offset, u64::MAX, true)?;
                    if tag == 3 || tag == 6 {
                        ActorInstanceLifecycleReceipt::Accepted { lifetime, request_sequence, close_generation }
                    } else {
                        ActorInstanceLifecycleReceipt::Retired { lifetime, request_sequence, close_generation }
                    }
                };
                if tag >= 5 {
                    Self::Ack(ActorInstanceLifecycleAck { receipt })
                } else {
                    Self::Receipt(receipt)
                }
            }
        };
        if offset != bytes.len() {
            return Err("actor-lifecycle.trailing");
        }
        Ok(value)
    }
}

pub(crate) fn valid_request(value: u64) -> bool {
    value != 0 && value <= REQUEST_SEQUENCE_MAXIMUM
}

pub(crate) fn read_unsigned(bytes: &[u8], offset: &mut usize, maximum: u64, nonzero: bool) -> Result<u64, &'static str> {
    let mut value = 0u64;
    for index in 0..10 {
        let byte = *bytes.get(*offset).ok_or("actor-lifecycle.truncated")?;
        *offset += 1;
        if index == 9 && byte & 126 != 0 {
            return Err("actor-lifecycle.overflow");
        }
        value |= u64::from(byte & 127) << (index * 7);
        if byte & 128 == 0 {
            if (index != 0 && byte == 0) || value > maximum || (nonzero && value == 0) {
                return Err("actor-lifecycle.noncanonical-authority");
            }
            return Ok(value);
        }
    }
    Err("actor-lifecycle.overlong")
}

/// 🪪️ Matches the first captured response to its exact open request; the caller must retain that guest lifetime.
pub fn actor_instance_captured_receipt_matches(open: ActorInstanceOpenRequest, receipt: ActorInstanceLifecycleReceipt) -> bool {
    open.is_valid()
        && receipt.is_valid()
        && matches!(receipt, ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence } if lifetime.activation_generation == open.activation_generation && lifetime.instance_id == open.instance_id && request_sequence == open.request_sequence)
}

/// 📨️ Checks receipt identity and acceptance ordering without manufacturing descendant terminal proof.
pub fn actor_instance_close_receipt_matches(request: ActorInstanceCloseRequest, accepted: Option<ActorInstanceLifecycleReceipt>, receipt: ActorInstanceLifecycleReceipt) -> bool {
    if !request.is_valid() || !receipt.is_valid() || request.lifetime != receipt.lifetime() || request.request_sequence != receipt.request_sequence() || matches!(receipt, ActorInstanceLifecycleReceipt::Captured { .. }) {
        return false;
    }
    match accepted {
        None => matches!(receipt, ActorInstanceLifecycleReceipt::Accepted { .. }),
        Some(accepted) => {
            matches!(accepted, ActorInstanceLifecycleReceipt::Accepted { .. })
                && accepted.is_valid()
                && accepted.lifetime() == receipt.lifetime()
                && accepted.request_sequence() == receipt.request_sequence()
                && accepted.close_generation() == receipt.close_generation()
        }
    }
}

pub(crate) mod decimal_generation {
    #[cfg(test)]
    use serde::{Deserializer, Serializer};

    #[cfg(test)]
    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        if *value == 0 {
            return Err(serde::ser::Error::custom("zero lifecycle generation"));
        }
        serializer.collect_str(value)
    }

    #[cfg(test)]
    struct DecimalVisitor;

    #[cfg(test)]
    impl<'de> serde::de::Visitor<'de> for DecimalVisitor {
        type Value = u64;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a canonical nonzero unsigned 64-bit decimal string")
        }
        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<u64, E> {
            if value.is_empty() || value.len() > 20 || value.as_bytes()[0] == b'0' || !value.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(E::custom("noncanonical lifecycle generation"));
            }
            value.parse().map_err(E::custom)
        }
    }

    #[cfg(test)]
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        deserializer.deserialize_str(DecimalVisitor)
    }

    /// 🔁️ `ToValue`/`FromValue` analogs of [`serialize`]/[`deserialize`] above — same decimal-string
    /// wire shape, `ToValue::to_value` stays infallible (the zero-generation guard only lives in
    /// [`from_value`]'s canonical-string check, mirroring [`deserialize`]'s own validation).
    pub fn to_value(value: &u64) -> ::protocol::value::DslValue {
        ::protocol::value::DslValue::String(value.to_string())
    }

    pub fn from_value(value: ::protocol::value::DslValue) -> Result<u64, ::protocol::value::ValueError> {
        let ::protocol::value::DslValue::String(text) = value else {
            return Err(::protocol::value::ValueError::new("expected a canonical nonzero unsigned 64-bit decimal string"));
        };
        if text.is_empty() || text.len() > 20 || text.as_bytes()[0] == b'0' || !text.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(::protocol::value::ValueError::new("noncanonical lifecycle generation"));
        }
        text.parse().map_err(|_| ::protocol::value::ValueError::new("noncanonical lifecycle generation"))
    }
}

pub(crate) mod request_sequence {
    #[cfg(test)]
    use serde::{Deserialize, Deserializer, Serializer};

    #[cfg(test)]
    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        if !super::valid_request(*value) {
            return Err(serde::ser::Error::custom("invalid lifecycle request"));
        }
        serializer.serialize_u64(*value)
    }

    #[cfg(test)]
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        let value = u64::deserialize(deserializer)?;
        if !super::valid_request(value) {
            return Err(serde::de::Error::custom("invalid lifecycle request"));
        }
        Ok(value)
    }

    /// 🔁️ `ToValue`/`FromValue` analogs of [`serialize`]/[`deserialize`] above — bare-integer wire
    /// shape (matches [`serde_json`]'s own JSON number, not a decimal string).
    pub fn to_value(value: &u64) -> ::protocol::value::DslValue {
        ::protocol::value::DslValue::uint(*value)
    }

    pub fn from_value(value: ::protocol::value::DslValue) -> Result<u64, ::protocol::value::ValueError> {
        let value = <u64 as ::protocol::value::FromValue>::from_value(value).map_err(|_| ::protocol::value::ValueError::new("expected an unsigned 64-bit integer"))?;
        if !super::valid_request(value) {
            return Err(::protocol::value::ValueError::new("invalid lifecycle request"));
        }
        Ok(value)
    }
}

//#region 🧪️SharedWireLaws
#[cfg(test)]
#[path = "🩹️patch/🧪️tests/🩹️patch/🦀️.rs"]
mod patch_receipt_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️SharedWireLaws
//#endregion 🚪️InstanceLifecycleWire
