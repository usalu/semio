//! 🧬️ StepMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `StepDiff` directly — apply-and-capture is banned by the recipe) and
//! `inverse()` is handcrafted per variant, key/index-aware.

use crate::schema::diff::{diff_set_snapshot, StepArgAdded, StepArgModified, StepArgsDiff, StepDiff, StepEntitiesDiff, StepEntityAdded, StepEntityDiff, StepEntityModified};






























use crate::schema::snapshot::{StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue};
use crate::StepSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

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
/// 📐️ Typed content mutation for `stdio.step`.
//#region 🔖️Leaves
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = StepSnapshot, diff = StepDiff, schema = "StepMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum StepMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
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
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "set-file-description", "set-file-name", "set-file-schema", "insert-entity", "remove-entity", "set-entity-name", "set-entity-arg", "insert-entity-arg", "remove-entity-arg"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` — diff is the single semantics source: computed first,
/// then applied, never re-derived by hand.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_step_mutation(snapshot: &mut StepSnapshot, mutation: &StepMutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepMutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
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
pub(crate) fn agg_diff(this: &StepMutation, base: &StepSnapshot) -> protocol::MutationOutcome<StepDiff> {
    protocol::MutationOutcome::new(match this {
        StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        StepMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<StepSnapshot, StepMutation>>::diff(patch, base),

        StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description }) => StepDiff { file_description: (base.header.file_description != *file_description).then(|| file_description.clone()), ..Default::default() },
        StepMutation::SetFileName(set_file_name::SetFileName { file_name }) => StepDiff { file_name: (base.header.file_name != *file_name).then(|| file_name.clone()), ..Default::default() },
        StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema }) => StepDiff { file_schema: (base.header.file_schema != *file_schema).then(|| file_schema.clone()), ..Default::default() },

        StepMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => StepDiff { entities: Some(StepEntitiesDiff { added: vec![StepEntityAdded { index: *index, entity: entity.clone() }], ..Default::default() }), ..Default::default() },

        // 🎯️ A target that does NOT exist must be REJECTED, not silently dropped: the diff is
        // emitted as written and `validate_entities_diff` (../🔺️diff) refuses it with the target
        // path the caller asked for (`["entities", "<id>"]`, `["entities", "<id>", "args", "<i>"]`),
        // which `apply_step_mutation` above turns into the outcome's messages. Pre-checking here and
        // returning an EMPTY diff instead made every impossible edit look like a successful no-op,
        // and left `missing_and_out_of_range_targets_are_rejected_without_mutating` reading
        // `messages()[0]` of an empty message list. A target that EXISTS and already carries the
        // requested value is the genuine no-op, and still yields the empty diff below.
        StepMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => StepDiff { entities: Some(StepEntitiesDiff { removed: vec![*id], ..Default::default() }), ..Default::default() },

        StepMutation::SetEntityName(set_entity_name::SetEntityName { id, name }) => match base.entities.iter().find(|e| e.id == *id) {
            Some(e) if e.name == *name => StepDiff::default(),
            _ => StepDiff { entities: Some(StepEntitiesDiff { modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { name: Some(name.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() },
        },

        StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, arg_index, value }) => match base.entities.iter().find(|e| e.id == *id) {
            Some(e) if e.args.get(*arg_index).is_some_and(|v| v == value) => StepDiff::default(),
            _ => StepDiff {
                entities: Some(StepEntitiesDiff {
                    modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { modified: vec![StepArgModified { index: *arg_index, value: value.clone() }], ..Default::default() }), ..Default::default() } }],
                    ..Default::default()
                }),
                ..Default::default()
            },
        },

        StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, arg_index, value }) => StepDiff {
            entities: Some(StepEntitiesDiff {
                modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { added: vec![StepArgAdded { index: *arg_index, value: value.clone() }], ..Default::default() }), ..Default::default() } }],
                ..Default::default()
            }),
            ..Default::default()
        },

        StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, arg_index }) => StepDiff {
            entities: Some(StepEntitiesDiff { modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { removed: vec![*arg_index], ..Default::default() }), ..Default::default() } }], ..Default::default() }),
            ..Default::default()
        },
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &StepMutation, base: &StepSnapshot) -> Result<Vec<StepMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        StepMutation::SetSnapshot(_) => vec![StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        StepMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<StepSnapshot, StepMutation>>::inverse(patch, base)?),

        StepMutation::SetFileDescription(_) => {
            vec![StepMutation::SetFileDescription(set_file_description::SetFileDescription { file_description: base.header.file_description.clone() })]
        }
        StepMutation::SetFileName(_) => vec![StepMutation::SetFileName(set_file_name::SetFileName { file_name: base.header.file_name.clone() })],
        StepMutation::SetFileSchema(_) => vec![StepMutation::SetFileSchema(set_file_schema::SetFileSchema { file_schema: base.header.file_schema.clone() })],

        StepMutation::InsertEntity(insert_entity::InsertEntity { entity, .. }) => vec![StepMutation::RemoveEntity(remove_entity::RemoveEntity { id: entity.id })],

        StepMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => match base.entities.iter().position(|e| e.id == *id) {
            Some(idx) => vec![StepMutation::InsertEntity(insert_entity::InsertEntity { index: idx, entity: base.entities[idx].clone() })],
            None => Vec::new(),
        },

        StepMutation::SetEntityName(set_entity_name::SetEntityName { id, .. }) => match base.entities.iter().find(|e| e.id == *id) {
            Some(e) => vec![StepMutation::SetEntityName(set_entity_name::SetEntityName { id: *id, name: e.name.clone() })],
            None => Vec::new(),
        },

        StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, arg_index, .. }) => match base.entities.iter().find(|e| e.id == *id).and_then(|e| e.args.get(*arg_index)) {
            Some(v) => vec![StepMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: *id, arg_index: *arg_index, value: v.clone() })],
            None => Vec::new(),
        },

        StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, arg_index, .. }) => vec![StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: *id, arg_index: *arg_index })],

        StepMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, arg_index }) => match base.entities.iter().find(|e| e.id == *id).and_then(|e| e.args.get(*arg_index)) {
            Some(v) => vec![StepMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: *id, arg_index: *arg_index, value: v.clone() })],
            None => Vec::new(),
        },
    }

    })
}
//#endregion 🔖️MutationTrait

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
        StepMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        StepMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: crate::engine::demo_step_snapshot() }),
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
