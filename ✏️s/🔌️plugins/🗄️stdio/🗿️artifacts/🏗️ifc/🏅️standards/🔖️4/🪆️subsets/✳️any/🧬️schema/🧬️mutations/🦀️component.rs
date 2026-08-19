//! 🧬️ IfcMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal `{NoMutation, SetSnapshot}` stub — HEADER scalar setters plus
//! `InsertEntity`/`RemoveEntity`/`SetEntityName`/`SetEntityArg`/`InsertEntityArg`/`RemoveEntityArg`
//! for the id-keyed `entities` collection and its per-entity positional `args`. Every variant's
//! `diff()` is handcrafted (constructs `IfcDiff` directly via the `schema::diff` builders) —
//! apply-and-capture is never used.

use crate::artifacts::ifc::schema::diff::{
    self, dec_entity, dec_entity_bin, dec_entity_list_bin, dec_ifc_value, dec_ifc_value_bin, dec_ifc_value_list, dec_ifc_value_list_bin, dec_str, enc_entity, enc_entity_bin, enc_entity_list_bin, enc_ifc_value, enc_ifc_value_bin, enc_ifc_value_list,
    enc_ifc_value_list_bin, enc_str, read_str_bin, split_top_level, strip_brackets, write_str_bin, IfcDiff,
};
use crate::artifacts::ifc::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};
use crate::artifacts::ifc::IfcSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.ifc`.
/// 🧪️ F6 CONFIRMED: `#[derive(dsl::DslOps)]` on this enum ALSO fails (independent confirmation
/// beyond `IfcDiff`'s `DiffCodec` blocker — see that file's doc comment), real `cargo check -p
/// semio-s-plugin-stdio --lib` output, verbatim:
/// ```text
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcValue: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️component.rs:27:21   (SetFileDescription { values: Vec<IfcValue> })
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcSnapshot: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️component.rs:23:19   (SetSnapshot { snapshot: IfcSnapshot })
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcEntity: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️component.rs:40:17   (InsertEntity { entity: IfcEntity })
/// ```
/// Same root cause as `IfcDiff` (§3a): `IfcValue` carries fields on 7 of its 9 variants, has no
/// `DslField` impl, and every variant here either carries it directly (`values`/`value`) or
/// transitively via `IfcSnapshot`/`IfcEntity` (`SetSnapshot`/`InsertEntity`). `OpText`/`OpBinary`
/// hand-rolled below, reusing `IfcDiff`'s `pub(crate)` grammar primitives
/// (`enc_str`/`enc_ifc_value`/`enc_entity`/`split_top_level`/...).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum IfcMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: IfcSnapshot,
    },
    /// 📇️ Sets the `FILE_DESCRIPTION` header record's raw value tuple.
    SetFileDescription {
        values: Vec<IfcValue>,
    },
    /// 📇️ Sets the `FILE_NAME` header record's raw value tuple.
    SetFileName {
        values: Vec<IfcValue>,
    },
    /// 📇️ Sets the `FILE_SCHEMA` header record's raw value tuple.
    SetFileSchema {
        values: Vec<IfcValue>,
    },
    /// ➕️ Inserts a fully-specified entity at `index` (final position, clamped to `len`).
    InsertEntity {
        index: usize,
        entity: IfcEntity,
    },
    /// ➖️ Removes the entity with id `id` (no-op if absent).
    RemoveEntity {
        id: u64,
    },
    /// 🏷️ Sets entity `id`'s EXPRESS type keyword (e.g. `"IFCWALL"`).
    SetEntityName {
        id: u64,
        name: String,
    },
    /// 📝️ Replaces the argument at `index` of entity `id`'s positional arg list.
    SetEntityArg {
        id: u64,
        index: usize,
        value: IfcValue,
    },
    /// ➕️ Inserts a new argument at `index` (final position) of entity `id`'s arg list.
    InsertEntityArg {
        id: u64,
        index: usize,
        value: IfcValue,
    },
    /// ➖️ Removes the argument at `index` of entity `id`'s arg list.
    RemoveEntityArg {
        id: u64,
        index: usize,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning a typed error outcome without changing the
/// snapshot when an entity or argument target is missing or out of range.
pub async fn apply_ifc_mutation(snapshot: &mut IfcSnapshot, mutation: &IfcMutation) -> protocol::MutationOutcome<IfcDiff> {
    let outcome = <IfcMutation as Mutation<IfcSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<IfcSnapshot> for IfcMutation {
    type Diff = IfcDiff;

    async fn diff(&self, base: &IfcSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            IfcMutation::NoMutation => IfcDiff::default(),
            IfcMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            IfcMutation::SetFileDescription { values } => diff::diff_set_file_description(values.clone()),
            IfcMutation::SetFileName { values } => diff::diff_set_file_name(values.clone()),
            IfcMutation::SetFileSchema { values } => diff::diff_set_file_schema(values.clone()),
            IfcMutation::InsertEntity { index, entity } => diff::diff_insert_entity(*index, entity.clone()),
            IfcMutation::RemoveEntity { id } => diff::diff_remove_entity(*id),
            IfcMutation::SetEntityName { id, name } => diff::diff_set_entity_name(*id, name),
            IfcMutation::SetEntityArg { id, index, value } => diff::diff_set_entity_arg(*id, *index, value.clone()),
            IfcMutation::InsertEntityArg { id, index, value } => diff::diff_insert_entity_arg(*id, *index, value.clone()),
            IfcMutation::RemoveEntityArg { id, index } => diff::diff_remove_entity_arg(*id, *index),
        })
    }

    /// ↩️ Handcrafted, key-aware mutation-level inverses. Entity/arg-targeted variants look the
    /// prior value up in `base`; a stale/absent id/index inverts to `NoMutation` (nothing to undo).
    async fn inverse(&self, base: &IfcSnapshot) -> Vec<Self> {
        let entity = |id: u64| base.entities.iter().find(|e| e.id == id);
        match self {
            IfcMutation::NoMutation => vec![IfcMutation::NoMutation],
            IfcMutation::SetSnapshot { .. } => vec![IfcMutation::SetSnapshot { snapshot: base.clone() }],
            IfcMutation::SetFileDescription { .. } => vec![IfcMutation::SetFileDescription { values: base.header.file_description.clone() }],
            IfcMutation::SetFileName { .. } => vec![IfcMutation::SetFileName { values: base.header.file_name.clone() }],
            IfcMutation::SetFileSchema { .. } => vec![IfcMutation::SetFileSchema { values: base.header.file_schema.clone() }],
            IfcMutation::InsertEntity { entity, .. } => vec![IfcMutation::RemoveEntity { id: entity.id }],
            IfcMutation::RemoveEntity { id } => match base.entities.iter().position(|e| e.id == *id) {
                Some(index) => vec![IfcMutation::InsertEntity { index, entity: base.entities[index].clone() }],
                None => vec![IfcMutation::NoMutation],
            },
            IfcMutation::SetEntityName { id, .. } => match entity(*id) {
                Some(e) => vec![IfcMutation::SetEntityName { id: *id, name: e.name.clone() }],
                None => vec![IfcMutation::NoMutation],
            },
            IfcMutation::SetEntityArg { id, index, .. } => match entity(*id).and_then(|e| e.args.get(*index)) {
                Some(v) => vec![IfcMutation::SetEntityArg { id: *id, index: *index, value: v.clone() }],
                None => vec![IfcMutation::NoMutation],
            },
            IfcMutation::InsertEntityArg { id, index, .. } => vec![IfcMutation::RemoveEntityArg { id: *id, index: *index }],
            IfcMutation::RemoveEntityArg { id, index } => match entity(*id).and_then(|e| e.args.get(*index)) {
                Some(v) => vec![IfcMutation::InsertEntityArg { id: *id, index: *index, value: v.clone() }],
                None => vec![IfcMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `IfcMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above) — reuses `IfcDiff`'s `pub(crate)` grammar primitives
/// (`enc_str`/`enc_ifc_value`/`enc_entity`/`split_top_level`/`encode_option`/...) rather than
/// duplicating them a second time in this file. Grammar: `keyword arg=value ...` (space-separated,
/// same shape the derive's own handcrafted-wrapper convention uses), one match arm per variant (no
/// `DslVariants` scaffolding available since nothing here derives it).
async fn enc_ifc_header(h: &IfcHeader) -> String {
    format!("[{},{},{}]", enc_ifc_value_list(&h.file_description), enc_ifc_value_list(&h.file_name), enc_ifc_value_list(&h.file_schema))
}
async fn dec_ifc_header(s: &str) -> Result<IfcHeader, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [fd, fname, fs] = parts.as_slice() else { return Err(format!("ifc header: expected 3 fields, got {}", parts.len())) };
    Ok(IfcHeader { file_description: dec_ifc_value_list(fd)?, file_name: dec_ifc_value_list(fname)?, file_schema: dec_ifc_value_list(fs)? })
}
async fn enc_ifc_snapshot(s: &IfcSnapshot) -> String {
    let entities = s.entities.iter().map(enc_entity).collect::<Vec<_>>().join(",");
    format!("[{},{},[{}]]", enc_str(&s.schema), enc_ifc_header(&s.header), entities)
}
async fn dec_ifc_snapshot(s: &str) -> Result<IfcSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, header, entities] = parts.as_slice() else { return Err(format!("ifc snapshot: expected 3 fields, got {}", parts.len())) };
    let entities = split_top_level(strip_brackets(entities)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_entity).collect::<Result<Vec<_>, String>>()?;
    Ok(IfcSnapshot { schema: dec_str(schema)?, header: dec_ifc_header(header)?, entities })
}

async fn print_ifc_mutation(m: &IfcMutation) -> String {
    match m {
        IfcMutation::NoMutation => "no-mutation".to_string(),
        IfcMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_ifc_snapshot(snapshot)),
        IfcMutation::SetFileDescription { values } => format!("set-file-description values={}", enc_ifc_value_list(values)),
        IfcMutation::SetFileName { values } => format!("set-file-name values={}", enc_ifc_value_list(values)),
        IfcMutation::SetFileSchema { values } => format!("set-file-schema values={}", enc_ifc_value_list(values)),
        IfcMutation::InsertEntity { index, entity } => format!("insert-entity index={index} entity={}", enc_entity(entity)),
        IfcMutation::RemoveEntity { id } => format!("remove-entity id={id}"),
        IfcMutation::SetEntityName { id, name } => format!("set-entity-name id={id} name={}", enc_str(name)),
        IfcMutation::SetEntityArg { id, index, value } => format!("set-entity-arg id={id} index={index} value={}", enc_ifc_value(value)),
        IfcMutation::InsertEntityArg { id, index, value } => format!("insert-entity-arg id={id} index={index} value={}", enc_ifc_value(value)),
        IfcMutation::RemoveEntityArg { id, index } => format!("remove-entity-arg id={id} index={index}"),
    }
}
async fn parse_ifc_mutation(line: &str) -> Result<IfcMutation, String> {
    if line == "no-mutation" {
        return Ok(IfcMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("ifc mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("ifc mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    let u64_arg = |k: &str| -> Result<u64, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(IfcMutation::SetSnapshot { snapshot: dec_ifc_snapshot(arg("snapshot")?)? }),
        "set-file-description" => Ok(IfcMutation::SetFileDescription { values: dec_ifc_value_list(arg("values")?)? }),
        "set-file-name" => Ok(IfcMutation::SetFileName { values: dec_ifc_value_list(arg("values")?)? }),
        "set-file-schema" => Ok(IfcMutation::SetFileSchema { values: dec_ifc_value_list(arg("values")?)? }),
        "insert-entity" => Ok(IfcMutation::InsertEntity { index: usize_arg("index")?, entity: dec_entity(arg("entity")?)? }),
        "remove-entity" => Ok(IfcMutation::RemoveEntity { id: u64_arg("id")? }),
        "set-entity-name" => Ok(IfcMutation::SetEntityName { id: u64_arg("id")?, name: dec_str(arg("name")?)? }),
        "set-entity-arg" => Ok(IfcMutation::SetEntityArg { id: u64_arg("id")?, index: usize_arg("index")?, value: dec_ifc_value(arg("value")?)? }),
        "insert-entity-arg" => Ok(IfcMutation::InsertEntityArg { id: u64_arg("id")?, index: usize_arg("index")?, value: dec_ifc_value(arg("value")?)? }),
        "remove-entity-arg" => Ok(IfcMutation::RemoveEntityArg { id: u64_arg("id")?, index: usize_arg("index")? }),
        other => Err(format!("ifc mutation: unknown keyword {other:?}")),
    }
}

impl OpText for IfcMutation {
    async fn print_op(&self) -> String {
        print_ifc_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_ifc_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ P2-FG1: mutation-specific real binary primitives backing the upgraded `OpBinary` impl below
/// — reuses `IfcDiff`'s `pub(crate)` recursive `enc_entity_bin`/`enc_ifc_value_list_bin`/
/// `write_str_bin` primitives (`../../🔺️diff/🦀️component.rs`, imported above) for the SHARED
/// `IfcEntity`/`IfcValue` shape (same intra-artifact-reuse split the TEXT codec above already
/// uses), only `IfcHeader`/`IfcSnapshot`'s own binary shape is genuinely new here.
async fn enc_ifc_header_bin(h: &IfcHeader, out: &mut Vec<u8>) {
    enc_ifc_value_list_bin(&h.file_description, out);
    enc_ifc_value_list_bin(&h.file_name, out);
    enc_ifc_value_list_bin(&h.file_schema, out);
}
async fn dec_ifc_header_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcHeader, String> {
    let file_description = dec_ifc_value_list_bin(reader)?;
    let file_name = dec_ifc_value_list_bin(reader)?;
    let file_schema = dec_ifc_value_list_bin(reader)?;
    Ok(IfcHeader { file_description, file_name, file_schema })
}
async fn enc_ifc_snapshot_bin(s: &IfcSnapshot, out: &mut Vec<u8>) {
    write_str_bin(out, &s.schema);
    enc_ifc_header_bin(&s.header, out);
    enc_entity_list_bin(&s.entities, out);
}
async fn dec_ifc_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcSnapshot, String> {
    let schema = read_str_bin(reader)?;
    let header = dec_ifc_header_bin(reader)?;
    let entities = dec_entity_list_bin(reader)?;
    Ok(IfcSnapshot { schema, header, entities })
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut (`IfcMutation` was one of 4
/// of stdio's 7 FG1 standards still on that shortcut per this wave's own P2-FG1 census). `tag` is
/// the `IfcMutation` variant ordinal, same 0-10 order `parse_ifc_mutation`'s own keyword match
/// uses. Every field is real (`id`/`index` varints, `IfcEntity`/`IfcValue` field-by-field via the
/// reused diff-sibling primitives) — the only place the recursion bottoms out through a fully
/// spec-expressible per-variant tag (`enc_ifc_value_bin`), never an opaque byte-chain fallback.
impl OpBinary for IfcMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            IfcMutation::NoMutation => 0,
            IfcMutation::SetSnapshot { .. } => 1,
            IfcMutation::SetFileDescription { .. } => 2,
            IfcMutation::SetFileName { .. } => 3,
            IfcMutation::SetFileSchema { .. } => 4,
            IfcMutation::InsertEntity { .. } => 5,
            IfcMutation::RemoveEntity { .. } => 6,
            IfcMutation::SetEntityName { .. } => 7,
            IfcMutation::SetEntityArg { .. } => 8,
            IfcMutation::InsertEntityArg { .. } => 9,
            IfcMutation::RemoveEntityArg { .. } => 10,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            IfcMutation::NoMutation => {}
            IfcMutation::SetSnapshot { snapshot } => enc_ifc_snapshot_bin(snapshot, &mut out),
            IfcMutation::SetFileDescription { values } => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::SetFileName { values } => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::SetFileSchema { values } => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::InsertEntity { index, entity } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_entity_bin(entity, &mut out);
            }
            IfcMutation::RemoveEntity { id } => store::pack_rt::write_varint_u64(&mut out, *id),
            IfcMutation::SetEntityName { id, name } => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                write_str_bin(&mut out, name);
            }
            IfcMutation::SetEntityArg { id, index, value } => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_ifc_value_bin(value, &mut out);
            }
            IfcMutation::InsertEntityArg { id, index, value } => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_ifc_value_bin(value, &mut out);
            }
            IfcMutation::RemoveEntityArg { id, index } => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
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
            0 => Ok(IfcMutation::NoMutation),
            1 => {
                let snapshot = dec_ifc_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(IfcMutation::SetSnapshot { snapshot })
            }
            2 => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileDescription { values })
            }
            3 => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileName { values })
            }
            4 => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileSchema { values })
            }
            5 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(IfcMutation::InsertEntity { index, entity })
            }
            6 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                Ok(IfcMutation::RemoveEntity { id })
            }
            7 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let name = read_str_bin(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(IfcMutation::SetEntityName { id, name })
            }
            8 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_ifc_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(IfcMutation::SetEntityArg { id, index, value })
            }
            9 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_ifc_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(IfcMutation::InsertEntityArg { id, index, value })
            }
            10 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(IfcMutation::RemoveEntityArg { id, index })
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: one representative `IfcMutation` per variant, real `print_op()`-conformance-law
/// fodder (`ops_grammar_conformance_law`) and `protocol_walk_law` fodder — every `IfcValue` tag
/// (incl. the recursive `Aggregate`/`TypedValue` cases) and `InsertEntity`'s bare `IfcEntity`
/// payload are exercised at least once.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<IfcMutation> {
    let demo_entity = |id: u64, name: &str, args: Vec<IfcValue>| IfcEntity { id, name: name.into(), args, complex: Vec::new() };
    vec![
        IfcMutation::NoMutation,
        IfcMutation::SetSnapshot { snapshot: crate::artifacts::ifc::engine::demo_ifc_snapshot() },
        IfcMutation::SetFileDescription { values: vec![IfcValue::String("demo".into())] },
        IfcMutation::SetFileName { values: vec![IfcValue::String("demo.ifc".into())] },
        IfcMutation::SetFileSchema { values: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] },
        IfcMutation::InsertEntity {
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
                    IfcValue::TypedValue("IFCLENGTHMEASURE".into(), vec![IfcValue::Real(3000.0)]),
                ],
            ),
        },
        IfcMutation::RemoveEntity { id: 2 },
        IfcMutation::SetEntityName { id: 1, name: "IFCSLAB".into() },
        IfcMutation::SetEntityArg { id: 1, index: 1, value: IfcValue::String("Wall-02".into()) },
        IfcMutation::InsertEntityArg { id: 1, index: 2, value: IfcValue::Derived },
        IfcMutation::RemoveEntityArg { id: 1, index: 0 },
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::ifc::schema::diff::IfcEntitiesDiff;
    use crate::artifacts::ifc::schema::snapshot::{IfcComplexType, IfcHeader};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    #[semio_framework_async_macros::async_test]
    async fn missing_entity_target_is_rejected_before_mutation() {
        let base = IfcSnapshot::default();
        let diff = IfcDiff { entities: Some(IfcEntitiesDiff { removed: vec![1], ..Default::default() }), ..Default::default() };
        let error = diff.apply(&base).expect_err("missing entity target must be rejected");
        assert_eq!(error.code, "invalid-remove-target");
        assert_eq!(error.target, vec!["entities", "1"]);
        assert_eq!(base, IfcSnapshot::default());
    }

    //#region Fixtures
    async fn entity(id: u64, name: &str, args: Vec<IfcValue>) -> IfcEntity {
        IfcEntity { id, name: name.into(), args, complex: vec![] }
    }

    async fn base_snapshot() -> IfcSnapshot {
        IfcSnapshot {
            schema: "stdio.ifc".into(),
            header: IfcHeader { file_description: vec![IfcValue::String("".into())], file_name: vec![IfcValue::String("semio.ifc".into())], file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] },
            entities: vec![
                entity(1, "IFCPROJECT", vec![IfcValue::String("gid".into()), IfcValue::Reference(2)]),
                entity(2, "IFCOWNERHISTORY", vec![IfcValue::Unset, IfcValue::Integer(0)]),
                entity(6, "IFCWALL", vec![IfcValue::String("gid-wall".into()), IfcValue::Reference(2), IfcValue::String("Wall-01".into())]),
            ],
        }
    }
    //#endregion Fixtures

    //#region 🔖️mutation_diff_law
    async fn assert_mutation_diff_law(base: &IfcSnapshot, mutation: IfcMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_ifc_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_ifc_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.diff().apply(base).expect("valid mutation diff"), applied_snapshot, "diff.diff().apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        assert_mutation_diff_law(&base, IfcMutation::NoMutation);
        let mut alt = base.clone();
        alt.header.file_name = vec![IfcValue::String("other.ifc".into())];
        assert_mutation_diff_law(&base, IfcMutation::SetSnapshot { snapshot: alt });
        assert_mutation_diff_law(&base, IfcMutation::SetFileDescription { values: vec![IfcValue::String("new desc".into())] });
        assert_mutation_diff_law(&base, IfcMutation::SetFileName { values: vec![IfcValue::String("renamed.ifc".into())] });
        assert_mutation_diff_law(&base, IfcMutation::SetFileSchema { values: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3".into())])] });
        assert_mutation_diff_law(&base, IfcMutation::InsertEntity { index: 1, entity: entity(99, "IFCSITE", vec![IfcValue::Unset]) });
        assert_mutation_diff_law(&base, IfcMutation::RemoveEntity { id: 2 });
        assert_mutation_diff_law(&base, IfcMutation::SetEntityName { id: 6, name: "IFCSLAB".into() });
        assert_mutation_diff_law(&base, IfcMutation::SetEntityArg { id: 6, index: 2, value: IfcValue::String("Wall-02".into()) });
        assert_mutation_diff_law(&base, IfcMutation::InsertEntityArg { id: 6, index: 1, value: IfcValue::Derived });
        assert_mutation_diff_law(&base, IfcMutation::RemoveEntityArg { id: 6, index: 0 });
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            IfcMutation::NoMutation,
            IfcMutation::SetFileDescription { values: vec![IfcValue::String("changed".into())] },
            IfcMutation::InsertEntity { index: 1, entity: entity(99, "IFCSITE", vec![IfcValue::Unset]) },
            IfcMutation::RemoveEntity { id: 2 },
            IfcMutation::SetEntityName { id: 6, name: "IFCSLAB".into() },
            IfcMutation::SetEntityArg { id: 6, index: 2, value: IfcValue::String("Wall-02".into()) },
            IfcMutation::InsertEntityArg { id: 6, index: 1, value: IfcValue::Derived },
            IfcMutation::RemoveEntityArg { id: 6, index: 0 },
        ];
        for m in variants {
            let mut snap = base.clone();
            apply_ifc_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_ifc_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            let d = m.diff(&base);
            let mutated = d.diff().apply(&base).expect("valid forward diff");
            let inv_d = d.diff().inverse(&base);
            assert_eq!(inv_d.apply(&mutated).expect("valid inverse diff"), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    async fn assert_absorb_law(base: &IfcSnapshot, m1: IfcMutation, m2: IfcMutation) {
        let d1 = m1.diff(base);
        let mid = d1.diff().apply(base).expect("valid first diff");
        let d2 = m2.diff(&mid);
        let sequential = d2.diff().apply(&mid).expect("valid second diff");

        let mut merged = d1.diff().clone();
        merged.absorb(d2.diff().clone());
        assert_eq!(merged.apply(base).expect("valid absorbed diff"), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = base_snapshot();

        // Insert+Remove-before: added entity's carried final index shifts once an earlier base
        // survivor is removed by the second mutation (the recipe's own canonical shift case).
        assert_absorb_law(&base, IfcMutation::InsertEntity { index: 1, entity: entity(100, "IFCSITE", vec![]) }, IfcMutation::RemoveEntity { id: 1 });

        // Insert+Insert-same-index: both survive.
        assert_absorb_law(&base, IfcMutation::InsertEntity { index: 1, entity: entity(100, "IFCSITE", vec![]) }, IfcMutation::InsertEntity { index: 1, entity: entity(101, "IFCBUILDING", vec![]) });

        // Add+SetField: the second mutation patches directly into the still-pending added entity.
        assert_absorb_law(&base, IfcMutation::InsertEntity { index: 0, entity: entity(100, "IFCSITE", vec![IfcValue::Unset]) }, IfcMutation::SetEntityName { id: 100, name: "IFCBUILDING".into() });

        // Modify+Remove: a pending field patch on a since-removed base entity vanishes.
        assert_absorb_law(&base, IfcMutation::SetEntityName { id: 6, name: "IFCSLAB".into() }, IfcMutation::RemoveEntity { id: 6 });

        // Insert then annihilate the very same insert.
        assert_absorb_law(&base, IfcMutation::InsertEntity { index: 0, entity: entity(100, "IFCSITE", vec![]) }, IfcMutation::RemoveEntity { id: 100 });

        // Insert-arg then set-that-same-arg patches into the still-pending added arg.
        assert_absorb_law(&base, IfcMutation::InsertEntityArg { id: 6, index: 0, value: IfcValue::Unset }, IfcMutation::SetEntityArg { id: 6, index: 0, value: IfcValue::Derived });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, IfcMutation::SetFileDescription { values: vec![IfcValue::String("first".into())] }, IfcMutation::SetFileDescription { values: vec![IfcValue::String("second".into())] });
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = IfcMutation::SetFileDescription { values: vec![IfcValue::String("one".into())] }.diff(&base);
        let mid1 = d1.diff().apply(&base).expect("valid first diff");
        let d2 = IfcMutation::InsertEntity { index: 0, entity: entity(100, "IFCSITE", vec![]) }.diff(&mid1);
        let mid2 = d2.diff().apply(&mid1).expect("valid second diff");
        let d3 = IfcMutation::SetEntityName { id: 100, name: "IFCBUILDING".into() }.diff(&mid2);

        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "absorb must associate");
        assert_eq!(left.apply(&base).expect("valid associated diff"), d3.diff().apply(&mid2).expect("valid third diff"), "associated absorb must match full sequential application");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.header.file_name = vec![IfcValue::String("changed.ifc".into())];
        b.entities.remove(0); // remove IFCPROJECT (id 1)
        b.entities[0].name = "IFCOWNERHISTORY2".into(); // modify id 2 (now index 0)
        b.entities.push(entity(200, "IFCBUILDINGSTOREY", vec![IfcValue::Real(3.0)])); // add id 200

        let d = IfcDiff::between(&a, &b);
        assert_eq!(d.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) must equal b");
        let d_rev = IfcDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) must equal a");
        assert!(IfcDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🗿️artifacts/🏗️ifc/📚️examples/🎬️demo/🖼️assets/🏗️example.ifc"));
        let text = match text {
            Ok(t) => t,
            // Fixture path is relative to this crate's manifest dir under the workspace layout;
            // fall back to a synthetic document so this law still exercises decode->encode->decode.
            Err(_) => store::ArtifactDsl::print_dsl(&base_snapshot()),
        };
        let decoded = <IfcSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse fixture");
        let reencoded = store::ArtifactDsl::print_dsl(&decoded);
        let redecoded = <IfcSnapshot as store::ArtifactDsl>::parse_dsl(&reencoded).expect("re-decode fixture");
        assert_eq!(decoded.header, redecoded.header);
        assert_eq!(decoded.entities, redecoded.entities);
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    /// 🌪️ `sweep_a`/`sweep_b` differ in EVERY mutable field: HEADER's three records, one removed
    /// entity, one entity modified in every field (name, an arg removed/modified/added, and
    /// `complex` exercising the COMPLEX-instance weak-list replace), one added entity.
    async fn sweep_a() -> IfcSnapshot {
        IfcSnapshot {
            schema: "stdio.ifc".into(),
            header: IfcHeader { file_description: vec![IfcValue::String("before desc".into())], file_name: vec![IfcValue::String("before.ifc".into())], file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])] },
            entities: vec![
                entity(1, "IFCPROJECT", vec![IfcValue::String("gone".into())]),
                IfcEntity {
                    id: 2,
                    name: "IFCQUANTITYAREA".into(),
                    args: vec![IfcValue::String("stay".into()), IfcValue::Real(1.0), IfcValue::Reference(9)],
                    complex: vec![IfcComplexType { name: "IFCPHYSICALSIMPLEQUANTITY".into(), args: vec![IfcValue::Unset] }],
                },
            ],
        }
    }

    async fn sweep_b() -> IfcSnapshot {
        IfcSnapshot {
            schema: "stdio.ifc".into(),
            header: IfcHeader { file_description: vec![IfcValue::String("after desc".into())], file_name: vec![IfcValue::String("after.ifc".into())], file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3".into())])] },
            entities: vec![
                IfcEntity {
                    id: 2,
                    name: "IFCQUANTITYVOLUME".into(),
                    // index 0 modified, index 1 removed (b is shorter here), index 2 unchanged
                    // relative position collapses -- exercised precisely via direct field asserts
                    // below rather than index-fragile equality.
                    args: vec![IfcValue::String("changed".into()), IfcValue::Reference(9)],
                    complex: vec![],
                },
                entity(300, "IFCBUILDINGSTOREY", vec![IfcValue::Real(3.0)]),
            ],
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = IfcDiff::between(&a, &b);
        assert_eq!(forward.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) must equal b");
        let backward = IfcDiff::between(&b, &a);
        assert_eq!(backward.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) must equal a");
        assert!(IfcDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        assert!(forward.file_description.is_some(), "file_description must be diffed");
        assert!(forward.file_name.is_some(), "file_name must be diffed");
        assert!(forward.file_schema.is_some(), "file_schema must be diffed");

        let ed: &IfcEntitiesDiff = forward.entities.as_ref().expect("entities diff must be present");
        assert_eq!(ed.removed, vec![1u64], "the removed entity (id 1) must be tracked");
        assert_eq!(ed.added.len(), 1, "exactly one entity must be added");
        assert_eq!(ed.added[0].entity.id, 300);
        assert_eq!(ed.modified.len(), 1, "exactly one entity must be modified");
        assert_eq!(ed.modified[0].id, 2);
        let md = &ed.modified[0].diff;
        assert!(md.name.is_some(), "name must be diffed");
        assert!(md.complex.is_some(), "complex must be diffed (non-empty -> empty)");
        let ad = md.args.as_ref().expect("args diff must be present");
        assert!(!ad.modified.is_empty(), "an arg must be modified (index 0)");
        assert!(!ad.removed.is_empty(), "an arg must be removed (a is longer)");

        let backward_ed = backward.entities.as_ref().expect("entities diff must be present");
        assert!(!backward_ed.added.is_empty(), "reverse direction must exercise an added entity (id 1 comes back)");
        let back_md = &backward_ed.modified[0].diff;
        let back_ad = back_md.args.as_ref().expect("args diff must be present");
        assert!(!back_ad.added.is_empty(), "reverse direction must exercise an added arg");
    }
    //#endregion 🔖️field_sweep

    #[semio_framework_async_macros::async_test]
    async fn out_of_range_entity_mutation_is_rejected_without_mutating() {
        let base = base_snapshot();
        let mut snap = base.clone();
        let outcome = apply_ifc_mutation(&mut snap, &IfcMutation::SetEntityName { id: 404, name: "X".into() });
        assert_eq!(snap, base);
        assert_eq!(outcome.messages()[0].target, vec!["entities", "404"]);
        let outcome = apply_ifc_mutation(&mut snap, &IfcMutation::RemoveEntityArg { id: 404, index: 0 });
        assert_eq!(snap, base);
        assert_eq!(outcome.messages()[0].target, vec!["entities", "404"]);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `IfcMutation` grammar —
    /// exercises every variant incl. `SetSnapshot`'s whole-snapshot payload and every `IfcValue`
    /// tag (`Unset`/`Derived`/`Integer`/`Real`/`String`/`Enum`/`Reference`/`Aggregate`/`TypedValue`).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            IfcMutation::NoMutation,
            IfcMutation::SetSnapshot { snapshot: base.clone() },
            IfcMutation::SetFileDescription { values: vec![IfcValue::String("new desc".into())] },
            IfcMutation::SetFileName { values: vec![IfcValue::Aggregate(vec![IfcValue::String("a".into()), IfcValue::Unset])] },
            IfcMutation::SetFileSchema { values: vec![] },
            IfcMutation::InsertEntity {
                index: 1,
                entity: entity(
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
                        IfcValue::TypedValue("IFCLENGTHMEASURE".into(), vec![IfcValue::Real(3000.0)]),
                    ],
                ),
            },
            IfcMutation::RemoveEntity { id: 2 },
            IfcMutation::SetEntityName { id: 6, name: "IFCSLAB".into() },
            IfcMutation::SetEntityArg { id: 6, index: 2, value: IfcValue::String("Wall-02".into()) },
            IfcMutation::InsertEntityArg { id: 6, index: 1, value: IfcValue::Derived },
            IfcMutation::RemoveEntityArg { id: 6, index: 0 },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = IfcMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = IfcMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion Tests
