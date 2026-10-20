//! ↩️ Inverse for `RemoveContent` — re-append the BASE leaves from `from` on with the BASE kind and
//! presentation. An entry whose kind is not a content kind, or a removal that removes nothing, inverts to
//! nothing.
use crate::standards::v1::subsets::any::schema::mutations::append_content::append_content;
use crate::standards::v1::subsets::any::schema::mutations::append_content::{AppendContent};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingContentKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<AppendContent>>(payload: &crate::standards::v1::subsets::any::schema::mutations::remove_content::component::RemoveContent, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(artifact) = base.durable_artifacts.get(&payload.content_id) else { return Vec::new() };
    let (Some(kind), Ok(from)) = (RemodelingContentKind::parse(&artifact.kind), usize::try_from(payload.from)) else { return Vec::new() };
    if from >= artifact.chunks.len() {
        return Vec::new();
    }
    vec![append_content(AppendContent { content_id: payload.content_id.clone(), kind, mime: artifact.mime.clone(), width: artifact.width, height: artifact.height, first: payload.from, chunks: artifact.chunks[from..].to_vec() })]

    })())
}
//#endregion 🔖️Inverse
