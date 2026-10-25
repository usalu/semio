//! 🧭️ Intent address and sequencer: the action capacities, the intent fields, address, bindings, command, stale check and the intent sequencer. See [`super`].

use super::*;

pub const ACTION_QUEUE_ITEM_CAPACITY: usize = 256;

pub const ACTION_BATCH_ITEM_CAPACITY: usize = 16;

pub const ACTION_NODE_CAPACITY: usize = 256;

pub const ACTION_DEPTH_CAPACITY: usize = 32;

pub const ACTION_STRING_BYTE_CAPACITY: usize = 4 * 1024;

pub const ACTION_ITEM_BYTE_CAPACITY: usize = 16 * 1024;

pub const ACTION_QUEUE_BYTE_CAPACITY: usize = 1024 * 1024;

pub const ACTION_CLAIM_CAPACITY: usize = 256;

pub const ACTION_CLAIM_BATCH_CAPACITY: usize = 16;

pub const RETAINED_ACTION_STRING_PAGE_BYTES: usize = 4 * 1024;

pub const RETAINED_ACTION_STRING_BYTE_CAPACITY: usize = 16 * 1024 * 1024;

//#region 🎬️Intent
/// 🏷️ The argument field a trigger's scalar payload travels under — React's `uiInputField`
/// (`🛠️ShellHelpers/🟦️.tsx`). The name belongs to the TRIGGER, not to the control that fired it.
pub const INTENT_VALUE_FIELD: &str = "value";

pub const INTENT_DELTA_FIELD: &str = "delta";

/// 🎯️ Where a gesture happened, as the published document addressed it: the addressing half of
/// [`ui_contract::UiIntent`], stamped onto a retained node by the document reconcile and read back
/// when that node fires. `surface`/`revision` are what [`intent_is_stale`] compares; `node`/`node_key`
/// are the identity a replay or a log still resolves after id churn from an intervening
/// reconciliation. A node with no address (the chrome's immediate-mode widgets, a synthesized
/// recovery panel) is not addressable and never gets an intent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiIntentAddress {
    pub surface: String,
    pub revision: u64,
    pub node: u64,
    pub node_key: String,
}

/// 🔗️ One retained node's dispatch contract: where it lives plus the versioned [`ui_contract::ActionId`]
/// it binds per [`ui_contract::Trigger`]. The bindings list is the wgpu twin of
/// `UiNodeRecord::bindings` — carried whole rather than flattened into one action name per field, so
/// binding PRESENCE is answerable (React's `NumberStepperView` gates `onDelta` on exactly that) and so
/// the action's `scope`/`version` survive, which a stringly [`ActionDescriptor`] drops.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UiIntentBindings {
    pub address: UiIntentAddress,
    pub bindings: Vec<(ui_contract::Trigger, ui_contract::ActionId)>,
}

impl UiIntentBindings {
    pub fn action_for(&self, trigger: ui_contract::Trigger) -> Option<&ui_contract::ActionId> {
        self.bindings.iter().find(|(bound, _)| *bound == trigger).map(|(_, action)| action)
    }

    pub fn binds(&self, trigger: ui_contract::Trigger) -> bool {
        self.bindings.iter().any(|(bound, _)| *bound == trigger)
    }
}

/// 🎬️ One user gesture against one addressed node — the renderer-side twin of
/// [`ui_contract::UiIntent`], carrying the `controller_id` the host's own dispatch seam still needs
/// (the contract's intent resolves its controller from the surface instead; wgpu's host cannot yet).
/// `args` are the binding's AUTHORED arguments and `input` is the trigger's own payload, kept apart
/// exactly as the contract keeps them — [`Self::descriptor`] is the one place they merge.
#[derive(Clone, Debug, PartialEq)]
pub struct UiIntentCommand {
    pub address: UiIntentAddress,
    pub trigger: ui_contract::Trigger,
    pub action: ui_contract::ActionId,
    pub args: Option<DslValue>,
    pub input: Option<DslValue>,
    pub seq: u64,
}

impl UiIntentCommand {
    /// 🌉️ This intent on the plugin action address — React's `uiIntentToActionDescriptor`. A scalar
    /// payload is NAMED by its trigger and merged OVER the authored args (replacing them wholesale
    /// threw the authored `{windowId}` away and muted whole panels); a map payload merges key-wise.
    pub fn descriptor(&self) -> ActionDescriptor {
        ActionDescriptor { controller_id: self.controller_id().to_string(), action: self.action_name(), args: self.payload() }
    }

    /// 🆔️ Who answers this intent — React's `intent.action.scope` (`🛠️ShellHelpers/🟦️.tsx:2054`).
    pub fn controller_id(&self) -> &str {
        self.action.scope.as_str()
    }

    /// 🗣️ The verb as the host addresses it — the bare name at version one, `name@version` beyond it,
    /// so a renderer can never silently invoke a different version of the same verb.
    pub fn action_name(&self) -> String {
        if self.action.version == 1 {
            self.action.name.as_str().to_string()
        } else {
            format!("{}@{}", self.action.name.as_str(), self.action.version)
        }
    }

    pub fn payload(&self) -> Option<DslValue> {
        let Some(input) = self.input.as_ref() else { return self.args.clone() };
        let named = match input {
            DslValue::Object(_) | DslValue::Array(_) => input.clone(),
            scalar => DslValue::Object(vec![(self.input_field().to_string(), scalar.clone())]),
        };
        match (self.args.as_ref(), &named) {
            (Some(DslValue::Object(authored)), DslValue::Object(merged)) => {
                let mut entries: Vec<(String, DslValue)> = authored.iter().filter(|(key, _)| !merged.iter().any(|(name, _)| name == key)).cloned().collect();
                entries.extend(merged.iter().cloned());
                Some(DslValue::Object(entries))
            }
            _ => Some(named),
        }
    }

    pub fn input_field(&self) -> &'static str {
        if matches!(self.trigger, ui_contract::Trigger::Delta) {
            INTENT_DELTA_FIELD
        } else {
            INTENT_VALUE_FIELD
        }
    }
}

/// 🔢️ master.md's own rule ("Stale intents (revision < current − 1) are dropped"), the same
/// predicate `🖌️render/🖱️dispatch/🦀️.rs::is_stale` applies: a gesture recorded more than one revision
/// behind the surface it is now being resolved against was fired at geometry the user never saw.
pub fn intent_is_stale(recorded: u64, current: u64) -> bool {
    current > recorded.saturating_add(1)
}

/// 🔂️ Renderer-monotonic per surface — the `seq` a fired intent carries, minted once per gesture so
/// the receiving side can order and de-duplicate independently of transport delivery order.
#[derive(Clone, Debug, Default)]
pub struct UiIntentSequencer {
    surfaces: Vec<(String, u64)>,
}

impl UiIntentSequencer {
    pub fn next(&mut self, surface: &str) -> u64 {
        match self.surfaces.iter_mut().find(|(name, _)| name == surface) {
            Some((_, seq)) => {
                *seq += 1;
                *seq
            }
            None => {
                self.surfaces.push((surface.to_string(), 1));
                1
            }
        }
    }

    pub fn last(&self, surface: &str) -> u64 {
        self.surfaces.iter().find(|(name, _)| name == surface).map_or(0, |(_, seq)| *seq)
    }

    /// 🧹️ Retires one surface-name scalar without minting an intent.
    pub fn close_step(&mut self) -> bool {
        if let Some((name, _)) = self.surfaces.last_mut() {
            if name.pop().is_some() {
                return false;
            }
            if name.capacity() > 0 {
                *name = String::new();
                return false;
            }
            self.surfaces.pop();
            return false;
        }
        if self.surfaces.capacity() > 0 {
            self.surfaces = Vec::new();
            return false;
        }
        true
    }
}
//#endregion 🎬️Intent
