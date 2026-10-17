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
pub enum Origin { Axis, Target, Tool, Playground, Configuration, Member }

/// 💥️ What a chosen value does to the launch: `env` joins the environment, `build_flags` go before
/// `--`, `args` after it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Effect { pub env: Vec<(String, String)>, pub build_flags: Vec<String>, pub args: Vec<String> }

/// 📮️ Where typed text goes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Destination { #[default] Token, Env(String), Flag(String), Positional }

/// 🎛️ One parameter a command accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub id: String,
    pub kind: ParameterKind,
    pub origin: Origin,
    pub default: Option<String>,
    pub required: bool,
    pub values: Vec<(String, Effect)>,
    pub effect: Effect,
    pub destination: Destination,
    pub(super) verbs: Option<Vec<String>>,
    pub(super) playground: bool,
    pub(super) scoped: bool,
}

impl Parameter {
    pub(super) fn choice(id: &str, origin: Origin, default: Option<&str>, values: Vec<(String, Effect)>) -> Self { Self { id: id.into(), kind: ParameterKind::Choice, origin, default: default.map(str::to_string), required: false, values, effect: Effect::default(), destination: Destination::Token, verbs: None, playground: false, scoped: false } }

    /// 🔬️ Whether a global axis is offered on a command the build runner runs.
    pub(super) fn applies(&self, verb: &str, playground: bool) -> bool {
        if playground { return self.playground; }
        !self.scoped || self.selects(verb)
    }

    /// 🧰️ Whether a global axis is offered on a command outside the build runner: it must name the verb and
    /// change more than the build flags, which such a command ignores.
    pub(super) fn applies_beyond_build(&self, verb: &str) -> bool {
        self.scoped && self.selects(verb) && (!self.effect.env.is_empty() || !self.effect.args.is_empty() || self.destination != Destination::Token || self.values.iter().any(|(_, effect)| !effect.env.is_empty() || !effect.args.is_empty()))
    }

    fn selects(&self, verb: &str) -> bool { self.verbs.as_ref().is_some_and(|verbs| verbs.iter().any(|known| known == verb)) }
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
