use super::store;
use store::io_schema::IoPayload;
use store::sqlite_snapshot::{SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteSnapshotPhase};
use semio_framework_diagnostic::{TextError, TextSpan};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::cell::Cell;

#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct AllocationRecord { literal: String }

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap() }
fn literal() -> String { let case = fixture(); case["literal"]["unit"].as_str().unwrap().repeat(case["literal"]["repeat"].as_u64().unwrap() as usize) }
fn limits() -> SqliteDatabaseLimits { SqliteDatabaseLimits { max_allocation_bytes: fixture()["maximumAllocationBytes"].as_u64().unwrap() as usize, ..SqliteDatabaseLimits::default() } }

fn encode(source: &AllocationRecord, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<IoPayload, ValueError> {
    store::encode_sqlite_snapshot_record_native(encoding, fixture()["envelopeId"].as_str().unwrap(), AllocationRecord::__dsl_spec_producer(), |native| source.__dsl_to_record_controlled(native), control)
}
fn decode(payload: &IoPayload, control: &mut SqliteSnapshotControl<'_>) -> Result<AllocationRecord, ValueError> {
    decode_at_binding(payload,control,&Cell::new(false))
}
fn decode_at_binding(payload:&IoPayload,control:&mut SqliteSnapshotControl<'_>,binding:&Cell<bool>)->Result<AllocationRecord,ValueError>{
    store::decode_sqlite_snapshot_record_native(payload, fixture()["envelopeId"].as_str().unwrap(), AllocationRecord::__dsl_spec_producer(), |record, native| {binding.set(true);let result=AllocationRecord::__dsl_from_record_controlled(record,native);binding.set(false);result}, control)
}

#[test]
fn sqlite_snapshot_native_allocation_encode_settles_success_before_repeated_production() {
    let source = AllocationRecord { literal: literal() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let limits = limits(); let mut yes = |_| true; let mut control = SqliteSnapshotControl::new(&mut yes, limits);
        let output = encode(&source, encoding, &mut control).unwrap();
        let used = limits.max_allocation_bytes - control.allocation_remaining_bytes();
        assert!(used >= source.literal.len(), "{encoding:?}: native ownership must settle into its caller");
        let exact = SqliteDatabaseLimits { max_allocation_bytes: used, ..limits }; let mut yes = |_| true; let mut control = SqliteSnapshotControl::new(&mut yes, exact);
        assert_eq!(encode(&source, encoding, &mut control).unwrap(), output); assert_eq!(control.allocation_remaining_bytes(), 0);
        let mut entered = false;
        let result = store::encode_sqlite_snapshot_record_native(encoding, "fixture.allocation", AllocationRecord::__dsl_spec_producer(), |native| { entered = true; source.__dsl_to_record_controlled(native) }, &mut control);
        assert!(result.is_err()); assert!(!entered, "metadata must refuse before subsequent literal construction"); assert_eq!(control.allocation_remaining_bytes(), 0);
        let mut yes = |_| true; let mut control = SqliteSnapshotControl::new(&mut yes, SqliteDatabaseLimits { max_value_bytes: fixture()["semanticBytes"].as_u64().unwrap() as usize, ..limits });
        assert_eq!(encode(&source, encoding, &mut control).unwrap(), output, "native backing is independent of semantic SQL payload");
    }
}

#[test]
fn sqlite_snapshot_native_allocation_decode_settles_success_before_repeated_binding() {
    let source = AllocationRecord { literal: literal() };
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let limits = limits(); let payload = encode(&source, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        let mut yes = |_| true; let mut control = SqliteSnapshotControl::new(&mut yes, limits);
        assert_eq!(decode(&payload, &mut control).unwrap().literal, source.literal);
        let used = limits.max_allocation_bytes - control.allocation_remaining_bytes(); assert!(used >= source.literal.len(), "{encoding:?}: parser and binding admission must settle");
        let mut yes = |_| true; let mut control = SqliteSnapshotControl::new(&mut yes, SqliteDatabaseLimits { max_allocation_bytes: used, ..limits });
        assert_eq!(decode(&payload, &mut control).unwrap().literal, source.literal); assert_eq!(control.allocation_remaining_bytes(), 0);
        let mut entered = false; let result = store::decode_sqlite_snapshot_record_native(&payload, "fixture.allocation", AllocationRecord::__dsl_spec_producer(), |record, native| { entered = true; AllocationRecord::__dsl_from_record_controlled(record, native) }, &mut control);
        assert!(result.is_err()); assert!(!entered); assert_eq!(control.allocation_remaining_bytes(), 0);
        let mut yes = |_| true; let mut control = SqliteSnapshotControl::new(&mut yes, SqliteDatabaseLimits { max_value_bytes: fixture()["semanticBytes"].as_u64().unwrap() as usize, ..limits });
        assert_eq!(decode(&payload, &mut control).unwrap().literal, source.literal);
    }
}

#[test]
fn sqlite_snapshot_native_allocation_canceled_input_and_output_keep_actual_admission() {
    let source = AllocationRecord { literal: literal() }; let neutral = fixture(); let threshold = neutral["literal"]["cancelAfterBytes"].as_u64().unwrap() as usize;
    let observed=Cell::new(false);let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.total==source.literal.len()&&event.completed>threshold&&event.completed<event.total{assert_eq!(event.owned_bytes,neutral["cancellation"]["stages"][0]["ownedLiteralBytes"].as_u64().unwrap()as usize);observed.set(true);false}else{true}};let mut borrowed=semio_framework_value::NativeDecodeControl::new(0,&mut callback);assert!(matches!(borrowed.borrow_text(source.literal.as_bytes()),Err(ValueError{kind:ValueRefusalKind::Canceled,..})));assert!(observed.get());assert_eq!(borrowed.owned_bytes(),0);assert!(matches!(borrowed.copy_text(&source.literal),Err(ValueError{kind:ValueRefusalKind::OwnershipLimit,..})));assert_eq!(borrowed.owned_bytes(),0);
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let limits = limits(); let payload = encode(&source, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap();
        for phase in [SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
            let stage=neutral["cancellation"]["stages"].as_array().unwrap().iter().find(|stage|stage["scope"].as_str()==Some(if phase==SqliteSnapshotPhase::DecodeNative{"owned-binding"}else{"owned-projection"})).unwrap();assert_eq!(stage["phase"].as_str(),Some(if phase==SqliteSnapshotPhase::DecodeNative{"DecodeNative"}else{"EncodeNative"}));assert_eq!(stage["ownedLiteralBytes"].as_u64().unwrap()as usize,source.literal.len());
            let reached = Cell::new(false);let binding=Cell::new(false);let borrowed=Cell::new(false);
            let mut callback = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { let interior=event.phase==phase&&event.total==source.literal.len()&&event.completed>threshold&&event.completed<event.total;if interior&&phase==SqliteSnapshotPhase::DecodeNative&&!binding.get(){borrowed.set(true);}if interior&&(phase!=SqliteSnapshotPhase::DecodeNative||binding.get()){reached.set(true);false}else{true} };
            let mut control = SqliteSnapshotControl::new(&mut callback, limits);
            let result = match phase { SqliteSnapshotPhase::DecodeNative => decode_at_binding(&payload, &mut control,&binding).map(|_| ()), SqliteSnapshotPhase::EncodeNative => encode(&source, encoding, &mut control).map(|_| ()), _ => unreachable!() };
            assert!(result.is_err()); assert!(reached.get(), "{encoding:?}/{phase:?}: actual interior copy must cancel");
            assert!(!binding.get());if encoding==SnapshotEncoding::Binary&&phase==SqliteSnapshotPhase::DecodeNative{assert!(borrowed.get(),"borrowed parser validation must finish before the selected owned binding");}
            let used = limits.max_allocation_bytes - control.allocation_remaining_bytes(); assert!(used >= source.literal.len(), "canceled allocated copy must remain charged");
            let canceled = Cell::new(false);let binding=Cell::new(false);
            let mut once = |event: store::sqlite_snapshot::SqliteSnapshotProgress| { if !canceled.get() && event.phase == phase && (phase!=SqliteSnapshotPhase::DecodeNative||binding.get())&&event.total == source.literal.len() && event.completed > threshold && event.completed < event.total { canceled.set(true); false } else { true } };
            let mut repeated = SqliteSnapshotControl::new(&mut once, SqliteDatabaseLimits { max_allocation_bytes: used, ..limits });
            let result = match phase { SqliteSnapshotPhase::DecodeNative => decode_at_binding(&payload, &mut repeated,&binding).map(|_| ()), SqliteSnapshotPhase::EncodeNative => encode(&source, encoding, &mut repeated).map(|_| ()), _ => unreachable!() };
            assert!(result.is_err()); assert!(canceled.get(), "{encoding:?}/{phase:?}: exact canceled admission {used} must reach the same owned copy: {result:?}; remaining {}", repeated.allocation_remaining_bytes()); assert_eq!(repeated.allocation_remaining_bytes(), 0);
            assert!(encode(&source, encoding, &mut repeated).is_err()); assert_eq!(repeated.allocation_remaining_bytes(), 0);
        }
    }
}

#[test]
fn sqlite_snapshot_native_allocation_typed_refusal_keeps_admission_and_literal_error() {
    let source = AllocationRecord { literal: literal() };
    let neutral:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🌱️value/⚠️refusal/🧫️fixtures/🔣️.json")).unwrap();
    let kinds=[ValueRefusalKind::InvalidValue,ValueRefusalKind::Canceled,ValueRefusalKind::OwnershipLimit,ValueRefusalKind::AllocationFailed,ValueRefusalKind::WorkLimit,ValueRefusalKind::DepthLimit,ValueRefusalKind::UnsupportedOwner,ValueRefusalKind::InvariantViolated,ValueRefusalKind::InvalidValue];
    let cases:Vec<_>=neutral["cases"].as_array().unwrap().iter().filter(|case|case["operation"]=="construct").collect();
    assert_eq!(cases.len(),kinds.len());
    for (case,kind) in cases.iter().zip(kinds) {
        assert_eq!(case["kind"].as_str(),Some(kind.as_str()));
        let message=case["message"].as_str().unwrap();
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let limits=limits();let payload=encode(&source,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
            let mut yes=|_|true;let mut control=SqliteSnapshotControl::new(&mut yes,limits);
            let result=store::encode_sqlite_snapshot_record_native(encoding,"fixture.allocation",AllocationRecord::__dsl_spec_producer(),|native|{let owned=native.copy_text(&source.literal)?;drop(owned);Err(ValueError::new(kind,message))},&mut control);
            let refusal=result.unwrap_err();assert_eq!(refusal.kind,kind);assert_eq!(refusal.message,message);let terminal=store::io_schema::IoError::from_value_error(refusal);assert_eq!(terminal.cause.kind,kind);assert_eq!(terminal.cause.message,message);assert!(limits.max_allocation_bytes-control.allocation_remaining_bytes()>=source.literal.len());
            let mut yes=|_|true;let mut control=SqliteSnapshotControl::new(&mut yes,limits);
            let result:Result<(),ValueError>=store::decode_sqlite_snapshot_record_native(&payload,"fixture.allocation",AllocationRecord::__dsl_spec_producer(),|_,native|{let owned=native.copy_text(&source.literal)?;drop(owned);Err(ValueError::new(kind,message))},&mut control);
            let refusal=result.unwrap_err();assert_eq!(refusal.kind,kind);assert_eq!(refusal.message,message);let terminal=store::io_schema::IoError::from_value_error(refusal);assert_eq!(terminal.cause.kind,kind);assert_eq!(terminal.cause.message,message);assert!(limits.max_allocation_bytes-control.allocation_remaining_bytes()>=source.literal.len());
        }
    }
}
