//! 🧬️ IfcMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal `{NoMutation, SetSnapshot}` stub — HEADER scalar setters plus
//! `InsertEntity`/`RemoveEntity`/`SetEntityName`/`SetEntityArg`/`InsertEntityArg`/`RemoveEntityArg`
//! for the id-keyed `entities` collection and its per-entity positional `args`. Every variant's
//! `diff()` is handcrafted (constructs `IfcDiff` directly via the `schema::diff` builders) —
//! apply-and-capture is never used.

use crate::schema::diff::{
    self, dec_entity, dec_entity_bin, dec_entity_list_bin, dec_ifc_value, dec_ifc_value_bin, dec_ifc_value_list, dec_ifc_value_list_bin, dec_str, enc_entity, enc_entity_bin, enc_entity_list_bin, enc_ifc_value, enc_ifc_value_bin, enc_ifc_value_list,
    enc_ifc_value_list_bin, enc_str, read_str_bin, split_top_level, strip_brackets, write_str_bin, IfcDiff,
};
use crate::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};
use crate::IfcSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};

//#region 🔖️Mutations
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
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcSnapshot: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️.rs:23:19   (SetSnapshot { snapshot: IfcSnapshot })
/// error[E0277]: the trait bound `v4::subsets::any::schema::snapshot::component::IfcEntity: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️.rs:40:17   (InsertEntity { entity: IfcEntity })
/// ```
/// Same root cause as `IfcDiff` (§3a): `IfcValue` carries fields on 7 of its 9 variants, has no
/// `DslField` impl, and every variant here either carries it directly (`values`/`value`) or
/// transitively via `IfcSnapshot`/`IfcEntity` (`SetSnapshot`/`InsertEntity`). `OpText`/`OpBinary`
/// stay hand-rolled below for the same reason, reusing `IfcDiff`'s `pub(crate)` grammar primitives
/// (`enc_str`/`enc_ifc_value`/`enc_entity`/`split_top_level`/...); `DESCRIPTORS`/`descriptor()` are
/// no longer hand-written, though — `#[derive(dsl::Mutations)]` synthesizes both from the per-leaf
/// `🔣️.json` descriptors beside this file, which does not need `DslField`.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = IfcSnapshot, diff = IfcDiff, schema = "IfcMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum IfcMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
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
pub const KINDS: &[&str] = &["set-snapshot", "set-file-description", "set-file-name", "set-file-schema", "insert-entity", "remove-entity", "set-entity-name", "set-entity-arg", "insert-entity-arg", "remove-entity-arg"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning a typed error outcome without changing the
/// snapshot when an entity or argument target is missing or out of range.
pub fn apply_ifc_mutation(snapshot: &mut IfcSnapshot, mutation: &IfcMutation) -> protocol::MutationOutcome<IfcDiff> {
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

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `IfcMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above) — reuses `IfcDiff`'s `pub(crate)` grammar primitives
/// (`enc_str`/`enc_ifc_value`/`enc_entity`/`split_top_level`/`encode_option`/...) rather than
/// duplicating them a second time in this file. Grammar: `keyword arg=value ...` (space-separated,
/// same shape the derive's own handcrafted-wrapper convention uses), one match arm per variant (no
/// `DslVariants` scaffolding available since nothing here derives it).
fn enc_ifc_header(h: &IfcHeader) -> String {
    format!("[{},{},{}]", enc_ifc_value_list(&h.file_description), enc_ifc_value_list(&h.file_name), enc_ifc_value_list(&h.file_schema))
}
fn dec_ifc_header(s: &str) -> Result<IfcHeader, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [fd, fname, fs] = parts.as_slice() else { return Err(format!("ifc header: expected 3 fields, got {}", parts.len())) };
    Ok(IfcHeader { file_description: dec_ifc_value_list(fd)?, file_name: dec_ifc_value_list(fname)?, file_schema: dec_ifc_value_list(fs)? })
}
fn enc_ifc_snapshot(s: &IfcSnapshot) -> String {
    let ifc_entity_separator = ",";
    let entities = s.entities.iter().map(enc_entity).collect::<Vec<_>>().join(ifc_entity_separator);
    format!("[{},{},[{}]]", enc_str(&s.schema), enc_ifc_header(&s.header), entities)
}
fn dec_ifc_snapshot(s: &str) -> Result<IfcSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, header, entities] = parts.as_slice() else { return Err(format!("ifc snapshot: expected 3 fields, got {}", parts.len())) };
    let entities = split_top_level(strip_brackets(entities)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_entity).collect::<Result<Vec<_>, String>>()?;
    Ok(IfcSnapshot { schema: dec_str(schema)?, header: dec_ifc_header(header)?, entities })
}

fn print_ifc_mutation(m: &IfcMutation) -> String {
    match m {
        IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_ifc_snapshot(snapshot)),
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values }) => format!("set-file-description values={}", enc_ifc_value_list(values)),
        IfcMutation::SetFileName(set_file_name::SetFileName { values }) => format!("set-file-name values={}", enc_ifc_value_list(values)),
        IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values }) => format!("set-file-schema values={}", enc_ifc_value_list(values)),
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => format!("insert-entity index={index} entity={}", enc_entity(entity)),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => format!("remove-entity id={id}"),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id, name }) => format!("set-entity-name id={id} name={}", enc_str(name)),
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, index, value }) => format!("set-entity-arg id={id} index={index} value={}", enc_ifc_value(value)),
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, index, value }) => format!("insert-entity-arg id={id} index={index} value={}", enc_ifc_value(value)),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, index }) => format!("remove-entity-arg id={id} index={index}"),
    }
}
fn parse_ifc_mutation(line: &str) -> Result<IfcMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("ifc mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("ifc mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    let u64_arg = |k: &str| -> Result<u64, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_ifc_snapshot(arg("snapshot")?)? })),
        "set-file-description" => Ok(IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: dec_ifc_value_list(arg("values")?)? })),
        "set-file-name" => Ok(IfcMutation::SetFileName(set_file_name::SetFileName { values: dec_ifc_value_list(arg("values")?)? })),
        "set-file-schema" => Ok(IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: dec_ifc_value_list(arg("values")?)? })),
        "insert-entity" => Ok(IfcMutation::InsertEntity(insert_entity::InsertEntity { index: usize_arg("index")?, entity: dec_entity(arg("entity")?)? })),
        "remove-entity" => Ok(IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: u64_arg("id")? })),
        "set-entity-name" => Ok(IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: u64_arg("id")?, name: dec_str(arg("name")?)? })),
        "set-entity-arg" => Ok(IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: u64_arg("id")?, index: usize_arg("index")?, value: dec_ifc_value(arg("value")?)? })),
        "insert-entity-arg" => Ok(IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: u64_arg("id")?, index: usize_arg("index")?, value: dec_ifc_value(arg("value")?)? })),
        "remove-entity-arg" => Ok(IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: u64_arg("id")?, index: usize_arg("index")? })),
        other => Err(format!("ifc mutation: unknown keyword {other:?}")),
    }
}

impl OpText for IfcMutation {
    fn print_op(&self) -> String {
        print_ifc_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_ifc_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ P2-FG1: mutation-specific real binary primitives backing the upgraded `OpBinary` impl below
/// — reuses `IfcDiff`'s `pub(crate)` recursive `enc_entity_bin`/`enc_ifc_value_list_bin`/
/// `write_str_bin` primitives (`../../🔺️diff/🦀️.rs`, imported above) for the SHARED
/// `IfcEntity`/`IfcValue` shape (same intra-artifact-reuse split the TEXT codec above already
/// uses), only `IfcHeader`/`IfcSnapshot`'s own binary shape is genuinely new here.
fn enc_ifc_header_bin(h: &IfcHeader, out: &mut Vec<u8>) {
    enc_ifc_value_list_bin(&h.file_description, out);
    enc_ifc_value_list_bin(&h.file_name, out);
    enc_ifc_value_list_bin(&h.file_schema, out);
}
fn dec_ifc_header_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcHeader, String> {
    let file_description = dec_ifc_value_list_bin(reader)?;
    let file_name = dec_ifc_value_list_bin(reader)?;
    let file_schema = dec_ifc_value_list_bin(reader)?;
    Ok(IfcHeader { file_description, file_name, file_schema })
}
fn enc_ifc_snapshot_bin(s: &IfcSnapshot, out: &mut Vec<u8>) {
    write_str_bin(out, &s.schema);
    enc_ifc_header_bin(&s.header, out);
    enc_entity_list_bin(&s.entities, out);
}
fn dec_ifc_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<IfcSnapshot, String> {
    let schema = read_str_bin(reader)?;
    let header = dec_ifc_header_bin(reader)?;
    let entities = dec_entity_list_bin(reader)?;
    Ok(IfcSnapshot { schema, header, entities })
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut (`IfcMutation` was one of 4
/// of stdio's 7 FG1 standards still on that shortcut per this wave's own P2-FG1 census). `tag` is
/// the `IfcMutation` variant ordinal, same 1-10 order `parse_ifc_mutation`'s own keyword match
/// uses (tag 0, formerly `NoMutation`, is retired rather than reassigned). Every field is real
/// (`id`/`index` varints, `IfcEntity`/`IfcValue` field-by-field via the reused diff-sibling
/// primitives) — the only place the recursion bottoms out through a fully spec-expressible
/// per-variant tag (`enc_ifc_value_bin`), never an opaque byte-chain fallback.
impl OpBinary for IfcMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            IfcMutation::SetSnapshot(..) => 1,
            IfcMutation::SetFileDescription(..) => 2,
            IfcMutation::SetFileName(..) => 3,
            IfcMutation::SetFileSchema(..) => 4,
            IfcMutation::InsertEntity(..) => 5,
            IfcMutation::RemoveEntity(..) => 6,
            IfcMutation::SetEntityName(..) => 7,
            IfcMutation::SetEntityArg(..) => 8,
            IfcMutation::InsertEntityArg(..) => 9,
            IfcMutation::RemoveEntityArg(..) => 10,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_ifc_snapshot_bin(snapshot, &mut out),
            IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values }) => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::SetFileName(set_file_name::SetFileName { values }) => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values }) => enc_ifc_value_list_bin(values, &mut out),
            IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_entity_bin(entity, &mut out);
            }
            IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => store::pack_rt::write_varint_u64(&mut out, *id),
            IfcMutation::SetEntityName(set_entity_name::SetEntityName { id, name }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                write_str_bin(&mut out, name);
            }
            IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_ifc_value_bin(value, &mut out);
            }
            IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_ifc_value_bin(value, &mut out);
            }
            IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, index }) => {
                store::pack_rt::write_varint_u64(&mut out, *id);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            1 => {
                let snapshot = dec_ifc_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            2 => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values }))
            }
            3 => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileName(set_file_name::SetFileName { values }))
            }
            4 => {
                let values = dec_ifc_value_list_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values }))
            }
            5 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let entity = dec_entity_bin(&mut reader).map_err(|e| malformed("op entity", reader.position(), e))?;
                Ok(IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity }))
            }
            6 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                Ok(IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id }))
            }
            7 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let name = read_str_bin(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(IfcMutation::SetEntityName(set_entity_name::SetEntityName { id, name }))
            }
            8 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_ifc_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, index, value }))
            }
            9 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = dec_ifc_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, index, value }))
            }
            10 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, index }))
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
pub(crate) fn demo_mutation_cases() -> Vec<IfcMutation> {
    let demo_entity = |id: u64, name: &str, args: Vec<IfcValue>| IfcEntity { id, name: name.into(), args, complex: Vec::new() };
    vec![
        IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: crate::engine::demo_ifc_snapshot() }),
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

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &IfcMutation, base: &IfcSnapshot) -> protocol::MutationOutcome<IfcDiff> {
    protocol::MutationOutcome::new(match this {
        IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values }) => diff::diff_set_file_description(values.clone()),
        IfcMutation::SetFileName(set_file_name::SetFileName { values }) => diff::diff_set_file_name(values.clone()),
        IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values }) => diff::diff_set_file_schema(values.clone()),
        IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity }) => diff::diff_insert_entity(*index, entity.clone()),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => diff::diff_remove_entity(*id),
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id, name }) => diff::diff_set_entity_name(*id, name),
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, index, value }) => diff::diff_set_entity_arg(*id, *index, value.clone()),
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, index, value }) => diff::diff_insert_entity_arg(*id, *index, value.clone()),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, index }) => diff::diff_remove_entity_arg(*id, *index),
    })
}

/// ↩️ Handcrafted, key-aware mutation-level inverses — lifted verbatim from the former
/// `impl Mutation`. Entity/arg-targeted variants look the prior value up in `base`; a stale/absent
/// id/index inverts to NO step at all (`return Vec::new()`) rather than a `NoMutation` sentinel,
/// since that variant no longer exists — `apply_ifc_mutation`-ing zero steps and applying a former
/// `NoMutation` step were always observationally identical.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &IfcMutation, base: &IfcSnapshot) -> Vec<IfcMutation> {
    let entity = |id: u64| base.entities.iter().find(|e| e.id == id);
    vec![match this {
        IfcMutation::SetSnapshot(_) => IfcMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        IfcMutation::SetFileDescription(_) => IfcMutation::SetFileDescription(set_file_description::SetFileDescription { values: base.header.file_description.clone() }),
        IfcMutation::SetFileName(_) => IfcMutation::SetFileName(set_file_name::SetFileName { values: base.header.file_name.clone() }),
        IfcMutation::SetFileSchema(_) => IfcMutation::SetFileSchema(set_file_schema::SetFileSchema { values: base.header.file_schema.clone() }),
        IfcMutation::InsertEntity(insert_entity::InsertEntity { entity, .. }) => IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id: entity.id }),
        IfcMutation::RemoveEntity(remove_entity::RemoveEntity { id }) => match base.entities.iter().position(|e| e.id == *id) {
            Some(index) => IfcMutation::InsertEntity(insert_entity::InsertEntity { index, entity: base.entities[index].clone() }),
            None => return Vec::new(),
        },
        IfcMutation::SetEntityName(set_entity_name::SetEntityName { id, .. }) => match entity(*id) {
            Some(e) => IfcMutation::SetEntityName(set_entity_name::SetEntityName { id: *id, name: e.name.clone() }),
            None => return Vec::new(),
        },
        IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id, index, .. }) => match entity(*id).and_then(|e| e.args.get(*index)) {
            Some(v) => IfcMutation::SetEntityArg(set_entity_arg::SetEntityArg { id: *id, index: *index, value: v.clone() }),
            None => return Vec::new(),
        },
        IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id, index, .. }) => IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id: *id, index: *index }),
        IfcMutation::RemoveEntityArg(remove_entity_arg::RemoveEntityArg { id, index }) => match entity(*id).and_then(|e| e.args.get(*index)) {
            Some(v) => IfcMutation::InsertEntityArg(insert_entity_arg::InsertEntityArg { id: *id, index: *index, value: v.clone() }),
            None => return Vec::new(),
        },
    }]
}
//#endregion 🔖️MutationTrait

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
