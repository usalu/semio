//! 🧰️ Rewriting test support: the Nakagin working graph and the fixture rule state the codec, application, operation and editor laws share.

use crate::standards::v1::subsets::any::schema::layout::RuleLayout;
use crate::standards::v1::subsets::any::schema::rule::{default_parameter_bindings, Assignment, Lhs, ParameterKind, ParameterSpec, Pattern, Rhs};
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_graph_manifest::PropertyValue;

const NAKAGIN_CHILD: &str = include_str!("../🧫️fixtures/🪆️child/🏢️initial/🪆️content/🔣️.json");

/// 🪆️ The authored Nakagin asset materializes its actual typed child.
pub fn nakagin_fixture() -> JackSnapshot {
    let child=semio_s_artifact_stdio_semio_graph::io::text::snapshot::decode_semio_graph_snapshot_json(NAKAGIN_CHILD).expect("declared complete Nakagin Semio child");
    let content=store::os_store::artifact_child::ArtifactChild::new("nakagin-jack-demo-content".into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:"nakagin-jack-demo-content".into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"graph".into()}});
    let mut graph=JackSnapshot{schema:JackSnapshot::SCHEMA.into(),name:"Nakagin Capsule Tower".into(),manifest_id:Some("nakagin".into()),manifest:semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::manifest::Manifest::nakagin_default(),camera:semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::Camera::default(),content,root_node_id:Some("7dc5b737-3b6b-4068-b315-b7bacc91c2e1".into()),query:semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::TRINITY_JACK_DEFAULT_QUERY.into()};
    semio_s_artifact_trinity_jack_semio_content::materialize_jack_snapshot(&mut graph.content,child);
    graph
}

pub fn fixture_rule_state() -> RewritingSnapshot {
    let lhs = Lhs { pattern: Pattern { left_var: "a".into(), left_kind: "Piece".into(), edge_var: Some("r".into()), edge_kind: Some("Connection".into()), right_var: Some("b".into()), right_kind: Some("Piece".into()) }, where_clause: Some("a.name = 'b'".into()) };
    let rhs = Rhs { set: vec![Assignment { var: "a".into(), prop: "label".into(), value: PropertyValue::String("$label".into()) }], parameters: vec![ParameterSpec { name: "label".into(), kind: ParameterKind::String, default: PropertyValue::String("nakagin-core".into()) }], ..Rhs::default() };
    let parameter_bindings = default_parameter_bindings(&rhs);
    RewritingSnapshot { working_graph: nakagin_fixture(), lhs, rhs, parameter_bindings, rule_layout: RuleLayout::new() }
}
