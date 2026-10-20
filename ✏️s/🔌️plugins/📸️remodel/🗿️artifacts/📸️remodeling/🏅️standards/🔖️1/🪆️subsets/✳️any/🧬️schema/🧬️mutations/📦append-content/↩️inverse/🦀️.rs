//! ↩️ Inverse for `AppendContent` — remove every leaf past the leaf count the BASE stored (0 removes
//! the entry); an append that adds nothing inverts to nothing.
use crate::standards::v1::subsets::any::schema::mutations::remove_content::RemoveContent;
use crate::standards::v1::subsets::any::schema::mutations::remove_content::remove_content;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<RemoveContent>>(payload: &crate::standards::v1::subsets::any::schema::mutations::append_content::component::AppendContent, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let stored = base.durable_artifacts.get(&payload.content_id).map_or(0, |artifact| artifact.chunks.len() as u64);
    if payload.first + payload.chunks.len() as u64 <= stored {
        return Vec::new();
    }
    vec![remove_content(payload.content_id.clone(), stored)]

    })())
}
//#endregion 🔖️Inverse
