//! ↩️ Inverse for `DeleteAsset` — recreates the captured BASE `ImageAsset`, which restores BOTH the
//! `assets` entry and the durable leaf the delete dropped with it. The payload is reconstituted from
//! the DOCUMENT's own durable leaf for that handle (`remodeling_asset`, `🦀️.rs:258` — the leaves this
//! very verb removes), so this inverse is a pure function of `base` with no process state behind it.
//! A key whose leaf the document does not carry ⇒ `Vec::new()`, never fabricated bytes.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::{remodeling_asset, RemodelingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteAsset, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match remodeling_asset(base, &payload.key) {
        Some(old) => vec![crate::artifacts::remodeling::mutations::create_asset::create_asset(payload.key.clone(), old)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
