use super::*;
use store::{BackboneChannelPort, BackboneMessage, OpBinary};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn document_backbone_op_binary_is_exact_canonical_and_bounded() {
    let rows = [
        (BackboneMessage::Genesis { pack: vec![0xaa] }, "01000001000801aa", false),
        (BackboneMessage::Mutations { envelopes: vec![0xdd] }, "01010001000801dd", true),
        (BackboneMessage::Ack { op_ids: Vec::new() }, "01020001000c00", true),
        (BackboneMessage::Ack { op_ids: vec!["a".into()] }, "010201016101000c010600", true),
    ];
    for (message, expected, hot) in rows {
        let encoded = message.encode_op().expect("canonical backbone message encodes");
        assert_eq!(hex(&encoded), expected);
        assert_eq!(store::decode_backbone_message_exact(&encoded).expect("exact backbone message decodes"), message);
        assert_eq!(store::decode_hot_backbone_message_exact(&encoded).is_ok(), hot);
    }
    for hostile in ["", "00020001000c00", "01030001000c00", "0182000001000c00", "010201ff01000c010600", "01020000", "01020001010c00", "01020001000800", "01020001000c0000"] {
        let bytes = (0..hostile.len()).step_by(2).map(|index| u8::from_str_radix(&hostile[index..index + 2], 16).expect("hostile hex")).collect::<Vec<_>>();
        assert!(store::decode_backbone_message_exact(&bytes).is_err(), "hostile {hostile} must be refused");
    }
}

#[semio_framework_async_macros::async_test]
async fn document_backbone_binding_reducer_preserves_generation_and_live_owner() {
    let zero_payload = DocumentBackboneBindingCommandV1 { operation: DocumentBackboneBindingOperationV1::Bind, instance_id: 0, binding_generation: 1, uri: "actor://v1:0:3:space-amap".into() }.encode().expect("canonical instance zero command encodes");
    let zero = decode_document_backbone_binding_command_v1(&zero_payload).expect("canonical instance zero command decodes").expect("binding command recognized");
    assert_eq!(zero.instance_id, 0);
    assert_eq!(DocumentBackboneBindingReceiptV1::bound(&zero).instance_id, 0);
    let uri = "actor://v1:7:3:space-amap";
    let (port, owner) = store::ActorBackboneChannelOwner::pair(uri);
    let message = BackboneMessage::Ack { op_ids: Vec::new() }.encode_op().expect("hot backbone acknowledgment encodes");
    owner.push_inbound(uri, &message).expect("live binding admits an addressed message");
    port.send(uri, &message).await.expect("live binding retains one pending acknowledgment");
    owner.begin_retire().expect("retirement closes admission synchronously");
    assert!(owner.push_inbound(uri, &message).is_err(), "retiring binding must refuse ingress before detach can await");
    assert!(owner.take_outbound().expect("retired binding exposes its discarded outbox").is_none(), "retirement must discard stale data before its receipt");
    assert!(owner.terminal_is_empty().expect("retired binding exposes exact terminal state"));
    let first = DocumentBackboneBindingCommandV1 { operation: DocumentBackboneBindingOperationV1::Bind, instance_id: 7, binding_generation: 1, uri: uri.into() };
    assert!(matches!(decide_document_backbone_binding_v1(&DocumentBackboneBindingStateV1::default(), &first), DocumentBackboneBindingDecisionV1::Bind(_)));
    let live = DocumentBackboneBindingStateV1 { generation: 1, uri: Some(uri.into()) };
    assert!(matches!(decide_document_backbone_binding_v1(&live, &first), DocumentBackboneBindingDecisionV1::Replay(_)));
    let replacement = DocumentBackboneBindingCommandV1 { binding_generation: 2, ..first.clone() };
    assert!(matches!(decide_document_backbone_binding_v1(&live, &replacement), DocumentBackboneBindingDecisionV1::Refuse(_)));
    let retire = DocumentBackboneBindingCommandV1 { operation: DocumentBackboneBindingOperationV1::Retire, ..first };
    assert!(matches!(decide_document_backbone_binding_v1(&live, &retire), DocumentBackboneBindingDecisionV1::Retire(_)));
    let retired = DocumentBackboneBindingStateV1 { generation: 1, uri: None };
    assert!(matches!(decide_document_backbone_binding_v1(&retired, &replacement), DocumentBackboneBindingDecisionV1::Bind(_)));
}

#[test]
fn document_backbone_binding_command_and_receipt_round_trip_exact_owner() {
    let command = DocumentBackboneBindingCommandV1 { operation: DocumentBackboneBindingOperationV1::Bind, instance_id: 7, binding_generation: 3, uri: "actor://v1:7:3:space-amap".into() };
    let encoded = command.encode().expect("canonical binding command encodes");
    assert_eq!(decode_document_backbone_binding_command_v1(&encoded).expect("command decodes"), Some(command.clone()));
    require_document_backbone_binding_receipt_v1(&DocumentBackboneBindingReceiptV1::bound(&command).encode(), &command).expect("exact bound receipt is accepted");
    let stale = DocumentBackboneBindingCommandV1 { binding_generation: 2, ..command.clone() };
    assert_eq!(require_document_backbone_binding_receipt_v1(&DocumentBackboneBindingReceiptV1::bound(&stale).encode(), &command), Err("plugin.document-backbone.receipt-owner".into()));
    assert_eq!(require_document_backbone_binding_receipt_v1(&DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.binding-live").encode(), &command), Err("plugin.document-backbone.binding-live".into()));
}

/// 🥇️ LAW: the control maps cross as the golden wire bytes of `🧫️fixtures/🔣️.json` `codec.control`, which the TypeScript
/// closed grammar pins too (`💻️os/🧪️tests/🧪️document-port-control-turn`): every command and receipt encodes to exactly its
/// row's bytes, and those bytes decode to the command, or are read as the receipt they are.
#[test]
fn document_backbone_control_maps_are_the_golden_wire_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("the binding fixture parses");
    let rows = fixture["codec"]["control"].as_array().expect("control rows");
    assert!(rows.len() >= 5, "the control rows cover both commands and all three receipts");
    for row in rows {
        let (id, value, golden) = (row["id"].as_str().expect("id"), &row["value"], row["hex"].as_str().expect("hex"));
        let bytes = (0..golden.len()).step_by(2).map(|index| u8::from_str_radix(&golden[index..index + 2], 16).expect("golden hex")).collect::<Vec<_>>();
        let operation = value["operation"].as_str().expect("operation");
        let owner = DocumentBackboneBindingCommandV1 {
            operation: if matches!(operation, "retire" | "retired") { DocumentBackboneBindingOperationV1::Retire } else { DocumentBackboneBindingOperationV1::Bind },
            instance_id: u32::try_from(value["instanceId"].as_u64().expect("instanceId")).expect("instance id fits"),
            binding_generation: value["bindingGeneration"].as_str().expect("bindingGeneration").parse().expect("generation"),
            uri: value["uri"].as_str().expect("uri").to_string(),
        };
        if value["schema"] == DOCUMENT_BACKBONE_BINDING_SCHEMA_V1 {
            assert_eq!(hex(&owner.encode().expect("the command encodes")), golden, "{id}: the command's bytes");
            assert_eq!(decode_document_backbone_binding_command_v1(&bytes), Ok(Some(owner)), "{id}: the golden bytes decode to the command");
            continue;
        }
        let code = value["code"].as_str().map(|code| &*Box::leak(code.to_string().into_boxed_str()));
        let receipt = match (operation, code) {
            ("bound", None) => DocumentBackboneBindingReceiptV1::bound(&owner),
            ("retired", None) => DocumentBackboneBindingReceiptV1::retired(&owner),
            ("refused", Some(code)) => DocumentBackboneBindingReceiptV1::refused(&owner, code),
            other => panic!("{id}: no receipt is {other:?}"),
        };
        assert_eq!(hex(&receipt.encode()), golden, "{id}: the receipt's bytes");
        assert_eq!(require_document_backbone_binding_receipt_v1(&bytes, &owner), code.map_or(Ok(()), |code| Err(code.to_string())), "{id}: the golden bytes read as the receipt");
    }
}

/// 🔤️ LAW (live fault F9): the control codec owns its canonical form. Whatever order the pack encoder writes a map's members
/// in, a command and a receipt go out with their members ordered by key bytes — the order the TypeScript closed grammar
/// demands — and a control whose members stand in another order is refused as noncanonical, never read.
#[test]
fn document_backbone_controls_are_ordered_by_key_bytes_whatever_the_pack_encoder_writes() {
    use semio_framework_value::DslValue;
    let command = DocumentBackboneBindingCommandV1 { operation: DocumentBackboneBindingOperationV1::Bind, instance_id: 7, binding_generation: 3, uri: "actor://v1:7:3:space-amap".into() };
    let keys = |bytes: &[u8]| match store::pack_rt::decode_wire_value(bytes).expect("the control decodes") {
        DslValue::Object(members) => members.into_iter().map(|(key, _)| key).collect::<Vec<_>>(),
        other => panic!("a control is a map, not {other:?}"),
    };
    assert_eq!(keys(&command.encode().expect("the command encodes")), ["bindingGeneration", "instanceId", "operation", "schema", "uri"]);
    assert_eq!(keys(&DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.binding-live").encode()), ["bindingGeneration", "code", "instanceId", "operation", "schema", "uri"]);
    let authored = |schema: &str, operation: &str| {
        store::pack_rt::encode_wire_value(&DslValue::Object(vec![
            ("schema".into(), DslValue::String(schema.into())),
            ("operation".into(), DslValue::String(operation.into())),
            ("instanceId".into(), DslValue::uint(7)),
            ("bindingGeneration".into(), DslValue::uint(3)),
            ("uri".into(), DslValue::String("actor://v1:7:3:space-amap".into())),
        ]))
    };
    let (authored_command, authored_receipt) = (authored(DOCUMENT_BACKBONE_BINDING_SCHEMA_V1, "bind"), authored(DOCUMENT_BACKBONE_BINDING_RECEIPT_SCHEMA_V1, "bound"));
    if keys(&authored_command) == ["schema", "operation", "instanceId", "bindingGeneration", "uri"] {
        assert_eq!(decode_document_backbone_binding_command_v1(&authored_command), Err("plugin.document-backbone.binding-noncanonical".into()));
        assert_eq!(require_document_backbone_binding_receipt_v1(&authored_receipt, &command), Err("plugin.document-backbone.receipt-noncanonical".into()));
    } else {
        assert_eq!(decode_document_backbone_binding_command_v1(&authored_command), Ok(Some(command.clone())));
        assert_eq!(require_document_backbone_binding_receipt_v1(&authored_receipt, &command), Ok(()));
    }
}
