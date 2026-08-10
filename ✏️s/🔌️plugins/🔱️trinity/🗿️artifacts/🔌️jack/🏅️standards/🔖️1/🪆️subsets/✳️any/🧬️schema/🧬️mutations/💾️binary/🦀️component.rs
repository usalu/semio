//! 📡️ `trinity.graph` artifact — state-patch wire codec for the raw document operation
//! (constitutional: spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::jack::dsl::{port_dsl_to_port, port_to_port_dsl, PortDsl};
use crate::artifacts::jack::schema::mutations::text::TrinityGraphMutation;
use crate::artifacts::jack::{EntityRef, JackSnapshot, PropertyValue};
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

/// ⚡️ Local mirror of `TrinityGraphMutation` for `protocol::OpText`/`OpBinary` — `entity: EntityRef`
/// and `ports`/`fixture` fields transitively carry foreign/tuple-variant shapes, so the real enum
/// can't derive `dsl::DslOps` directly. `fixture: JackSnapshot` binds through `JackSnapshot`'s own
/// hand-written `dsl::DslField` impl (in `🗣️dsl`) unchanged.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
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
        fixture: JackSnapshot,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
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
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for TrinityGraphOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn trinity_graph_operation_to_dsl(operation: &TrinityGraphMutation) -> TrinityGraphOperationDsl {
    match operation {
        TrinityGraphMutation::CreateNode { id, kind, name, x, y, width, height, ports } => {
            TrinityGraphOperationDsl::CreateNode { id: id.clone(), kind: kind.clone(), name: name.clone(), x: *x, y: *y, width: *width, height: *height, ports: ports.iter().map(port_to_port_dsl).collect() }
        }
        TrinityGraphMutation::DeleteNode { id } => TrinityGraphOperationDsl::DeleteNode { id: id.clone() },
        TrinityGraphMutation::CreateEdge { id, kind, source, target, properties } => TrinityGraphOperationDsl::CreateEdge { id: id.clone(), kind: kind.clone(), source: source.clone(), target: target.clone(), properties: properties.clone() },
        TrinityGraphMutation::DeleteEdge { id } => TrinityGraphOperationDsl::DeleteEdge { id: id.clone() },
        TrinityGraphMutation::Rename { id, name } => TrinityGraphOperationDsl::Rename { id: id.clone(), name: name.clone() },
        TrinityGraphMutation::Reposition { id, x, y } => TrinityGraphOperationDsl::Reposition { id: id.clone(), x: *x, y: *y },
        TrinityGraphMutation::SetDataProperty { entity, key, value } => TrinityGraphOperationDsl::SetDataProperty { entity: entity.into(), key: key.clone(), value: value.clone() },
        TrinityGraphMutation::ClearDataProperty { entity, key } => TrinityGraphOperationDsl::ClearDataProperty { entity: entity.into(), key: key.clone() },
        TrinityGraphMutation::SetFixture { fixture } => TrinityGraphOperationDsl::SetFixture { fixture: fixture.clone() },
    }
}

fn trinity_graph_operation_from_dsl(operation: TrinityGraphOperationDsl) -> TrinityGraphMutation {
    match operation {
        TrinityGraphOperationDsl::CreateNode { id, kind, name, x, y, width, height, ports } => {
            TrinityGraphMutation::CreateNode { id, kind, name, x, y, width, height, ports: ports.into_iter().map(port_dsl_to_port).collect() }
        }
        TrinityGraphOperationDsl::DeleteNode { id } => TrinityGraphMutation::DeleteNode { id },
        TrinityGraphOperationDsl::CreateEdge { id, kind, source, target, properties } => TrinityGraphMutation::CreateEdge { id, kind, source, target, properties },
        TrinityGraphOperationDsl::DeleteEdge { id } => TrinityGraphMutation::DeleteEdge { id },
        TrinityGraphOperationDsl::Rename { id, name } => TrinityGraphMutation::Rename { id, name },
        TrinityGraphOperationDsl::Reposition { id, x, y } => TrinityGraphMutation::Reposition { id, x, y },
        TrinityGraphOperationDsl::SetDataProperty { entity, key, value } => TrinityGraphMutation::SetDataProperty { entity: entity.into(), key, value },
        TrinityGraphOperationDsl::ClearDataProperty { entity, key } => TrinityGraphMutation::ClearDataProperty { entity: entity.into(), key },
        TrinityGraphOperationDsl::SetFixture { fixture } => TrinityGraphMutation::SetFixture { fixture },
    }
}
//#endregion 🔖️DslMirrors

//#region 🔖️OpText
/// ⚡️ One-line textual notation for [`TrinityGraphMutation`] (`protocol::OpText`), delegating to the
/// derive-generated `TrinityGraphOperationDsl` mirror.
impl OpText for TrinityGraphMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        <TrinityGraphOperationDsl as OpText>::parse_op(line).map(trinity_graph_operation_from_dsl)
    }

    fn print_op(&self) -> String {
        <TrinityGraphOperationDsl as OpText>::print_op(&trinity_graph_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `TrinityGraphOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl OpBinary for TrinityGraphMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        trinity_graph_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        TrinityGraphOperationDsl::decode_op(bytes).map(trinity_graph_operation_from_dsl)
    }
}
//#endregion 🔖️OpText

/// 📦️ Encodes a Trinity graph `Mutation` to its binary command form.
pub fn encode_op(operation: &TrinityGraphMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a Trinity graph `Mutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<TrinityGraphMutation, protocol::ProtocolError> {
    TrinityGraphMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::TRINITY_GRAPH_SCHEMA;

    #[test]
    fn rename_op_binary_round_trips_and_agrees_with_text() {
        let operation = TrinityGraphMutation::Rename { id: "node-1".into(), name: "Renamed".into() };
        ::store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn nakagin_document_text_round_trips_store_with_applied_operation() {
        let envelope = create_document_envelope_for_test();
        let mut doc_store = store::ArtifactStore::new(envelope);
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![TrinityGraphMutation::Rename { id: "node-1".into(), name: "Renamed".into() }], description: None }).ok();
        ::store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        ::store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    fn create_document_envelope_for_test() -> store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation> {
        create_document_envelope::<JackSnapshot, TrinityGraphMutation>(TRINITY_GRAPH_SCHEMA, "doc-text-test", crate::artifacts::jack::engine::empty_jack_document(), None)
    }
    use store::create_document_envelope;

    #[test]
    fn rename_op_text_round_trips() {
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::Rename { id: "node-1".into(), name: "Renamed".into() });
    }

    #[test]
    fn op_text_round_trip_create_node() {
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::CreateNode {
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
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::DeleteNode { id: "root".into() });
    }

    #[test]
    fn op_text_round_trip_create_edge() {
        let mut properties = crate::artifacts::jack::PropertyBag::new();
        properties.insert("u".into(), PropertyValue::Number(1.2));
        let mut nested = std::collections::BTreeMap::new();
        nested.insert("x".into(), PropertyValue::Number(0.0));
        properties.insert("meta".into(), PropertyValue::Object(nested));
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::CreateEdge {
            id: "e2".into(),
            kind: "Connection".into(),
            source: crate::artifacts::jack::port_key("root", "out-a"),
            target: crate::artifacts::jack::port_key("child", "in-a"),
            properties,
        });
    }

    #[test]
    fn op_text_round_trip_delete_edge() {
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::DeleteEdge { id: "e1".into() });
    }

    #[test]
    fn op_text_round_trip_rename() {
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::Rename { id: "root".into(), name: "renamed \"piece\"".into() });
    }

    #[test]
    fn op_text_round_trip_reposition() {
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::Reposition { id: "root".into(), x: 10.0, y: -20.5 });
    }

    #[test]
    fn op_text_round_trip_set_data_property() {
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::SetDataProperty { entity: EntityRef::Node("root".into()), key: "label".into(), value: PropertyValue::String("hi 'there'".into()) });
    }

    #[test]
    fn op_text_round_trip_clear_data_property() {
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::ClearDataProperty { entity: EntityRef::Edge("e1".into()), key: "u".into() });
    }

    #[test]
    fn op_text_round_trip_set_fixture() {
        ::store::os_store::test_support::assert_op_line_round_trip(&TrinityGraphMutation::SetFixture { fixture: crate::artifacts::jack::engine::empty_jack_document() });
    }

    #[test]
    fn parse_op_rejects_unknown_keyword() {
        let err = TrinityGraphMutation::parse_op("bogusOp x").expect_err("unknown op");
        assert!(err.message.contains("unknown mutation line"));
    }

    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{ArtifactId, Edit, SchemaId};
        use crate::artifacts::jack::schema::mutations::text::TrinityGraphStore;

        let mut store = TrinityGraphStore::new(create_document_envelope_for_test());
        crate::artifacts::jack::schema::mutations::text::dispatch_trinity_graph_mutations(&mut store, vec![TrinityGraphMutation::Rename { id: "node-1".into(), name: "Renamed".into() }]).unwrap_or(());
        if let Some(edit) = store.envelope().vcs.edits.last() {
            let edit: &Edit<TrinityGraphMutation> = edit;
            ::store::os_store::test_support::assert_command_envelope_round_trip::<JackSnapshot, TrinityGraphMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
        }
    }
}
//#endregion 🧪️Tests
