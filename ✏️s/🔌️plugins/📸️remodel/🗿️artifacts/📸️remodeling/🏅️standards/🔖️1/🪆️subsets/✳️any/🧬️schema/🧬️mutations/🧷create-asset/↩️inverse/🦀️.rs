//! ↩️ Inverse for `CreateAsset` — an overwrite's inverse is "recreate the OLD value" (same verb, which
//! re-mints the old durable leaf and drops the one this step minted); a fresh key's inverse is
//! `delete-asset`, which removes the `assets` entry AND the durable leaf the forward step minted, so
//! the pair is symmetric in both lanes. The OLD payload is reconstituted from the DOCUMENT's own
//! durable leaf for the overwritten handle (`remodeling_asset`, `🦀️.rs:258`), so this inverse is a
//! pure function of `base`. A document that carries the handle but not its leaf ⇒ `Vec::new()`, never
//! fabricated bytes.
use crate::mutations::{append_content, rebind_asset, remove_content, AppendContent, RemodelingMutation};
use crate::{durable_remodeling_asset, store_remodeling_asset, RemodelingContentKind, RemodelingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateAsset, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    let Some(artifact) = durable_remodeling_asset(&payload.asset) else {
        return Ok(Vec::new());
    };
    let Some(previous) = base.assets.get(&payload.key) else {
        return Ok(vec![crate::mutations::delete_asset::delete_asset(payload.key.clone())]);
    };
    let created = store_remodeling_asset(&payload.key, &payload.asset);
    let released = base.durable_artifacts.get(&previous.child_id);
    let shared = created.child_id != previous.child_id && base.durable_artifacts.get(&created.child_id) == Some(&artifact);
    let mut undo = vec![rebind_asset(base, &payload.key, Some(&previous.child_id))];
    if let Some(held) = released {
        if let Some(kind) = RemodelingContentKind::parse(&held.kind) {
            undo.push(append_content(AppendContent { content_id: previous.child_id.clone(), kind, mime: held.mime.clone(), width: held.width, height: held.height, first: 0, chunks: held.chunks.clone() }));
        }
    }
    if !shared {
        undo.push(remove_content(created.child_id, 0));
    }
    Ok(undo)
}
//#endregion 🔖️Inverse
