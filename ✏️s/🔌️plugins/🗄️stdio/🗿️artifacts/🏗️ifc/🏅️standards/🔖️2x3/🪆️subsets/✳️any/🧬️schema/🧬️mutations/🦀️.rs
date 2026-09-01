//! 🧬️ Ifc2x3Mutation — document mutation dispatch. Richer than `4`'s `SetSnapshot`-only stub: real
//! per-instance vocabulary (`UpsertInstance`/`RemoveInstance`/`SetHeader`) matching `Ifc2x3Diff`'s
//! own id-keyed shape.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::{
    dec_edm_preamble_bin, dec_instance_list, dec_optional_edm_preamble, dec_part21_header, dec_part21_header_bin, dec_part21_instance, dec_part21_instance_bin, dec_str, enc_edm_preamble_bin, enc_instance_list_into, enc_optional_edm_preamble,
    enc_part21_header, enc_part21_header_bin, enc_part21_instance, enc_part21_instance_bin, enc_str, read_str_bin, split_top_level, strip_brackets, write_str_bin, Ifc2x3Diff,
};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
#[cfg(test)]
use crate::artifacts::step::engine::part21::Part21Value;
use crate::artifacts::step::engine::part21::{Part21Document, Part21Header, Part21Instance};
use protocol::os_spr::command::DiffAlgebra;
use protocol::Mutation;

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.ifc.2x3`.
//#region 🔖️Leaves
#[path = "📄set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🧱upsert-instance/🦀️.rs"]
pub mod upsert_instance;
#[path = "🗑remove-instance/🦀️.rs"]
pub mod remove_instance;
#[path = "📋set-header/🦀️.rs"]
pub mod set_header;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Ifc2x3Snapshot, diff = Ifc2x3Diff, schema = "Ifc2x3Mutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum Ifc2x3Mutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    UpsertInstance(upsert_instance::UpsertInstance),
    RemoveInstance(remove_instance::RemoveInstance),
    SetHeader(set_header::SetHeader),
}

/// 📇️ Kebab-case spelling of every `Ifc2x3Mutation` variant, in declaration order -- the
/// exhaustive mutation catalog `../../🧪️oracle/🔣️.json`'s `kinds` array is required to
/// match verbatim (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps
/// that honest; the framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &["set-snapshot", "upsert-instance", "remove-instance", "set-header"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning the diff (computed against the PRE-mutation
/// state, per `Mutation::diff`'s contract).
pub fn apply_ifc2x3_mutation(snapshot: &mut Ifc2x3Snapshot, mutation: &Ifc2x3Mutation) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let outcome = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::diff(mutation, snapshot);
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
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &Ifc2x3Mutation, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
    let mut next = base.clone();
    match this {
        Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => {
            crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::validate_ifc2x3_snapshot(snapshot).expect("IFC2X3 SetSnapshot must carry a valid logical model");
            return protocol::MutationOutcome::new(Ifc2x3Diff::between(base, snapshot));
        }
        Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance }) => match next.document.instances.iter_mut().find(|candidate| candidate.id == instance.id) {
            Some(existing) => *existing = instance.clone(),
            None => next.document.instances.push(instance.clone()),
        },
        Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id }) => next.document.instances.retain(|instance| instance.id != *id),
        Ifc2x3Mutation::SetHeader(set_header::SetHeader { header }) => next.document.header = header.clone(),
    }
    protocol::MutationOutcome::new(Ifc2x3Diff::between(base, &next))
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &Ifc2x3Mutation, base: &Ifc2x3Snapshot) -> Vec<Ifc2x3Mutation> {
    let _ = this;
    vec![Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })]
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
fn enc_ifc2x3_snapshot_into(s: &Ifc2x3Snapshot, out: &mut String) {
    out.push('[');
    out.push_str(&enc_str(&s.schema));
    out.push(',');
    out.push_str(&enc_part21_header(&s.document.header));
    out.push(',');
    enc_instance_list_into(&s.document.instances, out);
    out.push(',');
    out.push_str(&enc_optional_edm_preamble(&s.edm_preamble));
    out.push(']');
}
fn dec_ifc2x3_snapshot(s: &str) -> Result<Ifc2x3Snapshot, String> {
    let fields = split_top_level(strip_brackets(s)?, ',');
    let [schema, header, instances, edm_preamble] = fields.as_slice() else {
        return Err(format!("ifc2x3 snapshot: expected 4 fields, got {}", fields.len()));
    };
    Ok(Ifc2x3Snapshot { schema: dec_str(schema)?, document: Part21Document { header: dec_part21_header(header)?, instances: dec_instance_list(instances)? }, edm_preamble: dec_optional_edm_preamble(edm_preamble)? })
}

fn print_ifc2x3_mutation(m: &Ifc2x3Mutation) -> String {
    match m {
        Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => {
            let mut out = String::with_capacity(snapshot.document.instances.len().saturating_mul(64).saturating_add(22));
            out.push_str("set-snapshot snapshot=");
            enc_ifc2x3_snapshot_into(snapshot, &mut out);
            out
        }
        Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance }) => format!("upsert-instance instance={}", enc_part21_instance(instance)),
        Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id }) => format!("remove-instance id={id}"),
        Ifc2x3Mutation::SetHeader(set_header::SetHeader { header }) => format!("set-header header={}", enc_part21_header(header)),
    }
}
fn parse_ifc2x3_mutation(line: &str) -> Result<Ifc2x3Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let (arg_key, arg_val) = rest.split_once('=').ok_or_else(|| format!("ifc2x3 mutation: missing arg for {keyword:?}"))?;
    match (keyword, arg_key) {
        ("set-snapshot", "snapshot") => Ok(Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_ifc2x3_snapshot(arg_val)? })),
        ("upsert-instance", "instance") => Ok(Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: dec_part21_instance(arg_val)? })),
        ("remove-instance", "id") => Ok(Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: arg_val.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        ("set-header", "header") => Ok(Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: dec_part21_header(arg_val)? })),
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
    match &s.edm_preamble {
        None => out.push(0),
        Some(value) => {
            out.push(1);
            enc_edm_preamble_bin(value, out);
        }
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
    let edm_preamble = match reader.read_u8().map_err(|e| e.to_string())? {
        0 => None,
        1 => Some(dec_edm_preamble_bin(reader)?),
        tag => return Err(format!("ifc2x3 snapshot: invalid EDM preamble presence {tag}")),
    };
    Ok(Ifc2x3Snapshot { schema, document: Part21Document { header, instances }, edm_preamble })
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
            Ifc2x3Mutation::SetSnapshot(..) => 1,
            Ifc2x3Mutation::UpsertInstance(..) => 2,
            Ifc2x3Mutation::RemoveInstance(..) => 3,
            Ifc2x3Mutation::SetHeader(..) => 4,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_ifc2x3_snapshot_bin(snapshot, &mut out),
            Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance }) => enc_part21_instance_bin(instance, &mut out),
            Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id }) => store::pack_rt::write_varint_u64(&mut out, *id),
            Ifc2x3Mutation::SetHeader(set_header::SetHeader { header }) => enc_part21_header_bin(header, &mut out),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported format {format}")));
        }
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        let mutation = match tag {
            1 => {
                let snapshot = dec_ifc2x3_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot })
            }
            2 => {
                let instance = dec_part21_instance_bin(&mut reader).map_err(|e| malformed("op instance", reader.position(), e))?;
                Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance })
            }
            3 => {
                let id = reader.read_varint_u64().map_err(|e| malformed("op id", reader.position(), e.to_string()))?;
                Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id })
            }
            4 => {
                let header = dec_part21_header_bin(&mut reader).map_err(|e| malformed("op header", reader.position(), e))?;
                Ifc2x3Mutation::SetHeader(set_header::SetHeader { header })
            }
            other => return Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        };
        if reader.remaining() != 0 {
            return Err(malformed("op trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
        }
        Ok(mutation)
    }
}
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
        Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: crate::artifacts::ifc::standards::v2x3::engine::demo_ifc2x3_snapshot() }),
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
        }),
        Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 2 }),
        Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: Part21Header { file_description: vec![], file_name: vec![], file_schema: vec![] } }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::os_spr::command::DiffAlgebra;
    use protocol::{DiffCodec, MutationDiff, OpBinary, OpText};
    use std::sync::OnceLock;
    async fn inst(id: u64, name: &str) -> Part21Instance {
        Part21Instance { id, entities: vec![(name.to_string(), vec![Part21Value::Int(id as i64)])] }
    }
    // 🚫️async: E1 pure fixture reader (OnceLock initializer, consumed inside a sync closure) — see R9
    fn exact_fixture_bytes() -> &'static [u8] {
        static BYTES: OnceLock<Vec<u8>> = OnceLock::new();
        BYTES.get_or_init(|| std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../temp/wellness-center-sama.ifc")).expect("read temp/wellness-center-sama.ifc"))
    }

    // 🚫️async: E1 pure fixture reader (OnceLock initializer, consumed inside a sync closure) — see R9
    fn exact_fixture() -> Ifc2x3Snapshot {
        static SNAPSHOT: OnceLock<Ifc2x3Snapshot> = OnceLock::new();
        SNAPSHOT.get_or_init(|| crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(exact_fixture_bytes()).expect("import IFC2X3 fixture")).clone()
    }

    async fn assert_exact(label: &str, actual: &[u8]) {
        let expected = exact_fixture_bytes();
        let first_difference = actual.iter().zip(expected).position(|(left, right)| left != right);
        assert!(actual == expected, "{label}: expected {} bytes, got {}; first differing byte: {first_difference:?}", expected.len(), actual.len(),);
    }

    #[semio_framework_async_macros::async_test]
    async fn upsert_then_inverse_restores_absent_id_via_remove() {
        let mut snap = Ifc2x3Snapshot::default();
        let mutation = Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: inst(1, "IFCWALL").await });
        let base = snap.clone();
        apply_ifc2x3_mutation(&mut snap, &mutation);
        assert_eq!(snap.document.instances.len(), 1);
        let inv = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::inverse(&mutation, &base);
        assert_eq!(inv, vec![Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base })]);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_then_inverse_restores_prior_instance() {
        let mut snap = Ifc2x3Snapshot::default();
        snap.document.instances.push(inst(2, "IFCDOOR").await);
        let base = snap.clone();
        let mutation = Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 2 });
        apply_ifc2x3_mutation(&mut snap, &mutation);
        assert!(snap.document.instances.is_empty());
        let inv = <Ifc2x3Mutation as Mutation<Ifc2x3Snapshot>>::inverse(&mutation, &base);
        assert_eq!(inv, vec![Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base })]);
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trips() {
        let mutation = Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: Part21Header::default() });
        let printed = OpText::print_op(&mutation);
        let parsed = <Ifc2x3Mutation as OpText>::parse_op(&printed).expect("parse");
        assert_eq!(parsed, mutation);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ `OpText`/`OpBinary` round-trip laws for the hand-rolled `Ifc2x3Mutation` grammar —
    /// exercises every variant incl. `SetSnapshot`'s whole-snapshot payload, `UpsertInstance`'s
    /// real COMPLEX (2-entity) instance, and every `Part21Value` tag (`Unset`/`Derived`/`Int`/
    /// `Real`/`Str`/`Enum`/`Ref`/`List`/`Typed`). Replaces the prior `serde_json` stub's implicit
    /// coverage — this is the real proof the JSON-transfer elimination didn't just move the bug.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
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

    //#region 🔖️LosslessLogicalModel
    #[semio_framework_async_macros::async_test]
    async fn exact_native_direct_pack_and_dsl_roundtrips() {
        let imported = exact_fixture();
        let direct = crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&imported).expect("direct export");
        assert_exact("direct export", &direct).await;
        assert_eq!(crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&imported).expect("repeat export"), direct);

        let packed = store::ArtifactPack::encode_pack(&imported);
        let unpacked = <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(&packed).expect("pack decode");
        assert!(unpacked == imported, "pack must retain the complete logical IFC model");
        assert_exact("pack export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&unpacked).expect("pack export")).await;

        let printed = store::ArtifactDsl::print_dsl(&imported);
        let parsed = <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("DSL parse");
        assert!(parsed == imported, "DSL must retain the complete logical IFC model");
        assert_exact("DSL export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&parsed).expect("DSL export")).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_native_between_noop_inverse_absorb_and_supported_rewrite() {
        let imported = exact_fixture();
        let self_diff = <Ifc2x3Diff as DiffAlgebra<Ifc2x3Snapshot>>::between(&imported, &imported);
        assert!(self_diff.is_empty());
        assert_exact("self diff export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&MutationDiff::apply(&self_diff, &imported).expect("valid self diff")).expect("self diff export")).await;

        let mut changed_header = imported.document.header.clone();
        changed_header.file_name = vec![Part21Value::Str("semio-roundtrip-changed.ifc".into())];
        let mutation = Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: changed_header });
        let d1 = Mutation::diff(&mutation, &imported);
        let changed = MutationDiff::apply(d1.diff(), &imported).expect("valid forward diff");
        let changed_bytes = crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&changed).expect("supported dirty export");
        assert!(changed_bytes != exact_fixture_bytes(), "effective IFC mutation must change deterministic output");
        let reparsed = crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(&changed_bytes).expect("re-import supported dirty export");
        assert_eq!(reparsed.document.header, changed.document.header);

        let inverse_mutation = Mutation::inverse(&mutation, &imported).into_iter().next().expect("inverse mutation");
        let d2 = Mutation::diff(&inverse_mutation, &changed);
        let restored = MutationDiff::apply(d2.diff(), &changed).expect("valid inverse diff");
        assert!(restored == imported, "inverse mutation must restore imported snapshot and provenance");
        assert_exact("inverse export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&restored).expect("inverse export")).await;

        let mut absorbed = d1.diff().clone();
        MutationDiff::absorb(&mut absorbed, d2.diff().clone());
        let absorbed_result = MutationDiff::apply(&absorbed, &imported).expect("valid absorbed diff");
        assert!(absorbed_result == imported, "absorbed mutation pair must restore imported snapshot");
        assert_exact("absorbed export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&absorbed_result).expect("absorbed export")).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_native_set_snapshot_codecs_retain_complete_logical_model() {
        let imported = exact_fixture();
        let projection = Ifc2x3Snapshot::default();
        {
            let diff = Ifc2x3Diff::between(&projection, &imported);
            let wire = diff.print_diff();
            let decoded = Ifc2x3Diff::parse_diff(&wire).expect("diff text decode");
            drop(wire);
            assert_eq!(decoded, diff);
            drop(diff);
            let applied = MutationDiff::apply(&decoded, &projection).expect("valid text diff");
            drop(decoded);
            assert!(applied == imported, "text diff must restore imported snapshot");
            assert_exact("text diff export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&applied).expect("text diff export")).await;
        }
        {
            let diff = Ifc2x3Diff::between(&projection, &imported);
            let wire = diff.encode_diff().expect("diff binary encode");
            let decoded = Ifc2x3Diff::decode_diff(&wire).expect("diff binary decode");
            drop(wire);
            assert_eq!(decoded, diff);
            drop(diff);
            let applied = MutationDiff::apply(&decoded, &projection).expect("valid binary diff");
            drop(decoded);
            assert!(applied == imported, "binary diff must restore imported snapshot");
            assert_exact("binary diff export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&applied).expect("binary diff export")).await;
        }
        {
            let mutation = Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: imported.clone() });
            let wire = mutation.print_op();
            drop(mutation);
            let decoded = Ifc2x3Mutation::parse_op(&wire).expect("op text decode");
            drop(wire);
            assert!(matches!(&decoded, Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) if snapshot == &imported), "set-snapshot text codec must retain the logical IFC model");
            let diff = Mutation::diff(&decoded, &projection);
            drop(decoded);
            let applied = MutationDiff::apply(diff.diff(), &projection).expect("valid text mutation diff");
            drop(diff);
            assert!(applied == imported, "set-snapshot text mutation must restore imported snapshot");
            assert_exact("set-snapshot text export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&applied).expect("set-snapshot text export")).await;
        }
        {
            let mutation = Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: imported.clone() });
            let wire = mutation.encode_op().expect("op binary encode");
            drop(mutation);
            let decoded = Ifc2x3Mutation::decode_op(&wire).expect("op binary decode");
            drop(wire);
            assert!(matches!(&decoded, Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) if snapshot == &imported), "set-snapshot binary codec must retain the logical IFC model");
            let diff = Mutation::diff(&decoded, &projection);
            drop(decoded);
            let applied = MutationDiff::apply(diff.diff(), &projection).expect("valid binary mutation diff");
            drop(diff);
            assert!(applied == imported, "set-snapshot binary mutation must restore imported snapshot");
            assert_exact("set-snapshot binary export", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&applied).expect("set-snapshot binary export")).await;
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_native_materializes_logical_edits_and_restores_interior_order() {
        let imported = exact_fixture();
        let mut edited = imported.clone();
        edited.document.instances[1].entities[0].0 = "IFCCHANGEDENTITY".into();
        let edited_bytes = crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&edited).expect("logical edit export");
        assert_ne!(edited_bytes, exact_fixture_bytes());
        assert_eq!(crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(&edited_bytes).expect("logical edit import"), edited);

        let target = imported.document.instances[1].clone();
        let mut replacement = target.clone();
        replacement.entities[0].0 = "IFCCHANGEDENTITY".into();
        let mutation = Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: replacement });
        let changed_outcome = Mutation::diff(&mutation, &imported);
        let changed = MutationDiff::apply(changed_outcome.diff(), &imported).expect("valid upsert diff");
        assert_eq!(changed.document.instances[1].id, target.id, "upsert moved an interior entity");
        let inverse = Mutation::inverse(&mutation, &imported).into_iter().next().expect("inverse");
        let restored_outcome = Mutation::diff(&inverse, &changed);
        let restored = MutationDiff::apply(restored_outcome.diff(), &changed).expect("valid inverse diff");
        assert_eq!(restored, imported);
        assert_exact("interior upsert inverse", &crate::artifacts::ifc::standards::v2x3::engine::encode_ifc2x3(&restored).expect("inverse export")).await;

        let mut op = Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 0 }).encode_op().expect("encode op");
        op.push(0);
        assert!(Ifc2x3Mutation::decode_op(&op).is_err(), "trailing op bytes accepted");
    }
    //#endregion 🔖️LosslessLogicalModel

    //#region 🔖️KindsGate
    /// 🧪️ Wave gate: `KINDS` must match the enum's own variants, in declaration order, and its
    /// spellings must match `print_op`'s own keyword for each -- the mutation catalog
    /// (`../../🧪️oracle/🔣️.json`) and the feature file are checked against never drift
    /// apart from the enum itself.
    #[semio_framework_async_macros::async_test]
    async fn kinds_const_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Ifc2x3Snapshot::default() }),
            Ifc2x3Mutation::UpsertInstance(upsert_instance::UpsertInstance { instance: inst(1, "IFCWALL").await }),
            Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 1 }),
            Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: Part21Header::default() }),
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            let printed = mutation.print_op();
            let keyword = printed.split(' ').next().unwrap_or(&printed);
            assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
        }
    }
    //#endregion 🔖️KindsGate
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/renames-the-ifcproject-instance/🦀️component.rs"]
    mod tests_set_snapshot_renames_the_ifcproject_instance;
}
//#endregion 🧪️FixtureTests
