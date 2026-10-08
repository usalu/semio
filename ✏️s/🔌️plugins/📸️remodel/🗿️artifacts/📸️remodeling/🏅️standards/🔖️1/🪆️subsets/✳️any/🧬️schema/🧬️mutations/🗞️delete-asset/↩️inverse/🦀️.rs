//! ↩️ Inverse for `DeleteAsset` — recreates the captured BASE `ImageAsset`, which restores BOTH the
//! `assets` entry and the durable leaf the delete dropped with it. The payload is reconstituted from
//! the DOCUMENT's own durable leaf for that handle (`remodeling_asset`, `🦀️.rs:258` — the leaves this
//! very verb removes), so this inverse is a pure function of `base` with no process state behind it.
//! A key whose leaf the document does not carry ⇒ `Vec::new()`, never fabricated bytes.
use crate::mutations::{append_content, rebind_asset, AppendContent, RemodelingMutation};
use crate::{RemodelingContentKind, RemodelingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteAsset, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    let Some(handle) = base.assets.get(&payload.key) else {
        return Ok(Vec::new());
    };
    let mut undo = vec![rebind_asset(base, &payload.key, Some(&handle.child_id))];
    if let Some(artifact) = base.durable_artifacts.get(&handle.child_id) {
        if let Some(kind) = RemodelingContentKind::parse(&artifact.kind) {
            undo.push(append_content(AppendContent { content_id: handle.child_id.clone(), kind, mime: artifact.mime.clone(), width: artifact.width, height: artifact.height, first: 0, chunks: artifact.chunks.clone() }));
        }
    }
    Ok(undo)
}
//#endregion 🔖️Inverse
