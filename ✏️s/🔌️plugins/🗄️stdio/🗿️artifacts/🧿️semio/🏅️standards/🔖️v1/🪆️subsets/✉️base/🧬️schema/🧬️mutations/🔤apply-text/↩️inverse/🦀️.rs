//! ↩️ Inverse for `ApplyText`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_text_mutations::schema::mutations::SemioTextMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_text;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyText, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_text::ApplyText>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Text(b) => <SemioTextMutation as Mutation<SemioTextSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyText { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
