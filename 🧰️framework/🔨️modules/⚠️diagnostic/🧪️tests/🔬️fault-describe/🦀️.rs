
use super::*;

#[test]
fn describe_pairs_the_code_with_the_message() {
    let fault = Fault::new(FaultOrigin::Renderer, "renderer.command.rejected", "envelope set is full");
    assert_eq!(fault.describe(), "renderer.command.rejected: envelope set is full");
}

#[test]
fn describe_keeps_the_code_when_the_message_is_empty() {
    let fault = Fault::new(FaultOrigin::Os, "os.fault.decode", "");
    assert_eq!(fault.describe(), "os.fault.decode: ");
}

#[test]
fn describe_survives_a_wire_round_trip() {
    let fault = Fault::new(FaultOrigin::Plugin, "plugin.host.body-too-large", "body exceeds the admitted budget");
    let decoded = decode_fault_bytes(&encode_fault_bytes(&fault));
    assert_eq!(decoded.describe(), fault.describe());
}
