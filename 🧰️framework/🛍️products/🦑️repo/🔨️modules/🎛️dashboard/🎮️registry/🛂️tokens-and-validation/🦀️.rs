//! 🛂️ Tokens and validation of the registry: the name tokens of a text and the validation scope that reports problems of a declaration. See [`super`].

use super::*;

// #region 🔖️Tokens
/// 🧩️ The `{name}` tokens of a text, in order of appearance.
pub fn tokens(text: &str) -> Vec<&str> {
    let (mut found, mut rest) = (Vec::new(), text);
    while let Some(start) = rest.find('{') {
        match rest[start + 1..].find('}').map(|end| &rest[start + 1..start + 1 + end]) {
            Some(name) if is_slug(name) => { found.push(name); rest = &rest[start + name.len() + 2..]; }
            _ => rest = &rest[start + 1..],
        }
    }
    found
}

pub(super) fn substitute(text: &str, value: &dyn Fn(&str) -> Result<String, String>) -> Result<String, String> {
    let (mut output, mut rest) = (String::with_capacity(text.len()), text);
    while let Some(start) = rest.find('{') {
        output.push_str(&rest[..start]);
        match rest[start + 1..].find('}').map(|end| &rest[start + 1..start + 1 + end]) {
            Some(name) if is_slug(name) => { output.push_str(&value(name)?); rest = &rest[start + name.len() + 2..]; }
            _ => { output.push('{'); rest = &rest[start + 1..]; }
        }
    }
    output.push_str(rest);
    Ok(output)
}
// #endregion 🔖️Tokens

// #region 🔖️Validation
pub(super) struct Scope<'a> { pub(super) file: &'a str, pub(super) at: String, pub(super) problems: &'a mut Vec<Problem> }

impl Scope<'_> {
    pub(super) fn localized(&mut self, name: &str, value: Option<&LocalizedText>) -> Option<LocalizedText> {
        let value = value?;
        if value.valid() { Some(value.clone()) } else { self.within(name).report("both en and de need non-whitespace single-line text of at most 1024 characters without terminal controls"); None }
    }

    pub(super) fn presentation(&mut self, entry: &mut Entry, title: Option<&LocalizedText>, description: Option<&LocalizedText>) {
        entry.title = self.localized("title", title);
        entry.description = self.localized("description", description);
        entry.relabel();
    }


    pub(super) fn report(&mut self, message: impl Into<String>) { self.problems.push(Problem { file: self.file.to_string(), at: self.at.clone(), message: message.into() }); }
    pub(super) fn within(&mut self, part: impl std::fmt::Display) -> Scope<'_> { Scope { file: self.file, at: if self.at.is_empty() { part.to_string() } else { format!("{}.{part}", self.at) }, problems: self.problems } }
    pub(super) fn text(&mut self, what: &str, value: &str) { if value.contains('\0') || is_absolute_literal(value) { self.report(format!("{what} {value:?} is an absolute path or carries a NUL; write `{{workspace}}/…`")); } }
    pub(super) fn env(&mut self, env: &BTreeMap<String, String>) -> Vec<(String, String)> {
        for (name, value) in env {
            if !is_env_name(name) { self.report(format!("environment name {name:?} is not a variable name")); }
            if NOISE_ENV.contains(&name.as_str()) { self.report(format!("{name} belongs to the dashboard's run policy and is never declared")); }
            self.text("environment value", value);
        }
        env.iter().map(|(name, value)| (name.clone(), value.clone())).collect()
    }
    pub(super) fn effect(&mut self, env: Option<&BTreeMap<String, String>>, build_flags: Option<&Vec<String>>, args: Option<&Vec<String>>) -> Effect {
        for flag in build_flags.into_iter().flatten() { if !is_flag(flag) { self.report(format!("buildFlags entry {flag:?} is not a flag")); } }
        for arg in args.into_iter().flatten() { self.text("argument", arg); }
        Effect { env: env.map(|env| self.env(env)).unwrap_or_default(), build_flags: build_flags.cloned().unwrap_or_default(), args: args.cloned().unwrap_or_default() }
    }
    pub(super) fn mcp(&mut self, mcp: Option<&Mcp>, parameters: &[ParameterDeclaration]) {
        let Some(mcp) = mcp else { return };
        if !is_slug(&mcp.server) { self.report(format!("mcp server {:?} must be lowercase words joined by `-`, `.` or `_`", mcp.server)); }
        if mcp.clients.is_empty() { self.report("mcp needs at least one client"); }
        for (client, stated) in &mcp.clients {
            if !AGENT_CLIENTS.contains(&client.as_str()) { self.report(format!("mcp client {client:?} is none of {}", AGENT_CLIENTS.join(", "))); }
            for name in stated.parameters.keys().filter(|name| !parameters.iter().any(|declared| &declared.id == *name)) { self.report(format!("mcp client {client} states {name:?}, which the tool does not accept")); }
        }
    }
    pub(super) fn verb(&mut self, verb: Option<&String>) { if let Some(verb) = verb { if !is_verb(verb) { self.report(format!("verb {verb:?} must be a lowercase word")); } } }
    pub(super) fn id(&mut self, id: &str) -> bool { if is_slug(id) { true } else { self.report(format!("id {id:?} must be lowercase words joined by `-`, `.` or `_`")); false } }
    fn references(&mut self, what: &str, references: &[String]) {
        for (index, reference) in references.iter().enumerate() {
            if !is_reference(reference) { self.report(format!("{what} {reference:?} must be `<project>:<target>[:configuration]`, `playground:<variant>`, `tool:<project>/<id>` or `ticket:<ticket id>/<id>`")); }
            if references[..index].contains(reference) { self.report(format!("{what} {reference:?} is listed twice")); }
        }
    }
    /// 🧐️ Validates references and pins/environment of `requires` entries and compound members.
    pub(super) fn members(&mut self, what: &str, declarations: &[MemberDeclaration]) -> Vec<Member> {
        let mut members: Vec<Member> = Vec::new();
        for (position, declaration) in declarations.iter().enumerate() {
            let mut scope = self.within(format!("{what}[{position}]"));
            scope.references(what, std::slice::from_ref(&declaration.run));
            if declarations[..position].iter().any(|known| known.run == declaration.run && known.parameters == declaration.parameters && known.env == declaration.env) { scope.report(format!("{what} {:?} is listed twice", declaration.run)); }
            for (key, value) in &declaration.parameters {
                if !is_slug(key) { scope.report(format!("parameter name {key:?} must be a parameter id")); }
                if !(value.is_string() || value.is_boolean() || value.is_i64() || value.is_u64()) { scope.report(format!("parameter {key:?} must be text, a number or a boolean")); }
                if let Some(text) = value.as_str() { scope.text("parameter value", text); }
            }
            let env = scope.env(&declaration.env);
            members.push(Member { run: declaration.run.clone(), pins: declaration.parameters.iter().map(|(key, value)| (key.clone(), pin_text(value))).collect(), env });
        }
        members
    }
    pub(super) fn ready(&mut self, ready: Option<&ReadyDeclaration>) -> Option<ReadySpec> {
        let ready = ready?;
        let mut scope = self.within("ready");
        if ready.port.is_none() && ready.port_env.is_none() { scope.report("needs `port` or `portEnv`"); }
        if ready.port.is_some_and(|port| port == 0 || port > 65535) { scope.report("`port` must be between 1 and 65535"); }
        if let Some(name) = &ready.port_env { if !is_env_name(name) && !is_slug(name) { scope.report(format!("portEnv {name:?} names neither a variable nor a parameter")); } }
        if let Some(path) = &ready.path { if !path.starts_with(['/', '?', '#']) || path.contains(|c: char| c.is_whitespace() || c == '\0') { scope.report(format!("path {path:?} must start with `/`, `?` or `#` and contain no whitespace")); } }
        if ready.printed == Some(true) && ready.path.is_some() { scope.report("`printed` takes the whole printed address and leaves no room for `path`"); }
        Some(ReadySpec { port: ready.port.and_then(|port| u16::try_from(port).ok()).filter(|port| *port > 0), port_env: ready.port_env.clone(), path: ready.path.clone().unwrap_or_default(), printed: ready.printed.unwrap_or(false) })
    }
    pub(super) fn parameters(&mut self, declarations: &[ParameterDeclaration], origin: Origin) -> Vec<Parameter> {
        let mut parameters: Vec<Parameter> = Vec::new();
        for (index, declaration) in declarations.iter().enumerate() {
            let mut scope = self.within(format!("parameters[{index}]"));
            let before = scope.problems.len();
            scope.id(&declaration.id);
            if ["workspace", "port"].contains(&declaration.id.as_str()) { scope.report(format!("parameter id {:?} is a built-in token", declaration.id)); }
            if parameters.iter().any(|known| known.id == declaration.id) { scope.report(format!("parameter {:?} is declared twice", declaration.id)); }
            if declaration.applies_to.is_some() && origin != Origin::Axis { scope.report("`appliesTo` belongs to the global axes of the workspace root project"); }
            if let Some(suggestions) = &declaration.suggestions {
                if declaration.kind != ParameterKind::Text { scope.report("`suggestions` belongs to text parameters"); }
                if suggestions.source == SuggestionSource::Projects && (suggestions.project_parameter.is_some() || suggestions.directory_parameter.is_some()) { scope.report("project suggestions have no related fields"); }
                for related in [&suggestions.project_parameter, &suggestions.directory_parameter].into_iter().flatten() {
                    if related == &declaration.id || !declarations.iter().any(|known| &known.id == related && known.kind == ParameterKind::Text) { scope.report(format!("suggestion field {related:?} must name another text parameter")); }
                }
            }
            let applies = declaration.applies_to.clone().unwrap_or_default();
            if applies.all == Some(false) { scope.report("`appliesTo.all` must be true"); }
            if applies.all.is_some() && applies.verbs.is_some() { scope.report("`appliesTo` cannot combine `all` and `verbs`"); }
            if declaration.applies_to.is_some() && applies.all != Some(true) && applies.verbs.is_none() && applies.playground != Some(true) { scope.report("`appliesTo` selects nothing"); }
            for verb in applies.verbs.iter().flatten() { if !is_verb(verb) { scope.report(format!("verb {verb:?} must be a lowercase word")); } }
            if applies.verbs.as_ref().is_some_and(Vec::is_empty) { scope.report("`appliesTo.verbs` lists no verb"); }
            let forbid = |scope: &mut Scope<'_>, kind: &str, members: &[(&str, bool)]| for (name, present) in members { if *present { scope.report(format!("a {kind} has no `{name}`")); } };
            let (effects, values, destinations) = ([("env", declaration.env.is_some()), ("buildFlags", declaration.build_flags.is_some()), ("args", declaration.args.is_some())], [("values", declaration.values.is_some())], [("valueEnv", declaration.value_env.is_some()), ("valueFlag", declaration.value_flag.is_some()), ("valuePositional", declaration.value_positional.is_some())]);
            let mut parameter = Parameter { id: declaration.id.clone(), title: scope.localized("title", declaration.title.as_ref()), description: scope.localized("description", declaration.description.as_ref()), kind: declaration.kind, origin, default: None, required: declaration.required.unwrap_or(false), values: Vec::new(), effect: Effect::default(), destination: Destination::Token, suggestions: declaration.suggestions.clone(), all: applies.all.unwrap_or(false), verbs: applies.verbs.clone(), playground: applies.playground.unwrap_or(false), scoped: declaration.applies_to.is_some() };
            match declaration.kind {
                ParameterKind::Choice => {
                    forbid(&mut scope, "choice", &effects); forbid(&mut scope, "choice", &destinations);
                    let values = declaration.values.clone().unwrap_or_default();
                    if values.is_empty() { scope.report("a choice needs `values`"); }
                    for (position, value) in values.iter().enumerate() {
                        let mut scope = scope.within(format!("values[{position}]"));
                        if value.id.is_empty() || value.id.len() > 128 || value.id.contains(|c: char| c.is_whitespace() || c == '=' || c == '\0') { scope.report(format!("value id {:?} must be one word without `=`", value.id)); }
                        if values[..position].iter().any(|known| known.id == value.id) { scope.report(format!("value {:?} is declared twice", value.id)); }
                        let effect = scope.effect(value.env.as_ref(), value.build_flags.as_ref(), value.args.as_ref());
                        parameter.values.push((value.id.clone(), effect));
                    }
                    match &declaration.default { None => {} Some(serde_json::Value::String(value)) if values.iter().any(|known| &known.id == value) => parameter.default = Some(value.clone()), Some(other) => scope.report(format!("default {other} is not one of the values")) }
                }
                ParameterKind::Flag => {
                    forbid(&mut scope, "flag", &values); forbid(&mut scope, "flag", &destinations);
                    if effects.iter().all(|(_, present)| !present) { scope.report("a flag needs `env`, `buildFlags` or `args`"); }
                    parameter.effect = scope.effect(declaration.env.as_ref(), declaration.build_flags.as_ref(), declaration.args.as_ref());
                    match &declaration.default { None | Some(serde_json::Value::Bool(false)) => {} Some(serde_json::Value::Bool(true)) => parameter.default = Some("true".into()), Some(other) => scope.report(format!("default {other} must be `true` or `false`")) }
                }
                ParameterKind::Text => {
                    forbid(&mut scope, "text", &effects); forbid(&mut scope, "text", &values);
                    if destinations.iter().filter(|(_, present)| *present).count() > 1 { scope.report("a text goes to one of `valueEnv`, `valueFlag` or `valuePositional`"); }
                    if declaration.value_positional == Some(false) { scope.report("`valuePositional` is `true` or absent"); }
                    parameter.destination = match (&declaration.value_env, &declaration.value_flag, declaration.value_positional) {
                        (Some(name), _, _) => {
                            if !is_env_name(name) { scope.report(format!("valueEnv {name:?} is not a variable name")); }
                            if NOISE_ENV.contains(&name.as_str()) { scope.report(format!("{name} belongs to the dashboard's run policy and is never declared")); }
                            Destination::Env(name.clone())
                        }
                        (_, Some(flag), _) => { if !is_value_flag(flag) { scope.report(format!("valueFlag {flag:?} is not a flag")); } Destination::Flag(flag.clone()) }
                        (_, _, Some(true)) => Destination::Positional,
                        _ => Destination::Token,
                    };
                    match &declaration.default { None => {} Some(serde_json::Value::String(value)) => { scope.text("default", value); parameter.default = Some(value.clone()); } Some(other) => scope.report(format!("default {other} must be text")) }
                }
            }
            if scope.problems.len() == before { parameters.push(parameter); }
        }
        parameters
    }
}

pub(super) fn decode<T: serde::de::DeserializeOwned>(file: &str, at: &str, value: &serde_json::Value, problems: &mut Vec<Problem>) -> Option<T> {
    serde_json::from_value(value.clone()).map_err(|error| problems.push(Problem { file: file.to_string(), at: at.to_string(), message: error.to_string() })).ok()
}

fn pin_text(value: &serde_json::Value) -> String { value.as_str().map_or_else(|| value.to_string(), str::to_string) }
// #endregion 🔖️Validation
