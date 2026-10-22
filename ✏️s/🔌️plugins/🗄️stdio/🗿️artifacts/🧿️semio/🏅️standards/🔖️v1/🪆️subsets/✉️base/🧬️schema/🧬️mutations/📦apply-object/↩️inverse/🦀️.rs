//! ↩️ Inverse for `ApplyObject`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_object;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyObject, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_object::ApplyObject>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Object(b) => <SemioObjectMutation as ColdMutation<SemioObjectSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyObject { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
