//! 📩️ Runs the PDF mutation Pack documents through the original receiving port.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
use dsl::OpBinary;
pub use semio_s_artifact_stdio_pdf_testkit::standards;
use standards::v1_7::subsets::base::{
    io::mutation_bridge::apply_pdf_mutation,
    schema::{mutations::{set_image, PdfMutation}, snapshot::{ObjRef, PdfDictEntry, PdfImage, PdfImageMask, PdfIndirectObject, PdfObject, PdfSnapshot}},
};

#[path = "../🦀️.rs"]
mod aggregate_operation_contracts;

fn receiving_stock() -> dsl::original_receiving::ReceivingStock {
    use semio_framework_value::RetainedCloneGrant;
    let quantum = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes: 32768, maximum_capacity_bytes: 16777216,
        maximum_release_bytes: 1048576, maximum_depth: 4096,
    };
    let stock = RetainedCloneGrant {
        maximum_items: 16777216, maximum_copy_bytes: 134217728, maximum_capacity_bytes: 268435456,
        maximum_release_bytes: 536870912, ..quantum
    };
    dsl::original_receiving::ReceivingStock { stock, quantum, fuel: 1 << 20 }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("pack-donation.json")).unwrap()
}

fn from_json(json: &str) -> PdfMutation {
    let parsed = semio_framework_pack_json::parse_bytes(json.as_bytes(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    <PdfMutation as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap()
}

fn archive_image(fixture: &serde_json::Value) -> PdfMutation {
    let id = fixture["id"].as_str().unwrap();
    let width = fixture["width"].as_u64().unwrap() as u32;
    let height = fixture["height"].as_u64().unwrap() as u32;
    let components = fixture["components"].as_u64().unwrap() as u32;
    let stride = fixture["stride"].as_u64().unwrap() as u32;
    let samples = (0..width * height * components).map(|i| (i * stride) as u8).collect();
    let mut image = if components == 3 { PdfImage::rgb8(id, width, height, samples) }
        else { PdfImage::gray8(id, width, height, samples) };
    image.decode = vec![0.5, 1.0];
    image.interpolate = true;
    image.soft_mask = Some("ArchiveAlpha".into());
    image.soft_mask_in_data = Some(1);
    image.mask = Some(PdfImageMask::ColorKey { ranges: vec![1, 2, 3, 4, 5, 6] });
    image.matte = Some(vec![0.1, 0.2, 0.3]);
    image.intent = Some("RelativeColorimetric".into());
    image.optional_content = Some("FixtureGroup".into());
    image.struct_parent = Some(17);
    image.extra = vec![PdfDictEntry { key: "Name".into(), value: PdfObject::Int(7) }];
    PdfMutation::SetImage(set_image::SetImage { image, index: Some(3) })
}

fn enclosed(bytes: &[u8]) -> (Vec<u8>, (usize, usize)) {
    let source = [0xff, 0, 0x89].into_iter().chain(bytes.iter().copied()).chain([0xff]).collect();
    (source, (3, 3 + bytes.len()))
}

fn donate(mutation: &PdfMutation) -> PdfMutation {
    use dsl::original_receiving::receive_to_completion;
    let bytes = mutation.encode_op().unwrap();
    let (source, range) = enclosed(&bytes);
    let actual = receive_to_completion::<PdfMutation>(&source, range, receiving_stock()).unwrap();
    assert_eq!(actual.encode_op().unwrap(), bytes);
    assert_eq!(pack::crc32c(&actual.encode_op().unwrap()), pack::crc32c(&source[range.0..range.1]));
    assert_eq!(actual, PdfMutation::decode_op(&bytes).unwrap());
    actual
}

#[test]
fn original_archived_set_image_donates_into_concrete_mutation() {
    let fixture = fixture();
    for lane in ["image", "alpha"] {
        let expected = archive_image(&fixture[lane]);
        let bytes = expected.encode_op().unwrap();
        assert_eq!(donate(&expected), expected);
        assert_eq!(semio_framework_hash::sha256_hex(&bytes), semio_framework_hash::sha256_hex(&expected.encode_op().unwrap()));
    }
}

#[test]
fn original_archived_pack_documents_refuse_damage() {
    use dsl::original_receiving::receive_to_completion;
    let bytes = archive_image(&fixture()["image"]).encode_op().unwrap();
    let mut damaged = bytes.clone();
    damaged[0] ^= 0xff;
    let truncated = bytes[..bytes.len() - 1].to_vec();
    for malformed in [damaged, truncated, Vec::new()] {
        assert!(PdfMutation::decode_op(&malformed).is_err());
        assert!(receive_to_completion::<PdfMutation>(&malformed, (0, malformed.len()), receiving_stock()).is_err());
    }
}

#[test]
fn original_archive_object_pair_keeps_the_typed_role_inverse() {
    use standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue};
    let fixture = fixture();
    let forward = donate(&from_json(fixture["objects"]["forward"].as_str().unwrap()));
    let inverse = donate(&from_json(fixture["objects"]["inverse"].as_str().unwrap()));
    let (PdfMutation::InsertObject(insert), PdfMutation::RemoveObject(remove)) = (&forward, &inverse) else { panic!("typed InsertObject and RemoveObject expected"); };
    assert_eq!(insert.id, ObjRef { num: 5, r#gen: 0 });
    assert_eq!(insert.value, PdfObject::Null);
    assert_eq!(insert.index, None);
    assert_eq!(remove.id, insert.id);
    let prior = vec![PdfAdmittedStreamRole {
        identity: PdfGraphIdentity { owner: ObjRef { num: 2, r#gen: 0 }, path: Vec::new() },
        dependencies: Vec::new(),
        value: PdfStreamRoleValue::MetadataText { text: "Neutral restored role.".into() },
    }];
    let removed = insert.admitted_stream_roles.as_ref().unwrap();
    let restored = remove.admitted_stream_roles.as_ref().unwrap();
    let intervening = removed.to_indexed().apply(&prior);
    assert!(intervening.is_empty());
    assert_eq!(restored.to_indexed(), removed.to_indexed().inverse(&prior));
    assert_eq!(restored.to_indexed().apply(&intervening), prior);
}

#[test]
fn original_archive_objects_restore_actual_snapshot_and_role_lane() {
    use standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue};
    let fixture = fixture();
    let forward = donate(&from_json(fixture["objects"]["forward"].as_str().unwrap()));
    let inverse = donate(&from_json(fixture["objects"]["inverse"].as_str().unwrap()));
    let id = ObjRef { num: 2, r#gen: 0 };
    let mut base = PdfSnapshot::default();
    base.objects.push(PdfIndirectObject { id, value: PdfObject::Null });
    base.admitted_stream_roles.push(PdfAdmittedStreamRole {
        identity: PdfGraphIdentity { owner: id, path: Vec::new() },
        dependencies: Vec::new(),
        value: PdfStreamRoleValue::MetadataText { text: "Neutral restored role.".into() },
    });
    assert_eq!(semio_s_artifact_stdio_contract::mutation_inverse(&forward, &base).unwrap(), vec![inverse.clone()]);
    let mut state = base.clone();
    apply_pdf_mutation(&mut state, &forward);
    assert!(state.objects.iter().any(|object| object.id.num == 5 && object.value == PdfObject::Null));
    assert!(state.admitted_stream_roles.is_empty());
    apply_pdf_mutation(&mut state, &inverse);
    assert_eq!(state.objects, base.objects);
    assert_eq!(state.admitted_stream_roles, base.admitted_stream_roles);
}

#[test]
fn original_archive_ordinary_leaves_apply_and_restore_real_pdf_fixtures() {
    use standards::v1_7::subsets::base::io::import::decode_pdf;
    let fixtures: [(&[u8], &[u8]); 3] = [
        (include_bytes!("../../../../../🧪️tests/🧫️fixtures/🔄️set-page-rotation/⬅️before.pdf"), include_bytes!("../../../../../🧪️tests/🧫️fixtures/🔄️set-page-rotation/➡️after.pdf")),
        (include_bytes!("../../../../../🧪️tests/🧫️fixtures/ℹ️set-info/⬅️before.pdf"), include_bytes!("../../../../../🧪️tests/🧫️fixtures/ℹ️set-info/➡️after.pdf")),
        (include_bytes!("../../../../../🧪️tests/🧫️fixtures/🗣️set-language/⬅️before.pdf"), include_bytes!("../../../../../🧪️tests/🧫️fixtures/🗣️set-language/➡️after.pdf")),
    ];
    let target = |state: &PdfSnapshot| (state.pages[0].rotate, state.info.clone(), state.language.clone());
    let fixture = fixture();
    for ((before_bytes, after_bytes), pair) in fixtures.into_iter().zip(fixture["ordinaryLeaves"].as_array().unwrap()) {
        let before = decode_pdf(before_bytes).unwrap();
        let after = decode_pdf(after_bytes).unwrap();
        let forward = donate(&from_json(pair["forward"].as_str().unwrap()));
        let inverse = donate(&from_json(pair["inverse"].as_str().unwrap()));
        assert_eq!(semio_s_artifact_stdio_contract::mutation_inverse(&forward, &before).unwrap(), vec![inverse.clone()]);
        let mut state = before.clone();
        apply_pdf_mutation(&mut state, &forward);
        assert_eq!(target(&state), target(&after));
        apply_pdf_mutation(&mut state, &inverse);
        assert_eq!(target(&state), target(&before));
    }
}
