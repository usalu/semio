//! ↩️ Inverse for `CreateAsset` — an overwrite's inverse is "recreate the OLD value" (same verb, which
//! re-mints the old durable leaf and drops the one this step minted); a fresh key's inverse is
//! `delete-asset`, which removes the `assets` entry AND the durable leaf the forward step minted, so
//! the pair is symmetric in both lanes. The OLD payload is reconstituted from the DOCUMENT's own
//! durable leaf for the overwritten handle (`remodeling_asset`, `🦀️.rs:258`), so this inverse is a
//! pure function of `base`. A document that carries the handle but not its leaf ⇒ `Vec::new()`, never
//! fabricated bytes.
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::CommitReconstruction;
use crate::standards::v1::subsets::any::schema::mutations::delete_asset::DeleteAsset;
use crate::standards::v1::subsets::any::schema::mutations::remove_content::RemoveContent;
use crate::standards::v1::subsets::any::schema::mutations::{append_content::append_content, remove_content::remove_content};
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::rebind_asset;
use crate::standards::v1::subsets::any::schema::mutations::append_content::{AppendContent};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::durable_remodeling_asset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::store_remodeling_asset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingContentKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<AppendContent> + From<CommitReconstruction> + From<DeleteAsset> + From<RemoveContent>>(payload: &super::CreateAsset, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let Some(artifact) = durable_remodeling_asset(&payload.asset) else {
        return Ok(Vec::new());
    };
    let Some(previous) = base.assets.get(&payload.key) else {
        return Ok(vec![crate::standards::v1::subsets::any::schema::mutations::delete_asset::delete_asset(payload.key.clone())]);
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
