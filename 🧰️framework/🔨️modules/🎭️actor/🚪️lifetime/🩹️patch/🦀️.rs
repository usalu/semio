//#region 🩹️IssuedPatchReceipt
use super::{decimal_generation, read_unsigned, ActorInstanceLifetime};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

pub const ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES: usize = 35;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(crate = "::protocol::value", rename_all = "camelCase", deny_unknown_fields)]
pub struct ActorUiPatchReceipt {
    pub lifetime: ActorInstanceLifetime,
    #[serde(with = "decimal_generation")]
    #[value(with = "decimal_generation")]
    pub patch_sequence: u64,
}

impl ActorUiPatchReceipt {
    pub fn is_valid(self) -> bool { self.lifetime.is_valid() && self.patch_sequence != 0 }

    /// 📤️ Encodes one issued authority without touching the destination on invalid input.
    pub fn encode(self, output: &mut [u8; ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES]) -> Result<usize, &'static str> {
        if !self.is_valid() { return Err("actor-patch.invalid-authority"); }
        let mut offset = 0;
        for mut value in [self.lifetime.activation_generation, u64::from(self.lifetime.instance_id), self.lifetime.guest_lifetime, self.patch_sequence] {
            loop {
                let byte = (value & 127) as u8;
                value >>= 7;
                output[offset] = byte | if value == 0 { 0 } else { 128 };
                offset += 1;
                if value == 0 { break; }
            }
        }
        Ok(offset)
    }

    /// 📥️ Reads exactly four canonical unsigned fields from a bounded borrowed envelope.
    pub fn decode(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.is_empty() || bytes.len() > ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES { return Err("actor-patch.envelope"); }
        let mut offset = 0;
        let activation_generation = read_unsigned(bytes, &mut offset, u64::MAX, true)?;
        let instance_id = read_unsigned(bytes, &mut offset, u64::from(u32::MAX), false)? as u32;
        let guest_lifetime = read_unsigned(bytes, &mut offset, u64::MAX, true)?;
        let patch_sequence = read_unsigned(bytes, &mut offset, u64::MAX, true)?;
        if offset != bytes.len() { return Err("actor-patch.trailing"); }
        Ok(Self { lifetime: ActorInstanceLifetime { activation_generation, instance_id, guest_lifetime }, patch_sequence })
    }

    /// 🔗️ The typed producer and consumer each validate the exact logical patch count.
    pub fn validate_pairing(receipt: Option<Self>, patch_count: usize) -> Result<(), &'static str> {
        match (patch_count, receipt) {
            (0, None) => Ok(()),
            (1, Some(receipt)) if receipt.is_valid() => Ok(()),
            _ => Err("actor-patch.unpaired-authority"),
        }
    }
}
//#endregion 🩹️IssuedPatchReceipt
