//! ↩️ Inverse for `ApplyBrep`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_brep::schema::mutations::SemioBrepMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_brep;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyBrep, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_brep::ApplyBrep>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Brep(b) => <SemioBrepMutation as ColdMutation<SemioBrepSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyBrep { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
