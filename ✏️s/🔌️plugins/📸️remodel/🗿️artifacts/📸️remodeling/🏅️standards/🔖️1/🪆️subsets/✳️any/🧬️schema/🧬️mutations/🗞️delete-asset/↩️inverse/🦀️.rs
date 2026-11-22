//! ↩️ Restores the deleted binding and, when it was released, its durable leaf.
use super::diff::unshared_asset_content;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::CommitReconstruction;
use crate::standards::v1::subsets::any::schema::mutations::append_content::append_content;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::rebind_asset;
use crate::standards::v1::subsets::any::schema::mutations::append_content::AppendContent;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingContentKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

pub(crate) fn restore_unshared_asset_content<Op: From<AppendContent>>(base: &RemodelingSnapshot, key: &str, content_id: &str) -> Option<Op> {
    let artifact = unshared_asset_content(base, key, content_id)?;
    let kind = RemodelingContentKind::parse(&artifact.kind)?;
    Some(append_content(AppendContent { content_id: content_id.to_owned(), kind, mime: artifact.mime.clone(), width: artifact.width, height: artifact.height, first: 0, chunks: artifact.chunks.clone() }))
}

//#region 🔖️Inverse
pub fn inverse<Op: From<AppendContent> + From<CommitReconstruction>>(payload: &crate::standards::v1::subsets::any::schema::mutations::delete_asset::component::DeleteAsset, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let Some(handle) = base.assets.get(&payload.key) else {
        return Ok(Vec::new());
    };
    let mut undo = vec![rebind_asset(base, &payload.key, Some(&handle.child_id))];
    undo.extend(restore_unshared_asset_content(base, &payload.key, &handle.child_id));
    Ok(undo)
}
//#endregion 🔖️Inverse
