//! 🧷️ Breach cache: the on-disk envelope, its digest and its plain and compressed encodings.

use super::{gzip_decode, gzip_encode, sha256_hex, Breach};
use serde::{Deserialize, Serialize};

//#region 🧷️Breach Cache

/// 🧷️ The on-disk JSON shape of one breach-cache document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreachCacheEnvelope {
    #[serde(rename = "entityId")]
    pub entity_id: String,
    pub script: String,
    pub breachs: Vec<Breach>,
}

/// 🔐️ The digest that keys one breach-cache document, over its canonical JSON encoding.
pub fn breach_cache_digest(envelope: &BreachCacheEnvelope) -> Result<String, String> {
    let encoded = serde_json::to_vec(envelope).map_err(|error| error.to_string())?;
    Ok(sha256_hex(&encoded))
}

/// 📄️ The canonical JSON encoding of one breach-cache document.
pub fn encode_breach_cache_json(envelope: &BreachCacheEnvelope) -> Result<String, String> {
    serde_json::to_string(envelope).map_err(|error| error.to_string())
}

/// 📄️ The breach-cache document one canonical JSON encoding carries.
pub fn parse_breach_cache(document: &str) -> Result<BreachCacheEnvelope, String> {
    serde_json::from_str(document).map_err(|error| error.to_string())
}

/// 🗜️ The compressed breach-cache document: gzip over its canonical JSON encoding.
pub fn encode_breach_cache(envelope: &BreachCacheEnvelope) -> Result<Vec<u8>, String> {
    let encoded = serde_json::to_vec(envelope).map_err(|error| error.to_string())?;
    Ok(gzip_encode(&encoded))
}

/// 📂️ The breach-cache document a compressed member carries.
pub fn decode_breach_cache(data: &[u8]) -> Result<BreachCacheEnvelope, String> {
    let payload = gzip_decode(data)?;
    serde_json::from_slice(&payload).map_err(|error| error.to_string())
}

/// 📂️ Every breach of a set of plain-JSON breach-cache documents, in document order.
pub fn breachs_from_cache(documents: &[String]) -> Result<Vec<Breach>, String> {
    let mut all = Vec::new();
    for document in documents {
        let envelope: BreachCacheEnvelope = serde_json::from_str(document).map_err(|error| error.to_string())?;
        all.extend(envelope.breachs);
    }
    Ok(all)
}

//#endregion 🧷️Breach Cache
