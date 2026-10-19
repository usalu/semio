//! 🔍️ Lookup and search of the registry: the accessors, the find and search operations, the parameters of an entry and the graph basis. See [`super`].

use super::*;

// #region 🔖️Registry
impl Registry {
    /// 🏠️ The workspace root the commands run in.
    pub fn root(&self) -> &Path { &self.root }
    /// 📜️ Every command, sorted by verb in launcher order and then by label.
    pub fn entries(&self) -> &[Entry] { &self.entries }
    /// ❌️ Every invalid declaration and dangling reference, each naming its file.
    pub fn problems(&self) -> &[Problem] { &self.problems }
    /// 📂️ The workspace-relative files that define the project graph: every manifest plus the workspace configuration.
    pub fn definitions(&self) -> &[String] { &self.definitions }
    /// 🌐️ The global axes the workspace root project declares.
    pub fn axes(&self) -> &[Parameter] { &self.axes }

    /// 🔎️ The command with this identity. `<project>:<target>:<configuration>` finds its target.
    pub fn find(&self, id: &str) -> Option<&Entry> { self.member(id).map(|(entry, _)| entry) }

    pub(super) fn member<'a, 'b>(&'a self, reference: &'b str) -> Option<(&'a Entry, Option<&'b str>)> {
        if let Some(position) = self.index.get(reference) { return Some((&self.entries[*position], None)); }
        let (head, configuration) = reference.rsplit_once(':')?;
        let entry = &self.entries[*self.index.get(head)?];
        matches!(&entry.action, Action::Target { configurations, .. } if configurations.iter().any(|known| known == configuration)).then_some((entry, Some(configuration)))
    }

    /// 🔦️ The positions of the commands whose label or identity contains every word, in registry
    /// order. Closed tickets only take part when `unlisted` is set.
    pub fn search(&self, words: &[&str], unlisted: bool) -> Vec<usize> {
        let needles: Vec<String> = words.iter().flat_map(|word| word.split_whitespace()).map(str::to_lowercase).collect();
        self.entries.iter().enumerate().filter(|(_, entry)| (unlisted || entry.listed) && needles.iter().all(|needle| entry.haystack.contains(needle.as_str()))).map(|(position, _)| position).collect()
    }

    /// 🎼️ Every parameter a command accepts, in the order their effects apply: the global axes
    /// that apply to it, its own, then the built-in ones.
    pub fn parameters(&self, entry: &Entry) -> Vec<Parameter> {
        let axes = |playground: bool, own: &[Parameter]| self.axes.iter().filter(|axis| axis.applies(&entry.verb, playground) && !own.iter().any(|parameter| parameter.id == axis.id) && !PLAYGROUND_PARAMETERS.contains(&axis.id.as_str()) && axis.id != CONFIGURATION).cloned().collect::<Vec<_>>();
        match &entry.action {
            Action::Target { configurations, .. } => {
                let mut parameters = axes(false, &entry.own);
                parameters.extend(entry.own.iter().cloned());
                if !configurations.is_empty() && !entry.own.iter().any(|parameter| parameter.id == CONFIGURATION) { parameters.push(Parameter::choice(CONFIGURATION, Origin::Configuration, None, configurations.iter().map(|name| (name.clone(), Effect::default())).collect())); }
                parameters
            }
            Action::Group { .. } => axes(false, &[]),
            Action::Tool { .. } => {
                let mut parameters: Vec<Parameter> = self.axes.iter().filter(|axis| axis.applies_beyond_build(&entry.verb) && !entry.own.iter().any(|parameter| parameter.id == axis.id) && !PLAYGROUND_PARAMETERS.contains(&axis.id.as_str()) && axis.id != CONFIGURATION).cloned().collect();
                parameters.extend(entry.own.iter().cloned());
                parameters
            }
            Action::Playground(playground) => {
                let plain = |values: &[&str]| values.iter().map(|value| ((*value).to_string(), Effect::default())).collect::<Vec<_>>();
                let mut parameters = vec![Parameter::choice("renderer", Origin::Playground, Some(RENDERERS[0]), plain(RENDERERS))];
                if !playground.examples.is_empty() { parameters.push(Parameter::choice("example", Origin::Playground, None, playground.examples.iter().map(|example| (example.clone(), Effect::default())).collect())); }
                let slots = playground.user_react.len().max(playground.user_wgpu.len());
                if slots > 0 { parameters.push(Parameter::choice("user-slot", Origin::Playground, None, (1..=slots).map(|slot| (slot.to_string(), Effect::default())).collect())); }
                parameters.push(Parameter::choice("app-role", Origin::Playground, None, vec![("viewer".into(), Effect { env: vec![("SEMIO_APP_ROLE".into(), "viewer".into())], ..Default::default() })]));
                let flag = |id: &str, default: bool, effect: Effect| Parameter { id: id.into(), title: None, description: None, kind: ParameterKind::Flag, origin: Origin::Playground, default: default.then(|| "true".to_string()), required: false, values: Vec::new(), effect, destination: Destination::Token, suggestions: None, all: false, verbs: None, playground: false, scoped: false };
                if let Some(hub) = &playground.hub { parameters.push(flag("hub", false, Effect { env: vec![(HUB_ENV.into(), hub.clone())], ..Default::default() })); }
                if playground.data_dir.is_some() || playground.user_data_dir.is_some() { parameters.push(flag("data", true, Effect::default())); }
                if playground.local_only { parameters.push(flag("local-only", false, Effect { env: vec![(LOCAL_ONLY_ENV.into(), "1".into())], ..Default::default() })); }
                parameters.push(Parameter::choice("language", Origin::Playground, None, Locale::ALL.iter().map(|locale| (locale.as_str().to_string(), Effect::default())).collect()));
                parameters.push(Parameter::choice("terminology", Origin::Playground, None, plain(&["native", "reuse"])));
                parameters.push(Parameter::choice("appearance", Origin::Playground, None, plain(&["dark", "light"])));
                parameters.extend(axes(true, &[]));
                parameters
            }
            Action::Compound { members, .. } => {
                let mut parameters: Vec<Parameter> = Vec::new();
                for member in members {
                    let Some((target, _)) = self.member(&member.run) else { continue };
                    if matches!(target.action, Action::Compound { .. }) { continue; }
                    for mut parameter in self.parameters(target) {
                        if member.pins.iter().any(|(key, _)| key == &parameter.id) || parameters.iter().any(|known| known.id == parameter.id) { continue; }
                        (parameter.origin, parameter.required) = (Origin::Member, false);
                        parameters.push(parameter);
                    }
                }
                parameters
            }
            Action::Repo(action) => repo_parameters(action),
        }
    }

    /// ⚡️ Whether the published graph and the source hashes captured with it remain current.
    /// `SEMIO_DASHBOARD_GRAPH=fresh` always answers no.
    pub fn graph_is_current(&self) -> bool {
        if std::env::var("SEMIO_DASHBOARD_GRAPH").is_ok_and(|value| value == "fresh") { return false; }
        let Some(graph) = Fingerprint::of(&self.root.join(graph_file())) else { return false };
        let mut basis = self.basis.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !basis.as_ref().is_some_and(|known| known.graph == graph) { *basis = GraphBasis::read(&self.root, &self.definitions); }
        let Some(basis) = basis.as_ref().filter(|known| known.graph == graph) else { return false };
        self.definitions.len() > GRAPH_OWNERS.len() && self.definitions.iter().all(|file| basis.hashes.get(file).is_some_and(|recorded| std::fs::read(self.root.join(file)).is_ok_and(|bytes| xxh3_64(&bytes) == *recorded)))
    }

    /// 🧷️ Adopts what discovery captured with its published graph, read ahead of the first launch.
    pub fn adopt_basis(&self, basis: Option<GraphBasis>) { *self.basis.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = basis; }
    /// 📌️ What discovery captured with its published graph, as far as this registry has read it.
    pub fn basis(&self) -> Option<GraphBasis> { self.basis.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() }
}
// #endregion 🔖️Registry
