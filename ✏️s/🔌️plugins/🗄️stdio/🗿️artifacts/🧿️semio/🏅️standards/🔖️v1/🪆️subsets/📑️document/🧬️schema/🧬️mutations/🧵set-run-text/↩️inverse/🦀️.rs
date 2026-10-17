//! ↩️ Inverse for `SetRunText`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::support::runs_of;
#[allow(unused_imports)]
use crate::schema::mutations::set_run_text;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetRunText, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_run_text::SetRunText>,
{
    let super::SetRunText { path, run_index, .. } = payload;
    Ok(match block_at(base, path).and_then(runs_of).and_then(|r| r.get(*run_index)) {
        Some(run) => vec![Op::from(set_run_text::SetRunText { path: path.clone(), run_index: *run_index, text: run.text.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
