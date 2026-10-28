//! 🧪 Original UTF-8 publication retains source authority and cancellation custody.
use super::*;
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant, ValueRefusalKind};
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use std::cell::Cell;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("step-publication.json")).unwrap() }
fn grant(value: &serde_json::Value) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: value["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: value["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: value["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: value["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: value["maximumDepth"].as_u64().unwrap() as usize } }
fn source(value: &serde_json::Value) -> IfcSnapshot { parse_ifc4x3_step(&format!("ISO-10303-21;HEADER;FILE_DESCRIPTION(('test'),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;{}ENDSEC;END-ISO-10303-21;", value["data"].as_str().unwrap())).unwrap() }
fn drain(native: &mut NativeEncodeControl<'_>, closing: RetainedCloneGrant) { for _ in 0..100000 { if !native.has_retirement_owner() { return; } native.close_retirement_recipient(closing).unwrap(); } panic!("STEP original recipient failed to drain"); }

#[test]
fn actual_original_step_publication_roundtrips_the_native_utf8_artifact() {
    let value = fixture(); let source = source(&value); let pointer = source.entities.as_ptr(); let mut callback = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new(); let mut native = NativeEncodeControl::new(value["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut callback); native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, grant(&value["bodyGrant"]));
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| write_ifc4x3_step_pack(&source, &mut owner));
    let bytes = result.unwrap(); let receipt = owner.progress(); assert!(!heap.overflowed); assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes)); assert!(receipt.retained_capacity_bytes > bytes.len()); drop(owner);
    assert!(!native.has_retirement_owner()); assert_eq!(source.entities.as_ptr(), pointer);
    let text = TxtSnapshot::decode_pack(&bytes).unwrap(); assert_eq!(parse_ifc4x3_step(&text.to_body()).unwrap(), source);
}

#[test]
fn every_short_publication_cancel_prefix_keeps_the_original_recipient() {
    let value = fixture(); let source = source(&value); let pointer = source.entities.as_ptr(); let calls = Cell::new(0usize); let allowance = Cell::new(usize::MAX);
    let mut callback = |_| { let count = calls.get(); calls.set(count + 1); count < allowance.get() };
    let mut recipient = NativeEncodeRetirementRecipient::new(); let mut native = NativeEncodeControl::new(value["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut callback); native.install_retirement_recipient(&mut recipient).unwrap();
    let baseline = write_ifc4x3_step_pack(&source, &mut semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, grant(&value["bodyGrant"]))).unwrap(); drop(baseline); let count = calls.get(); drop(native); drop(recipient);
    for prefix in 0..count {
        calls.set(0); allowance.set(prefix); let mut recipient = NativeEncodeRetirementRecipient::new(); let mut native = NativeEncodeControl::new(value["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut callback); native.install_retirement_recipient(&mut recipient).unwrap();
        let mut owner = semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, grant(&value["bodyGrant"]));
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| write_ifc4x3_step_pack(&source, &mut owner)); let receipt = owner.progress(); drop(owner);
        assert!(!heap.overflowed); assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes), "prefix {prefix}"); assert_eq!(result.unwrap_err().kind, ValueRefusalKind::Canceled, "prefix {prefix}"); allowance.set(usize::MAX); drain(&mut native, grant(&value["closeGrant"])); assert_eq!(source.entities.as_ptr(), pointer);
    }
}

#[test]
fn each_original_grant_axis_can_refuse_before_publication_and_later_drain() {
    let value = fixture(); let source = source(&value);
    for axis in value["deniedAxes"].as_array().unwrap() {
        let mut incoming = grant(&value["bodyGrant"]); match axis.as_str().unwrap() { "items" => incoming.maximum_items = 0, "copy" => incoming.maximum_copy_bytes = 0, "capacity" => incoming.maximum_capacity_bytes = 0, "release" => incoming.maximum_release_bytes = 0, "depth" => incoming.maximum_depth = 0, _ => unreachable!() }
        let mut callback = |_| true; let mut recipient = NativeEncodeRetirementRecipient::new(); let mut native = NativeEncodeControl::new(value["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut callback); native.install_retirement_recipient(&mut recipient).unwrap();
        let result = write_ifc4x3_step_pack(&source, &mut semio_framework_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, incoming)); assert!(result.is_err(), "{axis}"); drain(&mut native, grant(&value["closeGrant"]));
    }
}

#[test]
fn actual_registered_step_entry_retains_the_original_native_pair() {
    use semio_framework_os_kernel::io::{io_mechanism::{IoNativeDirection, IoRunControl}, io_schema::IoPayload};
    use semio_framework_value::NativeDecodeControl;
    use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
    let value = fixture(); let source = source(&value); let input = IoPayload::Binary(source.encode_pack()); let mut observe_decode = |_| true; let mut observe_encode = |_| true;
    let mut decode_recipient = NativeDecodeRetirementRecipient::new(); let mut encode_recipient = NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(value["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut observe_decode); let mut encode = NativeEncodeControl::new(value["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut observe_encode); decode.install_retirement_recipient(&mut decode_recipient).unwrap(); encode.install_retirement_recipient(&mut encode_recipient).unwrap();
    let mut control = IoRunControl::new(&mut decode, &mut encode, grant(&value["bodyGrant"]));
    let entry = crate::native_exchange::io_entries().iter().find(|entry| entry.from == IFC4X3_DIALECT && entry.into == TXT_DIALECT).unwrap();
    let result = (entry.run)(&input, &mut control).unwrap(); assert_eq!(control.direction(), IoNativeDirection::Decode); drop(control); assert!(!decode.has_retirement_owner()); assert!(!encode.has_retirement_owner());
    let IoPayload::Binary(bytes) = result.value else { unreachable!() }; let output = TxtSnapshot::decode_pack(&bytes).unwrap(); assert_eq!(parse_ifc4x3_step(&output.to_body()).unwrap(), source);
}
