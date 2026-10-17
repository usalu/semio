//! ↩️ Inverse for `ApplyPresentation`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_presentation::schema::mutations::SemioPresentationMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_presentation;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyPresentation, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_presentation::ApplyPresentation>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Presentation(b) => <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyPresentation { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
