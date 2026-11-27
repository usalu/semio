//! 🧮️ Recalculation of formula caches after an edit — the one shared helper every cell-editing path (typed draft, details pane) appends to its user mutations, so a single command carries the edit and the caches it makes stale.

use super::*;
use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::calculation::recalculate_workbook;

/// 🧮️ Appends one `SetCell` per formula whose cache the `mutations` make stale, each addressed in the state its predecessors leave behind; a sequence that cannot be applied or projected is returned unchanged.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn xlsx_with_recalculated_caches(snapshot: &XlsxSnapshot, mutations: Vec<XlsxMutation>) -> Vec<XlsxMutation> {
    recalculated(snapshot, &mutations).map_or(mutations.clone(), |recalculated| [mutations, recalculated].concat())
}

fn recalculated(snapshot: &XlsxSnapshot, mutations: &[XlsxMutation]) -> Option<Vec<XlsxMutation>> {
    let mut next = snapshot.clone();
    for mutation in mutations {
        apply_checked(&mut next, mutation)?;
    }
    let mut follow_ups = Vec::new();
    for cell in recalculate_workbook(&next.project_workbook().ok()?) {
        let mutation = XlsxMutation::SetCell(set_cell::SetCell {
            address: cell_address::xlsx_cell_address(&next, &cell.sheet, cell.row, cell.col).ok()?,
            value: XlsxCellValue::Formula { expr: cell.expr, cached: Some(Box::new(cell.cached)) },
            node: None,
        });
        apply_checked(&mut next, &mutation)?;
        follow_ups.push(mutation);
    }
    Some(follow_ups)
}

fn apply_checked(snapshot: &mut XlsxSnapshot, mutation: &XlsxMutation) -> Option<()> {
    match ::semio_os_kernel::ColdMutation::diff(mutation, snapshot).admit() {
        protocol::MutationAdmission::Admitted(outcome) => {
            *snapshot = protocol::apply_diff_cold(outcome.diff(), snapshot).ok()?;
            Some(())
        }
        protocol::MutationAdmission::Refused(_) => None,
    }
}
