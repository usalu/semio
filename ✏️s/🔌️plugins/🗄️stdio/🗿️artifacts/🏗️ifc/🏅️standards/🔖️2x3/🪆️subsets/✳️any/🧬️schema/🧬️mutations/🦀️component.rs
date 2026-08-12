//! 🧬️ Ifc2x3Mutation — document mutation dispatch. Richer than `4`'s `{NoMutation, SetSnapshot}`
//! stub: real per-instance vocabulary (`UpsertInstance`/`RemoveInstance`/`SetHeader`) matching
//! `Ifc2x3Diff`'s own id-keyed shape.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::{
    dec_part21_header, dec_part21_header_bin, dec_part21_instance, dec_part21_instance_bin, dec_str, diff_remove_instance,
    diff_set_header, diff_set_snapshot, diff_upsert_instance, enc_part21_header, enc_part21_header_bin, enc_part21_instance,
    enc_part21_instance_bin, enc_str, read_str_bin, split_top_level, strip_brackets, write_str_bin, Ifc2x3Diff,
};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.ifc.2x3`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Ifc2x3Mutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: Ifc2x3Snapshot },
    UpsertInstance { instance: Part21Instance },
    RemoveInstance { id: u64 },
    SetHeader { header: Part21Header },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff (computed against the PRE-mutation
/// state, per `Mutation::diff`'s contract).
pub fn apply_ifc2x3_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3Mutation) -> Ifc2x3Diff {
    let __diff = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
    match mutation {
        Ifc2x3Mutation::NoMutation => {}
        Ifc2x3Mutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        Ifc2x3Mutation::UpsertInstance { instance } => {
            snapshot.document.instances.retain(|i| i.id != instance.id);
            snapshot.document.instances.push(instance.clone());
        }
        Ifc2x3Mutation::RemoveInstance { id } => {
            snapshot.document.instances.retain(|i| i.id != *id);
        }
        Ifc2x3Mutation::SetHeader { header } => {
            snapshot.document.header = header.clone();
        }
    }
    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<Ifc2x3Snapshot> for Ifc2x3Mutation {
    type Diff = Ifc2x3Diff;

    fn diff(&self, base: &Ifc2x3Snapshot) -> Self::Diff {
        match self {
            Ifc2x3Mutation::NoMutation => Ifc2x3Diff::default(),
            Ifc2x3Mutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            Ifc2x3Mutation::UpsertInstance { instance } => diff_upsert_instance(instance),
            Ifc2x3Mutation::RemoveInstance { id } => diff_remove_instance(*id),
            Ifc2x3Mutation::SetHeader { header } => diff_set_header(header),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Vec<Self> {
        match self {
            Ifc2x3Mutation::NoMutation => vec![Ifc2x3Mutation::NoMutation],
            Ifc2x3Mutation::SetSnapshot { .. } => vec![Ifc2x3Mutation::SetSnapshot { snapshot: base.clone() }],
            Ifc2x3Mutation::UpsertInstance { instance } => match base.document.instance(instance.id) {
                Some(old) => vec![Ifc2x3Mutation::UpsertInstance { instance: old.clone() }],
                None => vec![Ifc2x3Mutation::RemoveInstance { id: instance.id }],
            },
            Ifc2x3Mutation::RemoveInstance { id } => match base.document.instance(*id) {
                Some(old) => vec![Ifc2x3Mutation::UpsertInstance { instance: old.clone() }],
                None => vec![Ifc2x3Mutation::NoMutation],
            },
            Ifc2x3Mutation::SetHeader { .. } => vec![Ifc2x3Mutation::SetHeader { header: base.document.header.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: **hand-rolled**
/// `OpText`/`OpBinary` for `Ifc2x3Mutation`, replacing the prior `serde_json::to_string`/`from_str`/
/// `to_vec`/`from_slice` literal-JSON-transfer shortcut — the LAST standard-specific
/// `POLICY_STDIO_JSON_TRANSFER_BAN` violation named anywhere in this program's own census (see
/// `📖️grammar-recipe.md`'s own citation of this exact file/line). `#[derive(dsl::DslOps)]` cannot
/// be used here either: `Part21Value` (reachable via `Part21Instance`/`Part21Header`/
/// `Ifc2x3Snapshot`) is a genuine data-carrying enum with no `DslField` impl, the identical root
/// cause `4`'s own `IfcMutation` doc comment documents for the isomorphic shape. Reuses the diff
/// sibling's `pub(crate)` grammar primitives (`enc_str`/`enc_part21_header`/`enc_part21_instance`/
/// `split_top_level`/...) rather than duplicating them a second time in this file — same
/// intra-artifact-reuse split `4`'s own `🧬️mutations/🦀️component.rs` uses. Grammar: `keyword
/// arg=value ...` (space-separated), one match arm per variant.
fn enc_ifc2x3_snapshot(s: &Ifc2x3Snapshot) -> String {
    let instances = s.document.instances.iter().map(enc_part21_instance).collect::<Vec<_>>().join(",");
    format!("[{},{},[{}]]", enc_str(&s.schema), enc_part21_header(&s.document.header), instances)
}
fn dec_ifc2x3_snapshot(s: &str) -> Result<Ifc2x3Snapshot, String> {
    // 🩹 `split_top_level` respects `[`/`]` depth, so a single depth-0 split of the whole inner
    // string already yields exactly 3 fields (`header`'s and `instances`'s own internal commas
    // sit at depth ≥1, matching `4`'s own `dec_ifc_snapshot` shape exactly).
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, header, instances] = parts.as_slice() else { return Err(format!("ifc2x3 snapshot: expected 3 fields, got {}", parts.len())) };
    let instances = split_top_level(strip_brackets(instances)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_part21_instance).collect::<Result<Vec<_>, String>>()?;
    Ok(Ifc2x3Snapshot { schema: dec_str(schema)?, document: Part21Document { header: dec_part21_header(header)?, instances } })
}

fn print_ifc2x3_mutation(m: &Ifc2x3Mutation) -> String {
    match m {
        Ifc2x3Mutation::NoMutation => "no-mutation".to_string(),
        Ifc2x3Mutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_ifc2x3_snapshot(snapshot)),
        Ifc2x3Mutation::UpsertInstance { instance } => format!("upsert-instance instance={}", enc_part21_instance(instance)),
        Ifc2x3Mutation::RemoveInstance { id } => format!("remove-instance id={id}"),
        Ifc2x3Mutation::SetHeader { header } => format!("set-header header={}", enc_part21_header(header)),
    }
}
fn parse_ifc2x3_mutation(line: &str) -> Result<Ifc2x3Mutation, String> {
    if line == "no-mutation" {
        return Ok(Ifc2x3Mutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let (arg_key, arg_val) = rest.split_once('=').ok_or_else(|| format!("ifc2x3 mutation: missing arg for {keyword:?}"))?;
    match (keyword, arg_key) {
        ("set-snapshot", "snapshot") => Ok(Ifc2x3Mutation::SetSnapshot { snapshot: dec_ifc2x3_snapshot(arg_val)? }),
        ("upsert-instance", "instance") => Ok(Ifc2x3Mutation::UpsertInstance { instance: dec_part21_instance(arg_val)? }),
        ("remove-instance", "id") => Ok(Ifc2x3Mutation::RemoveInstance { id: arg_val.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        ("set-header", "header") => Ok(Ifc2x3Mutation::SetHeader { header: dec_part21_header(arg_val)? }),
        (other, _) => Err(format!("ifc2x3 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for Ifc2x3Mutation {
    fn print_op(&self) -> String {
        print_ifc2x3_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_ifc2x3_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ Mutation-specific real binary primitives backing the upgraded `OpBinary` impl below — reuses
/// the diff sibling's `pub(crate)` recursive `enc_part21_instance_bin`/`enc_part21_header_bin`/
/// `write_str_bin` primitives for the SHARED `Part21Instance`/`Part21Header`/`Part21Value` shape
/// (same intra-artifact-reuse split the TEXT codec above already uses); only `Ifc2x3Snapshot`'s own
/// binary shape is genuinely new here.
fn enc_ifc2x3_snapshot_bin(s: &Ifc2x3Snapshot, out: &mut Vec<u8>) {
    write_str_bin(out, &s.schema);
    enc_part21_header_bin(&s.document.header, out);
    store::pack_rt::write_varint_u64(out, s.document.instances.len() as u64);
    for inst in &s.document.instances {
        enc_part21_instance_bin(inst, out);
    }
}
fn dec_ifc2x3_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<Ifc2x3Snapshot, String> {
    let schema = read_str_bin(reader)?;
    let header = dec_part21_header_bin(reader)?;
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut instances = Vec::with_capacity(count as usize);
    for _ in 0..count {
        instances.push(dec_part21_instance_bin(reader)?);
    }
    Ok(Ifc2x3Snapshot { schema, document: Part21Document { header, instances } })
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from the literal-JSON shortcut above. `tag` is the `Ifc2x3Mutation` variant ordinal,
/// same 0-4 order `parse_ifc2x3_mutation`'s own keyword match uses. Every field is real
/// (`id` varints, `Part21Instance`/`Part21Header` field-by-field via the reused diff-sibling
/// primitives) — the only place the recursion bottoms out through a fully spec-expressible
/// per-variant tag (`enc_part21_value_bin`), never an opaque byte-chain fallback.
impl protocol::OpBinary for Ifc2x3Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            Ifc2x3Mutation::NoMutation => 0,
            Ifc2x3Mutation::SetSnapshot { .. } => 1,
            Ifc2x3Mutation::UpsertInstance { .. } => 2,
            Ifc2x3Mutation::RemoveInstance { .. } => 3,
            Ifc2x3Mutation::SetHeader { .. } => 4,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Ifc2x3Mutation::NoMutation => {}
            Ifc2x3Mutation::SetSnapshot { snapshot } => enc_ifc2x3_snapshot_bin(snapshot, &mut out),
            Ifc2x3Mutation::UpsertInstance { instance } => enc_part21_instance_bin(instance, &mut out),
            Ifc2x3Mutation::RemoveInstance { id } => store::pack_rt::write_varint_u64(&mut out, *id),
            Ifc2x3Mutation::SetHeader { header } => enc_part21_header_bin(header, &mut out),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(Ifc2x3Mutation::NoMutation),
            1 => {
                let snapshot = dec_ifc2x3_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(Ifc2x3Mutation::SetSnapshot { snapshot })
            }
            2 => {
                let instance = dec_part21_instance_bin(&mut reader).map_err(|e| malformed("op instance", reader.position(), e))?;
                Ok(Ifc2x3Mutation::UpsertInstance { instance })
            }
            3 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                Ok(Ifc2x3Mutation::RemoveInstance { id })
            }
            4 => {
                let header = dec_part21_header_bin(&mut reader).map_err(|e| malformed("op header", reader.position(), e))?;
                Ok(Ifc2x3Mutation::SetHeader { header })
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ One representative `Ifc2x3Mutation` per variant, real `print_op()`-conformance-law fodder
/// (`ops_grammar_conformance_law`) and `protocol_walk_law` fodder — every `Part21Value` tag (incl.
/// the recursive `List`/`Typed` cases) and `UpsertInstance`'s bare `Part21Instance` payload (incl. a
/// real COMPLEX 2-entity instance) are exercised at least once.
pub(crate) fn demo_mutation_cases() -> Vec<Ifc2x3Mutation> {
    vec![
        Ifc2x3Mutation::NoMutation,
        Ifc2x3Mutation::SetSnapshot { snapshot: crate::artifacts::ifc::standards::v2x3::engine::demo_ifc2x3_snapshot() },
        Ifc2x3Mutation::UpsertInstance {
            instance: Part21Instance {
                id: 99,
                entities: vec![
                    ("IFCQUANTITYAREA".into(), vec![
                        Part21Value::Unset,
                        Part21Value::Derived,
                        Part21Value::Int(-7),
                        Part21Value::Real(3.25),
                        Part21Value::Str("hi".into()),
                        Part21Value::Enum("EDGE".into()),
                        Part21Value::Ref(42),
                        Part21Value::List(vec![Part21Value::Int(1), Part21Value::Int(2)]),
                        Part21Value::Typed("IFCLENGTHMEASURE".into(), vec![Part21Value::Real(3000.0)]),
                    ]),
                    ("IFCPHYSICALSIMPLEQUANTITY".into(), vec![Part21Value::Unset]),
                ],
            },
        },
        Ifc2x3Mutation::RemoveInstance { id: 2 },
        Ifc2x3Mutation::SetHeader { header: Part21Header { file_description: vec![], file_name: vec![], file_schema: vec![] } },
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn inst(id: u64, name: &str) -> Part21Instance {
        Part21Instance { id, entities: vec![(name.to_string(), vec![Part21Value::Int(id as i64)])] }
    }

    #[test]
    fn upsert_then_inverse_restores_absent_id_via_remove() {
        let mut snap = Ifc2x3Snapshot::default();
        let mutation = Ifc2x3Mutation::UpsertInstance { instance: inst(1, "IFCWALL") };
        let base = snap.clone();
        apply_ifc2x3_mutation(&mut snap, &mutation);
        assert_eq!(snap.document.instances.len(), 1);
        let inv = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::inverse(&mutation, &base);
        assert_eq!(inv, vec![Ifc2x3Mutation::RemoveInstance { id: 1 }]);
    }

    #[test]
    fn remove_then_inverse_restores_prior_instance() {
        let mut snap = Ifc2x3Snapshot::default();
        snap.document.instances.push(inst(2, "IFCDOOR"));
        let base = snap.clone();
        let mutation = Ifc2x3Mutation::RemoveInstance { id: 2 };
        apply_ifc2x3_mutation(&mut snap, &mutation);
        assert!(snap.document.instances.is_empty());
        let inv = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::inverse(&mutation, &base);
        assert_eq!(inv, vec![Ifc2x3Mutation::UpsertInstance { instance: inst(2, "IFCDOOR") }]);
    }

    #[test]
    fn op_text_round_trips() {
        let mutation = Ifc2x3Mutation::SetHeader { header: Part21Header::default() };
        let printed = protocol::OpText::print_op(&mutation);
        let parsed = <Ifc2x3Mutation as protocol::OpText>::parse_op(&printed).expect("parse");
        assert_eq!(parsed, mutation);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ `OpText`/`OpBinary` round-trip laws for the hand-rolled `Ifc2x3Mutation` grammar —
    /// exercises every variant incl. `SetSnapshot`'s whole-snapshot payload, `UpsertInstance`'s
    /// real COMPLEX (2-entity) instance, and every `Part21Value` tag (`Unset`/`Derived`/`Int`/
    /// `Real`/`Str`/`Enum`/`Ref`/`List`/`Typed`). Replaces the prior `serde_json` stub's implicit
    /// coverage — this is the real proof the JSON-transfer elimination didn't just move the bug.
    #[test]
    fn op_text_binary_roundtrip_law() {
        use protocol::{OpBinary, OpText};
        let mutations = demo_mutation_cases();
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = Ifc2x3Mutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e:?}"));
            let decoded = Ifc2x3Mutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e:?}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion 🧪️Tests
