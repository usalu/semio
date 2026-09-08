
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
    BYTES.get_or_init(|| std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../temp/wellness-center-sama.ifc")).expect("read temp/wellness-center-sama.ifc"))
}

// 🚫️async: E1 pure fixture reader (OnceLock initializer, consumed inside a sync closure) — see R9
fn exact_fixture() -> Ifc2x3Snapshot {
    static SNAPSHOT: OnceLock<Ifc2x3Snapshot> = OnceLock::new();
    SNAPSHOT.get_or_init(|| crate::standards::v2x3::engine::decode_ifc2x3(exact_fixture_bytes()).expect("import IFC2X3 fixture")).clone()
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
    assert_eq!(inv, vec![Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(base) })]);
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
    assert_eq!(inv, vec![Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(base) })]);
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
    let direct = crate::standards::v2x3::engine::encode_ifc2x3(&imported).expect("direct export");
    assert_exact("direct export", &direct).await;
    assert_eq!(crate::standards::v2x3::engine::encode_ifc2x3(&imported).expect("repeat export"), direct);

    let packed = store::ArtifactPack::encode_pack(&imported);
    let unpacked = <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(&packed).expect("pack decode");
    assert!(unpacked == imported, "pack must retain the complete logical IFC model");
    assert_exact("pack export", &crate::standards::v2x3::engine::encode_ifc2x3(&unpacked).expect("pack export")).await;

    let printed = store::ArtifactDsl::print_dsl(&imported);
    let parsed = <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("DSL parse");
    assert!(parsed == imported, "DSL must retain the complete logical IFC model");
    assert_exact("DSL export", &crate::standards::v2x3::engine::encode_ifc2x3(&parsed).expect("DSL export")).await;
}

#[semio_framework_async_macros::async_test]
async fn exact_native_between_noop_inverse_absorb_and_supported_rewrite() {
    let imported = exact_fixture();
    let self_diff = <Ifc2x3Diff as DiffAlgebra<Ifc2x3Snapshot>>::between(&imported, &imported);
    assert!(self_diff.is_empty());
    assert_exact("self diff export", &crate::standards::v2x3::engine::encode_ifc2x3(&MutationDiff::apply(&self_diff, &imported).expect("valid self diff")).expect("self diff export")).await;

    let mut changed_header = imported.document.header.clone();
    changed_header.file_name = vec![Part21Value::Str("semio-roundtrip-changed.ifc".into())];
    let mutation = Ifc2x3Mutation::SetHeader(set_header::SetHeader { header: changed_header });
    let d1 = Mutation::diff(&mutation, &imported);
    let changed = MutationDiff::apply(d1.diff(), &imported).expect("valid forward diff");
    let changed_bytes = crate::standards::v2x3::engine::encode_ifc2x3(&changed).expect("supported dirty export");
    assert!(changed_bytes != exact_fixture_bytes(), "effective IFC mutation must change deterministic output");
    let reparsed = crate::standards::v2x3::engine::decode_ifc2x3(&changed_bytes).expect("re-import supported dirty export");
    assert_eq!(reparsed.document.header, changed.document.header);

    let inverse_mutation = Mutation::inverse(&mutation, &imported).into_iter().next().expect("inverse mutation");
    let d2 = Mutation::diff(&inverse_mutation, &changed);
    let restored = MutationDiff::apply(d2.diff(), &changed).expect("valid inverse diff");
    assert!(restored == imported, "inverse mutation must restore imported snapshot and provenance");
    assert_exact("inverse export", &crate::standards::v2x3::engine::encode_ifc2x3(&restored).expect("inverse export")).await;

    let mut absorbed = d1.diff().clone();
    MutationDiff::absorb(&mut absorbed, d2.diff().clone());
    let absorbed_result = MutationDiff::apply(&absorbed, &imported).expect("valid absorbed diff");
    assert!(absorbed_result == imported, "absorbed mutation pair must restore imported snapshot");
    assert_exact("absorbed export", &crate::standards::v2x3::engine::encode_ifc2x3(&absorbed_result).expect("absorbed export")).await;
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
        assert_exact("text diff export", &crate::standards::v2x3::engine::encode_ifc2x3(&applied).expect("text diff export")).await;
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
        assert_exact("binary diff export", &crate::standards::v2x3::engine::encode_ifc2x3(&applied).expect("binary diff export")).await;
    }
    {
        let mutation = Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(imported.clone()) });
        let wire = mutation.print_op();
        drop(mutation);
        let decoded = Ifc2x3Mutation::parse_op(&wire).expect("op text decode");
        drop(wire);
        assert!(matches!(&decoded, Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) if snapshot.as_ref() == &imported), "set-snapshot text codec must retain the logical IFC model");
        let diff = Mutation::diff(&decoded, &projection);
        drop(decoded);
        let applied = MutationDiff::apply(diff.diff(), &projection).expect("valid text mutation diff");
        drop(diff);
        assert!(applied == imported, "set-snapshot text mutation must restore imported snapshot");
        assert_exact("set-snapshot text export", &crate::standards::v2x3::engine::encode_ifc2x3(&applied).expect("set-snapshot text export")).await;
    }
    {
        let mutation = Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::new(imported.clone()) });
        let wire = mutation.encode_op().expect("op binary encode");
        drop(mutation);
        let decoded = Ifc2x3Mutation::decode_op(&wire).expect("op binary decode");
        drop(wire);
        assert!(matches!(&decoded, Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) if snapshot.as_ref() == &imported), "set-snapshot binary codec must retain the logical IFC model");
        let diff = Mutation::diff(&decoded, &projection);
        drop(decoded);
        let applied = MutationDiff::apply(diff.diff(), &projection).expect("valid binary mutation diff");
        drop(diff);
        assert!(applied == imported, "set-snapshot binary mutation must restore imported snapshot");
        assert_exact("set-snapshot binary export", &crate::standards::v2x3::engine::encode_ifc2x3(&applied).expect("set-snapshot binary export")).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn exact_native_materializes_logical_edits_and_restores_interior_order() {
    let imported = exact_fixture();
    let mut edited = imported.clone();
    edited.document.instances[1].entities[0].0 = "IFCCHANGEDENTITY".into();
    let edited_bytes = crate::standards::v2x3::engine::encode_ifc2x3(&edited).expect("logical edit export");
    assert_ne!(edited_bytes, exact_fixture_bytes());
    assert_eq!(crate::standards::v2x3::engine::decode_ifc2x3(&edited_bytes).expect("logical edit import"), edited);

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
    assert_exact("interior upsert inverse", &crate::standards::v2x3::engine::encode_ifc2x3(&restored).expect("inverse export")).await;

    let mut op = Ifc2x3Mutation::RemoveInstance(remove_instance::RemoveInstance { id: 0 }).encode_op().expect("encode op");
    op.push(0);
    assert!(Ifc2x3Mutation::decode_op(&op).is_err(), "trailing op bytes accepted");
}
//#endregion 🔖️LosslessLogicalModel

//#region 🔖️KindsGate
/// 🧪️ Wave gate: `KINDS` must match the enum's own variants, in declaration order, and its
/// spellings must match `print_op`'s own keyword for each -- the mutation catalog
/// (`../../🔣️oracle.json`) and the feature file are checked against never drift
/// apart from the enum itself.
#[semio_framework_async_macros::async_test]
async fn kinds_const_matches_enum_variants_in_declaration_order() {
    let one_per_variant = vec![
        Ifc2x3Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Box::default() }),
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
