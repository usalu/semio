//! ↩️ Inverse for `SetRunStyle`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::support::runs_of;
#[allow(unused_imports)]
use crate::schema::mutations::set_run_style;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetRunStyle, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_run_style::SetRunStyle>,
{
    let super::SetRunStyle { path, run_index, .. } = payload;
    Ok(match block_at(base, path).and_then(runs_of).and_then(|r| r.get(*run_index)) {
        Some(run) => vec![Op::from(set_run_style::SetRunStyle { path: path.clone(), run_index: *run_index, style: run.style.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
