//! 🏷️ `ComponentReceiptV1`, the Rust projection of [`🔣️.json`](🔣️.json): the generated, never committed
//! binding of one component's semantic descriptor to the exact bytes one build profile produced. It is the only byte identity a
//! descriptor has (ticket 26/09/23 wave A2): the trusted catalog stores it per package, and every document-open package projection
//! carries it to the hosts that execute the bytes. The language-agnostic law is [`../🧫️fixtures/🔣️.json`](../🧫️fixtures/🔣️.json), shared
//! with the TypeScript projection [`🟦️.ts`](🟦️.ts).

use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

/// 🏷️ The schema tag every receipt carries (`ComponentReceiptV1.schema`).
pub const COMPONENT_RECEIPT_SCHEMA: &str = "semio.plugin.component-receipt/v1";
/// 🎯️ The one component target triple (`ComponentReceiptV1.target`).
pub const COMPONENT_TARGET: &str = "wasm32-wasip2";
/// 🧱️ The 256 MiB bound of the one shipped profile (`ComponentProfilePolicies.release.maximumBytes`).
pub const SHIPPED_COMPONENT_MAX_BYTES: u64 = 268_435_456;
/// 🧯️ The local-only 512 MiB cap of `prototype` (`ComponentProfilePolicies.prototype.maximumBytes`).
pub const ITERATE_COMPONENT_MAX_BYTES: u64 = 536_870_912;
/// 🪶️ The fewest bytes a component can have (`ComponentReceiptV1.wasmBytes.minimum`).
pub const COMPONENT_RECEIPT_MIN_WASM_BYTES: u64 = 8;
/// 📏️ The most bytes one canonical receipt file may have.
pub const COMPONENT_RECEIPT_MAX_BYTES: usize = 4096;
/// 🗂️ The receipt's file name beside the bytes it names, in a deliverable directory and in a plugin module.
pub const COMPONENT_RECEIPT_FILE: &str = "🧾️receipt.json";

/// 🎚️ `ComponentProfile`: `release` ships; `prototype` is the local single-plugin loop whose bytes never ship.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
pub enum ComponentProfileV1 {
    #[serde(rename = "release")]
    #[value(rename = "release")]
    Release,
    #[serde(rename = "prototype")]
    #[value(rename = "prototype")]
    Prototype,
}

impl ComponentProfileV1 {
    /// 📐️ The profile's byte cap from `ComponentProfilePolicies`.
    pub const fn maximum_bytes(self) -> u64 {
        match self {
            Self::Release => SHIPPED_COMPONENT_MAX_BYTES,
            Self::Prototype => ITERATE_COMPONENT_MAX_BYTES,
        }
    }

    /// 🚢️ Whether bytes of this profile may ship (`ComponentProfilePolicies.<profile>.shipped`).
    pub const fn shipped(self) -> bool {
        matches!(self, Self::Release)
    }
}

/// 🧾️ One component's binding of its semantic descriptor to the bytes one profile produced.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComponentReceiptV1 {
    pub schema: String,
    pub plugin_id: String,
    pub package_id: String,
    pub profile: ComponentProfileV1,
    pub target: String,
    pub wasm_sha256: String,
    pub wasm_bytes: u64,
    pub semantic_descriptor_sha256: String,
    pub toolchain: String,
    pub source_tree_digest: String,
    pub provenance_sha256: String,
}

/// 🚫️ Why a receipt was refused: the first schema rule or binding law it breaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentReceiptRefusalV1(pub &'static str);

impl std::fmt::Display for ComponentReceiptRefusalV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "component receipt {}", self.0)
    }
}

impl std::error::Error for ComponentReceiptRefusalV1 {}

fn kebab(value: &str) -> bool {
    !value.is_empty() && value.split('-').all(|segment| !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

fn sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn toolchain(value: &str) -> bool {
    let Some((release, commit)) = value.strip_prefix("rustc ").and_then(|rest| rest.split_once(' ')) else { return false };
    let (version, channel) = release.split_once('-').map_or((release, None), |(version, channel)| (version, Some(channel)));
    let numbers: Vec<&str> = version.split('.').collect();
    numbers.len() == 3
        && numbers.iter().all(|number| !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit()))
        && channel.is_none_or(|channel| !channel.is_empty() && channel.bytes().all(|byte| byte.is_ascii_lowercase()))
        && commit.strip_prefix('(').and_then(|hash| hash.strip_suffix(')')).is_some_and(|hash| hash.len() == 40 && hash.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
}

impl ComponentReceiptV1 {
    /// ✅️ Holds every schema rule: exact tags, kebab identities within their bounds, lowercase digests, a byte count inside the
    /// profile's cap and a `rustc <release> (<commit>)` toolchain.
    pub fn validate(&self) -> Result<(), ComponentReceiptRefusalV1> {
        let refused = |reason| Err(ComponentReceiptRefusalV1(reason));
        if self.schema != COMPONENT_RECEIPT_SCHEMA {
            return refused("schema is not semio.plugin.component-receipt/v1");
        }
        if self.plugin_id.len() > 128 || !kebab(&self.plugin_id) {
            return refused("pluginId is not a plugin id");
        }
        if self.package_id.len() > 134 || !self.package_id.strip_prefix("semio:").is_some_and(kebab) {
            return refused("packageId is not a package id");
        }
        if self.target != COMPONENT_TARGET {
            return refused("target is not wasm32-wasip2");
        }
        if ![&self.wasm_sha256, &self.semantic_descriptor_sha256, &self.source_tree_digest, &self.provenance_sha256].into_iter().all(|digest| sha256(digest)) {
            return refused("digest is not a lowercase SHA-256");
        }
        if !(COMPONENT_RECEIPT_MIN_WASM_BYTES..=self.profile.maximum_bytes()).contains(&self.wasm_bytes) {
            return refused("wasmBytes is outside its profile's bound");
        }
        if !toolchain(&self.toolchain) {
            return refused("toolchain is not a rustc release and commit hash");
        }
        Ok(())
    }

    /// 🔗️ Requires a valid shipped receipt naming exactly this package, these component bytes and this semantic descriptor.
    pub fn binds(&self, plugin_id: &str, package_id: &str, wasm_sha256: &str, semantic_descriptor_sha256: &str) -> Result<(), ComponentReceiptRefusalV1> {
        self.validate()?;
        if !self.profile.shipped() {
            return Err(ComponentReceiptRefusalV1("does not name shipped release bytes"));
        }
        if self.plugin_id != plugin_id || self.package_id != package_id {
            return Err(ComponentReceiptRefusalV1("names another package"));
        }
        if self.wasm_sha256 != wasm_sha256 {
            return Err(ComponentReceiptRefusalV1("names other component bytes"));
        }
        if self.semantic_descriptor_sha256 != semantic_descriptor_sha256 {
            return Err(ComponentReceiptRefusalV1("names another semantic descriptor"));
        }
        Ok(())
    }

    /// 🖋️ The one canonical encoding: schema key order, two-space indentation, one trailing newline.
    pub fn encode_canonical(&self) -> Vec<u8> {
        let mut bytes = serde_json::to_vec_pretty(self).expect("a component receipt is plain JSON");
        bytes.push(b'\n');
        bytes
    }

    /// 🔐️ Strict-decodes one receipt file: bounded, schema-valid and byte-equal to its own canonical encoding.
    pub fn decode_canonical(bytes: &[u8]) -> Result<Self, ComponentReceiptRefusalV1> {
        if bytes.is_empty() || bytes.len() > COMPONENT_RECEIPT_MAX_BYTES {
            return Err(ComponentReceiptRefusalV1("file exceeds its bound"));
        }
        let receipt: Self = serde_json::from_slice(bytes).map_err(|_| ComponentReceiptRefusalV1("is not a closed receipt object"))?;
        receipt.validate()?;
        if receipt.encode_canonical() != bytes {
            return Err(ComponentReceiptRefusalV1("is not in its canonical encoding"));
        }
        Ok(receipt)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
