//#region 🧪️ReturnControlWire
use crate::return_page::*;
use crate::byte_page::{ActorBytePage, ACTOR_BYTE_PAGE_BYTES};

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap() }
fn unhex(value: &str) -> Vec<u8> { (0..value.len()).step_by(2).map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16).unwrap()).collect() }

#[test]
fn actor_return_wire_drives_match_shared_vectors_and_reject_all_prefixes() {
    let fixture = fixture();
    assert_eq!(ACTOR_RETURN_DRIVE_MAXIMUM_BYTES, fixture["maximumDriveBytes"].as_u64().unwrap() as usize);
    for row in fixture["wireVectors"].as_array().unwrap() {
        let value: ActorReturnDrive = serde_json::from_value(row["value"].clone()).unwrap();
        let mut bytes = [91; ACTOR_RETURN_DRIVE_MAXIMUM_BYTES];
        let length = value.encode(&mut bytes).unwrap();
        assert_eq!(&bytes[..length], unhex(row["hex"].as_str().unwrap()));
        assert_eq!(ActorReturnDrive::decode(&bytes[..length]).unwrap(), value);
        assert_eq!(serde_json::to_value(value).unwrap(), row["value"]);
        for prefix in 0..length { assert!(ActorReturnDrive::decode(&bytes[..prefix]).is_err()); }
        assert!(bytes[length..].iter().all(|byte| *byte == 91));
    }
}

#[test]
fn actor_return_wire_invalid_drives_leave_the_destination_untouched() {
    let fixture = fixture();
    for row in fixture["malformedWire"].as_array().unwrap() { assert!(ActorReturnDrive::decode(&unhex(row.as_str().unwrap())).is_err()); }
    let origin: ActorReturnOrigin = serde_json::from_value(fixture["origin"].clone()).unwrap();
    let receipt: ActorReturnPageReceipt = serde_json::from_value(fixture["receipt"].clone()).unwrap();
    for value in [
        ActorReturnDrive::Execute { origin: ActorReturnOrigin { activation_generation: 0, ..origin } },
        ActorReturnDrive::Execute { origin: ActorReturnOrigin { request_sequence: 9_007_199_254_740_992, ..origin } },
        ActorReturnDrive::Control { control: ActorReturnControl::InputAck { receipt: ActorReturnPageReceipt { length: 4097, ..receipt } } },
        ActorReturnDrive::Control { control: ActorReturnControl::InputAck { receipt: ActorReturnPageReceipt { length: 0, final_page: false, ..receipt } } },
    ] {
        let mut bytes = [73; ACTOR_RETURN_DRIVE_MAXIMUM_BYTES];
        assert!(value.encode(&mut bytes).is_err());
        assert_eq!(bytes, [73; ACTOR_RETURN_DRIVE_MAXIMUM_BYTES]);
    }
}

fn result(value: &serde_json::Value) -> ActorReturnResult {
    match value["kind"].as_str().unwrap() {
        "refused" => ActorReturnResult::Refused { origin: serde_json::from_value(value["origin"].clone()).unwrap(), fault: serde_json::from_value(value["fault"].clone()).unwrap() },
        "pending" => ActorReturnResult::Pending { identity: serde_json::from_value(value["identity"].clone()).unwrap(), reason: serde_json::from_value(value["reason"].clone()).unwrap() },
        "retired" => ActorReturnResult::Retired { identity: serde_json::from_value(value["identity"].clone()).unwrap(), completion: serde_json::from_value(value["completion"].clone()).unwrap() },
        "control" => ActorReturnResult::Control { control: serde_json::from_value(value["control"].clone()).unwrap(), outcome: serde_json::from_value(value["outcome"].clone()).unwrap(), fault: serde_json::from_value(value["fault"].clone()).unwrap() },
        "protocolFault" => ActorReturnResult::ProtocolFault { fault: serde_json::from_value(value["fault"].clone()).unwrap() },
        _ => unreachable!(),
    }
}

#[test]
fn actor_return_wire_fixed_results_and_maximum_page_match_independent_oracles() {
    let fixture = fixture();
    assert_eq!(ACTOR_RETURN_RESULT_MAXIMUM_BYTES, fixture["maximumResultBytes"].as_u64().unwrap() as usize);
    for row in fixture["resultVectors"].as_array().unwrap() {
        let value = result(&row["value"]);
        let mut bytes = [91; ACTOR_RETURN_RESULT_MAXIMUM_BYTES];
        let length = value.encode(&mut bytes).unwrap();
        assert_eq!(&bytes[..length], unhex(row["hex"].as_str().unwrap()));
        assert_eq!(ActorReturnResult::decode(&bytes[..length]).unwrap(), value);
        for prefix in 0..length { assert!(ActorReturnResult::decode(&bytes[..prefix]).is_err()); }
        assert!(bytes[length..].iter().all(|byte| *byte == 91));
    }
    for row in fixture["pageResultVectors"].as_array().unwrap() {
        let length = row["pageLength"].as_u64().unwrap() as usize;
        let source: Vec<_> = (0..length).map(|index| if row["pattern"] == "zero" { 0 } else { ((index * 37 + 11) % 256) as u8 }).collect();
        let value = ActorReturnResult::Page { receipt: serde_json::from_value(row["receipt"].clone()).unwrap(), page: ActorBytePage::try_copy_from(&source).unwrap() };
        let mut bytes = [0; ACTOR_RETURN_RESULT_MAXIMUM_BYTES];
        let written = value.encode(&mut bytes).unwrap();
        let prefix = unhex(row["prefixHex"].as_str().unwrap());
        assert_eq!(written, row["wireBytes"].as_u64().unwrap() as usize);
        assert_eq!(&bytes[..prefix.len()], prefix);
        assert_eq!(&bytes[prefix.len()..prefix.len() + source.len()], source);
        assert_eq!(written - prefix.len(), ACTOR_BYTE_PAGE_BYTES);
        assert_eq!(ActorReturnResult::decode(&bytes[..written]).unwrap(), value);
        assert!(ActorReturnResult::decode(&bytes[..written - 1]).is_err());
        if length == 0 {
            bytes[prefix.len()] = 1;
            assert!(ActorReturnResult::decode(&bytes[..written]).is_err());
        }
    }
}

#[test]
fn actor_return_wire_result_faults_and_pairing_fail_before_write() {
    let fixture = fixture();
    for value in fixture["resultContradictions"].as_array().unwrap().iter().take(5) {
        let mut bytes = [73; ACTOR_RETURN_RESULT_MAXIMUM_BYTES];
        assert!(result(value).encode(&mut bytes).is_err());
        assert_eq!(bytes, [73; ACTOR_RETURN_RESULT_MAXIMUM_BYTES]);
    }
    let receipt: ActorReturnPageReceipt = serde_json::from_value(fixture["receipt"].clone()).unwrap();
    let value = ActorReturnResult::Page { receipt, page: ActorBytePage::try_copy_from(&[]).unwrap() };
    let mut bytes = [73; ACTOR_RETURN_RESULT_MAXIMUM_BYTES];
    assert!(value.encode(&mut bytes).is_err());
    assert_eq!(bytes, [73; ACTOR_RETURN_RESULT_MAXIMUM_BYTES]);
    for hex in ["05", "0501", "0107090b04", "0307090b03", "040007090b0000", "040307090b000a", "00070900"] { assert!(ActorReturnResult::decode(&unhex(hex)).is_err()); }
    for row in fixture["preAdmissionFaults"].as_array().unwrap() {
        assert!(ActorReturnDrive::decode(&unhex(row["invalidDriveHex"].as_str().unwrap())).is_err());
        let value = ActorReturnResult::ProtocolFault { fault: ActorReturnFault::MalformedControl };
        let length = value.encode(&mut bytes).unwrap();
        assert_eq!(&bytes[..length], unhex(row["resultHex"].as_str().unwrap()));
    }
}

#[test]
fn actor_return_wire_all_control_outcome_fault_combinations_match_schema_rules() {
    let fixture = fixture();
    let controls = &fixture["wireVectors"].as_array().unwrap()[1..5];
    let mut cases = 0;
    for (control_index, row) in controls.iter().enumerate() {
        let control: ActorReturnControl = serde_json::from_value(row["value"]["control"].clone()).unwrap();
        let control_bytes = unhex(row["hex"].as_str().unwrap());
        for (outcome_index, outcome) in fixture["resultEnums"]["outcome"].as_array().unwrap().iter().enumerate() {
            for (fault_index, fault) in fixture["resultEnums"]["fault"].as_array().unwrap().iter().enumerate() {
                let expected = if control_index == 0 { outcome_index >= 2 && fault_index != 0 }
                    else if outcome_index < 2 { fault_index == 0 } else { fault_index != 0 };
                let value = ActorReturnResult::Control { control, outcome: serde_json::from_value(outcome.clone()).unwrap(), fault: serde_json::from_value(fault.clone()).unwrap() };
                let mut output = [73; ACTOR_RETURN_RESULT_MAXIMUM_BYTES];
                let encoded = value.encode(&mut output);
                assert_eq!(encoded.is_ok(), expected, "control={control_index} outcome={outcome_index} fault={fault_index}");
                let mut wire = vec![4];
                wire.extend_from_slice(&control_bytes[1..]);
                wire.extend_from_slice(&[outcome_index as u8, fault_index as u8]);
                let decoded = ActorReturnResult::decode(&wire);
                assert_eq!(decoded.is_ok(), expected);
                if expected {
                    assert_eq!(&output[..encoded.unwrap()], wire);
                    assert_eq!(decoded.unwrap(), value);
                } else { assert_eq!(output, [73; ACTOR_RETURN_RESULT_MAXIMUM_BYTES]); }
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 224);
    assert!(serde_json::from_value::<ActorReturnPendingReason>(fixture["resultContradictions"][5]["reason"].clone()).is_err());
    assert!(serde_json::from_value::<ActorReturnCompletion>(fixture["resultContradictions"][6]["completion"].clone()).is_err());
    let mut output = [73; ACTOR_RETURN_RESULT_MAXIMUM_BYTES];
    assert!(result(&fixture["resultContradictions"][7]).encode(&mut output).is_err());
    assert_eq!(output, [73; ACTOR_RETURN_RESULT_MAXIMUM_BYTES]);
}
//#endregion 🧪️ReturnControlWire
