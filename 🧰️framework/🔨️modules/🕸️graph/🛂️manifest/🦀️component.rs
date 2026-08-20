//! 📜️ Compile-time graph manifest kernel: schema, registry, and strict validation.

use neural_engine::{Value, ValueType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod generated {
    include!("../🤖️generated/🦀️registry.rs");
}

pub use generated::*;

pub use crate::manifest::Manifest as GraphManifest;

//#region ⚠️ Errors
/// 🚨️ Compile-time `valueType` parsing failures.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum GraphManifestError {
    /// 📦️ A single-key `valueType` object didn't carry a recognized `schema` key.
    #[error("unsupported valueType object {0}")]
    UnsupportedValueTypeObject(serde_json::Value),
    /// 🔍️ A `valueType` value wasn't a string or a `{schema}` object.
    #[error("unsupported valueType {0}")]
    UnsupportedValueType(serde_json::Value),
}
//#endregion ⚠️ Errors

// #region 🔖️Property
/// 📊️ Runtime property value for graph instances.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    #[default]
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<PropertyValue>),
    Object(std::collections::BTreeMap<String, PropertyValue>),
}

impl PropertyValue {
    // 🚫️async: E1 pure accessor passed by name into `Option::and_then` (a sync fn-pointer slot) at
    // every call site in this crate; no consumer awaits it directly. See R9.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    // 🚫️async: E1 pure accessor, same reason as `as_str` above — see R9.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub async fn as_object(&self) -> Option<&std::collections::BTreeMap<String, PropertyValue>> {
        match self {
            Self::Object(m) => Some(m),
            _ => None,
        }
    }
}

//#region 🔖️DslField
// 🌱️ `PropertyValue` is structurally a dynamic JSON-equivalent literal (Null/Bool/Number/String/
// Array/Object), exactly like `dsl_core::DslValue` itself, so it binds as `Shape::Value` rather than
// through `#[derive(dsl_core::DslEnum)]`: the derive's tuple-variant codegen treats every single-field
// unnamed variant as a "newtype" delegating to the inner type's own `Shape::Record` (see
// `dsl_core::__rt::newtype_variant_spec`), which panics for a primitive/collection inner type such as
// `bool`/`f64`/`Vec<Self>`/`BTreeMap<String, Self>` — none of which are `Shape::Record`. Binding
// directly through `DslValue` (mirroring the engine's own `serde_json::Value` bridge) is both
// correct and the natural fit for an untyped recursive value type, and it needs no attributes on
// the Array/Object variants: recursion is carried by `DslValue` itself, not by field-level nesting.
async fn property_value_to_dsl_value(value: &PropertyValue) -> dsl_core::DslValue {
    match value {
        PropertyValue::Null => dsl_core::DslValue::Null,
        PropertyValue::Bool(b) => dsl_core::DslValue::Bool(*b),
        PropertyValue::Number(n) => dsl_core::DslValue::Number(*n),
        PropertyValue::String(s) => dsl_core::DslValue::String(s.clone()),
        PropertyValue::Array(items) => {
            // 🔀️ Rewritten from `.map(property_value_to_dsl_value).collect()` — the closure was
            // sync and this fn is self-recursive through Array/Object, so it also needs Box::pin
            // (R10 residue shapes #1 and #3).
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(Box::pin(property_value_to_dsl_value(item)).await);
            }
            dsl_core::DslValue::Array(out)
        }
        PropertyValue::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (k, v) in map {
                out.push((k.clone(), Box::pin(property_value_to_dsl_value(v)).await));
            }
            dsl_core::DslValue::Object(out)
        }
    }
}

async fn dsl_value_to_property_value(value: &dsl_core::DslValue) -> PropertyValue {
    match value {
        dsl_core::DslValue::Null => PropertyValue::Null,
        dsl_core::DslValue::Bool(b) => PropertyValue::Bool(*b),
        dsl_core::DslValue::Number(n) => PropertyValue::Number(*n),
        dsl_core::DslValue::String(s) => PropertyValue::String(s.clone()),
        dsl_core::DslValue::Array(items) => {
            // 🔀️ Same rewrite as `property_value_to_dsl_value` above, mirrored.
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(Box::pin(dsl_value_to_property_value(item)).await);
            }
            PropertyValue::Array(out)
        }
        dsl_core::DslValue::Object(entries) => {
            let mut out = std::collections::BTreeMap::new();
            for (k, v) in entries {
                out.insert(k.clone(), Box::pin(dsl_value_to_property_value(v)).await);
            }
            PropertyValue::Object(out)
        }
    }
}

impl dsl_core::DslField for PropertyValue {
    // 🚫️async: E1 impl of externally-declared trait `dsl_core::DslField` — `shape()` is E4-tagged
    // sync in the trait itself (fn-pointer transitivity through `Shape::Record`/`Table`), see
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`.
    fn shape() -> dsl_core::Shape {
        dsl_core::Shape::Value
    }

    async fn to_value(&self) -> dsl_core::FieldValue {
        dsl_core::FieldValue::Value(property_value_to_dsl_value(self).await)
    }

    async fn from_value(value: &dsl_core::FieldValue) -> Result<Self, String> {
        match value {
            dsl_core::FieldValue::Value(dsl_value) => Ok(dsl_value_to_property_value(dsl_value).await),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}
//#endregion 🔖️DslField

/// 🏷️ Compile-time property kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PropertyKind {
    Data,
    Derived,
}

// 🚫️async: E1 pure computation consumed by serde's derive-generated `Deserialize::deserialize`
// (a `deserialize_with` hook) — `Deserialize::deserialize` is an externally-declared trait method
// and is sync, so the hook it calls synchronously must stay sync too. See R9.
fn deserialize_value_type<'de, D>(deserializer: D) -> Result<ValueType, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = serde_json::Value::deserialize(deserializer)?;
    parse_value_type_value(&raw).map_err(serde::de::Error::custom)
}

// 🚫️async: E1 — `serialize_with` hook called synchronously from derive-generated `Serialize::serialize`. See R9.
fn serialize_value_type<S>(value: &ValueType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.serialize(serializer)
}

// 🚫️async: E1 pure computation, no I/O, whose only consumer (`deserialize_value_type` above) is
// itself sync-pinned by serde's external `Deserialize` trait. See R9.
fn parse_value_type_value(raw: &serde_json::Value) -> Result<ValueType, GraphManifestError> {
    if let Ok(vt) = serde_json::from_value::<ValueType>(raw.clone()) {
        return Ok(vt);
    }
    match raw {
        serde_json::Value::String(s) => Ok(match s.as_str() {
            "boolean" | "bool" => ValueType::Boolean,
            "integer" | "int" => ValueType::Integer,
            "number" | "decimal" | "float" => ValueType::Decimal,
            "text" | "string" => ValueType::Text,
            "object" | "any" => ValueType::Any,
            schema => ValueType::Schema(schema.into()),
        }),
        serde_json::Value::Object(map) if map.len() == 1 => {
            if let Some(schema) = map.get("schema").and_then(|v| v.as_str()) {
                return Ok(ValueType::Schema(schema.into()));
            }
            Err(GraphManifestError::UnsupportedValueTypeObject(raw.clone()))
        }
        other => Err(GraphManifestError::UnsupportedValueType(other.clone())),
    }
}

/// 📋️ Property definition on a kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDef {
    pub name: String,
    pub kind: PropertyKind,
    #[serde(default, deserialize_with = "deserialize_value_type", serialize_with = "serialize_value_type")]
    pub value_type: ValueType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}

pub type PropertyBag = std::collections::BTreeMap<String, PropertyValue>;

// #endregion 🔖️Property

// #region 🔖️Manifest
/// 🔌️ Port direction on a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PortDirection {
    In,
    Out,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum PortModelAxis {
    #[default]
    Ported,
    Normal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DirectednessAxis {
    #[default]
    Directed,
    Undirected,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ManifestAxes {
    #[serde(default)]
    pub port_model: PortModelAxis,
    #[serde(default)]
    pub directedness: DirectednessAxis,
}

/// 🏷️ Kind row in a manifest family.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KindDef {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub properties: Vec<PropertyDef>,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<PortDirection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation: Option<serde_json::Value>,
}

impl KindDef {
    pub async fn display_name(&self) -> &str {
        if self.name.is_empty() {
            &self.id
        } else {
            &self.name
        }
    }
}

/// 📜️ Compile-time schema for a graph.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub schema: String,
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub axes: ManifestAxes,
    #[serde(default)]
    pub node_kinds: Vec<KindDef>,
    #[serde(default)]
    pub edge_kinds: Vec<KindDef>,
    #[serde(default)]
    pub port_kinds: Vec<KindDef>,
    #[serde(default)]
    pub wire_kinds: Vec<KindDef>,
    #[serde(default)]
    pub layer_kinds: Vec<KindDef>,
    #[serde(default)]
    pub language_kinds: Vec<KindDef>,
    #[serde(default)]
    pub surface_kinds: Vec<KindDef>,
    #[serde(default)]
    pub window_kinds: Vec<KindDef>,
    #[serde(default)]
    pub file_node_kinds: Vec<KindDef>,
    #[serde(default)]
    pub descriptor_kinds: Vec<KindDef>,
    #[serde(default)]
    pub edge_tips: Vec<serde_json::Value>,
    #[serde(default)]
    pub kind_compatibility: Vec<serde_json::Value>,
}

impl Manifest {
    pub async fn node_kind(&self, id: &str) -> Option<&KindDef> {
        self.node_kinds.iter().find(|k| k.id == id)
    }

    pub async fn edge_kind(&self, id: &str) -> Option<&KindDef> {
        self.edge_kinds.iter().find(|k| k.id == id)
    }

    pub async fn port_kind(&self, id: &str) -> Option<&KindDef> {
        self.port_kinds.iter().find(|k| k.id == id)
    }

    pub async fn wire_kind(&self, id: &str) -> Option<&KindDef> {
        self.wire_kinds.iter().find(|k| k.id == id)
    }

    pub async fn layer_kind(&self, id: &str) -> Option<&KindDef> {
        self.layer_kinds.iter().find(|k| k.id == id)
    }

    pub async fn language_kind(&self, id: &str) -> Option<&KindDef> {
        self.language_kinds.iter().find(|k| k.id == id)
    }

    pub async fn to_trinity_manifest(&self) -> TrinityManifest {
        TrinityManifest {
            node_kinds: self.node_kinds.iter().map(|k| TrinityNodeKindDef { name: k.id.clone(), properties: k.properties.clone(), port_kinds: k.ports.clone() }).collect(),
            edge_kinds: self.edge_kinds.iter().map(|k| TrinityEdgeKindDef { name: k.id.clone(), properties: k.properties.clone() }).collect(),
            port_kinds: self
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
                    Some(TrinityPortKindDef { name: k.id.clone(), direction, properties: k.properties.clone() })
                })
                .collect(),
        }
    }
}

/// 🔺️ Trinity-shaped manifest projection for jack/ram consumers.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrinityManifest {
    #[serde(default)]
    pub node_kinds: Vec<TrinityNodeKindDef>,
    #[serde(default)]
    pub edge_kinds: Vec<TrinityEdgeKindDef>,
    #[serde(default)]
    pub port_kinds: Vec<TrinityPortKindDef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrinityNodeKindDef {
    pub name: String,
    #[serde(default)]
    pub properties: Vec<PropertyDef>,
    #[serde(default, rename = "portKinds")]
    pub port_kinds: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrinityEdgeKindDef {
    pub name: String,
    #[serde(default)]
    pub properties: Vec<PropertyDef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrinityPortKindDef {
    pub name: String,
    pub direction: PortDirection,
    #[serde(default)]
    pub properties: Vec<PropertyDef>,
}

impl TrinityManifest {
    pub async fn node_kind(&self, name: &str) -> Option<&TrinityNodeKindDef> {
        self.node_kinds.iter().find(|k| k.name == name)
    }

    pub async fn edge_kind(&self, name: &str) -> Option<&TrinityEdgeKindDef> {
        self.edge_kinds.iter().find(|k| k.name == name)
    }

    pub async fn port_kind(&self, name: &str) -> Option<&TrinityPortKindDef> {
        self.port_kinds.iter().find(|k| k.name == name)
    }

    /// 📜️ Nakagin capsule tower compile-time manifest.
    pub async fn nakagin_default() -> Self {
        nakagin::nakagin_manifest().to_trinity_manifest().await
    }
}

// #endregion 🔖️Manifest

// #region 🔖️Validator
/// 🛡️ Strict manifest validation errors.
#[derive(Clone, Debug, PartialEq)]
pub struct ManifestValidationError {
    pub path: String,
    pub message: String,
}

impl ManifestValidationError {
    async fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self { path: path.into(), message: message.into() }
    }
}

/// 🛡️ Validates runtime graph instances against a compile-time manifest.
#[derive(Clone, Debug)]
pub struct ManifestValidator<'a> {
    manifest: &'a Manifest,
}

impl<'a> ManifestValidator<'a> {
    pub async fn new(manifest: &'a Manifest) -> Self {
        Self { manifest }
    }

    pub async fn validate_node_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.node_kind(kind).await.is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("nodes/{kind}"), format!("unknown node kind {kind:?}")).await)
        }
    }

    pub async fn validate_edge_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.edge_kind(kind).await.is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("edges/{kind}"), format!("unknown edge kind {kind:?}")).await)
        }
    }

    pub async fn validate_port_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.port_kind(kind).await.is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("ports/{kind}"), format!("unknown port kind {kind:?}")).await)
        }
    }

    pub async fn validate_wire_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.wire_kind(kind).await.is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("wires/{kind}"), format!("unknown wire kind {kind:?}")).await)
        }
    }

    pub async fn validate_layer_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.layer_kind(kind).await.is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("layers/{kind}"), format!("unknown layer kind {kind:?}")).await)
        }
    }

    pub async fn validate_language_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.language_kind(kind).await.is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("languages/{kind}"), format!("unknown language kind {kind:?}")).await)
        }
    }

    pub async fn validate_node_properties(&self, kind: &str, properties: &PropertyBag) -> Result<(), ManifestValidationError> {
        let Some(def) = self.manifest.node_kind(kind).await else {
            return self.validate_node_kind(kind).await;
        };
        self.validate_property_bag(&format!("nodes/{kind}/properties"), &def.properties, properties).await
    }

    pub async fn validate_edge_properties(&self, kind: &str, properties: &PropertyBag) -> Result<(), ManifestValidationError> {
        let Some(def) = self.manifest.edge_kind(kind).await else {
            return self.validate_edge_kind(kind).await;
        };
        self.validate_property_bag(&format!("edges/{kind}/properties"), &def.properties, properties).await
    }

    async fn validate_property_bag(&self, path: &str, defs: &[PropertyDef], bag: &PropertyBag) -> Result<(), ManifestValidationError> {
        for def in defs {
            if def.kind == PropertyKind::Derived {
                continue;
            }
            let Some(value) = bag.get(&def.name) else {
                continue;
            };
            if !property_value_matches_type(value, &def.value_type).await {
                return Err(ManifestValidationError::new(format!("{path}/{}", def.name), format!("property type mismatch for {}", def.value_type.id())).await);
            }
        }
        for key in bag.keys() {
            if !defs.iter().any(|d| d.name == *key) {
                return Err(ManifestValidationError::new(format!("{path}/{key}"), format!("unknown property {key:?}")).await);
            }
        }
        Ok(())
    }

    pub async fn validate_trinity_graph(&self, nodes: &[TrinityNodeRef<'_>], edges: &[TrinityEdgeRef<'_>]) -> Result<(), ManifestValidationError> {
        for node in nodes {
            self.validate_node_kind(node.kind).await?;
            self.validate_node_properties(node.kind, node.properties).await?;
            for port in node.ports {
                self.validate_port_kind(port.kind).await?;
                if let Some(node_def) = self.manifest.node_kind(node.kind).await {
                    if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == port.kind) {
                        return Err(ManifestValidationError::new(format!("nodes/{}/ports/{}", node.id, port.kind), format!("port kind {} not declared on node kind {}", port.kind, node.kind)).await);
                    }
                }
            }
        }
        for edge in edges {
            self.validate_edge_kind(edge.kind).await?;
            self.validate_edge_properties(edge.kind, edge.properties).await?;
        }
        Ok(())
    }
}

async fn property_value_matches_type(value: &PropertyValue, expected: &ValueType) -> bool {
    if matches!(expected, ValueType::Any) {
        return true;
    }
    match value {
        PropertyValue::Object(_) if matches!(expected, ValueType::Schema(_)) => true,
        _ => {
            let neural = property_value_to_neural(value).await;
            expected.matches(&neural)
        }
    }
}

async fn property_value_to_neural(value: &PropertyValue) -> Value {
    match value {
        PropertyValue::Null => Value::null(),
        PropertyValue::Bool(b) => Value::Atom(neural_engine::Atom::Boolean(*b)),
        PropertyValue::Number(n) => Value::Atom(neural_engine::Atom::Decimal(*n)),
        PropertyValue::String(s) => Value::Atom(neural_engine::Atom::String(s.clone())),
        PropertyValue::Array(_) | PropertyValue::Object(_) => Value::Atom(neural_engine::Atom::Null),
    }
}

/// 🔌️ Trinity node reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityNodeRef<'a> {
    pub id: &'a str,
    pub kind: &'a str,
    pub properties: &'a PropertyBag,
    pub ports: &'a [TrinityPortRef<'a>],
}

/// 🔌️ Trinity port reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityPortRef<'a> {
    pub kind: &'a str,
}

/// 🔗️ Trinity edge reference for validation.
#[derive(Clone, Debug)]
pub struct TrinityEdgeRef<'a> {
    pub kind: &'a str,
    pub properties: &'a PropertyBag,
}

// #endregion 🔖️Validator

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
    // an `async fn` directly (std has no executor for it), so every async test body in this
    // module runs through this instead. Sound because this crate performs no real I/O: every
    // future here resolves on its first poll, so a single poll (never a spin-park loop) is
    // enough — panics loudly if that invariant is ever violated rather than hanging.
    fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn noop(_: *const ()) {}
        fn clone_raw(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
        let raw = RawWaker::new(std::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(raw) };
        let mut cx = Context::from_waker(&waker);
        let mut fut = Box::pin(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
        }
    }


    #[test]
    fn nakagin_manifest_loads() {
        block_on_test(async {
            let m = nakagin::nakagin_manifest();
            assert_eq!(m.id, "nakagin");
            assert!(m.node_kind("Piece").await.is_some());
            assert!(m.edge_kind("Connection").await.is_some());
        });
    }

    #[test]
    fn validator_rejects_unknown_node_kind() {
        block_on_test(async {
            let m = nakagin::nakagin_manifest();
            let v = ManifestValidator::new(&m);
            assert!(v.await.validate_node_kind("NoSuchNode").await.is_err());
        });
    }

    #[test]
    fn manifest_by_id_resolves() {
        block_on_test(async {
            let m = manifest_by_id("nakagin").expect("nakagin");
            assert!(m.node_kind("Balcony").await.is_some());
        });
    }

    #[test]
    fn property_value_dsl_field_round_trips_nested_array_and_object() {
        block_on_test(async {
            // 🌳️ Nested case: an Object containing an Array containing an Object — proves the
            // `dsl_core::DslField` bridge (via `dsl_core::DslValue`) recurses correctly at every depth, not just
            // for a flat value.
            let mut inner_obj = std::collections::BTreeMap::new();
            inner_obj.insert("flag".to_string(), PropertyValue::Bool(true));
            inner_obj.insert("label".to_string(), PropertyValue::String("leaf".to_string()));

            let array_of_objects = PropertyValue::Array(vec![PropertyValue::Number(1.0), PropertyValue::Object(inner_obj), PropertyValue::Null]);

            let mut root = std::collections::BTreeMap::new();
            root.insert("id".to_string(), PropertyValue::String("root".to_string()));
            root.insert("count".to_string(), PropertyValue::Number(3.0));
            root.insert("items".to_string(), array_of_objects);
            let value = PropertyValue::Object(root);

            let field_value = <PropertyValue as ::dsl_core::DslField>::to_value(&value).await;
            let round_tripped = <PropertyValue as ::dsl_core::DslField>::from_value(&field_value).await.expect("round trip must succeed");
            assert_eq!(round_tripped, value, "PropertyValue dsl_core::DslField round trip diverged for a nested Object/Array/Object value");
        });
    }
}
// #endregion 🔖️Tests
