//! 🧬️ StepMutation — document mutation dispatch. Every leaf builds its sparse `StepDiff` directly and a concrete,
//! key/index-aware inverse from its payload and reads of `base`.

use crate::schema::diff::{StepArgAdded, StepArgModified, StepArgsDiff, StepDiff, StepEntitiesDiff, StepEntityAdded, StepEntityDiff, StepEntityModified};






























use crate::schema::snapshot::{StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue};
use crate::StepSnapshot;

use protocol::Mutation;

//#region 🔖️Mutations
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
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

/// 🔗️ Every entity id `value` references, through aggregates and typed values.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn value_references(value: &StepValue, out: &mut Vec<u64>) {
    match value {
        StepValue::Reference(id) => out.push(*id),
        StepValue::Aggregate(items) => items.iter().for_each(|item| value_references(item, out)),
        StepValue::TypedValue { value, .. } => value_references(value, out),
        _ => {}
    }
}

/// 🔗️ Every entity id `entity` references, in its leading record and its complex constituents.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_references(entity: &StepEntity) -> Vec<u64> {
    let mut out = Vec::new();
    entity.args.iter().for_each(|value| value_references(value, &mut out));
    entity.complex.iter().flat_map(|part| part.args.iter()).for_each(|value| value_references(value, &mut out));
    out
}

/// 🧭️ How `next`'s entities follow from `base`'s without any intermediate dangling reference: `inserts` (dependencies first, each at the
/// index it takes among the entities present when it is applied), then the `kept` entities whose values are edited in place, then the
/// `removes` (dependents first).
pub struct NetEntityPlan<'a> {
    pub inserts: Vec<(usize, &'a StepEntity)>,
    pub kept: Vec<(&'a StepEntity, &'a StepEntity)>,
    pub removes: Vec<u64>,
}

/// 🧭️ Plans the entity edits that carry `base` to `next`. `None` when the retained entities change their relative order, or when the references
/// among added (or among removed) entities form a cycle, neither of which a stepwise edit can express.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn net_entity_plan<'a>(base: &'a StepSnapshot, next: &'a StepSnapshot) -> Option<NetEntityPlan<'a>> {
    let next_position = |id: u64| next.entities.iter().position(|entity| entity.id == id);
    let kept: Vec<(&StepEntity, &StepEntity)> = base.entities.iter().filter_map(|before| next.entities.iter().find(|after| after.id == before.id).map(|after| (before, after))).collect();
    if kept.windows(2).any(|pair| next_position(pair[0].1.id) > next_position(pair[1].1.id)) {
        return None;
    }
    let mut working: Vec<u64> = base.entities.iter().map(|entity| entity.id).collect();
    let mut pending: Vec<&StepEntity> = next.entities.iter().filter(|entity| !working.contains(&entity.id)).collect();
    let mut inserts = Vec::new();
    while !pending.is_empty() {
        let ready = pending.iter().position(|entity| entity_references(entity).iter().all(|id| *id == entity.id || working.contains(id) || !pending.iter().any(|other| other.id == *id)))?;
        let entity = pending.remove(ready);
        let position = next_position(entity.id)?;
        let index = working.iter().rposition(|id| next_position(*id).is_some_and(|at| at < position)).map_or(0, |at| at + 1);
        working.insert(index, entity.id);
        inserts.push((index, entity));
    }
    let mut doomed: Vec<&StepEntity> = base.entities.iter().filter(|entity| next_position(entity.id).is_none()).collect();
    let mut removes = Vec::new();
    while !doomed.is_empty() {
        let free = doomed.iter().position(|entity| !doomed.iter().any(|other| other.id != entity.id && entity_references(other).contains(&entity.id)))?;
        removes.push(doomed.remove(free).id);
    }
    Some(NetEntityPlan { inserts, kept, removes })
}

/// 🧮️ The leaf mutations that carry `base` to `next`, ordered so no intermediate snapshot holds a dangling reference: header slots become their
/// `set-file-*` leaf, then the [`net_entity_plan`] as `insert-entity`, in-place name and argument edits, and `remove-entity`. `None` when `next`
/// changes the document `schema`, a retained entity's complex constituents, or what [`net_entity_plan`] cannot plan.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn net_mutations(base: &StepSnapshot, next: &StepSnapshot) -> Option<Vec<StepMutation>> {
    if base.schema != next.schema {
        return None;
    }
    let plan = net_entity_plan(base, next)?;
    if plan.kept.iter().any(|(before, after)| before.complex != after.complex) {
        return None;
    }
    let mut leaves = Vec::new();
    if base.header.file_description != next.header.file_description {
        leaves.push(StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description: next.header.file_description.clone() }));
    }
    if base.header.file_name != next.header.file_name {
        leaves.push(StepMutation::SetFileName(set_file_name::SetFileName { file_name: next.header.file_name.clone() }));
    }
    if base.header.file_schema != next.header.file_schema {
        leaves.push(StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: next.header.file_schema.clone() }));
    }
    leaves.extend(plan.inserts.iter().map(|(index, entity)| StepMutation::InsertEntity(insert_entity::InsertEntity { index: *index, entity: (*entity).clone() })));
    for (before, after) in &plan.kept {
        if before.name != after.name {
            leaves.push(StepMutation::SetEntityName(set_entity_name::SetEntityName { id: after.id, name: after.name.clone() }));
        }
        let shared = before.args.len().min(after.args.len());
        leaves.extend((0..shared).filter(|arg_index| before.args[*arg_index] != after.args[*arg_index]).map(|arg_index| StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: after.id, arg_index, value: after.args[arg_index].clone() })));
        leaves.extend((shared..after.args.len()).map(|arg_index| StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: after.id, arg_index, value: after.args[arg_index].clone() })));
        leaves.extend((shared..before.args.len()).rev().map(|arg_index| StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: after.id, arg_index })));
    }
    leaves.extend(plan.removes.iter().map(|id| StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: *id })));
    Some(leaves)
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
    use crate::schema::snapshot::{StepFileDescription, StepFileName, StepFileSchema, StepValue as SV};
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
                    SV::TypedValue { type_name: "LENGTH_MEASURE".into(), value: Box::new(SV::Real(3000.0)) },
                ],
            ),
        }),
        StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: 2 }),
        StepMutation::SetEntityName(set_entity_name::SetEntityName { id: 1, name: "RENAMED".into() }),
        StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: 1, arg_index: 1, value: SV::Aggregate(vec![SV::Real(1.0), SV::Real(2.0), SV::Real(3.0)]) }),
        StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: 1, arg_index: 2, value: SV::TypedValue { type_name: "X".into(), value: Box::new(SV::Aggregate(vec![SV::Integer(1), SV::Integer(2)])) } }),
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
