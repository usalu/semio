//! 🔺️ Upserts one asset binding, shares matching image content, and releases its previous leaf
//! when the binding was its last owner.
use crate::standards::v1::subsets::any::schema::mutations::delete_asset::diff::unshared_asset_content;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingAssetEntry;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingContentDelta;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingContentRow;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRow;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingRows;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::durable_remodeling_asset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::store_remodeling_asset;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::create_asset::component::CreateAsset, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let handle = store_remodeling_asset(&payload.key, &payload.asset);
    let Some(artifact) = durable_remodeling_asset(&payload.asset) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The asset payload is malformed or exceeds its exact bounded envelope.", [payload.key.clone()]);
    };
    let held = base.durable_artifacts.get(&handle.child_id);
    if held.is_some_and(|held| held != &artifact) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", "The asset content ID names different durable content.", [handle.child_id]);
    }
    let previous = base.assets.get(&payload.key);
    let mut content = Vec::new();
    if let Some(previous) = previous.filter(|previous| previous.child_id != handle.child_id && unshared_asset_content(base, &payload.key, &previous.child_id).is_some()) {
        content.push(RemodelingContentRow::Truncate { id: previous.child_id.clone(), from: 0 });
    }
    if held.is_none() {
        content.push(RemodelingContentDelta::create(&handle.child_id, &artifact));
    }
    let entry = RemodelingAssetEntry { key: payload.key.clone(), child: handle };
    let row = if previous.is_some() { RemodelingRow::Replace { entity: entry } } else { RemodelingRow::Insert { entity: entry } };
    protocol::MutationOutcome::new(RemodelingDiff { assets: Some(RemodelingRows { rows: vec![row] }), durable_artifacts: (!content.is_empty()).then_some(RemodelingContentDelta { rows: content }), ..Default::default() })
}
//#endregion 🔖️Diff
