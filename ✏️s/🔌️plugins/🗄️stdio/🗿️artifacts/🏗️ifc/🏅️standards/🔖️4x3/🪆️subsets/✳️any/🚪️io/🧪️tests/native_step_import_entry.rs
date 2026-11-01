//! 🧪 The registered UTF-8-to-STEP route returns an original native graph pack.
use super::*;
use semio_s_artifact_stdio_ifc4x3_step::{TXT_DIALECT, IFC4X3_DIALECT, write_ifc4x3_step};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcSnapshot;
use semio_s_artifact_stdio_txt::TxtSnapshot;
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant};
use semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient;

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 16_000_000, maximum_copy_bytes: 1024 * 1024 * 1024, maximum_capacity_bytes: 128 * 1024 * 1024, maximum_release_bytes: 128 * 1024 * 1024, maximum_depth: 512 }
}
fn retire<T: semio_framework_value::retirement::RetireOwned>(value: T) {
    let mut r = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    while !r.terminal_is_empty() {
        r.step(grant()).unwrap();
    }
}

#[test]
fn actual_registered_txt_ifc_route_returns_an_original_native_graph_pack() {
    use pack::snapshot::ArtifactPack;
    use semio_framework_os_kernel::io::{io_mechanism::IoRunControl, io_schema::IoPayload};
    let source = TxtSnapshot::from_body(include_str!("triangulated.ifc"));
    let input = IoPayload::Binary(source.encode_pack());
    let mut receive = |_| true;
    let mut publish = |_| true;
    let mut dr = NativeDecodeRetirementRecipient::new();
    let mut er = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut decode = NativeDecodeControl::new(128 * 1024 * 1024, &mut receive);
    let mut encode = semio_framework_value::NativeEncodeControl::new(128 * 1024 * 1024, &mut publish);
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let mut control = IoRunControl::new(&mut decode, &mut encode, grant());
    let entry = io_entries().iter().find(|e| e.from == TXT_DIALECT && e.into == IFC4X3_DIALECT).unwrap();
    let output = (entry.run)(&input, &mut control).unwrap();
    let IoPayload::Binary(bytes) = &output.value else { panic!() };
    let graph = IfcSnapshot::decode_pack(bytes).unwrap();
    assert_eq!(graph.entities.len(), 35);
    drop(control);
    assert!(!decode.has_retirement_owner() && !encode.has_retirement_owner());
    if let Ok(path) = std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {
        std::fs::create_dir_all(&path).unwrap();
        std::fs::write(std::path::Path::new(&path).join("native-step-import.ifc"), write_ifc4x3_step(&graph).unwrap()).unwrap();
    }
    retire((source, input, output, graph));
}
