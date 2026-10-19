//! 🪟️ Window layout tree: the window and stack kinds, the stack corner, the window, stack and axis nodes, the layout child and root unions, the named layouts and the layout constructors and window kind collection. See [`super`].

use super::*;

// #region layout
//#region 🔖️WindowLayout
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
fn kind_window() -> String {
    "window".into()
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
fn kind_stack() -> String {
    "stack".into()
}

/// 🧭️ Corner of a window stack where a tab chip docks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum WindowStackCorner {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowLayoutWindowNode {
    #[serde(default = "kind_window")]
    #[value(default = "kind_window")]
    pub kind: String,
    pub window_kind_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub corner: Option<WindowStackCorner>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowLayoutStackNode {
    #[serde(default = "kind_stack")]
    #[value(default = "kind_stack")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "activeId")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub active_window_kind_id: Option<String>,
    pub children: Vec<WindowLayoutWindowNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowLayoutAxisNode {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    pub children: Vec<WindowLayoutChild>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WindowLayoutChild {
    Axis(WindowLayoutAxisNode),
    Stack(WindowLayoutStackNode),
}

impl<'de> Deserialize<'de> for WindowLayoutChild {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        match serde_window_layout_node_is_stack(&value).map_err(serde::de::Error::custom)? {
            true => serde_json::from_value(value).map(WindowLayoutChild::Stack).map_err(serde::de::Error::custom),
            false => serde_json::from_value(value).map(WindowLayoutChild::Axis).map_err(serde::de::Error::custom),
        }
    }
}

// 🌱️ Hand-written, not derived: `#[serde(untagged)]` has no `#[value(...)]` equivalent (the
// derive's supported container attributes are `rename_all`/`tag`/`content`/`default`/
// `deny_unknown_fields`/`transparent`/`bound` only — see `🌱️value/✨️derive/🦀️.rs`'s module
// docstring). The canonical `kind` field is the discriminator because both shapes may omit
// every optional field and an order-based trial would then misclassify an empty stack as an axis.
impl ToValue for WindowLayoutChild {
    fn to_value(&self) -> DslValue {
        match self {
            WindowLayoutChild::Axis(node) => node.to_value(),
            WindowLayoutChild::Stack(node) => node.to_value(),
        }
    }
}

impl FromValue for WindowLayoutChild {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match window_layout_node_is_stack(&value)? {
            true => WindowLayoutStackNode::from_value(value).map(WindowLayoutChild::Stack),
            false => WindowLayoutAxisNode::from_value(value).map(WindowLayoutChild::Axis),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WindowLayoutRoot {
    Axis(WindowLayoutAxisNode),
    Stack(WindowLayoutStackNode),
}

impl<'de> Deserialize<'de> for WindowLayoutRoot {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        match serde_window_layout_node_is_stack(&value).map_err(serde::de::Error::custom)? {
            true => serde_json::from_value(value).map(WindowLayoutRoot::Stack).map_err(serde::de::Error::custom),
            false => serde_json::from_value(value).map(WindowLayoutRoot::Axis).map_err(serde::de::Error::custom),
        }
    }
}

// 🌱️ Hand-written for the same reason as `WindowLayoutChild` above — `#[serde(untagged)]` has
// no `#[value(...)]` equivalent.
impl ToValue for WindowLayoutRoot {
    fn to_value(&self) -> DslValue {
        match self {
            WindowLayoutRoot::Axis(node) => node.to_value(),
            WindowLayoutRoot::Stack(node) => node.to_value(),
        }
    }
}

impl FromValue for WindowLayoutRoot {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match window_layout_node_is_stack(&value)? {
            true => WindowLayoutStackNode::from_value(value).map(WindowLayoutRoot::Stack),
            false => WindowLayoutAxisNode::from_value(value).map(WindowLayoutRoot::Axis),
        }
    }
}

fn window_layout_node_is_stack(value: &DslValue) -> Result<bool, semio_framework_value::ValueError> {
    let DslValue::Object(entries) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected window layout object")) };
    let kind = entries.iter().find(|(key, _)| key == "kind").and_then(|(_, value)| value.as_str()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing field `kind`"))?;
    window_layout_kind_is_stack(kind).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error))
}

fn serde_window_layout_node_is_stack(value: &serde_json::Value) -> Result<bool, String> {
    let kind = value.as_object().and_then(|entries| entries.get("kind")).and_then(serde_json::Value::as_str).ok_or_else(|| "missing field `kind`".to_string())?;
    window_layout_kind_is_stack(kind)
}

fn window_layout_kind_is_stack(kind: &str) -> Result<bool, String> {
    match kind {
        "stack" => Ok(true),
        "row" | "column" | "horizontal" | "vertical" => Ok(false),
        _ => Err(format!("unsupported window layout kind `{kind}`")),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowLayout {
    pub root: WindowLayoutRoot,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NamedLayout {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<IconName>,
    pub layout: WindowLayout,
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub group_path: Option<Vec<String>>,
}

pub fn create_window_layout(window_kind_id: impl Into<String>, title: Option<String>, instance_id: Option<String>, template_id: Option<String>) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode { kind: kind_window(), window_kind_id: window_kind_id.into(), title, instance_id, template_id, corner: None }
}

pub fn create_stack_layout(window_kind_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
    let mut children = Vec::with_capacity(window_kind_ids.len());
    for (index, id) in window_kind_ids.iter().enumerate() {
        children.push(create_window_layout(id.clone(), titles.and_then(|rows| rows.get(index).cloned()), None, None));
    }
    WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: kind_stack(), size: None, active_window_kind_id: None, children }) }
}

/// 🪟️ `direction` is an AXIS direction (`row`/`column`). `stack` is not an axis — it is the
/// tabbed leaf container — so a `stack` request is answered with the single stack root
/// [`create_stack_layout`] mints. Minting `Axis { kind: "stack" }` instead put a stack node
/// under a node the whole shell reads AS a stack, and every consumer that walks a stack's
/// children as window leaves (the React seed resolver
/// `convertFrameworkLayoutNodeToModeLayout`) then produced windows with no id — which is how
/// `🎞️animate` booted with zero panes (ticket 26/09/18, slice F1).
pub fn create_default_layout(window_ids: &[String], direction: &str, sizes: Option<&[f64]>, titles: Option<&[String]>) -> WindowLayout {
    if direction == kind_stack() {
        return create_stack_layout(window_ids, titles);
    }
    let mut children = Vec::with_capacity(window_ids.len());
    for (index, id) in window_ids.iter().enumerate() {
        let window = create_window_layout(id.clone(), titles.and_then(|rows| rows.get(index).cloned()).or_else(|| Some(id.clone())), None, None);
        children.push(WindowLayoutChild::Stack(WindowLayoutStackNode { kind: kind_stack(), size: sizes.and_then(|rows| rows.get(index).copied()), active_window_kind_id: None, children: vec![window] }));
    }
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: direction.into(), size: None, children }) }
}

pub fn create_tab_stack_layout(window_ids: &[String], titles: Option<&[String]>) -> WindowLayout {
    create_stack_layout(window_ids, titles)
}

/// 🪟️ Builds a balanced fallback layout for an app that declares no `default_layout`: a single
/// stack when there is one window, otherwise an even row of single-window stacks.
pub fn even_window_layout(window_ids: &[String]) -> WindowLayout {
    if window_ids.is_empty() {
        return WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: kind_stack(), size: None, active_window_kind_id: None, children: vec![] }) };
    }
    if window_ids.len() == 1 {
        return WindowLayout {
            root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: kind_stack(), size: None, active_window_kind_id: Some(window_ids[0].clone()), children: vec![create_window_layout(window_ids[0].clone(), None, None, None)] }),
        };
    }
    let count = window_ids.len() as f64;
    let mut children = Vec::with_capacity(window_ids.len());
    for id in window_ids {
        let window = create_window_layout(id.clone(), None, None, None);
        children.push(WindowLayoutChild::Stack(WindowLayoutStackNode { kind: kind_stack(), size: Some(1.0 / count), active_window_kind_id: Some(id.clone()), children: vec![window] }));
    }
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children }) }
}

pub fn create_named_layout(id: impl Into<String>, label: impl Into<String>, layout: WindowLayout, origin: impl Into<String>, icon_id: Option<IconName>, group_path: Option<Vec<String>>) -> NamedLayout {
    NamedLayout { id: id.into(), label: label.into(), icon_id, layout, origin: origin.into(), group_path }
}

pub fn merge_named_layouts(base: &[NamedLayout], extension: &[NamedLayout]) -> Vec<NamedLayout> {
    let mut merged: HashMap<String, NamedLayout> = HashMap::new();
    for entry in base {
        merged.insert(entry.id.clone(), entry.clone());
    }
    for entry in extension {
        merged.insert(entry.id.clone(), entry.clone());
    }
    merged.into_values().collect()
}

/// 🧭️ Collects every `window_kind_id` referenced by a layout tree.
pub fn collect_window_kind_ids_from_layout(layout: &WindowLayout) -> Vec<String> {
    let mut ids = Vec::new();
    collect_window_kind_ids_from_root(&layout.root, &mut ids);
    ids
}

fn collect_window_kind_ids_from_root(root: &WindowLayoutRoot, out: &mut Vec<String>) {
    match root {
        WindowLayoutRoot::Axis(axis) => collect_window_kind_ids_from_children(&axis.children, out),
        WindowLayoutRoot::Stack(stack) => collect_window_kind_ids_from_stack(stack, out),
    }
}

fn collect_window_kind_ids_from_children(children: &[WindowLayoutChild], out: &mut Vec<String>) {
    for child in children {
        match child {
            WindowLayoutChild::Axis(axis) => collect_window_kind_ids_from_children(&axis.children, out),
            WindowLayoutChild::Stack(stack) => collect_window_kind_ids_from_stack(stack, out),
        }
    }
}

fn collect_window_kind_ids_from_stack(stack: &WindowLayoutStackNode, out: &mut Vec<String>) {
    for window in &stack.children {
        out.push(window.window_kind_id.clone());
    }
}
//#endregion 🔖️WindowLayout
// #endregion layout
