//! 🧬️ StepMutation — document mutation dispatch. Every leaf builds its sparse `StepDiff` directly and a concrete,
//! key/index-aware inverse from its payload and reads of `base`.

use crate::schema::diff::{StepArgAdded, StepArgModified, StepArgsDiff, StepDiff, StepEntitiesDiff, StepEntityAdded, StepEntityDiff, StepEntityModified};






























use crate::schema::snapshot::{StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue};
use crate::StepSnapshot;

use protocol::Mutation;

//#region 🔖️Mutations
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "🧩insert-entity/🦀️.rs"]
pub mod insert_entity;
#[path = "➕insert-entity-arg/🦀️.rs"]
pub mod insert_entity_arg;
#[path = "🗑️remove-entity/🦀️.rs"]
pub mod remove_entity;
#[path = "➖remove-entity-arg/🦀️.rs"]
pub mod remove_entity_arg;
#[path = "🔧set-entity-arg/🦀️.rs"]
pub mod set_entity_arg;
#[path = "✏️set-entity-name/🦀️.rs"]
pub mod set_entity_name;
#[path = "📋️set-file-description/🦀️.rs"]
pub mod set_file_description;
#[path = "📛set-file-name/🦀️.rs"]
pub mod set_file_name;
#[path = "🏷️set-file-schema/🦀️.rs"]
pub mod set_file_schema;

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "StepMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum StepMutation {
    SetFileDescription(set_file_description::SetFileDescription),
    SetFileName(set_file_name::SetFileName),
    SetFileSchema(set_file_schema::SetFileSchema),
    InsertEntity(insert_entity::InsertEntity),
    RemoveEntity(remove_entity::RemoveEntity),
    SetEntityName(set_entity_name::SetEntityName),
    SetEntityArg(set_entity_arg::SetEntityArg),
    InsertEntityArg(insert_entity_arg::InsertEntityArg),
    RemoveEntityArg(remove_entity_arg::RemoveEntityArg),
}

/// 📇️ Kebab-case spelling of every `StepMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["set-file-description", "set-file-name", "set-file-schema", "insert-entity", "remove-entity", "set-entity-name", "set-entity-arg", "insert-entity-arg", "remove-entity-arg"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` — diff is the single semantics source: computed first,
/// then applied, never re-derived by hand.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[cfg(test)]
pub fn apply_step_mutation(snapshot: &mut StepSnapshot, mutation: &StepMutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepMutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
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








//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: one representative `StepMutation` per variant, real `print_op()`-conformance-law
/// fodder (`ops_grammar_conformance_law`) and `protocol_walk_law` fodder — every `StepValue` tag
/// (incl. the recursive `Aggregate`/`TypedValue` cases) and `InsertEntity`'s bare `StepEntity`
/// payload are exercised at least once.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<StepMutation> {
    use crate::schema::snapshot::{StepFileDescription, StepFileName, StepFileSchema, StepTypedValue, StepValue as SV};
    let demo_entity = |id: u64, name: &str, args: Vec<StepValue>| StepEntity { id, name: name.into(), args, complex: Vec::new() };
    vec![
        StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description: StepFileDescription { description: vec!["demo".into()], implementation_level: "2;1".into() } }),
        StepMutation::SetFileName(set_file_name::SetFileName {
            file_name: StepFileName {
                name: "demo.step".into(),
                timestamp: "2026-08-11T00:00:00".into(),
                author: vec!["Ueli".into()],
                organization: vec!["semio".into()],
                preprocessor_version: "semio".into(),
                originating_system: "".into(),
                authorization: "".into(),
            },
        }),
        StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] } }),
        StepMutation::InsertEntity(insert_entity::InsertEntity {
            index: 1,
            entity: demo_entity(
                50,
                "NEW",
                vec![
                    SV::Unset,
                    SV::Derived,
                    SV::Integer(-42),
                    SV::Real(3.5),
                    SV::String("s".into()),
                    SV::Enum("T".into()),
                    SV::Reference(9),
                    SV::Aggregate(vec![SV::Integer(1), SV::Real(2.0)]),
                    SV::TypedValue(StepTypedValue { type_name: "LENGTH_MEASURE".into(), value: Box::new(SV::Real(3000.0)) }),
                ],
            ),
        }),
        StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        StepMutation::SetEntityName(set_entity_name::SetEntityName { id: 1, name: "RENAMED".into() }),
        StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, arg_index: 1, value: SV::Aggregate(vec![SV::Real(1.0), SV::Real(2.0), SV::Real(3.0)]) }),
        StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 1, arg_index: 2, value: SV::TypedValue(StepTypedValue { type_name: "X".into(), value: Box::new(SV::Aggregate(vec![SV::Integer(1), SV::Integer(2)])) }) }),
        StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 1, arg_index: 0 }),
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

#[cfg(test)]
use protocol::{OpBinary,OpText};
