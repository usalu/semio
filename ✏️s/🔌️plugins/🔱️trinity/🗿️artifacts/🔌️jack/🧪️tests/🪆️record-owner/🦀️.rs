//! 🔌️ Actual Jack containers compose Graph fields through the canonical Record identity.
use semio_framework_dsl_record::DslField;
use crate::{Edge,NodeKindDef,EdgeKindDef,PortKindDef,PortDirection,PropertyBag,PropertyDef,PropertyKind,Manifest,JackSnapshot};

fn property()->PropertyDef{PropertyDef{name:"label".into(),kind:PropertyKind::Data,value_type:semio_framework_value::ValueType::Text,expr:None}}
fn round_trip<T:DslField+PartialEq+std::fmt::Debug>(value:T){let encoded=<T as DslField>::to_value(&value);let decoded=<T as DslField>::from_value(&encoded).expect("canonical nested owner binding");assert_eq!(value,decoded);}

#[test]
fn edge_owns_the_canonical_graph_property_bag(){round_trip(Edge{id:"edge".into(),kind:"link".into(),source:"source".into(),target:"target".into(),properties:PropertyBag::new()});}

#[test]
fn node_kind_owns_canonical_graph_property_declarations(){round_trip(NodeKindDef{name:"node".into(),properties:vec![property()],port_kinds:vec!["in".into()]});}

#[test]
fn edge_kind_owns_canonical_graph_property_declarations(){round_trip(EdgeKindDef{name:"edge".into(),properties:vec![property()]});}

#[test]
fn port_kind_owns_canonical_graph_properties_and_direction(){round_trip(PortKindDef{name:"port".into(),direction:PortDirection::In,properties:vec![property()]});}

#[test]
fn jack_manifest_propagates_each_canonical_graph_container(){round_trip(Manifest{node_kinds:vec![NodeKindDef{name:"node".into(),properties:vec![property()],port_kinds:vec!["in".into()]}],edge_kinds:vec![EdgeKindDef{name:"edge".into(),properties:vec![property()]}],port_kinds:vec![PortKindDef{name:"in".into(),direction:PortDirection::In,properties:vec![property()]}]});}

#[test]
fn jack_snapshot_implements_the_canonical_record_identity(){fn bound<T:DslField>(){}bound::<JackSnapshot>();}
