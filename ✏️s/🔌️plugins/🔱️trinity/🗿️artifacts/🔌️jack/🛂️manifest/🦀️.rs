//! 📜️ Jack manifest projection owned by the Jack artifact.
use semio_framework_graph_manifest::{GraphManifest, PortDirection, PropertyDef};

/// 🔺️ Trinity-shaped manifest projection for jack/ram consumers.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct NodeKindDef {
    pub name: String,
    #[value(default)]
    pub properties: Vec<PropertyDef>,
    #[value(default, rename = "portKinds")]
    pub port_kinds: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct EdgeKindDef {
    pub name: String,
    #[value(default)]
    pub properties: Vec<PropertyDef>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
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
        Self::from_graph(&crate::standards::v1::subsets::any::schema::graph_manifest::nakagin::nakagin_manifest())
    }

    /// 🪪️ The one interned manifest of an id: documents reference it by `manifestId` and never own a copy. Built-in ids resolve lazily; custom ones come from [`Manifest::register`].
    pub fn shared(id: &str) -> Option<std::sync::Arc<Manifest>> {
        let registry = manifest_registry();
        if let Some(found) = registry.read().ok()?.get(id) {
            return Some(found.clone());
        }
        let built_in = match id {
            "nakagin" => Self::nakagin_default(),
            _ => return None,
        };
        Some(registry.write().ok()?.entry(id.to_string()).or_insert_with(|| std::sync::Arc::new(built_in)).clone())
    }

    /// 🧬️ Interns a derived manifest under the id of its own content, so equal manifests share one id and one allocation.
    pub fn intern(manifest: Manifest) -> String {
        let text = semio_framework_pack_json::to_json_string(&manifest);
        let hash = text.bytes().fold(0xcbf29ce484222325_u64, |hash, byte| (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3));
        let mut id = format!("content-{hash:016x}");
        loop {
            match Self::shared(&id) {
                Some(existing) if *existing == manifest => return id,
                Some(_) => id.push('+'),
                None => {
                    Self::register(id.clone(), manifest);
                    return id;
                }
            }
        }
    }

    /// 🧷️ Interns a custom manifest under an id, replacing any earlier registration of that id.
    pub fn register(id: impl Into<String>, manifest: Manifest) -> std::sync::Arc<Manifest> {
        let shared = std::sync::Arc::new(manifest);
        manifest_registry().write().expect("manifest registry").insert(id.into(), shared.clone());
        shared
    }
}

fn manifest_registry() -> &'static std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<Manifest>>> {
    static REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<Manifest>>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(Default::default)
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

