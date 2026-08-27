//! 🧪️ Canonical pack parity, exact frozen-owner transfer, cancellation, and fault-latching laws.

use super::*;
use crate::os_dsl::schema::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
use serde::Deserialize;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

//#region 🔣️Fixture
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Fixture { version: u8, grants: Vec<usize>, cancel_after_steps: Vec<usize>, terminal_empty: bool, capture: Capture, cases: Vec<Case> }
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Capture { ordinal: usize, value: usize, later_ordinal: usize, later_value: usize, projections: usize, wire: Vec<u8> }
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Case { id: String, ordinal: u64, fields: [Option<Field>; 3], wire_bytes: usize, symbols: usize }
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
enum Field { Text { unit: String, repeat: usize, suffix: String }, U64 { value: String }, F64 { value: String } }
struct Root { ordinal: u64, fields: [Option<FieldValue>; 3], dropped: Arc<AtomicUsize> }
impl Drop for Root { fn drop(&mut self) { self.dropped.fetch_add(1, Ordering::SeqCst); } }
fn fixture() -> Fixture { let fixture: Fixture = serde_json::from_str(include_str!("🧪️fixture.json")).unwrap(); assert_eq!(fixture.version, 1); assert!(fixture.terminal_empty); fixture }
fn root(case: &Case) -> Arc<Root> {
    Arc::new(Root { ordinal: case.ordinal, fields: std::array::from_fn(|index| case.fields[index].as_ref().map(|field| match field {
        Field::Text { unit, repeat, suffix } => FieldValue::Text(unit.repeat(*repeat) + suffix),
        Field::U64 { value } => FieldValue::UInt(value.parse().unwrap()), Field::F64 { value } => FieldValue::Float(value.parse().unwrap()),
    })), dropped: Arc::new(AtomicUsize::new(0)) })
}
fn view(root: &Root) -> Result<ScalarRecordView<'_>, &'static str> {
    Ok(ScalarRecordView { ordinal: root.ordinal, fields: std::array::from_fn(|index| root.fields[index].as_ref().map(|field| match field {
        FieldValue::Text(value) => ScalarRecordField::Text(value), FieldValue::UInt(value) => ScalarRecordField::U64(*value), FieldValue::Float(value) => ScalarRecordField::F64(*value), _ => unreachable!(),
    })) })
}
fn actual_wire(root: &Root) -> Vec<u8> {
    let spec = RecordSpec::new(None, RecordLayout::Inline, root.fields.iter().enumerate().filter_map(|(index, field)| field.as_ref().map(|field| FieldSpec::new(index as u16, &format!("field{index}"), match field { FieldValue::Text(_) => Shape::Text, FieldValue::UInt(_) => Shape::UInt, FieldValue::Float(_) => Shape::Float, _ => unreachable!() }))).collect());
    let record = RecordValue { fields: root.fields.iter().enumerate().filter_map(|(index, field)| field.clone().map(|field| (index as u16, field))).collect() };
    let mut bytes = vec![1]; let mut ordinal = root.ordinal;
    loop { let byte = (ordinal & 127) as u8; ordinal >>= 7; bytes.push(byte | if ordinal == 0 { 0 } else { 128 }); if ordinal == 0 { break; } }
    bytes.extend(crate::os_pack::encode_record_body(&spec, &record, &crate::os_pack::EncodeOptions::default()).unwrap()); bytes
}
fn run(cursor: &mut ScalarRecordWireWitness<Root>, bytes: &[u8], grant: usize, stop: Option<usize>) -> Result<bool, &'static str> {
    let mut input = 0; let mut steps = 0;
    loop { let mut charged = 0;
        for _ in 0..grant {
            if stop == Some(steps) { return Ok(false); } steps += 1;
            match cursor.advance(bytes.get(input).copied())? {
                ScalarRecordWireStep::Progress { compared_bytes } => { assert!(compared_bytes <= 1); charged += compared_bytes; }
                ScalarRecordWireStep::Consumed { compared_bytes } => { assert_eq!(compared_bytes, 1); charged += compared_bytes; input += 1; }
                ScalarRecordWireStep::Complete => { assert_eq!(input, bytes.len()); assert_eq!(cursor.consumed_bytes(), bytes.len()); return Ok(true); }
            }
            assert!(charged <= grant); assert!(steps < 500_000);
        }
    }
}
fn close(mut cursor: ScalarRecordWireWitness<Root>, expected: &Arc<Root>) {
    cursor.begin_close(); let recovered = cursor.take_root().unwrap(); assert!(Arc::ptr_eq(&recovered, expected)); assert!(cursor.terminal_is_empty()); drop(recovered);
}
//#endregion 🔣️Fixture

//#region 🧪️Laws
#[test]
fn scalar_wire_matches_actual_pack_codec_at_one_and_production_grants() {
    let fixture = fixture();
    for case in &fixture.cases { let root = root(case); let wire = actual_wire(&root); assert_eq!(wire.len(), case.wire_bytes, "{}", case.id);
        let ordinal_bytes = if case.ordinal < 128 { 1 } else { 2 }; assert_eq!(wire[1 + ordinal_bytes] as usize, case.symbols);
        for grant in &fixture.grants { let mut cursor = ScalarRecordWireWitness::new(root.clone(), view); assert!(run(&mut cursor, &wire, *grant, None).unwrap()); close(cursor, &root); }
    }
}

#[test]
fn scalar_wire_cancel_transfers_exact_last_owner_after_worker_move() {
    let fixture = fixture();
    for case in &fixture.cases { for stop in &fixture.cancel_after_steps { let mut root = root(case); let dropped = root.dropped.clone(); let wire = actual_wire(&root);
        let mut cursor = ScalarRecordWireWitness::new(root.clone(), view); assert!(Arc::get_mut(&mut root).is_none()); let _ = run(&mut cursor, &wire, 1, Some(*stop)).unwrap();
        drop(root); assert_eq!(dropped.load(Ordering::SeqCst), 0);
        std::thread::spawn(move || { cursor.begin_close(); let owner = cursor.take_root().unwrap(); assert!(cursor.terminal_is_empty()); drop(owner); }).join().unwrap();
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
    } }
}

#[test]
fn scalar_wire_malformed_input_latches_fault_until_terminal_close() {
    let fixture = fixture();
    for case in &fixture.cases { let root = root(case); let wire = actual_wire(&root);
        let mut bad_format = wire.clone(); bad_format[0] ^= 1; let mut bad_ordinal = wire.clone(); bad_ordinal[1] ^= 1; let mut trailing = wire.clone(); trailing.push(0);
        let mut last_byte = wire.clone(); *last_byte.last_mut().unwrap() ^= 1;
        for invalid in [bad_format, bad_ordinal, trailing, last_byte, wire[..wire.len()-1].to_vec()] { let mut cursor = ScalarRecordWireWitness::new(root.clone(), view);
            assert!(run(&mut cursor, &invalid, 1, None).is_err()); assert!(cursor.advance(Some(1)).is_err()); assert!(cursor.take_root().is_none()); close(cursor, &root);
        }
    }
}

#[test]
fn scalar_wire_captures_atomic_source_projection_once() {
    let capture = fixture().capture;
    struct AtomicRoot { ordinal: AtomicUsize, value: AtomicUsize, projections: AtomicUsize }
    fn project(root: &AtomicRoot) -> Result<ScalarRecordView<'_>, &'static str> {
        root.projections.fetch_add(1, Ordering::SeqCst);
        Ok(ScalarRecordView { ordinal: root.ordinal.load(Ordering::SeqCst) as u64, fields: [Some(ScalarRecordField::U64(root.value.load(Ordering::SeqCst) as u64)), None, None] })
    }
    let root = Arc::new(AtomicRoot { ordinal: AtomicUsize::new(capture.ordinal), value: AtomicUsize::new(capture.value), projections: AtomicUsize::new(0) });
    let mut cursor = ScalarRecordWireWitness::new(root.clone(), project);
    assert_eq!(cursor.advance(Some(1)), Ok(ScalarRecordWireStep::Progress { compared_bytes: 0 }));
    root.ordinal.store(capture.later_ordinal, Ordering::SeqCst); root.value.store(capture.later_value, Ordering::SeqCst);
    let wire = capture.wire; let mut offset = 0;
    for _ in 0..100 { match cursor.advance(wire.get(offset).copied()).unwrap() {
        ScalarRecordWireStep::Consumed { .. } => offset += 1,
        ScalarRecordWireStep::Progress { .. } => {},
        ScalarRecordWireStep::Complete => break,
    } }
    assert_eq!(offset, wire.len()); assert_eq!(root.projections.load(Ordering::SeqCst), capture.projections);
    cursor.begin_close(); assert!(Arc::ptr_eq(&cursor.take_root().unwrap(), &root)); assert!(cursor.terminal_is_empty());
}
//#endregion 🧪️Laws
