//! 🧬️ StepMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `StepDiff` directly — apply-and-capture is banned by the recipe) and
//! `inverse()` is handcrafted per variant, key/index-aware.

use crate::artifacts::step::schema::diff::{
    dec_entity, dec_entity_bin, dec_file_description, dec_file_description_bin, dec_file_name, dec_file_name_bin, dec_file_schema, dec_file_schema_bin, dec_step_snapshot, dec_step_snapshot_bin, dec_str, dec_value, dec_value_bin, diff_set_snapshot,
    enc_entity, enc_entity_bin, enc_file_description, enc_file_description_bin, enc_file_name, enc_file_name_bin, enc_file_schema, enc_file_schema_bin, enc_step_snapshot, enc_step_snapshot_bin, enc_str, enc_value, enc_value_bin, parse_u64,
    parse_usize, read_str_bin, write_str_bin, StepArgAdded, StepArgModified, StepArgsDiff, StepDiff, StepEntitiesDiff, StepEntityAdded, StepEntityDiff, StepEntityModified,
};
use crate::artifacts::step::schema::snapshot::{StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue};
use crate::artifacts::step::StepSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.step`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum StepMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: StepSnapshot,
    },
    SetFileDescription {
        file_description: StepFileDescription,
    },
    SetFileName {
        file_name: StepFileName,
    },
    SetFileSchema {
        file_schema: StepFileSchema,
    },
    InsertEntity {
        index: usize,
        entity: StepEntity,
    },
    RemoveEntity {
        id: u64,
    },
    SetEntityName {
        id: u64,
        name: String,
    },
    SetEntityArg {
        id: u64,
        arg_index: usize,
        value: StepValue,
    },
    InsertEntityArg {
        id: u64,
        arg_index: usize,
        value: StepValue,
    },
    RemoveEntityArg {
        id: u64,
        arg_index: usize,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` — diff is the single semantics source: computed first,
/// then applied, never re-derived by hand.
pub async fn apply_step_mutation(snapshot: &mut StepSnapshot, mutation: &StepMutation) -> protocol::MutationOutcome<StepDiff> {
    let outcome = <StepMutation as Mutation<StepSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<StepSnapshot> for StepMutation {
    type Diff = StepDiff;

    async fn diff(&self, base: &StepSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            StepMutation::NoMutation => StepDiff::default(),

            StepMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),

            StepMutation::SetFileDescription { file_description } => StepDiff { file_description: (base.header.file_description != *file_description).then(|| file_description.clone()), ..Default::default() },
            StepMutation::SetFileName { file_name } => StepDiff { file_name: (base.header.file_name != *file_name).then(|| file_name.clone()), ..Default::default() },
            StepMutation::SetFileSchema { file_schema } => StepDiff { file_schema: (base.header.file_schema != *file_schema).then(|| file_schema.clone()), ..Default::default() },

            StepMutation::InsertEntity { index, entity } => StepDiff { entities: Some(StepEntitiesDiff { added: vec![StepEntityAdded { index: *index, entity: entity.clone() }], ..Default::default() }), ..Default::default() },

            StepMutation::RemoveEntity { id } => {
                if base.entities.iter().any(|e| e.id == *id) {
                    StepDiff { entities: Some(StepEntitiesDiff { removed: vec![*id], ..Default::default() }), ..Default::default() }
                } else {
                    StepDiff::default()
                }
            }

            StepMutation::SetEntityName { id, name } => match base.entities.iter().find(|e| e.id == *id) {
                Some(e) if e.name != *name => {
                    StepDiff { entities: Some(StepEntitiesDiff { modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { name: Some(name.clone()), ..Default::default() } }], ..Default::default() }), ..Default::default() }
                }
                _ => StepDiff::default(),
            },

            StepMutation::SetEntityArg { id, arg_index, value } => match base.entities.iter().find(|e| e.id == *id) {
                Some(e) if e.args.get(*arg_index).map(|v| v != value).unwrap_or(false) => StepDiff {
                    entities: Some(StepEntitiesDiff {
                        modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { modified: vec![StepArgModified { index: *arg_index, value: value.clone() }], ..Default::default() }), ..Default::default() } }],
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
                            modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { added: vec![StepArgAdded { index: *arg_index, value: value.clone() }], ..Default::default() }), ..Default::default() } }],
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
                        modified: vec![StepEntityModified { id: *id, diff: StepEntityDiff { args: Some(StepArgsDiff { removed: vec![*arg_index], ..Default::default() }), ..Default::default() } }],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                _ => StepDiff::default(),
            },
        })
    }

    async fn inverse(&self, base: &StepSnapshot) -> Vec<Self> {
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

            StepMutation::SetEntityArg { id, arg_index, .. } => match base.entities.iter().find(|e| e.id == *id).and_then(|e| e.args.get(*arg_index)) {
                Some(v) => vec![StepMutation::SetEntityArg { id: *id, arg_index: *arg_index, value: v.clone() }],
                None => vec![StepMutation::NoMutation],
            },

            StepMutation::InsertEntityArg { id, arg_index, .. } => vec![StepMutation::RemoveEntityArg { id: *id, arg_index: *arg_index }],

            StepMutation::RemoveEntityArg { id, arg_index } => match base.entities.iter().find(|e| e.id == *id).and_then(|e| e.args.get(*arg_index)) {
                Some(v) => vec![StepMutation::InsertEntityArg { id: *id, arg_index: *arg_index, value: v.clone() }],
                None => vec![StepMutation::NoMutation],
            },
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
async fn print_step_mutation(m: &StepMutation) -> String {
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
async fn parse_step_mutation(line: &str) -> Result<StepMutation, String> {
    if line == "no-mutation" {
        return Ok(StepMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("step mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
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
    async fn print_op(&self) -> String {
        print_step_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_step_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `StepMutation` variant ordinal, same 0-10 order `print_step_mutation`'s own keyword match uses.
/// Reuses `StepDiff`'s `pub(crate)` recursive `enc_value_bin`/`enc_entity_bin`/
/// `enc_step_snapshot_bin`/`write_str_bin` primitives (`../../🔺️diff/🦀️component.rs`, imported
/// above) — same intra-artifact-reuse split the TEXT codec above already uses.
impl OpBinary for StepMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            StepMutation::NoMutation => 0,
            StepMutation::SetSnapshot { .. } => 1,
            StepMutation::SetFileDescription { .. } => 2,
            StepMutation::SetFileName { .. } => 3,
            StepMutation::SetFileSchema { .. } => 4,
            StepMutation::InsertEntity { .. } => 5,
            StepMutation::RemoveEntity { .. } => 6,
            StepMutation::SetEntityName { .. } => 7,
            StepMutation::SetEntityArg { .. } => 8,
            StepMutation::InsertEntityArg { .. } => 9,
            StepMutation::RemoveEntityArg { .. } => 10,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            StepMutation::NoMutation => {}
            StepMutation::SetSnapshot { snapshot } => enc_step_snapshot_bin(snapshot, &mut out),
            StepMutation::SetFileDescription { file_description } => enc_file_description_bin(file_description, &mut out),
            StepMutation::SetFileName { file_name } => enc_file_name_bin(file_name, &mut out),
            StepMutation::SetFileSchema { file_schema } => enc_file_schema_bin(file_schema, &mut out),
            StepMutation::InsertEntity { index, entity } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_entity_bin(entity, &mut out);
            }
            StepMutation::RemoveEntity { id } => store::pack_rt::write_varint_u64(&mut out, *id),
            StepMutation::SetEntityName { id, name } => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                write_str_bin(&mut out, name);
            }
            StepMutation::SetEntityArg { id, arg_index, value } => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *arg_index as u64);
                enc_value_bin(value, &mut out);
            }
            StepMutation::InsertEntityArg { id, arg_index, value } => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *arg_index as u64);
                enc_value_bin(value, &mut out);
            }
            StepMutation::RemoveEntityArg { id, arg_index } => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *arg_index as u64);
            }
        }
        Ok(out)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(StepMutation::NoMutation),
            1 => {
                let snapshot = dec_step_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(StepMutation::SetSnapshot { snapshot })
            }
            2 => {
                let file_description = dec_file_description_bin(&mut reader).map_err(|e| malformed("op file_description", reader.position(), e))?;
                Ok(StepMutation::SetFileDescription { file_description })
            }
            3 => {
                let file_name = dec_file_name_bin(&mut reader).map_err(|e| malformed("op file_name", reader.position(), e))?;
                Ok(StepMutation::SetFileName { file_name })
            }
            4 => {
                let file_schema = dec_file_schema_bin(&mut reader).map_err(|e| malformed("op file_schema", reader.position(), e))?;
                Ok(StepMutation::SetFileSchema { file_schema })
            }
            5 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(StepMutation::InsertEntity { index, entity })
            }
            6 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                Ok(StepMutation::RemoveEntity { id })
            }
            7 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let name = read_str_bin(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(StepMutation::SetEntityName { id, name })
            }
            8 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let arg_index = reader.read_varint_u64().map_err(|e| malformed("op arg_index", reader.position(), e.to_string()))? as usize;
                let value = dec_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(StepMutation::SetEntityArg { id, arg_index, value })
            }
            9 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let arg_index = reader.read_varint_u64().map_err(|e| malformed("op arg_index", reader.position(), e.to_string()))? as usize;
                let value = dec_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(StepMutation::InsertEntityArg { id, arg_index, value })
            }
            10 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let arg_index = reader.read_varint_u64().map_err(|e| malformed("op arg_index", reader.position(), e.to_string()))? as usize;
                Ok(StepMutation::RemoveEntityArg { id, arg_index })
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: one representative `StepMutation` per variant, real `print_op()`-conformance-law
/// fodder (`ops_grammar_conformance_law`) and `protocol_walk_law` fodder — every `StepValue` tag
/// (incl. the recursive `Aggregate`/`TypedValue` cases) and `InsertEntity`'s bare `StepEntity`
/// payload are exercised at least once.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<StepMutation> {
    use crate::artifacts::step::schema::snapshot::{StepFileDescription, StepFileName, StepFileSchema, StepValue as SV};
    let demo_entity = |id: u64, name: &str, args: Vec<StepValue>| StepEntity { id, name: name.into(), args, complex: Vec::new() };
    vec![
        StepMutation::NoMutation,
        StepMutation::SetSnapshot { snapshot: crate::artifacts::step::engine::demo_step_snapshot() },
        StepMutation::SetFileDescription { file_description: StepFileDescription { description: vec!["demo".into()], implementation_level: "2;1".into() } },
        StepMutation::SetFileName {
            file_name: StepFileName {
                name: "demo.step".into(),
                timestamp: "2026-08-11T00:00:00".into(),
                author: vec!["Ueli".into()],
                organization: vec!["semio".into()],
                preprocessor_version: "semio".into(),
                originating_system: "".into(),
                authorization: "".into(),
            },
        },
        StepMutation::SetFileSchema { file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] } },
        StepMutation::InsertEntity {
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
                    SV::TypedValue { type_name: "IFCLENGTHMEASURE".into(), value: Box::new(SV::Real(3000.0)) },
                ],
            ),
        },
        StepMutation::RemoveEntity { id: 2 },
        StepMutation::SetEntityName { id: 1, name: "RENAMED".into() },
        StepMutation::SetEntityArg { id: 1, arg_index: 1, value: SV::Aggregate(vec![SV::Real(1.0), SV::Real(2.0), SV::Real(3.0)]) },
        StepMutation::InsertEntityArg { id: 1, arg_index: 2, value: SV::TypedValue { type_name: "X".into(), value: Box::new(SV::Aggregate(vec![SV::Integer(1), SV::Integer(2)])) } },
        StepMutation::RemoveEntityArg { id: 1, arg_index: 0 },
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::step::schema::snapshot::{StepHeader, StepValue as SV};

    async fn entity(id: u64, name: &str, args: Vec<StepValue>) -> StepEntity {
        StepEntity { id, name: name.into(), args, complex: Vec::new() }
    }

    async fn base_snapshot() -> StepSnapshot {
        StepSnapshot {
            schema: crate::artifacts::step::STDIO_STEP_DOCUMENT_SCHEMA.into(),
            header: StepHeader::default(),
            entities: vec![entity(1, "CARTESIAN_POINT", vec![SV::String("".into()), SV::Real(1.0)]), entity(2, "DIRECTION", vec![SV::Unset])],
        }
    }

    /// 🧪️ `mutation_diff_law`: ∀ variant, `m.diff(base).diff().apply(base) == { apply(&mut s, m); s }`
    /// and the returned diff equals `m.diff(base)`.
    async fn assert_mutation_diff_law(base: &StepSnapshot, m: StepMutation) {
        let expected_diff = <StepMutation as Mutation<StepSnapshot>>::diff(&m, base);
        let expected_state = expected_diff.diff().apply(base).expect("valid mutation diff");
        let mut actual_state = base.clone();
        let actual_diff = apply_step_mutation(&mut actual_state, &m);
        assert_eq!(actual_diff, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
        assert_eq!(actual_state, expected_state, "applied state must match for {m:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_covers_every_variant() {
        let base = base_snapshot();
        assert_mutation_diff_law(&base, StepMutation::NoMutation);
        let mut next = base.clone();
        next.entities[0].name = "X".into();
        assert_mutation_diff_law(&base, StepMutation::SetSnapshot { snapshot: next });
        assert_mutation_diff_law(&base, StepMutation::SetFileDescription { file_description: StepFileDescription { description: vec!["d".into()], implementation_level: "2;1".into() } });
        assert_mutation_diff_law(&base, StepMutation::SetFileName { file_name: StepFileName { name: "n".into(), ..Default::default() } });
        assert_mutation_diff_law(&base, StepMutation::SetFileSchema { file_schema: StepFileSchema { schemas: vec!["X".into()] } });
        assert_mutation_diff_law(&base, StepMutation::InsertEntity { index: 1, entity: entity(50, "NEW", vec![]) });
        assert_mutation_diff_law(&base, StepMutation::RemoveEntity { id: 2 });
        assert_mutation_diff_law(&base, StepMutation::SetEntityName { id: 1, name: "RENAMED".into() });
        assert_mutation_diff_law(&base, StepMutation::SetEntityArg { id: 1, arg_index: 1, value: SV::Real(9.0) });
        assert_mutation_diff_law(&base, StepMutation::InsertEntityArg { id: 1, arg_index: 2, value: SV::Enum("T".into()) });
        assert_mutation_diff_law(&base, StepMutation::RemoveEntityArg { id: 1, arg_index: 0 });
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_and_out_of_range_targets_are_rejected_without_mutating() {
        let base = base_snapshot();
        let mut snapshot = base.clone();
        let outcome = apply_step_mutation(&mut snapshot, &StepMutation::RemoveEntity { id: 999 });
        assert_eq!(snapshot, base);
        assert_eq!(outcome.messages()[0].target, vec!["entities", "999"]);
        let outcome = apply_step_mutation(&mut snapshot, &StepMutation::RemoveEntityArg { id: 1, arg_index: 99 });
        assert_eq!(snapshot, base);
        assert_eq!(outcome.messages()[0].target, vec!["entities", "1", "args", "99"]);
    }

    /// 🧪️ `inverse_law` (mutation level): every variant's `inverse()` round-trips.
    #[semio_framework_async_macros::async_test]
    async fn inverse_law_mutation_level_round_trips_every_variant() {
        let base = base_snapshot();
        let variants = vec![
            StepMutation::SetFileSchema { file_schema: StepFileSchema { schemas: vec!["CONFIG_CONTROL_DESIGN".into()] } },
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
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            StepMutation::NoMutation,
            StepMutation::SetSnapshot { snapshot: base.clone() },
            StepMutation::SetFileDescription { file_description: StepFileDescription { description: vec!["d1".into(), "d2".into()], implementation_level: "2;1".into() } },
            StepMutation::SetFileName {
                file_name: StepFileName {
                    name: "n.step".into(),
                    timestamp: "2026-08-10T00:00:00".into(),
                    author: vec!["A".into()],
                    organization: vec!["O".into()],
                    preprocessor_version: "pv".into(),
                    originating_system: "sys".into(),
                    authorization: "auth".into(),
                },
            },
            StepMutation::SetFileSchema { file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into(), "CONFIG_CONTROL_DESIGN".into()] } },
            StepMutation::InsertEntity {
                index: 1,
                entity: entity(
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
                        SV::TypedValue { type_name: "IFCLENGTHMEASURE".into(), value: Box::new(SV::Real(3000.0)) },
                    ],
                ),
            },
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
