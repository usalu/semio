//! 🧬️ IfcMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal stub — HEADER scalar setters plus
//! `InsertEntity`/`RemoveEntity`/`SetEntityName`/`SetEntityArg`/`InsertEntityArg`/`RemoveEntityArg`
//! for the id-keyed `entities` collection and its per-entity positional `args`. Every variant's
//! `diff()` is handcrafted (constructs `IfcDiff` directly via the `schema::diff` builders) —
//! apply-and-capture is never used.

use crate::schema::diff::{self, IfcDiff};


















use crate::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};
use crate::IfcSnapshot;

use protocol::{Mutation};

//#region 🔖️Mutations
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "➕insert-entity/🦀️.rs"]
pub mod insert_entity;
#[path = "🧩insert-entity-arg/🦀️.rs"]
pub mod insert_entity_arg;
#[path = "➖remove-entity/🦀️.rs"]
pub mod remove_entity;
#[path = "🧹remove-entity-arg/🦀️.rs"]
pub mod remove_entity_arg;
#[path = "🎛️set-entity-arg/🦀️.rs"]
pub mod set_entity_arg;
#[path = "🏷️set-entity-name/🦀️.rs"]
pub mod set_entity_name;
#[path = "🗒️set-file-description/🦀️.rs"]
pub mod set_file_description;
#[path = "📛️set-file-name/🦀️.rs"]
pub mod set_file_name;
#[path = "🧬️set-file-schema/🦀️.rs"]
pub mod set_file_schema;
/// 📐️ Typed content mutation for `stdio.ifc`.
/// 🧪️ F6 CONFIRMED: `#[derive(dsl::DslOps)]` on this enum fails (independent confirmation beyond
/// `IfcDiff`'s `DiffCodec` blocker — see that file's doc comment), real `cargo check -p
/// semio-s-plugin-stdio --lib` output, verbatim:
/// ```text
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcValue: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️.rs:27:21   (SetFileDescription { values: Vec<IfcValue> })
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcEntity: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️.rs:40:17   (InsertEntity { entity: IfcEntity })
/// ```
/// Same root cause as `IfcDiff` (§3a): `IfcValue` carries fields on 7 of its 9 variants, has no
/// `DslField` impl, and every variant here either carries it directly (`values`/`value`) or
/// transitively via `IfcEntity` (`InsertEntity`). `OpText`/`OpBinary`
/// stay hand-rolled below for the same reason, reusing `IfcDiff`'s `pub(crate)` grammar primitives
/// (`enc_str`/`enc_ifc_value`/`enc_entity`/`split_top_level`/...); `DESCRIPTORS`/`descriptor()` are
/// no longer hand-written, though — `#[derive(dsl::Mutations)]` synthesizes both from the per-leaf
/// `🔣️.json` descriptors beside this file, which does not need `DslField`.
//#region 🔖️Leaves
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = IfcSnapshot, diff = IfcDiff, schema = "IfcMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum IfcMutation {
    /// 📇️ Sets the `FILE_DESCRIPTION` header record's raw value tuple.
    SetFileDescription(set_file_description::SetFileDescription),
    /// 📇️ Sets the `FILE_NAME` header record's raw value tuple.
    SetFileName(set_file_name::SetFileName),
    /// 📇️ Sets the `FILE_SCHEMA` header record's raw value tuple.
    SetFileSchema(set_file_schema::SetFileSchema),
    /// ➕️ Inserts a fully-specified entity at `index` (final position, clamped to `len`).
    InsertEntity(insert_entity::InsertEntity),
    /// ➖️ Removes the entity with id `id` (no-op if absent).
    RemoveEntity(remove_entity::RemoveEntity),
    /// 🏷️ Sets entity `id`'s EXPRESS type keyword (e.g. `"IFCWALL"`).
    SetEntityName(set_entity_name::SetEntityName),
    /// 📝️ Replaces the argument at `index` of entity `id`'s positional arg list.
    SetEntityArg(set_entity_arg::SetEntityArg),
    /// 🧩️ Inserts a new argument at `index` (final position) of entity `id`'s arg list.
    InsertEntityArg(insert_entity_arg::InsertEntityArg),
    /// 🧹️ Removes the argument at `index` of entity `id`'s arg list.
    RemoveEntityArg(remove_entity_arg::RemoveEntityArg),
}

/// 📇️ Kebab-case spelling of every `IfcMutation` variant, in declaration order — the exhaustive
/// mutation catalog `../../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["set-file-description", "set-file-name", "set-file-schema", "insert-entity", "remove-entity", "set-entity-name", "set-entity-arg", "insert-entity-arg", "remove-entity-arg"];
//#endregion 🔖️Mutations


//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning a typed error outcome without changing the
/// snapshot when an entity or argument target is missing or out of range.
#[cfg(test)]
pub fn apply_ifc_mutation(snapshot: &mut IfcSnapshot, mutation: &IfcMutation) -> protocol::MutationOutcome<IfcDiff> {
    let outcome = <IfcMutation as Mutation<IfcSnapshot>>::diff(mutation, snapshot);
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
/// 🧪️ P2-FG1: one representative `IfcMutation` per variant, real `print_op()`-conformance-law
/// fodder (`ops_grammar_conformance_law`) and `protocol_walk_law` fodder — every `IfcValue` tag
/// (incl. the recursive `Aggregate`/`TypedValue` cases) and `InsertEntity`'s bare `IfcEntity`
/// payload are exercised at least once.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<IfcMutation> {
    let demo_entity = |id: u64, name: &str, args: Vec<IfcValue>| IfcEntity { id, name: name.into(), args, complex: Vec::new() };
    vec![
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: vec![IfcValue::String("demo".into())] }),
        IfcMutation::SetFileName(set_file_name::SetFileName { values: vec![IfcValue::String("demo.ifc".into())] }),
        IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] }),
        IfcMutation::InsertEntity(insert_entity::InsertEntity {
            index: 1,
            entity: demo_entity(
                99,
                "IFCSITE",
                vec![
                    IfcValue::Unset,
                    IfcValue::Derived,
                    IfcValue::Integer(-7),
                    IfcValue::Real(3.25),
                    IfcValue::String("hi".into()),
                    IfcValue::Enum("EDGE".into()),
                    IfcValue::Reference(42),
                    IfcValue::Aggregate(vec![IfcValue::Integer(1), IfcValue::Integer(2)]),
                    IfcValue::TypedValue { name: "IFCLENGTHMEASURE".into(), items: vec![IfcValue::Real(3000.0)] },
                ],
            ),
        }),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: 1, name: "IFCSLAB".into() }),
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, index: 1, value: IfcValue::String("Wall-02".into()) }),
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 1, index: 2, value: IfcValue::Derived }),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: 1, index: 0 }),
    ]
}
//#endregion 🔖️DemoCases


//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

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
