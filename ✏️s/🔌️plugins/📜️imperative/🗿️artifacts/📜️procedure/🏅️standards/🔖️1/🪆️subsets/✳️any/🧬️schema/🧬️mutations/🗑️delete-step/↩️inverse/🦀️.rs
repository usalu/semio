//! ↩️ Inverse for `DeleteStep` — reconstructs a `create-step` of the FULL captured BASE step at its
//! exact BASE position (its `bodies` cascade rides along inside the struct), so the `flow` child — a
//! hash over step ORDER — is restored byte-for-byte in ONE point-invertible row. Missing target ⇒
//! `Vec::new()`.
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteStep, base: &ProcedureSnapshot) -> Vec<ProcedureMutation> {
    let steps = crate::mutations::resolve_steps(base, &payload.path_ref);
    match steps.iter().position(|step| step.id == payload.id) {
        Some(index) => vec![crate::mutations::create_step::create_step_at(payload.path_ref.clone(), steps[index].clone(), index)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
