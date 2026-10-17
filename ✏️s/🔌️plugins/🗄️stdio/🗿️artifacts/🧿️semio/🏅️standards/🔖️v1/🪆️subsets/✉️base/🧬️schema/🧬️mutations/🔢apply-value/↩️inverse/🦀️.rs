//! ↩️ Inverse for `ApplyValue`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_mutations::schema::mutations::SemioValueMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_value;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyValue, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_value::ApplyValue>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Value(b) => <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyValue { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
