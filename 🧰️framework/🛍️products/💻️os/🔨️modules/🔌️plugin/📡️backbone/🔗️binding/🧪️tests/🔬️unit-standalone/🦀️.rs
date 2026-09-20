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
    let zero_wire = DocumentBackboneBindingWireV1 { schema: DOCUMENT_BACKBONE_BINDING_SCHEMA_V1.into(), operation: "bind".into(), instance_id: 0, binding_generation: 1, uri: "actor://v1:0:3:space-amap".into() };
    let zero_payload = store::pack_rt::encode_wire_value(&zero_wire.to_value());
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
    let command = DocumentBackboneBindingCommandV1 {
        operation: DocumentBackboneBindingOperationV1::Bind,
        instance_id: 7,
        binding_generation: 3,
        uri: "actor://v1:7:3:space-amap".into(),
    };
    let encoded = command.encode().expect("canonical binding command encodes");
    assert_eq!(decode_document_backbone_binding_command_v1(&encoded).expect("command decodes"), Some(command.clone()));
    require_document_backbone_binding_receipt_v1(&DocumentBackboneBindingReceiptV1::bound(&command).encode(), &command).expect("exact bound receipt is accepted");
    let stale = DocumentBackboneBindingCommandV1 { binding_generation: 2, ..command.clone() };
    assert_eq!(
        require_document_backbone_binding_receipt_v1(&DocumentBackboneBindingReceiptV1::bound(&stale).encode(), &command),
        Err("plugin.document-backbone.receipt-owner".into())
    );
    assert_eq!(
        require_document_backbone_binding_receipt_v1(&DocumentBackboneBindingReceiptV1::refused(&command, "plugin.document-backbone.binding-live").encode(), &command),
        Err("plugin.document-backbone.binding-live".into())
    );
}
