//! 🧬️ Model of the registry: the command kind, origin, effect, destination, parameter, action, entry and request. See [`super`].

use super::*;

// #region 🔖️Model
/// 🧬️ What kind of thing a command is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind { Target, Playground, Tool, Compound, Group, Ticket, Repo }

/// 🧲️ Where a parameter of a command comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Origin { Axis, Target, Tool, Playground, Configuration, Member, Repo }

/// 💥️ What a chosen value does to the launch: `env` joins the environment, `build_flags` go before
/// `--`, `args` after it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Effect { pub env: Vec<(String, String)>, pub build_flags: Vec<String>, pub args: Vec<String> }

/// 📮️ Where typed text goes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Destination { #[default] Token, Env(String), Flag(String), Positional }

/// 🌐️ A complete single-line text in each supported dashboard language.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalizedText { pub en: String, pub de: String }

pub(super) fn localized_declaration<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<LocalizedText>, D::Error> { LocalizedText::deserialize(deserializer).map(Some) }

impl LocalizedText {
    /// 🗣️ Selects the explicitly chosen dashboard language.
    pub fn get(&self, locale: Locale) -> &str { match locale { Locale::En => &self.en, Locale::De => &self.de } }

    pub(super) fn valid(&self) -> bool { [&self.en, &self.de].iter().all(|text| !text.trim().is_empty() && text.chars().count() <= 1024 && !text.chars().any(|character| character <= '\u{1f}' || character == '\u{7f}')) }
}

/// 🎛️ One parameter a command accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub id: String,
    pub title: Option<LocalizedText>,
    pub description: Option<LocalizedText>,
    pub kind: ParameterKind,
    pub origin: Origin,
    pub default: Option<String>,
    pub required: bool,
    pub values: Vec<(String, Effect)>,
    pub effect: Effect,
    pub destination: Destination,
    pub suggestions: Option<SuggestionSpec>,
    pub(super) all: bool,
    pub(super) verbs: Option<Vec<String>>,
    pub(super) playground: bool,
    pub(super) scoped: bool,
}

impl Parameter {
    /// 🏷️ A human title in the selected language, or the parameter identity.
    pub fn display_title(&self, locale: Locale) -> &str { self.title.as_ref().map_or(self.id.as_str(), |text| text.get(locale)) }

    pub(super) fn choice(id: &str, origin: Origin, default: Option<&str>, values: Vec<(String, Effect)>) -> Self { Self { id: id.into(), title: None, description: None, kind: ParameterKind::Choice, origin, default: default.map(str::to_string), required: false, values, effect: Effect::default(), destination: Destination::Token, suggestions: None, all: false, verbs: None, playground: false, scoped: false } }

    /// 🔬️ Whether a global axis is offered on a command the build runner runs.
    pub(super) fn applies(&self, verb: &str, playground: bool) -> bool {
        if playground { return self.playground; }
        !self.scoped || self.selects(verb)
    }

    /// 🧰️ Whether a global axis is offered on a command outside the build runner: it must select the verb and
    /// change more than the build flags, which such a command ignores.
    pub(super) fn applies_beyond_build(&self, verb: &str) -> bool {
        self.scoped && self.selects(verb) && (!self.effect.env.is_empty() || !self.effect.args.is_empty() || self.destination != Destination::Token || self.values.iter().any(|(_, effect)| !effect.env.is_empty() || !effect.args.is_empty()))
    }

    fn selects(&self, verb: &str) -> bool { self.all || self.verbs.as_ref().is_some_and(|verbs| verbs.iter().any(|known| known == verb)) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Value { pub(super) text: String, pub(super) stated: bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Text { Declared(String), Literal(String) }

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReadySpec { pub(super) port: Option<u16>, pub(super) port_env: Option<String>, pub(super) path: String, pub(super) printed: bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Member { pub(super) run: String, pub(super) pins: Vec<(String, String)>, pub(super) env: Vec<(String, String)> }

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Action {
    Target { project: String, target: String, configurations: Vec<String>, ready: Option<ReadySpec>, requires: Vec<Member> },
    Group { targets: Vec<String>, projects: Vec<String> },
    Playground(PlaygroundFacts),
    Tool { command: Vec<String>, cwd: String, env: Vec<(String, String)>, ready: Option<ReadySpec>, requires: Vec<Member> },
    Compound { members: Vec<Member>, stop: Stop },
    Repo(RepoAction),
}

/// 📇️ One command of the registry.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub id: String,
    pub title: Option<LocalizedText>,
    pub description: Option<LocalizedText>,
    pub kind: Kind,
    pub verb: String,
    pub path: Vec<Segment>,
    pub label: String,
    pub owner: Vec<String>,
    pub subject: String,
    pub qualifier: String,
    pub long_running: bool,
    /// 🙈️ Whether the command belongs to the default searchable set; a closed ticket's does not.
    pub listed: bool,
    /// ⚠️ Whether running the command changes repository state and deserves a confirmation.
    pub mutating: bool,
    pub source: String,
    /// 📟️ The MCP server the command is for the agent clients that start it, when it declares one.
    pub mcp: Option<Mcp>,
    pub(super) own: Vec<Parameter>,
    pub(super) action: Action,
    pub(super) haystack: String,
}

/// 📝️ What a developer chooses besides the command: parameter values, free extra arguments that
/// follow `--` and free extra environment.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Request {
    pub parameters: Vec<(String, String)>,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
}

impl Request {
    /// 🎲️ A request that only chooses parameter values.
    pub fn with_parameters(parameters: Vec<(String, String)>) -> Self { Self { parameters, ..Self::default() } }
}
// #endregion 🔖️Model


pub(super) fn repo_parameters(action: &RepoAction) -> Vec<Parameter> {
    let text = |id: &str, en: &str, de: &str, help_en: &str, help_de: &str| { let mut parameter = Parameter::choice(id, Origin::Repo, None, Vec::new()); parameter.kind = ParameterKind::Text; parameter.required = true; parameter.title = Some(LocalizedText { en: en.into(), de: de.into() }); parameter.description = Some(LocalizedText { en: help_en.into(), de: help_de.into() }); parameter };
    match action {
        RepoAction::DraftStart { .. } => vec![text("title", "Draft Title", "Entwurfstitel", "Give the draft a human title; its identifier is derived from it.", "Gib dem Entwurf einen verständlichen Titel; daraus entsteht seine Kennung.")],
        RepoAction::DraftDiscard { .. } => vec![text("slug", "Draft Identifier", "Entwurfskennung", "Enter the exact uppercase draft identifier, such as HUMAN-DRAFT.", "Gib die genaue Entwurfskennung in Großbuchstaben ein, etwa HUMAN-DRAFT.")],
        RepoAction::TicketReopen { .. } => vec![text("prompt", "Reason to Reopen", "Grund Zum Wiederöffnen", "Describe the work to continue; the reason is saved with the ticket.", "Beschreibe die fortzusetzende Arbeit; der Grund wird im Ticket gespeichert.")],
        RepoAction::Analyze { .. } => { let mut parameter = Parameter::choice("policy", Origin::Repo, Some("all"), ["all", "code"].into_iter().map(|value| (value.into(), Effect::default())).collect()); parameter.title = Some(LocalizedText { en: "Analysis Policy".into(), de: "Analyserichtlinie".into() }); parameter.description = Some(LocalizedText { en: "All runs the implementation's supported policies; Rust currently implements code.".into(), de: "All führt die unterstützten Richtlinien aus; Rust implementiert derzeit code.".into() }); vec![parameter] },
        _ => Vec::new(),
    }
}
