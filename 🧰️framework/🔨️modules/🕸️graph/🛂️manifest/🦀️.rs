//! 📜️ Compile-time graph manifest kernel: schema, registry, and strict validation.

use semio_framework_value::{ValueKind, ValueType};

#[path = "🛬️type/🦀️.rs"]
mod type_binding;

pub use crate::manifest::Manifest as GraphManifest;

//#region ⚠️ Errors
// 🌉️ `value_type_from_value` below is a `#[value(deserialize_with = "...")]` hook (RUNTIME-
// DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02 Phase 2) — the direct successor of
// the old serde `deserialize_with = "deserialize_value_type"` hook (and its `GraphManifestError`/
// `parse_value_type_value` helpers), NOT dead code: `ValueType`'s own native `dsl_core::FromValue`
// impl decodes its internally-tagged `{"kind": "boolean"}` shape only, but every `*.manifest.json`
// fixture (embedded verbatim as `${PREFIX}_MANIFEST_JSON` by `🤖️generated/🦀️*.rs`) spells
// `valueType` as a bare string (`"boolean"`/`"text"`/...) or a `{"schema": "..."}` object — the
// same gap the serde hook used to bridge. Confirmed by `cargo test -p semio-framework-graph`:
// dropping this hook broke `nakagin_manifest_loads` et al. with `ValueError("nodeKinds.41.
// properties.0.valueType.kind")` before this fix landed. The encode direction needs no matching
// hook: `ValueType::to_value`'s native shape is self-consistent for `Manifest::to_value()`'s own
// round trip (nothing needs it to reproduce the fixture text byte-for-byte) — the old
// `serialize_value_type` hook was itself just a thin wrapper over the same native `to_value` call.
fn value_type_from_value(value: semio_framework_value::DslValue) -> Result<ValueType, semio_framework_value::ValueError> {
    if let Ok(value_type) = <ValueType as semio_framework_value::FromValue>::from_value(value.clone()) {
        return Ok(value_type);
    }
    match value {
        semio_framework_value::DslValue::String(s) => Ok(match s.as_str() {
            "boolean" | "bool" => ValueType::Boolean,
            "integer" | "int" => ValueType::Integer,
            "number" | "decimal" | "float" => ValueType::Decimal,
            "text" | "string" => ValueType::Text,
            "object" | "any" => ValueType::Any,
            _ => ValueType::Schema(s),
        }),
        semio_framework_value::DslValue::Object(entries) if entries.len() == 1 => match entries.first() {
            Some((key, semio_framework_value::DslValue::String(schema))) if key == "schema" => Ok(ValueType::Schema(schema.clone())),
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unsupported valueType object {:?}", semio_framework_value::DslValue::Object(entries)))),
        },
        other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unsupported valueType {other:?}"))),
    }
}
//#endregion ⚠️ Errors

// #region 🔖️Property
/// 📊️ Runtime property value for graph instances.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub enum PropertyValue {
    #[default]
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<PropertyValue>),
    Object(PropertyBag),
}

#[path = "♻️retirement/🦀️.rs"]
mod retirement;
#[path="🗂️properties/🦀️.rs"]
mod property_members;
pub use property_members::PropertyBag;
#[path="🚦️properties/🦀️.rs"]
mod property_control;

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

    pub fn as_object(&self) -> Option<&PropertyBag> {
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
fn property_value_to_dsl_value(value: &PropertyValue) -> semio_framework_value::DslValue {
    match value {
        PropertyValue::Null => semio_framework_value::DslValue::Null,
        PropertyValue::Bool(b) => semio_framework_value::DslValue::Bool(*b),
        PropertyValue::Number(n) => semio_framework_value::DslValue::float(*n),
        PropertyValue::String(s) => semio_framework_value::DslValue::String(s.clone()),
        PropertyValue::Array(items) => {
            // 🔀️ Plain sync recursion — no suspension point, so no `Box::pin` is needed.
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(property_value_to_dsl_value(item));
            }
            semio_framework_value::DslValue::Array(out)
        }
        PropertyValue::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (k, v) in map {
                out.push((k.clone(), property_value_to_dsl_value(v)));
            }
            semio_framework_value::DslValue::Object(out)
        }
    }
}

fn dsl_value_to_property_value(value: &semio_framework_value::DslValue) -> PropertyValue {
    match value {
        semio_framework_value::DslValue::Null => PropertyValue::Null,
        semio_framework_value::DslValue::Bool(b) => PropertyValue::Bool(*b),
        semio_framework_value::DslValue::Number(n) => PropertyValue::Number(n.as_f64()),
        semio_framework_value::DslValue::String(s) => PropertyValue::String(s.clone()),
        semio_framework_value::DslValue::Bytes(bytes) => PropertyValue::Array(bytes.iter().map(|byte| PropertyValue::Number(f64::from(*byte))).collect()),
        semio_framework_value::DslValue::Array(items) => {
            // 🔀️ Same rewrite as `property_value_to_dsl_value` above, mirrored.
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(dsl_value_to_property_value(item));
            }
            PropertyValue::Array(out)
        }
        semio_framework_value::DslValue::Object(entries) => {
            let mut out = PropertyBag::new();
            for (k, v) in entries {
                out.insert(k.clone(), dsl_value_to_property_value(v));
            }
            PropertyValue::Object(out)
        }
    }
}

impl semio_framework_dsl_record::DslField for PropertyValue {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Value)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{property_control::encode(self,control).map(semio_framework_dsl_record::FieldValue::Value)}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{match value{semio_framework_dsl_record::FieldValue::Value(value)=>property_control::decode(value,control),_=>Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"graph property requires intrinsic field value"))}}
    fn retire_decoded(self){property_control::retire(self)}
    // 🚫️async: E1 impl of externally-declared trait `dsl_core::DslField` — every method is
    // E4-tagged sync in the trait itself (fn-pointer transitivity through `Shape::Record`/`Table`),
    // see `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs`.
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(property_value_to_dsl_value(self))
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(dsl_value) => Ok(dsl_value_to_property_value(dsl_value)),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}
//#endregion 🔖️DslField

//#region 🔖️ToFromValue
/// 🌱️ `ToValue`/`FromValue` (the `DslValue`-tree pair `Mutation`/`MutationDiff` payloads need —
/// distinct from `DslField`/`FieldValue` above, the text/binary DSL grammar's own trait, see that
/// region's header note) for the identical reason `DslField` binds as `Shape::Value`: reuse the
/// same recursive `property_value_to_dsl_value`/`dsl_value_to_property_value` walk rather than a
/// second one. An untagged enum (this was `#[serde(untagged)]`, now serde-free — Phase 2,
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS 26/09/02) has no `#[derive(ToValue,
/// FromValue)]` equivalent — it needs exactly this kind of hand-written structural match, per the
/// fan-out playbook's "Not supported by the derive" list.
impl semio_framework_value::ToValue for PropertyValue {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{property_control::encode(self,control)}
    fn to_value(&self) -> semio_framework_value::DslValue {
        property_value_to_dsl_value(self)
    }
}

impl semio_framework_value::FromValue for PropertyValue {
    fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{property_control::decode(value,control)}
    fn default_value_controlled(control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{control.checkpoint()?;Ok(Self::Null)}
    fn retire_decoded(self){property_control::retire(self)}
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(dsl_value_to_property_value(&value))
    }
}
//#endregion 🔖️ToFromValue

/// 🏷️ Compile-time property kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PropertyKind {
    Data,
    Derived,
}

/// 📋️ Property definition on a kind.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct PropertyDef {
    pub name: String,
    pub kind: PropertyKind,
    // 🌉️ `deserialize_with` mirrors the old serde hook — see `value_type_from_value`'s own
    // docstring above (this crate's `⚠️ Errors` region) for why it is still needed. The plain
    // per-field `ToValue::to_value` stays for the encode direction (no `serialize_with`).
    #[value(default, deserialize_with = "value_type_from_value", deserialize_controlled_with = "type_binding::decode", retire_with = "type_binding::retire")]
    pub value_type: ValueType,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
}

/// 📏️ The supplied retirement byte grant cannot release the exact schema string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValueTypeRetirementError {
    pub required_bytes: usize,
    pub maximum_bytes: usize,
}

impl std::fmt::Display for ValueTypeRetirementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value type retirement requires {} bytes, granted {}", self.required_bytes, self.maximum_bytes)
    }
}

impl std::error::Error for ValueTypeRetirementError {}

impl PropertyDef {
    /// 🧹️ Detaches at most one exact nested value-type string or list box. A terminal
    /// definition has only the definitionally shallow `Any` tag left for its final drop.
    pub fn retire_value_type_step(&mut self, maximum_bytes: usize) -> Result<Option<String>, ValueTypeRetirementError> {
        if let ValueType::Schema(value) = &self.value_type {
            if value.len() > maximum_bytes {
                return Err(ValueTypeRetirementError { required_bytes: value.len(), maximum_bytes });
            }
        }
        match std::mem::take(&mut self.value_type) {
            ValueType::Schema(value) => Ok(Some(value)),
            ValueType::List(value) => {
                self.value_type = *value;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn value_type_terminal_is_empty(&self) -> bool {
        matches!(self.value_type, ValueType::Any)
    }
}


// #endregion 🔖️Property

// #region 🔖️Manifest
/// 🔌️ Port direction on a node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PortDirection {
    In,
    Out,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum PortModelAxis {
    #[default]
    Ported,
    Normal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum DirectednessAxis {
    #[default]
    Directed,
    Undirected,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ManifestAxes {
    #[value(default)]
    pub port_model: PortModelAxis,
    #[value(default)]
    pub directedness: DirectednessAxis,
}

/// 🏷️ Kind row in a manifest family.
///
/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` requires every field's type to
/// carry the same rename convention as a per-field attribute, but `presentation` is already the
/// schema-erased `DslValue` itself (arbitrary-shaped, no meaningful rename), so it is simpler to
/// spell the whole impl by hand than to special-case one field's attribute.
#[derive(Clone, Debug, PartialEq)]
pub struct KindDef {
    pub id: String,
    pub name: String,
    pub properties: Vec<PropertyDef>,
    pub ports: Vec<String>,
    pub direction: Option<PortDirection>,
    pub presentation: Option<semio_framework_value::DslValue>,
}

impl semio_framework_value::ToValue for KindDef {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries: Vec<(String, semio_framework_value::DslValue)> = vec![
            ("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)),
            ("name".to_string(), semio_framework_value::ToValue::to_value(&self.name)),
            ("properties".to_string(), semio_framework_value::ToValue::to_value(&self.properties)),
            ("ports".to_string(), semio_framework_value::ToValue::to_value(&self.ports)),
        ];
        if self.direction.is_some() {
            entries.push(("direction".to_string(), semio_framework_value::ToValue::to_value(&self.direction)));
        }
        if let Some(presentation) = &self.presentation {
            entries.push(("presentation".to_string(), presentation.clone()));
        }
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for KindDef {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for KindDef, found {value:?}")));
        };
        let mut id = None;
        let mut name = String::new();
        let mut properties = Vec::new();
        let mut ports = Vec::new();
        let mut direction = None;
        let mut presentation = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "name" => name = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("name"))?,
                "properties" => properties = <Vec<PropertyDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("properties"))?,
                "ports" => ports = <Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("ports"))?,
                "direction" => direction = Some(<PortDirection as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("direction"))?),
                "presentation" => presentation = Some(entry),
                _ => {}
            }
        }
        Ok(KindDef {
            id: id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "KindDef missing id"))?,
            name,
            properties,
            ports,
            direction,
            presentation,
        })
    }
}

impl KindDef {
    pub fn display_name(&self) -> &str {
        if self.name.is_empty() {
            &self.id
        } else {
            &self.name
        }
    }
}

/// 📜️ Compile-time schema for a graph.
///
/// 🌉️ Hand-written, not derived — same reason as `KindDef` above: `edge_tips`/`kind_compatibility`
/// are schema-erased `DslValue` trees, arbitrary-shaped, so a plain field-list derive buys nothing
/// over spelling the impl directly.
#[derive(Clone, Debug, PartialEq)]
pub struct Manifest {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub axes: ManifestAxes,
    pub node_kinds: Vec<KindDef>,
    pub edge_kinds: Vec<KindDef>,
    pub port_kinds: Vec<KindDef>,
    pub wire_kinds: Vec<KindDef>,
    pub layer_kinds: Vec<KindDef>,
    pub language_kinds: Vec<KindDef>,
    pub surface_kinds: Vec<KindDef>,
    pub window_kinds: Vec<KindDef>,
    pub file_node_kinds: Vec<KindDef>,
    pub descriptor_kinds: Vec<KindDef>,
    pub edge_tips: Vec<semio_framework_value::DslValue>,
    pub kind_compatibility: Vec<semio_framework_value::DslValue>,
}

impl semio_framework_value::ToValue for Manifest {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object([
            ("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)),
            ("id".to_string(), semio_framework_value::ToValue::to_value(&self.id)),
            ("name".to_string(), semio_framework_value::ToValue::to_value(&self.name)),
            ("axes".to_string(), semio_framework_value::ToValue::to_value(&self.axes)),
            ("nodeKinds".to_string(), semio_framework_value::ToValue::to_value(&self.node_kinds)),
            ("edgeKinds".to_string(), semio_framework_value::ToValue::to_value(&self.edge_kinds)),
            ("portKinds".to_string(), semio_framework_value::ToValue::to_value(&self.port_kinds)),
            ("wireKinds".to_string(), semio_framework_value::ToValue::to_value(&self.wire_kinds)),
            ("layerKinds".to_string(), semio_framework_value::ToValue::to_value(&self.layer_kinds)),
            ("languageKinds".to_string(), semio_framework_value::ToValue::to_value(&self.language_kinds)),
            ("surfaceKinds".to_string(), semio_framework_value::ToValue::to_value(&self.surface_kinds)),
            ("windowKinds".to_string(), semio_framework_value::ToValue::to_value(&self.window_kinds)),
            ("fileNodeKinds".to_string(), semio_framework_value::ToValue::to_value(&self.file_node_kinds)),
            ("descriptorKinds".to_string(), semio_framework_value::ToValue::to_value(&self.descriptor_kinds)),
            ("edgeTips".to_string(), semio_framework_value::DslValue::Array(self.edge_tips.clone())),
            ("kindCompatibility".to_string(), semio_framework_value::DslValue::Array(self.kind_compatibility.clone())),
        ])
    }
}

impl semio_framework_value::FromValue for Manifest {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Manifest, found {value:?}")));
        };
        let mut schema = None;
        let mut id = None;
        let mut name = String::new();
        let mut axes = ManifestAxes::default();
        let mut node_kinds = Vec::new();
        let mut edge_kinds = Vec::new();
        let mut port_kinds = Vec::new();
        let mut wire_kinds = Vec::new();
        let mut layer_kinds = Vec::new();
        let mut language_kinds = Vec::new();
        let mut surface_kinds = Vec::new();
        let mut window_kinds = Vec::new();
        let mut file_node_kinds = Vec::new();
        let mut descriptor_kinds = Vec::new();
        let mut edge_tips = Vec::new();
        let mut kind_compatibility = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                "id" => id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "name" => name = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("name"))?,
                "axes" => axes = <ManifestAxes as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("axes"))?,
                "nodeKinds" => node_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("nodeKinds"))?,
                "edgeKinds" => edge_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("edgeKinds"))?,
                "portKinds" => port_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("portKinds"))?,
                "wireKinds" => wire_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("wireKinds"))?,
                "layerKinds" => layer_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("layerKinds"))?,
                "languageKinds" => language_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("languageKinds"))?,
                "surfaceKinds" => surface_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("surfaceKinds"))?,
                "windowKinds" => window_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("windowKinds"))?,
                "fileNodeKinds" => file_node_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("fileNodeKinds"))?,
                "descriptorKinds" => descriptor_kinds = <Vec<KindDef> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("descriptorKinds"))?,
                "edgeTips" => {
                    let semio_framework_value::DslValue::Array(items) = entry else {
                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for edgeTips").under("edgeTips"));
                    };
                    edge_tips = items;
                }
                "kindCompatibility" => {
                    let semio_framework_value::DslValue::Array(items) = entry else {
                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for kindCompatibility").under("kindCompatibility"));
                    };
                    kind_compatibility = items;
                }
                _ => {}
            }
        }
        Ok(Manifest {
            schema: schema.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Manifest missing schema"))?,
            id: id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Manifest missing id"))?,
            name,
            axes,
            node_kinds,
            edge_kinds,
            port_kinds,
            wire_kinds,
            layer_kinds,
            language_kinds,
            surface_kinds,
            window_kinds,
            file_node_kinds,
            descriptor_kinds,
            edge_tips,
            kind_compatibility,
        })
    }
}

impl Manifest {
    pub fn node_kind(&self, id: &str) -> Option<&KindDef> {
        self.node_kinds.iter().find(|k| k.id == id)
    }

    pub fn edge_kind(&self, id: &str) -> Option<&KindDef> {
        self.edge_kinds.iter().find(|k| k.id == id)
    }

    pub fn port_kind(&self, id: &str) -> Option<&KindDef> {
        self.port_kinds.iter().find(|k| k.id == id)
    }

    pub fn wire_kind(&self, id: &str) -> Option<&KindDef> {
        self.wire_kinds.iter().find(|k| k.id == id)
    }

    pub fn layer_kind(&self, id: &str) -> Option<&KindDef> {
        self.layer_kinds.iter().find(|k| k.id == id)
    }

    pub fn language_kind(&self, id: &str) -> Option<&KindDef> {
        self.language_kinds.iter().find(|k| k.id == id)
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
    fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self { path: path.into(), message: message.into() }
    }
}

/// 🛡️ Validates runtime graph instances against a compile-time manifest.
#[derive(Clone, Debug)]
pub struct ManifestValidator<'a> {
    manifest: &'a Manifest,
}

impl<'a> ManifestValidator<'a> {
    pub fn new(manifest: &'a Manifest) -> Self {
        Self { manifest }
    }

    pub fn validate_node_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.node_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("nodes/{kind}"), format!("unknown node kind {kind:?}")))
        }
    }

    pub fn validate_edge_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.edge_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("edges/{kind}"), format!("unknown edge kind {kind:?}")))
        }
    }

    pub fn validate_port_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.port_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("ports/{kind}"), format!("unknown port kind {kind:?}")))
        }
    }

    pub fn validate_wire_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.wire_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("wires/{kind}"), format!("unknown wire kind {kind:?}")))
        }
    }

    pub fn validate_layer_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.layer_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("layers/{kind}"), format!("unknown layer kind {kind:?}")))
        }
    }

    pub fn validate_language_kind(&self, kind: &str) -> Result<(), ManifestValidationError> {
        if self.manifest.language_kind(kind).is_some() {
            Ok(())
        } else {
            Err(ManifestValidationError::new(format!("languages/{kind}"), format!("unknown language kind {kind:?}")))
        }
    }

    pub fn validate_node_properties(&self, kind: &str, properties: &PropertyBag) -> Result<(), ManifestValidationError> {
        let Some(def) = self.manifest.node_kind(kind) else {
            return self.validate_node_kind(kind);
        };
        self.validate_property_bag(&format!("nodes/{kind}/properties"), &def.properties, properties)
    }

    pub fn validate_edge_properties(&self, kind: &str, properties: &PropertyBag) -> Result<(), ManifestValidationError> {
        let Some(def) = self.manifest.edge_kind(kind) else {
            return self.validate_edge_kind(kind);
        };
        self.validate_property_bag(&format!("edges/{kind}/properties"), &def.properties, properties)
    }

    fn validate_property_bag(&self, path: &str, defs: &[PropertyDef], bag: &PropertyBag) -> Result<(), ManifestValidationError> {
        for def in defs {
            if def.kind == PropertyKind::Derived {
                continue;
            }
            let Some(value) = bag.get(&def.name) else {
                continue;
            };
            if !property_value_matches_type(value, &def.value_type) {
                return Err(ManifestValidationError::new(format!("{path}/{}", def.name), format!("property type mismatch for {}", def.value_type.id())));
            }
        }
        for key in bag.keys() {
            if !defs.iter().any(|d| d.name == *key) {
                return Err(ManifestValidationError::new(format!("{path}/{key}"), format!("unknown property {key:?}")));
            }
        }
        Ok(())
    }

    pub fn validate_trinity_graph(&self, nodes: &[TrinityNodeRef<'_>], edges: &[TrinityEdgeRef<'_>]) -> Result<(), ManifestValidationError> {
        for node in nodes {
            self.validate_node_kind(node.kind)?;
            self.validate_node_properties(node.kind, node.properties)?;
            for port in node.ports {
                self.validate_port_kind(port.kind)?;
                if let Some(node_def) = self.manifest.node_kind(node.kind) {
                    if !node_def.ports.is_empty() && !node_def.ports.iter().any(|p| p == port.kind) {
                        return Err(ManifestValidationError::new(format!("nodes/{}/ports/{}", node.id, port.kind), format!("port kind {} not declared on node kind {}", port.kind, node.kind)));
                    }
                }
            }
        }
        for edge in edges {
            self.validate_edge_kind(edge.kind)?;
            self.validate_edge_properties(edge.kind, edge.properties)?;
        }
        Ok(())
    }
}

fn property_value_matches_type(value: &PropertyValue, expected: &ValueType) -> bool {
    if matches!(expected, ValueType::Any) {
        return true;
    }
    match value {
        PropertyValue::Object(_) if matches!(expected, ValueType::Schema(_)) => true,
        _ => {
            expected.matches(property_value_kind(value))
        }
    }
}

fn property_value_kind(value: &PropertyValue) -> ValueKind<'_> {
    match value {
        PropertyValue::Null => ValueKind::Null,
        PropertyValue::Bool(_) => ValueKind::Boolean,
        PropertyValue::Number(_) => ValueKind::Decimal,
        PropertyValue::String(_) => ValueKind::Text,
        PropertyValue::Array(_) | PropertyValue::Object(_) => ValueKind::Null,
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

#[cfg(test)]
#[path = "🧪️tests/🏷️type/🦀️.rs"]
mod type_tests;

#[path = "🪆️binding/🦀️.rs"]
mod property_declaration_binding;

#[path="📡️delta/🦀️.rs"]
mod map_delta;
