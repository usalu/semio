//! ↩️ Inverse for `ApplyVideo`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_video_mutations::schema::mutations::SemioVideoMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_video;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyVideo, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_video::ApplyVideo>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Video(b) => <SemioVideoMutation as ColdMutation<SemioVideoSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyVideo { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
