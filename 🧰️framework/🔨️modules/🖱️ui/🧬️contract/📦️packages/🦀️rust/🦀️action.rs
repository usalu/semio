//! @emoji 🎬️ Versioned `ActionId`, `Trigger`, `ActionBinding`, `UiIntent` and neutral `UiValue`.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Action

/// 🆔️ A versioned action address. `scope` names the controller/domain (the old `ActionDescriptor`'s
/// stringly `controller_id`, e.g. `"cad-play"`, grepped verbatim from the plugin fleet's
/// `ActionFactory::new(CONTROLLER_ID)` call sites), `name` the verb (the old `action`, e.g.
/// `"objectMove"`/`"setValue"`/`"addWidget"`), and `version` is new: it lets a renderer reject or
/// migrate a stale action instead of silently invoking the wrong one — the one axis the old stringly
/// pair never carried.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionId {
    pub scope: String,
    pub name: String,
    pub version: u16,
}

impl ActionId {
    /// 🏭️ `const fn`-friendly constructor — every field already owned, no allocation happens inside.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub const fn new(scope: String, name: String, version: u16) -> Self {
        Self { scope, name, version }
    }

    /// 🏭️ Version-1 convenience constructor — the common case; the plugin fleet will write thousands
    /// of these from `&str`/`String` call sites the old `ActionFactory::action` already used.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn v1(scope: impl Into<String>, name: impl Into<String>) -> Self {
        Self { scope: scope.into(), name: name.into(), version: 1 }
    }
}

impl std::fmt::Display for ActionId {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}@{}", self.scope, self.name, self.version)
    }
}

/// 🎯️ The lifecycle moment on a node that fires an [`ActionBinding`] — replaces the old single
/// implicit "the" action every node carried with a closed, named set, so one node can bind several
/// distinct moments (e.g. `Change` while typing, `Commit` on blur) without inventing parallel fields.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionBinding {
    pub trigger: Trigger,
    pub action: ActionId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
    /// 🔐️ An optional capability token a host must hold before this binding is even offered —
    /// orthogonal to `args`, which is data the action consumes rather than a permission gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<String>,
}

/// 📋️ A reference to a resolved context menu — replaces the old `UiMenuRef`'s `DslValue` args with
/// the crate-neutral [`UiValue`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuRef {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
}

/// 🎬️ One user action against a specific node at a specific revision — what a renderer emits and the
/// headless runtime dispatches. `revision`/`node_key` let the runtime recognise and drop a `Stale`
/// intent (one whose `revision` trails the surface's current revision by more than one) instead of
/// applying it against geometry the user never actually saw.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiIntent {
    pub surface: crate::SurfaceId,
    pub revision: crate::UiRevision,
    pub node: crate::UiNodeId,
    /// 🔑️ The node's own [`crate::UiNodeRecord::key`], carried alongside the id so a replay or a log
    /// entry still identifies the intended element after id churn from an intervening reconciliation.
    pub node_key: String,
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

//#region 🔖️Value
/// 🧬️ A neutral, JSON-shaped value — the ONE recursive type in this crate. Every node in
/// `🦀️document.rs` avoids inline recursion by addressing children through [`crate::UiNodeId`] instead
/// of nesting a node inside another; `UiValue` is the deliberate exception because it does not
/// describe document shape at all, it describes an arbitrary opaque payload (action args, extension
/// props) that genuinely IS JSON-shaped, and `Vec`/`BTreeMap` already give the schema an indirection to
/// resolve (heap-allocated, not an inline field) rather than the infinitely-sized-struct problem
/// direct node-in-node nesting would create.
///
/// ⚠️ The os-kernel's `DslValue` (`🧰️framework/🔨️modules/🌱️value/🦀️component.rs`) must NEVER appear in
/// this crate — this crate has no such dependency and stays `wasm32-wasip2`/`wasm32-unknown-unknown`
/// safe by construction. `From`/`Into` conversions between `UiValue` and `DslValue` belong in the
/// os-kernel crate, never here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UiValue {
    #[default]
    Null,
    Bool(bool),
    Number(f64),
    Text(String),
    List(Vec<UiValue>),
    Map(BTreeMap<String, UiValue>),
}
//#endregion 🔖️Value

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_id_displays_scope_dot_name_at_version() {
        let id = ActionId::v1("cad-play", "objectMove");
        assert_eq!(id.to_string(), "cad-play.objectMove@1");
        assert_eq!(ActionId::new("app".into(), "submit".into(), 3).to_string(), "app.submit@3");
    }

    #[allow(clippy::needless_pass_by_value)]
    fn value_round_trips(value: UiValue) {
        let first = serde_json::to_string(&value).expect("serialize");
        let deserialized: UiValue = serde_json::from_str(&first).expect("deserialize");
        let second = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(first, second);
        assert_eq!(value, deserialized);
    }

    #[test]
    fn every_ui_value_shape_round_trips() {
        value_round_trips(UiValue::Null);
        value_round_trips(UiValue::Bool(true));
        value_round_trips(UiValue::Number(-3.5));
        value_round_trips(UiValue::Text("hi".into()));
        value_round_trips(UiValue::List(vec![UiValue::Number(1.0), UiValue::Text("two".into())]));
        let mut map = BTreeMap::new();
        map.insert("id".to_string(), UiValue::Text("widget".into()));
        map.insert("nested".to_string(), UiValue::List(vec![UiValue::Bool(false), UiValue::Null]));
        value_round_trips(UiValue::Map(map));
    }

    #[test]
    fn ui_value_default_is_null() {
        assert_eq!(UiValue::default(), UiValue::Null);
    }

    #[test]
    fn action_binding_round_trips_with_and_without_args() {
        let full = ActionBinding { trigger: Trigger::Change, action: ActionId::v1("app", "setValue"), args: Some(UiValue::Text("scope".into())), capability: Some("edit".into()) };
        let first = serde_json::to_string(&full).expect("serialize");
        let back: ActionBinding = serde_json::from_str(&first).expect("deserialize");
        assert_eq!(full, back);

        let minimal = ActionBinding::default();
        let json = serde_json::to_value(&minimal).expect("serialize");
        assert!(json.get("args").is_none());
        assert!(json.get("capability").is_none());
    }

    #[test]
    fn menu_ref_round_trips() {
        let menu = MenuRef { id: "context.tree-item".into(), args: Some(UiValue::Number(2.0)) };
        let first = serde_json::to_string(&menu).expect("serialize");
        let back: MenuRef = serde_json::from_str(&first).expect("deserialize");
        assert_eq!(menu, back);
    }

    #[test]
    fn ui_intent_round_trips() {
        let intent = UiIntent {
            surface: crate::SurfaceId::from("note.play.navigator"),
            revision: crate::UiRevision(4),
            node: crate::UiNodeId(9),
            node_key: "row-9".into(),
            trigger: Trigger::Delta,
            action: ActionId::v1("cad-play", "objectMove"),
            args: Some(UiValue::Number(1.0)),
            input: Some(UiValue::Number(-2.0)),
            seq: 42,
        };
        let first = serde_json::to_string(&intent).expect("serialize");
        let deserialized: UiIntent = serde_json::from_str(&first).expect("deserialize");
        let second = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(first, second);
        assert_eq!(intent, deserialized);
    }

    #[test]
    fn every_trigger_variant_round_trips() {
        for trigger in [Trigger::Activate, Trigger::Change, Trigger::Commit, Trigger::Delta, Trigger::Drop, Trigger::Submit, Trigger::Abort, Trigger::RepeatLast, Trigger::HoverPreview] {
            let json = serde_json::to_string(&trigger).expect("serialize");
            let back: Trigger = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(trigger, back);
        }
    }
}
//#endregion 🧪️Tests
