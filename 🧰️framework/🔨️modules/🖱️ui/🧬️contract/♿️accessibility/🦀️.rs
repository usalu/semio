//! @emoji ♿️ The `AccessibilitySpec` carried by every node record.
//!
//! ⚠️ SCAFFOLD — owned by packet `contract-layout`. Replace this placeholder wholesale; keep the region
//! structure and the U1 sync rule (no `async fn` in this crate).

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Accessibility

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// 📢️ An ARIA-live-region politeness level, translated by each renderer into its own live-announce
/// mechanism (DOM `aria-live`, the GPU renderer's accessibility snapshot, ...).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Liveness {
    #[default]
    Off,
    Polite,
    Assertive,
}

/// ♿️ The accessibility intent every node carries once, resolved correctly by every renderer. No
/// `role` field: the semantic role is implied by [`crate::Component`] — a `Component::Button` is a
/// button on every renderer, so naming the role again here would just be a second, driftable source
/// of truth.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct AccessibilitySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<crate::Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::Label>,
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub live: Liveness,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub hidden: bool,
}
//#endregion 🔖️Accessibility

//#region 🔖️AccessibilityProjection
/// ♿️ The ARIA role a component IMPLIES. [`AccessibilitySpec`] carries no `role` on purpose — a
/// `Component::Button` is a button on every renderer — so this is the one place that implication is
/// written down, shared by every target instead of re-derived per renderer. React reaches the same
/// roles implicitly, through the HTML element each component view renders (`<button>`, `<p>`,
/// `<input>`, …); a renderer with no DOM (the wgpu target) has to say them out loud, which is what
/// makes its accessibility projection readable at all.
///
/// `activatable` is the record's own `Trigger::Activate` binding: a container you can press IS a
/// button, exactly as React's Interpreter spells it (`role={activateBinding ? "button" : role}`).
pub fn accessibility_role(component: &crate::Component, activatable: bool) -> &'static str {
    match component {
        crate::Component::Container(props) if activatable => match props.role {
            crate::ContainerRole::Toolbar | crate::ContainerRole::Form => container_role_name(props.role),
            _ => "button",
        },
        crate::Component::Container(props) => container_role_name(props.role),
        crate::Component::Text(_) => "paragraph",
        crate::Component::Button(_) => "button",
        crate::Component::Separator(_) => "separator",
        crate::Component::Input(_) => "textbox",
        crate::Component::Select(_) => "combobox",
        crate::Component::Toggle(_) => "switch",
        crate::Component::KeyValueList(_) => "list",
        crate::Component::Slider(_) | crate::Component::Ring(_) => "slider",
        crate::Component::NumberStepper(_) => "spinbutton",
        crate::Component::IconSelect(_) => "radiogroup",
        crate::Component::Tree(_) => "tree",
        crate::Component::TreeSection(_) => "group",
        crate::Component::TreeItem(_) => "treeitem",
        crate::Component::Image(_) => "img",
        crate::Component::Surface(_) => "application",
        crate::Component::Extension(_) => "region",
    }
}

/// 🗂️ The authored container role's own ARIA name — `Plain`/`Group`/`Field` are all plain grouping
/// (a `Field` is a label plus its control, which is a group, not a landmark), `Section` is a region.
fn container_role_name(role: crate::ContainerRole) -> &'static str {
    match role {
        crate::ContainerRole::Plain | crate::ContainerRole::Group | crate::ContainerRole::Field => "group",
        crate::ContainerRole::Section => "region",
        crate::ContainerRole::Form => "form",
        crate::ContainerRole::Toolbar => "toolbar",
    }
}

/// ⌨️ Whether a component takes keyboard focus of its own — the same set the wgpu `EventRouter`'s
/// `is_focusable` walks for Tab traversal, said once here so the projection and the traversal can
/// never disagree about who is reachable.
pub fn accessibility_is_focusable(component: &crate::Component, activatable: bool) -> bool {
    match component {
        crate::Component::Button(_)
        | crate::Component::Input(_)
        | crate::Component::Select(_)
        | crate::Component::Toggle(_)
        | crate::Component::Slider(_)
        | crate::Component::NumberStepper(_)
        | crate::Component::Ring(_)
        | crate::Component::IconSelect(_)
        | crate::Component::TreeItem(_) => true,
        crate::Component::Container(_) => activatable,
        _ => false,
    }
}

/// 📢️ The wire spelling of a liveness level, identical to its `serde` `rename_all = "camelCase"`
/// tag, so a projection consumer reads the same token the wire carries.
pub fn liveness_name(live: Liveness) -> &'static str {
    match live {
        Liveness::Off => "off",
        Liveness::Polite => "polite",
        Liveness::Assertive => "assertive",
    }
}

/// ♿️ ONE node of the accessibility projection a renderer publishes for an assistive technology to
/// read. Flat and ordered: `depth` carries the tree shape, so a consumer rebuilds nesting without
/// the projection having to nest (the same flat-table discipline [`crate::UiSnapshot`] uses).
///
/// A hidden node is KEPT, carrying `hidden: true`, rather than dropped — that is what React does
/// (`aria-hidden` on a rendered element), and a consumer that dropped it would silently disagree
/// with the DOM renderer about what exists.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityProjectionNode {
    pub node_id: u64,
    pub key: String,
    pub role: String,
    pub depth: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub live: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub hidden: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub focusable: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub actionable: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub focused: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect: Option<[f32; 4]>,
}

/// ♿️ Projects ONE published node record. Pure over the record — every renderer's own walk supplies
/// `depth`, and afterwards stamps whatever live state only it knows (`focused`, `rect`).
pub fn accessibility_projection_node(record: &crate::UiNodeRecord, depth: usize) -> AccessibilityProjectionNode {
    let activatable = record.bindings.iter().any(|binding| binding.trigger == crate::Trigger::Activate);
    AccessibilityProjectionNode {
        node_id: record.id.0,
        key: record.key.as_str().to_string(),
        role: accessibility_role(&record.component, activatable).to_string(),
        depth,
        label: record.accessibility.label.as_ref().map(|label| label.0.as_str().to_string()),
        description: record.accessibility.description.as_ref().map(|label| label.0.as_str().to_string()),
        live: liveness_name(record.accessibility.live).to_string(),
        shortcut: record.accessibility.shortcut.as_ref().map(|shortcut| shortcut.as_str().to_string()),
        hidden: record.accessibility.hidden,
        disabled: record.disabled,
        focusable: accessibility_is_focusable(&record.component, activatable),
        actionable: activatable || !record.bindings.is_empty(),
        focused: false,
        rect: None,
    }
}
//#endregion 🔖️AccessibilityProjection

#[cfg(test)]
#[path = "../🧪️tests/🔬️accessibility-unit/🦀️.rs"]
mod tests;
