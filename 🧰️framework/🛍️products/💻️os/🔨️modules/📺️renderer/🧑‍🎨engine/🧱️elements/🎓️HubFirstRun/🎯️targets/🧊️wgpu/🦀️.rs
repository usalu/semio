//! 🎓️ wgpu twin of `🎓️HubFirstRun` and of the first-run contract `📇️directory/🎓️first-run/🟦️.ts`.
//!
//! 🧬️ Nothing here is restated: stages, step structure, anchors, localized text, the seen-flag key and
//! the two workspace controls are read from the language-neutral `📇️directory/🎓️first-run/🔣️.json`
//! (shape `🧬️.schema.json`), the file the React contract reads too. Both twins are held to the same
//! vectors (`🧪️tests/🧫️fixtures/🔣️.json`). The tour itself is rendered by the shell's one introduction
//! overlay — this module only supplies its [`IntroductionDefinition`] and its entry/seen decisions.

use semio_framework::onboarding::{IntroductionDefinition, IntroductionInteraction, IntroductionInteractionKind, IntroductionPlacement, IntroductionStepDefinition};
use semio_ui_locale::{Locale, LocalizedLabel};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const CONTRACT_SOURCE: &str = include_str!("../../../../../../../../../🔨️modules/📇️directory/🎓️first-run/🔣️.json");

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Celebrate {
    sign_in: Option<String>,
    space: Option<String>,
    redeem: Option<String>,
    invite: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnchorSet {
    sign_in: String,
    sign_in_support: Vec<String>,
    space: String,
    space_support: Vec<String>,
    invite: String,
    invite_support: Vec<String>,
    redeem_field: String,
    celebrate: Celebrate,
}

impl AnchorSet {
    fn id(&self, name: &str) -> &str {
        match name {
            "signIn" => &self.sign_in,
            "space" => &self.space,
            "invite" => &self.invite,
            _ => &self.redeem_field,
        }
    }

    fn support(&self, name: &str) -> &[String] {
        match name {
            "signInSupport" => &self.sign_in_support,
            "spaceSupport" => &self.space_support,
            _ => &self.invite_support,
        }
    }

    fn celebrate(&self, name: &str) -> Option<&str> {
        match name {
            "signIn" => self.celebrate.sign_in.as_deref(),
            "space" => self.celebrate.space.as_deref(),
            "redeem" => self.celebrate.redeem.as_deref(),
            _ => self.celebrate.invite.as_deref(),
        }
    }
}

#[derive(Deserialize)]
struct InteractionData {
    on: String,
    label: String,
    celebrate: String,
}

#[derive(Deserialize)]
struct StepData {
    stage: String,
    title: String,
    body: String,
    placement: String,
    introduce: Option<String>,
    show: Option<String>,
    interactions: Vec<InteractionData>,
}

#[derive(Deserialize)]
struct LabelData {
    en: String,
    de: String,
}

#[derive(Deserialize)]
struct ControlData {
    id: String,
    icon: String,
    variant: String,
    label: LabelData,
}

#[derive(Deserialize)]
struct ControlsData {
    replay: ControlData,
    close: ControlData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contract {
    introduction_id: String,
    storage_key_prefix: String,
    seen_value: String,
    step_id_prefix: String,
    stages: Vec<String>,
    anchors: HashMap<String, AnchorSet>,
    steps: Vec<StepData>,
    completion: HashMap<String, String>,
    text: HashMap<String, HashMap<String, String>>,
    controls: ControlsData,
}

fn contract() -> &'static Contract {
    static CONTRACT: OnceLock<Contract> = OnceLock::new();
    CONTRACT.get_or_init(|| serde_json::from_str(CONTRACT_SOURCE).expect("the shared hub first-run contract is valid JSON of its schema"))
}

//#region 🪜️Stages
/// 🪜️ The ordered stages of the hub flow; the stage's position is also its step index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubFirstRunStage {
    Welcome,
    SignIn,
    Space,
    Invite,
    Done,
}

impl HubFirstRunStage {
    pub const ALL: [Self; 5] = [Self::Welcome, Self::SignIn, Self::Space, Self::Invite, Self::Done];

    /// 🏷️ The contract's spelling of this stage.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Welcome => "welcome",
            Self::SignIn => "signIn",
            Self::Space => "space",
            Self::Invite => "invite",
            Self::Done => "done",
        }
    }

    /// 🔢️ The step index this stage owns.
    pub fn index(self) -> usize {
        Self::ALL.iter().position(|stage| *stage == self).unwrap_or(0)
    }
}

/// 🧭️ What the shell knows about the person's hub right now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubFirstRunState {
    pub signed_in: bool,
    pub space_count: usize,
    pub open_space_id: Option<String>,
    pub can_invite: bool,
}

/// 🎯️ The first stage the person has not completed — where the tour opens.
pub fn hub_first_run_stage(state: &HubFirstRunState) -> HubFirstRunStage {
    if !state.signed_in {
        HubFirstRunStage::SignIn
    } else if state.space_count == 0 || state.open_space_id.is_none() {
        HubFirstRunStage::Space
    } else if state.can_invite {
        HubFirstRunStage::Invite
    } else {
        HubFirstRunStage::Done
    }
}

/// 🔢️ The step index the tour opens at for `state`.
pub fn hub_first_run_step_index(state: &HubFirstRunState) -> usize {
    hub_first_run_stage(state).index()
}

/// 🧭️ Projects the live workspace onto [`HubFirstRunState`]: `rows` are `(id, access)` pairs, `canInvite`
/// is the open space's `author` access, or any row's when no space is open.
pub fn hub_first_run_state_from(session_phase: &str, rows: &[(&str, &str)], active_space_id: Option<&str>) -> HubFirstRunState {
    let open = active_space_id.and_then(|id| rows.iter().find(|(row_id, _)| *row_id == id));
    HubFirstRunState {
        signed_in: session_phase == "signed-in",
        space_count: rows.len(),
        open_space_id: open.and(active_space_id).map(str::to_string),
        can_invite: open.map_or_else(|| rows.iter().any(|(_, access)| *access == "author"), |(_, access)| *access == "author"),
    }
}

/// ✅️ The shared completion rule (`🔣️.json` `completion`): whether the person has just done what `stage`
/// teaches — signed in, a space open, an invite ready. Welcome and done are never completed by doing.
pub fn hub_first_run_stage_done(stage: HubFirstRunStage, state: &HubFirstRunState, invite_ready: bool) -> bool {
    match contract().completion.get(stage.as_str()).map(String::as_str) {
        Some("signedIn") => state.signed_in,
        Some("spaceOpen") => state.signed_in && state.open_space_id.is_some(),
        Some("inviteReady") => invite_ready,
        _ => false,
    }
}
//#endregion 🪜️Stages

//#region 🎓️Definition
fn label(row: &str, key: &str) -> &'static str {
    contract().text.get(row).and_then(|text| text.get(key)).map_or("", String::as_str)
}

fn localized(key: &str) -> LocalizedLabel {
    LocalizedLabel::native(label("en", key), label("de", key))
}

fn placement(name: &str) -> IntroductionPlacement {
    match name {
        "top" => IntroductionPlacement::Top,
        "bottom" => IntroductionPlacement::Bottom,
        "left" => IntroductionPlacement::Left,
        "right" => IntroductionPlacement::Right,
        "center" => IntroductionPlacement::Center,
        _ => IntroductionPlacement::Auto,
    }
}

fn step(data: &StepData) -> IntroductionStepDefinition {
    let anchors = &contract().anchors["wgpu"];
    let mut step = IntroductionStepDefinition::new(format!("{}{}", contract().step_id_prefix, data.stage), localized(&data.title), localized(&data.body)).placement(placement(&data.placement));
    if let Some(name) = &data.introduce {
        step = step.introduce(anchors.id(name));
    }
    if let Some(name) = &data.show {
        step = step.show(anchors.support(name).to_vec());
    }
    step.interact(
        data.interactions
            .iter()
            .map(|interaction| IntroductionInteraction {
                on: IntroductionInteractionKind::Panel(anchors.id(&interaction.on).to_string()),
                label: localized(&interaction.label),
                celebrate: anchors.celebrate(&interaction.celebrate).map(str::to_string),
            })
            .collect(),
    )
}

/// 🎓️ The whole walkthrough in stage order, carrying both locales — the wgpu twin of
/// `hubFirstRunIntroductionV1`, anchored to the `framework.hub.*` controls this renderer paints.
pub fn hub_first_run_introduction() -> &'static IntroductionDefinition {
    static INTRODUCTION: OnceLock<IntroductionDefinition> = OnceLock::new();
    INTRODUCTION.get_or_init(|| IntroductionDefinition { title: localized("title"), steps: contract().steps.iter().map(step).collect() })
}

/// 🌐️ One localized string of the contract's text table, in `locale`.
pub fn hub_first_run_text(locale: Locale, key: &str) -> &'static str {
    label(
        match locale {
            Locale::En => "en",
            Locale::De => "de",
        },
        key,
    )
}
//#endregion 🎓️Definition

//#region 🔖️Seen
/// 🆔️ The introduction id the seen flag is filed under (`os.hub`).
pub fn hub_first_run_introduction_id() -> &'static str {
    &contract().introduction_id
}

/// 🔖️ The storage key of the seen flag — the key space every other introduction shares.
pub fn hub_first_run_storage_key() -> String {
    format!("{}{}", contract().storage_key_prefix, contract().introduction_id)
}

/// 🗄️ The one door the seen flag is read and written through.
pub trait HubFirstRunStorage {
    fn get_item(&self, key: &str) -> Option<String>;
    fn set_item(&mut self, key: &str, value: &str);
}

/// 🔎️ Whether the tour already ran: only the contract's seen value counts, an absent store reads unseen.
pub fn hub_first_run_seen(storage: Option<&dyn HubFirstRunStorage>) -> bool {
    storage.and_then(|storage| storage.get_item(&hub_first_run_storage_key())).as_deref() == Some(contract().seen_value.as_str())
}

/// ✍️ Records that the tour ran.
pub fn mark_hub_first_run_seen(storage: Option<&mut dyn HubFirstRunStorage>) {
    if let Some(storage) = storage {
        storage.set_item(&hub_first_run_storage_key(), &contract().seen_value);
    }
}

/// 🚦️ Whether to auto-start the tour: never once seen, never before the hub surface is open.
pub fn hub_first_run_should_start(storage: Option<&dyn HubFirstRunStorage>, hub_surface_open: bool) -> bool {
    hub_surface_open && !hub_first_run_seen(storage)
}

/// 🚪️ The step an auto-start opens on, or `None` when it must not start.
pub fn hub_first_run_auto_open_step(storage: Option<&dyn HubFirstRunStorage>, hub_surface_open: bool, state: &HubFirstRunState) -> Option<usize> {
    hub_first_run_should_start(storage, hub_surface_open).then(|| hub_first_run_step_index(state))
}

/// 🔁️ The step an explicit replay opens on, regardless of state.
pub fn hub_first_run_replay_step() -> usize {
    0
}
//#endregion 🔖️Seen

//#region 🎛️Controls
/// 🎛️ One workspace control — the replay or the close button — as the contract defines it.
#[derive(Clone, Copy)]
pub struct HubFirstRunControl(&'static ControlData);

impl HubFirstRunControl {
    pub fn id(self) -> &'static str {
        &self.0.id
    }

    pub fn icon(self) -> &'static str {
        &self.0.icon
    }

    pub fn variant(self) -> &'static str {
        &self.0.variant
    }

    pub fn label(self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => &self.0.label.en,
            Locale::De => &self.0.label.de,
        }
    }
}

/// 🎛️ The replay button, then the close button — the order the workspace paints them in.
pub fn hub_first_run_controls() -> [HubFirstRunControl; 2] {
    [HubFirstRunControl(&contract().controls.replay), HubFirstRunControl(&contract().controls.close)]
}
//#endregion 🎛️Controls

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
