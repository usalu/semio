//#region 🧪️IssuedPatchReceiptLaws
use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixture/🔣️.json")).unwrap() }

fn unhex(value: &str) -> Vec<u8> { (0..value.len()).step_by(2).map(|offset| u8::from_str_radix(&value[offset..offset + 2], 16).unwrap()).collect() }

#[test]
fn actor_ui_patch_receipt_matches_shared_wire_and_json_oracles() {
    let fixture = fixture();
    assert_eq!(ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, fixture["maximumBytes"].as_u64().unwrap() as usize);
    for row in fixture["vectors"].as_array().unwrap() {
        let receipt: ActorUiPatchReceipt = serde_json::from_value(row["value"].clone()).unwrap();
        let mut bytes = [0; ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES];
        let length = receipt.encode(&mut bytes).unwrap();
        assert_eq!(&bytes[..length], unhex(row["hex"].as_str().unwrap()));
        assert_eq!(ActorUiPatchReceipt::decode(&bytes[..length]).unwrap(), receipt);
        assert_eq!(serde_json::to_value(receipt).unwrap(), row["value"]);
        for prefix in 0..length { assert!(ActorUiPatchReceipt::decode(&bytes[..prefix]).is_err()); }
    }
}

#[test]
fn actor_ui_patch_receipt_rejects_malformed_and_unpaired_authority_before_writing() {
    let fixture = fixture();
    for hex in fixture["invalidHex"].as_array().unwrap() { assert!(ActorUiPatchReceipt::decode(&unhex(hex.as_str().unwrap())).is_err()); }
    let receipt: ActorUiPatchReceipt = serde_json::from_value(fixture["vectors"][1]["value"].clone()).unwrap();
    for invalid in [ActorUiPatchReceipt { patch_sequence: 0, ..receipt }, ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { guest_lifetime: 0, ..receipt.lifetime }, ..receipt }] {
        let mut bytes = [91; ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES];
        assert!(invalid.encode(&mut bytes).is_err());
        assert_eq!(bytes, [91; ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES]);
    }
    for row in fixture["pairing"].as_array().unwrap() {
        assert_eq!(ActorUiPatchReceipt::validate_pairing(row["hasReceipt"].as_bool().unwrap().then_some(receipt), row["patchCount"].as_u64().unwrap() as usize).is_ok(), row["accepted"].as_bool().unwrap());
    }
}

#[semio_framework_async_macros::async_test]
async fn actor_ui_patch_receipt_outer_field_preserves_order_and_rejects_partial_publication() {
    let fixture = fixture();
    for row in fixture["vectors"].as_array().unwrap() {
        let receipt: ActorUiPatchReceipt = serde_json::from_value(row["value"].clone()).unwrap();
        let turn = crate::TurnResult { ui_patches: vec![91], effects: vec![], command_ingress: vec![], lifecycle_receipt: None, ui_patch_receipt: Some(receipt), next_wake: None, status: crate::TurnStatus::Idle, usage: Default::default() };
        let mut bytes = Vec::new();
        turn.pack_encode(&mut bytes).await.unwrap();
        let body = unhex(row["hex"].as_str().unwrap());
        let mut expected = vec![1, 91, 0, 0, 0, body.len() as u8];
        expected.extend_from_slice(&body);
        expected.extend_from_slice(&[0; 26]);
        assert_eq!(bytes, expected);
        assert_eq!(crate::TurnResult::pack_decode(&bytes, &mut 0).await.unwrap(), turn);
        for invalid in [crate::TurnResult { ui_patch_receipt: None, ..turn.clone() }, crate::TurnResult { ui_patches: vec![], ..turn.clone() }] {
            let mut untouched = vec![73, 74];
            assert!(invalid.pack_encode(&mut untouched).await.is_err());
            assert_eq!(untouched, [73, 74]);
        }
        let mut missing = vec![1, 91, 0, 0, 0, 0];
        missing.extend_from_slice(&[0; 26]);
        assert!(crate::TurnResult::pack_decode(&missing, &mut 0).await.is_err());
    }
}
//#endregion 🧪️IssuedPatchReceiptLaws
