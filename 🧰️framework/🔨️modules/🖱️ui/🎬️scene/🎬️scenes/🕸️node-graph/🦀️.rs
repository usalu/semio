//! 🪢️ The node-graph scene payload with its port, node, edge, operator and hover records.
//!
//! Carved out of the former `semio-framework-ui-scene` scenes module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_ui_scene_doc::{
    deserialize_non_empty_interaction_domain_text, scene_lane_hash, value_decode, value_decode_default, value_decode_option, value_push, value_push_if_nonempty, value_push_option, SceneDoc, SceneLanePayload, SceneLaneRef,
};
use semio_framework_ui_viewport::Viewport2d;
use serde::{Deserialize, Serialize};

//#region 🔖️NodeGraphRecords
/// 🔌️ One port on a node-graph node: identity, display label and the value schemas it declares.
/// Direction is implied by whether the record lives in the owning node's `inputs` or `outputs` list,
/// not carried as a field. `value_type` is the comma-joined declared schema list a renderer
/// intersects to decide whether a drawn wire may land — undeclared means connectable.
///
/// @see `🧪️tests/🧫️fixtures/🔌️port-types/🔣️.json` — the law both node-graph renderers answer
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphPortRecord {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceKind")]
    pub artifact_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

/// 🕸️ One node-graph node: identity, label, layout rect, typed input/output ports.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphNodeRecord {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    pub inputs: Vec<NodeGraphPortRecord>,
    #[serde(default)]
    pub outputs: Vec<NodeGraphPortRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// 🕸️ One node-graph edge between two node/port endpoints.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphEdgeRecord {
    pub id: String,
    pub source_node_id: String,
    pub source_port_id: String,
    pub target_node_id: String,
    pub target_port_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// 🔎️ One spotlight/find result row for a node-graph surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphFindItem {
    pub id: String,
    pub label: String,
    pub category: String,
}

/// 🖱️ Hovered node id, if any.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphHover {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 🔌️ When set, the graph paints the hovered channel (port) on `node_id`, matching the
    /// `"{nodeId}@{portId}"` pick id (see `nodeGraphPickChannel`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port_id: Option<String>,
}

/// ➕️ Variadic input/output slot on an operator catalogue entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphOperatorVariadicRecord {
    pub slot_key: String,
    pub min: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
}

/// 🔌️ Declared operator channel (input or output).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphOperatorChannelRecord {
    pub code: String,
    pub abbreviation: String,
    pub name: String,
    pub full_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operators: Vec<String>,
    /// 🔤️ Value schemas this channel declares — what a wire on it carries, the vocabulary the
    /// port-compatibility oracle intersects. Empty is undeclared, which stays connectable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_types: Vec<String>,
    /// 📃️ Homogeneous collection element schemas, separate from the channel's wire schemas.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_types: Vec<String>,
    /// 🕳️ Opaque JSON-encoded default value — was `Option<serde_json::Value>`; this crate depends
    /// on nothing beyond `ui_contract`/`serde`, so the arbitrary-shaped default rides as a JSON
    /// string like every sibling `_json` field on these scene structs already does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub cardinality: String,
}

/// 🧠️ One operator catalogue entry offered to a flow-backed node-graph's spotlight/palette.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphOperatorRecord {
    pub id: String,
    pub extension: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub summary: String,
    #[serde(default)]
    pub inputs: Vec<NodeGraphOperatorChannelRecord>,
    #[serde(default)]
    pub outputs: Vec<NodeGraphOperatorChannelRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_input: Option<NodeGraphOperatorVariadicRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variadic_output: Option<NodeGraphOperatorVariadicRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<String>,
}

//#region 🔖️NodeGraphRecordsFromValue
/// 🌱️ Hand-written `FromValue` (decode direction only — nothing here needs `ToValue`, see the
/// Cargo.toml docstring on the `protocol` dependency for why this is hand-written rather than
/// `#[derive(ToValue, FromValue)]`) for the plugin-side JSON-round-trip shims (`space`'s
/// `json_array_to_node_graph_*` in `🕸️compiled-dag`/`🔄️workflow` window files) that decode a
/// first-party `pack::from_json_str::<Vec<NodeGraphXRecord>>(...)` off text a framework producer
/// still emits via `serde_json`. Mirrors each struct's own `#[serde(rename_all = "camelCase",
/// default, skip_serializing_if = ...)]` attributes field-by-field: a missing key decodes to the
/// same default a missing/omitted JSON key would under `serde`.
impl FromValue for NodeGraphPortRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        let opt = |key: &str| get(key).map(Option::<String>::from_value).transpose().map(Option::flatten);
        Ok(Self {
            id: String::from_value(field("id")?)?,
            label: opt("label")?,
            code: opt("code")?,
            abbreviation: opt("abbreviation")?,
            full_name: opt("fullName")?,
            artifact_kind: opt("resourceKind")?,
            value_type: opt("valueType")?,
        })
    }
}

impl FromValue for NodeGraphNodeRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        let opt = |key: &str| get(key).map(Option::<String>::from_value).transpose().map(Option::flatten);
        Ok(Self {
            id: String::from_value(field("id")?)?,
            label: opt("label")?,
            x: f64::from_value(field("x")?)?,
            y: f64::from_value(field("y")?)?,
            width: f64::from_value(field("width")?)?,
            height: f64::from_value(field("height")?)?,
            inputs: get("inputs").map(Vec::from_value).transpose()?.unwrap_or_default(),
            outputs: get("outputs").map(Vec::from_value).transpose()?.unwrap_or_default(),
            instance_id: opt("instanceId")?,
            plugin_id: opt("pluginId")?,
            app_id: opt("appId")?,
            icon: opt("icon")?,
        })
    }
}

impl FromValue for NodeGraphEdgeRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        let opt = |key: &str| get(key).map(Option::<String>::from_value).transpose().map(Option::flatten);
        Ok(Self {
            id: String::from_value(field("id")?)?,
            source_node_id: String::from_value(field("sourceNodeId")?)?,
            source_port_id: String::from_value(field("sourcePortId")?)?,
            target_node_id: String::from_value(field("targetNodeId")?)?,
            target_port_id: String::from_value(field("targetPortId")?)?,
            label: opt("label")?,
        })
    }
}

impl FromValue for NodeGraphFindItem {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        Ok(Self {
            id: String::from_value(field("id")?)?,
            label: String::from_value(field("label")?)?,
            category: String::from_value(field("category")?)?,
        })
    }
}

impl FromValue for NodeGraphOperatorVariadicRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        Ok(Self {
            slot_key: String::from_value(field("slotKey")?)?,
            min: usize::from_value(field("min")?)?,
            max: get("max").map(Option::<usize>::from_value).transpose()?.flatten(),
        })
    }
}

impl FromValue for NodeGraphOperatorChannelRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        let opt = |key: &str| get(key).map(Option::<String>::from_value).transpose().map(Option::flatten);
        Ok(Self {
            code: String::from_value(field("code")?)?,
            abbreviation: String::from_value(field("abbreviation")?)?,
            name: String::from_value(field("name")?)?,
            full_name: String::from_value(field("fullName")?)?,
            operators: get("operators").map(Vec::from_value).transpose()?.unwrap_or_default(),
            value_types: get("valueTypes").map(Vec::from_value).transpose()?.unwrap_or_default(),
            item_types: get("itemTypes").map(Vec::from_value).transpose()?.unwrap_or_default(),
            default_json: opt("defaultJson")?,
            label: opt("label")?,
            cardinality: get("cardinality").map(String::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

impl FromValue for NodeGraphOperatorRecord {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        Ok(Self {
            id: String::from_value(field("id")?)?,
            extension: String::from_value(field("extension")?)?,
            name: String::from_value(field("name")?)?,
            abbreviation: String::from_value(field("abbreviation")?)?,
            icon: String::from_value(field("icon")?)?,
            summary: String::from_value(field("summary")?)?,
            inputs: get("inputs").map(Vec::from_value).transpose()?.unwrap_or_default(),
            outputs: get("outputs").map(Vec::from_value).transpose()?.unwrap_or_default(),
            variadic_input: get("variadicInput").map(Option::<NodeGraphOperatorVariadicRecord>::from_value).transpose()?.flatten(),
            variadic_output: get("variadicOutput").map(Option::<NodeGraphOperatorVariadicRecord>::from_value).transpose()?.flatten(),
            group: get("group").map(Vec::from_value).transpose()?.unwrap_or_default(),
        })
    }
}
//#endregion 🔖️NodeGraphRecordsFromValue

//#region 🔖️NodeGraphRecordsToValue
/// 🌱️ Hand-written `ToValue` (the encode direction — see `🔖️NodeGraphRecordsFromValue` above for
/// why this is hand-written rather than derived) for the same record types, plus both directions
/// for `NodeGraphHover`, which had neither before this pass.
impl ToValue for NodeGraphPortRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push_option(&mut entries, "label", &self.label);
        value_push_option(&mut entries, "code", &self.code);
        value_push_option(&mut entries, "abbreviation", &self.abbreviation);
        value_push_option(&mut entries, "fullName", &self.full_name);
        value_push_option(&mut entries, "resourceKind", &self.artifact_kind);
        value_push_option(&mut entries, "valueType", &self.value_type);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphNodeRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push_option(&mut entries, "label", &self.label);
        value_push(&mut entries, "x", &self.x);
        value_push(&mut entries, "y", &self.y);
        value_push(&mut entries, "width", &self.width);
        value_push(&mut entries, "height", &self.height);
        value_push(&mut entries, "inputs", &self.inputs);
        value_push(&mut entries, "outputs", &self.outputs);
        value_push_option(&mut entries, "instanceId", &self.instance_id);
        value_push_option(&mut entries, "pluginId", &self.plugin_id);
        value_push_option(&mut entries, "appId", &self.app_id);
        value_push_option(&mut entries, "icon", &self.icon);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphEdgeRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "sourceNodeId", &self.source_node_id);
        value_push(&mut entries, "sourcePortId", &self.source_port_id);
        value_push(&mut entries, "targetNodeId", &self.target_node_id);
        value_push(&mut entries, "targetPortId", &self.target_port_id);
        value_push_option(&mut entries, "label", &self.label);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphFindItem {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "label", &self.label);
        value_push(&mut entries, "category", &self.category);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphHover {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push_option(&mut entries, "nodeId", &self.node_id);
        value_push_option(&mut entries, "portId", &self.port_id);
        DslValue::Object(entries)
    }
}

impl FromValue for NodeGraphHover {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self { node_id: value_decode_option(&entries, "nodeId")?, port_id: value_decode_option(&entries, "portId")? })
    }
}

impl ToValue for NodeGraphOperatorVariadicRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "slotKey", &self.slot_key);
        value_push(&mut entries, "min", &self.min);
        value_push_option(&mut entries, "max", &self.max);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphOperatorChannelRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "code", &self.code);
        value_push(&mut entries, "abbreviation", &self.abbreviation);
        value_push(&mut entries, "name", &self.name);
        value_push(&mut entries, "fullName", &self.full_name);
        value_push_if_nonempty(&mut entries, "operators", &self.operators);
        value_push_if_nonempty(&mut entries, "valueTypes", &self.value_types);
        value_push_if_nonempty(&mut entries, "itemTypes", &self.item_types);
        value_push_option(&mut entries, "defaultJson", &self.default_json);
        value_push_option(&mut entries, "label", &self.label);
        value_push(&mut entries, "cardinality", &self.cardinality);
        DslValue::Object(entries)
    }
}

impl ToValue for NodeGraphOperatorRecord {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "extension", &self.extension);
        value_push(&mut entries, "name", &self.name);
        value_push(&mut entries, "abbreviation", &self.abbreviation);
        value_push(&mut entries, "icon", &self.icon);
        value_push(&mut entries, "summary", &self.summary);
        value_push(&mut entries, "inputs", &self.inputs);
        value_push(&mut entries, "outputs", &self.outputs);
        value_push_option(&mut entries, "variadicInput", &self.variadic_input);
        value_push_option(&mut entries, "variadicOutput", &self.variadic_output);
        value_push_if_nonempty(&mut entries, "group", &self.group);
        DslValue::Object(entries)
    }
}
//#endregion 🔖️NodeGraphRecordsToValue
//#endregion 🔖️NodeGraphRecords

//#region 🔖️NodeGraphScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeGraphInteractionDomain {
    #[serde(deserialize_with = "deserialize_non_empty_interaction_domain_text")]
    pub id: String,
    #[serde(deserialize_with = "deserialize_non_empty_interaction_domain_text")]
    pub node_target_prefix: String,
    #[serde(deserialize_with = "deserialize_non_empty_interaction_domain_text")]
    pub edge_target_prefix: String,
    #[serde(deserialize_with = "deserialize_non_empty_interaction_domain_text")]
    pub handle_target_prefix: String,
}

impl ToValue for NodeGraphInteractionDomain {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "nodeTargetPrefix", &self.node_target_prefix);
        value_push(&mut entries, "edgeTargetPrefix", &self.edge_target_prefix);
        value_push(&mut entries, "handleTargetPrefix", &self.handle_target_prefix);
        DslValue::Object(entries)
    }
}

impl FromValue for NodeGraphInteractionDomain {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        let fields = ["id", "nodeTargetPrefix", "edgeTargetPrefix", "handleTargetPrefix"];
        if let Some((field, _)) = entries.iter().find(|(field, _)| !fields.contains(&field.as_str())) {
            return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("unknown field `{field}`")));
        }
        for field in fields {
            if entries.iter().filter(|(candidate, _)| candidate == field).count() > 1 {
                return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("duplicate field `{field}`")));
            }
        }
        let domain = Self {
            id: value_decode(&entries, "id")?,
            node_target_prefix: value_decode(&entries, "nodeTargetPrefix")?,
            edge_target_prefix: value_decode(&entries, "edgeTargetPrefix")?,
            handle_target_prefix: value_decode(&entries, "handleTargetPrefix")?,
        };
        if domain.id.is_empty() || domain.node_target_prefix.is_empty() || domain.edge_target_prefix.is_empty() || domain.handle_target_prefix.is_empty() {
            return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "node graph interaction domain fields must be non-empty"));
        }
        Ok(domain)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGraphScene {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
    #[serde(default)]
    pub nodes: Vec<NodeGraphNodeRecord>,
    #[serde(default)]
    pub edges: Vec<NodeGraphEdgeRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Viewport2d>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_domain: Option<NodeGraphInteractionDomain>,
    /// 🔌️ DOCUMENT-DERIVED operator records only — one per node this very graph holds, the way the OS
    /// workflow window derives a record per workflow node so the canvas can lay its ports out. The
    /// app's REGISTERED operator catalogue is NOT carried here: it is app-static, ~100 KB with the real
    /// `brep`/`math` sets installed against a 32 KiB fixed per-surface admission, and rides the reserved
    /// `framework.section.catalogue` surface once per app instance instead
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). A flow-backed scene leaves this empty and names
    /// its operators by kind id through `host_snapshot_json`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operators: Vec<NodeGraphOperatorRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub find_items: Vec<NodeGraphFindItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub selection: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover: Option<NodeGraphHover>,
    /// ✨️ Ids — nodes, edges, or `"{nodeId}@{portId}"` ports — the plugin wants highlighted, e.g.
    /// because they are transitively hovered from another window.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlighted: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_off_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lod_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computing_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_snapshot_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_peers_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_json: Option<String>,
}

impl SceneDoc for NodeGraphScene {
    const SCHEMA: &'static str = "node-graph@1";

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in NodeGraphSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().into(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        NodeGraphSceneLane::from_body_key(key).is_some_and(|lane| lane.put(self, payload))
    }
}

impl NodeGraphScene {
    /** 🕸️ Builds a node-graph scene with optional extensions unset. */
    pub fn base(nodes: Vec<NodeGraphNodeRecord>, edges: Vec<NodeGraphEdgeRecord>, viewport: Viewport2d) -> Self {
        Self {
            lanes: Vec::new(),
            nodes,
            edges,
            viewport: Some(viewport),
            editable: None,
            interaction_domain: None,
            operators: Vec::new(),
            find_items: Vec::new(),
            selection: Vec::new(),
            hover: None,
            highlighted: Vec::new(),
            preview_off_json: None,
            lod_json: None,
            controls_json: None,
            clusters_json: None,
            computing_json: None,
            status_json: None,
            capabilities_json: None,
            host_snapshot_json: None,
            presence_peers_json: None,
            eval_json: None,
        }
    }
}

impl ToValue for NodeGraphScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        value_push(&mut entries, "nodes", &self.nodes);
        value_push(&mut entries, "edges", &self.edges);
        value_push_option(&mut entries, "viewport", &self.viewport);
        value_push_option(&mut entries, "editable", &self.editable);
        value_push_option(&mut entries, "interactionDomain", &self.interaction_domain);
        value_push_if_nonempty(&mut entries, "operators", &self.operators);
        value_push_if_nonempty(&mut entries, "findItems", &self.find_items);
        value_push_if_nonempty(&mut entries, "selection", &self.selection);
        value_push_option(&mut entries, "hover", &self.hover);
        value_push_if_nonempty(&mut entries, "highlighted", &self.highlighted);
        value_push_option(&mut entries, "previewOffJson", &self.preview_off_json);
        value_push_option(&mut entries, "lodJson", &self.lod_json);
        value_push_option(&mut entries, "controlsJson", &self.controls_json);
        value_push_option(&mut entries, "clustersJson", &self.clusters_json);
        value_push_option(&mut entries, "computingJson", &self.computing_json);
        value_push_option(&mut entries, "statusJson", &self.status_json);
        value_push_option(&mut entries, "capabilitiesJson", &self.capabilities_json);
        value_push_option(&mut entries, "hostSnapshotJson", &self.host_snapshot_json);
        value_push_option(&mut entries, "presencePeersJson", &self.presence_peers_json);
        value_push_option(&mut entries, "evalJson", &self.eval_json);
        DslValue::Object(entries)
    }
}

impl FromValue for NodeGraphScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
            nodes: value_decode_default(&entries, "nodes", Vec::new)?,
            edges: value_decode_default(&entries, "edges", Vec::new)?,
            viewport: value_decode_option(&entries, "viewport")?,
            editable: value_decode_option(&entries, "editable")?,
            interaction_domain: value_decode_option(&entries, "interactionDomain")?,
            operators: value_decode_default(&entries, "operators", Vec::new)?,
            find_items: value_decode_default(&entries, "findItems", Vec::new)?,
            selection: value_decode_default(&entries, "selection", Vec::new)?,
            hover: value_decode_option(&entries, "hover")?,
            highlighted: value_decode_default(&entries, "highlighted", Vec::new)?,
            preview_off_json: value_decode_option(&entries, "previewOffJson")?,
            lod_json: value_decode_option(&entries, "lodJson")?,
            controls_json: value_decode_option(&entries, "controlsJson")?,
            clusters_json: value_decode_option(&entries, "clustersJson")?,
            computing_json: value_decode_option(&entries, "computingJson")?,
            status_json: value_decode_option(&entries, "statusJson")?,
            capabilities_json: value_decode_option(&entries, "capabilitiesJson")?,
            host_snapshot_json: value_decode_option(&entries, "hostSnapshotJson")?,
            presence_peers_json: value_decode_option(&entries, "presencePeersJson")?,
            eval_json: value_decode_option(&entries, "evalJson")?,
        })
    }
}

/// 🚚️ Document-sized node graph fields, pinned by the neutral node-graph lane fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeGraphSceneLane {
    Nodes,
    Edges,
    Operators,
    FindItems,
    Selection,
    Highlighted,
    PreviewOff,
    Lod,
    Controls,
    Clusters,
    Computing,
    Status,
    Capabilities,
    HostSnapshot,
    PresencePeers,
    Eval,
}

/// 🪧️ Node graph lane names.
pub const NODE_GRAPH_SCENE_LANE_NAMES: [&str; 16] = [
    "nodes",
    "edges",
    "operators",
    "findItems",
    "selection",
    "highlighted",
    "previewOff",
    "lod",
    "controls",
    "clusters",
    "computing",
    "status",
    "capabilities",
    "hostSnapshot",
    "presencePeers",
    "eval",
];

/// 🪧️ Node graph lane fields.
pub const NODE_GRAPH_SCENE_LANE_FIELDS: [&str; 16] = [
    "nodes",
    "edges",
    "operators",
    "findItems",
    "selection",
    "highlighted",
    "previewOffJson",
    "lodJson",
    "controlsJson",
    "clustersJson",
    "computingJson",
    "statusJson",
    "capabilitiesJson",
    "hostSnapshotJson",
    "presencePeersJson",
    "evalJson",
];

/// 🪧️ Node graph lane body keys.
pub const NODE_GRAPH_SCENE_LANE_BODY_KEYS: [&str; 16] = [
    "framework.scene.nodeGraph.nodes",
    "framework.scene.nodeGraph.edges",
    "framework.scene.nodeGraph.operators",
    "framework.scene.nodeGraph.findItems",
    "framework.scene.nodeGraph.selection",
    "framework.scene.nodeGraph.highlighted",
    "framework.scene.nodeGraph.previewOff",
    "framework.scene.nodeGraph.lod",
    "framework.scene.nodeGraph.controls",
    "framework.scene.nodeGraph.clusters",
    "framework.scene.nodeGraph.computing",
    "framework.scene.nodeGraph.status",
    "framework.scene.nodeGraph.capabilities",
    "framework.scene.nodeGraph.hostSnapshot",
    "framework.scene.nodeGraph.presencePeers",
    "framework.scene.nodeGraph.eval",
];

impl NodeGraphSceneLane {
    /// 📚️ Stable order of node graph payload carriers.
    pub const ALL: [Self; 16] = [
        Self::Nodes,
        Self::Edges,
        Self::Operators,
        Self::FindItems,
        Self::Selection,
        Self::Highlighted,
        Self::PreviewOff,
        Self::Lod,
        Self::Controls,
        Self::Clusters,
        Self::Computing,
        Self::Status,
        Self::Capabilities,
        Self::HostSnapshot,
        Self::PresencePeers,
        Self::Eval,
];

    /// 🏷️ Serialized lane name.
    pub fn name(self) -> &'static str { NODE_GRAPH_SCENE_LANE_NAMES[self as usize] }
    /// 🪧️ Serialized scene field.
    pub fn field(self) -> &'static str { NODE_GRAPH_SCENE_LANE_FIELDS[self as usize] }
    /// 🔑️ Reserved carrier key.
    pub fn body_key(self) -> &'static str { NODE_GRAPH_SCENE_LANE_BODY_KEYS[self as usize] }
    /// 🔎️ Resolves reserved carrier keys.
    pub fn from_body_key(key: &str) -> Option<Self> { Self::ALL.into_iter().find(|lane| lane.body_key() == key) }
    /// 📤️ Moves a document-sized payload out of its fixed scene spine.
    pub fn take(self, scene: &mut NodeGraphScene) -> Option<String> {
        match self {
            Self::Nodes => Some(semio_framework_pack_json::to_json_string(&std::mem::take(&mut scene.nodes))),
            Self::Edges => Some(semio_framework_pack_json::to_json_string(&std::mem::take(&mut scene.edges))),
            Self::Operators => Some(semio_framework_pack_json::to_json_string(&std::mem::take(&mut scene.operators))),
            Self::FindItems => Some(semio_framework_pack_json::to_json_string(&std::mem::take(&mut scene.find_items))),
            Self::Selection => Some(semio_framework_pack_json::to_json_string(&std::mem::take(&mut scene.selection))),
            Self::Highlighted => Some(semio_framework_pack_json::to_json_string(&std::mem::take(&mut scene.highlighted))),
            Self::PreviewOff => scene.preview_off_json.take(),
            Self::Lod => scene.lod_json.take(),
            Self::Controls => scene.controls_json.take(),
            Self::Clusters => scene.clusters_json.take(),
            Self::Computing => scene.computing_json.take(),
            Self::Status => scene.status_json.take(),
            Self::Capabilities => scene.capabilities_json.take(),
            Self::HostSnapshot => scene.host_snapshot_json.take(),
            Self::PresencePeers => scene.presence_peers_json.take(),
            Self::Eval => scene.eval_json.take(),
        }
    }

    /// 📥️ Restores complete carriers; invalid typed payloads leave the spine unchanged.
    pub fn put(self, scene: &mut NodeGraphScene, payload: String) -> bool {
        match self {
            Self::Nodes => { let Ok(value) = semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return false }; scene.nodes = value; },
            Self::Edges => { let Ok(value) = semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return false }; scene.edges = value; },
            Self::Operators => { let Ok(value) = semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return false }; scene.operators = value; },
            Self::FindItems => { let Ok(value) = semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return false }; scene.find_items = value; },
            Self::Selection => { let Ok(value) = semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return false }; scene.selection = value; },
            Self::Highlighted => { let Ok(value) = semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return false }; scene.highlighted = value; },
            Self::PreviewOff => scene.preview_off_json = Some(payload),
            Self::Lod => scene.lod_json = Some(payload),
            Self::Controls => scene.controls_json = Some(payload),
            Self::Clusters => scene.clusters_json = Some(payload),
            Self::Computing => scene.computing_json = Some(payload),
            Self::Status => scene.status_json = Some(payload),
            Self::Capabilities => scene.capabilities_json = Some(payload),
            Self::HostSnapshot => scene.host_snapshot_json = Some(payload),
            Self::PresencePeers => scene.presence_peers_json = Some(payload),
            Self::Eval => scene.eval_json = Some(payload),
        }
        true
    }
}
//#endregion 🔖️NodeGraphScene

#[cfg(test)]
#[path = "../../🧪️tests/🔬️node-graph-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️node-graph-value-round-trip/🦀️.rs"]
mod value_round_trip_tests;
