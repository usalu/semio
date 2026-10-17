//! 🩺️ Check and json of the registry: the consistency check and the launch plan and entry json projections. See [`super`].

use super::*;

// #region 🔖️Registry
impl Registry {
    /// 🏅️ The whole-workspace proof: every declaration is valid, every reference names a command,
    /// every command resolves with its defaults and no label outgrows the launcher.
    pub fn check(&self) -> Vec<Problem> {
        let mut problems = self.problems.clone();
        let mut claims: Vec<(u16, String, String, String)> = Vec::new();
        for entry in &self.entries {
            let mut report = |message: String| problems.push(Problem { file: entry.source.clone(), at: entry.id.clone(), message });
            if matches!(entry.action, Action::Repo(_)) { continue; }
            let accepted = self.parameters(entry);
            let chosen: Vec<(String, String)> = accepted.iter().filter(|parameter| parameter.required && parameter.default.is_none()).map(|parameter| (parameter.id.clone(), parameter.values.first().map_or_else(|| if parameter.kind == ParameterKind::Flag { "true".to_string() } else { "1".to_string() }, |(value, _)| value.clone()))).collect();
            match self.resolve_with(&entry.id, &Request::with_parameters(chosen), RunPolicy::default()) {
                Err(error) if !self.problems.iter().any(|problem| problem.at == entry.id) => report(error),
                Ok(launch) => {
                    for process in &launch.processes { for (key, _) in &process.env { if NOISE_ENV.contains(&key.as_str()) && !RUNNER_ENV.iter().any(|(owned, _)| owned == key) { report(format!("resolved environment carries {key}")); } } }
                    if let ([process], None, false) = (launch.processes.as_slice(), &launch.group, matches!(entry.action, Action::Playground(_))) { if let Some(ready) = process.ready.as_ref().filter(|_| entry.long_running) { claims.push((ready.port, entry.owner_key(), entry.id.clone(), entry.source.clone())); } }
                }
                Err(_) => {}
            }
            if let Action::Playground(playground) = &entry.action {
                for port in std::iter::once(playground.react).chain([playground.wgpu]).chain(playground.user_react.iter().copied()).chain(playground.user_wgpu.iter().copied()) { claims.push((port, entry.owner_key(), entry.id.clone(), entry.source.clone())); }
                for renderer in RENDERERS {
                    let Some(target) = playground_target(&playground.variant, renderer) else { continue };
                    let id = format!("{PLAYGROUND_OWNER}:{target}");
                    if !self.index.contains_key(&id) { report(format!("renderer {renderer} resolves to the build target {id}, which the project graph does not have")); }
                }
            }
            if entry.label.chars().count() > 240 { report(format!("label is {} characters long", entry.label.chars().count())); }
        }
        let mut by_port: BTreeMap<u16, Vec<&(u16, String, String, String)>> = BTreeMap::new();
        for claim in &claims { by_port.entry(claim.0).or_default().push(claim); }
        for (port, claimants) in by_port {
            let owners: HashSet<&str> = claimants.iter().map(|claim| claim.1.as_str()).collect();
            if owners.len() < 2 { continue; }
            let mut ids: Vec<&str> = claimants.iter().map(|claim| claim.2.as_str()).collect();
            ids.dedup();
            problems.push(Problem { file: claimants[0].3.clone(), at: claimants[0].2.clone(), message: format!("port {port} is claimed by commands of different owners that can run at the same time: {}", ids.join(", ")) });
        }
        problems
    }

    /// 🗺️ A resolved launch as `#/$defs/Launch` describes it, working directories workspace-relative.
    pub fn launch_plan_json(&self, launch: &Launch) -> serde_json::Value {
        let label = |label: &TaskLabel| serde_json::json!({ "verb": label.verb, "owner": label.owner, "subject": label.subject, "qualifier": label.qualifier, "parameters": label.parameters, "members": label.members });
        let ready = |ready: &Ready| { let mut value = serde_json::json!({ "port": ready.port, "path": ready.path }); if ready.printed { value["printed"] = true.into(); } value };
        let processes: Vec<serde_json::Value> = launch.processes.iter().map(|process| {
            let cwd = process.cwd.strip_prefix(&self.root).unwrap_or(&process.cwd).components().filter_map(|part| part.as_os_str().to_str()).collect::<Vec<_>>().join("/");
            let mut value = serde_json::json!({ "commandId": process.command_id, "cmd": process.cmd, "args": process.args, "cwd": cwd, "env": process.env, "label": label(&process.label), "longRunning": process.long_running });
            if let Some(known) = &process.ready { value["ready"] = ready(known); }
            value
        }).collect();
        let mut value = serde_json::json!({ "commandId": launch.command_id, "label": label(&launch.label), "processes": processes, "requires": launch.requires.iter().map(|required| self.launch_plan_json(required)).collect::<Vec<_>>(), "stop": launch.stop });
        if let Some(group) = &launch.group { value["group"] = group.clone().into(); }
        value
    }

    /// 🔣️ One command as `#/$defs/RegistryEntry` describes it.
    pub fn entry_json(&self, entry: &Entry) -> serde_json::Value {
        let ready = matches!(&entry.action, Action::Playground(_) | Action::Target { ready: Some(_), .. } | Action::Tool { ready: Some(_), .. }).then(|| self.resolve_with(&entry.id, &Request::default(), RunPolicy::default()).ok()).flatten().and_then(|launch| launch.processes.into_iter().find_map(|process| process.ready));
        let mut value = serde_json::json!({
            "id": entry.id, "kind": entry.kind, "verb": entry.verb, "label": entry.label, "owner": entry.owner, "longRunning": entry.long_running, "listed": entry.listed, "mutating": entry.mutating, "source": entry.source,
            "parameters": self.parameters(entry).iter().map(|parameter| { let mut row = serde_json::json!({ "id": parameter.id, "kind": parameter.kind, "values": parameter.values.iter().map(|(value, _)| value).collect::<Vec<_>>(), "required": parameter.required, "origin": parameter.origin }); if let Some(default) = &parameter.default { row["default"] = default.clone().into(); } row }).collect::<Vec<_>>(),
        });
        if let Some(ready) = ready { value["ready"] = serde_json::json!({ "port": ready.port, "path": ready.path }); if ready.printed { value["ready"]["printed"] = true.into(); } }
        if let Some(mcp) = &entry.mcp { value["mcp"] = serde_json::json!(mcp); }
        value
    }
}
// #endregion 🔖️Registry
