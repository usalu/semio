//! ⚙️ Imperative mutation bridges, shared path operations, laws, and behavior tests.

use crate::mutations::ProcedureMutation;
#[cfg(test)]
use crate::mutations::{create_step, delete_step, edit_step_params, register_procedure_mutation_descriptors, reorder_steps};
use crate::{Path, PathRef, ProcedureSnapshot, Step};

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's internally-tagged (`{"mutation": "createStep", …}`, camelCase payload
/// fields) JSON projection — exactly the shape the committed
/// `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` specification vectors and
/// `🛟️mutate-procedure-1`'s own `Examples` payloads carry — into a real [`ProcedureMutation`]. The
/// test adapter cannot name this crate's private `dsl`/`protocol`/`store` extern-crate aliases (the
/// generated host links only `semio-repo-test-host` and this crate), so the bridge belongs here
/// rather than there.
pub fn decode_procedure_mutation_json(text: &str) -> Result<ProcedureMutation, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 🌱 Resolves `snapshot`'s composed `s.stdio.semio.flow` child to the program in `program_json`
/// (a `{"steps": [...]}` `Path`). An imperative document persists only a content-addressed HANDLE,
/// and the working scene is an exact child owner, so a decoded `⬅️before` stands for no program
/// until its own child is materialized — exactly what each direct leaf's `cached_program()` does.
/// `🛟️mutate-procedure-1` needs the same materialization from outside, where neither `Path` nor its
/// `Dictionary`/`Value` argument types
/// can be constructed, so the program travels as JSON and is decoded here.
pub fn seed_procedure_flow_json(snapshot: &mut ProcedureSnapshot, program_json: &str) -> Result<(), String> {
    let path: Path = dsl::os_pack::json::from_json_str(program_json).map_err(|error| error.to_string())?;
    crate::materialize_procedure_flow(&mut snapshot.flow, &path);
    Ok(())
}

/// ▶️ Applies `mutation` in place and returns every diagnostic it raised as `(code, severity)`
/// pairs. All four committed vectors leave the document byte-identical — two refusals and two
/// `Warning`-level no-ops — so the pair is the evidence rather than a side channel.
pub fn apply_procedure_mutation_reporting(snapshot: &mut ProcedureSnapshot, mutation: &ProcedureMutation) -> Vec<(String, String)> {
    let outcome = <ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ The mutation's OWN computed undo steps, which is what an `inverse-<kind>` scenario has to
/// apply for the metamorphic law to mean anything.
pub fn inverse_procedure_mutation_steps(mutation: &ProcedureMutation, base: &ProcedureSnapshot) -> Vec<ProcedureMutation> {
    <ProcedureMutation as protocol::Mutation<ProcedureSnapshot>>::inverse(mutation, base)
}

/// 🔎️ The program the document's composed flow child currently resolves to, rendered as nested
/// `id:kind` entries in list order — the readable half of a divergence message, so a failing
/// scenario names WHICH step moved rather than only that two content digests differ.
pub fn procedure_program_summary(snapshot: &ProcedureSnapshot) -> String {
    fn render(path: &Path) -> String {
        path.steps
            .iter()
            .map(|step| {
                let bodies = step.bodies.iter().map(|(slot, body)| format!("{slot}{{{}}}", render(body))).collect::<Vec<_>>().join(" ");
                if bodies.is_empty() {
                    format!("{}:{}", step.id, step.kind)
                } else {
                    format!("{}:{}[{bodies}]", step.id, step.kind)
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    render(&crate::procedure_working_scene(snapshot).path)
}
//#endregion 🌉️ExternalCodecBridge

/// 🔎️ Resolves the step list a `PathRef` addresses (read from the live `flow` working scene, since
/// `ProcedureSnapshot` no longer carries `path` directly — ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`); a not-yet-materialized nested slot reads as
/// empty. Owned `Vec` (not a borrow) since the working scene is a cache lookup, not a live borrow of
/// `snapshot` itself. Shared by every direct leaf's `🔺️diff`/`↩️inverse` facet so base-state lookups agree.
pub fn resolve_steps(snapshot: &ProcedureSnapshot, path_ref: &PathRef) -> Vec<Step> {
    let path = crate::procedure_working_scene(snapshot).path;
    resolve_steps_in_path(&path, path_ref)
}

fn resolve_steps_in_path(path: &Path, path_ref: &PathRef) -> Vec<Step> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return path.steps.clone();
    }
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else { return Vec::new() };
    let Some(owner_step) = path.steps.iter().find(|step| &step.id == owner) else { return Vec::new() };
    owner_step.bodies.get(slot).map(|body| body.steps.clone()).unwrap_or_default()
}

/// 🔧 Resolves the MUTABLE step list at `path_ref` within a live working-scene `Path` — the
/// mutation-side counterpart of `resolve_steps`, used by every direct leaf's `🔺️diff` facet to edit a
/// full copy of the current path before re-minting a whole `flow` handle (composed children are
/// opaque; a diff never edits a sub-slice, only mints a whole replacement — see
/// `crate::diff_replace_flow`).
pub fn resolve_path_mut<'a>(path: &'a mut Path, path_ref: &PathRef) -> Option<&'a mut Vec<Step>> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&mut path.steps);
    }
    let owner = path_ref.owner.clone()?;
    let slot = path_ref.slot.clone()?;
    let owner_step = path.steps.iter_mut().find(|step| step.id == owner)?;
    Some(&mut owner_step.bodies.entry(slot).or_insert_with(Path::new).steps)
}

/// 🧹 Removes a now-empty nested body slot after a delete — mirrors the pre-migration snapshot-
/// level `prune_empty_slot` this same helper set used to own.
pub fn prune_empty_slot(path: &mut Path, path_ref: &PathRef) {
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else { return };
    if let Some(owner_step) = path.steps.iter_mut().find(|step| &step.id == owner) {
        if owner_step.bodies.get(slot).is_some_and(|body| body.steps.is_empty()) {
            owner_step.bodies.remove(slot);
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
