//! 🎬️ Canonical versioned semantic actions, bindings and intents.
use serde::{Serialize,Deserialize};
use semio_framework_value_derive::{ToValue,FromValue};
use std::fmt;
pub use semio_framework_ui_contract_values::*;
//#region 🔖️Action

/// 🆔️ A versioned action address. `scope` names the controller/domain (the old `ActionDescriptor`'s
/// stringly `controller_id`, e.g. `"cad-play"`, grepped verbatim from the plugin fleet's
/// `ActionFactory::new(CONTROLLER_ID)` call sites), `name` the verb (the old `action`, e.g.
/// `"objectMove"`/`"setValue"`/`"addWidget"`), and `version` is new: it lets a renderer reject or
/// migrate a stale action instead of silently invoking the wrong one — the one axis the old stringly
/// pair never carried.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ActionId {
    pub scope: UiText,
    pub name: UiText,
    pub version: u16,
}

impl ActionId {
    /// 🏭️ `const fn`-friendly constructor — every field already owned, no allocation happens inside.
    pub fn new(scope: UiText, name: UiText, version: u16) -> Self {
        Self { scope, name, version }
    }

    /// 🌱️ Version-1 convenience constructor — the common case; the plugin fleet will write thousands
    /// of these from `&str`/`String` call sites the old `ActionFactory::action` already used.
    pub fn try_v1(scope: impl AsRef<str>, name: impl AsRef<str>) -> Option<Self> {
        Some(Self { scope: UiText::try_from_str(scope.as_ref())?, name: UiText::try_from_str(name.as_ref())?, version: 1 })
    }
}

impl fmt::Display for ActionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}@{}", self.scope, self.name, self.version)
    }
}

/// 🎯️ The lifecycle moment on a node that fires an [`ActionBinding`] — replaces the old single
/// implicit "the" action every node carried with a closed, named set, so one node can bind several
/// distinct moments (e.g. `Change` while typing, `Commit` on blur) without inventing parallel fields.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum Trigger {
    #[default]
    Activate,
    Change,
    Commit,
    Delta,
    Drop,
    Submit,
    Abort,
    RepeatLast,
    HoverPreview,
}

/// 🔗️ One node-carried binding from a [`Trigger`] moment to a versioned [`ActionId`]. Replaces every
/// old `on_change`/`action`/`drop_action`/... field scattered across the wgpu target's per-component
/// node structs — a record's `bindings: Vec<ActionBinding>` is the one place any of them now live.
// 🌱️ No `ToValue`/`FromValue` here: `args: Option<UiValue>` and `UiValue` itself is the deliberate
// DslValue-free exception (see `UiValue`'s docstring below) — a derived impl would need
// `UiValue: ToValue`, which is exactly the forbidden conversion.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionBinding {
    pub trigger: Trigger,
    pub action: ActionId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
    /// 🔐️ An optional capability token a host must hold before this binding is even offered —
    /// orthogonal to `args`, which is data the action consumes rather than a permission gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<UiText>,
}

impl ActionBinding {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            trigger: self.trigger,
            action: self.action.clone(),
            args: match self.args.as_ref() {
                Some(args) => Some(args.credited_clone()?),
                None => None,
            },
            capability: self.capability.clone(),
        })
    }
}

/// 📋️ A reference to a resolved context menu — replaces the old `UiMenuRef`'s `DslValue` args with
/// the crate-neutral [`UiValue`].
// 🌱️ No `ToValue`/`FromValue` here — same `UiValue` exception as `ActionBinding` above.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRef {
    pub id: UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
}

impl MenuRef {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            id: self.id.clone(),
            args: match self.args.as_ref() {
                Some(args) => Some(args.credited_clone()?),
                None => None,
            },
        })
    }
}

/// 👤️ One user action against a specific node at a specific revision — what a renderer emits and the
/// headless runtime dispatches. `revision`/`node_key` let the runtime recognise and drop a `Stale`
/// intent (one whose `revision` trails the surface's current revision by more than one) instead of
/// applying it against geometry the user never actually saw.
// 🌱️ No `ToValue`/`FromValue` here — same `UiValue` exception as `ActionBinding` above (`args`/`input`).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiIntent {
    pub surface: crate::SurfaceId,
    pub revision: crate::UiRevision,
    pub node: crate::UiNodeId,
    /// 🔑️ The node's own [`crate::UiNodeRecord::key`], carried alongside the id so a replay or a log
    /// entry still identifies the intended element after id churn from an intervening reconciliation.
    pub node_key: UiText,
    pub trigger: Trigger,
    pub action: ActionId,
    /// 🔁️ Echoed verbatim from the firing [`ActionBinding::args`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
    /// ✍️ The trigger-specific payload: `Change`'s new value, `Delta`'s signed step count, `Drop`'s
    /// dropped payload — `None` for triggers that carry no data of their own (`Activate`, `Submit`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<UiValue>,
    /// 🔢️ Renderer-monotonic per surface — lets the runtime order and de-duplicate intents
    /// independently of transport delivery order.
    pub seq: u64,
}
//#endregion 🔖️Action

pub type UiNodeBindings=UiFixedList<ActionBinding,UI_NODE_BINDINGS>;
#[path="🧾️fields/🦀️.rs"]
mod fields;
#[path="♻️retirement/🦀️.rs"]
mod typed_retirement;
#[path="🧾️copy/🦀️.rs"]
mod typed_copy;
#[path="🧾️compare/🦀️.rs"]
mod typed_compare;
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

