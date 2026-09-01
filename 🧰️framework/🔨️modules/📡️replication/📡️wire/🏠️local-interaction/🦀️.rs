//! 🏠️ Exact local interaction transport values; live retained owners are separate from cold serde composition.

use std::collections::BTreeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::{DomainSelection, SelectionMode};

#[path = "📡️transport/🦀️.rs"]
pub mod transport;
pub use transport::*;

#[path = "🌳️root/🦀️.rs"]
mod retained_root;
pub use retained_root::{LocalInteractionRoot, LocalInteractionRootPatch, LocalInteractionRootRetirement, LocalInteractionRootStep, LocalInteractionRootUpdate, LocalInteractionUpdateStep};

//#region 🧬️Contract
/// 🗺️ The complete local selection/mode/granularity state, excluding current hover and peer filtering.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalInteractionState {
    pub selection: BTreeMap<String, DomainSelection>,
    pub active_mode: BTreeMap<String, SelectionMode>,
    pub active_granularity: BTreeMap<String, String>,
}

/// 🔐️ Publication authority binds exact interaction, document, and topology identities.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalInteractionIdentity {
    pub app_instance_id: u32,
    #[serde(with = "decimal_u64")]
    pub generation: u64,
    #[serde(with = "revision_hex")]
    pub revision: [u8; 32],
    #[serde(with = "revision_hex")]
    pub document_revision: [u8; 32],
    #[serde(with = "revision_hex")]
    pub topology_revision: [u8; 32],
}

/// 🩹️ Every nullable field is required; null removes only that domain entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalInteractionDomainPatch {
    #[serde(deserialize_with = "required_nullable")]
    pub selection: Option<DomainSelection>,
    #[serde(deserialize_with = "required_nullable")]
    pub active_mode: Option<SelectionMode>,
    #[serde(deserialize_with = "required_nullable")]
    pub active_granularity: Option<String>,
}

/// 📸️ One exact captured immutable state and its authority identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalInteractionCapture {
    pub identity: LocalInteractionIdentity,
    pub state: LocalInteractionState,
}

/// 🔁️ Exact full or sparse restoration; base must be current, never a historical tutorial identity.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum LocalInteractionRestore {
    Full { base: LocalInteractionIdentity, state: LocalInteractionState },
    Domains { base: LocalInteractionIdentity, domains: BTreeMap<String, LocalInteractionDomainPatch> },
}

/// 📃️ Query page transport; admission checks the fixed 4096-byte maximum before owning a page.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalInteractionQueryToken {
    #[serde(with = "decimal_u64")]
    pub request_id: u64,
    #[serde(with = "decimal_u64")]
    pub query_generation: u64,
    pub identity: LocalInteractionIdentity,
    #[serde(with = "decimal_u64")]
    pub ordinal: u64,
}

/// 📃️ Fixed-admission query page; generation distinguishes requests across app lifetime reuse.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalInteractionPage {
    #[serde(with = "decimal_u64")]
    pub request_id: u64,
    #[serde(with = "decimal_u64")]
    pub query_generation: u64,
    pub identity: LocalInteractionIdentity,
    #[serde(with = "decimal_u64")]
    pub ordinal: u64,
    pub terminal: bool,
    pub bytes: Vec<u8>,
}
//#endregion 🧬️Contract

//#region 🧮️ColdComposition
impl LocalInteractionRestore {
    /// 🧊️ Exact immutable authority accessor, with no lookup or cloning.
    pub fn base(&self) -> &LocalInteractionIdentity {
        match self { Self::Full { base, .. } | Self::Domains { base, .. } => base }
    }

    /// 🧊️ Cold tutorial composition only; live producers require retained topology validation and Store authority.
    pub fn apply_cold(&self, before: &LocalInteractionState, current: &LocalInteractionIdentity) -> Result<LocalInteractionState, &'static str> {
        if self.base() != current { return Err("stale-authority"); }
        match self {
            Self::Full { state, .. } => Ok(state.clone()),
            Self::Domains { domains, .. } => {
                let mut next = before.clone();
                for (domain, patch) in domains {
                    apply_field(&mut next.selection, domain, &patch.selection);
                    apply_field(&mut next.active_mode, domain, &patch.active_mode);
                    apply_field(&mut next.active_granularity, domain, &patch.active_granularity);
                }
                Ok(next)
            }
        }
    }
}

fn apply_field<T: Clone>(map: &mut BTreeMap<String, T>, domain: &str, value: &Option<T>) {
    if let Some(value) = value { map.insert(domain.to_owned(), value.clone()); }
    else { map.remove(domain); }
}
//#endregion 🧮️ColdComposition

//#region 📦️ColdJsonCodec
fn required_nullable<'de, D: Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(deserializer)
}

mod decimal_u64 {
    use super::*;
    pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> { serializer.serialize_str(&value.to_string()) }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        let text = String::deserialize(deserializer)?;
        if text.is_empty() || text.len() > 20 || (text.len() > 1 && text.starts_with('0')) || !text.bytes().all(|byte| byte.is_ascii_digit()) { return Err(serde::de::Error::custom("invalid decimal u64")); }
        text.parse().map_err(serde::de::Error::custom)
    }
}

mod revision_hex {
    use super::*;
    pub fn serialize<S: Serializer>(value: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error> {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut text = String::with_capacity(64);
        for byte in value { text.push(HEX[(byte >> 4) as usize] as char); text.push(HEX[(byte & 15) as usize] as char); }
        serializer.serialize_str(&text)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<[u8; 32], D::Error> {
        let text = String::deserialize(deserializer)?;
        if text.len() != 64 || !text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) { return Err(serde::de::Error::custom("invalid full revision")); }
        let mut bytes = [0; 32];
        for (index, pair) in text.as_bytes().chunks_exact(2).enumerate() { bytes[index] = nibble(pair[0]) * 16 + nibble(pair[1]); }
        Ok(bytes)
    }
    fn nibble(byte: u8) -> u8 { if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 } }
}
//#endregion 📦️ColdJsonCodec

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🌳️root/🧪️tests/🦀️.rs"]
mod retained_root_tests;

#[cfg(test)]
#[path = "🌳️root/🩹️update/🧪️tests/🦀️.rs"]
mod retained_update_tests;
