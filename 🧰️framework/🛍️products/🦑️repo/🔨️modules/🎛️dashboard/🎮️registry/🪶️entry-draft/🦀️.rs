//! 🪶️ Entry draft of the registry: the draft a source states, the entry construction and the argument helpers. See [`super`].

use super::*;

// #region 🔖️Registry
/// 📙️ What a source states about a command before the registry labels and files it.
pub(super) struct Draft<'a> { pub(super) id: String, pub(super) kind: Kind, pub(super) verb: String, pub(super) path: Vec<Segment>, pub(super) owner: Vec<String>, pub(super) subject: String, pub(super) qualifier: String, pub(super) long_running: bool, pub(super) source: &'a str, pub(super) own: Vec<Parameter>, pub(super) action: Action }

impl Entry {
    /// 🏷️ A human command title in the selected language, or its source label.
    pub fn display_title(&self, locale: Locale) -> &str { self.title.as_ref().map_or(self.label.as_str(), |text| text.get(locale)) }


    pub(super) fn new(draft: Draft<'_>) -> Self {
        let Draft { id, kind, verb, path, owner, subject, qualifier, long_running, source, own, action } = draft;
        let mut entry = Self { id, title: None, description: None, kind, verb, path, label: String::new(), owner, subject, qualifier, long_running, listed: true, mutating: false, source: source.to_string(), mcp: None, own, action, haystack: String::new() };
        entry.relabel();
        entry
    }

    pub(super) fn relabel(&mut self) {
        self.label = std::iter::once(self.verb.as_str()).chain(self.path.iter().map(|part| part.label.as_str())).collect::<Vec<_>>().join(" / ");
        self.haystack = format!("{} {}", self.label, self.id).to_lowercase();
        for text in self.title.iter().chain(self.description.iter()).chain(self.own.iter().flat_map(|parameter| parameter.title.iter().chain(parameter.description.iter()))) { self.haystack.push_str(&format!(" {} {}", text.en, text.de).to_lowercase()); }
    }

    /// 🛂 Who owns the port a command claims: commands of one owner are alternatives of the same service
    /// (the targets of a project, the renderers of a playground), commands of two owners can run together.
    pub(super) fn owner_key(&self) -> String {
        match &self.action {
            Action::Target { project, .. } => project.clone(),
            Action::Playground(_) | Action::Group { .. } | Action::Compound { .. } | Action::Repo(_) => self.id.clone(),
            Action::Tool { .. } => self.id.split_once(':').and_then(|(_, rest)| rest.rsplit_once('/').map(|(owner, _)| owner.to_string())).unwrap_or_else(|| self.id.clone()),
        }
    }

    /// 🛠️ The repo-domain action a repo command performs in process.
    pub fn repo_action(&self) -> Option<&RepoAction> { match &self.action { Action::Repo(action) => Some(action), _ => None } }
    /// 🚥️ Whether the command starts several processes.
    pub fn is_compound(&self) -> bool { matches!(self.action, Action::Compound { .. }) }
    /// 🎪️ The playground variant a playground command runs.
    pub fn playground(&self) -> Option<&PlaygroundFacts> { match &self.action { Action::Playground(playground) => Some(playground), _ => None } }
}

/// 🎬️ A catalog example is chosen by its slug or by its bare id, the part after the emoji identity.
pub(super) fn canonical_value(parameter: &Parameter, text: &str) -> String {
    if parameter.origin != Origin::Playground || parameter.id != "example" || parameter.values.iter().any(|(known, _)| known == text) { return text.to_string(); }
    parameter.values.iter().map(|(known, _)| known).find(|known| known.rsplit_once('\u{FE0F}').is_some_and(|(_, bare)| bare == text)).cloned().unwrap_or_else(|| text.to_string())
}

pub(super) fn unknown_parameter(id: &str, key: &str, accepted: &[Parameter]) -> String {
    format!("{id}: unknown parameter {key:?}; it accepts {}", if accepted.is_empty() { "none".to_string() } else { accepted.iter().map(|parameter| parameter.id.as_str()).collect::<Vec<_>>().join(", ") })
}

pub(super) fn invokes_build(cmd: &str) -> bool { cmd == "repo" }
// #endregion 🔖️Registry
