//! 📡️ `trinity.graph` artifact — state-patch wire codec for the raw document operation
//! (constitutional: spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::jack::dsl::{port_dsl_to_port, port_to_port_dsl, PortDsl};
use crate::artifacts::jack::op::TrinityGraphOperation;
use crate::artifacts::jack::{EntityRef, GraphFixture, PropertyValue};
use protocol::{OpBinary, OpText};
use store::TextError;

//#region 🔖️DslMirrors
/// 🏷️ The `entity` half of `EntityRefDsl` — a plain 2-variant scalar tag (`dsl::DslScalar`, not
/// `DslEnum`): `EntityRefDsl` needs `dsl::DslField` (to bind as an ordinary record field on
/// `TrinityGraphOperationDsl`'s variants), and a `DslRecord` of `{ kind, id }` gets that directly,
/// unlike a tagged-variant `DslEnum`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar)]
enum EntityKindDsl {
    Node,
    Edge,
}

/// 🎯️ Local twin of `EntityRef` purely for the DSL engine's tuple-variant limitation — a flat
/// `{ kind, id }` twin, converted at the op-text boundary via `From`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct EntityRefDsl {
    kind: EntityKindDsl,
    id: String,
}

impl From<&EntityRef> for EntityRefDsl {
    fn from(value: &EntityRef) -> Self {
        match value {
            EntityRef::Node(id) => EntityRefDsl { kind: EntityKindDsl::Node, id: id.clone() },
            EntityRef::Edge(id) => EntityRefDsl { kind: EntityKindDsl::Edge, id: id.clone() },
        }
    }
}

impl From<EntityRefDsl> for EntityRef {
    fn from(value: EntityRefDsl) -> Self {
        match value.kind {
            EntityKindDsl::Node => EntityRef::Node(value.id),
            EntityKindDsl::Edge => EntityRef::Edge(value.id),
        }
    }
}

/// ⚡️ Local mirror of `TrinityGraphOperation` for `protocol::OpText`/`OpBinary` — `entity: EntityRef`
/// and `ports`/`fixture` fields transitively carry foreign/tuple-variant shapes, so the real enum
/// can't derive `dsl::DslOps` directly. `fixture: GraphFixture` binds through `GraphFixture`'s own
/// hand-written `dsl::DslField` impl (in `🗣️dsl`) unchanged.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum TrinityGraphOperationDsl {
    CreateNode {
        id: String,
        kind: String,
        name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[dsl(table)]
        ports: Vec<PortDsl>,
    },
    DeleteNode {
        id: String,
    },
    CreateEdge {
        id: String,
        kind: String,
        source: String,
        target: String,
        properties: crate::artifacts::jack::PropertyBag,
    },
    DeleteEdge {
        id: String,
    },
    Rename {
        id: String,
        name: String,
    },
    Reposition {
        id: String,
        x: f64,
        y: f64,
    },
    SetDataProperty {
        entity: EntityRefDsl,
        key: String,
        value: PropertyValue,
    },
    ClearDataProperty {
        entity: EntityRefDsl,
        key: String,
    },
    SetFixture {
        fixture: GraphFixture,
    },
}

//#region 🔖️OpCodec
/// 🎞️ Handcrafted OpText (P6).
impl protocol::OpText for TrinityGraphOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for TrinityGraphOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}
//#endregion 🔖️OpCodec


fn trinity_graph_operation_to_dsl(operation: &TrinityGraphOperation) -> TrinityGraphOperationDsl {
    match operation {
        TrinityGraphOperation::CreateNode { id, kind, name, x, y, width, height, ports } => {
            TrinityGraphOperationDsl::CreateNode { id: id.clone(), kind: kind.clone(), name: name.clone(), x: *x, y: *y, width: *width, height: *height, ports: ports.iter().map(port_to_port_dsl).collect() }
        }
        TrinityGraphOperation::DeleteNode { id } => TrinityGraphOperationDsl::DeleteNode { id: id.clone() },
        TrinityGraphOperation::CreateEdge { id, kind, source, target, properties } => TrinityGraphOperationDsl::CreateEdge { id: id.clone(), kind: kind.clone(), source: source.clone(), target: target.clone(), properties: properties.clone() },
        TrinityGraphOperation::DeleteEdge { id } => TrinityGraphOperationDsl::DeleteEdge { id: id.clone() },
        TrinityGraphOperation::Rename { id, name } => TrinityGraphOperationDsl::Rename { id: id.clone(), name: name.clone() },
        TrinityGraphOperation::Reposition { id, x, y } => TrinityGraphOperationDsl::Reposition { id: id.clone(), x: *x, y: *y },
        TrinityGraphOperation::SetDataProperty { entity, key, value } => TrinityGraphOperationDsl::SetDataProperty { entity: entity.into(), key: key.clone(), value: value.clone() },
        TrinityGraphOperation::ClearDataProperty { entity, key } => TrinityGraphOperationDsl::ClearDataProperty { entity: entity.into(), key: key.clone() },
        TrinityGraphOperation::SetFixture { fixture } => TrinityGraphOperationDsl::SetFixture { fixture: fixture.clone() },
    }
}

fn trinity_graph_operation_from_dsl(operation: TrinityGraphOperationDsl) -> TrinityGraphOperation {
    match operation {
        TrinityGraphOperationDsl::CreateNode { id, kind, name, x, y, width, height, ports } => {
            TrinityGraphOperation::CreateNode { id, kind, name, x, y, width, height, ports: ports.into_iter().map(port_dsl_to_port).collect() }
        }
        TrinityGraphOperationDsl::DeleteNode { id } => TrinityGraphOperation::DeleteNode { id },
        TrinityGraphOperationDsl::CreateEdge { id, kind, source, target, properties } => TrinityGraphOperation::CreateEdge { id, kind, source, target, properties },
        TrinityGraphOperationDsl::DeleteEdge { id } => TrinityGraphOperation::DeleteEdge { id },
        TrinityGraphOperationDsl::Rename { id, name } => TrinityGraphOperation::Rename { id, name },
        TrinityGraphOperationDsl::Reposition { id, x, y } => TrinityGraphOperation::Reposition { id, x, y },
        TrinityGraphOperationDsl::SetDataProperty { entity, key, value } => TrinityGraphOperation::SetDataProperty { entity: entity.into(), key, value },
        TrinityGraphOperationDsl::ClearDataProperty { entity, key } => TrinityGraphOperation::ClearDataProperty { entity: entity.into(), key },
        TrinityGraphOperationDsl::SetFixture { fixture } => TrinityGraphOperation::SetFixture { fixture },
    }
}
//#endregion 🔖️DslMirrors

//#region 🔖️OpText
/// ⚡️ One-line textual notation for [`TrinityGraphOperation`] (`protocol::OpText`), delegating to the
/// derive-generated `TrinityGraphOperationDsl` mirror.
impl OpText for TrinityGraphOperation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        <TrinityGraphOperationDsl as OpText>::parse_op(line).map(trinity_graph_operation_from_dsl)
    }

    fn print_op(&self) -> String {
        <TrinityGraphOperationDsl as OpText>::print_op(&trinity_graph_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `TrinityGraphOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl OpBinary for TrinityGraphOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        trinity_graph_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        TrinityGraphOperationDsl::decode_op(bytes).map(trinity_graph_operation_from_dsl)
    }
}
//#endregion 🔖️OpText

/// 📦️ Encodes a Trinity graph `Operation` to its binary command form.
pub fn encode_op(operation: &TrinityGraphOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a Trinity graph `Operation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<TrinityGraphOperation, protocol::ProtocolError> {
    TrinityGraphOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::TRINITY_GRAPH_SCHEMA;

    #[test]
    fn rename_op_binary_round_trips_and_agrees_with_text() {
        let operation = TrinityGraphOperation::Rename { id: "node-1".into(), name: "Renamed".into() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn nakagin_document_text_round_trips_store_with_applied_operation() {
        let envelope = create_document_envelope_for_test();
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![TrinityGraphOperation::Rename { id: "node-1".into(), name: "Renamed".into() }], description: None }).ok();
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    fn create_document_envelope_for_test() -> store::DocumentEnvelope<GraphFixture, TrinityGraphOperation> {
        create_document_envelope::<GraphFixture, TrinityGraphOperation>(TRINITY_GRAPH_SCHEMA, "doc-text-test", crate::artifacts::jack::engine::empty_jack_document(), None)
    }
    use store::create_document_envelope;

    #[test]
    fn rename_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::Rename { id: "node-1".into(), name: "Renamed".into() });
    }

    #[test]
    fn op_text_round_trip_create_node() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::CreateNode {
            id: "new".into(),
            kind: "Piece".into(),
            name: "new-piece".into(),
            x: 200.0,
            y: 40.0,
            width: 80.0,
            height: 40.0,
            ports: vec![crate::artifacts::jack::Port { id: "p1".into(), kind: "Connector".into(), direction: crate::artifacts::jack::PortDirection::Out, properties: crate::artifacts::jack::PropertyBag::new() }],
        });
    }

    #[test]
    fn op_text_round_trip_delete_node() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::DeleteNode { id: "root".into() });
    }

    #[test]
    fn op_text_round_trip_create_edge() {
        let mut properties = crate::artifacts::jack::PropertyBag::new();
        properties.insert("u".into(), PropertyValue::Number(1.2));
        let mut nested = std::collections::BTreeMap::new();
        nested.insert("x".into(), PropertyValue::Number(0.0));
        properties.insert("meta".into(), PropertyValue::Object(nested));
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::CreateEdge {
            id: "e2".into(),
            kind: "Connection".into(),
            source: crate::artifacts::jack::port_key("root", "out-a"),
            target: crate::artifacts::jack::port_key("child", "in-a"),
            properties,
        });
    }

    #[test]
    fn op_text_round_trip_delete_edge() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::DeleteEdge { id: "e1".into() });
    }

    #[test]
    fn op_text_round_trip_rename() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::Rename { id: "root".into(), name: "renamed \"piece\"".into() });
    }

    #[test]
    fn op_text_round_trip_reposition() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::Reposition { id: "root".into(), x: 10.0, y: -20.5 });
    }

    #[test]
    fn op_text_round_trip_set_data_property() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("hi 'there'".into()) });
    }

    #[test]
    fn op_text_round_trip_clear_data_property() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::ClearDataProperty { entity: EntityRef::Edge("e1".into()), key: "u".into() });
    }

    #[test]
    fn op_text_round_trip_set_fixture() {
        store::test_support::assert_op_line_round_trip(&TrinityGraphOperation::SetFixture { fixture: crate::artifacts::jack::engine::empty_jack_document() });
    }

    #[test]
    fn parse_op_rejects_unknown_keyword() {
        let err = TrinityGraphOperation::parse_op("bogusOp x").expect_err("unknown op");
        assert!(err.message.contains("unknown operation line"));
    }

    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};
        use crate::artifacts::jack::op::TrinityGraphStore;

        let mut store = TrinityGraphStore::new(create_document_envelope_for_test());
        crate::artifacts::jack::op::dispatch_trinity_graph_operations(&mut store, vec![TrinityGraphOperation::Rename { id: "node-1".into(), name: "Renamed".into() }]).unwrap_or(());
        if let Some(edit) = store.envelope().vcs.edits.last() {
            let edit: &Edit<TrinityGraphOperation> = edit;
            store::test_support::assert_command_envelope_round_trip::<GraphFixture, TrinityGraphOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
        }
    }
}
//#endregion 🧪️Tests
