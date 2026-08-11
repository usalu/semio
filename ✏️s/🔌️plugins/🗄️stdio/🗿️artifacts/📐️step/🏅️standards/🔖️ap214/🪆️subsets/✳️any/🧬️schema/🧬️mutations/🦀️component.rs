//! 🧬️ StepMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `StepDiff` directly — apply-and-capture is banned by the recipe) and
//! `inverse()` is handcrafted per variant, key/index-aware.

use crate::artifacts::step::schema::diff::{
    dec_entity, dec_file_description, dec_file_name, dec_file_schema, dec_step_snapshot, dec_str, dec_value, diff_set_snapshot, enc_entity, enc_file_description,
    enc_file_name, enc_file_schema, enc_step_snapshot, enc_str, enc_value, parse_u64, parse_usize, StepArgAdded, StepArgModified, StepArgsDiff, StepDiff,
    StepEntitiesDiff, StepEntityAdded, StepEntityDiff, StepEntityModified,
};
use crate::artifacts::step::schema::snapshot::{StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue};
use crate::artifacts::step::StepSnapshot;
use protocol::{Mutation, MutationDiff, OpText};
#[cfg(test)]
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.step`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum StepMutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: StepSnapshot },
    SetFileDescription { file_description: StepFileDescription },
    SetFileName { file_name: StepFileName },
    SetFileSchema { file_schema: StepFileSchema },
    InsertEntity { index: usize, entity: StepEntity },
    RemoveEntity { id: u64 },
    SetEntityName { id: u64, name: String },
    SetEntityArg { id: u64, arg_index: usize, value: StepValue },
    InsertEntityArg { id: u64, arg_index: usize, value: StepValue },
    RemoveEntityArg { id: u64, arg_index: usize },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` — diff is the single semantics source: computed first,
/// then applied, never re-derived by hand.
pub fn apply_step_mutation(snapshot: &mut StepSnapshot, mutation: &StepMutation) -> StepDiff {
    let diff = <StepMutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
    *snapshot = <StepDiff as MutationDiff<StepSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<StepSnapshot> for StepMutation {
    type Diff = StepDiff;

    fn diff(&self, base: &StepSnapshot) -> Self::Diff {
        match self {
            StepMutation::NoMutation => StepDiff::default(),

            StepMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),

            StepMutation::SetFileDescription { file_description } => StepDiff {
                file_description: (base.header.file_description != *file_description).then(|| file_description.clone()),
                ..Default::default()
            },
            StepMutation::SetFileName { file_name } => StepDiff {
                file_name: (base.header.file_name != *file_name).then(|| file_name.clone()),
                ..Default::default()
            },
            StepMutation::SetFileSchema { file_schema } => StepDiff {
                file_schema: (base.header.file_schema != *file_schema).then(|| file_schema.clone()),
                ..Default::default()
            },

            StepMutation::InsertEntity { index, entity } => StepDiff {
                entities: Some(StepEntitiesDiff { added: vec![StepEntityAdded { index: *index, entity: entity.clone() }], ..Default::default() }),
                ..Default::default()
            },

            StepMutation::RemoveEntity { id } => {
                if base.entities.iter().any(|e| e.id == *id) {
                    StepDiff { entities: Some(StepEntitiesDiff { removed: vec![*id], ..Default::default() }), ..Default::default() }
                } else {
                    StepDiff::default()
                }
            }

            StepMutation::SetEntityName { id, name } => match base.entities.iter().find(|e| e.id == *id) {
                Some(e) if e.name != *name => StepDiff {
                    entities: Some(StepEntitiesDiff {
                        modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { name: Some(name.clone()), ..Default::default() } }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                _ => StepDiff::default(),
            },

            StepMutation::SetEntityArg { id, arg_index, value } => match base.entities.iter().find(|e| e.id == *id) {
                Some(e) if e.args.get(*arg_index).map(|v| v != value).unwrap_or(false) => StepDiff {
                    entities: Some(StepEntitiesDiff {
                        modified: vec![StepEntityModified {
                            id: *id,
                            diff: StepEntityDiff {
                                args: Some(StepArgsDiff { modified: vec![StepArgModified { index: *arg_index, value: value.clone() }], ..Default::default() }),
                                ..Default::default()
                            },
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                _ => StepDiff::default(),
            },

            StepMutation::InsertEntityArg { id, arg_index, value } => {
                if base.entities.iter().any(|e| e.id == *id) {
                    StepDiff {
                        entities: Some(StepEntitiesDiff {
                            modified: vec![StepEntityModified {
                                id: *id,
                                diff: StepEntityDiff {
                                    args: Some(StepArgsDiff { added: vec![StepArgAdded { index: *arg_index, value: value.clone() }], ..Default::default() }),
                                    ..Default::default()
                                },
                            }],
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                } else {
                    StepDiff::default()
                }
            }

            StepMutation::RemoveEntityArg { id, arg_index } => match base.entities.iter().find(|e| e.id == *id) {
                Some(e) if *arg_index < e.args.len() => StepDiff {
                    entities: Some(StepEntitiesDiff {
                        modified: vec![StepEntityModified {
                            id: *id,
                            diff: StepEntityDiff { args: Some(StepArgsDiff { removed: vec![*arg_index], ..Default::default() }), ..Default::default() },
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                _ => StepDiff::default(),
            },
        }
    }

    fn inverse(&self, base: &StepSnapshot) -> Vec<Self> {
        match self {
            StepMutation::NoMutation => vec![StepMutation::NoMutation],

            StepMutation::SetSnapshot { .. } => vec![StepMutation::SetSnapshot { snapshot: base.clone() }],

            StepMutation::SetFileDescription { .. } => {
                vec![StepMutation::SetFileDescription { file_description: base.header.file_description.clone() }]
            }
            StepMutation::SetFileName { .. } => vec![StepMutation::SetFileName { file_name: base.header.file_name.clone() }],
            StepMutation::SetFileSchema { .. } => vec![StepMutation::SetFileSchema { file_schema: base.header.file_schema.clone() }],

            StepMutation::InsertEntity { entity, .. } => vec![StepMutation::RemoveEntity { id: entity.id }],

            StepMutation::RemoveEntity { id } => match base.entities.iter().position(|e| e.id == *id) {
                Some(idx) => vec![StepMutation::InsertEntity { index: idx, entity: base.entities[idx].clone() }],
                None => vec![StepMutation::NoMutation],
            },

            StepMutation::SetEntityName { id, .. } => match base.entities.iter().find(|e| e.id == *id) {
                Some(e) => vec![StepMutation::SetEntityName { id: *id, name: e.name.clone() }],
                None => vec![StepMutation::NoMutation],
            },

            StepMutation::SetEntityArg { id, arg_index, .. } => {
                match base.entities.iter().find(|e| e.id == *id).and_then(|e| e.args.get(*arg_index)) {
                    Some(v) => vec![StepMutation::SetEntityArg { id: *id, arg_index: *arg_index, value: v.clone() }],
                    None => vec![StepMutation::NoMutation],
                }
            }

            StepMutation::InsertEntityArg { id, arg_index, .. } => vec![StepMutation::RemoveEntityArg { id: *id, arg_index: *arg_index }],

            StepMutation::RemoveEntityArg { id, arg_index } => {
                match base.entities.iter().find(|e| e.id == *id).and_then(|e| e.args.get(*arg_index)) {
                    Some(v) => vec![StepMutation::InsertEntityArg { id: *id, arg_index: *arg_index, value: v.clone() }],
                    None => vec![StepMutation::NoMutation],
                }
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `StepMutation` — real `cargo check` confirms 3a
/// on the mutation side too, independently of the diff side: `InsertEntity.entity: StepEntity`
/// fails (`StepEntity: DslField` unsatisfied) and `SetEntityArg`/`InsertEntityArg`'s
/// `value: StepValue` fail directly (`StepValue: DslField` unsatisfied) — `#[derive(dsl::DslOps)]`
/// requires `DslField` on every variant field, transitively; `StepValue`/`StepEntity` are real
/// data-carrying types with no `DslField` impl, same root cause as `SvgMutation`'s `InsertElement`/
/// `SetSnapshot` blockers. Reuses `StepDiff`'s `pub(crate)` grammar primitives (`enc_value`/
/// `enc_entity`/`enc_step_snapshot`/...) rather than duplicating them — same pattern `SvgMutation`
/// uses against `SvgDiff`. Grammar: `keyword arg=value ...` (space-separated), one match arm per
/// variant (no `DslVariants` scaffolding available since nothing here derives it).
fn print_step_mutation(m: &StepMutation) -> String {
    match m {
        StepMutation::NoMutation => "no-mutation".to_string(),
        StepMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_step_snapshot(snapshot)),
        StepMutation::SetFileDescription { file_description } => format!("set-file-description file-description={}", enc_file_description(file_description)),
        StepMutation::SetFileName { file_name } => format!("set-file-name file-name={}", enc_file_name(file_name)),
        StepMutation::SetFileSchema { file_schema } => format!("set-file-schema file-schema={}", enc_file_schema(file_schema)),
        StepMutation::InsertEntity { index, entity } => format!("insert-entity index={index} entity={}", enc_entity(entity)),
        StepMutation::RemoveEntity { id } => format!("remove-entity id={id}"),
        StepMutation::SetEntityName { id, name } => format!("set-entity-name id={id} name={}", enc_str(name)),
        StepMutation::SetEntityArg { id, arg_index, value } => format!("set-entity-arg id={id} arg-index={arg_index} value={}", enc_value(value)),
        StepMutation::InsertEntityArg { id, arg_index, value } => format!("insert-entity-arg id={id} arg-index={arg_index} value={}", enc_value(value)),
        StepMutation::RemoveEntityArg { id, arg_index } => format!("remove-entity-arg id={id} arg-index={arg_index}"),
    }
}
fn parse_step_mutation(line: &str) -> Result<StepMutation, String> {
    if line == "no-mutation" {
        return Ok(StepMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("step mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("step mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { parse_usize(arg(k)?) };
    let u64_arg = |k: &str| -> Result<u64, String> { parse_u64(arg(k)?) };
    match keyword {
        "set-snapshot" => Ok(StepMutation::SetSnapshot { snapshot: dec_step_snapshot(arg("snapshot")?)? }),
        "set-file-description" => Ok(StepMutation::SetFileDescription { file_description: dec_file_description(arg("file-description")?)? }),
        "set-file-name" => Ok(StepMutation::SetFileName { file_name: dec_file_name(arg("file-name")?)? }),
        "set-file-schema" => Ok(StepMutation::SetFileSchema { file_schema: dec_file_schema(arg("file-schema")?)? }),
        "insert-entity" => Ok(StepMutation::InsertEntity { index: usize_arg("index")?, entity: dec_entity(arg("entity")?)? }),
        "remove-entity" => Ok(StepMutation::RemoveEntity { id: u64_arg("id")? }),
        "set-entity-name" => Ok(StepMutation::SetEntityName { id: u64_arg("id")?, name: dec_str(arg("name")?)? }),
        "set-entity-arg" => Ok(StepMutation::SetEntityArg { id: u64_arg("id")?, arg_index: usize_arg("arg-index")?, value: dec_value(arg("value")?)? }),
        "insert-entity-arg" => Ok(StepMutation::InsertEntityArg { id: u64_arg("id")?, arg_index: usize_arg("arg-index")?, value: dec_value(arg("value")?)? }),
        "remove-entity-arg" => Ok(StepMutation::RemoveEntityArg { id: u64_arg("id")?, arg_index: usize_arg("arg-index")? }),
        other => Err(format!("step mutation: unknown keyword {other:?}")),
    }
}

impl OpText for StepMutation {
    fn print_op(&self) -> String {
        print_step_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_step_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// ⚡️ Binary = the text bytes verbatim, same simplification `StepDiff`'s hand-rolled codec uses.
impl protocol::OpBinary for StepMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::schema::snapshot::{StepHeader, StepValue as SV};

    fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
        StepEntity { id, name: name.into(), args, complex: Vec::new() }
    }

    fn base_snapshot() -> StepSnapshot {
        StepSnapshot {
            schema: crate::artifacts::step::STDIO_STEP_DOCUMENT_SCHEMA.into(),
            header: StepHeader::default(),
            entities: vec![entity(1, "CARTESIAN_POINT", vec![SV::String("".into()), SV::Real(1.0)]), entity(2, "DIRECTION", vec![SV::Unset])],
        }
    }

    /// 🧪️ `mutation_diff_law`: ∀ variant, `m.diff(base).apply(base) == { apply(&mut s, m); s }`
    /// and the returned diff equals `m.diff(base)`.
    fn assert_mutation_diff_law(base: &StepSnapshot, m: StepMutation) {
        let expected_diff = <StepMutation as Mutation<StepSnapshot>>::diff(&m, base);
        let expected_state = expected_diff.apply(base);
        let mut actual_state = base.clone();
        let actual_diff = apply_step_mutation(&mut actual_state, &m);
        assert_eq!(actual_diff, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
        assert_eq!(actual_state, expected_state, "applied state must match for {m:?}");
    }

    #[test]
    fn mutation_diff_law_covers_every_variant() {
        let base = base_snapshot();
        assert_mutation_diff_law(&base, StepMutation::NoMutation);
        let mut next = base.clone();
        next.entities[0].name = "X".into();
        assert_mutation_diff_law(&base, StepMutation::SetSnapshot { snapshot: next });
        assert_mutation_diff_law(&base, StepMutation::SetFileDescription { file_description: crate::artifacts::step::schema::snapshot::StepFileDescription { description: vec!["d".into()], implementation_level: "2;1".into() } });
        assert_mutation_diff_law(&base, StepMutation::SetFileName { file_name: crate::artifacts::step::schema::snapshot::StepFileName { name: "n".into(), ..Default::default() } });
        assert_mutation_diff_law(&base, StepMutation::SetFileSchema { file_schema: crate::artifacts::step::schema::snapshot::StepFileSchema { schemas: vec!["X".into()] } });
        assert_mutation_diff_law(&base, StepMutation::InsertEntity { index: 1, entity: entity(50, "NEW", vec![]) });
        assert_mutation_diff_law(&base, StepMutation::RemoveEntity { id: 2 });
        assert_mutation_diff_law(&base, StepMutation::SetEntityName { id: 1, name: "RENAMED".into() });
        assert_mutation_diff_law(&base, StepMutation::SetEntityArg { id: 1, arg_index: 1, value: SV::Real(9.0) });
        assert_mutation_diff_law(&base, StepMutation::InsertEntityArg { id: 1, arg_index: 2, value: SV::Enum("T".into()) });
        assert_mutation_diff_law(&base, StepMutation::RemoveEntityArg { id: 1, arg_index: 0 });
        // Graceful no-ops on missing keys must never panic.
        assert_mutation_diff_law(&base, StepMutation::RemoveEntity { id: 999 });
        assert_mutation_diff_law(&base, StepMutation::SetEntityArg { id: 999, arg_index: 0, value: SV::Unset });
        assert_mutation_diff_law(&base, StepMutation::RemoveEntityArg { id: 1, arg_index: 99 });
    }

    /// 🧪️ `inverse_law` (mutation level): every variant's `inverse()` round-trips.
    #[test]
    fn inverse_law_mutation_level_round_trips_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            StepMutation::SetFileSchema { file_schema: crate::artifacts::step::schema::snapshot::StepFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] } },
            StepMutation::InsertEntity { index: 1, entity: entity(50, "NEW", vec![SV::Integer(3)]) },
            StepMutation::RemoveEntity { id: 2 },
            StepMutation::SetEntityName { id: 1, name: "RENAMED".into() },
            StepMutation::SetEntityArg { id: 1, arg_index: 1, value: SV::Real(42.0) },
            StepMutation::InsertEntityArg { id: 1, arg_index: 2, value: SV::Enum("F".into()) },
            StepMutation::RemoveEntityArg { id: 1, arg_index: 0 },
        ];
        for m in variants {
            let mut state = base.clone();
            apply_step_mutation(&mut state, &m);
            let inverses = <StepMutation as Mutation<StepSnapshot>>::inverse(&m, &base);
            let mut restored = state.clone();
            for inv in &inverses {
                apply_step_mutation(&mut restored, inv);
            }
            assert_eq!(restored, base, "mutation-level inverse must restore base for {m:?}");
        }
    }

    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `StepMutation` grammar —
    /// exercises every variant incl. `InsertEntity`'s bare `StepEntity` payload and
    /// `SetEntityArg`/`InsertEntityArg`'s bare `StepValue` payload (every `StepValue` variant,
    /// incl. the recursive `Aggregate`/`TypedValue` cases).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            StepMutation::NoMutation,
            StepMutation::SetSnapshot { snapshot: base.clone() },
            StepMutation::SetFileDescription { file_description: crate::artifacts::step::schema::snapshot::StepFileDescription { description: vec!["d1".into(), "d2".into()], implementation_level: "2;1".into() } },
            StepMutation::SetFileName { file_name: crate::artifacts::step::schema::snapshot::StepFileName { name: "n.step".into(), timestamp: "2026-08-10T00:00:00".into(), author: vec!["A".into()], organization: vec!["O".into()], preprocessor_version: "pv".into(), originating_system: "sys".into(), authorization: "auth".into() } },
            StepMutation::SetFileSchema { file_schema: crate::artifacts::step::schema::snapshot::StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into(), "CONFIG_CONTROL_DESIGN".into()] } },
            StepMutation::InsertEntity { index: 1, entity: entity(50, "NEW", vec![SV::Unset, SV::Derived, SV::Integer(-42), SV::Real(3.5), SV::String("s".into()), SV::Enum("T".into()), SV::Reference(9), SV::Aggregate(vec![SV::Integer(1), SV::Real(2.0)]), SV::TypedValue { type_name: "IFCLENGTHMEASURE".into(), value: Box::new(SV::Real(3000.0)) }]) },
            StepMutation::RemoveEntity { id: 2 },
            StepMutation::SetEntityName { id: 1, name: "RENAMED".into() },
            StepMutation::SetEntityArg { id: 1, arg_index: 1, value: SV::Aggregate(vec![SV::Real(1.0), SV::Real(2.0), SV::Real(3.0)]) },
            StepMutation::InsertEntityArg { id: 1, arg_index: 2, value: SV::TypedValue { type_name: "X".into(), value: Box::new(SV::Aggregate(vec![SV::Integer(1), SV::Integer(2)])) } },
            StepMutation::RemoveEntityArg { id: 1, arg_index: 0 },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = StepMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = StepMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
}
//#endregion 🧪️Tests
