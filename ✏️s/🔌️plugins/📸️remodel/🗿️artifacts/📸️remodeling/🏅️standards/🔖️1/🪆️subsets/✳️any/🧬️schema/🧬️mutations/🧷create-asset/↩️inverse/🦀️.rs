//! ↩️ Restores the previous binding and releases only content introduced by this upsert.
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::CommitReconstruction;
use crate::standards::v1::subsets::any::schema::mutations::delete_asset::{delete_asset, inverse::restore_unshared_asset_content, DeleteAsset};
use crate::standards::v1::subsets::any::schema::mutations::remove_content::RemoveContent;
use crate::standards::v1::subsets::any::schema::mutations::remove_content::remove_content;
use semio_s_artifact_remodel_remodeling_mut_results::standards::v1::subsets::any::schema::mutations::commit_reconstruction::rebind_asset;
use crate::standards::v1::subsets::any::schema::mutations::append_content::AppendContent;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::durable_remodeling_asset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::store_remodeling_asset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<AppendContent> + From<CommitReconstruction> + From<DeleteAsset> + From<RemoveContent>>(payload: &crate::standards::v1::subsets::any::schema::mutations::create_asset::component::CreateAsset, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let Some(artifact) = durable_remodeling_asset(&payload.asset) else {
        return Ok(Vec::new());
    };
    let created = store_remodeling_asset(&payload.key, &payload.asset);
    let held = base.durable_artifacts.get(&created.child_id);
    if held.is_some_and(|held| held != &artifact) {
        return Ok(Vec::new());
    }
    let Some(previous) = base.assets.get(&payload.key) else {
        return Ok(vec![if held.is_some() { rebind_asset(base, &payload.key, None) } else { delete_asset(payload.key.clone()) }]);
    };
    let mut undo = Vec::new();
    if held.is_none() {
        undo.push(remove_content(created.child_id.clone(), 0));
    }
    if previous.child_id != created.child_id {
        undo.push(rebind_asset(base, &payload.key, Some(&previous.child_id)));
        undo.extend(restore_unshared_asset_content(base, &payload.key, &previous.child_id));
    }
    Ok(undo)
}
//#endregion 🔖️Inverse
