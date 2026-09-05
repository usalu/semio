//! ⚙️ Imperative mutation bridges, shared path operations, laws, and behavior tests.

use crate::artifacts::procedure::mutations::{create_step, delete_step, edit_step_params, register_procedure_mutation_descriptors, reorder_steps, ProcedureMutation};
use crate::artifacts::procedure::{ProcedureSnapshot, Path, PathRef, Step};

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
    crate::artifacts::procedure::materialize_procedure_flow(&mut snapshot.flow, &path);
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
    render(&crate::artifacts::procedure::procedure_working_scene(snapshot).path)
}
//#endregion 🌉️ExternalCodecBridge

/// 🔎️ Resolves the step list a `PathRef` addresses (read from the live `flow` working scene, since
/// `ProcedureSnapshot` no longer carries `path` directly — ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`); a not-yet-materialized nested slot reads as
/// empty. Owned `Vec` (not a borrow) since the working scene is a cache lookup, not a live borrow of
/// `snapshot` itself. Shared by every direct leaf's `🔺️diff`/`↩️inverse` facet so base-state lookups agree.
pub fn resolve_steps(snapshot: &ProcedureSnapshot, path_ref: &PathRef) -> Vec<Step> {
    let path = crate::artifacts::procedure::procedure_working_scene(snapshot).path;
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
/// `crate::artifacts::procedure::diff_replace_flow`).
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
mod tests {
    use super::*;
    use crate::artifacts::procedure::schema::default_snapshot;
    use crate::artifacts::procedure::Dictionary;
    use neural_engine::{Atom, Value};
    use protocol::os_spr::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;

    use std::collections::BTreeMap;

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    //#region 🔖️MutationLaws
    #[semio_framework_async_macros::async_test]
    async fn create_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &create_step(PathRef::default(), step("step-99", "log.print")));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &delete_step(PathRef::default(), "step-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_step_missing_target_is_error() {
        let base = default_snapshot();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &delete_step(PathRef::default(), "step-missing".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_steps_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &reorder_steps(PathRef::default(), "step-2".into(), 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_steps_missing_target_is_error() {
        let base = default_snapshot();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &reorder_steps(PathRef::default(), "step-missing".into(), 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_step_params_inverse_law() {
        let base = default_snapshot();
        let params = Dictionary::new().insert("message", Value::Atom(Atom::String("hi".into())));
        assert_mutation_inverse_law(&base, &edit_step_params(PathRef::default(), "step-2".into(), params));
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_step_params_missing_target_is_error() {
        let base = default_snapshot();
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &edit_step_params(PathRef::default(), "step-missing".into(), Dictionary::new()));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_step_duplicate_id_fatal_never_applies() {
        let base = default_snapshot();
        let mutation = create_step(PathRef::default(), step("step-1", "log.print"));
        protocol::os_spr::testkit::assert_fatal_never_applies(&protocol::Mutation::diff(&mutation, &base));
    }

    #[semio_framework_async_macros::async_test]
    async fn create_step_diff_absorb_law() {
        use protocol::Mutation;
        let base = default_snapshot();
        let d1 = create_step(PathRef::default(), step("step-97", "log.print")).diff(&base).into_parts().0;
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = create_step(PathRef::default(), step("step-98", "log.print")).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_procedure_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
        for kind in ProcedureMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(ProcedureMutation::kinds().len(), 4);
    }
    //#endregion 🔖️MutationLaws
}
//#endregion 🧪️Tests
