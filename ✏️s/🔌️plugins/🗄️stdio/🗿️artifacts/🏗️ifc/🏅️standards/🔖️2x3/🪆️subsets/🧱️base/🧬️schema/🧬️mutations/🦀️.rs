//! 🧬️ Ifc2x3Mutation — document mutation dispatch. Richer than `4`'s `SetSnapshot`-only stub: real
//! per-instance vocabulary (`UpsertInstance`/`RemoveInstance`/`SetHeader`) matching `Ifc2x3Diff`'s
//! own id-keyed shape.

use crate::standards::v2x3::subsets::base::schema::diff::{enc_part21_instance, Ifc2x3Diff};



















use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;
#[cfg(test)]
use semio_s_artifact_stdio_contract::part21::Part21Value;
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance};

//#region 🔖️Mutations
#[path = "🗑️remove-instance/🦀️.rs"]
pub mod remove_instance;
#[path = "📋set-header/🦀️.rs"]
pub mod set_header;
/// 📐️ Typed content mutation for `stdio.ifc.2x3`.
//#region 🔖️Leaves
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🧱upsert-instance/🦀️.rs"]
pub mod upsert_instance;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Ifc2x3Snapshot, diff = Ifc2x3Diff, schema = "Ifc2x3Mutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum Ifc2x3Mutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    UpsertInstance(upsert_instance::UpsertInstance),
    RemoveInstance(remove_instance::RemoveInstance),
    SetHeader(set_header::SetHeader),
}

/// 📇️ Kebab-case spelling of every `Ifc2x3Mutation` variant, in declaration order -- the
/// exhaustive mutation catalog `../../🔣️oracle.json`'s `kinds` array is required to
/// match verbatim (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps
/// that honest; the framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "upsert-instance", "remove-instance", "set-header"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff (computed against the PRE-mutation
/// state, per `Mutation::diff`'s contract).
pub fn apply_ifc2x3_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3Mutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let outcome = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &Ifc2x3Mutation, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let mut next = base.clone();
    match this {
        Ifc2x3Mutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Mutation>>::diff(patch, base),
        Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => {
            // 🪓 The RAW model-edit path: it carries the logical model verbatim and never validates.
            // Schema conformance is owned by the two gates that can report it — `encode_ifc2x3`
            // (refuses to export a non-IFC2X3 model) and every SUBSET's own validator reached
            // through `build()` (`Ifc2x3CobieMutation`/`Ifc2x3SavMutation`/`Ifc2x3Cv20Mutation`
            // each `reject` instead). The `expect` that used to stand here aborted the whole
            // process, and `agg_inverse` below hands back `SetSnapshot(base)` for EVERY mutation —
            // so an inverse taken against a still-default snapshot panicked by construction.
            return protocol::MutationOutcome::new(Ifc2x3Diff::between(base, snapshot));
        }
        Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance }) => match next.document.instances.iter_mut().find(|candidate| candidate.id == instance.id) {
            Some(existing) => *existing = instance.clone(),
            None => next.document.instances.push(instance.clone()),
        },
        Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id }) => next.document.instances.retain(|instance| instance.id != *id),
        Ifc2x3Mutation::SetHeader(set_header::SetHeader { header }) => next.document.header = header.clone(),
    }
    protocol::MutationOutcome::new(Ifc2x3Diff::between(base, &next))
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &Ifc2x3Mutation, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3Mutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        Ifc2x3Mutation::PatchSnapshot(patch) => <patch_snapshot::PatchSnapshot as protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Mutation>>::inverse(patch, base)?,
        _ => vec![Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(base.clone()) })],
    }

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs








//#region 🔖️OpBinaryCodec


//#endregion 🔖️OpBinaryCodec




//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ One representative `Ifc2x3Mutation` per variant, real `print_op()`-conformance-law fodder
/// (`ops_grammar_conformance_law`) and `protocol_walk_law` fodder — every `Part21Value` tag (incl.
/// the recursive `List`/`Typed` cases) and `UpsertInstance`'s bare `Part21Instance` payload (incl. a
/// real COMPLEX 2-entity instance) are exercised at least once.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<Ifc2x3Mutation> {
    vec![
        Ifc2x3Mutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(crate::standards::v2x3::engine::demo_ifc2x3_snapshot()) }),
        Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance {
            instance: Part21Instance {
                id: 99,
                entities: vec![
                    (
                        "IFCQUANTITYAREA".into(),
                        vec![
                            Part21Value::Unset,
                            Part21Value::Derived,
                            Part21Value::Int(-7),
                            Part21Value::Real(3.25.into()),
                            Part21Value::Str("hi".into()),
                            Part21Value::Enum("EDGE".into()),
                            Part21Value::Ref(42),
                            Part21Value::List(vec![Part21Value::Int(1), Part21Value::Int(2)]),
                            Part21Value::Typed { name: "IFCLENGTHMEASURE".into(), items: vec![Part21Value::Real(3000.0.into())] },
                        ],
                    ),
                    ("IFCPHYSICALSIMPLEQUANTITY".into(), vec![Part21Value::Unset]),
                ],
            },
        }),
        Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 2 }),
        Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: Part21Header { file_description: vec![], file_name: vec![], file_schema: vec![] } }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
