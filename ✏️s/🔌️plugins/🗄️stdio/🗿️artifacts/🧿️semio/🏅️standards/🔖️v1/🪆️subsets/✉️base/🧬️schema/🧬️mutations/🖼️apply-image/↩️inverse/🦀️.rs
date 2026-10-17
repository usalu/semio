//! ↩️ Inverse for `ApplyImage`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_image::schema::mutations::SemioImageMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_image;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyImage, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_image::ApplyImage>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Image(b) => <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyImage { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
