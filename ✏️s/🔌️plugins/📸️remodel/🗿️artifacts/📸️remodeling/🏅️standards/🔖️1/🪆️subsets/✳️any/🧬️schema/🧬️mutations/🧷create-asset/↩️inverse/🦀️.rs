//! ↩️ Inverse for `CreateAsset` — an overwrite's inverse is "recreate the OLD value" (same verb, which
//! re-mints the old durable leaf and drops the one this step minted); a fresh key's inverse is
//! `delete-asset`, which removes the `assets` entry AND the durable leaf the forward step minted, so
//! the pair is symmetric in both lanes. The OLD payload is reconstituted from the DOCUMENT's own
//! durable leaf for the overwritten handle (`remodeling_asset`, `🦀️.rs:258`), so this inverse is a
//! pure function of `base`. A document that carries the handle but not its leaf ⇒ `Vec::new()`, never
//! fabricated bytes.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::{remodeling_asset, RemodelingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateAsset, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.assets.get(&payload.key) {
        Some(_) => match remodeling_asset(base, &payload.key) {
            Some(old) => vec![super::create_asset(payload.key.clone(), old)],
            None => Vec::new(),
        },
        None => vec![crate::artifacts::remodeling::mutations::delete_asset::delete_asset(payload.key.clone())],
    }
}
//#endregion 🔖️Inverse
