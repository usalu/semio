//! ↩️ Inverse for `ApplyTable`.

#[allow(unused_imports)]
use protocol::{ColdMutation, Mutation};
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_table_mutations::schema::mutations::SemioTableMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_table;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyTable, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_table::ApplyTable>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Table(b) => <SemioTableMutation as ColdMutation<SemioTableSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyTable { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
