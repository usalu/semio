//! 🎚️ `shell-chrome-state` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
/// ⏱️ The hover-armed tooltip candidate — (re)armed the first frame `control_id` becomes hovered,
/// painted once `CHROME_TOOLTIP_DELAY_MS` elapses. Dismissed immediately on hover-out: this crate has no
/// animation-clock scaffolding anywhere (`engine::Ui::needs_frame`'s own doc comment admits the same gap
/// for animations generally), matching `OverlayKind::Tooltip::dismiss_policy`'s own documented admission
/// that hover-out-delay isn't actually debounced yet.
#[derive(Clone, Debug)]
pub struct ChromeTooltipHover {
    pub control_id: String,
    pub anchor_x: f32,
    pub anchor_y: f32,
    pub started_ms: f64,
}

//#region 🧯️TransientNoticeAndAgentOverlays
/// 🧯️ The single live notice. `severity` is `semio_framework::Severity` — the same frozen four-level
/// vocabulary React's `TransientNotice.kind` carries, not a renderer-local enum — and `code` is the
/// frozen `Fault.code` (`viewer.read-only`, `app.command.rejected`, …) a probe asserts on, mirroring
/// React's `data-notice-code` attribute.
#[derive(Clone, Debug, PartialEq)]
pub struct ShellTransientNotice {
    pub message: String,
    pub severity: semio_framework::Severity,
    pub code: Option<String>,
    pub shown_at_ms: f64,
}

impl ShellChromeBuildState {
    /// 🧯️ Shows a non-blocking, auto-dismissing notice, replacing whatever is currently showing —
    /// `showTransientNotice`'s own contract ("only one can render at a time").
    pub fn show_transient_notice(&mut self, message: impl Into<String>, severity: semio_framework::Severity, code: Option<&str>, now_ms: f64) {
        self.transient_notice = Some(ShellTransientNotice { message: message.into(), severity, code: code.map(str::to_string), shown_at_ms: now_ms });
    }

    pub fn dismiss_transient_notice(&mut self) {
        self.transient_notice = None;
    }

    /// ⏱️ Clears the notice once its 4 s deadline passed; returns `true` when one is still showing.
    pub fn expire_transient_notice(&mut self, now_ms: f64) -> bool {
        if self.transient_notice.as_ref().is_some_and(|notice| transient_notice_expired(notice, now_ms)) {
            self.transient_notice = None;
        }
        self.transient_notice.is_some()
    }

    /// ⏰️ Earliest wall-clock deadline that can change Shell-owned chrome without input.
    pub fn next_deadline_ms(&self, now_ms: f64) -> Option<f64> {
        let tooltip = self.tooltip_hover.as_ref().filter(|hover| !chrome_tooltip_ready(hover, now_ms)).map(|hover| hover.started_ms + CHROME_TOOLTIP_DELAY_MS);
        let notice = self.transient_notice.as_ref().map(|notice| notice.shown_at_ms + TRANSIENT_NOTICE_AUTO_DISMISS_MS);
        match (tooltip, notice) {
            (Some(tooltip), Some(notice)) => Some(tooltip.min(notice)),
            (Some(deadline), None) | (None, Some(deadline)) => Some(deadline),
            (None, None) => None,
        }
    }
}

/// 📣️ The structured guest refusal behind one dispatch-fault string (design §20.12): the guest's own `Fault` — its code,
/// severity and the `params` a notice fills its placeholders from — and the refusing app's published notices. Recorded where a
/// program call refuses ([`ShellState::refused_guest_call`]), taken by the funnel ([`ShellState::note_dispatch_fault`]) when
/// the string it classifies names the same code, cleared when the next dispatch starts.
#[derive(Clone, Debug)]
pub struct RefusedGuestFault {
    pub fault: semio_framework::Fault,
    pub notices: Vec<semio_framework::FaultNoticeDefinition>,
}

impl ChromeState {
    /// 🧯️ The shell-wide entry point every non-fatal fault path calls — the wgpu twin of React's
    /// `showTransientNoticeRef.current(...)`.
    pub fn show_transient_notice(&mut self, message: impl Into<String>, severity: semio_framework::Severity, code: Option<&str>) {
        let now = chrome_now_ms();
        self.chrome_build.show_transient_notice(message, severity, code, now);
    }

    /// 🧯️ Read side for probes and tests — the live notice, if any.
    pub fn transient_notice(&self) -> Option<&ShellTransientNotice> {
        self.chrome_build.transient_notice.as_ref()
    }

    /// 🌉️ The agent bridge consumer this shell drives — a transport hands it one socket message at a
    /// time and drains [`crate::agent_bridge::AgentBridgeState::take_outbox_encoded`] afterwards.
    pub fn apply_agent_bridge_frame(&mut self, bytes: &[u8]) -> Result<(), semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::BridgeFrameFault> {
        let now = chrome_now_ms();
        self.chrome_build.agent.apply_encoded_frame(bytes, now)
    }

    /// 🌉️ Frames the transport must write back (approval decisions, hello, ping, bye).
    pub fn take_agent_bridge_outbox(&mut self) -> Vec<Vec<u8>> {
        self.chrome_build.agent.take_outbox_encoded()
    }

    /// 🚦️ Live agent status/presence — what the chat panel header's indicator paints.
    pub fn agent_presence_tone(&self) -> semio_framework_os_renderer_wgpu_agent_elements::agent_presence::AgentPresenceTone {
        semio_framework_os_renderer_wgpu_agent_elements::agent_presence::agent_presence_tone(self.chrome_build.agent.status, &self.chrome_build.agent.presence)
    }
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#region 🗨️ChromeDialog
/// 🗨️ The wgpu twin of React's `UIDialog`: one declared `DialogDefinition` resolved to the active locale —
/// title, body, its staged fields, its decision choices, the submit and Cancel. Every button dispatches
/// through the generic chrome hit pipeline with the args effective in that frame (`effective_action_args`
/// over the staged values and the `Effect::OpenDialog` seed), a choice adds its own id
/// (`DialogChoice::dispatch_args`, gated on the args it requires), and every dismissal (Cancel, Escape, a veil click) sends the cancel
/// action. Focus walks the fields (a reference field's chips right after it), Cancel, the choices and the
/// submit in that order — the order `🛂️manifest/🧫️fixtures/🧫️dialog-choices` pins for every renderer.
///
/// 🎛️ Every `ActionArgControl` has its own staged field: text, a number (a stepper adds −/+ buttons), a
/// slider or dial with its detents painted as ticks, a select, a segmented choice of pressed buttons, a
/// toggle, one number field per vector axis sharing the vector's bounds, precision and detents, a colour
/// (the `color_input` recipe: a swatch beside its hex text and, with alpha, an opacity slider), and a
/// reference list of removable chips with a "use current selection" button reading the shell's live
/// interaction selection.
#[derive(Clone, Debug)]
pub struct ChromeDialogRequest {
    pub id: String,
    pub controller_id: String,
    pub title: String,
    pub body: String,
    pub defs: Vec<semio_framework::ActionArgDef>,
    pub fields: Vec<ChromeDialogField>,
    pub choices: Vec<ChromeDialogChoice>,
    pub confirm_label: String,
    pub confirm_action: String,
    pub cancel_label: String,
    pub cancel_action: Option<String>,
    pub seed: Option<DslValue>,
    pub staged: Vec<(String, DslValue)>,
    pub focus: usize,
    pub labels: ChromeDialogLabels,
}

/// 🏷️ The dialog chrome's own copy, resolved once per opening from exhaustive `LocalizedLabel`s.
#[derive(Clone, Debug)]
struct ChromeDialogLabels {
    pub on: String,
    off: String,
    pub choose: String,
    pub use_selection: String,
    pub remove: String,
    pub empty: String,
    pub decrease: String,
    pub increase: String,
}

/// ✍️ One staged field: its arg id, resolved label, control and the draft text a text/number field edits.
#[derive(Clone, Debug)]
struct ChromeDialogField {
    pub id: String,
    pub label: String,
    pub required: bool,
    pub kind: ChromeDialogFieldKind,
    pub draft: String,
}

/// 🎛️ The staged controls this renderer paints — one per `ActionArgControl`; a vector arg becomes one
/// `Axis` field per component, so every component is its own labelled focus stop, and a colour with alpha
/// a `Color` field plus a `Slider` over its fourth `component` (index, dims, the value when absent).
#[derive(Clone, Debug)]
pub enum ChromeDialogFieldKind {
    Text,
    Number { step: f64, precision: Option<u16>, min: Option<f64>, max: Option<f64>, stepper: bool },
    Slider { min: f64, max: f64, step: f64, precision: Option<u16>, unit: Option<String>, snaps: Vec<f64>, component: Option<(usize, usize, f64)> },
    Select { options: Vec<(String, String)> },
    Segmented { options: Vec<(String, String)> },
    Toggle,
    Axis { axis: usize, dims: usize, step: f64, precision: Option<u16>, min: Option<f64>, max: Option<f64>, snaps: Vec<f64> },
    Color { alpha: bool },
    Reference { domain: Option<String>, granularity: Option<String>, many: bool },
}

/// 🔀️ One decision button with its resolved label and consequence text.
#[derive(Clone, Debug)]
struct ChromeDialogChoice {
    pub choice: semio_framework::DialogChoice,
    pub label: String,
    pub description: Option<String>,
}

impl ChromeDialogRequest {
    /// 🗨️ Resolves `dialog` against `terminology`/`locale`; `seed` is the `Effect::OpenDialog` args.
    pub fn from_definition(controller_id: &str, dialog: &semio_framework::DialogDefinition, seed: Option<DslValue>, terminology: Terminology, locale: Locale) -> Self {
        use semio_framework::ActionArgControl as Control;
        let resolve = |label: &LocalizedLabel| label.resolve(terminology, locale).to_string();
        let positive = |step: Option<f64>| step.filter(|step| step.is_finite() && *step > 0.0).unwrap_or(1.0);
        let digits = |precision: Option<u32>| precision.map(|precision| u16::try_from(precision).unwrap_or(ui_contract::UI_NUMBER_PRECISION_MAX));
        let initial = semio_framework::effective_action_args(&dialog.args, &semio_framework_value::DslValue::Object(Vec::new()), seed.as_ref());
        let fields = dialog
            .args
            .iter()
            .flat_map(|def| {
                let label = resolve(&def.label);
                let kinds: Vec<(String, ChromeDialogFieldKind)> = match def.control() {
                    Control::Number { min, max, step, precision, .. } => vec![(label, ChromeDialogFieldKind::Number { step: positive(step), precision: digits(precision), min, max, stepper: false })],
                    Control::Stepper { min, max, step, precision, .. } => vec![(label, ChromeDialogFieldKind::Number { step: positive(step), precision: digits(precision), min, max, stepper: true })],
                    Control::Slider { min, max, step, precision, unit, snaps, .. } | Control::Dial { min, max, step, precision, unit, snaps, .. } => {
                        let snaps = if ui_contract::snaps_are_valid(snaps.iter().copied(), min, max) { snaps } else { Vec::new() };
                        vec![(label, ChromeDialogFieldKind::Slider { min, max: max.max(min), step: positive(step), precision: digits(precision), unit, snaps, component: None })]
                    }
                    Control::Select { options } => vec![(label, ChromeDialogFieldKind::Select { options: options.iter().map(|option| (option.value.clone(), resolve(&option.label))).collect() })],
                    Control::Segmented { options } => vec![(label, ChromeDialogFieldKind::Segmented { options: options.iter().map(|option| (option.value.clone(), resolve(&option.label))).collect() })],
                    Control::Toggle => vec![(label, ChromeDialogFieldKind::Toggle)],
                    Control::Vector { dims, step, precision, min, max, snaps, .. } => {
                        let dims = dims as usize;
                        let precision = digits(precision);
                        let step = step.filter(|step| step.is_finite() && *step > 0.0).or_else(|| precision.map(|precision| 10_f64.powi(-i32::from(precision)))).unwrap_or(1.0);
                        let snaps = if ui_contract::snaps_are_valid(snaps.iter().copied(), min.unwrap_or(f64::NEG_INFINITY), max.unwrap_or(f64::INFINITY)) { snaps } else { Vec::new() };
                        (0..dims).map(|axis| (format!("{label} {}", chrome_dialog_axis_name(axis)), ChromeDialogFieldKind::Axis { axis, dims, step, precision, min, max, snaps: snaps.clone() })).collect()
                    }
                    Control::Color { alpha } => {
                        let opacity = LocalizedLabel::native("Opacity", "Deckkraft").resolve(terminology, locale).to_string();
                        let mut kinds = vec![(label.clone(), ChromeDialogFieldKind::Color { alpha })];
                        if alpha {
                            kinds.push((format!("{label} {opacity}"), ChromeDialogFieldKind::Slider { min: 0.0, max: 1.0, step: 0.01, precision: Some(2), unit: None, snaps: Vec::new(), component: Some((3, 4, 1.0)) }));
                        }
                        kinds
                    }
                    Control::Reference { domain, granularity, many, .. } => vec![(label, ChromeDialogFieldKind::Reference { domain, granularity, many })],
                    _ => vec![(label, ChromeDialogFieldKind::Text)],
                };
                let value = initial.get(&def.id).cloned();
                kinds.into_iter().map(move |(label, kind)| {
                    let draft = match (&kind, value.as_ref()) {
                        (ChromeDialogFieldKind::Number { precision, .. }, Some(value)) => value.as_f64().map_or_else(String::new, |value| chrome_dialog_number_text(value, *precision)),
                        (ChromeDialogFieldKind::Axis { axis, precision, .. }, Some(value)) => value.as_array().and_then(|tuple| tuple.get(*axis)).and_then(semio_framework_value::DslValue::as_f64).map_or_else(String::new, |value| chrome_dialog_number_text(value, *precision)),
                        (ChromeDialogFieldKind::Color { alpha }, Some(value)) => ui_contract::ui_color_hex(&value.as_array().map(|components| components.iter().filter_map(semio_framework_value::DslValue::as_f64).collect::<Vec<_>>()).unwrap_or_default(), *alpha),
                        (ChromeDialogFieldKind::Text, Some(value)) => value.as_str().unwrap_or_default().to_string(),
                        _ => String::new(),
                    };
                    ChromeDialogField { id: def.id.clone(), label, required: def.required, kind, draft }
                })
            })
            .collect();
        let native = |en: &str, de: &str| LocalizedLabel::native(en, de).resolve(terminology, locale).to_string();
        Self {
            id: dialog.id.clone(),
            controller_id: controller_id.to_string(),
            title: resolve(&dialog.title),
            body: dialog.body.as_ref().map(resolve).unwrap_or_default(),
            defs: dialog.args.clone(),
            fields,
            choices: dialog.choices.iter().map(|choice| ChromeDialogChoice { choice: choice.clone(), label: resolve(&choice.label), description: choice.description.as_ref().map(resolve) }).collect(),
            confirm_label: resolve(&dialog.submit_label),
            confirm_action: dialog.submit_action.as_str().to_string(),
            cancel_label: dialog.cancel_label.as_ref().map_or_else(|| native("Cancel", "Abbrechen"), resolve),
            cancel_action: dialog.cancel_action.as_ref().map(|action| action.as_str().to_string()),
            seed,
            staged: Vec::new(),
            focus: 0,
            labels: ChromeDialogLabels {
                on: native("On", "Ein"),
                off: native("Off", "Aus"),
                choose: native("Choose…", "Auswählen…"),
                use_selection: LocalizedLabel::native("Use current selection", "Aktuelle Auswahl verwenden").resolve(terminology, locale).to_string(),
                remove: LocalizedLabel::native("Remove {item}", "{item} entfernen").resolve(terminology, locale).to_string(),
                empty: LocalizedLabel::native("Nothing selected", "Nichts ausgewählt").resolve(terminology, locale).to_string(),
                decrease: LocalizedLabel::native("Decrease", "Verringern").resolve(terminology, locale).to_string(),
                increase: LocalizedLabel::native("Increase", "Erhöhen").resolve(terminology, locale).to_string(),
            },
        }
    }

    /// 🧮️ The args every button dispatches this frame: staged values over the seed and the defaults, and a colour
    /// field's edited hex (`ui_contract::parse_ui_color_hex`) over its colour — keeping the alpha when the text carries
    /// none, so a hex passing through a short form while it is typed never loses the opacity. An unedited hex (the
    /// one the colour prints as) changes nothing, so an untouched colour keeps its exact components.
    pub fn effective(&self) -> DslValue {
        let mut effective = semio_framework::effective_action_args(&self.defs, &semio_framework_value::DslValue::Object(self.staged.clone()), self.seed.as_ref());
        for field in &self.fields {
            let ChromeDialogFieldKind::Color { alpha } = field.kind else { continue };
            let base: Vec<f64> = effective.get(&field.id).and_then(|value| value.as_array().map(|components| components.iter().filter_map(semio_framework_value::DslValue::as_f64).collect())).unwrap_or_default();
            let Some(parsed) = ui_contract::parse_ui_color_hex(&field.draft).filter(|_| field.draft != ui_contract::ui_color_hex(&base, alpha)) else { continue };
            let typed_alpha = matches!(field.draft.trim().trim_start_matches('#').len(), 4 | 8);
            let rgba = [parsed[0], parsed[1], parsed[2], if typed_alpha { parsed[3] } else { base.get(3).copied().unwrap_or(1.0) }];
            let value = semio_framework_value::DslValue::Array(rgba[..if alpha { 4 } else { 3 }].iter().map(|component| semio_framework_value::DslValue::float(*component)).collect());
            if let semio_framework_value::DslValue::Object(entries) = &mut effective {
                match entries.iter_mut().find(|(key, _)| *key == field.id) {
                    Some(entry) => entry.1 = value,
                    None => entries.push((field.id.clone(), value)),
                }
            }
        }
        effective
    }

    /// 🛑️ Whether a required field is still unset or a choice left its catalog — the submit gate.
    pub fn resolved(&self) -> bool {
        semio_framework::unresolved_action_args(&self.defs, &self.effective()).is_empty()
    }

    /// 🎯️ Every focus stop in visiting order: each field (a reference field followed by its chips), Cancel, the
    /// choices, the submit.
    pub fn stops(&self) -> Vec<ChromeDialogStop> {
        let mut stops = Vec::new();
        for (index, field) in self.fields.iter().enumerate() {
            stops.push(ChromeDialogStop::Field(index));
            if matches!(field.kind, ChromeDialogFieldKind::Reference { .. }) {
                stops.extend((0..self.reference_ids(index).len()).map(|chip| ChromeDialogStop::Chip(index, chip)));
            }
        }
        stops.push(ChromeDialogStop::Cancel);
        stops.extend((0..self.choices.len()).map(ChromeDialogStop::Choice));
        stops.push(ChromeDialogStop::Confirm);
        stops
    }

    pub fn focused(&self) -> ChromeDialogStop {
        let stops = self.stops();
        stops[self.focus.min(stops.len() - 1)]
    }

    pub fn move_focus(&mut self, backwards: bool) {
        let count = self.stops().len();
        self.focus = if backwards { (self.focus + count - 1) % count } else { (self.focus + 1) % count };
    }

    pub fn focus_stop(&mut self, stop: ChromeDialogStop) {
        if let Some(index) = self.stops().iter().position(|candidate| *candidate == stop) {
            self.focus = index;
        }
    }

    pub fn stage(&mut self, id: &str, value: DslValue) {
        match self.staged.iter_mut().find(|(key, _)| key == id) {
            Some(entry) => entry.1 = value,
            None => self.staged.push((id.to_string(), value)),
        }
    }

    pub fn field_value(&self, index: usize) -> Option<DslValue> {
        self.effective().get(&self.fields.get(index)?.id).cloned()
    }

    /// ✍️ Replaces a text/number/axis/colour field's draft and stages the value it means (`Null` for an unreadable
    /// number); an axis splices its component into the vector, clamped to the vector's bounds and rounded to its
    /// precision, and a colour's draft is read by [`Self::effective`].
    pub fn set_draft(&mut self, index: usize, draft: String) {
        let Some(field) = self.fields.get(index) else { return };
        let number = draft.trim().parse::<f64>().ok().filter(|value| value.is_finite());
        let value = match field.kind {
            ChromeDialogFieldKind::Number { precision, .. } => number.map_or(semio_framework_value::DslValue::Null, |value| semio_framework_value::DslValue::float(precision.map_or(value, |precision| ui_contract::round_ui_number(value, precision)))),
            ChromeDialogFieldKind::Axis { axis, dims, precision, min, max, .. } => {
                let mut tuple: Vec<DslValue> = self.field_value(index).and_then(|value| value.as_array().map(<[DslValue]>::to_vec)).unwrap_or_default();
                tuple.resize(dims, semio_framework_value::DslValue::float(0.0));
                tuple[axis] = number.map_or(semio_framework_value::DslValue::Null, |value| semio_framework_value::DslValue::float(precision.map_or(value, |precision| ui_contract::round_ui_number(value, precision)).max(min.unwrap_or(f64::NEG_INFINITY)).min(max.unwrap_or(f64::INFINITY))));
                semio_framework_value::DslValue::Array(tuple)
            }
            ChromeDialogFieldKind::Color { .. } => {
                self.fields[index].draft = draft;
                return;
            }
            _ => semio_framework_value::DslValue::String(draft.clone()),
        };
        let id = field.id.clone();
        self.fields[index].draft = draft;
        self.stage(&id, value);
    }

    /// 🔀️ Flips a toggle field.
    pub fn flip(&mut self, index: usize) {
        let on = matches!(self.field_value(index), Some(DslValue::Bool(true)));
        if let Some(id) = self.fields.get(index).filter(|field| matches!(field.kind, ChromeDialogFieldKind::Toggle)).map(|field| field.id.clone()) {
            self.stage(&id, semio_framework_value::DslValue::Bool(!on));
        }
    }

    /// 📋️ A select or segmented field's options.
    pub fn options(&self, index: usize) -> &[(String, String)] {
        match self.fields.get(index).map(|field| &field.kind) {
            Some(ChromeDialogFieldKind::Select { options } | ChromeDialogFieldKind::Segmented { options }) => options,
            _ => &[],
        }
    }

    /// 🔽️ Moves a select or segmented field to its next (`forward`) or previous option, wrapping.
    pub fn cycle(&mut self, index: usize, forward: bool) {
        let options = self.options(index);
        if options.is_empty() {
            return;
        }
        let current = self.field_value(index).and_then(|value| value.as_str().map(str::to_string));
        let position = current.and_then(|current| options.iter().position(|(value, _)| *value == current));
        let next = match (position, forward) {
            (None, true) => 0,
            (None, false) => options.len() - 1,
            (Some(position), true) => (position + 1) % options.len(),
            (Some(position), false) => (position + options.len() - 1) % options.len(),
        };
        self.choose(index, next);
    }

    /// 🔘️ Picks one option of a select or segmented field.
    pub fn choose(&mut self, index: usize, option: usize) {
        let Some(value) = self.options(index).get(option).map(|(value, _)| value.clone()) else { return };
        let id = self.fields[index].id.clone();
        self.stage(&id, semio_framework_value::DslValue::String(value));
    }

    /// 🔢️ Steps a number or axis field through the shared keyboard law (`ui_contract::ui_number_key_value`): a
    /// sign walks one rung of its step ladder, a page (`page`) jumps to the adjacent detent of an axis or walks ten
    /// rungs, inside its bounds.
    pub fn nudge_by(&mut self, index: usize, sign: f64, page: bool) {
        let Some(field) = self.fields.get(index) else { return };
        let (step, precision, min, max, snaps) = match &field.kind {
            ChromeDialogFieldKind::Number { step, precision, min, max, .. } => (*step, *precision, *min, *max, Vec::new()),
            ChromeDialogFieldKind::Axis { step, precision, min, max, snaps, .. } => (*step, *precision, *min, *max, snaps.clone()),
            _ => return,
        };
        let current = field.draft.trim().parse::<f64>().ok().filter(|value| value.is_finite()).unwrap_or(0.0);
        let key = match (page, sign > 0.0) {
            (false, true) => ui_contract::SliderKey::Increment,
            (false, false) => ui_contract::SliderKey::Decrement,
            (true, true) => ui_contract::SliderKey::PageUp,
            (true, false) => ui_contract::SliderKey::PageDown,
        };
        self.set_draft(index, chrome_dialog_number_text(ui_contract::ui_number_key_value(current, min, max, step, precision, None, snaps, key, false), precision));
    }

    /// 🔢️ Steps a number or axis field one rung of its step ladder.
    pub fn nudge(&mut self, index: usize, sign: f64) {
        self.nudge_by(index, sign, false);
    }

    /// 🎚️ A slider field's current value, on its travel — a component slider's component of its vector.
    pub fn slider_value(&self, index: usize) -> Option<f64> {
        let ChromeDialogFieldKind::Slider { min, max, component, .. } = self.fields.get(index)?.kind else { return None };
        let value = self.field_value(index);
        let current = match component {
            Some((at, _, absent)) => value.and_then(|value| value.as_array().and_then(|components| components.get(at)).and_then(semio_framework_value::DslValue::as_f64)).unwrap_or(absent),
            None => value.and_then(|value| value.as_f64()).unwrap_or(min),
        };
        Some(current.clamp(min, max))
    }

    /// 🎚️ Sets a slider field, clamped to its travel and rounded to its precision — a component slider splices it
    /// into its vector.
    pub fn set_slider(&mut self, index: usize, value: f64) {
        let Some(ChromeDialogFieldKind::Slider { min, max, precision, component, .. }) = self.fields.get(index).map(|field| field.kind.clone()) else { return };
        let value = semio_framework_value::DslValue::float(precision.map_or(value.clamp(min, max), |precision| ui_contract::round_ui_number(value.clamp(min, max), precision)));
        let staged = match component {
            Some((at, dims, _)) => {
                let mut tuple: Vec<DslValue> = self.field_value(index).and_then(|current| current.as_array().map(<[DslValue]>::to_vec)).unwrap_or_default();
                tuple.resize(dims, semio_framework_value::DslValue::float(0.0));
                tuple[at] = value;
                semio_framework_value::DslValue::Array(tuple)
            }
            None => value,
        };
        let id = self.fields[index].id.clone();
        let printed = component.and_then(|_| staged.as_array().map(|components| components.iter().filter_map(semio_framework_value::DslValue::as_f64).collect::<Vec<_>>()));
        self.stage(&id, staged);
        if let Some(components) = printed {
            for field in self.fields.iter_mut().filter(|field| field.id == id) {
                if let ChromeDialogFieldKind::Color { alpha } = field.kind {
                    field.draft = ui_contract::ui_color_hex(&components, alpha);
                }
            }
        }
    }

    /// ⌨️ One slider key through the shared keyboard law (`ui_contract::slider_key_value`, pinned by
    /// `🧫️number-controls` for every renderer): arrows walk the step ladder and never snap (`large` — a `Shift`
    /// chord — walks ten rungs), PageUp/PageDown jump to the adjacent detent (ten rungs without one), Home/End go
    /// to the ends. `false` for a key a slider does not take.
    pub fn slide(&mut self, index: usize, action: &ui_wgpu::wgpu::KeyAction, large: bool) -> bool {
        use ui_contract::SliderKey;
        use ui_wgpu::wgpu::KeyAction;
        let (Some(ChromeDialogFieldKind::Slider { min, max, step, precision, snaps, .. }), Some(value)) = (self.fields.get(index).map(|field| field.kind.clone()), self.slider_value(index)) else { return false };
        let key = match action {
            KeyAction::ArrowLeft | KeyAction::ArrowDown => SliderKey::Decrement,
            KeyAction::ArrowRight | KeyAction::ArrowUp => SliderKey::Increment,
            KeyAction::PageUp => SliderKey::PageUp,
            KeyAction::PageDown => SliderKey::PageDown,
            KeyAction::Home => SliderKey::Home,
            KeyAction::End => SliderKey::End,
            _ => return false,
        };
        self.set_slider(index, ui_contract::slider_key_value(value, min, max, step, precision, None, snaps, key, large));
        true
    }

    /// 👆️ A pointer at `share` of a slider's rail: the shared detent law of every renderer.
    pub fn point_slider(&mut self, index: usize, share: f64) {
        let Some(ChromeDialogFieldKind::Slider { min, max, step, snaps, .. }) = self.fields.get(index).map(|field| field.kind.clone()) else { return };
        self.set_slider(index, ui_contract::slider_pointer_value(min + share.clamp(0.0, 1.0) * (max - min), min, max, step, snaps, ui_contract::UiNumberScale::Linear));
    }

    /// 🧷️ A reference field's referenced ids, in order.
    pub fn reference_ids(&self, index: usize) -> Vec<String> {
        match self.field_value(index) {
            Some(semio_framework_value::DslValue::Array(ids)) => ids.iter().filter_map(|id| id.as_str().map(str::to_string)).collect(),
            Some(semio_framework_value::DslValue::String(id)) if !id.is_empty() => vec![id],
            _ => Vec::new(),
        }
    }

    /// 🧷️ Stages `ids` into a reference field — a list when it takes many, else the first id.
    fn stage_references(&mut self, index: usize, ids: Vec<String>) {
        let Some(ChromeDialogFieldKind::Reference { many, .. }) = self.fields.get(index).map(|field| field.kind.clone()) else { return };
        let value = if many { semio_framework_value::DslValue::Array(ids.into_iter().map(semio_framework_value::DslValue::String).collect()) } else { ids.into_iter().next().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String) };
        let id = self.fields[index].id.clone();
        self.stage(&id, value);
    }

    /// 🎯️ "Use current selection": the live selection of the field's domain replaces its references.
    pub fn use_selection(&mut self, index: usize, selection: &HashMap<String, semio_framework::DomainSelection>) {
        let Some(ChromeDialogFieldKind::Reference { domain, granularity, .. }) = self.fields.get(index).map(|field| field.kind.clone()) else { return };
        self.stage_references(index, reference_selection_ids(selection, domain.as_deref(), granularity.as_deref()));
    }

    /// ✖️ Removes one reference; focus stays on the chip that takes its place, else returns to the field.
    pub fn remove_chip(&mut self, index: usize, chip: usize) {
        let mut ids = self.reference_ids(index);
        if chip >= ids.len() {
            return;
        }
        ids.remove(chip);
        let left = ids.len();
        self.stage_references(index, ids);
        self.focus_stop(if left == 0 { ChromeDialogStop::Field(index) } else { ChromeDialogStop::Chip(index, chip.min(left - 1)) });
    }

    /// ▶️ The action a stop dispatches now: `None` for a field or chip, for a gated submit or choice, and for a
    /// Cancel without a declared cancel action.
    pub fn action(&self, stop: ChromeDialogStop) -> Option<ActionDescriptor> {
        let descriptor = |action: &str, args: Option<DslValue>| ActionDescriptor { controller_id: self.controller_id.clone(), action: action.to_string(), args };
        match stop {
            ChromeDialogStop::Field(_) | ChromeDialogStop::Chip(..) => None,
            ChromeDialogStop::Cancel => self.cancel_action.as_deref().map(|action| descriptor(action, None)),
            ChromeDialogStop::Choice(index) => {
                let choice = &self.choices.get(index)?.choice;
                let effective = self.effective();
                choice.unresolved_args(&self.defs, &effective).is_empty().then(|| descriptor(choice.action.as_str(), Some(choice.dispatch_args(&self.defs, &effective))))
            }
            ChromeDialogStop::Confirm => self.resolved().then(|| descriptor(&self.confirm_action, Some(self.effective()))),
        }
    }

    /// 🔏️ The key of the dialog's own accessibility node; every control it paints registers under `{key}.…`.
    pub fn accessibility_key(&self) -> String {
        format!("shell.dialog.{}", self.id)
    }

    /// 🧿️ The painted control `key` names: a focus stop, or a segment (`Some(option)`) or stepper button (the sign)
    /// of a field.
    pub fn accessibility_target(&self, key: &str) -> Option<(ChromeDialogStop, Option<usize>, f64)> {
        if let Some(stop) = self.stops().into_iter().find(|stop| self.control_id(*stop) == key) {
            return Some((stop, None, 0.0));
        }
        (0..self.fields.len()).find_map(|index| {
            let part = key.strip_prefix(&self.control_id(ChromeDialogStop::Field(index)))?;
            match part {
                ".decrement" => Some((ChromeDialogStop::Field(index), None, -1.0)),
                ".increment" => Some((ChromeDialogStop::Field(index), None, 1.0)),
                _ => part.strip_prefix(".option.").and_then(|value| self.options(index).iter().position(|(option, _)| option == value)).map(|option| (ChromeDialogStop::Field(index), Some(option), 0.0)),
            }
        })
    }

    /// 🏷️ The chrome control id a stop registers its hit and accessible name under.
    pub fn control_id(&self, stop: ChromeDialogStop) -> String {
        match stop {
            ChromeDialogStop::Field(index) => match self.fields[index].kind {
                ChromeDialogFieldKind::Axis { axis, .. } => format!("shell.dialog.{}.field.{}.{}", self.id, self.fields[index].id, chrome_dialog_axis_name(axis)),
                ChromeDialogFieldKind::Slider { component: Some(_), .. } => format!("shell.dialog.{}.field.{}.alpha", self.id, self.fields[index].id),
                _ => format!("shell.dialog.{}.field.{}", self.id, self.fields[index].id),
            },
            ChromeDialogStop::Chip(index, chip) => format!("shell.dialog.{}.field.{}.chip.{}", self.id, self.fields[index].id, self.reference_ids(index).get(chip).cloned().unwrap_or_default()),
            ChromeDialogStop::Cancel => format!("shell.dialog.{}.cancel", self.id),
            ChromeDialogStop::Choice(index) => format!("shell.dialog.{}.choice.{}", self.id, self.choices[index].choice.id),
            ChromeDialogStop::Confirm => format!("shell.dialog.{}.confirm", self.id),
        }
    }

    /// 📝️ What a field shows: its draft, the chosen option's label, the toggle state, the slider value or the
    /// references — and whether that is placeholder text.
    pub fn field_text(&self, index: usize) -> (String, bool) {
        let field = &self.fields[index];
        match &field.kind {
            ChromeDialogFieldKind::Text | ChromeDialogFieldKind::Number { .. } | ChromeDialogFieldKind::Axis { .. } | ChromeDialogFieldKind::Color { .. } => (field.draft.clone(), false),
            ChromeDialogFieldKind::Toggle => (if matches!(self.field_value(index), Some(DslValue::Bool(true))) { self.labels.on.clone() } else { self.labels.off.clone() }, false),
            ChromeDialogFieldKind::Select { options } | ChromeDialogFieldKind::Segmented { options } => {
                let current = self.field_value(index).and_then(|value| value.as_str().map(str::to_string));
                match current.and_then(|current| options.iter().find(|(value, _)| *value == current)) {
                    Some((_, label)) => (label.clone(), false),
                    None => (self.labels.choose.clone(), true),
                }
            }
            ChromeDialogFieldKind::Slider { precision, unit, .. } => {
                let value = self.slider_value(index).map_or_else(String::new, |value| chrome_dialog_number_text(value, *precision));
                (unit.as_deref().map_or_else(|| value.clone(), |unit| format!("{value} {unit}")), false)
            }
            ChromeDialogFieldKind::Reference { .. } => {
                let ids = self.reference_ids(index);
                if ids.is_empty() {
                    (self.labels.empty.clone(), true)
                } else {
                    (ids.join(", "), false)
                }
            }
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn confirm(id: &str, title: &str, action: &str) -> Self {
        let dialog = semio_framework::DialogDefinition::new(id, LocalizedLabel::native(title, title), semio_framework::ActionRef::new(action));
        Self::from_definition("test", &dialog, None, Terminology::Native, Locale::En)
    }
}

//#endregion 🗨️ChromeDialog
/// 🎓️ Live playback state for `AppDefinition.introduction` — which step is showing. Steps themselves are
/// re-read fresh from `session.app.introduction` every frame, never cached, so a plugin hot-reload
/// mid-session can't desync from a stale copy.
#[derive(Clone, Debug)]
pub struct ChromeTourState {
    pub step_index: usize,
    /// ✅️ Indices into the active step's `interactions` that are done — reset whenever `step_index` changes.
    pub completed_interactions: Vec<usize>,
}

impl ShellChromeBuildState {
    #[cfg(any(test, feature = "testkit"))]
    pub fn register_tooltip(&mut self, control_id: impl Into<String>, title: impl Into<String>) {
        let title = title.into();
        if !title.is_empty() {
            self.tooltip_titles.insert(control_id.into(), title);
        }
    }

    /// 💡️ Whether the ACTIVE driver lets a chrome control show a hover tooltip at all —
    /// `useControlTooltipText`'s own first two guards (`🏷️Label/🟦️.tsx:186-190`). `label_visible`
    /// says whether the control already paints its caption beside its icon, which React reads off
    /// `useControlInlineText` and never repeats in a tooltip.
    pub fn tooltip_shows(&self, label_visible: bool) -> bool {
        self.driver.tooltip_shows(label_visible)
    }

    pub fn compute_click_edge(&mut self, pointer_down: bool) {
        self.clicked_this_frame = pointer_down && !self.previous_pointer_down;
        self.previous_pointer_down = pointer_down;
    }

    pub fn dialog_open(&self) -> bool {
        !self.dialog_stack.is_empty()
    }

    /// 🎓️ Whether the introduction is PLAYING, i.e. its veil and card own the pointer.
    pub fn tour_is_open(&self) -> bool {
        self.tour_state.is_some()
    }

    pub fn open_dialog(&mut self, request: ChromeDialogRequest) {
        self.dialog_stack.push(request);
    }

    pub fn close_topmost_dialog(&mut self) {
        self.dialog_stack.pop();
    }

    pub fn start_introduction(&mut self) {
        self.tour_state = Some(ChromeTourState { step_index: 0, completed_interactions: Vec::new() });
    }

    pub fn skip_introduction(&mut self) {
        self.tour_state = None;
    }

    pub fn advance_introduction(&mut self, step_count: usize) {
        if let Some(tour) = self.tour_state.as_mut() {
            if tour.step_index + 1 >= step_count {
                self.tour_state = None;
            } else {
                tour.step_index += 1;
                tour.completed_interactions.clear();
            }
        }
    }

    pub fn back_introduction(&mut self) {
        if let Some(tour) = self.tour_state.as_mut() {
            tour.step_index = tour.step_index.saturating_sub(1);
            tour.completed_interactions.clear();
        }
    }

    pub fn introduction_was_seen(&self, app_id: &str) -> bool {
        self.introduction_seen.get(app_id).copied().unwrap_or(false)
    }

    pub fn mark_introduction_seen(&mut self, app_id: &str) {
        if !self.introduction_was_seen(app_id) {
            self.introduction_seen.insert(app_id.to_string(), true);
            self.introduction_seen_writes.push(app_id.to_string());
        }
    }

    /// 🎓️ Ends the tour AND records the answer, the wgpu twin of `🏛️ShellHost/🟦️.tsx`'s
    /// `dismissIntroduction` — which persists the device-local seen flag on Skip exactly as it does on
    /// Done, so a tour the user answered never comes back. [`skip_introduction`] stays the
    /// answer-less variant, for the two places a TUTORIAL takes the surface over (Design Decision 8).
    pub fn dismiss_introduction(&mut self, app_id: &str) {
        self.tour_state = None;
        self.mark_introduction_seen(app_id);
    }
}

/// 🎓️ A rect registered this frame under a logical element id (`register_element_rect`) — `fallback`
/// entries (folded-chrome chips) never override an already-registered primary entry, mirroring
/// `ui/js/react/index.tsx`'s `useIntroductionAnchorRect` "never downgrade" rule.
pub struct ChromeElementRectEntry {
    pub rect: Rect,
}

impl ShellChromeBuildState {
    pub fn register_element_rect(&mut self, id: impl Into<String>, rect: Rect) {
        self.element_rects.insert(id.into(), ChromeElementRectEntry { rect });
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn register_element_rect_fallback(&mut self, id: impl Into<String>, rect: Rect) {
        let id = id.into();
        self.element_rects.entry(id).or_insert(ChromeElementRectEntry { rect });
    }

    pub fn resolve_element_rect(&self, id: &str) -> Option<Rect> {
        self.element_rects.get(id).map(|entry| entry.rect)
    }
}

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🗨️ChromeDialog
/// 🎯️ One focus stop of the dialog, in visiting order. `Chip(field, index)` is one removable reference of a
/// reference field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChromeDialogStop {
    Field(usize),
    Chip(usize, usize),
    Cancel,
    Choice(usize),
    Confirm,
}

/// 🧭️ A vector component's name — React's staged vector row names them `x y z w`, then by index.
pub fn chrome_dialog_axis_name(axis: usize) -> String {
    ["x", "y", "z", "w"].get(axis).map_or_else(|| axis.to_string(), |name| (*name).to_string())
}

/// 🔢️ A number field's text: at its precision when it has one, else twelve significant digits.
pub fn chrome_dialog_number_text(value: f64, precision: Option<u16>) -> String {
    precision.map_or_else(|| ui_contract::format_ui_number(value), |precision| ui_contract::format_ui_number_fixed(value, precision))
}
//#endregion 🗨️ChromeDialog
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
/// ⏱️ Pure delay-threshold check, factored out of `render_chrome_tooltip` so it's unit-testable without
/// a `DrawList`/`FontAtlas` fixture.
pub fn chrome_tooltip_ready(hover: &ChromeTooltipHover, now_ms: f64) -> bool {
    now_ms - hover.started_ms >= CHROME_TOOLTIP_DELAY_MS
}

/// ⏱️ Hover-to-tooltip delay — the React chrome's own `CHROME_CONTROL_TOOLTIP_DELAY_MS`
/// (`💡️ChromeControlHint/🟦️.tsx:20`). It was 500 ms here, which made every wgpu tooltip feel a fifth
/// of a second slower than the same control under React; WGPU-RENDERER-REACT-PARITY packet W1j
/// aligns the number, since the surrounding arm/dismiss behaviour was already identical.
pub const CHROME_TOOLTIP_DELAY_MS: f64 = 400.0;
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
/// ⏱️ Pure deadline check, factored out so the 4 s rule is testable without a frame clock.
pub fn transient_notice_expired(notice: &ShellTransientNotice, now_ms: f64) -> bool {
    now_ms - notice.shown_at_ms >= TRANSIENT_NOTICE_AUTO_DISMISS_MS
}

/// ⏱️ `setTimeout(…, 4000)` in `showTransientNotice` — the auto-dismiss deadline.
pub const TRANSIENT_NOTICE_AUTO_DISMISS_MS: f64 = 4000.0;
//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome
