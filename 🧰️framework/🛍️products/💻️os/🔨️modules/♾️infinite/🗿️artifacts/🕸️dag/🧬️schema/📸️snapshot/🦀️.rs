//! 🕸️ Persisted DAG nodes, ports, document fixtures, and presentation values.
use crate::vcs::*;
use ::graph::manifest::PropertyBag;
use dsl::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeSet;

// #region 🔖️IoNode
const EMPTY_PORTS: &[IoPortSpec] = &[];

/// @emoji 🔤️ Converts spaced or dashed labels into PascalCase display text.
pub fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}

/// @emoji 🏷️ Normalizes node display name and abbreviation to PascalCase.
pub fn normalize_node_display(name: &str, abbreviation: &str) -> (String, String) {
    (to_pascal_case(name), to_pascal_case(abbreviation))
}

pub fn default_node_width() -> f64 {
    72.0
}

pub fn default_node_height() -> f64 {
    DAG_CHANNEL_ROW_HEIGHT
}

/// 📏️ Fixed height of one input or output channel row on computation nodes.
pub const DAG_CHANNEL_ROW_HEIGHT: f64 = ui_styling::metrics::dag::CHANNEL_ROW_HEIGHT;

/// 🔌️ Visual shape of a port handle cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum PortShape {
    #[default]
    Semicircle,
    Triangle,
}

/// 📐️ Edge routing style between port handles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum EdgeRouteStyle {
    #[default]
    Bezier,
    SharpSz,
}

/// 🪝️ Named horizontal port on a DAG node edge.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct IoPortSpec {
    pub id: String,
    pub label: String,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub code: String,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub abbreviation: String,
    #[value(rename = "fullName", default, skip_serializing_if = "String::is_empty")]
    pub full_name: String,
    #[value(rename = "type", skip_serializing_if = "Option::is_none")]
    #[dsl(key = "type")]
    pub value_type: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub default: Option<DslValue>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub value: Option<DslValue>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[value(rename = "resourceKind", skip_serializing_if = "Option::is_none")]
    pub artifact_kind: Option<String>,
    #[value(default = "default_port_cardinality")]
    pub cardinality: String,
    #[value(default)]
    pub shape: PortShape,
    #[value(default = "default_port_visible")]
    pub visible: bool,
    #[value(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
}

fn default_port_visible() -> bool {
    true
}

fn default_port_cardinality() -> String {
    "!".into()
}

impl Default for IoPortSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            code: String::new(),
            abbreviation: String::new(),
            full_name: String::new(),
            value_type: None,
            default: None,
            value: None,
            connected: None,
            artifact_kind: None,
            cardinality: default_port_cardinality(),
            shape: PortShape::default(),
            visible: true,
            resolved: None,
        }
    }
}

impl IoPortSpec {
    pub fn named(code: impl Into<String>, abbreviation: impl Into<String>, id: impl Into<String>, full_name: impl Into<String>) -> Self {
        let id = id.into();
        let abbreviation = abbreviation.into();
        Self { code: code.into(), abbreviation: abbreviation.clone(), label: abbreviation, id, full_name: full_name.into(), cardinality: default_port_cardinality(), shape: PortShape::default(), ..Default::default() }
    }

    pub fn simple(id: impl Into<String>, label: impl Into<String>) -> Self {
        let id = id.into();
        let label = label.into();
        let code = if id.len() <= 2 { id.to_uppercase() } else { id.chars().take(2).collect::<String>().to_uppercase() };
        let abbreviation = if label.len() <= 3 { label.clone() } else { label.chars().take(3).collect() };
        Self { id, label: label.clone(), code, abbreviation, full_name: label, ..Default::default() }
    }

    pub fn display_code(&self) -> &str {
        if !self.code.is_empty() {
            return self.code.as_str();
        }
        self.label.as_str()
    }

    pub fn label_with_cardinality(&self, lod: DagDrawLod) -> String {
        let cardinality = match self.resolved {
            Some(false) => "?",
            _ => self.cardinality.as_str(),
        };
        let label = self.display_label(lod).trim();
        if label.is_empty() {
            return cardinality.to_string();
        }
        format!("{cardinality} {label}")
    }
}

/// 🖼️ Screen media payload for output nodes.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DagMedia {
    pub kind: DagMediaKind,
    pub src: String,
}

/// 🎬️ Screen media kind discriminator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum DagMediaKind {
    Image,
    Svg,
    Pdf,
    Video,
}
// #endregion 🔖️Media

/// 👁️ Typed preview payload rendered inside a preview node.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslEnum)]
#[value(rename_all = "camelCase", tag = "variant")]
pub enum DagPreviewContent {
    #[default]
    Empty,
    Scalar {
        text: String,
    },
    Image {
        src: String,
    },
    Tree {
        json: DslValue,
    },
}

/// 🧩️ Tagged node kind: computation, slider, select, or screen.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DagNodeKind {
    Computation {
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
        #[value(default, rename = "variadic_inputs")]
        variadic_inputs: bool,
        #[value(default, rename = "variadic_outputs")]
        variadic_outputs: bool,
    },
    Slider {
        min: f64,
        max: f64,
        step: f64,
        value: f64,
        output: IoPortSpec,
    },
    Select {
        options: Vec<String>,
        #[value(default)]
        selected: u64,
        output: IoPortSpec,
    },
    Screen {
        #[value(default)]
        media: Option<DagMedia>,
        input: IoPortSpec,
    },
    Note {
        text: String,
        output: IoPortSpec,
    },
    Image {
        #[value(default)]
        src: String,
        output: IoPortSpec,
    },
    Preview {
        #[value(default)]
        content: DagPreviewContent,
        #[value(default)]
        expanded: BTreeSet<String>,
        input: IoPortSpec,
    },
    Action {
        label: String,
        input: IoPortSpec,
    },
    Export {
        label: String,
        format: String,
        input: IoPortSpec,
    },
    Cluster {
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
    },
    AppInstance {
        #[value(rename = "instanceId")]
        instance_id: String,
        #[value(rename = "pluginId")]
        plugin_id: String,
        #[value(rename = "appId")]
        app_id: String,
        #[value(rename = "appIcon", default)]
        icon: String,
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
    },
}

/// 🏷️ Serialized `kind` tag for a {@link DagNodeKind} variant.
pub fn dag_node_kind_tag(kind: &DagNodeKind) -> &'static str {
    match kind {
        DagNodeKind::Computation { .. } => "computation",
        DagNodeKind::Slider { .. } => "slider",
        DagNodeKind::Select { .. } => "select",
        DagNodeKind::Screen { .. } => "screen",
        DagNodeKind::Note { .. } => "note",
        DagNodeKind::Image { .. } => "image",
        DagNodeKind::Preview { .. } => "preview",
        DagNodeKind::Action { .. } => "action",
        DagNodeKind::Export { .. } => "export",
        DagNodeKind::Cluster { .. } => "cluster",
        DagNodeKind::AppInstance { .. } => "appInstance",
    }
}

/// 📦️ DAG node with shared layout fields and a tagged kind.
// 🔀️ `ToValue`/`FromValue` are HAND-WRITTEN below, not derived: `kind` is `#[serde(flatten)]` and the
// derive has no `flatten`, so only a hand-written impl reproduces serde's shape.
#[derive(Clone, Debug, PartialEq)]
pub struct DagNodeSpec {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
    pub icon: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub operator_kind: Option<String>,
    pub properties: PropertyBag,
    // 🔀️ serde flattens `kind` into the parent object; `#[derive(ToValue)]` has no `flatten`, so the
    // first-party encoding nests it under a `kind` key instead. Only the serde form is on a wire
    // today — the value form exists to satisfy `ArtifactStore`'s bounds.
    pub kind: DagNodeKind,
}

impl Default for DagNodeSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            abbreviation: String::new(),
            icon: String::new(),
            x: 0.0,
            y: 0.0,
            width: default_node_width(),
            height: default_node_height(),
            operator_kind: None,
            properties: PropertyBag::new(),
            kind: DagNodeKind::Computation { inputs: vec![], outputs: vec![], variadic_inputs: false, variadic_outputs: false },
        }
    }
}

/// 🔀️ Hand-written, not derived: `kind` is `#[serde(flatten)]`-merged onto this struct's own object,
/// and `flatten` is deliberately unsupported by `#[derive(ToValue, FromValue)]` (see that derive's
/// module doc's "Deliberately NOT supported" list). Mirrors `DagNodeKind`'s own internally-tagged
/// shape (a flat object carrying `kind` + the matched variant's own fields) by merging its entries
/// directly into this struct's own entries on encode, and by handing the FULL decoded entries back to
/// `DagNodeKind::from_value` on decode — its internally-tagged decoder already reads only the keys it
/// needs from that object and ignores the rest, exactly like serde's own flatten.
impl ::semio_framework_os_kernel::ToValue for DagNodeSpec {
    fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
        let mut entries: Vec<(String, ::semio_framework_os_kernel::DslValue)> = vec![
            ("id".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.id)),
            ("name".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.name)),
            ("abbreviation".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.abbreviation)),
            ("icon".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.icon)),
            ("x".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.x)),
            ("y".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.y)),
            ("width".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.width)),
            ("height".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.height)),
        ];
        if let Some(operator_kind) = &self.operator_kind {
            entries.push(("operatorKind".to_string(), ::semio_framework_os_kernel::ToValue::to_value(operator_kind)));
        }
        entries.push(("properties".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.properties)));
        match ::semio_framework_os_kernel::ToValue::to_value(&self.kind) {
            ::semio_framework_os_kernel::DslValue::Object(kind_entries) => entries.extend(kind_entries),
            other => entries.push(("kind".to_string(), other)),
        }
        ::semio_framework_os_kernel::DslValue::Object(entries)
    }
}
impl ::semio_framework_os_kernel::FromValue for DagNodeSpec {
    fn from_value(value: ::semio_framework_os_kernel::DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> {
        let entries = ::semio_framework_os_kernel::DslValue::into_object(value)?;
        let find = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let id = match find("id") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("id"))?,
            None => return Err(::semio_framework_os_kernel::ValueError::new("missing field `id`")),
        };
        let name = match find("name") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("name"))?,
            None => return Err(::semio_framework_os_kernel::ValueError::new("missing field `name`")),
        };
        let abbreviation = match find("abbreviation") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("abbreviation"))?,
            None => ::std::default::Default::default(),
        };
        let icon = match find("icon") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("icon"))?,
            None => ::std::default::Default::default(),
        };
        let x = match find("x") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("x"))?,
            None => ::std::default::Default::default(),
        };
        let y = match find("y") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("y"))?,
            None => ::std::default::Default::default(),
        };
        let width = match find("width") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("width"))?,
            None => default_node_width(),
        };
        let height = match find("height") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("height"))?,
            None => default_node_height(),
        };
        let operator_kind = match find("operatorKind") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("operatorKind"))?,
            None => ::std::default::Default::default(),
        };
        let properties = match find("properties") {
            Some(v) => ::semio_framework_os_kernel::FromValue::from_value(v).map_err(|error: ::semio_framework_os_kernel::ValueError| error.under("properties"))?,
            None => ::std::default::Default::default(),
        };
        let kind = ::semio_framework_os_kernel::FromValue::from_value(::semio_framework_os_kernel::DslValue::Object(entries))?;
        Ok(Self { id, name, abbreviation, icon, x, y, width, height, operator_kind, properties, kind })
    }
}

impl DagNodeSpec {
    /// 🔧️ Builds a computation node with explicit IO ports.
    #[allow(clippy::too_many_arguments, reason = "positional constructor called by external crates (framework/surface/node-graph/rs, sequence/core/rs, flow/core/rs); bundling into a params struct is a breaking API change out of this crate's scope")]
    pub fn computation(id: String, name: &str, abbreviation: &str, icon: String, inputs: Vec<IoPortSpec>, outputs: Vec<IoPortSpec>, variadic_inputs: bool, variadic_outputs: bool, x: f64, y: f64, width: f64, height: f64) -> Self {
        let (name, abbreviation) = normalize_node_display(name, abbreviation);
        Self { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } }
    }

    /// 🧩️ Builds a cluster node with contract IO ports.
    #[allow(clippy::too_many_arguments, reason = "positional constructor called by external crates (framework/surface/node-graph/rs, sequence/core/rs, flow/core/rs); bundling into a params struct is a breaking API change out of this crate's scope")]
    pub fn cluster(id: String, name: &str, abbreviation: &str, icon: String, inputs: Vec<IoPortSpec>, outputs: Vec<IoPortSpec>, x: f64, y: f64, width: f64, height: f64) -> Self {
        let (name, abbreviation) = normalize_node_display(name, abbreviation);
        Self { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Cluster { inputs, outputs } }
    }

    /// ➕️ Whether the node exposes variadic input insert controls.
    pub fn variadic_inputs(&self) -> bool {
        match &self.kind {
            DagNodeKind::Computation { variadic_inputs, .. } => *variadic_inputs,
            _ => false,
        }
    }

    /// ➕️ Whether the node exposes variadic output insert controls.
    pub fn variadic_outputs(&self) -> bool {
        match &self.kind {
            DagNodeKind::Computation { variadic_outputs, .. } => *variadic_outputs,
            _ => false,
        }
    }

    /// ⬅️ Effective input ports for the node kind.
    pub fn inputs(&self) -> &[IoPortSpec] {
        match &self.kind {
            DagNodeKind::Computation { inputs, .. } | DagNodeKind::Cluster { inputs, .. } | DagNodeKind::AppInstance { inputs, .. } => inputs,
            DagNodeKind::Screen { input, .. } | DagNodeKind::Preview { input, .. } | DagNodeKind::Action { input, .. } | DagNodeKind::Export { input, .. } => std::slice::from_ref(input),
            _ => EMPTY_PORTS,
        }
    }

    /// ➡️ Effective output ports for the node kind.
    pub fn outputs(&self) -> &[IoPortSpec] {
        match &self.kind {
            DagNodeKind::Computation { outputs, .. } | DagNodeKind::Cluster { outputs, .. } | DagNodeKind::AppInstance { outputs, .. } => outputs,
            DagNodeKind::Slider { output, .. } | DagNodeKind::Select { output, .. } | DagNodeKind::Note { output, .. } | DagNodeKind::Image { output, .. } => std::slice::from_ref(output),
            _ => EMPTY_PORTS,
        }
    }
}

/// 🏷️ Whether a draw LOD tier shows a node caption at all.
///
/// A tier decides WHETHER a node is captioned, never WHAT the caption says: the caption is always
/// the node's name, clipped to the node's own measured width by
/// `canvas::text::ellipsize_by_measure`. The tier used to pick the content too — `Detail` (a tier
/// you reach by zooming IN past `Normal`) served `DagNodeSpec::abbreviation`, so a graph whose
/// operator records abbreviate to a single letter replaced every title with one glyph the moment
/// the camera crossed the band floor, and got the full names back on zooming further in to `Micro`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagNodeLabel {
    None,
    Name,
}

/// 📶️ Camera-zoom draw tier for DAG node chrome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagDrawLod {
    Minimap,
    Overview,
    Compact,
    Normal,
    Detail,
    Micro,
}

impl DagDrawLod {
    pub fn label(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::Overview => "overview",
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detail => "detail",
            Self::Micro => "micro",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id.trim() {
            "minimap" => Some(Self::Minimap),
            "overview" => Some(Self::Overview),
            "compact" => Some(Self::Compact),
            "normal" => Some(Self::Normal),
            "detail" => Some(Self::Detail),
            "micro" => Some(Self::Micro),
            _ => None,
        }
    }

    pub fn from_scale_index(index: usize) -> Self {
        match index {
            0 => Self::Minimap,
            1 => Self::Overview,
            2 => Self::Compact,
            3 => Self::Normal,
            4 => Self::Detail,
            _ => Self::Micro,
        }
    }

    pub fn node_icon_visible(self) -> bool {
        matches!(self, Self::Overview)
    }

    pub fn node_label(self) -> DagNodeLabel {
        match self {
            Self::Minimap | Self::Overview => DagNodeLabel::None,
            Self::Compact | Self::Normal | Self::Detail | Self::Micro => DagNodeLabel::Name,
        }
    }

    pub fn node_label_is_horizontal(self) -> bool {
        matches!(self, Self::Compact)
    }

    pub fn shows_computation_layout(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }

    pub fn shows_port_labels(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }

    pub fn shows_handles(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    pub fn uses_input_row_connection_hitbox(self) -> bool {
        self == Self::Normal
    }

    pub fn uses_channel_row_pick(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    pub fn allows_connection_hit_picking(self) -> bool {
        self.uses_input_row_connection_hitbox() || self.shows_handles()
    }

    pub fn shows_controls(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }

    pub fn shows_detail_text(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    pub fn edge_stroke_screen_px(self) -> f64 {
        match self {
            Self::Minimap => DAG_EDGE_STROKE_MINIMAP_SCREEN_PX,
            _ => DAG_EDGE_STROKE_SCREEN_PX,
        }
    }
}

impl IoPortSpec {
    /// 🏷️ Channel label for the active draw LOD (normal → abbreviation, detail → name, micro → fullName).
    pub fn display_label(&self, lod: DagDrawLod) -> &str {
        match lod {
            DagDrawLod::Micro => {
                if !self.full_name.is_empty() {
                    return self.full_name.as_str();
                }
                self.id.as_str()
            }
            DagDrawLod::Detail => self.id.as_str(),
            DagDrawLod::Normal => {
                if !self.abbreviation.is_empty() {
                    return self.abbreviation.as_str();
                }
                if !self.label.is_empty() {
                    return self.label.as_str();
                }
                self.id.as_str()
            }
            _ => self.display_code(),
        }
    }

    pub fn display_label_layout_width(&self, px: f64) -> f64 {
        [self.label_with_cardinality(DagDrawLod::Normal), self.label_with_cardinality(DagDrawLod::Detail), self.label_with_cardinality(DagDrawLod::Micro)].into_iter().map(|label| port_label_text_width(&label, px)).fold(0.0, f64::max)
    }
}

/// 📦️ `dag.fixture` document.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagFixture {
    pub schema: String,
    pub camera: DagCamera,
    pub nodes: Vec<DagNodeSpec>,
    pub edges: Vec<DagFixtureEdge>,
}

/// 📷️ Fixture camera snapshot.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DagCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🔗️ Edge between port handles.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct DagFixtureEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[value(default)]
    pub route_style: EdgeRouteStyle,
    #[value(default)]
    pub properties: PropertyBag,
}

impl Default for DagFixture {
    fn default() -> Self {
        let document = <DagSnapshot as crate::os_store::ArtifactDsl>::parse_dsl(crate::DAG_DEMO_TEXT)
            .expect("bundled DAG demo DSL is valid DagSnapshot text");
        Self { schema: document.schema, camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: document.nodes, edges: document.edges }
    }
}

pub fn split_dag_endpoint(endpoint: &str) -> (String, String) {
    if let Some((node, port)) = endpoint.rsplit_once('@') {
        return (node.to_string(), port.to_string());
    }
    (endpoint.to_string(), "out".into())
}

fn dag_visual_kind(node: &DagNodeSpec) -> String {
    node.operator_kind.clone().unwrap_or_else(|| dag_node_kind_tag(&node.kind).to_string())
}

/// 📝️ Render a DAG fixture as wire-literal compiled text.
pub fn dag_fixture_to_wire_literal(fixture: &DagFixture) -> String {
    use ::graph::dsl::{wire_literal_from_dag, WireEdge, WireNode};
    let nodes = fixture.nodes.iter().map(|node| WireNode { id: node.id.clone(), kind: dag_visual_kind(node), port: None, properties: node.properties.clone() }).collect::<Vec<_>>();
    let edges = fixture
        .edges
        .iter()
        .map(|edge| {
            let (from, from_port) = split_dag_endpoint(&edge.source);
            let (to, to_port) = split_dag_endpoint(&edge.target);
            WireEdge { from, from_port, to, to_port, directed: true, properties: edge.properties.clone() }
        })
        .collect::<Vec<_>>();
    wire_literal_from_dag(&nodes, &edges)
}

/// 🧵️ Build execution wire rows from an enriched DAG fixture.
pub fn dag_fixture_execution_rows(fixture: &DagFixture) -> (Vec<::graph::dsl::WireNode>, Vec<::graph::dsl::WireEdge>) {
    use ::graph::dsl::{WireEdge, WireNode};
    use std::collections::HashSet;
    let executable: HashSet<String> = fixture.nodes.iter().filter_map(|node| node.operator_kind.as_ref().map(|_| node.id.clone())).collect();
    let nodes = fixture
        .nodes
        .iter()
        .filter_map(|node| {
            let kind = node.operator_kind.clone()?;
            Some(WireNode { id: node.id.clone(), kind, port: None, properties: node.properties.clone() })
        })
        .collect();
    let edges = fixture
        .edges
        .iter()
        .filter_map(|edge| {
            let (from, from_port) = split_dag_endpoint(&edge.source);
            let (to, to_port) = split_dag_endpoint(&edge.target);
            if !executable.contains(&from) || !executable.contains(&to) {
                return None;
            }
            Some(WireEdge { from, from_port, to, to_port, directed: true, properties: edge.properties.clone() })
        })
        .collect();
    (nodes, edges)
}

pub fn port_label_text_width(label: &str, px: f64) -> f64 {
    let trimmed = label.trim();
    if trimmed.is_empty() || px < 4.0 {
        return 0.0;
    }
    let pad = px * 0.28;
    trimmed.len() as f64 * px * 0.62 + pad * 2.0
}

pub const DAG_EDGE_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_EDGE;

pub const DAG_EDGE_STROKE_MINIMAP_SCREEN_PX: f64 = ui_styling::strokes::DAG_EDGE_MINIMAP;

/// 🔀️ Advances a persisted select node to its next valid option.
pub fn advance_select_option(node: &mut DagNodeSpec) -> Option<String> {
    let DagNodeKind::Select { options, selected, .. } = &mut node.kind else {
        return None;
    };
    if options.is_empty() {
        return None;
    }
    let count = dag_index_to_wire(options.len());
    *selected = ((*selected % count).checked_add(1)?) % count;
    options.get(usize::try_from(*selected).ok()?).cloned()
}
