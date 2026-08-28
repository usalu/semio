//#region 📤️RetainedReturnWire
use crate::byte_page::{ActorBytePage, ACTOR_BYTE_PAGE_BYTES};
use crate::instance_lifetime::{decimal_generation, read_unsigned, request_sequence, valid_request, REQUEST_SEQUENCE_MAXIMUM};
use serde::{Deserialize, Serialize};

pub const ACTOR_RETURN_DRIVE_MAXIMUM_BYTES: usize = 43;
pub const ACTOR_RETURN_RESULT_MAXIMUM_BYTES: usize = 4138;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorReturnOrigin {
    #[serde(with = "decimal_generation")]
    pub activation_generation: u64,
    #[serde(with = "request_sequence")]
    pub request_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorReturnIdentity {
    pub origin: ActorReturnOrigin,
    #[serde(with = "decimal_generation")]
    pub return_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorReturnPageReceipt {
    pub identity: ActorReturnIdentity,
    #[serde(with = "decimal_generation")]
    pub page_sequence: u64,
    pub length: u32,
    #[serde(rename = "final")]
    pub final_page: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum ActorReturnControl {
    Poll { identity: ActorReturnIdentity },
    InputAck { receipt: ActorReturnPageReceipt },
    Cancel { identity: ActorReturnIdentity },
    RetiredAck { identity: ActorReturnIdentity },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum ActorReturnDrive {
    Execute { origin: ActorReturnOrigin },
    Control { control: ActorReturnControl },
}

macro_rules! wire_enum {
    ($name:ident { $($variant:ident = $tag:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[repr(u8)]
        pub enum $name { $($variant = $tag),+ }
        impl $name {
            fn decode(tag: u8) -> Result<Self, &'static str> {
                match tag { $($tag => Ok(Self::$variant),)+ _ => Err("actor-return.enum") }
            }
        }
    };
}

wire_enum!(ActorReturnPendingReason { Working = 0, Blocked = 1, AwaitingInput = 2, Closing = 3 });
wire_enum!(ActorReturnCompletion { Complete = 0, Cancelled = 1, Faulted = 2 });
wire_enum!(ActorReturnControlOutcome { Accepted = 0, Duplicate = 1, Blocked = 2, Refused = 3 });
wire_enum!(ActorReturnFault {
    None = 0, Capacity = 1, SequenceExhausted = 2, StaleOrigin = 3, StaleIdentity = 4,
    WrongPage = 5, InputNotRetired = 6, NotRetired = 7, ClockUnavailable = 8,
    ClockBackward = 9, Deadline = 10, OwnerFault = 11, MalformedControl = 12, MixedControl = 13,
});

#[derive(Debug, PartialEq, Eq)]
pub enum ActorReturnResult {
    Refused { origin: ActorReturnOrigin, fault: ActorReturnFault },
    Pending { identity: ActorReturnIdentity, reason: ActorReturnPendingReason },
    Page { receipt: ActorReturnPageReceipt, page: ActorBytePage },
    Retired { identity: ActorReturnIdentity, completion: ActorReturnCompletion },
    Control { control: ActorReturnControl, outcome: ActorReturnControlOutcome, fault: ActorReturnFault },
    ProtocolFault { fault: ActorReturnFault },
}

impl ActorReturnOrigin {
    pub fn is_valid(self) -> bool { self.activation_generation != 0 && valid_request(self.request_sequence) }
}

impl ActorReturnIdentity {
    pub fn is_valid(self) -> bool { self.origin.is_valid() && self.return_sequence != 0 }
}

impl ActorReturnPageReceipt {
    pub fn is_valid(self) -> bool {
        self.identity.is_valid() && self.page_sequence != 0 && self.length <= ACTOR_BYTE_PAGE_BYTES as u32 && (self.final_page || self.length != 0)
    }
}

impl ActorReturnControl {
    pub fn identity(self) -> ActorReturnIdentity {
        match self {
            Self::Poll { identity } | Self::Cancel { identity } | Self::RetiredAck { identity } => identity,
            Self::InputAck { receipt } => receipt.identity,
        }
    }

    pub fn is_valid(self) -> bool {
        match self { Self::InputAck { receipt } => receipt.is_valid(), _ => self.identity().is_valid() }
    }
}

impl ActorReturnDrive {
    pub fn is_valid(self) -> bool {
        match self { Self::Execute { origin } => origin.is_valid(), Self::Control { control } => control.is_valid() }
    }

    /// 📬️ Validates the entire authority before touching caller-owned fixed output storage.
    pub fn encode(&self, output: &mut [u8; ACTOR_RETURN_DRIVE_MAXIMUM_BYTES]) -> Result<usize, &'static str> {
        if !self.is_valid() { return Err("actor-return.invalid-authority"); }
        let mut writer = Writer { output, offset: 0 };
        match *self {
            Self::Execute { origin } => { writer.byte(0); writer.origin(origin); }
            Self::Control { control } => { writer.byte(1); writer.control(control); }
        }
        Ok(writer.offset)
    }

    /// 📥️ Reads one exact canonical drive without constructing authority for malformed input.
    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > ACTOR_RETURN_DRIVE_MAXIMUM_BYTES { return Err("actor-return.envelope"); }
        let mut reader = Reader { bytes, offset: 0 };
        let value = match reader.byte()? {
            0 => Self::Execute { origin: reader.origin()? },
            1 => Self::Control { control: reader.control()? },
            _ => return Err("actor-return.drive-tag"),
        };
        reader.finish()?;
        if !value.is_valid() { return Err("actor-return.invalid-authority"); }
        Ok(value)
    }
}

impl ActorReturnResult {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Refused { origin, fault } => origin.is_valid() && *fault != ActorReturnFault::None,
            Self::Pending { identity, .. } | Self::Retired { identity, .. } => identity.is_valid(),
            Self::Page { receipt, page } => receipt.is_valid() && receipt.length as usize == page.len(),
            Self::Control { control, outcome, fault } => {
                let success = matches!(outcome, ActorReturnControlOutcome::Accepted | ActorReturnControlOutcome::Duplicate);
                control.is_valid() && success == (*fault == ActorReturnFault::None) && (!matches!(control, ActorReturnControl::Poll { .. }) || !success)
            }
            Self::ProtocolFault { fault } => matches!(fault, ActorReturnFault::MalformedControl | ActorReturnFault::MixedControl),
        }
    }

    /// 📦️ Emits at most one fixed page; invalid pairing or refusal leaves all output bytes untouched.
    pub fn encode(&self, output: &mut [u8; ACTOR_RETURN_RESULT_MAXIMUM_BYTES]) -> Result<usize, &'static str> {
        if !self.is_valid() { return Err("actor-return.invalid-result"); }
        let mut writer = Writer { output, offset: 0 };
        match self {
            Self::Refused { origin, fault } => { writer.byte(0); writer.origin(*origin); writer.byte(*fault as u8); }
            Self::Pending { identity, reason } => { writer.byte(1); writer.identity(*identity); writer.byte(*reason as u8); }
            Self::Page { receipt, page } => { writer.byte(2); writer.receipt(*receipt); writer.bytes(page.storage()); }
            Self::Retired { identity, completion } => { writer.byte(3); writer.identity(*identity); writer.byte(*completion as u8); }
            Self::Control { control, outcome, fault } => { writer.byte(4); writer.control(*control); writer.byte(*outcome as u8); writer.byte(*fault as u8); }
            Self::ProtocolFault { fault } => { writer.byte(5); writer.byte(*fault as u8); }
        }
        Ok(writer.offset)
    }

    /// 🧾️ Decodes fixed control or page storage, enforcing exact lengths and zero unused page bytes.
    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > ACTOR_RETURN_RESULT_MAXIMUM_BYTES { return Err("actor-return.envelope"); }
        let mut reader = Reader { bytes, offset: 0 };
        let value = match reader.byte()? {
            0 => Self::Refused { origin: reader.origin()?, fault: ActorReturnFault::decode(reader.byte()?)? },
            1 => Self::Pending { identity: reader.identity()?, reason: ActorReturnPendingReason::decode(reader.byte()?)? },
            2 => {
                let receipt = reader.receipt()?;
                let storage: [u8; ACTOR_BYTE_PAGE_BYTES] = reader.take(ACTOR_BYTE_PAGE_BYTES)?.try_into().map_err(|_| "actor-return.page-length")?;
                Self::Page { receipt, page: ActorBytePage::try_from_array(storage, receipt.length)? }
            }
            3 => Self::Retired { identity: reader.identity()?, completion: ActorReturnCompletion::decode(reader.byte()?)? },
            4 => Self::Control { control: reader.control()?, outcome: ActorReturnControlOutcome::decode(reader.byte()?)?, fault: ActorReturnFault::decode(reader.byte()?)? },
            5 => Self::ProtocolFault { fault: ActorReturnFault::decode(reader.byte()?)? },
            _ => return Err("actor-return.result-tag"),
        };
        reader.finish()?;
        if !value.is_valid() { return Err("actor-return.invalid-result"); }
        Ok(value)
    }
}

//#region 🔢️CanonicalFields
struct Writer<'a> { output: &'a mut [u8], offset: usize }

impl Writer<'_> {
    fn byte(&mut self, value: u8) { self.output[self.offset] = value; self.offset += 1; }

    fn bytes(&mut self, value: &[u8]) {
        self.output[self.offset..self.offset + value.len()].copy_from_slice(value);
        self.offset += value.len();
    }

    fn unsigned(&mut self, mut value: u64) {
        loop {
            let byte = (value & 127) as u8;
            value >>= 7;
            self.byte(byte | if value == 0 { 0 } else { 128 });
            if value == 0 { break; }
        }
    }

    fn origin(&mut self, value: ActorReturnOrigin) {
        self.unsigned(value.activation_generation);
        self.unsigned(value.request_sequence);
    }

    fn identity(&mut self, value: ActorReturnIdentity) { self.origin(value.origin); self.unsigned(value.return_sequence); }

    fn receipt(&mut self, value: ActorReturnPageReceipt) {
        self.identity(value.identity);
        self.unsigned(value.page_sequence);
        self.unsigned(u64::from(value.length));
        self.byte(u8::from(value.final_page));
    }

    fn control(&mut self, value: ActorReturnControl) {
        match value {
            ActorReturnControl::Poll { identity } => { self.byte(0); self.identity(identity); }
            ActorReturnControl::InputAck { receipt } => { self.byte(1); self.receipt(receipt); }
            ActorReturnControl::Cancel { identity } => { self.byte(2); self.identity(identity); }
            ActorReturnControl::RetiredAck { identity } => { self.byte(3); self.identity(identity); }
        }
    }
}

struct Reader<'a> { bytes: &'a [u8], offset: usize }

impl<'a> Reader<'a> {
    fn take(&mut self, length: usize) -> Result<&'a [u8], &'static str> {
        let end = self.offset.checked_add(length).ok_or("actor-return.length")?;
        let bytes = self.bytes.get(self.offset..end).ok_or("actor-return.truncated")?;
        self.offset = end;
        Ok(bytes)
    }

    fn byte(&mut self) -> Result<u8, &'static str> { Ok(self.take(1)?[0]) }

    fn unsigned(&mut self, maximum: u64, nonzero: bool) -> Result<u64, &'static str> {
        read_unsigned(self.bytes, &mut self.offset, maximum, nonzero).map_err(|_| "actor-return.noncanonical-field")
    }

    fn origin(&mut self) -> Result<ActorReturnOrigin, &'static str> {
        Ok(ActorReturnOrigin { activation_generation: self.unsigned(u64::MAX, true)?, request_sequence: self.unsigned(REQUEST_SEQUENCE_MAXIMUM, true)? })
    }

    fn identity(&mut self) -> Result<ActorReturnIdentity, &'static str> {
        Ok(ActorReturnIdentity { origin: self.origin()?, return_sequence: self.unsigned(u64::MAX, true)? })
    }

    fn receipt(&mut self) -> Result<ActorReturnPageReceipt, &'static str> {
        let identity = self.identity()?;
        let page_sequence = self.unsigned(u64::MAX, true)?;
        let length = self.unsigned(ACTOR_BYTE_PAGE_BYTES as u64, false)? as u32;
        let final_page = match self.byte()? { 0 => false, 1 => true, _ => return Err("actor-return.boolean") };
        let receipt = ActorReturnPageReceipt { identity, page_sequence, length, final_page };
        if !receipt.is_valid() { return Err("actor-return.receipt"); }
        Ok(receipt)
    }

    fn control(&mut self) -> Result<ActorReturnControl, &'static str> {
        match self.byte()? {
            0 => Ok(ActorReturnControl::Poll { identity: self.identity()? }),
            1 => Ok(ActorReturnControl::InputAck { receipt: self.receipt()? }),
            2 => Ok(ActorReturnControl::Cancel { identity: self.identity()? }),
            3 => Ok(ActorReturnControl::RetiredAck { identity: self.identity()? }),
            _ => Err("actor-return.control-tag"),
        }
    }

    fn finish(&self) -> Result<(), &'static str> {
        if self.offset == self.bytes.len() { Ok(()) } else { Err("actor-return.trailing") }
    }
}
//#endregion 🔢️CanonicalFields
//#endregion 📤️RetainedReturnWire
