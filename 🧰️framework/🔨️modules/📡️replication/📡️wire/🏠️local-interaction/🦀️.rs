//! 🏠️ Exact local interaction transport values; live retained owners are separate from cold value composition.

use std::collections::BTreeMap;
use crate::{DomainSelection, SelectionMode};

#[path = "📡️transport/🦀️.rs"]
pub mod transport;
pub use transport::*;

#[path = "🌳️root/🦀️.rs"]
mod retained_root;
pub use retained_root::{LocalInteractionRoot, LocalInteractionRootPatch, LocalInteractionRootRetirement, LocalInteractionRootStep, LocalInteractionRootUpdate, LocalInteractionUpdateStep};

//#region 🧬️Contract
/// 🗺️ The complete local selection/mode/granularity state, excluding current hover and peer filtering.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalInteractionState {
    pub selection: BTreeMap<String, DomainSelection>,
    pub active_mode: BTreeMap<String, SelectionMode>,
    pub active_granularity: BTreeMap<String, String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin in
/// `🎮️mutation/🦀️.rs` documents (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01). Mirrors `#[serde(rename_all = "camelCase", deny_unknown_fields)]` byte-for-byte: all
/// three fields required, any other key is a decode error.
impl crate::value::ToValue for LocalInteractionState {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("selection".to_string(), crate::value::ToValue::to_value(&self.selection)),
            ("activeMode".to_string(), crate::value::ToValue::to_value(&self.active_mode)),
            ("activeGranularity".to_string(), crate::value::ToValue::to_value(&self.active_granularity)),
        ])
    }
}
impl crate::value::FromValue for LocalInteractionState {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionState, found {value:?}")));
        };
        let mut selection = None;
        let mut active_mode = None;
        let mut active_granularity = None;
        for (key, entry) in fields {
            match key.as_str() {
                "selection" => selection = Some(<BTreeMap<String, DomainSelection> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("selection"))?),
                "activeMode" => active_mode = Some(<BTreeMap<String, SelectionMode> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("activeMode"))?),
                "activeGranularity" => active_granularity = Some(<BTreeMap<String, String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("activeGranularity"))?),
                other => return Err(crate::value::ValueError::new(format!("unknown field `{other}` for LocalInteractionState"))),
            }
        }
        Ok(LocalInteractionState {
            selection: selection.ok_or_else(|| crate::value::ValueError::new("LocalInteractionState missing selection"))?,
            active_mode: active_mode.ok_or_else(|| crate::value::ValueError::new("LocalInteractionState missing activeMode"))?,
            active_granularity: active_granularity.ok_or_else(|| crate::value::ValueError::new("LocalInteractionState missing activeGranularity"))?,
        })
    }
}

/// 🔐️ Publication authority binds exact interaction, document, and topology identities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalInteractionIdentity {
    pub app_instance_id: u32,
    pub generation: u64,
    pub revision: [u8; 32],
    pub document_revision: [u8; 32],
    pub topology_revision: [u8; 32],
}

/// 🌱️ Hand-written, not derived — same reason as `LocalInteractionState` above. `generation`
/// mirrors `#[serde(with = "decimal_u64")]` (a canonical decimal string, lossless past JS's 2^53
/// float-precision boundary); `revision`/`document_revision`/`topology_revision` mirror
/// `#[serde(with = "revision_hex")]` (64 lowercase hex chars). `deny_unknown_fields` enforced.
impl crate::value::ToValue for LocalInteractionIdentity {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("appInstanceId".to_string(), crate::value::ToValue::to_value(&self.app_instance_id)),
            ("generation".to_string(), encode_decimal_u64(self.generation)),
            ("revision".to_string(), encode_revision_hex(&self.revision)),
            ("documentRevision".to_string(), encode_revision_hex(&self.document_revision)),
            ("topologyRevision".to_string(), encode_revision_hex(&self.topology_revision)),
        ])
    }
}
impl crate::value::FromValue for LocalInteractionIdentity {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionIdentity, found {value:?}")));
        };
        let mut app_instance_id = None;
        let mut generation = None;
        let mut revision = None;
        let mut document_revision = None;
        let mut topology_revision = None;
        for (key, entry) in fields {
            match key.as_str() {
                "appInstanceId" => app_instance_id = Some(<u32 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("appInstanceId"))?),
                "generation" => generation = Some(decode_decimal_u64(entry).map_err(|e| e.under("generation"))?),
                "revision" => revision = Some(decode_revision_hex(entry).map_err(|e| e.under("revision"))?),
                "documentRevision" => document_revision = Some(decode_revision_hex(entry).map_err(|e| e.under("documentRevision"))?),
                "topologyRevision" => topology_revision = Some(decode_revision_hex(entry).map_err(|e| e.under("topologyRevision"))?),
                other => return Err(crate::value::ValueError::new(format!("unknown field `{other}` for LocalInteractionIdentity"))),
            }
        }
        Ok(LocalInteractionIdentity {
            app_instance_id: app_instance_id.ok_or_else(|| crate::value::ValueError::new("LocalInteractionIdentity missing appInstanceId"))?,
            generation: generation.ok_or_else(|| crate::value::ValueError::new("LocalInteractionIdentity missing generation"))?,
            revision: revision.ok_or_else(|| crate::value::ValueError::new("LocalInteractionIdentity missing revision"))?,
            document_revision: document_revision.ok_or_else(|| crate::value::ValueError::new("LocalInteractionIdentity missing documentRevision"))?,
            topology_revision: topology_revision.ok_or_else(|| crate::value::ValueError::new("LocalInteractionIdentity missing topologyRevision"))?,
        })
    }
}

/// 🩹️ Every nullable field is required; null removes only that domain entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalInteractionDomainPatch {
    pub selection: Option<DomainSelection>,
    pub active_mode: Option<SelectionMode>,
    pub active_granularity: Option<String>,
}

/// 🌱️ Hand-written, not derived — same reason as `LocalInteractionState` above. Mirrors
/// `#[serde(deserialize_with = "required_nullable")]`: the KEY must be present (missing ⇒ decode
/// error) even though its VALUE may be `null` (⇒ `None`) — distinct from an ordinary `Option<T>`
/// field, where a missing key and a `null` value both decode to `None`. Tracked with a nested
/// `Option<Option<T>>` accumulator so "absent" and "present-but-null" stay distinguishable.
impl crate::value::ToValue for LocalInteractionDomainPatch {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("selection".to_string(), crate::value::ToValue::to_value(&self.selection)),
            ("activeMode".to_string(), crate::value::ToValue::to_value(&self.active_mode)),
            ("activeGranularity".to_string(), crate::value::ToValue::to_value(&self.active_granularity)),
        ])
    }
}
impl crate::value::FromValue for LocalInteractionDomainPatch {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionDomainPatch, found {value:?}")));
        };
        let mut selection: Option<Option<DomainSelection>> = None;
        let mut active_mode: Option<Option<SelectionMode>> = None;
        let mut active_granularity: Option<Option<String>> = None;
        for (key, entry) in fields {
            match key.as_str() {
                "selection" => selection = Some(<Option<DomainSelection> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("selection"))?),
                "activeMode" => active_mode = Some(<Option<SelectionMode> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("activeMode"))?),
                "activeGranularity" => active_granularity = Some(<Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("activeGranularity"))?),
                other => return Err(crate::value::ValueError::new(format!("unknown field `{other}` for LocalInteractionDomainPatch"))),
            }
        }
        Ok(LocalInteractionDomainPatch {
            selection: selection.ok_or_else(|| crate::value::ValueError::new("LocalInteractionDomainPatch missing selection"))?,
            active_mode: active_mode.ok_or_else(|| crate::value::ValueError::new("LocalInteractionDomainPatch missing activeMode"))?,
            active_granularity: active_granularity.ok_or_else(|| crate::value::ValueError::new("LocalInteractionDomainPatch missing activeGranularity"))?,
        })
    }
}

/// 📸️ One exact captured immutable state and its authority identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalInteractionCapture {
    pub identity: LocalInteractionIdentity,
    pub state: LocalInteractionState,
}

/// 🌱️ Hand-written, not derived — same reason as `LocalInteractionState` above.
impl crate::value::ToValue for LocalInteractionCapture {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("identity".to_string(), crate::value::ToValue::to_value(&self.identity)), ("state".to_string(), crate::value::ToValue::to_value(&self.state))])
    }
}
impl crate::value::FromValue for LocalInteractionCapture {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionCapture, found {value:?}")));
        };
        let mut identity = None;
        let mut state = None;
        for (key, entry) in fields {
            match key.as_str() {
                "identity" => identity = Some(<LocalInteractionIdentity as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("identity"))?),
                "state" => state = Some(<LocalInteractionState as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("state"))?),
                other => return Err(crate::value::ValueError::new(format!("unknown field `{other}` for LocalInteractionCapture"))),
            }
        }
        Ok(LocalInteractionCapture {
            identity: identity.ok_or_else(|| crate::value::ValueError::new("LocalInteractionCapture missing identity"))?,
            state: state.ok_or_else(|| crate::value::ValueError::new("LocalInteractionCapture missing state"))?,
        })
    }
}

/// 🔁️ Exact full or sparse restoration; base must be current, never a historical tutorial identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalInteractionRestore {
    Full { base: LocalInteractionIdentity, state: LocalInteractionState },
    Domains { base: LocalInteractionIdentity, domains: BTreeMap<String, LocalInteractionDomainPatch> },
}

/// 🌱️ Hand-written, not derived — same reason as `LocalInteractionState` above. Internally tagged
/// on `"kind"`, mirroring `#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]`.
impl crate::value::ToValue for LocalInteractionRestore {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            LocalInteractionRestore::Full { base, state } => crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("full".to_string())),
                ("base".to_string(), crate::value::ToValue::to_value(base)),
                ("state".to_string(), crate::value::ToValue::to_value(state)),
            ]),
            LocalInteractionRestore::Domains { base, domains } => crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("domains".to_string())),
                ("base".to_string(), crate::value::ToValue::to_value(base)),
                ("domains".to_string(), crate::value::ToValue::to_value(domains)),
            ]),
        }
    }
}
impl crate::value::FromValue for LocalInteractionRestore {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionRestore, found {value:?}")));
        };
        let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = match get("kind") {
            Some(crate::value::DslValue::String(s)) => s,
            _ => return Err(crate::value::ValueError::new("LocalInteractionRestore missing kind")),
        };
        let known: &[&str] = match kind.as_str() { "full" => &["kind", "base", "state"], "domains" => &["kind", "base", "domains"], _ => &["kind"] };
        if let Some((unknown, _)) = fields.iter().find(|(k, _)| !known.contains(&k.as_str())) {
            return Err(crate::value::ValueError::new(format!("unknown field `{unknown}` for LocalInteractionRestore")));
        }
        match kind.as_str() {
            "full" => Ok(LocalInteractionRestore::Full {
                base: <LocalInteractionIdentity as crate::value::FromValue>::from_value(get("base").ok_or_else(|| crate::value::ValueError::new("LocalInteractionRestore.full missing base"))?).map_err(|e| e.under("base"))?,
                state: <LocalInteractionState as crate::value::FromValue>::from_value(get("state").ok_or_else(|| crate::value::ValueError::new("LocalInteractionRestore.full missing state"))?).map_err(|e| e.under("state"))?,
            }),
            "domains" => Ok(LocalInteractionRestore::Domains {
                base: <LocalInteractionIdentity as crate::value::FromValue>::from_value(get("base").ok_or_else(|| crate::value::ValueError::new("LocalInteractionRestore.domains missing base"))?).map_err(|e| e.under("base"))?,
                domains: <BTreeMap<String, LocalInteractionDomainPatch> as crate::value::FromValue>::from_value(get("domains").ok_or_else(|| crate::value::ValueError::new("LocalInteractionRestore.domains missing domains"))?).map_err(|e| e.under("domains"))?,
            }),
            other => Err(crate::value::ValueError::new(format!("unknown LocalInteractionRestore kind `{other}`"))),
        }
    }
}

/// 📃️ Query page transport; admission checks the fixed 4096-byte maximum before owning a page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalInteractionQueryToken {
    pub request_id: u64,
    pub query_generation: u64,
    pub identity: LocalInteractionIdentity,
    pub ordinal: u64,
}

/// 🌱️ Hand-written, not derived — same reason as `LocalInteractionState` above. `request_id`/
/// `query_generation`/`ordinal` mirror `#[serde(with = "decimal_u64")]`.
impl crate::value::ToValue for LocalInteractionQueryToken {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("requestId".to_string(), encode_decimal_u64(self.request_id)),
            ("queryGeneration".to_string(), encode_decimal_u64(self.query_generation)),
            ("identity".to_string(), crate::value::ToValue::to_value(&self.identity)),
            ("ordinal".to_string(), encode_decimal_u64(self.ordinal)),
        ])
    }
}
impl crate::value::FromValue for LocalInteractionQueryToken {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionQueryToken, found {value:?}")));
        };
        let mut request_id = None;
        let mut query_generation = None;
        let mut identity = None;
        let mut ordinal = None;
        for (key, entry) in fields {
            match key.as_str() {
                "requestId" => request_id = Some(decode_decimal_u64(entry).map_err(|e| e.under("requestId"))?),
                "queryGeneration" => query_generation = Some(decode_decimal_u64(entry).map_err(|e| e.under("queryGeneration"))?),
                "identity" => identity = Some(<LocalInteractionIdentity as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("identity"))?),
                "ordinal" => ordinal = Some(decode_decimal_u64(entry).map_err(|e| e.under("ordinal"))?),
                other => return Err(crate::value::ValueError::new(format!("unknown field `{other}` for LocalInteractionQueryToken"))),
            }
        }
        Ok(LocalInteractionQueryToken {
            request_id: request_id.ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryToken missing requestId"))?,
            query_generation: query_generation.ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryToken missing queryGeneration"))?,
            identity: identity.ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryToken missing identity"))?,
            ordinal: ordinal.ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryToken missing ordinal"))?,
        })
    }
}

/// 📃️ Fixed-admission query page; generation distinguishes requests across app lifetime reuse.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalInteractionPage {
    pub request_id: u64,
    pub query_generation: u64,
    pub identity: LocalInteractionIdentity,
    pub ordinal: u64,
    pub terminal: bool,
    pub bytes: Vec<u8>,
}

/// 🌱️ Hand-written, not derived — same reason as `LocalInteractionQueryToken` above.
impl crate::value::ToValue for LocalInteractionPage {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("requestId".to_string(), encode_decimal_u64(self.request_id)),
            ("queryGeneration".to_string(), encode_decimal_u64(self.query_generation)),
            ("identity".to_string(), crate::value::ToValue::to_value(&self.identity)),
            ("ordinal".to_string(), encode_decimal_u64(self.ordinal)),
            ("terminal".to_string(), crate::value::ToValue::to_value(&self.terminal)),
            ("bytes".to_string(), crate::value::ToValue::to_value(&self.bytes)),
        ])
    }
}
impl crate::value::FromValue for LocalInteractionPage {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionPage, found {value:?}")));
        };
        let mut request_id = None;
        let mut query_generation = None;
        let mut identity = None;
        let mut ordinal = None;
        let mut terminal = None;
        let mut bytes = None;
        for (key, entry) in fields {
            match key.as_str() {
                "requestId" => request_id = Some(decode_decimal_u64(entry).map_err(|e| e.under("requestId"))?),
                "queryGeneration" => query_generation = Some(decode_decimal_u64(entry).map_err(|e| e.under("queryGeneration"))?),
                "identity" => identity = Some(<LocalInteractionIdentity as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("identity"))?),
                "ordinal" => ordinal = Some(decode_decimal_u64(entry).map_err(|e| e.under("ordinal"))?),
                "terminal" => terminal = Some(<bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("terminal"))?),
                "bytes" => bytes = Some(<Vec<u8> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("bytes"))?),
                other => return Err(crate::value::ValueError::new(format!("unknown field `{other}` for LocalInteractionPage"))),
            }
        }
        Ok(LocalInteractionPage {
            request_id: request_id.ok_or_else(|| crate::value::ValueError::new("LocalInteractionPage missing requestId"))?,
            query_generation: query_generation.ok_or_else(|| crate::value::ValueError::new("LocalInteractionPage missing queryGeneration"))?,
            identity: identity.ok_or_else(|| crate::value::ValueError::new("LocalInteractionPage missing identity"))?,
            ordinal: ordinal.ok_or_else(|| crate::value::ValueError::new("LocalInteractionPage missing ordinal"))?,
            terminal: terminal.ok_or_else(|| crate::value::ValueError::new("LocalInteractionPage missing terminal"))?,
            bytes: bytes.ok_or_else(|| crate::value::ValueError::new("LocalInteractionPage missing bytes"))?,
        })
    }
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

//#region 📦️ColdValueCodec
/// 🌱️ Canonical decimal-string encoding for a `u64` that must survive JS's 2^53 float-precision
/// boundary losslessly — the first-party twin of the former `#[serde(with = "decimal_u64")]`.
fn encode_decimal_u64(value: u64) -> crate::value::DslValue {
    crate::value::DslValue::String(value.to_string())
}
fn decode_decimal_u64(value: crate::value::DslValue) -> Result<u64, crate::value::ValueError> {
    let crate::value::DslValue::String(text) = value else {
        return Err(crate::value::ValueError::new(format!("expected a decimal string, found {value:?}")));
    };
    if text.is_empty() || text.len() > 20 || (text.len() > 1 && text.starts_with('0')) || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(crate::value::ValueError::new("invalid decimal u64"));
    }
    text.parse().map_err(|_| crate::value::ValueError::new("invalid decimal u64"))
}

/// 🌱️ Canonical 64-lowercase-hex-char encoding for a `[u8; 32]` revision — the first-party twin
/// of the former `#[serde(with = "revision_hex")]`.
fn encode_revision_hex(value: &[u8; 32]) -> crate::value::DslValue {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(64);
    for byte in value { text.push(HEX[(byte >> 4) as usize] as char); text.push(HEX[(byte & 15) as usize] as char); }
    crate::value::DslValue::String(text)
}
fn decode_revision_hex(value: crate::value::DslValue) -> Result<[u8; 32], crate::value::ValueError> {
    let crate::value::DslValue::String(text) = value else {
        return Err(crate::value::ValueError::new(format!("expected a hex string, found {value:?}")));
    };
    if text.len() != 64 || !text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err(crate::value::ValueError::new("invalid full revision"));
    }
    let mut bytes = [0; 32];
    for (index, pair) in text.as_bytes().chunks_exact(2).enumerate() { bytes[index] = nibble(pair[0]) * 16 + nibble(pair[1]); }
    Ok(bytes)
}
fn nibble(byte: u8) -> u8 { if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 } }
//#endregion 📦️ColdValueCodec

//#region 🧪️TestBridge
/// 🌱️ `#[cfg(test)]`-only bridge between the `serde_json::Value` fixtures every test file in this
/// module already loads from disk and the `crate::value::DslValue` tree `ToValue`/`FromValue`
/// produce/consume — lets fixture-driven tests keep comparing against literal JSON without any
/// shipped type needing `serde::Serialize`/`Deserialize`. `serde_json` stays a dev-dependency only.
#[cfg(test)]
fn json_to_dsl(value: serde_json::Value) -> crate::value::DslValue {
    crate::value::DslValue::from(&value)
}
#[cfg(test)]
fn dsl_to_json(value: &crate::value::DslValue) -> serde_json::Value {
    serde_json::Value::from(value)
}
/// 🌱️ Test-only convenience: decode a `DslValue` (typically freshly built by `json_to_dsl`) via
/// `FromValue`, panicking with the decode error on failure (mirrors `serde_json::from_value(...)
/// .unwrap()`'s former ergonomics).
#[cfg(test)]
fn from_json<T: crate::value::FromValue>(value: serde_json::Value) -> T {
    T::from_value(json_to_dsl(value)).unwrap()
}
//#endregion 🧪️TestBridge

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🌳️root/🧪️tests/🦀️.rs"]
mod retained_root_tests;

#[cfg(test)]
#[path = "🌳️root/🩹️update/🧪️tests/🦀️.rs"]
mod retained_update_tests;
