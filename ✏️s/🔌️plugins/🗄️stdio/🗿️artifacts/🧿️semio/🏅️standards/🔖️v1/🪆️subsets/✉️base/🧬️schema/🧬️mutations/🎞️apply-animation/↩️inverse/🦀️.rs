//! ↩️ Inverse for `ApplyAnimation`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_animation::schema::mutations::SemioAnimationMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_animation;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyAnimation, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_animation::ApplyAnimation>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Animation(b) => <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyAnimation { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
