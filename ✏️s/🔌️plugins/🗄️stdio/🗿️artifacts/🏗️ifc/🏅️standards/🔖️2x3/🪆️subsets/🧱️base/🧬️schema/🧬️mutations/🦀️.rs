//! 🧬️ Ifc2x3Mutation — document mutation dispatch. Real
//! per-instance vocabulary (`UpsertInstance`/`RemoveInstance`/`SetHeader`) matching `Ifc2x3Diff`'s
//! own id-keyed shape.

use crate::standards::v2x3::subsets::base::schema::diff::Ifc2x3Diff;



















use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;
#[cfg(test)]
use semio_s_artifact_stdio_contract::part21::Part21Value;
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance};

//#region 🔖️Mutations
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "🗑️remove-instance/🦀️.rs"]
pub mod remove_instance;
#[path = "📋set-header/🦀️.rs"]
pub mod set_header;
/// 📐️ Typed content mutation for `stdio.ifc.2x3`.
//#region 🔖️Leaves
#[path = "🧱upsert-instance/🦀️.rs"]
pub mod upsert_instance;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Ifc2x3Snapshot, diff = Ifc2x3Diff, schema = "Ifc2x3Mutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum Ifc2x3Mutation {
    UpsertInstance(upsert_instance::UpsertInstance),
    RemoveInstance(remove_instance::RemoveInstance),
    SetHeader(set_header::SetHeader),
}

/// 📇️ Kebab-case spelling of every `Ifc2x3Mutation` variant, in declaration order -- the
/// exhaustive mutation catalog `../../🔣️oracle.json`'s `kinds` array is required to
/// match verbatim (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps
/// that honest; the framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["upsert-instance", "remove-instance", "set-header"];
//#endregion 🔖️Mutations


//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff (computed against the PRE-mutation
/// state, per `Mutation::diff`'s contract).
#[cfg(test)]
pub fn apply_ifc2x3_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3Mutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let outcome = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply


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
            index: Some(0),
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
