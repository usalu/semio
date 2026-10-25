//! 🚀️ Resolve and launch of the registry: the resolution of an invocation into a launch and the process plan of an entry. See [`super`].

use super::*;

// #region 🔖️Registry
impl Registry {
    /// 🏎️ Resolves a command with the request under the dashboard's current run policy.
    pub fn resolve(&self, id: &str, request: &Request) -> Result<Launch, String> {
        let policy = RunPolicy::current();
        let launch = self.resolve_with(id, request, policy)?;
        let reuses = |launch: &Launch| launch.processes.iter().any(|process| !process.long_running && invokes_build(&process.cmd));
        if !(reuses(&launch) || launch.requires.iter().any(reuses)) || !self.graph_is_current() { return Ok(launch); }
        self.resolve_with(id, request, RunPolicy { reuse_published_graph: true, ..policy })
    }

    /// 🧪️ Resolves a command under an explicit run policy: a pure function of the registry.
    pub fn resolve_with(&self, id: &str, request: &Request, policy: RunPolicy) -> Result<Launch, String> {
        for (key, value) in &request.env {
            if !is_env_name(key) { return Err(format!("extra environment name {key:?} is not a variable name")); }
            if NOISE_ENV.contains(&key.as_str()) { return Err(format!("{key} belongs to the dashboard's run policy and cannot be set")); }
            if value.contains('\0') { return Err(format!("extra environment {key:?} carries a NUL")); }
        }
        let mut launch = self.launch(id, &request.parameters, &request.args, &request.env, policy, &mut Vec::new())?;
        let workspace = self.root.display().to_string();
        for process in &mut launch.processes {
            let port = process.ready.as_ref().map(|ready| ready.port.to_string());
            for (key, value) in request.env.iter().filter(|(_, value)| value.contains("{port}")) {
                let expanded = substitute(value, &|name| match name {
                    "workspace" => Ok(workspace.clone()),
                    "port" => port.clone().ok_or_else(|| format!("extra environment {key}: `{{port}}` needs a command with a ready port")),
                    other => request.parameters.iter().rev().find(|(known, _)| known == other).map(|(_, chosen)| chosen.clone()).ok_or_else(|| format!("extra environment {key}: `{{{other}}}` names no chosen parameter")),
                })?;
                set_env(&mut process.env, key, &expanded);
            }
        }
        Ok(launch)
    }

    pub(super) fn launch(&self, id: &str, chosen: &[(String, String)], extra: &[String], extra_env: &[(String, String)], policy: RunPolicy, stack: &mut Vec<String>) -> Result<Launch, String> {
        let (entry, configuration) = self.member(id).ok_or_else(|| format!("unknown command {id:?}"))?;
        if stack.iter().any(|known| known == &entry.id) { return Err(format!("{} requires itself through {}", entry.id, stack.join(" → "))); }
        stack.push(entry.id.clone());
        let mut requires: Vec<Member> = Vec::new();
        let (processes, group, stop) = match &entry.action {
            Action::Compound { members, stop } => {
                if !extra.is_empty() { return Err(format!("{} is a compound and takes no extra arguments", entry.id)); }
                let accepted = self.parameters(entry);
                for (key, _) in chosen { if !accepted.iter().any(|parameter| &parameter.id == key) { return Err(unknown_parameter(&entry.id, key, &accepted)); } }
                let mut processes = Vec::new();
                let workspace = self.root.display().to_string();
                for member in members {
                    let (target, configuration) = self.member(&member.run).ok_or_else(|| format!("{}: member {:?} names no command", entry.id, member.run))?;
                    if matches!(target.action, Action::Compound { .. }) { return Err(format!("{}: member {:?} is a compound; compounds do not nest", entry.id, member.run)); }
                    let offered = self.parameters(target);
                    let passed: Vec<(String, String)> = chosen.iter().filter(|(key, _)| offered.iter().any(|parameter| &parameter.id == key) && !member.pins.iter().any(|(pin, _)| pin == key)).cloned().collect();
                    let (mut process, needs) = self.process(target, configuration, &passed, &member.pins, &[], extra_env, policy).map_err(|error| format!("{}: member {}: {error}", entry.id, member.run))?;
                    for (key, value) in &member.env { set_env(&mut process.env, key, &value.replace("{workspace}", &workspace)); }
                    for need in needs { if !requires.contains(&need) { requires.push(need); } }
                    processes.push(process);
                }
                (processes, Some(entry.id.clone()), *stop)
            }
            _ => { let (process, needs) = self.process(entry, configuration, chosen, &[], extra, extra_env, policy)?; requires = needs; (vec![process], None, Stop::Independent) }
        };
        let workspace = self.root.display().to_string();
        let requires = requires.iter().map(|need| {
            let mut launch = self.launch(&need.run, &need.pins, &[], &[], policy, stack)?;
            for process in &mut launch.processes { for (key, value) in &need.env { set_env(&mut process.env, key, &value.replace("{workspace}", &workspace)); } }
            Ok(launch)
        }).collect::<Result<Vec<_>, String>>()?;
        let requires = requires.into_iter().filter(|need| group.is_none() || !need.processes.iter().all(|process| processes.contains(process))).collect();
        stack.pop();
        let label = match &entry.action {
            Action::Compound { .. } => TaskLabel { verb: entry.verb.clone(), owner: entry.owner.clone(), subject: entry.subject.clone(), qualifier: String::new(), parameters: chosen.to_vec(), members: processes.len() as u16 },
            _ => processes[0].label.clone(),
        };
        Ok(Launch { command_id: if group.is_some() { entry.id.clone() } else { processes[0].command_id.clone() }, label, processes, requires, group, stop })
    }

    pub(super) fn process(&self, entry: &Entry, configuration: Option<&str>, chosen: &[(String, String)], pins: &[(String, String)], extra: &[String], extra_env: &[(String, String)], policy: RunPolicy) -> Result<(LaunchProcess, Vec<Member>), String> {
        let parameters = self.parameters(entry);
        let workspace = self.root.display().to_string();
        let mut pins = pins.to_vec();
        if let Some(configuration) = configuration { pins.push((CONFIGURATION.into(), configuration.to_string())); }
        for (key, value) in chosen.iter().chain(&pins) {
            if !parameters.iter().any(|parameter| &parameter.id == key) { return Err(unknown_parameter(&entry.id, key, &parameters)); }
            if value.contains('\0') { return Err(format!("{}: parameter {key:?} carries a NUL", entry.id)); }
        }
        let mut values: Vec<Option<Value>> = Vec::with_capacity(parameters.len());
        for parameter in &parameters {
            let typed = chosen.iter().rev().find(|(key, _)| key == &parameter.id).map(|(_, text)| Value { text: canonical_value(parameter, text), stated: true });
            let declared = pins.iter().find(|(key, _)| key == &parameter.id).map(|(_, text)| (text, true)).or_else(|| parameter.default.as_ref().map(|text| (text, false)));
            let value = match (typed, declared) {
                (Some(value), _) => Some(value),
                (None, Some((text, stated))) if parameter.kind == ParameterKind::Text => Some(Value { stated, text: substitute(text, &|name| if name == "workspace" { Ok(workspace.clone()) } else { parameters.iter().zip(&values).find(|(known, _)| known.id == name).and_then(|(_, value)| value.as_ref().map(|value| value.text.clone())).ok_or_else(|| format!("{}: parameter {:?} uses `{{{name}}}`, which has no value before it", entry.id, parameter.id)) })? }),
                (None, Some((text, stated))) => Some(Value { text: text.clone(), stated }),
                (None, None) => None,
            };
            match (&value, parameter.kind) {
                (None, _) if parameter.required => return Err(format!("{}: parameter {:?} is required", entry.id, parameter.id)),
                (Some(value), ParameterKind::Choice) if !parameter.values.iter().any(|(known, _)| known == &value.text) => return Err(format!("{}: parameter {:?} has no value {:?}; choose {}", entry.id, parameter.id, value.text, parameter.values.iter().map(|(known, _)| known.as_str()).collect::<Vec<_>>().join(", "))),
                (Some(value), ParameterKind::Flag) if value.text != "true" && value.text != "false" => return Err(format!("{}: flag {:?} is `true` or `false`, not {:?}", entry.id, parameter.id, value.text)),
                _ => {}
            }
            values.push(value);
        }
        let value_of = |id: &str| parameters.iter().zip(&values).find(|(parameter, _)| parameter.id == id).and_then(|(_, value)| value.as_ref().map(|value| value.text.as_str()));
        let (mut env, mut build_flags, mut args): (Vec<(String, Text)>, Vec<String>, Vec<Text>) = Default::default();
        let put = |env: &mut Vec<(String, Text)>, key: &str, text: Text| match env.iter_mut().find(|(name, _)| name == key) { Some(pair) => pair.1 = text, None => env.push((key.to_string(), text)) };
        if let Action::Tool { env: stated, .. } = &entry.action { for (key, text) in stated { put(&mut env, key, Text::Declared(text.clone())); } }
        for (parameter, value) in parameters.iter().zip(&values) {
            let Some(Value { text: value, .. }) = value else { continue };
            let effect = match parameter.kind { ParameterKind::Choice => parameter.values.iter().find(|(known, _)| known == value).map(|(_, effect)| effect), ParameterKind::Flag => (value == "true").then_some(&parameter.effect), ParameterKind::Text => None };
            for (key, text) in effect.iter().flat_map(|effect| &effect.env) { put(&mut env, key, Text::Declared(text.clone())); }
            build_flags.extend(effect.iter().flat_map(|effect| effect.build_flags.iter().cloned()));
            args.extend(effect.iter().flat_map(|effect| effect.args.iter().cloned().map(Text::Declared)));
            match (&parameter.destination, parameter.kind) {
                (Destination::Env(name), ParameterKind::Text) => put(&mut env, name, Text::Literal(value.clone())),
                (Destination::Flag(flag), ParameterKind::Text) if flag.ends_with('=') => args.push(Text::Literal(format!("{flag}{value}"))),
                (Destination::Flag(flag), ParameterKind::Text) => args.extend([Text::Literal(flag.clone()), Text::Literal(value.clone())]),
                (Destination::Positional, ParameterKind::Text) => args.push(Text::Literal(value.clone())),
                _ => {}
            }
        }
        let named = |name: &str| -> Result<String, String> { if name == "workspace" { Ok(workspace.clone()) } else { value_of(name).map(str::to_string).ok_or_else(|| format!("{}: `{{{name}}}` has no value; choose parameter {name:?}", entry.id)) } };
        for (key, text) in extra_env.iter().filter(|(_, text)| !text.contains("{port}")) {
            let expanded = substitute(text, &|name| named(name).map_err(|_| format!("extra environment {key}: `{{{name}}}` names no chosen parameter")))?;
            put(&mut env, key, Text::Literal(expanded));
        }
        let (ready_spec, requires) = match &entry.action { Action::Target { ready, requires, .. } | Action::Tool { ready, requires, .. } => (ready.clone(), requires.clone()), _ => (None, Vec::new()) };
        if matches!(entry.action, Action::Tool { .. }) { build_flags.clear(); }
        let mut ready = None;
        if let Some(spec) = &ready_spec {
            let stated = match spec.port_env.as_ref().and_then(|name| env.iter().find(|(key, _)| key == name).map(|(_, text)| text).cloned().or_else(|| parameters.iter().find(|parameter| &parameter.id == name).and_then(|parameter| match &parameter.destination { Destination::Env(variable) => env.iter().find(|(key, _)| key == variable).map(|(_, text)| text.clone()), _ => None })).or_else(|| value_of(name).map(|value| Text::Literal(value.to_string())))) {
                Some(Text::Declared(text)) => Some(substitute(&text, &|name| if name == "port" { Err(format!("{}: the ready port is defined through itself", entry.id)) } else { named(name) })?),
                Some(Text::Literal(text)) => Some(text),
                None => None,
            };
            let port = match stated {
                Some(text) => text.parse::<u16>().ok().filter(|port| *port > 0).ok_or_else(|| format!("{}: ready port {text:?} is not a port", entry.id))?,
                None => spec.port.ok_or_else(|| format!("{}: ready.portEnv {:?} has no value; choose it or declare `port`", entry.id, spec.port_env.clone().unwrap_or_default()))?,
            };
            if let Some(name) = spec.port_env.as_ref().filter(|name| is_env_name(name) && !env.iter().any(|(key, _)| &key == name) && !parameters.iter().any(|parameter| &&parameter.id == name)) { env.push((name.clone(), Text::Literal(port.to_string()))); }
            ready = Some(Ready { port, path: spec.path.clone(), printed: spec.printed });
        }
        let port = ready.as_ref().map(|ready| ready.port);
        let full = |text: &Text| -> Result<String, String> { match text { Text::Literal(text) => Ok(text.clone()), Text::Declared(text) => substitute(text, &|name| if name == "port" { port.map(|port| port.to_string()).ok_or_else(|| format!("{}: `{{port}}` needs a `ready` declaration", entry.id)) } else { named(name) }) } };
        let mut env = env.iter().map(|(key, text)| Ok((key.clone(), full(text)?))).collect::<Result<Vec<(String, String)>, String>>()?;
        let mut args = args.iter().map(&full).collect::<Result<Vec<_>, _>>()?;
        args.extend(extra.iter().cloned());
        let tail = |mut head: Vec<String>, build_flags: Vec<String>, args: Vec<String>| { for flag in build_flags { if !head.contains(&flag) { head.push(flag); } } if !args.is_empty() { head.push("--".into()); head.extend(args); } head };
        let mut label = TaskLabel { verb: entry.verb.clone(), owner: entry.owner.clone(), subject: entry.subject.clone(), qualifier: entry.qualifier.clone(), parameters: Vec::new(), members: 0 };
        let mut command_id = entry.id.clone();
        let mut hidden: &[&str] = &[];
        let (cmd, args, cwd, build) = match &entry.action {
            Action::Target { project, target, .. } => {
                let configuration = value_of(CONFIGURATION).filter(|_| parameters.iter().any(|parameter| parameter.origin == Origin::Configuration));
                if let Some(configuration) = configuration { command_id = format!("{command_id}:{configuration}"); label.qualifier = if label.qualifier.is_empty() { configuration.to_string() } else { format!("{}:{configuration}", label.qualifier) }; hidden = &[CONFIGURATION]; }
                ("repo".to_string(), tail(vec!["run".into(), configuration.map_or_else(|| format!("{project}:{target}"), |configuration| format!("{project}:{target}:{configuration}"))], build_flags, args), self.root.clone(), true)
            }
            Action::Group { targets, projects } => ("repo".to_string(), tail(vec!["run-many".into(), "-t".into(), targets.join(","), "-p".into(), projects.join(",")], build_flags, args), self.root.clone(), true),
            Action::Tool { command, cwd, .. } => {
                let mut words = command.iter().map(|word| full(&Text::Declared(word.clone()))).collect::<Result<Vec<_>, _>>()?;
                let cmd = words.remove(0);
                words.extend(args);
                let build = invokes_build(&cmd);
                (cmd, words, full(&Text::Declared(cwd.clone()))?.split('/').filter(|part| !part.is_empty()).fold(self.root.clone(), |path, part| path.join(part)), build)
            }
            Action::Playground(playground) => {
                let renderer = value_of("renderer").unwrap_or(RENDERERS[0]);
                let slot = value_of("user-slot").and_then(|slot| slot.parse::<usize>().ok());
                let (port, slots) = if renderer == "react" { (playground.react, &playground.user_react) } else { (playground.wgpu, &playground.user_wgpu) };
                let port = match slot { Some(slot) => *slots.get(slot.wrapping_sub(1)).ok_or_else(|| format!("{}: renderer {renderer} has no user slot {slot}", entry.id))?, None => port };
                let lock = |id: &str| value_of(id).map_or(dashboard_core::options::Lock::All, |value| dashboard_core::options::Lock::Individual(value.to_string()));
                let row = dashboard_core::catalog::PlaygroundEntry { variant: playground.variant.clone(), plugin_id: playground.plugin.clone(), app: playground.app.clone(), ports: dashboard_core::catalog::Ports { react: playground.react.into(), wgpu: playground.wgpu.into() }, ..Default::default() };
                let options = dashboard_core::env_contract::DevOptions { renderer: renderer.to_string(), port: Some(port), example: lock("example"), language: lock("language"), terminology: lock("terminology"), appearance: lock("appearance"), ..Default::default() };
                let stated = std::mem::replace(&mut env, dashboard_core::env_contract::build_dev_env(&playground.variant, Some(&row), &options));
                for (key, value) in stated { set_env(&mut env, &key, &value); }
                let port = env.iter().find(|(key, _)| key == "S_OS_PORT").and_then(|(_, value)| value.parse::<u16>().ok()).unwrap_or(port);
                let target = playground_target(&playground.variant, renderer).ok_or_else(|| format!("{}: unknown renderer {renderer:?}", entry.id))?;
                let mut args = args;
                if renderer == "wgpu-native" {
                    if let Some(example) = value_of("example") { args.splice(0..0, ["--example".to_string(), example.to_string()]); }
                } else {
                    let path = playground.viewer_path.clone().filter(|_| value_of("app-role") == Some("viewer")).unwrap_or_default();
                    ready = Some(Ready { port, path, printed: false });
                    if value_of("data") == Some("true") {
                        let directory = match (slot, &playground.user_data_dir, &playground.data_dir) { (Some(slot), Some(users), _) => Some(users.replace("{N}", &slot.to_string())), (None, _, Some(base)) => Some(base.clone()), _ => None };
                        if let Some(directory) = directory { set_env(&mut env, DATA_ENV, &format!("{}/{}", self.root.display().to_string().trim_end_matches(['/', '\\']), directory.trim_start_matches('/'))); }
                    }
                }
                label.subject = format!("{}·{renderer}", playground.variant);
                label.qualifier = value_of("example").map(|example| format!("#{example}")).into_iter().chain(slot.map(|slot| format!("@{slot}"))).collect::<Vec<_>>().join(" ");
                hidden = &["renderer", "example", "user-slot"];
                ("repo".to_string(), tail(vec!["run".into(), format!("@semio-tech/framework-os-dev:{target}")], build_flags, args), self.root.clone(), true)
            }
            Action::Repo(action) => {
                if !extra.is_empty() { return Err(format!("{} takes no extra arguments", entry.id)); }
                let action = action.selected(&|id| value_of(id).map(str::to_string))?;
                action.admit_root(&self.root)?;
                match policy.repo {
                    RepoImplementation::Go => (dashboard_repo_domain::go_binary_path(&self.root).display().to_string(), action.go_argv(), self.root.clone(), false),
                    RepoImplementation::Rust => (std::env::current_exe().map_err(|error| format!("{}: {error}", entry.id))?.display().to_string(), vec!["repo-view".into(), "--action".into(), serde_json::to_string(&action).map_err(|error| error.to_string())?], self.root.clone(), false),
                }
            }
            Action::Compound { .. } => return Err(format!("{}: compounds do not nest", entry.id)),
        };
        if !matches!(entry.action, Action::Repo(_)) { policy.apply(&mut env, build, entry.long_running); }
        label.parameters = parameters.iter().zip(&values).filter_map(|(parameter, value)| value.as_ref().filter(|value| value.stated && !hidden.contains(&parameter.id.as_str()) && value.text != "false" && Some(&value.text) != parameter.default.as_ref()).map(|value| (parameter.id.clone(), value.text.clone()))).collect();
        Ok((LaunchProcess { command_id, cmd, args, cwd, env, label, ready, long_running: entry.long_running }, requires))
    }
}
// #endregion 🔖️Registry
