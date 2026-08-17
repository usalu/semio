//! 🔗️ 🔗️ S Studio app command — `connect-media-ports`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "connect-media-ports")]
pub struct ConnectMediaPorts {
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
}

pub fn handle(payload: &ConnectMediaPorts, doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    match crate::engine::space::negotiate_connect_or_notify(doc.snapshot, &payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id) {
        Ok(contract) => Ok(Emit::mutations(vec![crate::engine::space::connect_edge_operation(&payload.source_node_id, &payload.source_port_id, &payload.target_node_id, &payload.target_port_id, contract)])),
        Err(effect) => Ok(Emit::effect(effect)),
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::space::testkit::{apply_mutations, studio_emit};
    use crate::engine::space::SpaceCommand;
    use crate::demo_space_projection;
    use semio_framework_os::{register_artifact_descriptor, ArtifactKindSpec, MediaClass, MediaForm, MediaPortDirection, MediaType, MediaWireFormat};

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ConnectMediaPorts(ConnectMediaPorts {
            source_node_id: "n1".into(),
            source_port_id: "n1:out:out".into(),
            target_node_id: "n2".into(),
            target_port_id: "n2:in:in".into(),
        }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::DisconnectMediaEdge(crate::engine::space::commands::disconnect_media_edge::DisconnectMediaEdge { edge_id: "e1".into() }));
    }

    #[test]
    fn connect_media_ports_rejects_incompatible_types_via_notice() {
        register_artifact_descriptor(&ArtifactKindSpec {
            id: "test.contract.2d".into(),
            name: "Test 2D".into(),
            source_format: "test.2d".into(),
            component_kind: "test".into(),
            dimension: "2d".into(),
            media_capability: semio_framework_os::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            schema: "test.contract.2d.schema".into(),
            export_formats: vec!["svg".into()],
            import_formats: vec!["svg".into()],
            export_stdio_kinds: vec![],
            import_stdio_kinds: vec![],
        });
        register_artifact_descriptor(&ArtifactKindSpec {
            id: "test.contract.3d".into(),
            name: "Test 3D".into(),
            source_format: "test.3d".into(),
            component_kind: "test".into(),
            dimension: "3d".into(),
            media_capability: semio_framework_os::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            schema: "test.contract.3d.schema".into(),
            export_formats: vec!["glb".into()],
            import_formats: vec!["glb".into()],
            export_stdio_kinds: vec![],
            import_stdio_kinds: vec![],
        });
        let mut projection = demo_space_projection();
        let src_out = crate::engine::space::testkit::test_port("contract-src", "out", MediaPortDirection::Out, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, "test.contract.2d");
        let dst_in = crate::engine::space::testkit::test_port("contract-dst", "in", MediaPortDirection::In, MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, "test.contract.3d");
        projection.graph.nodes.push(crate::engine::space::testkit::test_node("contract-src", vec![], vec![src_out]));
        projection.graph.nodes.push(crate::engine::space::testkit::test_node("contract-dst", vec![dst_in], vec![]));
        let config = SpaceConfig::default();
        let emit = studio_emit(
            &projection,
            &config,
            &SpaceCommand::ConnectMediaPorts(ConnectMediaPorts { source_node_id: "contract-src".into(), source_port_id: "contract-src:out:out".into(), target_node_id: "contract-dst".into(), target_port_id: "contract-dst:in:in".into() }),
        )
        .expect("handle");
        assert!(emit.artifact_mutations.is_empty(), "an incompatible connect must not push WorkflowMutation::ConnectPorts");
        assert!(matches!(emit.effects.first(), Some(semio_framework_plugin::HostEffect::Notify { .. })), "an incompatible connect must surface a Notify effect instead");
    }

    #[test]
    fn connect_media_ports_negotiates_a_contract_for_compatible_types() {
        register_artifact_descriptor(&ArtifactKindSpec {
            id: "test.contract.doc-a".into(),
            name: "Test Doc A".into(),
            source_format: "test.doc".into(),
            component_kind: "test".into(),
            dimension: "data".into(),
            media_capability: semio_framework_os::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            schema: "test.contract.doc.schema".into(),
            export_formats: vec![],
            import_formats: vec![],
            export_stdio_kinds: vec![],
            import_stdio_kinds: vec![],
        });
        register_artifact_descriptor(&ArtifactKindSpec {
            id: "test.contract.doc-b".into(),
            name: "Test Doc B".into(),
            source_format: "test.doc".into(),
            component_kind: "test".into(),
            dimension: "data".into(),
            media_capability: semio_framework_os::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            schema: "test.contract.doc.schema".into(),
            export_formats: vec![],
            import_formats: vec![],
            export_stdio_kinds: vec![],
            import_stdio_kinds: vec![],
        });
        let mut projection = demo_space_projection();
        let src_out = crate::engine::space::testkit::test_port("contract-src-2", "out", MediaPortDirection::Out, MediaType { class: MediaClass::Data, form: MediaForm::Value }, "test.contract.doc-a");
        let dst_in = crate::engine::space::testkit::test_port("contract-dst-2", "in", MediaPortDirection::In, MediaType { class: MediaClass::Data, form: MediaForm::Value }, "test.contract.doc-b");
        projection.graph.nodes.push(crate::engine::space::testkit::test_node("contract-src-2", vec![], vec![src_out]));
        projection.graph.nodes.push(crate::engine::space::testkit::test_node("contract-dst-2", vec![dst_in], vec![]));
        let config = SpaceConfig::default();
        let emit = studio_emit(
            &projection,
            &config,
            &SpaceCommand::ConnectMediaPorts(ConnectMediaPorts { source_node_id: "contract-src-2".into(), source_port_id: "contract-src-2:out:out".into(), target_node_id: "contract-dst-2".into(), target_port_id: "contract-dst-2:in:in".into() }),
        )
        .expect("handle");
        let edge = emit
            .artifact_mutations
            .iter()
            .find_map(|operation| match operation {
                WorkflowMutation::ConnectPorts { edge } if edge.source_node_id == "contract-src-2" => Some(edge.clone()),
                _ => None,
            })
            .expect("a compatible connect must push WorkflowMutation::ConnectPorts with a negotiated contract");
        assert_eq!(edge.contract.kind_id, "test.contract.doc-b");
        assert_eq!(edge.contract.wire, MediaWireFormat::Document { schema: "test.contract.doc.schema".into() });
        assert!(edge.contract.conversion.is_none());
        let next = apply_mutations(&projection, &emit.artifact_mutations);
        assert!(semio_framework_os::validate_workflow(&next.graph).ok, "a freshly negotiated edge must pass validate_workflow's contract-consistency check");
    }
}
//#endregion 🧪️Tests
