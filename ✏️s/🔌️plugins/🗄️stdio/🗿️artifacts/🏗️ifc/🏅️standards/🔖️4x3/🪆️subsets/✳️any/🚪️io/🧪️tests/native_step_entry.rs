//! 🧪 The registered STEP-to-UTF-8 route retains the original native pair.
use super::*;
use semio_s_artifact_stdio_ifc4x3_step::{parse_ifc4x3_step, IFC4X3_DIALECT, TXT_DIALECT};
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant};
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("step-publication.json")).unwrap() }
fn grant(value: &serde_json::Value) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: value["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: value["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: value["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: value["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: value["maximumDepth"].as_u64().unwrap() as usize } }
fn source(value: &serde_json::Value) -> IfcSnapshot { parse_ifc4x3_step(&format!("ISO-10303-21;HEADER;FILE_DESCRIPTION(('test'),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4X3_ADD2'));ENDSEC;DATA;{}ENDSEC;END-ISO-10303-21;", value["data"].as_str().unwrap())).unwrap() }
fn drain(native: &mut NativeEncodeControl<'_>, closing: RetainedCloneGrant) { for _ in 0..100000 { if !native.has_retirement_owner() { return; } native.close_retirement_recipient(closing).unwrap(); } panic!("STEP original recipient failed to drain"); }

#[test]
fn actual_registered_step_entry_retains_the_original_native_pair() {
    use semio_framework_os_kernel::io::{io_mechanism::{IoNativeDirection, IoRunControl}, io_schema::IoPayload};
    use semio_framework_value::NativeDecodeControl;
    use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;
    let value = fixture(); let source = source(&value); let input = IoPayload::Binary(source.encode_pack()); let mut observe_decode = |_| true; let mut observe_encode = |_| true;
    let mut decode_recipient = NativeDecodeRetirementRecipient::new(); let mut encode_recipient = NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(value["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut observe_decode); let mut encode = NativeEncodeControl::new(value["nativeMaximumBytes"].as_u64().unwrap() as usize, &mut observe_encode); decode.install_retirement_recipient(&mut decode_recipient).unwrap(); encode.install_retirement_recipient(&mut encode_recipient).unwrap();
    let mut control = IoRunControl::new(&mut decode, &mut encode, grant(&value["bodyGrant"]));
    let entry = io_entries().iter().find(|entry| entry.from == IFC4X3_DIALECT && entry.into == TXT_DIALECT).unwrap();
    let result = (entry.run)(&input, &mut control).unwrap(); assert_eq!(control.direction(), IoNativeDirection::Decode); drop(control); assert!(!decode.has_retirement_owner()); assert!(!encode.has_retirement_owner());
    let IoPayload::Binary(bytes) = result.value else { unreachable!() }; let output = TxtSnapshot::decode_pack(&bytes).unwrap(); assert_eq!(parse_ifc4x3_step(&output.to_body()).unwrap(), source);
}
