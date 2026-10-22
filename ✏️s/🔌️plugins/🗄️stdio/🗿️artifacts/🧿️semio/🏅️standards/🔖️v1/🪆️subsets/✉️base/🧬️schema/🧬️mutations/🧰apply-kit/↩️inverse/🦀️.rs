//! ↩️ Inverse for `ApplyKit`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_kit::schema::mutations::SemioKitMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_kit;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyKit, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_kit::ApplyKit>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Kit(b) => <SemioKitMutation as ColdMutation<SemioKitSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyKit { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
