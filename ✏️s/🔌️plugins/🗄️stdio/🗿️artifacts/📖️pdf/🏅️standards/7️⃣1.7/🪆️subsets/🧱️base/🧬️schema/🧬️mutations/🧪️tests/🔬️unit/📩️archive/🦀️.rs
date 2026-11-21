//! 📩️ Runs the canonical PDF mutation archive laws through their registered codec.

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
use dsl::OpBinary;
pub use semio_s_artifact_stdio_pdf_testkit::standards;
use standards::v1_7::subsets::base::{
    io::mutation_bridge::apply_pdf_mutation,
    schema::{mutations::PdfMutation, snapshot::{ObjRef, PdfIndirectObject, PdfObject, PdfSnapshot}},
};

#[path = "../🦀️.rs"]
mod aggregate_operation_contracts;

fn image_archive_receiving_stock() -> dsl::original_receiving::ReceivingStock {
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

#[test]
fn original_archived_set_image_donates_into_concrete_mutation() {
    use dsl::original_receiving::receive_to_completion;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    for lane in ["image", "alpha"] {
        let expected = archive_image(&fixture[lane]);
        let frame = expected.encode_op().unwrap();
        let range = (3, 3 + frame.len());
        let source: Vec<_> = [0xff, 0, 0x89].into_iter().chain(frame.iter().copied()).chain([0xff]).collect();
        let actual = receive_to_completion::<PdfMutation>(&source, range, image_archive_receiving_stock()).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual, PdfMutation::decode_op(&frame).unwrap());
        let encoded = actual.encode_op().unwrap();
        assert_eq!(encoded, frame);
        assert_eq!(pack::crc32c(&encoded), pack::crc32c(&source[range.0..range.1]));
        assert_eq!(semio_framework_hash::sha256_hex(&encoded), semio_framework_hash::sha256_hex(&frame));
    }
}

#[test]
fn original_archived_set_image_refuses_malformed_samples_and_metadata() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let frame = archive_image(&fixture["image"]).encode_op().unwrap();
    let mut reader = dsl::ByteReader::new(&frame);
    let format = reader.read_u8().unwrap();
    let tag = reader.read_u8().unwrap();
    let length = reader.read_varint_u64().unwrap() as usize;
    let body = std::str::from_utf8(reader.read_bytes(length).unwrap()).unwrap();
    for row in fixture["malformed"].as_array().unwrap() {
        let source = body.replacen(row[0].as_str().unwrap(), row[1].as_str().unwrap(), 1);
        assert_ne!(source, body, "{row}");
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(format);
        writer.write_u8(tag);
        writer.write_varint_u64(source.len() as u64);
        writer.write_bytes(source.as_bytes());
        let malformed = writer.into_bytes();
        assert!(PdfMutation::decode_op(&malformed).is_err(), "{row}");
        let refused = receive_to_completion::<PdfMutation>(&malformed, (0, malformed.len()), image_archive_receiving_stock()).unwrap_err();
        assert_eq!(refused.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{row}");
    }
}

#[test]
fn original_archived_set_image_keeps_ordinary_grammar_for_other_shapes() {
    use dsl::original_receiving::receive_to_completion;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    for body in fixture["ordinaryGrammar"].as_array().unwrap() {
        let body = body.as_str().unwrap();
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(1);
        writer.write_u8(26);
        writer.write_varint_u64(body.len() as u64);
        writer.write_bytes(body.as_bytes());
        let frame = writer.into_bytes();
        let expected = PdfMutation::decode_op(&frame).unwrap();
        let actual = receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(actual.encode_op().unwrap(), expected.encode_op().unwrap());
    }
}

#[test]
fn original_archived_set_image_leaves_occupied_recipient_and_donor_intact() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let expected = archive_image(&fixture["image"]);
    let frame = expected.encode_op().unwrap();
    let limits = image_archive_receiving_stock();
    let grant = limits.quantum;
    let birth_grant = semio_framework_value::RetainedCloneGrant { maximum_capacity_bytes: 0, maximum_release_bytes: 0, ..grant };
    let (mut cursor, admission) = PdfMutation::admit_receiving(&frame, (0, frame.len()), birth_grant).result.unwrap().unwrap();
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
    let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
    wallet.post(admission).unwrap();
    let mut reserved_exact_backing = false;
    let mut total = admission;
    let mut peak_copy = 0;
    while !cursor.is_ready() {
        let body = grant.maximum_copy_bytes;
        let demand = cursor.demands(&frame, body).unwrap();
        peak_copy = peak_copy.max(demand.copy_bytes);
        let units = cursor.step_work_demand(&frame, body).unwrap();
        if demand.capacity_bytes == 49_152 {
            let short = semio_framework_value::RetainedCloneGrant { maximum_capacity_bytes: 49_151, ..grant };
            let denied = wallet.run(short, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
            assert_eq!(denied.performed, 0);
            assert_eq!(denied.actual, None);
            assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
            assert_eq!(cursor.demands(&frame, body).unwrap(), demand);
        }
        let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
        reserved_exact_backing |= turn.actual == Some(49_152);
        let result = turn.output;
        assert!(result.work > 0);
        let progress = result.result.unwrap();
        wallet.post(progress).unwrap();
        total = total.checked_add(progress).unwrap();
    }
    assert!(reserved_exact_backing);
    let occupied = archive_image(&fixture["alpha"]);
    let mut target = Some(occupied.clone());
    let turn = cursor.take_into(&mut target, grant);
    assert_eq!(turn.work, 0);
    assert_eq!(turn.result.unwrap(), RetainedCloneProgress::default());
    assert_eq!(target.take().unwrap(), occupied);
    assert!(cursor.is_ready());
    let turn = cursor.take_into(&mut target, grant);
    assert_eq!(turn.work, 1);
    assert_eq!(target.unwrap(), expected);
    wallet.post(turn.result.unwrap()).unwrap();
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
    println!("[DEBUG] original RGB receive through Ready: {total:?}; peak copy quote {peak_copy}");
}

#[test]
fn original_archived_set_image_cancellation_retires_exact_backing() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, ValueRefusalKind};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let frame = archive_image(&fixture["image"]).encode_op().unwrap();
    let limits = image_archive_receiving_stock();
    let grant = limits.quantum;
    let (mut cursor, admission) = PdfMutation::admit_receiving(&frame, (0, frame.len()), grant).result.unwrap().unwrap();
    let active = std::cell::Cell::new(true);
    let mut checkpoint = |_| active.get();
    let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
    let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
    wallet.post(admission).unwrap();
    let mut total = admission;
    let body = grant.maximum_copy_bytes;
    loop {
        let units = cursor.step_work_demand(&frame, body).unwrap();
        let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
        let progress = turn.output.result.unwrap();
        wallet.post(progress).unwrap();
        total = total.checked_add(progress).unwrap();
        if turn.actual == Some(49_152) { break; }
        assert!(!cursor.is_ready());
    }
    active.set(false);
    let units = cursor.step_work_demand(&frame, body).unwrap();
    let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
    assert_eq!(turn.performed, 0);
    assert_eq!(turn.output.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
    let mut output = None;
    assert_eq!(cursor.take_into(&mut output, grant).work, 0);
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
        let progress = turn.result.unwrap().progress();
        wallet.post(progress).unwrap();
        total = total.checked_add(progress).unwrap();
    }
    assert!(output.is_none());
    assert_eq!(total.retained_capacity_bytes, total.released_bytes);
    println!("[DEBUG] canceled original RGB released {} bytes", total.released_bytes);
}

#[test]
fn original_archived_set_image_metadata_uses_exact_funded_allocations() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneProgress};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let expected = archive_image(&fixture["image"]);
    let frame = expected.encode_op().unwrap();
    let expected_allocations: Vec<_> = fixture["metadata"]["allocations"].as_array().unwrap().iter().map(|row| {
        let element = match row["element"].as_str().unwrap() {
            "f64" => size_of::<f64>(),
            "u32" => size_of::<u32>(),
            "PdfDictEntry" => size_of::<PdfDictEntry>(),
            _ => panic!("unknown metadata corpus element"),
        };
        row["count"].as_u64().unwrap() as usize * element
    }).collect();
    let limits = image_archive_receiving_stock();
    let grant = limits.quantum;
    let body = grant.maximum_copy_bytes;
    let (mut cursor, admitted) = PdfMutation::admit_receiving(&frame, (0, frame.len()), grant).result.unwrap().unwrap();
    let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
    let mut progress = admitted;
    let mut checkpoint = |_| true;
    let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
    let mut seen = Vec::new();
    while !cursor.is_ready() {
        let units = cursor.step_work_demand(&frame, body).unwrap();
        let demand = cursor.demands(&frame, body).unwrap();
        if units > 8 && demand.capacity_bytes > 0 {
            let short = semio_framework_value::RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant };
            let denied = wallet.run(short, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
            assert_eq!(denied.performed, 0);
            assert_eq!(denied.actual, None);
            assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
            assert_eq!(cursor.demands(&frame, body).unwrap(), demand);
        }
        let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
        let receipt = turn.output.result.unwrap();
        assert!(receipt.copied_items > 0 && turn.performed > 0);
        assert_eq!(receipt.retained_capacity_bytes, turn.actual.unwrap_or(0));
        if units > 8 && demand.capacity_bytes > 0 {
            assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
            seen.push(demand.capacity_bytes);
        }
        wallet.post(receipt).unwrap();
        progress = progress.checked_add(receipt).unwrap();
    }
    assert_eq!(seen, expected_allocations);
    println!("[DEBUG] paid metadata allocation bytes {seen:?}; original receiving receipt {progress:?}");
    let mut value = None;
    assert_eq!(cursor.take_into(&mut value, grant).work, 1);
    assert_eq!(value.unwrap(), expected);
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
        wallet.post(turn.result.unwrap().progress()).unwrap();
    }
}

#[test]
fn original_archived_set_image_cancels_after_paid_metadata_placement() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, ValueRefusalKind};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let frame = archive_image(&fixture["image"]).encode_op().unwrap();
    let limits = image_archive_receiving_stock();
    let grant = limits.quantum;
    let body = grant.maximum_copy_bytes;
    let (mut cursor, admitted) = PdfMutation::admit_receiving(&frame, (0, frame.len()), grant).result.unwrap().unwrap();
    let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
    let active = std::cell::Cell::new(true);
    let mut checkpoint = |_| active.get();
    let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
    let mut total = admitted;
    let mut seen = 0;
    let end = fixture["metadata"]["allocations"].as_array().unwrap().len();
    while seen < end {
        let units = cursor.step_work_demand(&frame, body).unwrap();
        let turn = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
        let receipt = turn.output.result.unwrap();
        if units > 8 && receipt.retained_capacity_bytes > 0 { seen += 1; }
        total = total.checked_add(receipt).unwrap();
        wallet.post(receipt).unwrap();
        assert!(!cursor.is_ready());
    }
    active.set(false);
    let units = cursor.step_work_demand(&frame, body).unwrap();
    let refused = wallet.run(grant, units, |effect| cursor.step_in(&frame, &mut native, body, effect)).unwrap().unwrap();
    assert_eq!(refused.performed, 0);
    assert_eq!(refused.output.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
    assert!(!cursor.is_ready());
    while !cursor.terminal_is_empty() {
        let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
        let receipt = turn.result.unwrap().progress();
        total = total.checked_add(receipt).unwrap();
        wallet.post(receipt).unwrap();
    }
    assert_eq!(total.retained_capacity_bytes, total.released_bytes);
    println!("[DEBUG] paid metadata cancellation retired {} bytes", total.released_bytes);
}

#[test]
fn original_archived_set_image_metadata_matches_cold_type_refusals() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("set-image-donation.json")).unwrap();
    let frame = archive_image(&fixture["image"]).encode_op().unwrap();
    let mut reader = dsl::ByteReader::new(&frame);
    let format = reader.read_u8().unwrap();
    let tag = reader.read_u8().unwrap();
    let payload_length = reader.read_varint_u64().unwrap() as usize;
    let body = std::str::from_utf8(reader.read_bytes(payload_length).unwrap()).unwrap();
    for row in fixture["metadata"]["invalid"].as_array().unwrap() {
        let malformed = body.replacen(row[0].as_str().unwrap(), row[1].as_str().unwrap(), 1);
        assert_ne!(malformed, body, "{row}");
        let mut writer = dsl::ByteWriter::new();
        writer.write_u8(format);
        writer.write_u8(tag);
        writer.write_varint_u64(malformed.len() as u64);
        writer.write_bytes(malformed.as_bytes());
        let frame = writer.into_bytes();
        assert!(PdfMutation::decode_op(&frame).is_err(), "{row}");
        let error = receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap_err();
        assert_eq!(error.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{row}");
    }
}

fn object_archive_cases() -> serde_json::Value {
    serde_json::from_str(include_str!("set-image-donation.json")).unwrap()
}

fn object_archive_frame(tag: u8, json: &str) -> Vec<u8> {
    let mut writer = dsl::ByteWriter::new();
    writer.write_u8(1);
    writer.write_u8(tag);
    writer.write_varint_u64(json.len() as u64);
    writer.write_bytes(json.as_bytes());
    writer.into_bytes()
}

fn object_archive_fixtures() -> (Vec<u8>, Vec<u8>) {
    let cases = object_archive_cases();
    let forward = object_archive_frame(cases["archive"]["tag"].as_u64().unwrap() as u8, cases["archive"]["json"].as_str().unwrap());
    let inverse = object_archive_frame(cases["objects"]["inverseTag"].as_u64().unwrap() as u8, cases["objects"]["inverseJson"].as_str().unwrap());
    (forward, inverse)
}

#[test]
fn original_archive_object_pair_preserves_raw_bytes_and_typed_role_inverse() {
    use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue};
    use dsl::original_receiving::receive_to_completion;
    let (forward, inverse) = object_archive_fixtures();
    let frozen = forward.clone();
    assert_eq!(forward.len(), 141);
    assert_eq!(&forward[..8], &[1, 6, 137, 1, b'{', b'"', b'm', b'u']);
    assert_eq!(pack::crc32c(&forward[..20]), 0x94c0df0d);
    assert_eq!(u32::from_le_bytes(forward[20..24].try_into().unwrap()), 0x4f747265);
    let mut output = Vec::new();
    for frame in [&forward, &inverse] {
        let oracle = PdfMutation::decode_op(frame).unwrap();
        let original: Vec<_> = [0xff, 0].into_iter().chain(frame.iter().copied()).chain([0xff]).collect();
        let value = receive_to_completion::<PdfMutation>(&original, (2, original.len() - 1), image_archive_receiving_stock()).unwrap();
        assert_eq!(value, oracle);
        let exact = value.encode_op().unwrap();
        assert_eq!(exact, oracle.encode_op().unwrap());
        let mut reader = dsl::ByteReader::new(frame);
        assert_eq!(reader.read_u8().unwrap(), 1);
        let _ = reader.read_u8().unwrap();
        let length = reader.read_varint_u64().unwrap() as usize;
        let json: serde_json::Value = serde_json::from_slice(reader.read_bytes(length).unwrap()).unwrap();
        assert_eq!(json["id"]["num"], 5);
        assert_eq!(reader.remaining(), 0);
        println!("[DEBUG] object frame {} bytes crc32c {:08x} sha256 {}; canonical {} bytes crc32c {:08x} sha256 {}",
            frame.len(), pack::crc32c(frame), semio_framework_hash::sha256_hex(frame),
            exact.len(), pack::crc32c(&exact), semio_framework_hash::sha256_hex(&exact));
        output.push(value);
    }
    assert_eq!(forward, frozen);
    assert_eq!(output[1].encode_op().unwrap(), inverse);
    assert!(output[0].encode_op().unwrap().len() < forward.len());
    let (PdfMutation::InsertObject(insert), PdfMutation::RemoveObject(remove)) = (&output[0], &output[1]) else { panic!("typed InsertObject and RemoveObject expected"); };
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
    let intervening = removed.apply(&prior);
    assert!(intervening.is_empty());
    assert_eq!(restored, &removed.inverse(&prior));
    assert_eq!(restored.apply(&intervening), prior);
}

#[test]
fn original_archive_objects_keep_frame_and_type_refusals() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let cases = object_archive_cases();
    let (forward, _) = object_archive_fixtures();
    for row in cases["archive"]["refusals"].as_array().unwrap() {
        let malformed = if let Some(json) = row["json"].as_str() {
            object_archive_frame(row["tag"].as_u64().unwrap() as u8, json)
        } else {
            let header: Vec<u8> = serde_json::from_value(row["header"].clone()).unwrap();
            header.into_iter().chain(forward[4..].iter().copied()).collect()
        };
        assert_eq!(PdfMutation::decode_op(&malformed).is_ok(), row["coldAccepts"].as_bool().unwrap_or(false), "{row}");
        let expected = if row["kind"] == "work-limit" { ValueRefusalKind::WorkLimit } else { ValueRefusalKind::InvalidValue };
        let error = receive_to_completion::<PdfMutation>(&malformed, (0, malformed.len()), image_archive_receiving_stock()).unwrap_err();
        assert_eq!(error.refusal_kind(), Some(expected), "{row}");
    }
    for (tag, template, rows) in [
        (6, &cases["archive"]["json"], &cases["objects"]["invalidForward"]),
        (7, &cases["objects"]["inverseJson"], &cases["objects"]["invalidInverse"]),
    ] {
        let template = template.as_str().unwrap();
        for row in rows.as_array().unwrap() {
            let body = template.replacen(row[0].as_str().unwrap(), row[1].as_str().unwrap(), 1);
            assert_ne!(body, template);
            let malformed = object_archive_frame(tag, &body);
            assert!(PdfMutation::decode_op(&malformed).is_err(), "{row}");
            let error = receive_to_completion::<PdfMutation>(&malformed, (0, malformed.len()), image_archive_receiving_stock()).unwrap_err();
            assert_eq!(error.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{row}");
        }
    }
}

#[test]
fn original_archive_objects_settle_each_storage_or_cancel() {
    use crate::standards::v1_7::subsets::base::schema::{diff::PdfIndexedItem, stream_roles::PdfAdmittedStreamRole};
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, ValueRefusalKind};
    let (forward, inverse) = object_archive_fixtures();
    for (frame, size) in [(&forward, size_of::<usize>()), (&inverse, size_of::<PdfIndexedItem<PdfAdmittedStreamRole>>())] {
        for cancel in [false, true] {
            let limits = image_archive_receiving_stock();
            let grant = limits.quantum;
            let body = grant.maximum_copy_bytes;
            let (mut cursor, admitted) = PdfMutation::admit_receiving(frame, (0, frame.len()), grant).result.unwrap().unwrap();
            assert!(cursor.demands(&frame.clone(), body).is_err());
            let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
            let mut progress = admitted;
            let active = std::cell::Cell::new(true);
            let mut checkpoint = |_| active.get();
            let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
            let mut allocations = Vec::new();
            while !cursor.is_ready() {
                let work = cursor.step_work_demand(frame, body).unwrap();
                let demand = cursor.demands(frame, body).unwrap();
                if work > 8 && demand.capacity_bytes > 0 {
                    assert_eq!(demand.capacity_bytes, size);
                    let short = RetainedCloneGrant { maximum_capacity_bytes: size - 1, ..grant };
                    let denied = wallet.run(short, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                    assert_eq!(denied.performed, 0);
                    assert_eq!(denied.actual, None);
                    assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
                    assert_eq!(cursor.demands(frame, body).unwrap(), demand);
                }
                let turn = wallet.run(grant, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                let receipt = turn.output.result.unwrap();
                assert_eq!(receipt.retained_capacity_bytes, turn.actual.unwrap_or(0));
                if work > 8 && receipt.retained_capacity_bytes > 0 { allocations.push(receipt.retained_capacity_bytes); }
                wallet.post(receipt).unwrap();
                progress = progress.checked_add(receipt).unwrap();
                if cancel && !allocations.is_empty() { break; }
            }
            assert_eq!(allocations, [size]);
            if cancel {
                active.set(false);
                let work = cursor.step_work_demand(frame, body).unwrap();
                let denied = wallet.run(grant, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                assert_eq!(denied.performed, 0);
                assert_eq!(denied.output.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
                assert_eq!(cursor.take_into(&mut None, grant).work, 0);
            } else {
                let mark = PdfMutation::decode_op(&inverse).unwrap();
                let mut target = Some(mark.clone());
                let denied = cursor.take_into(&mut target, grant);
                assert_eq!(denied.work, 0);
                assert_eq!(denied.result.unwrap(), RetainedCloneProgress::default());
                assert_eq!(target.take(), Some(mark));
                assert_eq!(cursor.take_into(&mut target, grant).work, 1);
                assert_eq!(target.unwrap(), PdfMutation::decode_op(frame).unwrap());
                println!("[DEBUG] archive tag {} carried {size} typed bytes; receiving receipt {progress:?}", frame[1]);
            }
            while !cursor.terminal_is_empty() {
                let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
                let receipt = turn.result.unwrap().progress();
                wallet.post(receipt).unwrap();
                progress = progress.checked_add(receipt).unwrap();
            }
            if cancel {
                assert_eq!(progress.retained_capacity_bytes, progress.released_bytes);
                println!("[DEBUG] canceled archive tag {} with {} balanced bytes", frame[1], progress.released_bytes);
            }
        }
    }
}

fn leaf_archive_frame(tag: u8, row: &serde_json::Value) -> Vec<u8> {
    object_archive_frame(tag, row["json"].as_str().unwrap())
}

#[test]
fn original_archive_ordinary_leaves_preserve_all_six_fixture_frames() {
    use dsl::original_receiving::receive_to_completion;
    let laws = object_archive_cases();
    for pair in laws["ordinaryLeaves"]["pairs"].as_array().unwrap() {
        let tag = pair["tag"].as_u64().unwrap() as u8;
        for side in ["forward", "inverse"] {
            let row = &pair[side];
            let frame = leaf_archive_frame(tag, row);
            let enclosing: Vec<_> = [0xff, 0].into_iter().chain(frame.iter().copied()).chain([0xff]).collect();
            let decoded = receive_to_completion::<PdfMutation>(&enclosing, (2, 2 + frame.len()), image_archive_receiving_stock()).unwrap();
            assert_eq!(decoded, PdfMutation::decode_op(&frame).unwrap());
            assert_eq!(decoded.encode_op().unwrap(), frame);
            assert_eq!(row["frameBytes"].as_u64().unwrap() as usize, frame.len());
            assert_eq!(format!("{:08x}", pack::crc32c(&frame)), row["crc32c"].as_str().unwrap());
            assert_eq!(semio_framework_hash::sha256_hex(&frame), row["sha256"].as_str().unwrap());
        }
    }
}

#[test]
fn original_archive_ordinary_leaves_preserve_exact_typed_validation() {
    use dsl::original_receiving::receive_to_completion;
    use semio_framework_value::ValueRefusalKind;
    let laws = object_archive_cases();
    for row in laws["ordinaryLeaves"]["invalid"].as_array().unwrap() {
        let frame = object_archive_frame(row[0].as_u64().unwrap() as u8, row[1].as_str().unwrap());
        assert!(PdfMutation::decode_op(&frame).is_err(), "{row}");
        let refusal = receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap_err();
        assert_eq!(refusal.refusal_kind(), Some(ValueRefusalKind::InvalidValue), "{row}");
    }
    for row in laws["ordinaryLeaves"]["normalized"].as_array().unwrap() {
        let frame = object_archive_frame(row[0].as_u64().unwrap() as u8, row[1].as_str().unwrap());
        let decoded = receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap();
        assert_eq!(decoded, PdfMutation::decode_op(&frame).unwrap());
    }
}

#[test]
fn original_archive_ordinary_leaves_close_before_publication_and_settle() {
    use dsl::original_receiving::{OpBinaryReceiving, OpBinaryReceivingCursor};
    use semio_framework_value::{AuthoringWallet, NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, ValueRefusalKind};
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry;
    let laws = object_archive_cases();
    let pairs = laws["ordinaryLeaves"]["pairs"].as_array().unwrap();
    let mut frames: Vec<_> = pairs.iter().flat_map(|pair| {
        let tag = pair["tag"].as_u64().unwrap() as u8;
        [leaf_archive_frame(tag, &pair["forward"]), leaf_archive_frame(tag, &pair["inverse"])]
    }).collect();
    let extra = laws["ordinaryLeaves"]["normalized"].as_array().unwrap().last().unwrap();
    frames.push(object_archive_frame(extra[0].as_u64().unwrap() as u8, extra[1].as_str().unwrap()));
    for (index, frame) in frames.iter().enumerate() {
        let mut turns = None;
        for cancel in [false, true] {
            let limits = image_archive_receiving_stock();
            let grant = limits.quantum;
            let body = grant.maximum_copy_bytes;
            let (mut cursor, mut progress) = PdfMutation::admit_receiving(frame, (0, frame.len()), grant).result.unwrap().unwrap();
            let mut wallet = AuthoringWallet::new(limits.stock, u128::from(limits.fuel));
            let active = std::cell::Cell::new(true);
            let mut checkpoint = |_| active.get();
            let mut native = NativeDecodeControl::new(limits.stock.maximum_capacity_bytes, &mut checkpoint);
            let mut count = 0;
            let mut typed = false;
            let mut added = 0;
            let mut last_work = 0;
            while !cursor.is_ready() {
                let demand = cursor.demands(frame, body).unwrap();
                let work = cursor.step_work_demand(frame, body).unwrap();
                if !typed && work == 5 {
                    let short = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant };
                    for (grant, work) in [(short, work), (grant, work - 1)] {
                        let denied = wallet.run(grant, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                        assert_eq!(denied.performed, 0);
                        assert!(denied.actual.is_none());
                        assert_eq!(denied.output.result.unwrap(), RetainedCloneProgress::default());
                        assert_eq!(cursor.demands(frame, body).unwrap(), demand);
                    }
                }
                typed |= work == 5;
                if work == 200 && demand.capacity_bytes > 0 {
                    assert_eq!(demand.capacity_bytes, size_of::<PdfDictEntry>());
                    let short = RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant };
                    let declined = wallet.run(short, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                    assert_eq!(declined.performed, 0);
                    assert!(declined.actual.is_none());
                    assert_eq!(declined.output.result.unwrap(), RetainedCloneProgress::default());
                    assert_eq!(cursor.demands(frame, body).unwrap(), demand);
                }
                if cancel && Some(count + 1) == turns { active.set(false); }
                let turn = wallet.run(grant, work, |effect| cursor.step_in(frame, &mut native, body, effect)).unwrap().unwrap();
                if !active.get() {
                    assert_eq!(turn.performed, 0);
                    assert_eq!(turn.output.result.unwrap_err().refusal_kind(), Some(ValueRefusalKind::Canceled));
                    break;
                }
                let receipt = turn.output.result.unwrap();
                if work == 200 { added += receipt.retained_capacity_bytes; }
                assert_eq!(receipt.retained_capacity_bytes, turn.actual.unwrap_or(0));
                wallet.post(receipt).unwrap();
                progress = progress.checked_add(receipt).unwrap();
                count += 1;
                last_work = work;
            }
            assert!(typed);
            assert_eq!(added, if index + 1 == frames.len() { size_of::<PdfDictEntry>() } else { 0 });
            if cancel {
                assert_eq!(cursor.take_into(&mut None, grant).work, 0);
            } else {
                assert_eq!(last_work, 2);
                turns = Some(count);
                let existing = PdfMutation::decode_op(&frames[0]).unwrap();
                let mut output = Some(existing.clone());
                let denied = cursor.take_into(&mut output, grant);
                assert_eq!(denied.work, 0);
                assert_eq!(denied.result.unwrap(), RetainedCloneProgress::default());
                assert_eq!(output.take(), Some(existing));
                assert_eq!(cursor.take_into(&mut output, grant).work, 1);
                assert_eq!(output.unwrap(), PdfMutation::decode_op(frame).unwrap());
                println!("[DEBUG] ordinary archive frame {index} tag {}: {count} turns, {added} typed allocation bytes, receipt {progress:?}", frame[1]);
            }
            while !cursor.terminal_is_empty() {
                let (turn, _) = wallet.run_settled(grant, 4096, |effect| cursor.close_step(effect)).unwrap().unwrap();
                let receipt = turn.result.unwrap().progress();
                wallet.post(receipt).unwrap();
                progress = progress.checked_add(receipt).unwrap();
            }
            if cancel {
                assert_eq!(progress.retained_capacity_bytes, progress.released_bytes);
                println!("[DEBUG] canceled ordinary frame {index} with {} balanced bytes", progress.released_bytes);
            }
        }
    }
}

// Registered snapshot replay test.
#[test]
fn original_archive_objects_restore_actual_snapshot_and_role_lane() {
    use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue};
    use dsl::original_receiving::receive_to_completion;
    let (forward, inverse) = object_archive_fixtures();
    let forward = receive_to_completion::<PdfMutation>(&forward, (0, forward.len()), image_archive_receiving_stock()).unwrap();
    let inverse = receive_to_completion::<PdfMutation>(&inverse, (0, inverse.len()), image_archive_receiving_stock()).unwrap();
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
    use crate::standards::v1_7::subsets::base::io::import::decode_pdf;
    use dsl::original_receiving::receive_to_completion;
    let fixtures: [(&[u8], &[u8]); 3] = [
        (include_bytes!("../../../../../🧪️tests/🧫️fixtures/🔄️set-page-rotation/⬅️before.pdf"), include_bytes!("../../../../../🧪️tests/🧫️fixtures/🔄️set-page-rotation/➡️after.pdf")),
        (include_bytes!("../../../../../🧪️tests/🧫️fixtures/ℹ️set-info/⬅️before.pdf"), include_bytes!("../../../../../🧪️tests/🧫️fixtures/ℹ️set-info/➡️after.pdf")),
        (include_bytes!("../../../../../🧪️tests/🧫️fixtures/🗣️set-language/⬅️before.pdf"), include_bytes!("../../../../../🧪️tests/🧫️fixtures/🗣️set-language/➡️after.pdf")),
    ];
    let target = |state: &PdfSnapshot| (state.pages[0].rotate, state.info.clone(), state.language.clone());
    let laws = object_archive_cases();
    for ((before_bytes, after_bytes), pair) in fixtures.into_iter().zip(laws["ordinaryLeaves"]["pairs"].as_array().unwrap()) {
        let before = decode_pdf(before_bytes).unwrap();
        let after = decode_pdf(after_bytes).unwrap();
        let tag = pair["tag"].as_u64().unwrap() as u8;
        let [forward, inverse] = ["forward", "inverse"].map(|side| {
            let frame = leaf_archive_frame(tag, &pair[side]);
            receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap()
        });
        assert_eq!(semio_s_artifact_stdio_contract::mutation_inverse(&forward, &before).unwrap(), vec![inverse.clone()]);
        let mut state = before.clone();
        apply_pdf_mutation(&mut state, &forward);
        assert_eq!(target(&state), target(&after));
        apply_pdf_mutation(&mut state, &inverse);
        assert_eq!(target(&state), target(&before));
    }
}
