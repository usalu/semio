//! 🎛️ Container roles and row actions: the container role, input kind and row action placement enums, the drop overlay, select item and key value records, the `RowAction` and the `RowTarget` with their credited arguments. See [`super`].

use super::*;

/// 🐳️ counterpart, added per the packet brief for the layouts those two collapsed variants could not
/// previously express as a single node.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum ContainerRole {
    #[default]
    Plain,
    Section,
    Group,
    Field,
    Form,
    Toolbar,
}

/// ⌨️ The closed set of input kinds. Grepped from the fleet's actual `UiInputNode.input_kind` string
/// literals (`"text"`, `"textarea"`, `"number"`, `"file"`) plus the values the React `🟦️Interpreter`
/// already branches on for `inputKind` (`"number"`, `"longText"`, `"date"`, `"color"`, `"file"`,
/// default `"text"`) — the union of both sides, since the renderer supports more kinds than any
/// current plugin emits yet. Note the old Rust/TS spelling mismatch this closes: Rust emitted
/// `"textarea"`, TS checked `"longText"`; this enum has exactly one spelling (`LongText`) for that
/// kind. No `Search` kind exists anywhere in the fleet today, so it is not included (see the
/// packet's own report for the grep evidence — adding it back is a one-variant change, not a design
/// change, if a plugin needs it later).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum InputKind {
    #[default]
    Text,
    LongText,
    Number,
    Date,
    Color,
    File,
}

/// 📍️ Where a [`RowAction`] paints: on the tree row itself, or folded into the row's context menu.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum RowActionPlacement {
    #[default]
    Row,
    Menu,
}

//#region 🧱️Nested
/// 📥️ Hover-state copy for a [`ContainerProps::drop_overlay`] — shown while a drag is over the
/// container, ahead of its `Drop`-triggered [`crate::ActionBinding`] firing on release.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct DropOverlaySpec {
    pub title: Label,
    pub hint: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<crate::UiText>,
}

/// 🔽️ One option of a [`Component::Select`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct SelectItem {
    pub value: crate::UiText,
    pub label: Label,
}

/// 🗝️ One row of a [`Component::KeyValueList`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct KeyValueEntry {
    pub label: Label,
    pub value: crate::UiText,
}

/// 🎬️ One action affordance painted on (or reachable from) a [`Component::TreeItem`] or
/// [`Component::TableRow`] row. It names only its `verb`: the scope, version and argument map are the
/// row's ONE [`RowTarget`], which every row action and the row's activation inherit, so N actions cost N
/// verbs, never N argument maps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct RowAction {
    /// 🖼️ Icon key. See [`ButtonProps::icon`] for why this is a plain `String`, not a closed enum.
    pub icon: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    /// ▶️ The verb this action fires on the row's [`RowTarget`].
    pub verb: crate::UiText,
    #[serde(default, skip_serializing_if = "is_default_row_action_placement")]
    #[value(default, skip_serializing_if = "is_default_row_action_placement")]
    pub placement: RowActionPlacement,
    /// 🚫️ A disabled action paints and announces disabled and never dispatches ([`RowTarget::action_binding`] refuses it
    /// typed). Only a disabled action carries the flag, so an enabled row costs nothing for it.
    #[serde(default, skip_serializing_if = "is_enabled_row_action")]
    #[value(default, skip_serializing_if = "is_enabled_row_action")]
    pub disabled: bool,
    /// 💬️ Why a disabled action cannot run, producer-localized. A disabled action stays focusable (`aria-disabled`, WAI-ARIA
    /// focusable-when-disabled) and every renderer names this reason as its description (`aria-describedby`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Label>,
}

impl RowAction {
    /// 🚫️ This action with its enabled state set — `true` paints it disabled and refuses its dispatch.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// 🧾️ This action disabled because of `reason` — the text every renderer announces as the disabled action's description.
    pub fn disabled_because(mut self, reason: Label) -> Self {
        self.disabled = true;
        self.reason = Some(reason);
        self
    }
}

/// 🚫️ Why a row action does not dispatch — the ONE typed refusal every host answers a row verb with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowActionRefusal {
    /// 🚫️ The action is disabled on this row.
    Disabled,
    /// 🧮️ The target's argument map could not be admitted into one more binding.
    Credit,
}

/// 🎯️ The ONE action target of a row: the controller `scope` at its contract `version` that every
/// [`RowAction::verb`] and the row's `activation` fire in, the ONE argument map they all inherit, and the
/// verb the row's primary activation (open, select) fires. A host dispatches a row verb as
/// [`RowTarget::binding`] — the same [`crate::ActionBinding`] shape any record binding has, so a tree row
/// and a table row with the same target dispatch identically. `args` is a [`crate::UiMap`] handle, not an
/// inline [`crate::UiValue`], so a targeted row costs one scope and one activation verb inline.
// 🌱️ No `ToValue`/`FromValue` here: `args` embeds `UiMap`, the deliberate DslValue-free exception —
// see `UiValue`'s docstring in `🎬️action.rs`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowTarget {
    pub scope: crate::UiText,
    pub version: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<crate::UiMap>,
    /// ▶️ The verb the row's primary activation fires on this target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation: Option<crate::UiText>,
}

impl RowTarget {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self { scope: self.scope.clone(), version: self.version, args: credited_args(&self.args)?, activation: self.activation.clone() })
    }

    /// 🔗️ The `Trigger::Activate` binding `verb` fires on this target: its scope and version, its argument map.
    pub fn binding(&self, verb: &crate::UiText) -> Option<crate::ActionBinding> {
        Some(crate::ActionBinding { trigger: crate::Trigger::Activate, action: crate::ActionId::new(self.scope.clone(), verb.clone(), self.version), args: credited_args(&self.args)?.map(crate::UiValue::Map), capability: None })
    }

    /// 🎬️ The versioned id one of the row's actions fires on this target — refused typed when the action is disabled, so a
    /// host never dispatches what the row paints disabled.
    pub fn action_id(&self, action: &RowAction) -> Result<crate::ActionId, RowActionRefusal> {
        if action.disabled {
            return Err(RowActionRefusal::Disabled);
        }
        Ok(crate::ActionId::new(self.scope.clone(), action.verb.clone(), self.version))
    }

    /// 🔗️ The binding one of the row's actions fires on this target: [`Self::action_id`] with the target's argument map.
    pub fn action_binding(&self, action: &RowAction) -> Result<crate::ActionBinding, RowActionRefusal> {
        let action = self.action_id(action)?;
        Ok(crate::ActionBinding { trigger: crate::Trigger::Activate, action, args: credited_args(&self.args).ok_or(RowActionRefusal::Credit)?.map(crate::UiValue::Map), capability: None })
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn credited_args(args: &Option<crate::UiMap>) -> Option<Option<crate::UiMap>> {
    match args {
        Some(args) => Some(Some(args.credited_clone()?)),
        None => Some(None),
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default_row_action_placement(value: &RowActionPlacement) -> bool {
    *value == RowActionPlacement::default()
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_enabled_row_action(disabled: &bool) -> bool {
    !*disabled
}
//#endregion 🧱️Nested
