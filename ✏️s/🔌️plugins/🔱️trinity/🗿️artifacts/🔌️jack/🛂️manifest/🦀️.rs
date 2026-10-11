//! 📜️ Jack manifest projection owned by the Jack artifact.
use crate::{GraphManifest, PortDirection, PropertyDef};

/// 🔺️ Trinity-shaped manifest projection for jack/ram consumers.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct Manifest {
    #[value(default)]
    pub node_kinds: Vec<NodeKindDef>,
    #[value(default)]
    pub edge_kinds: Vec<EdgeKindDef>,
    #[value(default)]
    pub port_kinds: Vec<PortKindDef>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct NodeKindDef {
    pub name: String,
    #[value(default)]
    pub properties: Vec<PropertyDef>,
    #[value(default, rename = "portKinds")]
    pub port_kinds: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct EdgeKindDef {
    pub name: String,
    #[value(default)]
    pub properties: Vec<PropertyDef>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct PortKindDef {
    pub name: String,
    pub direction: PortDirection,
    #[value(default)]
    pub properties: Vec<PropertyDef>,
}

impl Manifest {
    pub fn node_kind(&self, name: &str) -> Option<&NodeKindDef> {
        self.node_kinds.iter().find(|k| k.name == name)
    }

    pub fn edge_kind(&self, name: &str) -> Option<&EdgeKindDef> {
        self.edge_kinds.iter().find(|k| k.name == name)
    }

    pub fn port_kind(&self, name: &str) -> Option<&PortKindDef> {
        self.port_kinds.iter().find(|k| k.name == name)
    }

    /// 📜️ Nakagin capsule tower compile-time manifest.
    pub fn nakagin_default() -> Self {
        Self::from_graph(&crate::graph_manifest::nakagin::nakagin_manifest())
    }
}

impl Manifest {
    pub fn from_graph(graph: &GraphManifest) -> Self {
        Self {
            node_kinds: graph.node_kinds.iter().map(|k| NodeKindDef { name: k.id.clone(), properties: k.properties.clone(), port_kinds: k.ports.clone() }).collect(),
            edge_kinds: graph.edge_kinds.iter().map(|k| EdgeKindDef { name: k.id.clone(), properties: k.properties.clone() }).collect(),
            port_kinds: graph
                .port_kinds
                .iter()
                .filter_map(|k| {
                    let direction = k.direction.or_else(|| {
                        k.presentation.as_ref().and_then(|p| p.get("direction")).and_then(|d| match d.as_str()? {
                            "in" => Some(PortDirection::In),
                            "out" => Some(PortDirection::Out),
                            _ => None,
                        })
                    })?;
                    Some(PortKindDef { name: k.id.clone(), direction, properties: k.properties.clone() })
                })
                .collect(),
        }
    }
}
