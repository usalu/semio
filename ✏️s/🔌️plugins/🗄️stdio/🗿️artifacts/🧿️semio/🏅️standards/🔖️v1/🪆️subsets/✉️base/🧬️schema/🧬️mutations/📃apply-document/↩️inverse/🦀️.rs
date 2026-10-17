//! ↩️ Inverse for `ApplyDocument`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_document::schema::mutations::SemioDocumentMutation;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSubsetSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::apply_document;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ApplyDocument, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::apply_document::ApplyDocument>,
{
    Ok(match &base.subset {
        SemioSubsetSnapshot::Document(b) => <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::inverse(&payload.mutation, b)?.into_iter().map(|inner| Op::from(super::ApplyDocument { mutation: inner })).collect(),
        _ => Vec::new(),
    })
}
//#endregion 🔖️Inverse
