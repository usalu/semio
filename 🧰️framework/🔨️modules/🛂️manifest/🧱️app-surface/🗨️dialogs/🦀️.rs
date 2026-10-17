//! 🗨️ Dialogs of the app surface: the declared modal form dialog, its choices and the neutral tone rule. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️Dialog
/// 🗨️ A declared modal form dialog: a glass veil covers the screen and an info box (styled
/// identically to every glass overlay box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)
/// presents `args` as a staged form. Submit dispatches `submit_action` with the merged effective
/// args; empty `args` degenerates to a message/confirm dialog. Opened only via
/// `Effect::OpenDialog`; the shell owns open/close as ephemeral chrome state, never the document.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DialogDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub title: LocalizedLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub body: Option<LocalizedLabel>,
    pub args: Vec<ActionArgDef>,
    /// 📇️ References an action owned by the active window kind, dispatched with merged args.
    pub submit_action: ActionRef,
    pub submit_label: LocalizedLabel,
    /// 📇️ Optional active-window action reference dispatched on any dismissal (Escape, veil
    /// click, or the Cancel button).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub cancel_action: Option<ActionRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub cancel_label: Option<LocalizedLabel>,
    /// 🔀️ Further decisions offered beside the submit, in focus order before it. Each is gated on and
    /// dispatches only the args it [`requires`](DialogChoice::requires) (plus the seed context), with
    /// [`DIALOG_CHOICE_ARG`] naming the choice, so one action can serve several choices
    /// (`historyEditCommit{choice}`); the submit alone is gated on the dialog's required args.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<DialogChoice>,
}

/// 🔀️ The arg key a [`DialogChoice`] dispatch carries its own `id` under; reserved in a dialog with choices.
pub const DIALOG_CHOICE_ARG: &str = "choice";

/// 🎛️ One decision button of a [`DialogDefinition`]: a localized `label`, an optional `description` a
/// reader hears with it (`aria-describedby`) and a sighted user reads beside it, the `action` it
/// dispatches, the staged args it `requires` (enabled once they resolve, and the only form args it
/// sends), its visual `tone`, and whether it is `destructive` (danger styling, and an agent lane must
/// ask before taking it).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DialogChoice {
    pub id: String,
    pub label: LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LocalizedLabel>,
    pub action: ActionRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<String>,
    #[serde(default, skip_serializing_if = "is_neutral_tone")]
    #[value(default, skip_serializing_if = "is_neutral_tone")]
    pub tone: semio_framework_ui_contract::Tone,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    #[value(default, skip_serializing_if = "std::ops::Not::not")]
    pub destructive: bool,
}

fn is_neutral_tone(tone: &semio_framework_ui_contract::Tone) -> bool {
    *tone == semio_framework_ui_contract::Tone::Neutral
}

impl DialogChoice {
    pub fn new(id: impl Into<String>, label: impl Into<LocalizedLabel>, action: ActionRef) -> Self {
        Self { id: id.into(), label: label.into(), description: None, action, requires: Vec::new(), tone: semio_framework_ui_contract::Tone::Neutral, destructive: false }
    }

    /// 🔗️ The staged args this choice consumes: it is enabled once they resolve and sends only them.
    pub fn requires(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.requires = args.into_iter().map(Into::into).collect();
        self
    }

    /// 🚧️ The args of [`requires`](Self::requires) still unresolved in `effective` (absent, blank, or off
    /// their current choice set) — the choice is enabled exactly when this is empty.
    pub fn unresolved_args(&self, defs: &[ActionArgDef], effective: &DslValue) -> Vec<String> {
        let required: Vec<ActionArgDef> = defs.iter().filter(|def| self.requires.contains(&def.id)).map(|def| ActionArgDef { required: true, ..def.clone() }).collect();
        unresolved_action_args(&required, effective)
    }

    /// 📤️ The dispatch args: the seed context of `effective` (keys no arg of `defs` declares), the args this
    /// choice requires, and [`DIALOG_CHOICE_ARG`] naming it.
    pub fn dispatch_args(&self, defs: &[ActionArgDef], effective: &DslValue) -> DslValue {
        let mut entries = effective.as_object().map(<[_]>::to_vec).unwrap_or_default();
        entries.retain(|(key, _)| key != DIALOG_CHOICE_ARG && (self.requires.contains(key) || !defs.iter().any(|def| def.id == *key)));
        entries.push((DIALOG_CHOICE_ARG.to_string(), DslValue::String(self.id.clone())));
        DslValue::Object(entries)
    }

    /// 📝️ The consequence a reader hears with the button and a sighted user reads beside it.
    pub fn description(mut self, description: impl Into<LocalizedLabel>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 🎨️ The button's colour role.
    pub fn tone(mut self, tone: semio_framework_ui_contract::Tone) -> Self {
        self.tone = tone;
        self
    }

    /// ⚠️ Marks the choice destructive: danger styling, and an agent must ask before taking it.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }
}

// 🚫️async: E1 — pure builder methods (self-mutation only, zero suspension points), reverted
// per R9: `catch_unwind` sync-closure test consumers are language-barred from async.
impl DialogDefinition {
    pub fn new(id: impl Into<String>, title: impl Into<LocalizedLabel>, submit_action: ActionRef) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: None,
            args: Vec::new(),
            submit_action,
            // 🌐️ "OK" is identical in both locales — a real (not placeholder) translation choice.
            submit_label: LocalizedLabel::native("OK", "OK"),
            cancel_action: None,
            cancel_label: None,
            choices: Vec::new(),
        }
    }

    /// 🔀️ Appends one decision offered beside the submit.
    pub fn choice(mut self, choice: DialogChoice) -> Self {
        self.choices.push(choice);
        self
    }

    /// 🧐️ Choice ids are non-empty and unique, a choice requires only declared args, and no staged arg
    /// shadows [`DIALOG_CHOICE_ARG`].
    pub fn validate_choices(&self) -> Result<(), String> {
        let mut ids = std::collections::BTreeSet::new();
        for choice in &self.choices {
            if choice.id.trim().is_empty() {
                return Err("choice id must be non-empty".to_string());
            }
            if !ids.insert(choice.id.as_str()) {
                return Err(format!("duplicate choice id {}", choice.id));
            }
            if let Some(arg) = choice.requires.iter().find(|arg| !self.args.iter().any(|def| def.id == **arg)) {
                return Err(format!("choice {} requires undeclared arg {arg}", choice.id));
            }
        }
        if !self.choices.is_empty() && self.args.iter().any(|arg| arg.id == DIALOG_CHOICE_ARG) {
            return Err(format!("arg {DIALOG_CHOICE_ARG} is reserved in a dialog with choices"));
        }
        Ok(())
    }

    /// 📝️ Attaches explanatory body text shown below the title.
    pub fn body(mut self, body: impl Into<LocalizedLabel>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// 🧾️ Attaches the staged-form field declarations.
    pub fn args(mut self, args: Vec<ActionArgDef>) -> Self {
        self.args = args;
        self
    }

    /// ✅️ Overrides the submit button label (default "OK").
    pub fn submit_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// ❌️ Overrides the cancel button label (default "Cancel", applied by the renderer).
    pub fn cancel_label(mut self, label: impl Into<LocalizedLabel>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }

    /// 🚪️ Declares an action dispatched on any dismissal (Escape, veil click, Cancel button).
    pub fn on_cancel(mut self, action: ActionRef) -> Self {
        self.cancel_action = Some(action);
        self
    }
}
//#endregion 🔖️Dialog
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
