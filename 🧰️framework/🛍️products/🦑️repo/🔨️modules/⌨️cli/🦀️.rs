//! 🧭️ `semio`: the monorepo orchestrator CLI. It owns the verb dispatch and hands every dashboard
//! verb to `semio-framework-repo-dashboard`, which owns the command modules themselves.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/🦀️.rs

use semio_framework_repo_dashboard::{args, command_tree, daemon, playground_catalog, playground_session, plugin_registry, root_delegation, terminal, usage, workflow};
use std::io::IsTerminal;
use std::path::PathBuf;

// #region 🔖️Dispatch
/// 🚦️ Runs one `semio` invocation and returns its process exit code.
pub fn run(argv: &[String]) -> i32 {
    let root = semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    if argv.is_empty() {
        if !std::io::stdout().is_terminal() {
            usage::print();
            return 1;
        }
        return terminal::run(&root);
    }
    let parsed = args::parse(argv);
    match parsed.verb.as_str() {
        "daemon" => daemon::run(&root, &parsed),
        "workflow" => workflow::run(&root, &parsed),
        "dev" => playground_session::run(&root, &parsed),
        "catalog" => playground_catalog::run(&root, &parsed),
        "command-tree" => command_tree::run(&root, &parsed),
        "plugin" if parsed.segments.first().map(String::as_str) == Some("registry") => plugin_registry::run(&root, parsed.segments.get(1).map_or("generate", String::as_str)),
        _ => root_delegation::run(&root, &parsed),
    }
}
// #endregion 🔖️Dispatch

//#region 🧭️RepoCli

/// 🧭️ The repo command line: the event stream, its three renderers, the declared command tree and
/// one handler per verb, each a thin front for the domain crate that owns the aggregate. Twin of the
/// Go `cli` package.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go
pub mod repo_cli {
    use semio_framework_repo_codebase::{Codebase, Scope as CodebaseScope};
    use semio_framework_repo_graphql::{Executor, FsRepoContext, RecordingContext, RepoContext};
    use semio_framework_repo_identity::{emoji_text, entity as identity_entity, flat};
    use semio_framework_repo_model::{ansi, entity, DraftCreateInput, TicketStatus, TreeFilter, TreeNode, TreeNodeKind};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Map, Value as Json};
    use std::collections::{BTreeMap, BTreeSet};
    use std::io::Write;

    //#region 🔑️Events

    /// 🏷️ The start event kind.
    pub const KIND_START: &str = "start";
    /// 🏷️ The log event kind.
    pub const KIND_LOG: &str = "log";
    /// 🏷️ The progress event kind.
    pub const KIND_PROGRESS: &str = "progress";
    /// 🏷️ The result event kind.
    pub const KIND_RESULT: &str = "result";
    /// 🏷️ The error event kind.
    pub const KIND_ERROR: &str = "error";
    /// 🏷️ The done event kind.
    pub const KIND_DONE: &str = "done";

    /// ⏳️ How far a long operation has come.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct Progress {
        #[serde(default, skip_serializing_if = "is_zero")]
        pub current: i64,
        #[serde(default, skip_serializing_if = "is_zero")]
        pub total: i64,
        #[serde(default, skip_serializing_if = "is_zero")]
        pub percent: i64,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub step: String,
    }

    /// 🏺️ A produced artifact an event points at.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct Artifact {
        #[serde(rename = "type", default)]
        pub kind: String,
        #[serde(default)]
        pub uri: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub note: String,
    }

    /// ❌️ The failure an error event carries.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct ErrPayload {
        #[serde(default)]
        pub code: String,
        #[serde(default)]
        pub message: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub detail: String,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub fatal: bool,
    }

    /// 🏁️ The terminal payload of a stream.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct DonePayload {
        #[serde(rename = "exit_code", default)]
        pub exit_code: i32,
        #[serde(default)]
        pub status: String,
    }

    /// 📡️ One event of the stream every verb produces and every renderer consumes.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct Event {
        #[serde(default)]
        pub kind: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub command: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub id: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub message: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        pub level: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub progress: Option<Progress>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<Json>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub artifact: Option<Artifact>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<ErrPayload>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub done: Option<DonePayload>,
    }

    impl Event {
        /// 🟢️ The event that opens a stream.
        pub fn start(command: &str) -> Event {
            Event { kind: KIND_START.to_string(), command: command.to_string(), ..Event::default() }
        }

        /// 📦️ One result payload.
        pub fn result(command: &str, data: Json) -> Event {
            Event { kind: KIND_RESULT.to_string(), command: command.to_string(), data: Some(data), ..Event::default() }
        }

        /// 📝️ One informational line.
        pub fn log(command: &str, message: impl Into<String>) -> Event {
            Event { kind: KIND_LOG.to_string(), command: command.to_string(), message: message.into(), level: "info".to_string(), ..Event::default() }
        }

        /// ❌️ One failure.
        pub fn failure(command: &str, code: &str, message: impl Into<String>, detail: impl Into<String>) -> Event {
            Event { kind: KIND_ERROR.to_string(), command: command.to_string(), error: Some(ErrPayload { code: code.to_string(), message: message.into(), detail: detail.into(), fatal: true }), ..Event::default() }
        }

        /// 🏁️ The event that closes a stream.
        pub fn finish(exit_code: i32, status: &str) -> Event {
            Event { kind: KIND_DONE.to_string(), done: Some(DonePayload { exit_code, status: status.to_string() }), ..Event::default() }
        }
    }

    fn is_zero(value: &i64) -> bool {
        *value == 0
    }

    /// 🧯️ The error code of an internal failure.
    pub const ERR_INTERNAL: &str = "E_INTERNAL";
    /// 🧯️ The error code of an unreadable request.
    pub const ERR_PARSE: &str = "E_PARSE";
    /// 🧯️ The error code of a cancelled request.
    pub const ERR_CANCELED: &str = "E_CANCELED";

    /// 🚦️ The exit code of a successful stream.
    pub const EXIT_OK: i32 = 0;
    /// 🚦️ The exit code of a failed stream.
    pub const EXIT_ERROR: i32 = 1;
    /// 🚦️ The exit code of a misused command.
    pub const EXIT_USAGE: i32 = 2;
    /// 🚦️ The exit code of a cancelled stream.
    pub const EXIT_CANCELED: i32 = 130;

    /// 🕸️ Executes one GraphQL document as the `graphql` engine command: start, the result or the
    /// failure, done. Twin of `Engine.Run` with `CmdGraphQL`.
    pub fn graphql_events(context: &dyn RepoContext, query: &str, variables: &Map<String, Json>) -> Vec<Event> {
        let executor = Executor::new(context);
        let mut events = vec![Event::start("graphql")];
        match executor.execute(query, variables) {
            Ok(data) => {
                events.push(Event::result("graphql", data));
                events.push(Event::finish(EXIT_OK, "ok"));
            }
            Err(error) => {
                events.push(Event::failure("graphql", ERR_INTERNAL, error.message, ""));
                events.push(Event::finish(EXIT_ERROR, "error"));
            }
        }
        events
    }

    //#endregion 🔑️Events

    //#region 🐹️GoFormatting

    /// 🔢️ A float the way Go's `encoding/json` writes it.
    fn go_json_float(value: f64) -> String {
        let magnitude = value.abs();
        if magnitude != 0.0 && !(1e-6..1e21).contains(&magnitude) {
            let rendered = format!("{value:e}");
            return match rendered.split_once('e') {
                Some((mantissa, exponent)) if !exponent.starts_with('-') => format!("{mantissa}e+{exponent}"),
                _ => rendered,
            };
        }
        format!("{value}")
    }

    /// 🔢️ A float the way Go's `%v` verb prints it: shortest digits, exponent form outside `1e-4..1e6`.
    fn go_float_v(value: f64) -> String {
        if value == 0.0 {
            return "0".to_string();
        }
        let rendered = format!("{value:e}");
        let Some((mantissa, exponent)) = rendered.split_once('e') else { return rendered };
        let exponent: i32 = exponent.parse().unwrap_or_default();
        if !(-4..6).contains(&exponent) {
            let sign = if exponent < 0 { '-' } else { '+' };
            return format!("{mantissa}e{sign}{:02}", exponent.abs());
        }
        format!("{value}")
    }

    /// 🔤️ A string literal the way Go's `encoding/json` writes it with HTML escaping switched off.
    fn go_json_string(text: &str, out: &mut String) {
        out.push('"');
        for character in text.chars() {
            match character {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{2028}' => out.push_str("\\u2028"),
                '\u{2029}' => out.push_str("\\u2029"),
                control if (control as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", control as u32)),
                other => out.push(other),
            }
        }
        out.push('"');
    }

    /// 🧾️ A value as compact JSON the way Go's `encoding/json` writes it: members sorted, no HTML
    /// escaping, Go's float spelling.
    pub fn go_json(value: &Json) -> String {
        let mut out = String::new();
        write_go_json(value, &mut out);
        out
    }

    fn write_go_json(value: &Json, out: &mut String) {
        match value {
            Json::Null => out.push_str("null"),
            Json::Bool(flag) => out.push_str(if *flag { "true" } else { "false" }),
            Json::Number(number) => match (number.as_i64(), number.as_u64(), number.as_f64()) {
                (Some(integer), _, _) => out.push_str(&integer.to_string()),
                (_, Some(integer), _) => out.push_str(&integer.to_string()),
                (_, _, Some(float)) => out.push_str(&go_json_float(float)),
                _ => out.push_str("null"),
            },
            Json::String(text) => go_json_string(text, out),
            Json::Array(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    write_go_json(item, out);
                }
                out.push(']');
            }
            Json::Object(members) => {
                out.push('{');
                for (index, (key, member)) in members.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    go_json_string(key, out);
                    out.push(':');
                    write_go_json(member, out);
                }
                out.push('}');
            }
        }
    }

    /// 🧾️ An ordered record as compact JSON, members in the stated order.
    fn go_json_ordered(members: &[(&str, Json)]) -> String {
        let mut out = String::from("{");
        for (index, (key, member)) in members.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            go_json_string(key, &mut out);
            out.push(':');
            write_go_json(member, &mut out);
        }
        out.push('}');
        out
    }

    /// 🖨️ A decoded JSON value the way Go's `%v` verb prints it.
    fn go_sprint(value: &Json) -> String {
        match value {
            Json::Null => "<nil>".to_string(),
            Json::Bool(flag) => flag.to_string(),
            Json::Number(number) => go_float_v(number.as_f64().unwrap_or_default()),
            Json::String(text) => text.clone(),
            Json::Array(items) => format!("[{}]", items.iter().map(go_sprint).collect::<Vec<String>>().join(" ")),
            Json::Object(members) => format!("map[{}]", members.iter().map(|(key, member)| format!("{key}:{}", go_sprint(member))).collect::<Vec<String>>().join(" ")),
        }
    }

    /// 🖨️ A template field the way `text/template` prints it: `<no value>` for a missing member.
    fn go_field(value: Option<&Json>) -> String {
        match value {
            None | Some(Json::Null) => "<no value>".to_string(),
            Some(value) => go_sprint(value),
        }
    }

    /// ☑️ Whether `text/template`'s `if` takes a decoded JSON value as true.
    fn go_truthy(value: Option<&Json>) -> bool {
        match value {
            None | Some(Json::Null) => false,
            Some(Json::Bool(flag)) => *flag,
            Some(Json::Number(number)) => number.as_f64().unwrap_or_default() != 0.0,
            Some(Json::String(text)) => !text.is_empty(),
            Some(Json::Array(items)) => !items.is_empty(),
            Some(Json::Object(members)) => !members.is_empty(),
        }
    }

    /// ⏱️ A millisecond duration the way Go's `time.Duration.String` spells it.
    pub fn go_duration(milliseconds: i64) -> String {
        if milliseconds == 0 {
            return "0s".to_string();
        }
        let sign = if milliseconds < 0 { "-" } else { "" };
        let total = milliseconds.unsigned_abs();
        if total < 1000 {
            return format!("{sign}{total}ms");
        }
        let hours = total / 3_600_000;
        let minutes = (total % 3_600_000) / 60_000;
        let seconds = (total % 60_000) / 1000;
        let fraction = total % 1000;
        let mut out = sign.to_string();
        if hours > 0 {
            out.push_str(&format!("{hours}h"));
        }
        if hours > 0 || minutes > 0 {
            out.push_str(&format!("{minutes}m"));
        }
        out.push_str(&seconds.to_string());
        if fraction > 0 {
            out.push_str(format!(".{fraction:03}").trim_end_matches('0'));
        }
        out.push('s');
        out
    }

    //#endregion 🐹️GoFormatting

    //#region 🌩️Renderers

    /// 🖌️ `colorize` of the text templates.
    fn paint(text: &str, colour: &str, is_tty: bool) -> String {
        ansi::colorize(text, colour, is_tty)
    }

    /// 🖨️ The three renderings a stream can take.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Format {
        Json,
        Markdown,
        Text,
    }

    impl Format {
        /// 🔤️ The format a `--format` value names, the human text for anything unknown.
        pub fn parse(raw: &str) -> Format {
            match raw {
                "json" => Format::Json,
                "md" => Format::Markdown,
                _ => Format::Text,
            }
        }
    }

    /// 🖨️ What a renderer wrote and the exit code the terminal event carried.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct Rendered {
        pub out: String,
        pub err: String,
        pub exit_code: i32,
    }

    /// 🖨️ Renders one stream in one format. The human renderer takes the terminal decision and the
    /// elapsed time from the caller, so a recorded stream renders the same bytes on every host.
    pub fn render(events: &[Event], format: Format, is_tty: bool, verbose: bool, elapsed_ms: i64) -> Rendered {
        match format {
            Format::Json => render_ndjson(events),
            Format::Markdown => render_markdown_stream(events),
            Format::Text => render_human(events, is_tty, verbose, elapsed_ms),
        }
    }

    fn render_ndjson(events: &[Event]) -> Rendered {
        let mut rendered = Rendered::default();
        for event in events {
            if event.kind == KIND_DONE {
                if let Some(done) = &event.done {
                    rendered.exit_code = done.exit_code;
                }
            }
            if event.kind == KIND_RESULT {
                if let Some(data) = &event.data {
                    rendered.out.push_str(&go_json(data));
                    rendered.out.push('\n');
                }
            }
            if event.kind == KIND_ERROR {
                if let Some(error) = &event.error {
                    let mut members: Vec<(&str, Json)> = vec![("code", Json::String(error.code.clone())), ("message", Json::String(error.message.clone()))];
                    if !error.detail.is_empty() {
                        members.push(("detail", Json::String(error.detail.clone())));
                    }
                    if error.fatal {
                        members.push(("fatal", Json::Bool(true)));
                    }
                    rendered.err.push_str(&go_json_ordered(&members));
                    rendered.err.push('\n');
                }
            }
        }
        rendered
    }

    fn render_human(events: &[Event], is_tty: bool, verbose: bool, elapsed_ms: i64) -> Rendered {
        let mut rendered = Rendered::default();
        let clear = if is_tty { "\r\u{1b}[K" } else { "" };
        for event in events {
            if event.kind == KIND_DONE {
                if let Some(done) = &event.done {
                    rendered.exit_code = done.exit_code;
                    let duration = go_duration(elapsed_ms);
                    let summary = if done.exit_code != 0 {
                        format!("{} failed {}  {duration} (exit: {})", paint("failed", "red", is_tty), event.command, done.exit_code)
                    } else {
                        format!("{} done  {}  {duration}", paint("ok", "green", is_tty), event.command)
                    };
                    rendered.out.push_str(clear);
                    rendered.out.push_str(&summary);
                    rendered.out.push('\n');
                    continue;
                }
            }
            if event.kind == KIND_ERROR {
                if let Some(error) = &event.error {
                    rendered.out.push_str(clear);
                    rendered.err.push_str(&format!("{} error: {}\n", paint("error", "red", is_tty), error.message));
                    if verbose && !error.detail.is_empty() {
                        rendered.err.push_str(&error.detail);
                        rendered.err.push('\n');
                    }
                    continue;
                }
            }
            if event.kind == KIND_LOG && !event.message.is_empty() {
                rendered.out.push_str(clear);
                rendered.err.push_str(&format!("{} {}\n", paint("-", "dim", is_tty), event.message));
                continue;
            }
            if event.kind == KIND_RESULT {
                if let Some(data) = &event.data {
                    rendered.out.push_str(clear);
                    rendered.out.push_str(&format_result(&event.command, data, is_tty));
                }
            }
            if event.kind == KIND_PROGRESS {
                if let Some(progress) = &event.progress {
                    if is_tty {
                        rendered.out.push_str(&format!("\r{} {}% ({}/{}) {}", paint("...", "blue", true), progress.percent, progress.current, progress.total, progress.step));
                    } else if progress.percent % 10 == 0 && progress.percent > 0 {
                        rendered.out.push_str(&format!("progress: {}% {}\n", progress.percent, progress.step));
                    }
                }
            }
        }
        rendered
    }

    fn render_markdown_stream(events: &[Event]) -> Rendered {
        let mut rendered = Rendered::default();
        for event in events {
            if event.kind == KIND_DONE {
                if let Some(done) = &event.done {
                    rendered.exit_code = done.exit_code;
                    continue;
                }
            }
            if event.kind == KIND_RESULT {
                if let Some(data) = &event.data {
                    rendered.out.push_str(&format_markdown_result(&event.command, data));
                }
            }
            if event.kind == KIND_ERROR {
                if let Some(error) = &event.error {
                    rendered.err.push_str(&format!("**Error: {}**\n", error.message));
                    if !error.detail.is_empty() {
                        rendered.err.push_str(&format!("> {}\n", error.detail));
                    }
                }
            }
        }
        rendered
    }

    /// 📡️ The markdown a tool result carries for a stream: every result, every failure.
    pub fn events_to_markdown(events: &[Event]) -> String {
        let mut out = String::new();
        for event in events {
            if event.kind == KIND_RESULT {
                if let Some(data) = &event.data {
                    out.push_str(&format_markdown_result(&event.command, data));
                }
            }
            if event.kind == KIND_ERROR {
                if let Some(error) = &event.error {
                    out.push_str(&format!("**Error: {}**\n", error.message));
                }
            }
        }
        out
    }

    /// 🔍️ The payload a result renders: its `data` member when it wraps one, else itself.
    fn payload_of(data: &Json) -> Option<&Map<String, Json>> {
        let raw = data.as_object()?;
        Some(raw.get("data").and_then(Json::as_object).unwrap_or(raw))
    }

    /// 🔤️ The statute id a breach record names, whether its kind is a record or a bare id.
    fn breach_kind(breach: &Map<String, Json>) -> String {
        match breach.get("kind") {
            Some(Json::Object(kind)) => go_field(kind.get("id")),
            Some(Json::String(kind)) => kind.clone(),
            _ => String::new(),
        }
    }

    /// 📜️ The array members of a map member, empty when it is not an array.
    fn array_of<'a>(members: &'a Map<String, Json>, key: &str) -> Option<&'a Vec<Json>> {
        members.get(key).and_then(Json::as_array)
    }

    /// 🎯️ Renders a goal list with its tickets as the goal tree, twin of `RenderGoalTree`.
    pub fn render_goal_tree(goals: &[Json], tickets: &[Json], markdown: bool) -> String {
        let text = |value: &Map<String, Json>, key: &str| value.get(key).and_then(Json::as_str).unwrap_or_default().to_string();
        let goal_records: Vec<semio_framework_repo_tree::GoalRecord> = goals
            .iter()
            .filter_map(Json::as_object)
            .map(|goal| semio_framework_repo_tree::GoalRecord {
                id: text(goal, "id"),
                title: text(goal, "title"),
                status: text(goal, "status"),
                due_date: text(goal, "dueDate"),
                created_at: text(goal, "createdAt"),
                description: text(goal, "description"),
                ..Default::default()
            })
            .collect();
        let ticket_records: Vec<semio_framework_repo_tree::TicketRecord> = tickets
            .iter()
            .filter_map(Json::as_object)
            .map(|ticket| {
                let dates = ticket.get("date").and_then(Json::as_object);
                let created = dates.map(|dates| text(dates, "created")).filter(|created| !created.is_empty()).unwrap_or_else(|| text(ticket, "createdAt"));
                semio_framework_repo_tree::TicketRecord {
                    id: text(ticket, "id"),
                    slug: text(ticket, "slug"),
                    title: text(ticket, "title"),
                    description: text(ticket, "prompt"),
                    summary: text(ticket, "summary"),
                    goal: text(ticket, "goal"),
                    parent: text(ticket, "parent"),
                    status: text(ticket, "status"),
                    started: created,
                    finished: dates.map(|dates| text(dates, "finished")).unwrap_or_default(),
                    ..Default::default()
                }
            })
            .collect();
        let roots = semio_framework_repo_tree::build_goal_tree(&goal_records, &ticket_records);
        let format = if markdown { semio_framework_repo_tree::TreeRenderFormat::Markdown } else { semio_framework_repo_tree::TreeRenderFormat::Text };
        semio_framework_repo_tree::render_goal_tree_nodes(&roots, format, &semio_framework_repo_tree::DefaultEntityRenderer)
    }

    /// 📑️ Renders one section and its children as the section tree, twin of `RenderSectionTree`.
    pub fn render_section_tree(section: &Map<String, Json>, markdown: bool) -> String {
        fn walk(out: &mut String, section: &Map<String, Json>, prefix: &str, is_last: bool, is_root: bool, markdown: bool) {
            let mut data = Map::new();
            data.insert("path".to_string(), section.get("path").cloned().filter(Json::is_string).unwrap_or(Json::String(String::new())));
            data.insert("name".to_string(), section.get("name").cloned().filter(Json::is_string).unwrap_or(Json::String(String::new())));
            for key in ["startLine", "endLine"] {
                data.insert(key.to_string(), Json::from(section.get(key).and_then(Json::as_f64).unwrap_or_default()));
            }
            let children: Vec<&Map<String, Json>> = section.get("children").and_then(Json::as_array).map(|children| children.iter().filter_map(Json::as_object).collect()).unwrap_or_default();
            let last = children.len().saturating_sub(1);
            if markdown {
                out.push_str(&format!("{prefix}{}\n", entity::render_markdown("section", &data)));
                for (index, child) in children.iter().enumerate() {
                    walk(out, child, &format!("{prefix}  "), index == last, false, markdown);
                }
                return;
            }
            let connector = match (is_root, is_last) {
                (true, _) => "",
                (false, true) => "└️─️─️ ",
                (false, false) => "├️─️─️ ",
            };
            out.push_str(&format!("{prefix}{connector}{}\n", entity::render_human("section", &data, false)));
            let next = match (is_root, is_last) {
                (true, _) => prefix.to_string(),
                (false, true) => format!("{prefix}    "),
                (false, false) => format!("{prefix}│️   "),
            };
            for (index, child) in children.iter().enumerate() {
                walk(out, child, &next, index == last, false, markdown);
            }
        }
        let mut out = String::new();
        walk(&mut out, section, "", true, true, markdown);
        out
    }

    /// 🖥️ One result payload as the human lines it prints, twin of `formatResult`.
    pub fn format_result(_command: &str, data: &Json, is_tty: bool) -> String {
        let Some(payload) = payload_of(data) else { return format!("{} {}\n", paint("->", "blue", is_tty), go_json(data)) };
        if let Some(markdown) = payload.get("markdown").and_then(Json::as_str) {
            return markdown.to_string();
        }
        let mut out = String::new();
        if let Some(analyze) = payload.get("analyze").and_then(Json::as_object) {
            if let Some(metrics) = analyze.get("metrics").and_then(Json::as_object) {
                let autofixable = if go_truthy(metrics.get("autofixable")) { format!(" ({} autofixable)", go_field(metrics.get("autofixable"))) } else { String::new() };
                out.push_str(&format!("{} found {} breachs{autofixable}\n", paint("->", "blue", is_tty), go_field(metrics.get("total"))));
            }
            if let Some(breachs) = array_of(analyze, "breachs") {
                for breach in breachs.iter().filter_map(Json::as_object) {
                    let location = format!("{}:{}", go_sprint(breach.get("scope").unwrap_or(&Json::Null)), go_sprint(breach.get("line").unwrap_or(&Json::Null)));
                    out.push_str(&format!("  {} {} {} {}\n", paint("breach", "red", is_tty), breach_kind(breach), paint(&location, "dim", is_tty), go_sprint(breach.get("summary").unwrap_or(&Json::Null))));
                }
            }
            if !out.is_empty() {
                return out;
            }
        }
        if let Some(fix) = payload.get("fix").and_then(Json::as_object) {
            return format!("{} fixed {} breachs ({} remaining)\n", paint("->", "blue", is_tty), go_field(fix.get("fixed")), go_field(fix.get("remaining")));
        }
        if let Some(repo) = payload.get("repo").and_then(Json::as_object) {
            if let Some(goals) = array_of(repo, "goals") {
                return render_goal_tree(goals, array_of(repo, "tickets").map(Vec::as_slice).unwrap_or_default(), false);
            }
        }
        let width = if is_tty { ansi::terminal_width() } else { 0 };
        let line = |kind: &str, entity_data: &Map<String, Json>| {
            let rendered = entity::render_human(kind, entity_data, is_tty);
            if width > 0 {
                ansi::truncate(&rendered, width)
            } else {
                rendered
            }
        };
        let list = |kind: &str, items: &[Json]| items.iter().filter_map(Json::as_object).map(|item| format!("{}\n", line(kind, item))).collect::<String>();
        if let Some(repo) = payload.get("repo").and_then(Json::as_object) {
            for (key, kind) in [("tickets", "ticket"), ("goals", "goal"), ("bundles", "bundle"), ("technologies", "technology"), ("folders", "folder"), ("files", "file"), ("contributors", "contributor"), ("policies", "policy"), ("statutes", "statute")] {
                if let Some(items) = array_of(repo, key) {
                    return list(kind, items);
                }
            }
        }
        for (key, kind) in [("todos", "todo"), ("sections", "section"), ("definitions", "definition"), ("drafts", "draft")] {
            if let Some(items) = array_of(payload, key) {
                return list(kind, items);
            }
        }
        for key in ["ticket", "goal", "bundle", "folder", "policy", "contributor", "draft", "todo", "technology", "definition"] {
            if let Some(found) = payload.get(key).and_then(Json::as_object) {
                let mut found = found.clone();
                if let Some(id) = payload.get("id").filter(|id| id.is_string()) {
                    found.entry("id".to_string()).or_insert_with(|| id.clone());
                }
                return format!("{}\n", entity::render_human(key, &found, is_tty));
            }
        }
        if let Some(file) = payload.get("file").and_then(Json::as_object) {
            out.push_str(&format!("{}\n", line("file", file)));
            fn sections(out: &mut String, items: &[Json], indent: &str, line: &dyn Fn(&str, &Map<String, Json>) -> String) {
                for section in items.iter().filter_map(Json::as_object) {
                    out.push_str(&format!("{indent}{}\n", line("section", section)));
                    if let Some(children) = section.get("children").and_then(Json::as_array).filter(|children| !children.is_empty()) {
                        sections(out, children, &format!("{indent}  "), line);
                    }
                }
            }
            if let Some(items) = array_of(file, "sections") {
                sections(&mut out, items, "  ", &line);
            }
            if let Some(definitions) = array_of(file, "definitions") {
                for definition in definitions.iter().filter_map(Json::as_object) {
                    out.push_str(&format!("  {}\n", line("definition", definition)));
                }
            }
            return out;
        }
        if let Some(section) = payload.get("section").and_then(Json::as_object) {
            return render_section_tree(section, false);
        }
        for (key, value) in payload {
            if let Some(found) = value.as_object() {
                let kind = entity::infer_kind(key);
                if !kind.is_empty() {
                    return format!("{}\n", entity::render_human(kind, found, is_tty));
                }
            }
        }
        if let Some(id) = payload.get("id").and_then(Json::as_str) {
            if let Some(path) = payload.get("path").and_then(Json::as_str) {
                return format!("{} {id} {path}\n", paint("->", "blue", is_tty));
            }
            return format!("{} item {id}\n", paint("->", "blue", is_tty));
        }
        format!("{}\n", entity::render_human("root", payload, is_tty))
    }

    /// 📰️ One result payload as the markdown it prints, twin of `formatMarkdownResult`.
    pub fn format_markdown_result(command: &str, data: &Json) -> String {
        let Some(payload) = payload_of(data) else {
            let mut root = Map::new();
            root.insert("name".to_string(), Json::String(go_json(data)));
            return format!("{}\n", entity::render_markdown_link("root", &root));
        };
        if let Some(markdown) = payload.get("markdown").and_then(Json::as_str) {
            return markdown.to_string();
        }
        let mut out = String::new();
        if let Some(analyze) = payload.get("analyze").and_then(Json::as_object) {
            if let Some(metrics) = analyze.get("metrics").and_then(Json::as_object) {
                out.push_str(&format!("- **Total Breachs**: {}\n", go_field(metrics.get("total"))));
                if metrics.get("autofixable").is_some_and(|value| !value.is_null()) {
                    out.push_str(&format!("- **Autofixable**: {}\n", go_field(metrics.get("autofixable"))));
                }
            }
            if let Some(breachs) = array_of(analyze, "breachs") {
                for breach in breachs.iter().filter_map(Json::as_object) {
                    let kind = breach_kind(breach);
                    out.push_str(&format!(
                        "- [{kind}](repo://statute/{}) - {}:{} - {}\n",
                        entity::path_to_uri_path(&kind),
                        go_sprint(breach.get("scope").unwrap_or(&Json::Null)),
                        go_sprint(breach.get("line").unwrap_or(&Json::Null)),
                        go_sprint(breach.get("summary").unwrap_or(&Json::Null))
                    ));
                }
            }
            return out;
        }
        if let Some(repo) = payload.get("repo").and_then(Json::as_object) {
            if let Some(goals) = array_of(repo, "goals") {
                return render_goal_tree(goals, array_of(repo, "tickets").map(Vec::as_slice).unwrap_or_default(), true);
            }
        }
        let wrapped = payload.get("repo").and_then(Json::as_object);
        for (in_repo, key, kind) in [
            (true, "tickets", "ticket"),
            (true, "goals", "goal"),
            (true, "bundles", "bundle"),
            (true, "technologies", "technology"),
            (true, "folders", "folder"),
            (true, "files", "file"),
            (true, "contributors", "contributor"),
            (true, "policies", "policy"),
            (true, "statutes", "statute"),
            (false, "todos", "todo"),
            (false, "sections", "section"),
            (false, "definitions", "definition"),
            (false, "drafts", "draft"),
        ] {
            let container = if in_repo { wrapped } else { Some(payload) };
            if let Some(items) = container.and_then(|container| array_of(container, key)) {
                return items.iter().filter_map(Json::as_object).map(|item| format!("{}\n", entity::render_markdown(kind, item))).collect();
            }
        }
        let is_list = command.ends_with(" list") || command.ends_with(" tree");
        for key in ["ticket", "goal", "bundle", "folder", "file", "definition", "contributor", "policy", "technology", "draft", "todo", "checkpoint"] {
            if let Some(found) = payload.get(key).and_then(Json::as_object) {
                let mut found = found.clone();
                if let Some(id) = payload.get("id").filter(|id| id.is_string()) {
                    found.entry("id".to_string()).or_insert_with(|| id.clone());
                }
                if key == "file" && !is_list {
                    return format_markdown_file(&found);
                }
                if is_list {
                    return format!("{}\n", entity::render_markdown(key, &found));
                }
                return format!("{}\n", entity::render_markdown_link(key, &found));
            }
        }
        if let Some(section) = payload.get("section").and_then(Json::as_object) {
            return render_section_tree(section, true);
        }
        for (key, value) in payload {
            if let Some(found) = value.as_object() {
                let kind = entity::infer_kind(key);
                if !kind.is_empty() {
                    return format!("{}\n", entity::render_markdown_link(kind, found));
                }
            }
        }
        format!("{}\n", entity::render_markdown_link("root", payload))
    }

    /// 📄️ One file with its sections and definitions as markdown, twin of `formatMarkdownFile`.
    fn format_markdown_file(file: &Map<String, Json>) -> String {
        let mut out = format!("{}\n", entity::render_markdown_link("file", file));
        let owner = match file.get("id").and_then(Json::as_str).filter(|id| !id.is_empty()) {
            Some(id) => id.to_string(),
            None => file.get("path").and_then(Json::as_str).unwrap_or_default().to_string(),
        };
        fn sections(out: &mut String, items: &[Json], indent: &str, owner: &str) {
            for section in items.iter().filter_map(Json::as_object) {
                let mut section = section.clone();
                section.entry("filePath".to_string()).or_insert_with(|| Json::String(owner.to_string()));
                out.push_str(&format!("{indent}{}\n", entity::render_markdown("section", &section)));
                if let Some(children) = section.get("children").and_then(Json::as_array).filter(|children| !children.is_empty()) {
                    sections(out, children, &format!("{indent}  "), owner);
                }
            }
        }
        if let Some(items) = array_of(file, "sections") {
            sections(&mut out, items, "  ", &owner);
        }
        if let Some(definitions) = array_of(file, "definitions") {
            for definition in definitions.iter().filter_map(Json::as_object) {
                let mut definition = definition.clone();
                definition.entry("filePath".to_string()).or_insert_with(|| Json::String(owner.clone()));
                out.push_str(&format!("  {}\n", entity::render_markdown("definition", &definition)));
            }
        }
        out
    }

    //#endregion 🌩️Renderers

    //#region 🎮️CommandFramework

    /// 🚩️ One typed flag value.
    #[derive(Clone, Debug, PartialEq)]
    pub enum FlagValue {
        Text(String),
        Bool(bool),
        Int(i64),
        Texts(Vec<String>),
        Ints(Vec<i64>),
        Duration(i64),
    }

    impl FlagValue {
        /// 🧾️ The projection value: a duration in milliseconds, a list always an array.
        fn projection(&self) -> Json {
            match self {
                FlagValue::Text(text) => Json::String(text.clone()),
                FlagValue::Bool(flag) => Json::Bool(*flag),
                FlagValue::Int(number) => Json::from(*number),
                FlagValue::Texts(items) => Json::Array(items.iter().cloned().map(Json::String).collect()),
                FlagValue::Ints(items) => Json::Array(items.iter().map(|item| Json::from(*item)).collect()),
                FlagValue::Duration(nanoseconds) => Json::from(nanoseconds / 1_000_000),
            }
        }
    }

    /// 🚩️ One declared flag.
    #[derive(Clone, Debug)]
    pub struct Flag {
        pub name: String,
        pub shorthand: String,
        pub usage: String,
        pub default: FlagValue,
    }

    impl Flag {
        /// 🔤️ A string flag.
        pub fn text(name: &str, default: &str, usage: &str) -> Flag {
            Flag { name: name.to_string(), shorthand: String::new(), usage: usage.to_string(), default: FlagValue::Text(default.to_string()) }
        }

        /// ☑️ A boolean flag, false unless passed.
        pub fn boolean(name: &str, usage: &str) -> Flag {
            Flag { name: name.to_string(), shorthand: String::new(), usage: usage.to_string(), default: FlagValue::Bool(false) }
        }

        /// 🔢️ An integer flag.
        pub fn int(name: &str, default: i64, usage: &str) -> Flag {
            Flag { name: name.to_string(), shorthand: String::new(), usage: usage.to_string(), default: FlagValue::Int(default) }
        }

        /// 📜️ A comma-separated string list flag.
        pub fn texts(name: &str, default: &[&str], usage: &str) -> Flag {
            Flag { name: name.to_string(), shorthand: String::new(), usage: usage.to_string(), default: FlagValue::Texts(default.iter().map(|item| (*item).to_string()).collect()) }
        }

        /// 🔢️ A comma-separated integer list flag.
        pub fn ints(name: &str, usage: &str) -> Flag {
            Flag { name: name.to_string(), shorthand: String::new(), usage: usage.to_string(), default: FlagValue::Ints(Vec::new()) }
        }

        /// ⏱️ A duration flag, the default given in nanoseconds.
        pub fn duration(name: &str, default_nanoseconds: i64, usage: &str) -> Flag {
            Flag { name: name.to_string(), shorthand: String::new(), usage: usage.to_string(), default: FlagValue::Duration(default_nanoseconds) }
        }

        /// 🔡️ Adds the one-letter spelling.
        pub fn short(mut self, shorthand: &str) -> Flag {
            self.shorthand = shorthand.to_string();
            self
        }

        /// 🧮️ Parses a raw argument into this flag's type, refusing with the verbatim reference wording.
        fn parse(&self, raw: &str) -> Result<FlagValue, String> {
            match &self.default {
                FlagValue::Text(_) => Ok(FlagValue::Text(raw.to_string())),
                FlagValue::Bool(_) => match raw {
                    "1" | "t" | "T" | "TRUE" | "true" | "True" => Ok(FlagValue::Bool(true)),
                    "0" | "f" | "F" | "FALSE" | "false" | "False" => Ok(FlagValue::Bool(false)),
                    _ => Err(format!("invalid boolean for --{}: {raw}", self.name)),
                },
                FlagValue::Int(_) => go_atoi(raw).map(FlagValue::Int).ok_or_else(|| format!("invalid integer for --{}: {raw}", self.name)),
                FlagValue::Texts(_) => Ok(FlagValue::Texts(if raw.is_empty() { Vec::new() } else { raw.split(',').map(str::to_string).collect() })),
                FlagValue::Ints(_) => {
                    if raw.is_empty() {
                        return Ok(FlagValue::Ints(Vec::new()));
                    }
                    raw.split(',').map(go_atoi).collect::<Option<Vec<i64>>>().map(FlagValue::Ints).ok_or_else(|| format!("invalid integer list for --{}: {raw}", self.name))
                }
                FlagValue::Duration(_) => parse_duration(raw).map(FlagValue::Duration).ok_or_else(|| format!("invalid duration for --{}: {raw}", self.name)),
            }
        }
    }

    /// 🔢️ Go's `strconv.Atoi`: an optional sign and decimal digits, nothing else.
    fn go_atoi(raw: &str) -> Option<i64> {
        let digits = raw.strip_prefix(['+', '-']).unwrap_or(raw);
        if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        raw.parse().ok()
    }

    /// ⏱️ Go's `time.ParseDuration` in nanoseconds: signed decimal numbers each with a unit.
    pub fn parse_duration(raw: &str) -> Option<i64> {
        let (negative, mut rest) = match raw.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, raw.strip_prefix('+').unwrap_or(raw)),
        };
        if rest == "0" {
            return Some(0);
        }
        if rest.is_empty() {
            return None;
        }
        let mut total = 0f64;
        while !rest.is_empty() {
            let number_end = rest.find(|character: char| !character.is_ascii_digit() && character != '.').unwrap_or(rest.len());
            let number = &rest[..number_end];
            if number.is_empty() || number == "." {
                return None;
            }
            let value: f64 = number.parse().ok()?;
            rest = &rest[number_end..];
            let unit_end = rest.find(|character: char| character.is_ascii_digit() || character == '.').unwrap_or(rest.len());
            let scale = match &rest[..unit_end] {
                "ns" => 1f64,
                "us" | "µs" | "μs" => 1e3,
                "ms" => 1e6,
                "s" => 1e9,
                "m" => 60e9,
                "h" => 3600e9,
                _ => return None,
            };
            rest = &rest[unit_end..];
            total += value * scale;
        }
        let nanoseconds = total.round() as i64;
        Some(if negative { -nanoseconds } else { nanoseconds })
    }

    /// 🧮️ How many operands a command accepts.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Arity {
        Any,
        None,
        Exact(usize),
        AtMost(usize),
        Between(usize, usize),
    }

    impl Arity {
        /// ✅️ Checks an operand count, refusing with the verbatim reference wording.
        fn check(self, count: usize) -> Result<(), String> {
            match self {
                Arity::Any => Ok(()),
                Arity::None if count != 0 => Err(format!("accepts 0 arg(s), received {count}")),
                Arity::Exact(wanted) if count != wanted => Err(format!("accepts {wanted} arg(s), received {count}")),
                Arity::AtMost(maximum) if count > maximum => Err(format!("accepts at most {maximum} arg(s), received {count}")),
                Arity::Between(minimum, maximum) if count < minimum || count > maximum => Err(format!("accepts between {minimum} and {maximum} arg(s), received {count}")),
                _ => Ok(()),
            }
        }
    }

    /// ▶️ What running a command does.
    pub type Action = fn(&mut Invocation<'_>) -> Result<(), CliError>;

    /// 🌳️ One declared command: an immutable spec, so a parse never leaks state into the next one.
    #[derive(Clone)]
    pub struct Command {
        pub use_line: String,
        pub short: String,
        pub arity: Arity,
        pub aliases: Vec<String>,
        pub flags: Vec<Flag>,
        pub persistent: Vec<Flag>,
        pub raw: bool,
        pub action: Option<Action>,
        pub children: Vec<Command>,
    }

    impl Command {
        /// 🆕️ A command with its use line and summary.
        pub fn new(use_line: &str, short: &str) -> Command {
            Command { use_line: use_line.to_string(), short: short.to_string(), arity: Arity::Any, aliases: Vec::new(), flags: Vec::new(), persistent: Vec::new(), raw: false, action: None, children: Vec::new() }
        }

        /// 🧮️ Sets the accepted operand count.
        pub fn args(mut self, arity: Arity) -> Command {
            self.arity = arity;
            self
        }

        /// 🚩️ Adds one local flag unless one of that name exists.
        pub fn flag(mut self, flag: Flag) -> Command {
            if !self.flags.iter().any(|existing| existing.name == flag.name) {
                self.flags.push(flag);
            }
            self
        }

        /// 🚩️ Adds many local flags.
        pub fn flags(self, flags: Vec<Flag>) -> Command {
            flags.into_iter().fold(self, Command::flag)
        }

        /// 🌐️ Adds one flag every descendant sees.
        pub fn persistent(mut self, flag: Flag) -> Command {
            self.persistent.push(flag);
            self
        }

        /// 🔀️ Adds an alternative name.
        pub fn alias(mut self, alias: &str) -> Command {
            self.aliases.push(alias.to_string());
            self
        }

        /// 🙈️ Hands every token after the name to the action untouched.
        pub fn raw(mut self) -> Command {
            self.raw = true;
            self
        }

        /// ▶️ Sets the action.
        pub fn run(mut self, action: Action) -> Command {
            self.action = Some(action);
            self
        }

        /// 🌿️ Appends children in declaration order.
        pub fn children(mut self, children: Vec<Command>) -> Command {
            self.children.extend(children);
            self
        }

        /// 🏷️ The name: the first word of the use line.
        pub fn name(&self) -> &str {
            self.use_line.split(' ').next().unwrap_or_default()
        }

        /// 🔎️ The child a token names, by name or alias.
        pub fn child(&self, name: &str) -> Option<&Command> {
            self.children.iter().find(|child| child.name() == name || child.aliases.iter().any(|alias| alias == name))
        }

        /// 📖️ The usage text, twin of `Command.Help`.
        pub fn help(&self) -> String {
            let mut text = format!("{}\n\nUsage:\n  {}", self.short, self.use_line);
            if !self.children.is_empty() {
                text.push_str("\n\nCommands:\n");
                for child in &self.children {
                    text.push_str(&format!("  {:<20} {}\n", child.name(), child.short));
                }
            }
            text
        }
    }

    /// 🧾️ One parsed argv: the selected command chain, the operands, every visible flag with its
    /// effective value and whether the caller passed it, and whether `--help` short-circuited.
    pub struct Parsed<'a> {
        pub chain: Vec<&'a Command>,
        pub positional: Vec<String>,
        pub flags: BTreeMap<String, (FlagValue, bool)>,
        pub help: bool,
    }

    impl<'a> Parsed<'a> {
        /// 🌳️ The selected command.
        pub fn selected(&self) -> &'a Command {
            self.chain.last().copied().expect("a parse always selects the root")
        }
    }

    /// 🔎️ The flag a name resolves to from a command chain: the selected command's own flags first,
    /// then the persistent flags of the chain from the selected command upwards.
    fn lookup_flag<'a>(chain: &[&'a Command], name: &str, short: bool) -> Option<&'a Flag> {
        let selected = chain.last()?;
        let find = |flags: &'a [Flag]| flags.iter().find(|flag| short && !flag.shorthand.is_empty() && flag.shorthand == name).or_else(|| flags.iter().find(|flag| flag.name == name));
        find(&selected.flags).or_else(|| chain.iter().rev().find_map(|command| find(&command.persistent)))
    }

    /// 🧭️ Parses one argv against a command tree, twin of `selectAndParse` plus the arity check.
    pub fn parse<'a>(root: &'a Command, argv: &[String]) -> Result<Parsed<'a>, String> {
        let mut chain: Vec<&Command> = vec![root];
        let mut positional: Vec<String> = Vec::new();
        let mut changed: Vec<(*const Flag, FlagValue)> = Vec::new();
        let mut help = false;
        let mut index = 0;
        while index < argv.len() {
            let arg = &argv[index];
            let selected = *chain.last().expect("the chain holds the root");
            if selected.raw {
                positional.extend(argv[index..].iter().cloned());
                break;
            }
            if arg == "--help" || arg == "-h" {
                help = true;
                positional.clear();
                break;
            }
            if arg == "--" {
                positional.extend(argv[index + 1..].iter().cloned());
                break;
            }
            if arg.starts_with('-') {
                let (mut name, short) = match arg.strip_prefix("--") {
                    Some(rest) => (rest.to_string(), false),
                    None => (arg[1..].to_string(), true),
                };
                let mut inline: Option<String> = None;
                if let Some((key, value)) = name.clone().split_once('=') {
                    name = key.to_string();
                    inline = Some(value.to_string()).filter(|value| !value.is_empty());
                }
                let flag = lookup_flag(&chain, &name, short).ok_or_else(|| format!("unknown flag: {arg}"))?;
                let (raw, consumed) = match (&flag.default, inline) {
                    (FlagValue::Bool(_), None) => ("true".to_string(), 1),
                    (_, Some(value)) => (value, 1),
                    _ => match argv.get(index + 1) {
                        Some(value) => (value.clone(), 2),
                        None => return Err(format!("flag needs an argument: {arg}")),
                    },
                };
                let value = flag.parse(&raw)?;
                let key: *const Flag = flag;
                changed.retain(|(existing, _)| *existing != key);
                changed.push((key, value));
                index += consumed;
                continue;
            }
            if positional.is_empty() {
                if let Some(child) = selected.child(arg) {
                    chain.push(child);
                    index += 1;
                    continue;
                }
            }
            positional.push(arg.clone());
            index += 1;
        }
        let selected = *chain.last().expect("the chain holds the root");
        if !help {
            selected.arity.check(positional.len())?;
        }
        let mut flags: BTreeMap<String, (FlagValue, bool)> = BTreeMap::new();
        let visible = chain.iter().flat_map(|command| command.persistent.iter()).chain(selected.flags.iter());
        for flag in visible {
            let key: *const Flag = flag;
            let entry = match changed.iter().find(|(existing, _)| *existing == key) {
                Some((_, value)) => (value.clone(), true),
                None => (flag.default.clone(), false),
            };
            flags.insert(flag.name.clone(), entry);
        }
        Ok(Parsed { chain, positional, flags, help })
    }

    /// ❌️ Why an invocation stopped: a message for standard error, or a bare exit code the renderer
    /// already explained.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum CliError {
        Message(String),
        Exit(i32),
    }

    impl From<String> for CliError {
        fn from(message: String) -> CliError {
            CliError::Message(message)
        }
    }

    impl From<&str> for CliError {
        fn from(message: &str) -> CliError {
            CliError::Message(message.to_string())
        }
    }

    /// ⚙️ The persistent settings every verb reads.
    #[derive(Clone, Debug)]
    pub struct Config {
        pub format: Format,
        pub verbose: bool,
        pub repo: String,
        pub timeout_nanoseconds: i64,
    }

    /// 🎬️ One running command: the parse, the resolved settings and the two output streams.
    pub struct Invocation<'a> {
        pub parsed: Parsed<'a>,
        pub config: Config,
        pub out: &'a mut dyn Write,
        pub err: &'a mut dyn Write,
        pub is_tty: bool,
    }

    impl<'a> Invocation<'a> {
        /// 🔤️ A string flag, empty when it is not visible.
        pub fn text(&self, name: &str) -> String {
            match self.parsed.flags.get(name) {
                Some((FlagValue::Text(text), _)) => text.clone(),
                _ => String::new(),
            }
        }

        /// ☑️ A boolean flag, false when it is not visible.
        pub fn flag(&self, name: &str) -> bool {
            matches!(self.parsed.flags.get(name), Some((FlagValue::Bool(true), _)))
        }

        /// 🔢️ An integer flag, zero when it is not visible.
        pub fn int(&self, name: &str) -> i64 {
            match self.parsed.flags.get(name) {
                Some((FlagValue::Int(number), _)) => *number,
                _ => 0,
            }
        }

        /// 📜️ A string list flag.
        pub fn texts(&self, name: &str) -> Vec<String> {
            match self.parsed.flags.get(name) {
                Some((FlagValue::Texts(items), _)) => items.clone(),
                _ => Vec::new(),
            }
        }

        /// 🔢️ An integer list flag.
        pub fn ints(&self, name: &str) -> Vec<i64> {
            match self.parsed.flags.get(name) {
                Some((FlagValue::Ints(items), _)) => items.clone(),
                _ => Vec::new(),
            }
        }

        /// 🚩️ Whether the caller passed a flag.
        pub fn changed(&self, name: &str) -> bool {
            self.parsed.flags.get(name).is_some_and(|(_, changed)| *changed)
        }

        /// 📜️ The operands.
        pub fn args(&self) -> &[String] {
            &self.parsed.positional
        }

        /// 🔤️ The operand at an index, empty when absent.
        pub fn arg(&self, index: usize) -> String {
            self.parsed.positional.get(index).cloned().unwrap_or_default()
        }

        /// 🏠️ The repository root the invocation runs against.
        pub fn root(&self) -> std::path::PathBuf {
            if !self.config.repo.is_empty() {
                return semio_framework_repo_workspace::find_repo_root(std::path::Path::new(&self.config.repo));
            }
            semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")))
        }

        /// 🗄️ The production repository context of the invocation's root.
        pub fn context(&self) -> FsRepoContext {
            FsRepoContext::open(&self.root())
        }

        /// ✍️ Writes text to standard output.
        pub fn print(&mut self, text: &str) {
            let _ = self.out.write_all(text.as_bytes());
        }

        /// ✍️ Writes one line to standard output.
        pub fn println(&mut self, text: &str) {
            self.print(&format!("{text}\n"));
        }

        /// 🖨️ Renders a produced stream in the invocation's format, failing with its exit code.
        pub fn render(&mut self, events: &[Event], started: std::time::Instant) -> Result<(), CliError> {
            let rendered = render(events, self.config.format, self.is_tty, self.config.verbose, started.elapsed().as_millis() as i64);
            let _ = self.out.write_all(rendered.out.as_bytes());
            let _ = self.err.write_all(rendered.err.as_bytes());
            if rendered.exit_code != 0 {
                return Err(CliError::Exit(rendered.exit_code));
            }
            Ok(())
        }

        /// 🕸️ Executes one document against the repository and renders the stream.
        pub fn graphql(&mut self, query: &str, variables: Json) -> Result<(), CliError> {
            let started = std::time::Instant::now();
            let variables = match variables {
                Json::Object(members) => members,
                _ => Map::new(),
            };
            let context = self.context();
            let events = graphql_events(&context, query, &variables);
            self.render(&events, started)
        }

        /// 📡️ Renders one collected stream of results under a command name.
        pub fn stream(&mut self, command: &str, results: Vec<Json>, logs: Vec<String>) -> Result<(), CliError> {
            let started = std::time::Instant::now();
            let mut events = vec![Event::start(command)];
            events.extend(results.into_iter().map(|data| Event::result(command, data)));
            events.extend(logs.into_iter().map(|line| Event::log(command, line)));
            events.push(Event::finish(EXIT_OK, "ok"));
            self.render(&events, started)
        }
    }

    //#endregion 🎮️CommandFramework

    //#region 🌳️CommandTree

    /// ⏱️ The wall-clock budget of one invocation when `--timeout` is not given: five minutes.
    pub const DEFAULT_TIMEOUT_NANOSECONDS: i64 = 300_000_000_000;

    /// 🌳️ The whole declared repo command tree, twin of `NewRootWithConfig`.
    pub fn root() -> Command {
        Command::new("repo", "Monorepo CLI for Compose")
            .persistent(Flag::text("format", "md", "Output format: md, text, json"))
            .persistent(Flag::boolean("md", "Shorthand for --format md"))
            .persistent(Flag::boolean("text", "Shorthand for --format text"))
            .persistent(Flag::boolean("json", "Shorthand for --format json"))
            .persistent(Flag::boolean("verbose", "Verbose output"))
            .persistent(Flag::text("repo", "", "Repo root path"))
            .persistent(Flag::duration("timeout", DEFAULT_TIMEOUT_NANOSECONDS, "Timeout for command execution"))
            .children(vec![
                Command::new("mcp [kind]", "Run MCP server (optional kind: client, cursor, copilot, claude, codex, kiro)").args(Arity::AtMost(1)).flag(Flag::boolean("dry-run", "Initialize and exit without starting server")).run(verbs::mcp),
                Command::new("graphql [query]", "Execute a GraphQL query").args(Arity::AtMost(1)).flag(Flag::text("query", "", "GraphQL query")).flag(Flag::text("vars", "", "GraphQL variables JSON").short("v")).run(verbs::graphql),
                Command::new("test [testable-id-or-uri]...", "Run tests for given entities").run(verbs::test),
                ticket_command(),
                todo_command(),
                goal_command(),
                contributor_command(),
                folder_command(),
                file_command(),
                section_command(),
                Command::new("move <source> <target>", "Move an artifact from source to target").args(Arity::Exact(2)).run(verbs::move_artifact),
                Command::new("integrate [source] [target]", "Integrate source code into a target file section")
                    .args(Arity::AtMost(2))
                    .flags(vec![Flag::text("file", "", "Source file path"), Flag::text("target-file", "", "Target file path"), Flag::text("target-section", "", "Target section name"), Flag::text("parent-section", "", "Parent section name")])
                    .run(verbs::integrate),
                Command::new("extract [source] [target]", "Extract a section from a source file into a target file")
                    .args(Arity::AtMost(2))
                    .flags(vec![Flag::text("file", "", "Source file path"), Flag::text("section", "", "Section name"), Flag::text("parent-section", "", "Parent section name"), Flag::text("target-file", "", "Target file path")])
                    .run(verbs::extract),
                Command::new("rename <old> <new> [scope]", "Rename a token across non-gitignored files (all case variants)").args(Arity::Between(2, 3)).run(verbs::rename),
                Command::new("sync", "Synchronize monorepo artifacts").run(verbs::sync).children(vec![
                    Command::new("github", "Synchronize local state with GitHub").run(verbs::sync_management),
                    Command::new("management", "Synchronize local state with management provider").run(verbs::sync_management),
                ]),
                Command::new("search [query]", "Search monorepo tree").args(Arity::AtMost(1)).flags(tree_flags()).run(verbs::search),
                Command::new("list [query]", "Stream a flat list of monorepo items").args(Arity::AtMost(1)).flag(Flag::int("limit", 0, "Maximum number of items to output (0 = unlimited)")).flags(tree_flags()).flag(Flag::boolean("sorted", "Collect all results and sort by ID before output")).run(verbs::list),
                Command::new("query [keywords]", "Keyword search across monorepo resources").run(verbs::query),
                Command::new("tree", "Show a repository tree").children(vec![
                    Command::new("monorepo [query]", "Show the monorepo tree").args(Arity::AtMost(1)).flag(Flag::text("scope", "", "Restrict the projection to this repository-relative path")).flags(tree_flags()).run(verbs::tree_monorepo),
                    Command::new("goal", "Show the goal and ticket tree").args(Arity::None).run(verbs::tree_goal),
                    Command::new("statute", "Show the statute tree").args(Arity::None).run(verbs::tree_statute),
                    Command::new("territory", "Show the territory tree").args(Arity::None).run(verbs::tree_territory),
                ]),
                Command::new("export [output]", "Export repo data to an event log").args(Arity::AtMost(1)).run(verbs::export),
                Command::new("hook <event> <client>", "Run a lifecycle hook (git or agent)")
                    .args(Arity::Between(1, 2))
                    .flags(vec![Flag::text("tool-name", "", "Tool name for tool-related events"), Flag::text("tool-args", "", "Tool arguments for tool-related events"), Flag::text("file", "", "File path for code events"), Flag::text("parent", "", "Parent agent info for agent events")])
                    .run(verbs::hook),
                Command::new("mermaid <visualization>", "Generate mermaid diagram strings").children(vec![
                    Command::new("loc-by-technologies-bundles-folders-files", "LOC treemap grouped by technology, bundle, folder, file").args(Arity::None).run(verbs::mermaid_technologies),
                    Command::new("loc-by-contributors", "LOC treemap grouped by contributor (via git blame)").args(Arity::None).run(verbs::mermaid_contributors),
                    Command::new("loc-by-language", "LOC treemap grouped by programming language").args(Arity::None).run(verbs::mermaid_languages),
                ]),
                Command::new("loc", "Tracked-file LOC (code, markup, data) plus git deltas; internal scan (no cloc)")
                    .args(Arity::None)
                    .flags(vec![
                        Flag::boolean("history", &format!("Per-commit time series; walks --branch (default: {}) without checkout", semio_framework_repo_metrics::DEFAULT_BRANCH)),
                        Flag::boolean("by-contributors", "Break down cumulative line deltas by first author (FindAndUpdateContributor alias)"),
                        Flag::text("by-contributor", "", "Restrict git deltas and history rows to this contributor alias (case-insensitive)"),
                        Flag::text("branch", semio_framework_repo_metrics::DEFAULT_BRANCH, "With --history: git ref to log (default dev branch). Ignored when --history is false"),
                        Flag::texts("languages", &semio_framework_repo_metrics::DEFAULT_CODE_LANGUAGES, "Limits which programming-language buckets are counted; markup/data always included when matched"),
                    ])
                    .run(verbs::loc),
                Command::new("technology", "Manage technologies").run(verbs::technology).children(vec![
                    Command::new("list", "List technologies").flags(stream_flags()).run(verbs::technology_list),
                    Command::new("tree", "Show technology tree Structure").flags(stream_flags()).run(verbs::technology_tree),
                ]),
                Command::new("bundle", "Bundle management commands").children(vec![
                    Command::new("list", "List bundles").flags(stream_flags()).flags(status_flags()).run(verbs::bundle_list),
                    Command::new("tree", "Show bundle tree").flags(stream_flags()).flags(status_flags()).run(verbs::bundle_tree),
                ]),
                Command::new("analyze [scope]", "Analyze codebase for breachs").args(Arity::AtMost(1)).run(verbs::analyze),
                Command::new("entity-emojis", "List all entity-identifying emojis").args(Arity::None).run(verbs::entity_emojis),
                Command::new("configure", "Remove blocking git hooks; repo config files are edited manually").run(verbs::configure),
                Command::new("micro-commit [subcommand] [args...]", "Micro-commit workflow (reset, prepare-commit-msg, stage, diff, prepare)").raw().run(verbs::micro_commit),
                Command::new("benchmark", "Run benchmarks for all ecosystems").flag(Flag::boolean("dry-run", "Initialize and exit without running benchmarks")).run(verbs::benchmark),
                Command::new("update [target]", "Update dependencies (npm, python, rust, go, dotnet)")
                    .flag(Flag::boolean("dry-run", "Show what would be updated without making changes"))
                    .flag(Flag::boolean("apply", "Apply updates (default is dry-run)"))
                    .run(verbs::update),
                Command::new("auth", "Server authentication").children(vec![
                    Command::new("whoami", "Show current authenticated developer").run(verbs::auth_whoami),
                    Command::new("status", "Show server connection status").run(verbs::auth_status),
                ]),
                Command::new("autofix [scope]", "Apply autofixes for breachs").args(Arity::AtMost(1)).run(verbs::autofix),
                Command::new("statute", "Statute management commands").children(vec![
                    Command::new("list", "List statutes").flags(stream_flags()).run(verbs::statute_list),
                    Command::new("tree", "Show statute tree").flags(stream_flags()).run(verbs::statute_tree),
                ]),
                Command::new("checkpoint", "Checkpoint management commands").children(vec![Command::new("list", "List checkpoints").flag(Flag::int("limit", 100, "Maximum number of checkpoints to show")).flags(stream_flags()).run(verbs::checkpoint_list)]),
                Command::new("interaction", "Interaction management commands").children(vec![
                    Command::new("list", "List all interactions").flag(Flag::boolean("sorted", "Sort interactions by date instead of streaming")).flags(stream_flags()).run(verbs::interaction_list),
                    Command::new("tree", "Show interactions within goal/ticket tree").flags(stream_flags()).run(verbs::interaction_tree),
                ]),
                Command::new("draft", "Draft management commands").children(vec![
                    Command::new("create [title]", "Create a new draft").flag(Flag::texts("files", &[], "Files to include in the draft")).run(verbs::draft_create),
                    Command::new("delete [slug]", "Delete a draft").run(verbs::draft_delete),
                    list_command("List drafts", verbs::draft_list),
                ]),
                Command::new("definition", "Definition management commands").children(vec![Command::new("list", "List definitions").alias("tree").args(Arity::AtMost(1)).flag(Flag::text("file", "", "File path")).flags(stream_flags()).run(verbs::definition_list)]),
            ])
    }

    /// 📃️ A `list` subcommand fronting one read document.
    fn list_command(short: &str, action: Action) -> Command {
        Command::new("list", short).args(Arity::None).run(action)
    }

    /// 🔘️ Paired `--only-*` / `--no-*` switches.
    fn only_no(pairs: &[(&str, &str, &str)]) -> Vec<Flag> {
        pairs.iter().flat_map(|(name, only, no)| [Flag::boolean(&format!("only-{name}"), only), Flag::boolean(&format!("no-{name}"), no)]).collect()
    }

    /// 🌳️ The monorepo-tree filter vocabulary, twin of `bindTreeFlags`.
    fn tree_flags() -> Vec<Flag> {
        let mut flags = only_no(&[
            ("technology", "Only show technologies", "Exclude technologies"),
            ("bundle", "Only show bundles", "Exclude bundles"),
            ("folder", "Only show folders", "Exclude folders"),
            ("file", "Only show files", "Exclude files"),
            ("section", "Only show sections", "Exclude sections"),
            ("definition", "Only show definitions", "Exclude definitions"),
            ("goal", "Only show goals", "Exclude goals"),
            ("ticket", "Only show tickets", "Exclude tickets"),
            ("draft", "Only show drafts", "Exclude drafts"),
            ("policy", "Only show policies", "Exclude policies"),
            ("contributor", "Only show contributors", "Exclude contributors"),
            ("checkpoint", "Only show checkpoints", "Exclude checkpoints"),
            ("statute", "Only show statutes", "Exclude statutes"),
            ("todo", "Only show todos", "Exclude todos"),
            ("breach", "Only show breaches", "Exclude breaches"),
            ("library", "Only show library bundles", "Exclude library bundles"),
            ("schema", "Only show schema bundles", "Exclude schema bundles"),
            ("binary", "Only show binary bundles", "Exclude binary bundles"),
            ("client", "Only show client bundles", "Exclude client bundles"),
            ("site", "Only show site bundles", "Exclude site bundles"),
            ("assets", "Only show asset bundles", "Exclude asset bundles"),
            ("organization", "Only show organization folders", "Exclude organization folders"),
            ("required", "Only show required folders", "Exclude required folders"),
            ("code", "Only show code files", "Exclude code files"),
            ("script", "Only show script files", "Exclude script files"),
            ("config", "Only show config files", "Exclude config files"),
            ("lab", "Only show lab files", "Exclude lab files"),
            ("docs", "Only show docs files", "Exclude docs files"),
            ("resource", "Only show resource files", "Exclude resource files"),
            ("template", "Only show template files", "Exclude template files"),
            ("license", "Only show license files", "Exclude license files"),
            ("implementation", "Only show implementation definitions", "Exclude implementation definitions"),
            ("interface", "Only show interface definitions", "Exclude interface definitions"),
            ("constant", "Only show constant definitions", "Exclude constant definitions"),
        ]);
        flags.extend([
            Flag::boolean("only-open", "Only show open items"),
            Flag::boolean("only-closed", "Only show closed items"),
            Flag::boolean("open", "Only show open items"),
            Flag::boolean("closed", "Only show closed items"),
            Flag::ints("only-year", "Only show years"),
            Flag::ints("no-year", "Exclude years"),
            Flag::ints("only-month", "Only show months"),
            Flag::ints("no-month", "Exclude months"),
            Flag::ints("only-day", "Only show days"),
            Flag::ints("no-day", "Exclude days"),
            Flag::texts("only-contributor-name", &[], "Only show specific contributors"),
            Flag::texts("no-contributor-name", &[], "Exclude specific contributors"),
            Flag::texts("only-policy-name", &[], "Only show specific policies"),
            Flag::texts("no-policy-name", &[], "Exclude specific policies"),
            Flag::text("query", "", "Full-text search query"),
        ]);
        flags
    }

    /// 📡️ The stream filter vocabulary, twin of `bindStreamFlags`.
    fn stream_flags() -> Vec<Flag> {
        let mut flags = vec![Flag::boolean("show-ignored", "Show ignored folders and files"), Flag::boolean("show-generated", "Show generated folders and files")];
        flags.extend(only_no(&[
            ("code", "Only show code files", "Exclude code files"),
            ("script", "Only show script files", "Exclude script files"),
            ("config", "Only show config files", "Exclude config files"),
            ("lab", "Only show lab files", "Exclude lab files"),
            ("docs", "Only show docs files", "Exclude docs files"),
            ("resource", "Only show resource files", "Exclude resource files"),
            ("template", "Only show template files", "Exclude template files"),
            ("license", "Only show license files", "Exclude license files"),
            ("library", "Only show library bundles", "Exclude library bundles"),
            ("schema", "Only show schema bundles", "Exclude schema bundles"),
            ("binary", "Only show binary bundles", "Exclude binary bundles"),
            ("client", "Only show client bundles", "Exclude client bundles"),
            ("site", "Only show site bundles", "Exclude site bundles"),
            ("assets", "Only show asset bundles", "Exclude asset bundles"),
            ("organization", "Only show organization folders", "Exclude organization folders"),
            ("required", "Only show required folders", "Exclude required folders"),
            ("implementation", "Only show implementation definitions", "Exclude implementation definitions"),
            ("interface", "Only show interface definitions", "Exclude interface definitions"),
            ("constant", "Only show constant definitions", "Exclude constant definitions"),
        ]));
        flags.extend([
            Flag::ints("no-year", "Exclude years"),
            Flag::ints("only-year", "Only show years"),
            Flag::ints("no-month", "Exclude months"),
            Flag::ints("only-month", "Only show months"),
            Flag::ints("no-day", "Exclude days"),
            Flag::ints("only-day", "Only show days"),
            Flag::texts("no-contributor", &[], "Exclude contributors"),
            Flag::texts("only-contributor", &[], "Only show contributors"),
            Flag::texts("no-policy", &[], "Exclude policies"),
            Flag::texts("only-policy", &[], "Only show policies"),
            Flag::texts("no-breach", &[], "Exclude statutes"),
            Flag::texts("only-breach", &[], "Only show statutes"),
            Flag::text("filter", "", "Filter string"),
            Flag::text("query", "", "Full-text search query"),
            Flag::boolean("regex", "Use regex for filter"),
            Flag::boolean("match-case", "Match case for filter"),
            Flag::boolean("match-whole-word", "Match whole word for filter"),
        ]);
        flags
    }

    /// 🔲️ The status vocabulary, twin of `bindStatusFlags`.
    fn status_flags() -> Vec<Flag> {
        vec![Flag::boolean("open", "Show only open items"), Flag::boolean("closed", "Show only closed items"), Flag::text("status", "", "Filter by status (open or closed)")]
    }

    /// 🤖️ One switch per allowed LLM, effort and client slug, twin of `addLLMFlags` and friends.
    fn vocabulary_flags(llm: bool, effort: bool, client: bool) -> Vec<Flag> {
        let mut flags = Vec::new();
        if llm {
            flags.extend(semio_framework_repo_model::allowed_llms().iter().map(|slug| Flag::boolean(slug, &format!("Use {slug} as LLM"))));
        }
        if effort {
            flags.extend(semio_framework_repo_model::allowed_efforts().iter().map(|slug| Flag::boolean(slug, &format!("Use {slug} as LLM reasoning effort"))));
        }
        if client {
            flags.extend(semio_framework_repo_model::allowed_clients().iter().map(|slug| Flag::boolean(slug, &format!("Use {slug} as Client"))));
        }
        flags
    }

    fn ticket_command() -> Command {
        Command::new("ticket", "Ticket management commands").children(vec![
            Command::new("change <path>", "Change a ticket")
                .args(Arity::Exact(1))
                .flags(vec![
                    Flag::text("parent", "", "New parent ticket slug"),
                    Flag::text("title", "", "New title"),
                    Flag::text("prompt", "", "New prompt"),
                    Flag::text("goal", "", "New goal ID"),
                    Flag::text("effort", "", "LLM reasoning effort (low, medium, high, max)"),
                    Flag::boolean("no-management", "Skip management provider sync"),
                ])
                .flags(vocabulary_flags(true, true, true))
                .run(verbs::ticket_change),
            Command::new("open [emoji] [goal] [title] [prompt] [client] [llm]", "Open a new ticket")
                .flags(vec![
                    Flag::text("emoji", "", "Ticket emoji"),
                    Flag::text("title", "", "Ticket title"),
                    Flag::text("prompt", "", "Ticket prompt"),
                    Flag::text("llm", "", "LLM"),
                    Flag::text("effort", "", "LLM reasoning effort (low, medium, high, max)"),
                    Flag::text("client", "", "Client"),
                    Flag::boolean("no-issue", "Skip management provider issue"),
                    Flag::text("draft", "", "Draft ID"),
                    Flag::text("goal", "", "Goal ID"),
                    Flag::boolean("no-management", "Skip management provider operations"),
                    Flag::text("parent", "", "Parent ticket slug"),
                    Flag::text("issue", "", "Link to existing GitHub issue URL instead of creating new one"),
                ])
                .flags(vocabulary_flags(true, true, true))
                .run(verbs::ticket_open),
            Command::new("close [path] [summary] [files...]", "Close a ticket")
                .flags(vec![
                    Flag::boolean("all", "Close all open tickets"),
                    Flag::int("year", 0, "Ticket year"),
                    Flag::int("month", 0, "Ticket month"),
                    Flag::int("day", 0, "Ticket day"),
                    Flag::text("slug", "", "Ticket slug"),
                    Flag::boolean("no-management", "Skip management provider operations"),
                    Flag::text("summary", "", "Summary"),
                    Flag::texts("files", &[], "Files"),
                    Flag::text("title", "", "Title"),
                ])
                .run(verbs::ticket_close),
            Command::new("reopen [path] [prompt] [client] [llm]", "Reopen a ticket")
                .flags(vec![
                    Flag::boolean("no-management", "Skip management provider operations"),
                    Flag::int("year", 0, "Ticket year"),
                    Flag::int("month", 0, "Ticket month"),
                    Flag::int("day", 0, "Ticket day"),
                    Flag::text("slug", "", "Ticket slug"),
                    Flag::text("prompt", "", "Prompt"),
                    Flag::text("llm", "", "LLM"),
                    Flag::text("effort", "", "LLM reasoning effort (low, medium, high, max)"),
                    Flag::text("client", "", "Client"),
                    Flag::text("title", "", "Title"),
                    Flag::text("draft", "", "Draft ID"),
                    Flag::text("goal", "", "Goal ID"),
                    Flag::text("parent", "", "Parent ticket slug"),
                ])
                .flags(vocabulary_flags(true, true, true))
                .run(verbs::ticket_reopen),
            Command::new("purge-artifacts [path]", "Delete ticket-folder files above 5 MiB and subfolders above 10 MiB").flag(Flag::boolean("all", "Purge oversized artifacts in every ticket folder")).run(verbs::ticket_purge_artifacts),
            Command::new("list", "List tickets")
                .args(Arity::None)
                .flags(vec![Flag::text("status", "", "Filter by status (open or closed)"), Flag::int("year", 0, "Ticket year"), Flag::int("month", 0, "Ticket month"), Flag::int("day", 0, "Ticket day")])
                .run(verbs::ticket_list),
            single_ticket_command("show", "Show one ticket", verbs::ticket_show),
            single_ticket_command("files", "List the files of one ticket", verbs::ticket_files),
        ])
    }

    /// 🎫️ A ticket verb whose operand or `--slug` flag names one ticket.
    fn single_ticket_command(name: &str, short: &str, action: Action) -> Command {
        Command::new(&format!("{name} [path]"), short)
            .args(Arity::AtMost(1))
            .flags(vec![Flag::int("year", 0, "Ticket year"), Flag::int("month", 0, "Ticket month"), Flag::int("day", 0, "Ticket day"), Flag::text("slug", "", "Ticket slug")])
            .run(action)
    }

    fn todo_command() -> Command {
        Command::new("todo", "Todo management commands").children(vec![
            Command::new("create [parent-id] [name] [description]", "Create a todo").args(Arity::AtMost(3)).flags(vec![Flag::text("parent", "", "Parent ID"), Flag::text("name", "", "Todo name"), Flag::text("description", "", "Todo description")]).run(verbs::todo_create),
            Command::new("change [id] --name <new-name> --description <new-description>", "Change a todo")
                .args(Arity::AtMost(1))
                .flags(vec![Flag::text("id", "", "Todo ID"), Flag::text("name", "", "New name"), Flag::text("description", "", "New description")])
                .run(verbs::todo_change),
            Command::new("delete [id]", "Delete a todo").args(Arity::AtMost(1)).flag(Flag::text("id", "", "Todo ID")).run(verbs::todo_delete),
            Command::new("search [search-string]", "Search todos").args(Arity::AtMost(1)).run(verbs::todo_search),
            list_command("List todos", verbs::todo_list),
        ])
    }

    fn goal_command() -> Command {
        Command::new("goal", "Goal management commands").children(vec![
            Command::new("change <SLUG>", "Change a goal")
                .args(Arity::Exact(1))
                .flags(vec![
                    Flag::text("title", "", "New title"),
                    Flag::text("description", "", "New description"),
                    Flag::text("due-date", "", "New due date (YYYY-MM-DD)"),
                    Flag::text("parent", "", "New parent goal ID"),
                    Flag::text("llm", "", "New LLM"),
                    Flag::text("effort", "", "New LLM reasoning effort (low, medium, high, max)"),
                    Flag::boolean("no-management", "Skip management provider sync"),
                ])
                .flags(vocabulary_flags(true, true, false))
                .run(verbs::goal_change),
            Command::new("open [title] [description] [prompt] [client] [llm]", "Open a new goal")
                .flags(vec![
                    Flag::text("title", "", "Goal title"),
                    Flag::text("description", "", "Goal description"),
                    Flag::text("prompt", "", "Goal prompt"),
                    Flag::text("due-date", "", "Goal due date (e.g., 2026-02-15)"),
                    Flag::text("llm", "", "LLM"),
                    Flag::text("effort", "", "LLM reasoning effort (low, medium, high, max)"),
                    Flag::text("client", "", "Client"),
                    Flag::boolean("no-management", "Skip management provider synchronization"),
                    Flag::text("parent", "", "Parent goal ID"),
                    Flag::text("bundle", "", "Bundle name associated with this goal"),
                    Flag::text("milestone", "", "Link to existing GitHub milestone URL instead of creating new one"),
                ])
                .flags(vocabulary_flags(true, true, true))
                .run(verbs::goal_open),
            Command::new("close [id] [summary]", "Close a goal").flag(Flag::boolean("no-management", "Skip management provider synchronization")).run(verbs::goal_close),
            Command::new("reopen [id] [prompt] [client] [llm]", "Reopen a goal")
                .flags(vec![
                    Flag::boolean("no-management", "Skip management provider synchronization"),
                    Flag::text("prompt", "", "Prompt"),
                    Flag::text("title", "", "New title"),
                    Flag::text("description", "", "New description"),
                    Flag::text("due-date", "", "New due date"),
                    Flag::text("parent", "", "New parent goal"),
                    Flag::text("llm", "", "LLM"),
                    Flag::text("effort", "", "LLM reasoning effort (low, medium, high, max)"),
                    Flag::text("client", "", "Client"),
                ])
                .flags(vocabulary_flags(true, true, true))
                .run(verbs::goal_reopen),
            list_command("List goals", verbs::goal_list),
            Command::new("tree", "Show the goal and ticket tree").args(Arity::None).run(verbs::goal_tree),
        ])
    }

    fn contributor_command() -> Command {
        Command::new("contributor", "Contributor management commands").children(vec![
            Command::new("add", "Add a contributor").args(Arity::AtMost(3)).flags(vec![Flag::text("github", "", "GitHub username"), Flag::text("name", "", "Contributor name"), Flag::texts("email", &[], "Contributor emails")]).run(verbs::contributor_add),
            Command::new("remove", "Remove a contributor").args(Arity::AtMost(1)).flag(Flag::text("github", "", "GitHub username")).run(verbs::contributor_remove),
            list_command("List contributors", verbs::contributor_list),
        ])
    }

    fn folder_command() -> Command {
        Command::new("folder", "Folder management commands").children(vec![
            Command::new("create", "Create a folder").args(Arity::AtMost(1)).flag(Flag::text("path", "", "Folder path")).run(verbs::folder_create),
            Command::new("move", "Move a folder").args(Arity::AtMost(2)).flags(vec![Flag::text("source", "", "Source path"), Flag::text("target", "", "Target path")]).run(verbs::folder_move),
            Command::new("delete", "Delete a folder").args(Arity::AtMost(1)).flag(Flag::text("path", "", "Folder path")).run(verbs::folder_delete),
            list_command("List folders", verbs::folder_list),
        ])
    }

    fn file_command() -> Command {
        Command::new("file", "File management commands").children(vec![
            Command::new("create", "Create a file").args(Arity::AtMost(1)).flag(Flag::text("path", "", "File path")).run(verbs::file_create),
            Command::new("move", "Move a file").args(Arity::AtMost(2)).flags(vec![Flag::text("source", "", "Source path"), Flag::text("target", "", "Target path")]).run(verbs::file_move),
            Command::new("delete", "Delete a file").args(Arity::AtMost(1)).flag(Flag::text("path", "", "File path")).run(verbs::file_delete),
            list_command("List files", verbs::file_list),
        ])
    }

    fn section_command() -> Command {
        Command::new("section", "Section management commands").children(vec![
            Command::new("create", "Create a section").args(Arity::AtMost(3)).flags(vec![Flag::text("file", "", "File path"), Flag::text("name", "", "Section name"), Flag::text("parent", "", "Parent section")]).run(verbs::section_create),
            Command::new("move", "Move a section").args(Arity::AtMost(3)).flags(vec![Flag::text("file", "", "File path"), Flag::text("old", "", "Old section name"), Flag::text("new", "", "New section name")]).run(verbs::section_move),
            Command::new("delete", "Delete a section").args(Arity::AtMost(2)).flags(vec![Flag::text("file", "", "File path"), Flag::text("name", "", "Section name")]).run(verbs::section_delete),
            Command::new("integrate", "Integrate source code into a target file section")
                .args(Arity::AtMost(4))
                .flags(vec![Flag::text("source", "", "Source file path"), Flag::text("target-section", "", "Target section name"), Flag::text("target-file", "", "Target file path"), Flag::text("target-parent", "", "Target parent section name")])
                .run(verbs::section_integrate),
            Command::new("extract", "Extract a section from a source file into a target file")
                .args(Arity::AtMost(3))
                .flags(vec![Flag::text("source-file", "", "Source file path"), Flag::text("source-section", "", "Source section name"), Flag::text("target-file", "", "Target file path")])
                .run(verbs::section_extract),
            Command::new("list", "List the sections of a file").args(Arity::AtMost(1)).flag(Flag::text("file", "", "File path")).run(verbs::section_list),
        ])
    }

    //#endregion 🌳️CommandTree

    //#region 🗂️TreeSource

    /// 🧹️ One scope as the record paths spell it: forward slashes, no outer separators, `.` empty.
    pub fn normalize_tree_scope(scope: &str) -> String {
        let scope = scope.trim().replace('\\', "/");
        let scope = scope.trim_matches('/');
        if scope == "." {
            String::new()
        } else {
            scope.to_string()
        }
    }

    /// 🎯️ Whether a record path is the scope itself or lies below it.
    pub fn path_within_scope(path: &str, scope: &str) -> bool {
        scope.is_empty() || path == scope || path.starts_with(&format!("{scope}/"))
    }

    /// 🗂️ The tree source the tree verbs project from: every record read out of the repository
    /// through the repository context, twin of the Go `FsTreeSource`.
    #[derive(Default)]
    pub struct FsTreeSource {
        pub root: String,
        pub include_sections: bool,
        pub technologies: Vec<semio_framework_repo_tree::TechnologyRecord>,
        pub folders: Vec<semio_framework_repo_tree::FolderRecord>,
        pub files: Vec<semio_framework_repo_tree::FileRecord>,
        pub goals: Vec<semio_framework_repo_tree::GoalRecord>,
        pub tickets: Vec<semio_framework_repo_tree::TicketRecord>,
        pub drafts: Vec<semio_framework_repo_tree::DraftRecord>,
        pub policies: Vec<semio_framework_repo_tree::PolicyRecord>,
        pub contributors: Vec<semio_framework_repo_tree::ContributorRecord>,
        pub checkpoints: Vec<semio_framework_repo_tree::CheckpointRecord>,
    }

    impl FsTreeSource {
        /// 🎯️ Reads the records of one repository-relative subtree; an empty scope is the whole
        /// repository and also loads the aggregates that have no place in the codebase.
        pub fn load(context: &dyn RepoContext, include_sections: bool, scope: &str) -> FsTreeSource {
            let scope = normalize_tree_scope(scope);
            let mut source = FsTreeSource { root: context.root_dir(), include_sections, ..FsTreeSource::default() };
            let bundles = context.bundles();
            for technology in context.technologies() {
                let id = model_technology_id(&technology);
                let records: Vec<semio_framework_repo_tree::BundleRecord> = bundles
                    .iter()
                    .filter(|bundle| bundle.technology_name == technology.name && path_within_scope(&bundle.root, &scope))
                    .map(|bundle| {
                        let id = model_bundle_id(bundle);
                        semio_framework_repo_tree::BundleRecord { uri: format!("repo://bundle/{id}"), id, name: bundle.name.clone(), kind: bundle.kind.as_str().to_string(), emoji: bundle.emoji.clone(), root: bundle.root.clone(), source_root: bundle.source_root.clone() }
                    })
                    .collect();
                if !scope.is_empty() && records.is_empty() {
                    continue;
                }
                source.technologies.push(semio_framework_repo_tree::TechnologyRecord { uri: format!("repo://technology/{id}"), id, name: technology.name.clone(), kind: technology.kind.as_str().to_string(), emoji: technology.emoji.clone(), bundles: records });
            }
            for folder in context.folders().into_iter().filter(|folder| path_within_scope(&folder.path, &scope)) {
                source.folders.push(semio_framework_repo_tree::FolderRecord { id: folder.id, path: folder.path, name: folder.name, uri: folder.uri, kind: folder.kind.as_str().to_string(), parent_id: folder.parent_id.unwrap_or_default() });
            }
            for file in context.files().into_iter().filter(|file| path_within_scope(&file.path, &scope)) {
                source.files.push(semio_framework_repo_tree::FileRecord { id: file.id, path: file.path, name: file.name, uri: file.uri, kind: file.kind, parent_id: file.folder_id.unwrap_or_default() });
            }
            if !scope.is_empty() {
                return source;
            }
            for goal in context.goals().unwrap_or_default() {
                source.goals.push(semio_framework_repo_tree::GoalRecord { uri: format!("repo://goal/{}", goal.id), id: goal.id, title: goal.title, status: goal.status.as_str().to_string(), due_date: goal.dates.due, description: goal.description, ..Default::default() });
            }
            for ticket in context.tickets(None, None, None, None).unwrap_or_default() {
                source.tickets.push(semio_framework_repo_tree::TicketRecord {
                    id: format!("{}/{}/{}/{}", ticket.year, ticket.month, ticket.day, ticket.slug),
                    uri: format!("repo://ticket/{}", ticket.slug),
                    slug: ticket.slug,
                    title: ticket.title,
                    description: ticket.description,
                    summary: ticket.summary,
                    goal: ticket.goal,
                    parent: ticket.parent,
                    year: ticket.year,
                    month: ticket.month,
                    day: ticket.day,
                    status: ticket.status.as_str().to_string(),
                    ..Default::default()
                });
            }
            for draft in context.drafts().unwrap_or_default() {
                source.drafts.push(semio_framework_repo_tree::DraftRecord { uri: format!("repo://draft/{}", draft.id), id: draft.id, ..Default::default() });
            }
            for policy in context.policies() {
                source.policies.push(semio_framework_repo_tree::PolicyRecord { id: policy.id, name: policy.name, description: policy.description.unwrap_or_default(), groups: policy.groups.unwrap_or_default() });
            }
            for contributor in context.contributors().unwrap_or_default() {
                source.contributors.push(semio_framework_repo_tree::ContributorRecord {
                    id: contributor.alias.clone(),
                    uri: format!("repo://contributor/{}", contributor.alias),
                    alias: contributor.alias,
                    name: contributor.name,
                    github: contributor.github,
                    email: contributor.emails.first().cloned().unwrap_or_default(),
                    emails: contributor.emails,
                    ..Default::default()
                });
            }
            for checkpoint in context.checkpoints(Some(100)).unwrap_or_default() {
                source.checkpoints.push(semio_framework_repo_tree::CheckpointRecord { id: checkpoint.id, uri: format!("repo://checkpoint/{}", checkpoint.sha), sha: checkpoint.sha, title: checkpoint.title, author_id: checkpoint.author_id.unwrap_or_default(), ..Default::default() });
            }
            source
        }
    }

    /// 📑️ One parsed section as the record the tree module reads.
    fn section_record(section: &semio_framework_repo_model::Section, file_path: &str) -> semio_framework_repo_tree::SectionRecord {
        semio_framework_repo_tree::SectionRecord {
            id: section.id.clone(),
            path: if section.path.is_empty() { format!("{file_path}#{}", section.name) } else { section.path.clone() },
            name: section.name.clone(),
            start_line: section.start_line,
            end_line: section.end_line,
            definitions: section
                .definitions
                .iter()
                .map(|definition| semio_framework_repo_tree::DefinitionRecord { id: definition.id.clone(), name: definition.name.clone(), kind: definition.kind.as_str().to_string(), file_path: file_path.to_string(), start_line: definition.start_line, end_line: definition.end_line, ..Default::default() })
                .collect(),
            children: section.children.iter().map(|child| section_record(child, file_path)).collect(),
            ..Default::default()
        }
    }

    impl semio_framework_repo_tree::TreeSource for FsTreeSource {
        fn technologies(&self) -> Vec<semio_framework_repo_tree::TechnologyRecord> {
            self.technologies.clone()
        }
        fn folders(&self) -> Vec<semio_framework_repo_tree::FolderRecord> {
            self.folders.clone()
        }
        fn files(&self) -> Vec<semio_framework_repo_tree::FileRecord> {
            self.files.clone()
        }
        fn goals(&self) -> Vec<semio_framework_repo_tree::GoalRecord> {
            self.goals.clone()
        }
        fn tickets(&self) -> Vec<semio_framework_repo_tree::TicketRecord> {
            self.tickets.clone()
        }
        fn drafts(&self) -> Vec<semio_framework_repo_tree::DraftRecord> {
            self.drafts.clone()
        }
        fn policies(&self) -> Vec<semio_framework_repo_tree::PolicyRecord> {
            self.policies.clone()
        }
        fn contributors(&self) -> Vec<semio_framework_repo_tree::ContributorRecord> {
            self.contributors.clone()
        }
        fn checkpoints(&self) -> Vec<semio_framework_repo_tree::CheckpointRecord> {
            self.checkpoints.clone()
        }
        fn sessions(&self) -> Vec<semio_framework_repo_tree::SessionRecord> {
            Vec::new()
        }
        fn sections(&self, file_path: &str) -> Vec<semio_framework_repo_tree::SectionRecord> {
            if !self.include_sections {
                return Vec::new();
            }
            let Ok(content) = std::fs::read_to_string(std::path::Path::new(&self.root).join(file_path)) else { return Vec::new() };
            semio_framework_repo_languages::parse_sections(&content, file_path).iter().map(|section| section_record(section, file_path)).collect()
        }
    }

    //#endregion 🗂️TreeSource

    //#region 🪪️ModelIds

    /// 📜️ A technology's artifact id, twin of `Technology.GetID`.
    pub fn model_technology_id(technology: &semio_framework_repo_model::Technology) -> String {
        let emoji = if technology.emoji.is_empty() { technology.kind.as_str().to_string() } else { technology.emoji.clone() };
        emoji_text(&emoji) + &flat(&technology.name)
    }

    /// 🔵️ A bundle's artifact id, twin of `Bundle.GetID`: the technology segment resolves its emoji
    /// from the technologies of the repository the process runs in.
    pub fn model_bundle_id(bundle: &semio_framework_repo_model::Bundle) -> String {
        let emoji = if bundle.emoji.is_empty() {
            match bundle.kind.as_str() {
                "schema" => emoji_text(identity_entity("bundle-schema")),
                "binary" => emoji_text(identity_entity("bundle-binary")),
                "ui" => emoji_text(identity_entity("bundle-ui")),
                "site" => emoji_text(identity_entity("bundle-site")),
                "assets" => emoji_text(identity_entity("bundle-assets")),
                "library" => emoji_text(identity_entity("bundle-library")),
                "example" => emoji_text(identity_entity("bundle-example")),
                "repo" => emoji_text(identity_entity("bundle-repo")),
                _ => String::new(),
            }
        } else {
            emoji_text(&bundle.emoji)
        };
        let (technology_code, bundle_code) = match bundle.name.split_once('/') {
            Some((head, tail)) => (head.to_string(), tail.to_string()),
            None => (bundle.name.clone(), bundle.name.clone()),
        };
        let technology_emoji = process_codebase()
            .technologies()
            .into_iter()
            .find(|technology| technology.name == technology_code)
            .map(|technology| technology.emoji)
            .filter(|emoji| !emoji.is_empty())
            .unwrap_or_else(|| semio_framework_repo_model::derive_technology_kind(&technology_code).as_str().to_string());
        emoji_text(&technology_emoji) + &flat(&technology_code) + &emoji + &flat(&bundle_code)
    }

    /// 🗂️ The codebase of the repository the process runs in, which Go's global registries read.
    fn process_codebase() -> Codebase {
        Codebase::new(semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))))
    }

    /// 💗️ A section's artifact id, twin of `Section.GetID`.
    pub fn model_section_id(codebase: &Codebase, section: &semio_framework_repo_model::Section) -> String {
        if section.file_path.is_empty() {
            let emoji = if section.emoji.is_empty() { identity_entity("section").to_string() } else { section.emoji.clone() };
            return emoji_text(&emoji) + &flat(&section.name);
        }
        let file_id = codebase.build_file_id(&section.file_path);
        if !section.path.is_empty() {
            return semio_framework_repo_model::build_section_id(&file_id, &section.path.replace('#', "/").split('/').map(str::to_string).collect::<Vec<String>>());
        }
        semio_framework_repo_model::build_section_id(&file_id, &[format!("{}{}", section.emoji, section.name)])
    }

    /// 💕️ A definition's artifact id, twin of `Definition.GetID`.
    pub fn model_definition_id(codebase: &Codebase, definition: &semio_framework_repo_model::Definition) -> String {
        if definition.file_path.is_empty() {
            let mut data = Map::new();
            data.insert("kind".to_string(), Json::String(definition.kind.as_str().to_string()));
            return entity::artifact_id("definition", &data);
        }
        let parts: Vec<String> = if definition.section_path.is_empty() { Vec::new() } else { definition.section_path.replace('#', "/").split('/').map(str::to_string).collect() };
        semio_framework_repo_model::build_definition_id(&codebase.build_file_id(&definition.file_path), &parts, &definition.name, definition.kind)
    }

    /// 📦️ Every snapshot entity of a repository in taxonomy order, twin of `repoExportSource`.
    pub fn export_entities(context: &dyn RepoContext) -> Result<Vec<semio_framework_repo_events::ExportEntity>, String> {
        let codebase = process_codebase();
        let payload = |value: Result<semio_framework_repo_events::Payload, semio_framework_repo_events::StoreError>| value.map_err(|error| error.to_string());
        let mut entities = Vec::new();
        for value in context.technologies() {
            entities.push(semio_framework_repo_events::ExportEntity { kind: "technology".to_string(), id: model_technology_id(&value), value: payload(semio_framework_repo_events::Payload::of(&value))? });
        }
        for value in context.bundles() {
            entities.push(semio_framework_repo_events::ExportEntity { kind: "bundle".to_string(), id: model_bundle_id(&value), value: payload(semio_framework_repo_events::Payload::of(&value))? });
        }
        for value in context.folders() {
            entities.push(semio_framework_repo_events::ExportEntity { kind: "folder".to_string(), id: codebase.build_folder_id(&value.path), value: payload(semio_framework_repo_events::Payload::of(&value))? });
        }
        for value in context.files() {
            entities.push(semio_framework_repo_events::ExportEntity { kind: "file".to_string(), id: codebase.build_file_id(&value.path), value: payload(semio_framework_repo_events::Payload::of(&value))? });
        }
        for value in context.sections() {
            entities.push(semio_framework_repo_events::ExportEntity { kind: "section".to_string(), id: model_section_id(&codebase, &value), value: payload(semio_framework_repo_events::Payload::of(&value))? });
        }
        for value in context.definitions() {
            entities.push(semio_framework_repo_events::ExportEntity { kind: "definition".to_string(), id: model_definition_id(&codebase, &value), value: payload(semio_framework_repo_events::Payload::of(&value))? });
        }
        Ok(entities)
    }

    //#endregion 🪪️ModelIds

    //#region 🔎️StreamOptions

    /// 🧹️ The stream filters a list verb applies, twin of `model.StreamOptions`.
    #[derive(Clone, Debug, Default)]
    pub struct StreamOptions {
        pub exclude_kinds: Vec<String>,
        pub include_kinds: Vec<String>,
        pub exclude_bundle_kinds: Vec<String>,
        pub include_bundle_kinds: Vec<String>,
        pub exclude_definition_kinds: Vec<String>,
        pub include_definition_kinds: Vec<String>,
        pub filter: String,
        pub query: String,
        pub match_case: bool,
        pub match_whole_word: bool,
    }

    impl StreamOptions {
        /// 🧹️ Reads the options the stream flags carry, twin of `getStreamOptions`.
        pub fn of(invocation: &Invocation<'_>) -> StreamOptions {
            let pick = |names: &[(&str, &str)], prefix: &str| names.iter().filter(|(flag, _)| invocation.flag(&format!("{prefix}-{flag}"))).map(|(_, kind)| (*kind).to_string()).collect::<Vec<String>>();
            let files = [("code", "code"), ("script", "script"), ("config", "config"), ("lab", "lab"), ("docs", "docs"), ("resource", "resource"), ("template", "template"), ("license", "license")];
            let bundles = [("library", "library"), ("schema", "schema"), ("binary", "binary"), ("client", "ui"), ("site", "site"), ("assets", "assets")];
            let definitions = [("implementation", "implementation"), ("interface", "interface"), ("constant", "constant")];
            StreamOptions {
                exclude_kinds: pick(&files, "no"),
                include_kinds: pick(&files, "only"),
                exclude_bundle_kinds: pick(&bundles, "no"),
                include_bundle_kinds: pick(&bundles, "only"),
                exclude_definition_kinds: pick(&definitions, "no"),
                include_definition_kinds: pick(&definitions, "only"),
                filter: invocation.text("filter"),
                query: invocation.text("query"),
                match_case: invocation.flag("match-case"),
                match_whole_word: invocation.flag("match-whole-word"),
            }
        }

        /// 🟤️ Whether a name passes the literal filter, twin of `MatchesFilter`.
        pub fn matches_filter(&self, name: &str) -> bool {
            if self.filter.is_empty() {
                return true;
            }
            let (target, pattern) = if self.match_case { (name.to_string(), self.filter.clone()) } else { (name.to_lowercase(), self.filter.to_lowercase()) };
            if self.match_whole_word {
                return target.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_')).any(|word| word == pattern);
            }
            target.contains(&pattern)
        }

        /// 🔍️ Whether a text passes the fuzzy query, twin of `MatchesQuery`.
        pub fn matches_query(&self, text: &str) -> bool {
            if self.query.is_empty() || text.to_lowercase().contains(&self.query.to_lowercase()) {
                return true;
            }
            let queries: Vec<semio_framework_repo_search::MatchQuery> = self.query.split_whitespace().map(|term| semio_framework_repo_search::MatchQuery::new(&term.to_lowercase()).with_fuzziness(2)).collect();
            semio_framework_repo_search::score(&text.to_lowercase(), &queries).is_some()
        }

        /// 🔵️ Whether a bundle kind passes the include and exclude lists.
        pub fn includes_bundle_kind(&self, kind: &str) -> bool {
            (self.include_bundle_kinds.is_empty() || self.include_bundle_kinds.iter().any(|wanted| wanted == kind)) && !self.exclude_bundle_kinds.iter().any(|unwanted| unwanted == kind)
        }

        /// 💕️ Whether a definition kind passes the include and exclude lists.
        pub fn includes_definition_kind(&self, kind: &str) -> bool {
            (self.include_definition_kinds.is_empty() || self.include_definition_kinds.iter().any(|wanted| wanted == kind)) && !self.exclude_definition_kinds.iter().any(|unwanted| unwanted == kind)
        }
    }

    /// 🔲️ The status filter of a list verb, twin of `getStatusFilter`.
    fn status_filter(invocation: &Invocation<'_>) -> Option<String> {
        if invocation.flag("open") {
            return Some("open".to_string());
        }
        if invocation.flag("closed") {
            return Some("closed".to_string());
        }
        Some(invocation.text("status")).filter(|status| !status.is_empty())
    }

    /// 🧹️ The monorepo-tree filter the tree flags carry, twin of `buildTreeFilterFromFlags`.
    fn tree_filter(invocation: &Invocation<'_>) -> TreeFilter {
        let kinds = [
            ("technology", TreeNodeKind::Technology),
            ("bundle", TreeNodeKind::Bundle),
            ("folder", TreeNodeKind::Folder),
            ("file", TreeNodeKind::File),
            ("section", TreeNodeKind::Section),
            ("definition", TreeNodeKind::Definition),
            ("goal", TreeNodeKind::Goal),
            ("ticket", TreeNodeKind::Ticket),
            ("draft", TreeNodeKind::Draft),
            ("policy", TreeNodeKind::Policy),
            ("contributor", TreeNodeKind::Contributor),
            ("checkpoint", TreeNodeKind::Checkpoint),
            ("statute", TreeNodeKind::Statute),
            ("todo", TreeNodeKind::Todo),
            ("breach", TreeNodeKind::Breach),
        ];
        let sub_kinds = [
            ("library", TreeNodeKind::Bundle, "library"),
            ("schema", TreeNodeKind::Bundle, "schema"),
            ("binary", TreeNodeKind::Bundle, "binary"),
            ("client", TreeNodeKind::Bundle, "ui"),
            ("site", TreeNodeKind::Bundle, "site"),
            ("assets", TreeNodeKind::Bundle, "assets"),
            ("organization", TreeNodeKind::Folder, "organization"),
            ("code", TreeNodeKind::File, "code"),
            ("script", TreeNodeKind::File, "script"),
            ("config", TreeNodeKind::File, "config"),
            ("lab", TreeNodeKind::File, "lab"),
            ("docs", TreeNodeKind::File, "docs"),
            ("resource", TreeNodeKind::File, "resource"),
            ("template", TreeNodeKind::File, "template"),
            ("license", TreeNodeKind::File, "license"),
            ("implementation", TreeNodeKind::Definition, "implementation"),
            ("interface", TreeNodeKind::Definition, "interface"),
            ("constant", TreeNodeKind::Definition, "constant"),
        ];
        let mut only_kinds = BTreeMap::new();
        let mut exclude_kinds = BTreeMap::new();
        for (name, kind) in kinds {
            if invocation.flag(&format!("only-{name}")) {
                only_kinds.insert(kind, true);
            }
            if invocation.flag(&format!("no-{name}")) {
                exclude_kinds.insert(kind, true);
            }
        }
        let mut only_sub_kinds: BTreeMap<TreeNodeKind, Vec<String>> = BTreeMap::new();
        let mut exclude_sub_kinds: BTreeMap<TreeNodeKind, Vec<String>> = BTreeMap::new();
        for (name, kind, value) in sub_kinds {
            if invocation.flag(&format!("only-{name}")) {
                only_sub_kinds.entry(kind).or_default().push(value.to_string());
            }
            if invocation.flag(&format!("no-{name}")) {
                exclude_sub_kinds.entry(kind).or_default().push(value.to_string());
            }
        }
        let mut status = String::new();
        if invocation.flag("only-open") || invocation.flag("open") {
            status = "open".to_string();
        }
        if invocation.flag("only-closed") || invocation.flag("closed") {
            status = "closed".to_string();
        }
        let ints = |name: &str| Some(invocation.ints(name)).filter(|items| !items.is_empty());
        let texts = |name: &str| Some(invocation.texts(name)).filter(|items| !items.is_empty());
        TreeFilter {
            query: invocation.text("query"),
            only_kinds: Some(only_kinds),
            exclude_kinds: Some(exclude_kinds),
            only_sub_kinds: Some(only_sub_kinds),
            exclude_sub_kinds: Some(exclude_sub_kinds),
            only_years: ints("only-year"),
            exclude_years: ints("no-year"),
            only_months: ints("only-month"),
            exclude_months: ints("no-month"),
            only_days: ints("only-day"),
            exclude_days: ints("no-day"),
            only_status: status,
            only_contributors: texts("only-contributor-name"),
            exclude_contributors: texts("no-contributor-name"),
            only_policies: texts("only-policy-name"),
            exclude_policies: texts("no-policy-name"),
        }
    }

    /// 🌳️ Every non-category node of a tree, depth first.
    fn flatten_tree(node: &TreeNode, out: &mut Vec<TreeNode>) {
        if node.kind != TreeNodeKind::Category {
            out.push(node.clone());
        }
        for child in node.children.as_deref().unwrap_or_default() {
            flatten_tree(child, out);
        }
    }

    /// 🌳️ The monorepo tree a search or list verb answers from.
    fn searched_tree(invocation: &Invocation<'_>, positional: usize) -> (TreeNode, TreeFilter) {
        let mut filter = tree_filter(invocation);
        if invocation.args().len() > positional {
            filter.query = invocation.arg(positional);
        }
        let include_sections = filter.only_kinds.as_ref().is_some_and(|kinds| kinds.contains_key(&TreeNodeKind::Section) || kinds.contains_key(&TreeNodeKind::Definition));
        let context = invocation.context();
        let source = FsTreeSource::load(&context, include_sections, "");
        let mut node = semio_framework_repo_tree::build_monorepo_tree(&source, semio_framework_repo_tree::TreeBuildOptions { include_sections, ..Default::default() });
        if !filter.query.is_empty() {
            node = semio_framework_repo_tree::search_tree_in_memory(&node, &filter.query);
        }
        (semio_framework_repo_tree::filter_monorepo_tree(&node, Some(&filter)), filter)
    }

    /// 🖨️ One tree node in the invocation's format, exactly as `search` renders one.
    fn write_tree_node(invocation: &mut Invocation<'_>, node: &TreeNode) -> Result<(), CliError> {
        let text = match invocation.config.format {
            Format::Json => format!("{}\n", serde_json::to_string(node).map_err(|error| error.to_string())?),
            Format::Markdown => semio_framework_repo_tree::render_monorepo_tree_markdown(node, &semio_framework_repo_tree::DefaultEntityRenderer),
            Format::Text => semio_framework_repo_tree::render_monorepo_tree(node, &semio_framework_repo_tree::DefaultEntityRenderer),
        };
        invocation.print(&text);
        Ok(())
    }

    /// 🌲️ A forest under the anonymous category root the monorepo tree carries.
    fn write_tree_forest(invocation: &mut Invocation<'_>, roots: Vec<TreeNode>) -> Result<(), CliError> {
        let mut node = semio_framework_repo_tree::tree_node(TreeNodeKind::Category, "", ".", "");
        node.children = Some(roots);
        write_tree_node(invocation, &node)
    }

    /// 🌿️ A box-drawn text tree as log lines.
    fn text_tree_lines(nodes: &[TreeNode], prefix: &str, lines: &mut Vec<String>) {
        let last = nodes.len().saturating_sub(1);
        for (index, node) in nodes.iter().enumerate() {
            let (connector, child_prefix) = if index == last { ("└️─️─️ ", format!("{prefix}    ")) } else { ("├️─️─️ ", format!("{prefix}│️   ")) };
            let label = if node.description.is_empty() { node.label.clone() } else { format!("{} - {}", node.label, node.description) };
            lines.push(format!("{prefix}{connector}{label}"));
            text_tree_lines(node.children.as_deref().unwrap_or_default(), &child_prefix, lines);
        }
    }

    //#endregion 🔎️StreamOptions

    //#region 🚦️Verbs

    /// 🚦️ One handler per verb, each a thin front for its domain owner.
    pub mod verbs {
        use super::*;

        /// 🎫️ A `YY/MM/DD/SLUG` operand split into its date and slug, zeros when it is no path.
        fn ticket_path(raw: &str) -> Option<(i64, i64, i64, String)> {
            let parts: Vec<&str> = raw.split('/').collect();
            if parts.len() < 4 {
                return None;
            }
            Some((parts[0].parse().unwrap_or_default(), parts[1].parse().unwrap_or_default(), parts[2].parse().unwrap_or_default(), parts[3..].join("/")))
        }

        /// 🔤️ The first of a flag and an operand that is present.
        fn flag_or_arg(invocation: &Invocation<'_>, flag: &str, index: usize) -> String {
            let value = invocation.text(flag);
            if value.is_empty() {
                invocation.arg(index)
            } else {
                value
            }
        }

        /// 🧲️ A vocabulary value from its flag, its switches or the first operand that contains a slug,
        /// twin of `extractLLMFromArgs` and friends.
        fn extract_vocabulary(invocation: &Invocation<'_>, flag: &str, allowed: &[String], normalize: fn(&str) -> String, args: Vec<String>) -> (String, Vec<String>) {
            let value = invocation.text(flag);
            if !value.is_empty() {
                return (value, args);
            }
            if let Some(found) = allowed.iter().find(|slug| invocation.flag(slug)) {
                return (found.clone(), args);
            }
            let mut remaining = Vec::new();
            let mut found = String::new();
            for arg in args {
                if !found.is_empty() {
                    remaining.push(arg);
                    continue;
                }
                let normalized = normalize(&arg);
                let best = allowed.iter().filter(|slug| normalized.contains(&normalize(slug))).max_by_key(|slug| slug.len()).cloned();
                match best {
                    Some(slug) => found = slug,
                    None => remaining.push(arg),
                }
            }
            (found, remaining)
        }

        fn extract_client(invocation: &Invocation<'_>, args: Vec<String>) -> (String, Vec<String>) {
            extract_vocabulary(invocation, "client", semio_framework_repo_model::allowed_clients(), semio_framework_repo_model::normalize_client_slug, args)
        }

        fn extract_llm(invocation: &Invocation<'_>, args: Vec<String>) -> (String, Vec<String>) {
            extract_vocabulary(invocation, "llm", semio_framework_repo_model::allowed_llms(), semio_framework_repo_model::normalize_llm_slug, args)
        }

        fn extract_effort(invocation: &Invocation<'_>, args: Vec<String>) -> (String, Vec<String>) {
            extract_vocabulary(invocation, "effort", semio_framework_repo_model::allowed_efforts(), semio_framework_repo_model::normalize_effort_slug, args)
        }

        /// 📣️ The wire spelling of a client enum value.
        fn client_enum(client: &str) -> String {
            client.replace('-', "_").to_uppercase()
        }

        fn missing_client() -> CliError {
            CliError::Message(format!("missing client. Use --client <value>, --<client-name> flag, or positional arg. Allowed: {}", semio_framework_repo_model::allowed_clients().join(", ")))
        }

        fn missing_llm() -> CliError {
            CliError::Message(format!("missing llm. Use --llm <value>, --<llm-name> flag, or positional arg. Allowed: {}", semio_framework_repo_model::allowed_llms().join(", ")))
        }

        /// 🧾️ Prints the lines a move plan reported.
        fn print_lines(invocation: &mut Invocation<'_>, lines: Vec<String>) {
            for line in lines {
                invocation.println(&line);
            }
        }

        /// 🧭️ The section name a section slug resolves to in a file, twin of `ResolveSectionName`.
        pub fn resolve_section_name(root: &std::path::Path, file_path: &str, slug: &str) -> String {
            let fallback = semio_framework_repo_workspace_unslugify(slug);
            let Ok(content) = std::fs::read_to_string(root.join(file_path)) else { return fallback };
            fn find(sections: &[semio_framework_repo_model::Section], slug: &str) -> Option<String> {
                for section in sections {
                    if flat(&section.name) == flat(slug) || semio_framework_repo_identity::slugify(&section.name).eq_ignore_ascii_case(slug) {
                        return Some(section.name.clone());
                    }
                    if let Some(found) = find(&section.children, slug) {
                        return Some(found);
                    }
                }
                None
            }
            find(&semio_framework_repo_languages::parse_sections(&content, file_path), slug).unwrap_or(fallback)
        }

        /// 🔤️ A slug read back as words, twin of `UnSlugify`.
        fn semio_framework_repo_workspace_unslugify(slug: &str) -> String {
            slug.split(['-', '_']).filter(|word| !word.is_empty()).map(|word| {
                let mut characters = word.chars();
                match characters.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + &characters.as_str().to_lowercase(),
                    None => String::new(),
                }
            }).collect::<Vec<String>>().join(" ")
        }

        pub fn mcp(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            if invocation.flag("dry-run") {
                return Ok(());
            }
            let raw = if invocation.args().is_empty() { std::env::var(semio_framework_repo_mcp::PROFILE_ENVIRONMENT).unwrap_or_default() } else { invocation.arg(0) };
            let profile = semio_framework_repo_mcp::Profile::parse(&raw).map_err(|error| CliError::Message(error.to_string()))?;
            semio_framework_repo_mcp::serve_stdio(crate::mcp_verb::RepoRepository::open(&invocation.root(), profile), profile).map_err(|error| CliError::Message(error.to_string()))
        }

        pub fn graphql(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut query = flag_or_arg(invocation, "query", 0);
            if query.is_empty() {
                return Err("missing query".into());
            }
            let mut variables = Map::new();
            let raw = invocation.text("vars");
            if !raw.is_empty() {
                variables = serde_json::from_str::<Map<String, Json>>(&raw).map_err(|error| CliError::Message(format!("invalid variables JSON: {error}")))?;
            }
            if let Ok(Json::Object(payload)) = serde_json::from_str::<Json>(&query) {
                if let Some(inner) = payload.get("query").and_then(Json::as_str).filter(|inner| !inner.is_empty()) {
                    query = inner.to_string();
                    if let Some(Json::Object(extra)) = payload.get("variables") {
                        for (key, value) in extra {
                            variables.entry(key.clone()).or_insert_with(|| value.clone());
                        }
                    }
                }
            }
            invocation.graphql(&query, Json::Object(variables))
        }

        pub fn test(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let root = invocation.root();
            let bundles = Codebase::new(&root).bundles().iter().map(semio_framework_repo_test_runner::SnapshotBundle::from).collect();
            let mut snapshot = semio_framework_repo_test_runner::FilesystemSnapshot::for_bundles(&root, bundles);
            let identity = semio_framework_repo_test_runner::PendingIdentity;
            let scopes = semio_framework_repo_test_runner::resolve_test_scopes(invocation.args(), &snapshot, &identity, &identity);
            let mut first: Option<CliError> = None;
            for scope in &scopes {
                if !scope.file_path.is_empty() {
                    snapshot.hydrate(&scope.file_path);
                }
                let plan = semio_framework_repo_test_runner::plan_scope(&snapshot, scope);
                for problem in plan.problems {
                    first.get_or_insert(CliError::Message(problem));
                }
                for planned in &plan.invocations {
                    invocation.print(&semio_framework_repo_test_runner::running_line(planned));
                    match semio_framework_repo_test_runner::run_invocation_inherited(planned) {
                        Ok(0) => {}
                        Ok(code) => {
                            first.get_or_insert(CliError::Exit(code));
                        }
                        Err(error) => {
                            first.get_or_insert(CliError::Message(error));
                        }
                    }
                }
            }
            first.map_or(Ok(()), Err)
        }

        pub fn ticket_open(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut remaining: Vec<String> = invocation.args().to_vec();
            let mut take = |flag: &str| {
                let value = invocation.text(flag);
                if value.is_empty() && !remaining.is_empty() {
                    return remaining.remove(0);
                }
                value
            };
            let emoji = take("emoji");
            let goal = take("goal");
            let title = take("title");
            let mut prompt = take("prompt");
            let (client, remaining) = extract_client(invocation, remaining);
            let (llm, remaining) = extract_llm(invocation, remaining);
            let (effort, _) = extract_effort(invocation, remaining);
            if emoji.is_empty() {
                return Err("missing emoji".into());
            }
            if title.is_empty() {
                return Err("missing title".into());
            }
            if prompt.is_empty() {
                prompt = title.clone();
            }
            if client.is_empty() {
                return Err(missing_client());
            }
            if goal.is_empty() {
                return Err("missing goal. Use --goal <goal-id>".into());
            }
            let mut input = json!({ "emoji": emoji, "title": title, "prompt": prompt, "client": client_enum(&client), "noIssue": invocation.flag("no-issue"), "noManagement": invocation.flag("no-management"), "goal": goal });
            for (key, value) in [("llm", llm), ("effort", effort), ("draft", invocation.text("draft")), ("parent", invocation.text("parent")), ("issue", invocation.text("issue"))] {
                if !value.is_empty() {
                    input[key] = Json::String(value);
                }
            }
            invocation.graphql("mutation TicketOpen($input: TicketOpenInput!) { ticketOpen(input: $input) { id slug year month day status path uri } }", json!({ "input": input }))
        }

        pub fn ticket_close(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let (mut year, mut month, mut day, mut slug) = (invocation.int("year"), invocation.int("month"), invocation.int("day"), invocation.text("slug"));
            let mut summary = invocation.text("summary");
            let mut files = invocation.texts("files");
            let close_all = invocation.flag("all");
            if !close_all {
                if !invocation.args().is_empty() && (year == 0 || month == 0 || day == 0 || slug.is_empty()) {
                    if let Some(parsed) = ticket_path(&invocation.arg(0)) {
                        (year, month, day, slug) = parsed;
                    }
                }
                if summary.is_empty() && invocation.args().len() > 1 {
                    summary = invocation.arg(1);
                }
                if files.is_empty() && invocation.args().len() > 2 {
                    files = invocation.args()[2..].to_vec();
                }
                if year == 0 || month == 0 || day == 0 || slug.is_empty() {
                    return Err("missing ticket path (use YYYY/MM/DD/SLUG or flags)".into());
                }
                if summary.is_empty() {
                    return Err("missing summary".into());
                }
                if files.is_empty() {
                    return Err("missing files".into());
                }
            }
            let mut input = json!({ "noManagement": invocation.flag("no-management"), "all": close_all });
            if !close_all {
                input["year"] = json!(year);
                input["month"] = json!(month);
                input["day"] = json!(day);
                input["slug"] = json!(slug);
                input["summary"] = json!(summary);
                input["files"] = json!(files);
            }
            let title = invocation.text("title");
            if !title.is_empty() {
                input["title"] = json!(title);
            }
            invocation.graphql("mutation TicketClose($input: TicketCloseInput!) { ticketClose(input: $input) { id slug status dates { started finished } } }", json!({ "input": input }))
        }

        pub fn ticket_reopen(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let (mut year, mut month, mut day, mut slug) = (invocation.int("year"), invocation.int("month"), invocation.int("day"), invocation.text("slug"));
            let mut remaining: Vec<String> = invocation.args().to_vec();
            if !remaining.is_empty() && (year == 0 || month == 0 || day == 0 || slug.is_empty()) {
                if let Some(parsed) = ticket_path(&remaining[0]) {
                    (year, month, day, slug) = parsed;
                    remaining.remove(0);
                }
            }
            let mut prompt = invocation.text("prompt");
            if prompt.is_empty() && !remaining.is_empty() {
                prompt = remaining.remove(0);
            }
            let (client, remaining) = extract_client(invocation, remaining);
            let (llm, remaining) = extract_llm(invocation, remaining);
            let (effort, _) = extract_effort(invocation, remaining);
            if year == 0 || month == 0 || day == 0 || slug.is_empty() {
                return Err("missing ticket path".into());
            }
            if prompt.is_empty() {
                return Err("missing prompt".into());
            }
            if client.is_empty() {
                return Err(missing_client());
            }
            let mut input = json!({ "year": year, "month": month, "day": day, "slug": slug, "prompt": prompt, "client": client_enum(&client), "noManagement": invocation.flag("no-management") });
            for (key, value) in [("llm", llm), ("effort", effort), ("title", invocation.text("title")), ("draft", invocation.text("draft")), ("goal", invocation.text("goal")), ("parent", invocation.text("parent"))] {
                if !value.is_empty() {
                    input[key] = Json::String(value);
                }
            }
            invocation.graphql("mutation TicketReopen($input: TicketReopenInput!) { ticketReopen(input: $input) { id slug status } }", json!({ "input": input }))
        }

        pub fn ticket_change(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let path = invocation.arg(0);
            let parts: Vec<&str> = path.split('/').collect();
            if parts.len() < 4 {
                return Err("invalid ticket path format: expected YYYY/MM/DD/SLUG".into());
            }
            let year: i64 = parts[0].parse().map_err(|_| CliError::Message("invalid year".to_string()))?;
            let month: i64 = parts[1].parse().map_err(|_| CliError::Message("invalid month".to_string()))?;
            let day: i64 = parts[2].parse().map_err(|_| CliError::Message("invalid day".to_string()))?;
            let (client, _) = extract_client(invocation, Vec::new());
            let (llm, _) = extract_llm(invocation, Vec::new());
            let (effort, _) = extract_effort(invocation, Vec::new());
            let mut input = json!({ "year": year, "month": month, "day": day, "slug": parts[3..].join("/"), "noManagement": invocation.flag("no-management") });
            for key in ["title", "prompt", "goal", "parent"] {
                if invocation.changed(key) {
                    input[key] = json!(invocation.text(key));
                }
            }
            if !client.is_empty() {
                input["client"] = json!(client_enum(&client));
            }
            if !llm.is_empty() {
                input["llm"] = json!(llm);
            }
            if !effort.is_empty() {
                input["effort"] = json!(effort);
            }
            invocation.graphql("mutation TicketChange($input: TicketChangeInput!) { ticketChange(input: $input) { id slug status parent } }", json!({ "input": input }))
        }

        pub fn ticket_purge_artifacts(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let context = invocation.context();
            if invocation.flag("all") || invocation.args().is_empty() {
                let tickets_dir = semio_framework_repo_workspace::tickets_dir_for_root(context.root());
                let mut purged = 0;
                for folder in semio_framework_repo_tickets::ticket_folder_roots(&tickets_dir) {
                    semio_framework_repo_tickets::purge_oversized_artifacts(&folder).map_err(CliError::Message)?;
                    purged += 1;
                }
                invocation.println(&format!("Purged oversized artifacts in {purged} ticket folders"));
                return Ok(());
            }
            let id = semio_framework_repo_tickets::TicketId::parse(&invocation.arg(0)).map_err(|error| CliError::Message(error.message))?;
            let ticket = context.with_tickets(semio_framework_repo_providers::McpClientKind::Generic, |service| service.read(&id)).map_err(|error| CliError::Message(error.message))?;
            semio_framework_repo_tickets::purge_oversized_artifacts(std::path::Path::new(&ticket.folder_path)).map_err(CliError::Message)?;
            invocation.println(&format!("Purged oversized artifacts in {}", ticket.folder_path));
            Ok(())
        }

        pub fn ticket_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut variables = Map::new();
            for name in ["year", "month", "day"] {
                let value = invocation.int(name);
                if value > 0 {
                    variables.insert(name.to_string(), json!(value));
                }
            }
            let status = invocation.text("status");
            if !status.is_empty() {
                variables.insert("status".to_string(), json!(status.to_uppercase()));
            }
            invocation.graphql("query Tickets { repo { tickets { id slug title status prompt } } }", Json::Object(variables))
        }

        fn single_ticket(invocation: &mut Invocation<'_>, query: &str) -> Result<(), CliError> {
            let (mut year, mut month, mut day, mut slug) = (invocation.int("year"), invocation.int("month"), invocation.int("day"), invocation.text("slug"));
            if !invocation.args().is_empty() {
                let operand = invocation.arg(0);
                let parts: Vec<&str> = operand.split('/').collect();
                if parts.len() >= 4 {
                    let tail = parts.len() - 4;
                    year = parts[tail].parse().unwrap_or_default();
                    month = parts[tail + 1].parse().unwrap_or_default();
                    day = parts[tail + 2].parse().unwrap_or_default();
                    slug = parts[tail + 3].to_string();
                }
            }
            if slug.is_empty() {
                return Err("missing ticket path or --slug".into());
            }
            invocation.graphql(query, json!({ "year": year, "month": month, "day": day, "slug": slug }))
        }

        pub fn ticket_show(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            single_ticket(invocation, "query Ticket($year: Int!, $month: Int!, $day: Int!, $slug: String!) { ticket(year: $year, month: $month, day: $day, slug: $slug) { id slug year month day title status goal parent prompt summary path uri } }")
        }

        pub fn ticket_files(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            single_ticket(invocation, "query TicketFiles($year: Int!, $month: Int!, $day: Int!, $slug: String!) { ticket(year: $year, month: $month, day: $day, slug: $slug) { id files { id path name kind } } }")
        }

        pub fn todo_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let parent = flag_or_arg(invocation, "parent", 0);
            let name = flag_or_arg(invocation, "name", 1);
            let description = if invocation.args().len() > 2 { invocation.arg(2) } else { invocation.text("description") };
            if parent.is_empty() || name.is_empty() {
                return Err("missing parent-id or name".into());
            }
            invocation.graphql("mutation TodoCreate($input: TodoCreateInput!) { todoCreate(input: $input) { id name description } }", json!({ "input": { "parentId": parent, "name": name, "description": description } }))
        }

        pub fn todo_change(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let id = flag_or_arg(invocation, "id", 0);
            if id.is_empty() {
                return Err("missing id".into());
            }
            let mut input = json!({ "id": id });
            for key in ["name", "description"] {
                if invocation.changed(key) {
                    input[key] = json!(invocation.text(key));
                }
            }
            invocation.graphql("mutation TodoChange($input: TodoUpdateInput!) { todoChange(input: $input) { id name description } }", json!({ "input": input }))
        }

        pub fn todo_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let id = flag_or_arg(invocation, "id", 0);
            if id.is_empty() {
                return Err("missing id".into());
            }
            invocation.graphql("mutation TodoDelete($id: ID!) { todoDelete(id: $id) }", json!({ "id": id }))
        }

        pub fn todo_search(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let search = invocation.arg(0);
            invocation.graphql("query Todos($filter: FilterInput) { todos(filter: $filter) { id name description } }", json!({ "filter": { "filter": search } }))
        }

        pub fn todo_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            invocation.graphql("query Todos($filter: FilterInput) { todos(filter: $filter) { id name description } }", json!({}))
        }

        pub fn goal_change(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let id = invocation.arg(0);
            let (llm, _) = extract_llm(invocation, Vec::new());
            let (effort, _) = extract_effort(invocation, Vec::new());
            let mut input = json!({ "id": id, "noManagement": invocation.flag("no-management") });
            for (flag, key) in [("title", "title"), ("description", "description"), ("due-date", "dueDate"), ("parent", "parent")] {
                if invocation.changed(flag) {
                    input[key] = json!(invocation.text(flag));
                }
            }
            if !llm.is_empty() {
                input["llm"] = json!(llm);
            }
            if !effort.is_empty() {
                input["effort"] = json!(effort);
            }
            invocation.graphql("mutation GoalChange($id: ID!, $input: GoalChangeInput!) { goalChange(id: $id, input: $input) { id title description dueDate parent } }", json!({ "id": id, "input": input }))
        }

        pub fn goal_open(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut remaining: Vec<String> = invocation.args().to_vec();
            let mut take = |flag: &str| {
                let value = invocation.text(flag);
                if value.is_empty() && !remaining.is_empty() {
                    return remaining.remove(0);
                }
                value
            };
            let title = take("title");
            let description = take("description");
            let prompt = take("prompt");
            let (client, remaining) = extract_client(invocation, remaining);
            let (llm, remaining) = extract_llm(invocation, remaining);
            let (effort, _) = extract_effort(invocation, remaining);
            let due_date = invocation.text("due-date");
            for (value, message) in [(&title, "missing title"), (&description, "missing description"), (&prompt, "missing prompt"), (&due_date, "missing due-date")] {
                if value.is_empty() {
                    return Err(message.into());
                }
            }
            if client.is_empty() {
                return Err(missing_client());
            }
            if llm.is_empty() {
                return Err(missing_llm());
            }
            let mut input = json!({ "title": title, "description": description, "prompt": prompt, "dueDate": due_date, "llm": llm, "client": client, "noManagement": invocation.flag("no-management") });
            for (key, value) in [("effort", effort), ("parent", invocation.text("parent")), ("bundle", invocation.text("bundle")), ("milestone", invocation.text("milestone"))] {
                if !value.is_empty() {
                    input[key] = Json::String(value);
                }
            }
            invocation.graphql("mutation GoalCreate($input: GoalCreateInput!) { goalCreate(input: $input) { id title status prompt dueDate client llm } }", json!({ "input": input }))
        }

        pub fn goal_close(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let id = invocation.arg(0);
            let summary = invocation.arg(1);
            if id.is_empty() {
                return Err("missing goal id".into());
            }
            if summary.is_empty() {
                return Err("missing summary".into());
            }
            invocation.graphql("mutation GoalClose($input: GoalCloseInput!) { goalClose(input: $input) { id status } }", json!({ "input": { "id": id, "summary": summary, "noManagement": invocation.flag("no-management") } }))
        }

        pub fn goal_reopen(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let id = invocation.arg(0);
            let mut remaining: Vec<String> = invocation.args().iter().skip(1).cloned().collect();
            let mut prompt = invocation.text("prompt");
            if prompt.is_empty() && !remaining.is_empty() {
                prompt = remaining.remove(0);
            }
            let (client, remaining) = extract_client(invocation, remaining);
            let (llm, remaining) = extract_llm(invocation, remaining);
            let (effort, _) = extract_effort(invocation, remaining);
            if id.is_empty() {
                return Err("missing goal id".into());
            }
            if prompt.is_empty() {
                return Err("missing prompt".into());
            }
            if client.is_empty() {
                return Err(missing_client());
            }
            if llm.is_empty() {
                return Err(missing_llm());
            }
            let mut input = json!({ "id": id, "prompt": prompt, "client": client, "llm": llm, "noManagement": invocation.flag("no-management") });
            for (key, value) in [("effort", effort), ("title", invocation.text("title")), ("description", invocation.text("description")), ("dueDate", invocation.text("due-date")), ("parent", invocation.text("parent"))] {
                if !value.is_empty() {
                    input[key] = Json::String(value);
                }
            }
            invocation.graphql("mutation GoalReopen($input: GoalReopenInput!) { goalReopen(input: $input) { id status } }", json!({ "input": input }))
        }

        pub fn goal_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            invocation.graphql("query Goals { repo { goals { id title status dueDate description } } }", json!({}))
        }

        pub fn goal_tree(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            invocation.graphql("query GoalTree { repo { goals { id title status dueDate description } tickets { id slug title status goal parent year month day } } }", json!({}))
        }

        pub fn contributor_add(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let github = flag_or_arg(invocation, "github", 0);
            let name = flag_or_arg(invocation, "name", 1);
            let mut emails = invocation.texts("email");
            if emails.is_empty() && invocation.args().len() > 2 {
                emails = invocation.args()[2..].to_vec();
            }
            if github.is_empty() {
                return Err("missing github".into());
            }
            let mut input = json!({ "github": github });
            if !name.is_empty() {
                input["name"] = json!(name);
            }
            if !emails.is_empty() {
                input["emails"] = json!(emails);
            }
            invocation.graphql("mutation ContributorAdd($input: ContributorAddInput!) { contributorAdd(input: $input) { id github name emails } }", json!({ "input": input }))
        }

        pub fn contributor_remove(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let github = flag_or_arg(invocation, "github", 0);
            if github.is_empty() {
                return Err("missing github".into());
            }
            invocation.graphql("mutation ContributorRemove($github: String!) { contributorRemove(github: $github) }", json!({ "github": github }))
        }

        pub fn contributor_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            invocation.graphql("query Contributors { repo { contributors { id emails name } } }", json!({}))
        }

        fn path_mutation(invocation: &mut Invocation<'_>, query: &str) -> Result<(), CliError> {
            let path = flag_or_arg(invocation, "path", 0);
            if path.is_empty() {
                return Err("missing path".into());
            }
            invocation.graphql(query, json!({ "path": path }))
        }

        fn move_mutation(invocation: &mut Invocation<'_>, query: &str) -> Result<(), CliError> {
            let source = flag_or_arg(invocation, "source", 0);
            let target = flag_or_arg(invocation, "target", 1);
            if source.is_empty() || target.is_empty() {
                return Err("missing source or target".into());
            }
            invocation.graphql(query, json!({ "src": source, "dst": target }))
        }

        pub fn folder_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            path_mutation(invocation, "mutation FolderCreate($path: String!) { folderCreate(path: $path) { id path name uri } }")
        }

        pub fn folder_move(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            move_mutation(invocation, "mutation FolderMove($src: String!, $dst: String!) { folderMove(src: $src, dst: $dst) { id path name uri } }")
        }

        pub fn folder_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            path_mutation(invocation, "mutation FolderDelete($path: String!) { folderDelete(path: $path) }")
        }

        pub fn folder_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            invocation.graphql("query Folders { repo { folders { id path name kind } } }", json!({}))
        }

        pub fn file_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            path_mutation(invocation, "mutation FileCreate($path: String!) { fileCreate(path: $path) { id path name uri } }")
        }

        pub fn file_move(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            move_mutation(invocation, "mutation FileMove($src: String!, $dst: String!) { fileMove(src: $src, dst: $dst) { id path name uri } }")
        }

        pub fn file_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            path_mutation(invocation, "mutation FileDelete($path: String!) { fileDelete(path: $path) }")
        }

        pub fn file_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            invocation.graphql("query Files { repo { files { id path name kind extension } } }", json!({}))
        }

        pub fn section_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let file = flag_or_arg(invocation, "file", 0);
            let name = flag_or_arg(invocation, "name", 1);
            let parent = flag_or_arg(invocation, "parent", 2);
            if file.is_empty() || name.is_empty() {
                return Err("missing file or name".into());
            }
            let mut variables = json!({ "file": file, "name": name });
            if !parent.is_empty() {
                variables["parent"] = json!(parent);
            }
            invocation.graphql("mutation SectionCreate($file: String!, $name: String!, $parent: String) { sectionCreate(file: $file, name: $name, parent: $parent) { id name range { start end } } }", variables)
        }

        pub fn section_move(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let file = flag_or_arg(invocation, "file", 0);
            let old = flag_or_arg(invocation, "old", 1);
            let new = flag_or_arg(invocation, "new", 2);
            if file.is_empty() || old.is_empty() || new.is_empty() {
                return Err("missing file or names".into());
            }
            invocation.graphql("mutation SectionMove($file: String!, $oldName: String!, $newName: String!) { sectionMove(file: $file, oldName: $oldName, newName: $newName) { id name range { start end } } }", json!({ "file": file, "oldName": old, "newName": new }))
        }

        pub fn section_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let file = flag_or_arg(invocation, "file", 0);
            let name = flag_or_arg(invocation, "name", 1);
            if file.is_empty() || name.is_empty() {
                return Err("missing file or name".into());
            }
            invocation.graphql("mutation SectionDelete($file: String!, $name: String!) { sectionDelete(file: $file, name: $name) }", json!({ "file": file, "name": name }))
        }

        pub fn section_integrate(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let source = flag_or_arg(invocation, "source", 0);
            let target_section = flag_or_arg(invocation, "target-section", 1);
            let target_file = flag_or_arg(invocation, "target-file", 2);
            let target_parent = flag_or_arg(invocation, "target-parent", 3);
            if source.is_empty() || target_section.is_empty() || target_file.is_empty() {
                return Err("missing source, target section, or target file".into());
            }
            let mut variables = json!({ "source": source, "targetSection": target_section, "targetFile": target_file });
            if !target_parent.is_empty() {
                variables["targetParent"] = json!(target_parent);
            }
            invocation.graphql("mutation Integrate($source: String!, $targetSection: String!, $targetFile: String!, $targetParent: String) { integrate(source: $source, targetSection: $targetSection, targetFile: $targetFile, targetParent: $targetParent) { id } }", variables)
        }

        pub fn section_extract(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let source_file = flag_or_arg(invocation, "source-file", 0);
            let source_section = flag_or_arg(invocation, "source-section", 1);
            let target_file = flag_or_arg(invocation, "target-file", 2);
            if source_file.is_empty() || source_section.is_empty() || target_file.is_empty() {
                return Err("missing source file, source section, or target file".into());
            }
            invocation.graphql("mutation Extract($sourceFile: String!, $sourceSection: String!, $targetFile: String!) { extract(sourceFile: $sourceFile, sourceSection: $sourceSection, targetFile: $targetFile) { id } }", json!({ "sourceFile": source_file, "sourceSection": source_section, "targetFile": target_file }))
        }

        pub fn section_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let file = if invocation.args().is_empty() { invocation.text("file") } else { invocation.arg(0) };
            invocation.graphql("query Sections($path: String!) { file(path: $path) { sections { id path name range { start end } definitions { name } children { name } } } }", json!({ "path": file }))
        }

        /// 🚚️ Moves one artifact onto another by the kinds its two references name.
        pub fn move_artifact(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let context = invocation.context();
            let root = context.root().to_path_buf();
            let source = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(0));
            let target = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(1));
            let lines = match (source.kind.as_str(), target.kind.as_str()) {
                ("file", "file") => {
                    context.file_move(&source.path, &target.path).map_err(|error| CliError::Message(error.message))?;
                    vec![format!("\n📄️Moved file: {} → {}", source.path, target.path)]
                }
                ("folder", "folder") => {
                    context.folder_move(&source.path, &target.path).map_err(|error| CliError::Message(error.message))?;
                    vec![format!("\n📁️Moved folder: {} → {}", source.path, target.path)]
                }
                ("section", "section") if source.path == target.path => {
                    let (Some(old), Some(new)) = (source.section_parts.last(), target.section_parts.last()) else { return Err("missing section path".into()) };
                    context.move_section(&source.path, &resolve_section_name(&root, &source.path, old), &resolve_section_name(&root, &target.path, new)).map_err(|error| CliError::Message(error.message))?
                }
                ("file", "section") => {
                    let parts = &target.section_parts;
                    let name = parts.last().map(|slug| resolve_section_name(&root, &target.path, slug)).unwrap_or_default();
                    let parent = if parts.len() > 1 { resolve_section_name(&root, &target.path, &parts[parts.len() - 2]) } else { String::new() };
                    let lines = context.integrate_file(&source.path, &name, &target.path, &parent).map_err(|error| CliError::Message(error.message))?;
                    context.file_delete(&source.path).map_err(|error| CliError::Message(error.message))?;
                    lines
                }
                ("section", "file") => {
                    let Some(slug) = source.section_parts.last() else { return Err("missing section path".into()) };
                    context.extract_section(&source.path, &resolve_section_name(&root, &source.path, slug), &target.path).map_err(|error| CliError::Message(error.message))?
                }
                (from, to) => return Err(CliError::Message(format!("unsupported move: {from} → {to}"))),
            };
            print_lines(invocation, lines);
            Ok(())
        }

        pub fn integrate(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let context = invocation.context();
            let root = context.root().to_path_buf();
            if invocation.args().len() == 2 {
                let source = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(0));
                let target = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(1));
                if source.kind == "file" && target.kind == "section" {
                    let parts = &target.section_parts;
                    let name = parts.last().map(|slug| resolve_section_name(&root, &target.path, slug)).unwrap_or_default();
                    let parent = if parts.len() > 1 { resolve_section_name(&root, &target.path, &parts[parts.len() - 2]) } else { String::new() };
                    let lines = context.integrate_file(&source.path, &name, &target.path, &parent).map_err(|error| CliError::Message(error.message))?;
                    print_lines(invocation, lines);
                    return Ok(());
                }
            }
            let file = flag_or_arg(invocation, "file", 0);
            let (target_file, target_section, parent) = (invocation.text("target-file"), invocation.text("target-section"), invocation.text("parent-section"));
            if file.is_empty() || target_file.is_empty() || target_section.is_empty() {
                return Err("missing file, target-file, or target-section".into());
            }
            let lines = context.integrate_file(&file, &target_section, &target_file, &parent).map_err(|error| CliError::Message(error.message))?;
            print_lines(invocation, lines);
            Ok(())
        }

        pub fn extract(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let context = invocation.context();
            let root = context.root().to_path_buf();
            if invocation.args().len() == 2 {
                let source = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(0));
                let target = semio_framework_repo_identity::parse_artifact_ref(&invocation.arg(1));
                if source.kind == "section" && target.kind == "file" {
                    let Some(slug) = source.section_parts.last() else { return Err("missing section path".into()) };
                    let lines = context.extract_section(&source.path, &resolve_section_name(&root, &source.path, slug), &target.path).map_err(|error| CliError::Message(error.message))?;
                    print_lines(invocation, lines);
                    return Ok(());
                }
            }
            let file = flag_or_arg(invocation, "file", 0);
            let (section, target_file) = (invocation.text("section"), invocation.text("target-file"));
            if file.is_empty() || section.is_empty() || target_file.is_empty() {
                return Err("missing file, section, or target-file".into());
            }
            let lines = context.extract_section(&file, &section, &target_file).map_err(|error| CliError::Message(error.message))?;
            print_lines(invocation, lines);
            Ok(())
        }

        pub fn rename(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let lines = invocation.context().rename_token(&invocation.arg(0), &invocation.arg(1), &invocation.arg(2)).map_err(|error| CliError::Message(error.message))?;
            print_lines(invocation, lines);
            Ok(())
        }

        pub fn sync(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            if !invocation.args().is_empty() {
                return Err(CliError::Message(format!("unknown sync target {:?}", invocation.arg(0))));
            }
            let help = invocation.parsed.selected().help();
            invocation.print(&help);
            Ok(())
        }

        pub fn sync_management(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            invocation.graphql("mutation SyncManagement { syncManagement }", json!({}))
        }

        pub fn search(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let (node, _) = searched_tree(invocation, 0);
            write_tree_node(invocation, &node)
        }

        pub fn list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let (node, _) = searched_tree(invocation, 0);
            let mut nodes = Vec::new();
            flatten_tree(&node, &mut nodes);
            if invocation.flag("sorted") {
                nodes.sort_by(|left, right| left.id.cmp(&right.id));
            }
            let limit = invocation.int("limit");
            if limit > 0 && nodes.len() > limit as usize {
                nodes.truncate(limit as usize);
            }
            let format = invocation.config.format;
            for node in nodes {
                let kind = semio_framework_repo_tree::tree_node_kind_to_entity_kind(node.kind);
                let data: Option<Map<String, Json>> = node.data.as_ref().map(|data| data.iter().map(|(key, value)| (key.clone(), value.clone())).collect());
                let label = if node.uri.is_empty() { node.label.clone() } else { format!("[{}]({})", node.label, node.uri) };
                let line = match (format, data) {
                    (Format::Json, Some(data)) => {
                        let mut wrapped = Map::new();
                        wrapped.insert((if kind.is_empty() { node.kind.as_str() } else { kind }).to_string(), Json::Object(data));
                        go_json(&Json::Object(wrapped))
                    }
                    (Format::Json, None) => serde_json::to_string(&node).map_err(|error| error.to_string())?,
                    (Format::Markdown, Some(data)) => entity::render_markdown(if kind.is_empty() { node.kind.as_str() } else { kind }, &data),
                    (Format::Markdown, None) => format!("- {label}"),
                    (Format::Text, Some(data)) if !kind.is_empty() => entity::render_human(kind, &data, false),
                    (Format::Text, _) => label,
                };
                invocation.println(&line);
            }
            Ok(())
        }

        pub fn query(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let context = invocation.context();
            let source = FsTreeSource::load(&context, true, "");
            let node = semio_framework_repo_tree::build_monorepo_tree(&source, semio_framework_repo_tree::TreeBuildOptions { include_sections: true, ..Default::default() });
            let matched = semio_framework_repo_tree::search_tree_in_memory(&node, &invocation.args().join(" "));
            let mut nodes = Vec::new();
            flatten_tree(&matched, &mut nodes);
            for node in nodes {
                let id = if node.id.is_empty() { node.label.clone() } else { node.id.clone() };
                invocation.println(&id);
            }
            Ok(())
        }

        pub fn tree_monorepo(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut filter = tree_filter(invocation);
            if !invocation.args().is_empty() {
                filter.query = invocation.arg(0);
            }
            let include_sections = filter.only_kinds.as_ref().is_some_and(|kinds| kinds.contains_key(&TreeNodeKind::Section) || kinds.contains_key(&TreeNodeKind::Definition));
            let context = invocation.context();
            let source = FsTreeSource::load(&context, include_sections, &invocation.text("scope"));
            let mut node = semio_framework_repo_tree::build_monorepo_tree(&source, semio_framework_repo_tree::TreeBuildOptions { include_sections, ..Default::default() });
            if !filter.query.is_empty() {
                node = semio_framework_repo_tree::search_tree_in_memory(&node, &filter.query);
            }
            let filtered = semio_framework_repo_tree::filter_monorepo_tree(&node, Some(&filter));
            write_tree_node(invocation, &filtered)
        }

        pub fn tree_goal(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let context = invocation.context();
            let source = FsTreeSource::load(&context, false, "");
            let roots = semio_framework_repo_tree::build_goal_tree(&source.goals, &source.tickets);
            let text = match invocation.config.format {
                Format::Json => format!("{}\n", serde_json::to_string(&roots).map_err(|error| error.to_string())?),
                Format::Markdown => semio_framework_repo_tree::render_goal_tree_nodes(&roots, semio_framework_repo_tree::TreeRenderFormat::Markdown, &semio_framework_repo_tree::DefaultEntityRenderer),
                Format::Text => semio_framework_repo_tree::render_goal_tree_nodes(&roots, semio_framework_repo_tree::TreeRenderFormat::Text, &semio_framework_repo_tree::DefaultEntityRenderer),
            };
            invocation.print(&text);
            Ok(())
        }

        pub fn tree_statute(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            write_tree_forest(invocation, semio_framework_repo_tree::build_statute_tree(&semio_framework_repo_tree::declared_statutes(), &semio_framework_repo_tree::DeclaredStatuteCatalog))
        }

        pub fn tree_territory(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            write_tree_forest(invocation, semio_framework_repo_tree::build_territory_tree(&semio_framework_repo_tree::declared_territories(), &semio_framework_repo_tree::DeclaredStatuteCatalog))
        }

        pub fn export(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let context = invocation.context();
            let output = if invocation.args().is_empty() { context.root().join("repo.events.jsonl") } else { std::path::PathBuf::from(invocation.arg(0)) };
            let entities = export_entities(&context).map_err(CliError::Message)?;
            let snapshot = semio_framework_repo_events::build_export_snapshot(&entities, &semio_framework_repo_events::Uninterrupted).map_err(|error| CliError::Message(error.to_string()))?;
            semio_framework_repo_events::Store::new(&output).append(&snapshot.inputs, &semio_framework_repo_events::Uninterrupted).map_err(|error| CliError::Message(error.to_string()))?;
            let result = Json::Object(
                [
                    ("path", json!(output.display().to_string())),
                    ("snapshot", json!(snapshot.snapshot)),
                    ("technologies", json!(snapshot.count("technology"))),
                    ("bundles", json!(snapshot.count("bundle"))),
                    ("folders", json!(snapshot.count("folder"))),
                    ("files", json!(snapshot.count("file"))),
                    ("sections", json!(snapshot.count("section"))),
                    ("definitions", json!(snapshot.count("definition"))),
                ]
                .into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect(),
            );
            invocation.println(&serde_json::to_string_pretty(&result).map_err(|error| error.to_string())?);
            Ok(())
        }

        pub fn hook(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            use std::io::{IsTerminal, Read};
            let root = invocation.root();
            let client = invocation.arg(1);
            let mut input: Option<Json> = None;
            if !std::io::stdin().is_terminal() {
                let mut raw = String::new();
                if std::io::stdin().read_to_string(&mut raw).is_ok() && !raw.trim().is_empty() {
                    input = serde_json::from_str(&raw).ok();
                }
            }
            let mut tool_name = invocation.text("tool-name");
            if tool_name.is_empty() {
                tool_name = semio_framework_repo_hooks::extract_tool_name(input.as_ref());
            }
            let (event, resolved_parent) = semio_framework_repo_hooks::resolve_hook_event(&invocation.arg(0), &client, &tool_name, input.as_ref()).map_err(|error| CliError::Message(error.message))?;
            let mut parent = invocation.text("parent");
            if parent.is_empty() {
                parent = resolved_parent;
            }
            let second = semio_framework_repo_tickets::Clock::stamp(&semio_framework_repo_tickets::SystemClock).replacen(' ', "T", 1) + "Z";
            let context = semio_framework_repo_hooks::HookContext {
                event: event.as_str().to_string(),
                client: client.clone(),
                second,
                repo_root: root.display().to_string(),
                tool_name,
                tool_args: invocation.text("tool-args"),
                file_path: invocation.text("file"),
                parent_info: parent.clone(),
                extra: BTreeMap::new(),
                input: input.clone(),
            };
            let environment = semio_framework_repo_hooks::SystemHookEnvironment::new(&root);
            let result = semio_framework_repo_hooks::dispatch_hook(&context, &environment, &semio_framework_repo_hooks::WorkspaceTestFileResolver::new(&root));
            if semio_framework_repo_hooks::hook_event_kind(event) != semio_framework_repo_hooks::HookKind::Version {
                let config = semio_framework_repo_hooks::load_repo_config(&root);
                if config.logging.session {
                    #[cfg(unix)]
                    let parent_pid = Some(std::os::unix::process::parent_id());
                    #[cfg(not(unix))]
                    let parent_pid = None;
                    let session = semio_framework_repo_hooks::resolve_log_session_id(&context, parent_pid);
                    let (year, month, day) = semio_framework_repo_tickets::Clock::today(&semio_framework_repo_tickets::SystemClock);
                    let directory = semio_framework_repo_hooks::session_log_dir(&root, year as i32, month as u32, day as u32, &session);
                    let mut store = semio_framework_repo_hooks::DirectorySessionStore::new(directory.parent().map(std::path::Path::to_path_buf).unwrap_or(directory.clone()));
                    let _ = semio_framework_repo_hooks::record_session_hook(&mut store, &context, &result, &session, &config.logging, &environment);
                }
            }
            let native = semio_framework_repo_hooks::resolve_native_event_name(&client, event, &parent, input.as_ref());
            let output = semio_framework_repo_hooks::render_hook_output(&client, event, &parent, &native, &result, invocation.config.format == Format::Json);
            if !output.stdout.is_empty() {
                invocation.println(&output.stdout);
            }
            if !output.stderr.is_empty() {
                let _ = writeln!(invocation.err, "{}", output.stderr);
            }
            if output.exit_code != 0 {
                return Err(CliError::Exit(output.exit_code));
            }
            Ok(())
        }

        /// 📄️ Every considered source file with its physical line count, the treemaps' input.
        fn counted_files(root: &std::path::Path) -> Vec<(String, i64)> {
            let codebase = Codebase::new(root);
            codebase
                .scope_to_files(&CodebaseScope::Repo)
                .into_iter()
                .filter_map(|path| {
                    let content = std::fs::read_to_string(root.join(&path)).ok()?;
                    let lines = if content.is_empty() { 0 } else { content.matches('\n').count() as i64 + 1 };
                    Some((path, lines))
                })
                .filter(|(_, lines)| *lines > 0)
                .collect()
        }

        pub fn mermaid_technologies(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let root = invocation.root();
            let codebase = Codebase::new(&root);
            let technologies = codebase.technologies();
            let files: Vec<semio_framework_repo_tree::MermaidLocFile> = counted_files(&root)
                .into_iter()
                .filter_map(|(path, loc)| {
                    let bundle = codebase.bundle_by_path(&path)?;
                    let root_path = bundle.root.trim_end_matches('/').to_string();
                    let relative = path.strip_prefix(&format!("{root_path}/")).unwrap_or(&path).to_string();
                    let folder = relative.rsplit_once('/').map(|(folder, _)| folder.to_string()).unwrap_or_default();
                    let name = relative.rsplit('/').next().unwrap_or(&relative).to_string();
                    Some(semio_framework_repo_tree::MermaidLocFile { bundle_name: bundle.name, folder, kind: semio_framework_repo_model::derive_file_kind(&name), name, loc })
                })
                .collect();
            invocation.print(&semio_framework_repo_tree::mermaid_loc_by_technologies_bundles_folders_files(&technologies, &files));
            Ok(())
        }

        pub fn mermaid_contributors(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let root = invocation.root();
            let mut authors: BTreeMap<String, i64> = BTreeMap::new();
            for (path, _) in counted_files(&root) {
                let Ok(output) = std::process::Command::new("git").args(["blame", "--line-porcelain"]).arg(root.join(&path)).current_dir(&root).output() else { continue };
                if !output.status.success() {
                    continue;
                }
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    if let Some(author) = line.strip_prefix("author ") {
                        *authors.entry(author.to_string()).or_default() += 1;
                    }
                }
            }
            invocation.print(&semio_framework_repo_tree::mermaid_loc_flat(semio_framework_repo_tree::MERMAID_TITLE_BY_CONTRIBUTORS, &authors));
            Ok(())
        }

        pub fn mermaid_languages(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let root = invocation.root();
            let mut languages: BTreeMap<String, i64> = BTreeMap::new();
            for (path, loc) in counted_files(&root) {
                if let Some(language) = semio_framework_repo_languages::language_for_path(&path) {
                    *languages.entry(language.name.clone()).or_default() += loc;
                }
            }
            invocation.print(&semio_framework_repo_tree::mermaid_loc_flat(semio_framework_repo_tree::MERMAID_TITLE_BY_LANGUAGE, &languages));
            Ok(())
        }

        pub fn loc(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let root = invocation.root();
            let mut languages = invocation.texts("languages");
            if languages.is_empty() {
                languages = semio_framework_repo_metrics::DEFAULT_CODE_LANGUAGES.iter().map(|language| (*language).to_string()).collect();
            }
            let history = invocation.flag("history");
            let by_contributors = invocation.flag("by-contributors");
            let branch = if history { Some(invocation.text("branch")).filter(|branch| !branch.trim().is_empty()).unwrap_or_else(|| semio_framework_repo_metrics::DEFAULT_BRANCH.to_string()) } else { String::new() };
            let options = semio_framework_repo_metrics::LocOptions { languages, history, by_contributors, branch, contributor: invocation.text("by-contributor") };
            let ignore = semio_framework_repo_workspace::GitIgnore::compile_file(&root.join(".gitignore")).ok();
            let store = semio_framework_repo_contributors::FsContributorStore::new(&root);
            let alias = |name: &str, email: &str| {
                let author = if email.trim().is_empty() { name.trim().to_string() } else { format!("{} <{}>", name.trim(), email.trim()) };
                if author.is_empty() {
                    return "unknown".to_string();
                }
                semio_framework_repo_contributors::find_contributor(&store, &author)
            };
            let report = semio_framework_repo_metrics::build_loc_report(&semio_framework_repo_metrics::SystemGit::new(&root), &options, ignore.as_ref(), &alias).map_err(CliError::Message)?;
            let text = match invocation.config.format {
                Format::Json => format!("{}\n", serde_json::to_string_pretty(&json!({ "loc": report })).map_err(|error| error.to_string())?),
                Format::Markdown => semio_framework_repo_metrics::render_markdown(&report, history, by_contributors),
                Format::Text => semio_framework_repo_metrics::render_text(&report, history, by_contributors),
            };
            invocation.print(&text);
            Ok(())
        }

        pub fn technology(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            if invocation.args().len() == 3 && invocation.arg(1) == "generate" {
                return match invocation.arg(2).as_str() {
                    kind @ ("requirements" | "docs" | "todos") => Err(CliError::Message(format!("technology {} generate {kind}: the 🏃️test-runner Rust twin carries no technology document generator", invocation.arg(0)))),
                    kind => Err(CliError::Message(format!("unknown generate kind {kind:?} (use requirements, docs, or todos)"))),
                };
            }
            let help = invocation.parsed.selected().help();
            invocation.print(&help);
            Ok(())
        }

        /// 📜️ Every technology the stream options keep.
        fn filtered_technologies(invocation: &Invocation<'_>) -> Vec<semio_framework_repo_model::Technology> {
            let options = StreamOptions::of(invocation);
            Codebase::new(&invocation.root()).technologies().into_iter().filter(|technology| options.matches_filter(&technology.name) && options.matches_query(&format!("{} {}", technology.name, technology.kind.as_str()))).collect()
        }

        pub fn technology_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let results = filtered_technologies(invocation).into_iter().map(|technology| json!({ "technology": technology })).collect();
            invocation.stream("technology list", results, Vec::new())
        }

        pub fn technology_tree(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut results: Vec<Json> = filtered_technologies(invocation).into_iter().map(|technology| json!({ "technology": technology })).collect();
            results.sort_by_key(|data| format_markdown_result("technology tree", data));
            invocation.stream("technology tree", results, Vec::new())
        }

        /// 🔵️ Every bundle the stream and status options keep, twin of `StreamBundles`.
        fn filtered_bundles(invocation: &Invocation<'_>) -> Vec<semio_framework_repo_model::Bundle> {
            let options = StreamOptions::of(invocation);
            let root = invocation.root();
            let codebase = Codebase::new(&root);
            let status = status_filter(invocation);
            let open_bundles: BTreeSet<String> = match &status {
                Some(_) => invocation
                    .context()
                    .tickets(None, None, None, Some(TicketStatus::Open))
                    .unwrap_or_default()
                    .iter()
                    .flat_map(|ticket| ticket.interactions.iter().flat_map(|interaction| interaction.files.iter()))
                    .filter_map(|file| {
                        let parts: Vec<&str> = file.path.split('/').collect();
                        (parts.len() >= 2 && parts[0].starts_with('@')).then(|| format!("{}/{}", parts[0], parts[1]))
                    })
                    .collect(),
                None => BTreeSet::new(),
            };
            codebase
                .bundles()
                .into_iter()
                .filter(|bundle| options.matches_filter(&bundle.name) && options.matches_query(&format!("{} {}", bundle.name, bundle.kind.as_str())) && options.includes_bundle_kind(bundle.kind.as_str()))
                .filter(|bundle| {
                    if options.include_kinds.is_empty() && options.exclude_kinds.is_empty() {
                        return true;
                    }
                    let kinds: BTreeSet<String> = codebase.glob_by_extension(&format!("{}/**/*", bundle.root), &[], &[], true).iter().map(|path| semio_framework_repo_model::derive_file_kind(&semio_framework_repo_identity::base_of(path)).as_str().to_string()).collect();
                    (options.include_kinds.is_empty() || options.include_kinds.iter().any(|kind| kinds.contains(kind))) && !options.exclude_kinds.iter().any(|kind| kinds.contains(kind))
                })
                .filter(|bundle| match status.as_deref() {
                    Some("open") => open_bundles.contains(&bundle.name),
                    Some("closed") => !open_bundles.contains(&bundle.name),
                    _ => true,
                })
                .collect()
        }

        pub fn bundle_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let results = filtered_bundles(invocation).into_iter().map(|bundle| json!({ "bundle": bundle })).collect();
            invocation.stream("bundle list", results, Vec::new())
        }

        pub fn bundle_tree(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut bundles = filtered_bundles(invocation);
            bundles.sort_by(|left, right| left.name.cmp(&right.name));
            if invocation.config.format == Format::Markdown {
                let markdown: String = bundles.iter().filter_map(|bundle| serde_json::to_value(bundle).ok()).filter_map(|value| value.as_object().cloned()).map(|data| format!("{}\n", entity::render_markdown("bundle", &data))).collect();
                return invocation.stream("bundle tree", vec![json!({ "markdown": markdown })], Vec::new());
            }
            let last = bundles.len().saturating_sub(1);
            let mut lines = vec![".".to_string()];
            lines.extend(bundles.iter().enumerate().map(|(index, bundle)| format!("{}{}", if index == last { "└️─️─️ " } else { "├️─️─️ " }, bundle.name)));
            invocation.stream("bundle tree", Vec::new(), lines)
        }

        pub fn analyze(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut variables = Map::new();
            if !invocation.args().is_empty() {
                variables.insert("scope".to_string(), json!(invocation.arg(0)));
            }
            let started = std::time::Instant::now();
            let context = invocation.context();
            let events = graphql_events(&context, "query Analyze($scope: String) { analyze(scope: $scope) { breachs { id summary scope line column excerpt kind { id priority autofixable reason solution } } metrics { total autofixable byPriority { high medium low } } } }", &variables);
            let events: Vec<Event> = events.into_iter().map(|mut event| {
                if event.kind == KIND_START || event.kind == KIND_RESULT || event.kind == KIND_ERROR {
                    event.command = "analyze".to_string();
                }
                event
            }).collect();
            invocation.render(&events, started)
        }

        pub fn autofix(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let started = std::time::Instant::now();
            let scope = invocation.args().first().cloned();
            let fixed = invocation.context().autofix(scope.as_deref()).map_err(|error| CliError::Message(error.message))?;
            let events = vec![Event::start("autofix"), Event::result("autofix", json!({ "fix": fixed })), Event::finish(EXIT_OK, "ok")];
            invocation.render(&events, started)
        }

        pub fn entity_emojis(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let emojis = semio_framework_repo_identity::all_entity_emojis();
            if invocation.config.format == Format::Json {
                invocation.println(&go_json(&json!(emojis)));
                return Ok(());
            }
            for emoji in emojis {
                invocation.println(&emoji);
            }
            Ok(())
        }

        pub fn configure(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let root = invocation.root();
            let removed = semio_framework_repo_providers::remove_git_hooks(&root).map_err(|error| CliError::Message(error.message))?;
            semio_framework_repo_providers::install_micro_commit_hooks(&root).map_err(|error| CliError::Message(error.message))?;
            invocation.println("repo config generation is disabled; edit checked-in config files manually");
            if removed.is_empty() {
                invocation.println("git hooks: none to remove; micro-commit hooks installed");
            } else {
                invocation.println(&format!("git hooks removed: {}; micro-commit hooks installed", removed.join(", ")));
            }
            Ok(())
        }

        pub fn micro_commit(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let root = invocation.root();
            let pinned = std::env::var("SEMIO_MICRO_COMMIT_BUN").ok();
            let bun_install = std::env::var("BUN_INSTALL").ok();
            let bun = semio_framework_repo_hooks::bun_candidates(&root, pinned.as_deref(), bun_install.as_deref())
                .into_iter()
                .find(|candidate| candidate.is_file())
                .or_else(|| semio_framework_repo_workspace::look_path("bun"))
                .ok_or_else(|| CliError::Message("bun not found".to_string()))?;
            let argv = semio_framework_repo_hooks::micro_commit_argv(&bun.display().to_string(), invocation.args());
            let status = std::process::Command::new(&argv[0]).args(&argv[1..]).current_dir(&root).status().map_err(|error| CliError::Message(error.to_string()))?;
            match status.code() {
                Some(0) => Ok(()),
                Some(code) => Err(CliError::Exit(code)),
                None => Err(CliError::Exit(EXIT_ERROR)),
            }
        }

        pub fn benchmark(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            if invocation.flag("dry-run") {
                return Ok(());
            }
            let root = invocation.root();
            let tasks: [(&str, &str, &[&str], &str); 5] = [
                ("Typescript", "npx", &["tsx", "compose.benchmark.ts"], "js/compose"),
                ("Python", "uv", &["run", "compose.benchmark.py"], "py/compose"),
                ("Go", "go", &["run", "compose_benchmark.go"], "go/compose"),
                ("C#", "dotnet", &["run", "--technology", "Compose.Benchmark/Compose.Benchmark.csproj", "--configuration", "Release"], "net"),
                ("Rust", "cargo", &["run", "--release", "--bin", "compose-benchmark"], "rs/compose"),
            ];
            invocation.println("Running benchmarks...");
            let mut results = Vec::new();
            for (name, program, args, directory) in tasks {
                invocation.println(&format!("Running {name}..."));
                let directory = root.join(directory);
                if !directory.is_dir() {
                    invocation.println(&format!("Skipping {name}: directory {} not found", directory.display()));
                    continue;
                }
                match semio_framework_repo_workspace::spawn_command(program).args(args).current_dir(&directory).output() {
                    Ok(output) if output.status.success() => results.extend(semio_framework_repo_metrics::parse_benchmark_output(name, &String::from_utf8_lossy(&output.stdout))),
                    Ok(output) => invocation.println(&format!("{name} failed: {}\n{}", output.status, String::from_utf8_lossy(&output.stderr))),
                    Err(error) => invocation.println(&format!("{name} failed: {error}")),
                }
            }
            if results.is_empty() {
                return Ok(());
            }
            let report = semio_framework_repo_workspace::repo_meta_path_for_root(&root, "📊️metrics/benchmark.csv");
            if let Some(parent) = report.parent() {
                std::fs::create_dir_all(parent).map_err(|error| CliError::Message(error.to_string()))?;
            }
            std::fs::write(&report, semio_framework_repo_metrics::benchmark_csv(&results)).map_err(|error| CliError::Message(error.to_string()))?;
            invocation.println(&format!("Benchmark report written to {}", report.display()));
            Ok(())
        }

        pub fn update(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let target = if invocation.args().is_empty() { "all".to_string() } else { invocation.arg(0) };
            invocation.println("=== Dependency Update Script ===");
            if !invocation.flag("apply") || invocation.flag("dry-run") {
                invocation.println("Running in DRY RUN mode - no changes will be made.");
                invocation.println(&format!("Target: {target}"));
                invocation.println("\n=== Update Complete ===");
                return Ok(());
            }
            invocation.println(&format!("Target: {target}"));
            let root = invocation.root();
            let document = std::fs::read_to_string(root.join(".github").join("dependabot.yml")).map_err(|error| CliError::Message(format!("dependabot.yml not found: {error}")))?;
            let dependabot = semio_framework_repo_yaml::unmarshal(&document).map_err(CliError::Message)?;
            let mut paths: BTreeMap<&str, Vec<String>> = BTreeMap::new();
            for update in dependabot.get("updates").and_then(Json::as_array).map(Vec::as_slice).unwrap_or_default() {
                let directory = update.get("directory").and_then(Json::as_str).unwrap_or_default().trim_start_matches('/').to_string();
                let ecosystem = match update.get("package-ecosystem").and_then(Json::as_str).unwrap_or_default() {
                    "uv" => "python",
                    "cargo" => "rust",
                    "gomod" => "go",
                    _ => continue,
                };
                paths.entry(ecosystem).or_default().push(directory);
            }
            let run = |invocation: &mut Invocation<'_>, directory: &std::path::Path, program: &str, args: &[&str]| {
                invocation.println(&format!("  Running: {program} {} in {}", args.join(" "), directory.display()));
                let _ = semio_framework_repo_workspace::spawn_command(program).args(args).current_dir(directory).status();
            };
            if target == "all" || target == "npm" {
                invocation.println("\n[NPM] Updating npm packages...");
                run(invocation, &root, "npm", &["update", "-S"]);
                invocation.println("[NPM] Done.");
            }
            for (ecosystem, label, manifest, commands) in [
                ("python", "Python", "pyproject.toml", vec![vec!["uv", "lock", "--upgrade"]]),
                ("rust", "Rust", "Cargo.toml", vec![vec!["cargo", "update"]]),
                ("go", "Go", "go.mod", vec![vec!["go", "get", "-u", "./..."], vec!["go", "mod", "tidy"]]),
            ] {
                if target != "all" && target != ecosystem {
                    continue;
                }
                invocation.println(&format!("\n[{label}] Updating {label} packages..."));
                for directory in paths.get(ecosystem).cloned().unwrap_or_default() {
                    let full = root.join(&directory);
                    if !full.join(manifest).is_file() {
                        continue;
                    }
                    invocation.println(&format!("  Updating {directory}..."));
                    for command in &commands {
                        run(invocation, &full, command[0], &command[1..]);
                    }
                }
                invocation.println(&format!("[{label}] Done."));
            }
            invocation.println("\n=== Update Complete ===");
            Ok(())
        }

        pub fn auth_whoami(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let info = semio_framework_repo_events::server_whoami().map_err(|error| CliError::Message(format!("not authenticated: {error}")))?;
            if invocation.config.format == Format::Json {
                invocation.println(&serde_json::to_string_pretty(&info).map_err(|error| error.to_string())?);
                return Ok(());
            }
            let field = |key: &str| info.get(key).map(go_sprint).unwrap_or_else(|| "%!s(<nil>)".to_string());
            invocation.println(&format!("Email: {}", field("email")));
            invocation.println(&format!("Name: {}", field("display_name")));
            invocation.println(&format!("Role: {}", field("role")));
            Ok(())
        }

        pub fn auth_status(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let address = semio_framework_repo_events::server_addr();
            if address.is_empty() {
                invocation.println("Server: not configured (set COMPOSE_SERVER_ADDR)");
                return Ok(());
            }
            invocation.println(&format!("Server: {address}"));
            if semio_framework_repo_events::server_token().is_empty() {
                invocation.println("Token: not set (set COMPOSE_SERVER_TOKEN)");
                return Ok(());
            }
            invocation.println("Token: configured");
            match semio_framework_repo_events::server_whoami() {
                Ok(info) => {
                    let field = |key: &str| info.get(key).map(go_sprint).unwrap_or_default();
                    invocation.println(&format!("Auth: {} ({})", field("email"), field("role")));
                }
                Err(error) => invocation.println(&format!("Auth: failed ({error})")),
            }
            Ok(())
        }

        pub fn statute_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let options = StreamOptions::of(invocation);
            let results = semio_framework_repo_statutes::statutes()
                .iter()
                .filter(|meta| options.matches_filter(meta.kind.0.as_str()) && options.matches_query(meta.kind.0.as_str()))
                .map(|meta| json!({ "statute": meta }))
                .collect();
            invocation.stream("statute list", results, Vec::new())
        }

        pub fn statute_tree(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let options = StreamOptions::of(invocation);
            let mut kinds: Vec<semio_framework_repo_model::Statute> = semio_framework_repo_statutes::statutes().iter().filter(|meta| options.matches_filter(meta.kind.0.as_str()) && options.matches_query(meta.kind.0.as_str())).map(|meta| meta.kind.clone()).collect();
            kinds.sort_by(|left, right| left.0.cmp(&right.0));
            let roots = semio_framework_repo_tree::build_statute_tree(&kinds, &semio_framework_repo_tree::DeclaredStatuteCatalog);
            if invocation.config.format == Format::Markdown {
                fn walk(nodes: &[TreeNode], indent: &str, out: &mut String) {
                    for node in nodes {
                        match &node.data {
                            Some(data) => {
                                let mut entry = Map::new();
                                entry.insert("id".to_string(), data.get("id").cloned().unwrap_or(Json::Null));
                                entry.insert("description".to_string(), data.get("reason").cloned().unwrap_or(Json::Null));
                                out.push_str(&format!("{indent}{}\n", entity::render_markdown("statute", &entry)));
                            }
                            None => out.push_str(&format!("{indent}- {}\n", node.label)),
                        }
                        walk(node.children.as_deref().unwrap_or_default(), &format!("{indent}  "), out);
                    }
                }
                let mut markdown = String::new();
                walk(&roots, "", &mut markdown);
                return invocation.stream("statute tree", vec![json!({ "markdown": markdown })], Vec::new());
            }
            let mut lines = Vec::new();
            text_tree_lines(&roots, "", &mut lines);
            invocation.stream("statute tree", Vec::new(), lines)
        }

        pub fn checkpoint_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let options = StreamOptions::of(invocation);
            let limit = if invocation.int("limit") > 0 { invocation.int("limit") } else { 100 };
            let results = invocation
                .context()
                .checkpoints(Some(limit))
                .map_err(|error| CliError::Message(error.message))?
                .into_iter()
                .filter(|checkpoint| options.matches_filter(&format!("{} {}", checkpoint.sha, checkpoint.title)) && options.matches_query(&format!("{} {}", checkpoint.sha, checkpoint.title)))
                .map(|checkpoint| json!({ "checkpoint": checkpoint }))
                .collect();
            invocation.stream("checkpoint list", results, Vec::new())
        }

        pub fn interaction_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let mut interactions = invocation.context().interactions().map_err(|error| CliError::Message(error.message))?;
            if invocation.flag("sorted") {
                interactions.sort_by(|left, right| left.interaction.date.cmp(&right.interaction.date));
            }
            let results = interactions.into_iter().map(|interaction| json!({ "interaction": interaction })).collect();
            invocation.stream("interaction list", results, Vec::new())
        }

        pub fn interaction_tree(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let context = invocation.context();
            let mut nodes: Vec<(String, String, String)> = Vec::new();
            for goal in context.goals().unwrap_or_default() {
                nodes.push((goal.id.clone(), format!("{} - {} - {}", entity::goal_artifact_id(&goal.id), goal.title, goal.status.as_str()), goal.parent.clone()));
            }
            for ticket in context.tickets(None, None, None, None).unwrap_or_default() {
                let id = emoji_text(identity_entity("ticket")) + &flat(&ticket.slug);
                for interaction in &ticket.interactions {
                    nodes.push((format!("{id}/{}/{}", interaction.kind, interaction.date), format!("🔄️ {} [{}] {} @{}", interaction.kind, interaction.date, interaction.client, interaction.author), id.clone()));
                }
                nodes.push((id.clone(), format!("{id} - {} - {}", ticket.title, ticket.status.as_str()), ticket.goal.clone()));
            }
            let known: BTreeSet<String> = nodes.iter().map(|(id, _, _)| id.clone()).collect();
            fn render(nodes: &[(String, String, String)], known: &BTreeSet<String>, parent: Option<&str>, prefix: &str, lines: &mut Vec<String>) {
                let mut level: Vec<&(String, String, String)> = nodes
                    .iter()
                    .filter(|(id, _, owner)| match parent {
                        None => owner.is_empty() || owner == id || !known.contains(owner),
                        Some(parent) => owner == parent && owner != id,
                    })
                    .collect();
                level.sort_by(|left, right| left.0.cmp(&right.0));
                let last = level.len().saturating_sub(1);
                for (index, (id, label, _)) in level.into_iter().enumerate() {
                    let (connector, child_prefix) = if index == last { ("└️─️─️ ", format!("{prefix}    ")) } else { ("├️─️─️ ", format!("{prefix}│️   ")) };
                    lines.push(format!("{prefix}{connector}{label}"));
                    render(nodes, known, Some(id), &child_prefix, lines);
                }
            }
            let mut lines = Vec::new();
            render(&nodes, &known, None, "", &mut lines);
            invocation.stream("interaction tree", Vec::new(), lines)
        }

        pub fn draft_create(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            if invocation.args().is_empty() {
                return Err("missing title".into());
            }
            let draft = invocation.context().draft_create(DraftCreateInput { title: invocation.arg(0), files: invocation.texts("files") }).map_err(|error| CliError::Message(error.message))?;
            invocation.stream("draft create", vec![json!({ "draft": draft })], Vec::new())
        }

        pub fn draft_delete(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            if invocation.args().is_empty() {
                return Err("missing slug".into());
            }
            invocation.context().draft_delete(&invocation.arg(0)).map_err(|error| CliError::Message(error.message))?;
            invocation.stream("draft delete", Vec::new(), Vec::new())
        }

        pub fn draft_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            invocation.graphql("query Drafts { drafts { id uri } }", json!({}))
        }

        pub fn definition_list(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
            let path = flag_or_arg(invocation, "file", 0);
            if path.is_empty() {
                return Err("missing file".into());
            }
            let options = StreamOptions::of(invocation);
            let content = std::fs::read_to_string(invocation.root().join(&path)).map_err(|error| CliError::Message(format!("{path}: {error}")))?;
            let results = semio_framework_repo_codebase::file_definitions(&content, &path)
                .into_iter()
                .filter(|definition| options.includes_definition_kind(definition.kind.as_str()) && options.matches_filter(&definition.name) && options.matches_query(&definition.name))
                .map(|definition| json!({ "definition": definition }))
                .collect();
            invocation.stream("definition list", results, Vec::new())
        }
    }

    //#endregion 🚦️Verbs

    //#region 🧪️Projection

    /// 🧾️ Parses one argv against the repo command tree and projects the selected command path, the
    /// operands, only the flags the caller passed, and whether `--help` short-circuited, as JSON text.
    pub fn projection(argv: &[String]) -> Result<String, String> {
        let root = root();
        let parsed = parse(&root, argv)?;
        let path: Vec<Json> = parsed.chain.iter().map(|command| Json::String(command.name().to_string())).collect();
        let positional: Vec<Json> = if parsed.help { Vec::new() } else { parsed.positional.iter().cloned().map(Json::String).collect() };
        let flags: Map<String, Json> = parsed.flags.iter().filter(|(_, (_, changed))| *changed).map(|(name, (value, _))| (name.clone(), value.projection())).collect();
        let mut projection = Map::new();
        projection.insert("path".to_string(), Json::Array(path));
        projection.insert("positional".to_string(), Json::Array(positional));
        projection.insert("flags".to_string(), Json::Object(flags));
        projection.insert("help".to_string(), Json::Bool(parsed.help));
        Ok(go_json(&Json::Object(projection)))
    }

    /// 📖️ The usage text of one command path below the root, `None` when no such command exists.
    pub fn usage(path: &[String]) -> Option<String> {
        let root = root();
        let mut command = &root;
        for name in path {
            command = command.child(name)?;
        }
        Some(command.help())
    }

    /// 🗺️ Every command path of the tree, root first, each as `/`-joined names.
    pub fn command_paths() -> Vec<String> {
        let root = root();
        let mut paths = vec![String::new()];
        for child in &root.children {
            paths.push(child.name().to_string());
            for grandchild in &child.children {
                paths.push(format!("{}/{}", child.name(), grandchild.name()));
            }
        }
        paths
    }

    /// 🔤️ The format a projection names: `json`, `md`, anything else is the human text.
    fn projection_format(raw: &str) -> Format {
        Format::parse(raw)
    }

    /// 🖨️ Renders one recorded event stream given as JSON text and answers `out`, `err` and
    /// `exitCode` as JSON text.
    pub fn render_stream_json(events: &str, format: &str, is_tty: bool, verbose: bool, elapsed_ms: i64) -> Result<String, String> {
        let events: Vec<Event> = serde_json::from_str(events).map_err(|error| error.to_string())?;
        let rendered = render(&events, projection_format(format), is_tty, verbose, elapsed_ms);
        Ok(go_json(&json!({ "out": rendered.out, "err": rendered.err, "exitCode": rendered.exit_code })))
    }

    /// 🕸️ Executes one document against a recorded repository and answers the three renderings of the
    /// stream plus its exit code, as JSON text.
    pub fn graphql_roundtrip_json(records: &str, query: &str, variables: &str) -> Result<String, String> {
        let recording = RecordingContext::from_text(records).map_err(|error| error.message)?;
        let variables: Map<String, Json> = if variables.trim().is_empty() { Map::new() } else { serde_json::from_str(variables).map_err(|error| error.to_string())? };
        let events = graphql_events(&recording, query, &variables);
        let ndjson = render(&events, Format::Json, false, false, 0);
        let human = render(&events, Format::Text, false, false, 0);
        let markdown = render(&events, Format::Markdown, false, false, 0);
        Ok(go_json(&json!({ "ndjson": ndjson.out, "human": human.out, "markdown": markdown.out, "exitCode": ndjson.exit_code })))
    }

    /// 🕸️ The invocations the `test` verb would announce for one snapshot and one operand list, as
    /// JSON text: the `Running: …` lines in order plus the planner's refusals. Nothing executes.
    pub fn test_verb_lines_json(snapshot: &str, operands: &[String]) -> Result<String, String> {
        let snapshot: semio_framework_repo_test_runner::FilesystemSnapshot = serde_json::from_str(snapshot).map_err(|error| error.to_string())?;
        let identity = semio_framework_repo_test_runner::PendingIdentity;
        let scopes = semio_framework_repo_test_runner::resolve_test_scopes(operands, &snapshot, &identity, &identity);
        let plan = semio_framework_repo_test_runner::plan_scopes(&snapshot, &scopes);
        let lines: Vec<String> = plan.invocations.iter().map(|invocation| format!("Running: {} (in {})", invocation.argv.join(" "), invocation.cwd)).collect();
        Ok(go_json(&json!({ "lines": lines, "problems": plan.problems })))
    }

    /// 📤️ The event batch the `export` verb would append for one recorded repository, as JSON text:
    /// the snapshot digest, the per-kind counts in taxonomy order and every input id and record.
    pub fn export_records_json(records: &str) -> Result<String, String> {
        let recording = RecordingContext::from_text(records).map_err(|error| error.message)?;
        let entities = export_entities(&recording)?;
        let snapshot = semio_framework_repo_events::build_export_snapshot(&entities, &semio_framework_repo_events::Uninterrupted).map_err(|error| error.to_string())?;
        let counts: Vec<Json> = ["technology", "bundle", "folder", "file", "section", "definition"].iter().map(|kind| json!({ "kind": kind, "count": snapshot.count(kind) })).collect();
        let ids: Vec<String> = snapshot.inputs.iter().map(|input| input.id.clone()).collect();
        let encoded: Vec<String> = snapshot.inputs.iter().map(|input| format!("{}\u{1}{}", input.kind, input.data.text())).collect();
        Ok(go_json(&json!({ "snapshot": snapshot.snapshot, "counts": counts, "inputIds": ids, "records": encoded })))
    }

    /// 🪝️ What the `hook` verb would write for one native invocation, as JSON text. The environment and
    /// the test-file resolver are the inert ones, so the projection is a pure function of the request.
    pub fn hook_verb_dispatch_json(request: &str) -> Result<String, String> {
        let request: Map<String, Json> = serde_json::from_str(request).map_err(|error| error.to_string())?;
        let text = |key: &str| request.get(key).and_then(Json::as_str).unwrap_or_default().to_string();
        let input = request.get("input").cloned().filter(|input| !input.is_null());
        let mut tool_name = text("toolName");
        if tool_name.is_empty() {
            tool_name = semio_framework_repo_hooks::extract_tool_name(input.as_ref());
        }
        let client = text("client");
        let (event, resolved_parent) = semio_framework_repo_hooks::resolve_hook_event(&text("event"), &client, &tool_name, input.as_ref()).map_err(|error| error.message)?;
        let mut parent = text("parent");
        if parent.is_empty() {
            parent = resolved_parent;
        }
        let context = semio_framework_repo_hooks::HookContext {
            event: event.as_str().to_string(),
            client: client.clone(),
            second: text("second"),
            repo_root: text("repoRoot"),
            tool_name,
            tool_args: text("toolArgs"),
            file_path: text("file"),
            parent_info: parent.clone(),
            extra: BTreeMap::new(),
            input: input.clone(),
        };
        let result = semio_framework_repo_hooks::dispatch_hook(&context, &semio_framework_repo_hooks::InertEnvironment, &semio_framework_repo_hooks::InertTestFileResolver);
        let native = semio_framework_repo_hooks::resolve_native_event_name(&client, event, &parent, input.as_ref());
        let output = semio_framework_repo_hooks::render_hook_output(&client, event, &parent, &native, &result, request.get("json").and_then(Json::as_bool).unwrap_or(false));
        Ok(go_json(&json!({ "stdout": output.stdout, "stderr": output.stderr, "exitCode": output.exit_code })))
    }

    /// 🧺️ A shared byte sink the stdio server writes its responses into.
    #[derive(Clone, Default)]
    struct SharedBuffer(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl Write for SharedBuffer {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    /// 🤝️ Serves one recorded stdio conversation with the server the `mcp` verb builds and answers the
    /// response lines a client would have read, twin of `McpVerbConversation`.
    pub fn mcp_conversation(profile: &str, requests: &[String]) -> Result<Vec<String>, String> {
        let profile = semio_framework_repo_mcp::Profile::parse(profile).map_err(|error| error.to_string())?;
        let root = semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
        let server = std::sync::Arc::new(semio_framework_repo_mcp::repository_server(crate::mcp_verb::RepoRepository::open(&root, profile), profile, semio_framework_repo_mcp::Limits::default()).map_err(|error| error.to_string())?);
        let sink = SharedBuffer::default();
        let input = format!("{}\n", requests.join("\n"));
        match semio_framework_repo_mcp::serve(&server, "stdio", input.as_bytes(), sink.clone()) {
            Ok(()) | Err(semio_framework_repo_mcp::Error::PeerDropped) => {}
            Err(error) => return Err(error.to_string()),
        }
        let written = sink.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        Ok(String::from_utf8_lossy(&written).lines().filter(|line| !line.trim().is_empty()).map(str::to_string).collect())
    }

    //#endregion 🧪️Projection

    //#region 🚀️Run

    /// 🚀️ Runs one repo CLI invocation on the process streams and answers its exit code.
    pub fn run(argv: &[String]) -> i32 {
        use std::io::IsTerminal;
        let stdout = std::io::stdout();
        let stderr = std::io::stderr();
        let is_tty = stdout.is_terminal() && std::env::var_os("NO_COLOR").is_none_or(|value| value.is_empty());
        let mut out = stdout.lock();
        let mut err = stderr.lock();
        run_with(argv, &mut out, &mut err, is_tty)
    }

    /// 🚀️ Runs one repo CLI invocation on the given streams and answers its exit code.
    pub fn run_with(argv: &[String], out: &mut dyn Write, err: &mut dyn Write, is_tty: bool) -> i32 {
        let root = root();
        let parsed = match parse(&root, argv) {
            Ok(parsed) => parsed,
            Err(message) => {
                let _ = writeln!(err, "{message}");
                return EXIT_ERROR;
            }
        };
        if parsed.help {
            let _ = out.write_all(parsed.selected().help().as_bytes());
            return EXIT_OK;
        }
        let selected = parsed.selected();
        let text = |name: &str| match parsed.flags.get(name) {
            Some((FlagValue::Text(value), _)) => value.clone(),
            _ => String::new(),
        };
        let switch = |name: &str| matches!(parsed.flags.get(name), Some((FlagValue::Bool(true), _)));
        let mut format = Format::parse(&text("format"));
        if switch("json") {
            format = Format::Json;
        } else if switch("text") {
            format = Format::Text;
        } else if switch("md") {
            format = Format::Markdown;
        }
        let timeout_nanoseconds = match parsed.flags.get("timeout") {
            Some((FlagValue::Duration(value), _)) => value.to_owned(),
            _ => DEFAULT_TIMEOUT_NANOSECONDS,
        };
        let config = Config { format, verbose: switch("verbose"), repo: text("repo"), timeout_nanoseconds };
        let Some(action) = selected.action else {
            if let Some(first) = parsed.positional.first() {
                let _ = writeln!(err, "unknown command {first:?} for {:?}", selected.name());
                return EXIT_ERROR;
            }
            let _ = out.write_all(selected.help().as_bytes());
            return EXIT_OK;
        };
        let mut invocation = Invocation { parsed, config, out, err, is_tty };
        match action(&mut invocation) {
            Ok(()) => EXIT_OK,
            Err(CliError::Exit(code)) => code,
            Err(CliError::Message(message)) => {
                let _ = writeln!(invocation.err, "{message}");
                EXIT_ERROR
            }
        }
    }

    //#endregion 🚀️Run
}

//#endregion 🧭️RepoCli

//#region 🔌️McpVerb

/// 🔌️ The production repository the repo MCP server serves: every tool reaches the domain owner the
/// matching verb reaches, every resource reads through the repository's GraphQL surface.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust/🦀️.rs
pub mod mcp_verb {
    use semio_framework_repo_graphql::{Executor, FsRepoContext, RepoContext};
    use semio_framework_repo_mcp::{Content, Context, GetPromptResult, HandlerError, Profile, Repository, RepositoryResult, ResourceContent};
    use semio_framework_repo_model::{GoalCloseInput, GoalCreateInput, GoalReopenInput, TicketCloseInput, TicketStatus};
    use semio_framework_repo_providers::McpClientKind;
    use serde_json::{json, Map, Value as Json};
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};

    /// 🔌️ The repository of one repo root, answering as one MCP profile.
    pub struct RepoRepository {
        root: PathBuf,
        profile: Profile,
    }

    impl RepoRepository {
        /// 🆕️ Binds the repository to a root and the profile the server was started with.
        pub fn open(root: &Path, profile: Profile) -> RepoRepository {
            RepoRepository { root: root.to_path_buf(), profile }
        }

        fn context(&self) -> FsRepoContext {
            FsRepoContext::open(&self.root)
        }

        /// 🪪️ The editor surface the profile stands for.
        fn kind(&self) -> McpClientKind {
            match self.profile {
                Profile::Generic => McpClientKind::Generic,
                Profile::Cursor => McpClientKind::Cursor,
                Profile::Kiro => McpClientKind::Kiro,
                Profile::Copilot => McpClientKind::Copilot,
                Profile::Claude => McpClientKind::Claude,
                Profile::Codex => McpClientKind::Codex,
            }
        }

        /// 🛤️ A path argument as a repository-relative path.
        fn relative(&self, path: &str) -> String {
            let root = self.root.display().to_string().replace('\\', "/");
            let path = path.replace('\\', "/");
            path.strip_prefix(&format!("{root}/")).map(str::to_string).unwrap_or(path)
        }

        /// 📚️ One GraphQL read rendered as the YAML a resource carries.
        fn yaml_of_query(&self, query: &str) -> Result<String, HandlerError> {
            let context = self.context();
            let data = Executor::new(&context).execute(query, &Map::new()).map_err(|error| HandlerError::tool(&error.message))?;
            semio_framework_repo_yaml::marshal(&data).map_err(|error| HandlerError::tool(&error))
        }
    }

    /// 🎼️ An optional string argument; a present value that is not a non-empty string is refused.
    fn optional(arguments: &Json, key: &str) -> Result<String, HandlerError> {
        match arguments.get(key) {
            None => Ok(String::new()),
            Some(Json::String(text)) if !text.is_empty() => Ok(text.clone()),
            Some(_) => Err(HandlerError::tool(&format!("invalid {key}"))),
        }
    }

    /// 🎼️ A required string argument.
    fn required(arguments: &Json, key: &str) -> Result<String, HandlerError> {
        if arguments.get(key).is_none() {
            return Err(HandlerError::tool(&format!("missing {key}")));
        }
        optional(arguments, key)
    }

    /// 🎼️ An optional boolean argument.
    fn switch(arguments: &Json, key: &str) -> Result<bool, HandlerError> {
        match arguments.get(key) {
            None => Ok(false),
            Some(Json::Bool(flag)) => Ok(*flag),
            Some(_) => Err(HandlerError::tool(&format!("invalid {key}"))),
        }
    }

    /// 🎼️ An optional string list argument.
    fn strings(arguments: &Json, key: &str) -> Result<Vec<String>, HandlerError> {
        match arguments.get(key) {
            None => Ok(Vec::new()),
            Some(Json::Array(items)) if !items.is_empty() => items.iter().map(|item| item.as_str().filter(|text| !text.is_empty()).map(str::to_string).ok_or_else(|| HandlerError::tool(&format!("invalid {key}")))).collect(),
            Some(_) => Err(HandlerError::tool(&format!("invalid {key}"))),
        }
    }

    /// 🎫️ A `YY/MM/DD/SLUG` path, emoji spellings and four-digit years accepted.
    fn ticket_id(path: &str) -> Result<semio_framework_repo_tickets::TicketId, HandlerError> {
        let mut id = semio_framework_repo_tickets::TicketId::parse(path).map_err(|error| HandlerError::tool(&error.message))?;
        if id.year >= 2000 {
            id.year %= 100;
        }
        Ok(id)
    }

    /// 🧰️ A tool result carrying the markdown of one entity payload.
    fn entity_result(key: &str, entity: Json, extra: &[String]) -> RepositoryResult {
        let mut data = Map::new();
        data.insert(key.to_string(), entity);
        let mut lines = vec![crate::repo_cli::format_markdown_result("graphql", &Json::Object(data))];
        lines.extend(extra.iter().cloned());
        RepositoryResult { text: lines.join("\n"), structured: None, is_error: false }
    }

    /// 🧰️ A tool result carrying plain lines.
    fn lines_result(lines: Vec<String>) -> RepositoryResult {
        RepositoryResult { text: lines.join("\n"), structured: None, is_error: false }
    }

    /// 🎫️ The id and uri a ticket record answers with.
    fn ticket_identity(slug: &str) -> (String, String) {
        let id = semio_framework_repo_identity::emoji_text(semio_framework_repo_identity::entity("ticket")) + &semio_framework_repo_identity::flat(slug);
        let uri = format!("repo://ticket/{id}");
        (id, uri)
    }

    impl RepoRepository {
        fn ticket_open(&self, arguments: &Json) -> Result<RepositoryResult, HandlerError> {
            let kind = self.kind();
            let mut client = optional(arguments, "client")?;
            if client.trim().is_empty() {
                client = semio_framework_repo_providers::hook_client_for_mcp_kind(kind).to_string();
            }
            let request = semio_framework_repo_tickets::TicketOpenRequest {
                emoji: optional(arguments, "emoji")?,
                title: optional(arguments, "title")?,
                prompt: optional(arguments, "prompt")?,
                llm: optional(arguments, "llm")?,
                effort: optional(arguments, "effort")?,
                client,
                goal: optional(arguments, "goal")?,
                parent: optional(arguments, "parent")?,
                no_issue: switch(arguments, "no_issue")?,
                no_management: switch(arguments, "no_management")?,
                issue: optional(arguments, "issue")?,
                session: String::new(),
                plan_id: optional(arguments, "plan_id")?,
                spec_id: optional(arguments, "spec_id")?,
            };
            let context = self.context();
            let outcome = context.with_tickets(kind, |service| service.open(&request)).map_err(|error| HandlerError::tool(&error.message))?;
            let id = semio_framework_repo_tickets::TicketId::parse(&outcome.id).map_err(|error| HandlerError::tool(&error.message))?;
            let ticket = context.with_tickets(kind, |service| service.read(&id)).map_err(|error| HandlerError::tool(&error.message))?;
            let (artifact, uri) = ticket_identity(&ticket.slug);
            let mut payload = json!({ "id": artifact, "slug": ticket.slug, "year": ticket.year, "month": ticket.month, "day": ticket.day, "status": ticket.status, "path": ticket.folder_path, "uri": uri });
            let mut extra = outcome.warnings.clone();
            if let Some(issue) = ticket.management.as_ref().map(|management| management.issue.clone()).filter(|issue| !issue.is_empty()) {
                payload["github"] = json!({ "issue": issue });
                extra.push(format!("GitHub issue: {issue}"));
            }
            Ok(entity_result("ticketOpen", payload, &extra))
        }

        fn ticket_close(&self, arguments: &Json) -> Result<RepositoryResult, HandlerError> {
            let path = optional(arguments, "path")?;
            let context = self.context();
            let id = if path.is_empty() {
                let open = context.tickets(None, None, None, Some(TicketStatus::Open)).map_err(|error| HandlerError::tool(&error.message))?;
                let latest = open.into_iter().max_by(|left, right| (left.year, left.month, left.day, &left.slug).cmp(&(right.year, right.month, right.day, &right.slug))).ok_or_else(|| HandlerError::tool("no path provided and no open tickets found"))?;
                semio_framework_repo_tickets::TicketId::new(latest.year, latest.month, latest.day, latest.slug)
            } else {
                ticket_id(&path)?
            };
            let title = optional(arguments, "title")?;
            let input = TicketCloseInput {
                year: id.year,
                month: id.month,
                day: id.day,
                slug: id.slug,
                summary: optional(arguments, "summary")?,
                files: Some(strings(arguments, "files")?),
                title: Some(title).filter(|title| !title.is_empty()),
                no_management: switch(arguments, "no_management")?,
                all: false,
            };
            let ticket = context.ticket_close(input).map_err(|error| HandlerError::tool(&error.message))?;
            let (artifact, _) = ticket_identity(&ticket.slug);
            let finished = ticket.interactions.iter().rev().find(|interaction| interaction.kind.starts_with("ticket.close")).map(|interaction| interaction.date.clone()).unwrap_or_default();
            let started = ticket.interactions.first().map(|interaction| interaction.date.clone()).unwrap_or_default();
            Ok(entity_result("ticketClose", json!({ "id": artifact, "slug": ticket.slug, "status": ticket.status, "dates": { "created": started, "finished": finished } }), &[]))
        }

        fn ticket_reopen(&self, arguments: &Json) -> Result<RepositoryResult, HandlerError> {
            let kind = self.kind();
            let path = optional(arguments, "path")?;
            let context = self.context();
            let mut id = if path.is_empty() {
                let tickets = context.tickets(None, None, None, None).map_err(|error| HandlerError::tool(&error.message))?;
                let latest = tickets.into_iter().max_by(|left, right| (left.year, left.month, left.day, left.interactions.first().map(|interaction| interaction.date.clone())).cmp(&(right.year, right.month, right.day, right.interactions.first().map(|interaction| interaction.date.clone())))).ok_or_else(|| HandlerError::tool("no path provided and no tickets found"))?;
                semio_framework_repo_tickets::TicketId::new(latest.year, latest.month, latest.day, latest.slug)
            } else {
                ticket_id(&path)?
            };
            let no_management = switch(arguments, "no_management")?;
            let title = optional(arguments, "title")?;
            if !title.is_empty() {
                id = context.retitle(&id, Some(&title), no_management).map_err(|error| HandlerError::tool(&error.message))?;
            }
            let mut client = optional(arguments, "client")?;
            if client.trim().is_empty() {
                client = semio_framework_repo_providers::hook_client_for_mcp_kind(kind).to_string();
            }
            let request = semio_framework_repo_tickets::TicketReopenRequest {
                id: id.id(),
                prompt: optional(arguments, "prompt")?,
                llm: optional(arguments, "llm")?,
                effort: optional(arguments, "effort")?,
                client,
                goal: optional(arguments, "goal")?,
                parent: optional(arguments, "parent")?,
                no_management,
                session: String::new(),
                plan_id: optional(arguments, "plan_id")?,
                spec_id: optional(arguments, "spec_id")?,
            };
            let outcome = context.with_tickets(kind, |service| service.reopen(&request)).map_err(|error| HandlerError::tool(&error.message))?;
            let mut lines = outcome.warnings.clone();
            lines.push(format!("\n🔓️ Ticket reopened: {}", id.slug));
            Ok(lines_result(lines))
        }

        fn goal_open(&self, arguments: &Json) -> Result<RepositoryResult, HandlerError> {
            let input = GoalCreateInput {
                title: optional(arguments, "title")?,
                description: optional(arguments, "description")?,
                prompt: optional(arguments, "prompt")?,
                due_date: optional(arguments, "due_date")?,
                llm: optional(arguments, "llm")?,
                effort: String::new(),
                client: optional(arguments, "client")?,
                no_management: switch(arguments, "no_management")?,
                parent: optional(arguments, "parent")?,
                milestone: optional(arguments, "milestone")?,
            };
            let goal = self.context().goal_create(input).map_err(|error| HandlerError::tool(&error.message))?;
            Ok(entity_result("goalCreate", json!({ "id": goal.id, "title": goal.title, "status": goal.status, "prompt": goal.prompt, "dueDate": goal.dates.due, "client": goal.client, "llm": goal.llm }), &[]))
        }

        fn goal_close(&self, arguments: &Json) -> Result<RepositoryResult, HandlerError> {
            let input = GoalCloseInput { id: required(arguments, "id")?, summary: required(arguments, "summary")?, no_management: switch(arguments, "no_management")? };
            let goal = self.context().goal_close(input).map_err(|error| HandlerError::tool(&error.message))?;
            Ok(entity_result("goalClose", json!({ "id": goal.id, "status": goal.status }), &[]))
        }

        fn goal_reopen(&self, arguments: &Json) -> Result<RepositoryResult, HandlerError> {
            let some = |value: String| Some(value).filter(|value| !value.is_empty());
            let input = GoalReopenInput {
                id: required(arguments, "id")?,
                prompt: required(arguments, "prompt")?,
                llm: required(arguments, "llm")?,
                client: required(arguments, "client")?,
                effort: String::new(),
                title: some(optional(arguments, "title")?),
                description: some(optional(arguments, "description")?),
                due_date: some(optional(arguments, "due_date")?),
                parent: None,
                no_management: switch(arguments, "no_management")?,
            };
            let goal = self.context().goal_reopen(input).map_err(|error| HandlerError::tool(&error.message))?;
            Ok(entity_result("goalReopen", json!({ "id": goal.id, "status": goal.status }), &[]))
        }
    }

    impl Repository for RepoRepository {
        fn call(&self, _context: &Context, name: &str, arguments: &Json) -> Result<RepositoryResult, HandlerError> {
            let context = self.context();
            let failed = |error: semio_framework_repo_graphql::ContextError| HandlerError::tool(&error.message);
            match name {
                "ticket_open" => self.ticket_open(arguments),
                "ticket_close" => self.ticket_close(arguments),
                "ticket_reopen" => self.ticket_reopen(arguments),
                "goal_open" => self.goal_open(arguments),
                "goal_close" => self.goal_close(arguments),
                "goal_reopen" => self.goal_reopen(arguments),
                "section_move" => context.move_section(&self.relative(&required(arguments, "file")?), &required(arguments, "old_name")?, &required(arguments, "new_name")?).map(lines_result).map_err(failed),
                "file_integrate" => context
                    .integrate_file(&self.relative(&required(arguments, "source")?), &required(arguments, "target_section")?, &self.relative(&required(arguments, "target_file")?), &optional(arguments, "target_parent_section")?)
                    .map(lines_result)
                    .map_err(failed),
                "section_extract" => context.extract_section(&self.relative(&required(arguments, "source_file")?), &required(arguments, "source_section")?, &self.relative(&required(arguments, "target_file")?)).map(lines_result).map_err(failed),
                _ => Err(HandlerError::tool("tool not found")),
            }
        }

        fn read(&self, _context: &Context, uri: &str) -> Result<ResourceContent, HandlerError> {
            let text = match uri {
                "repo://" => self.yaml_of_query("query Repo { repo { id name bundles { id } tickets { id } policies { id } contributors { id } } }")?,
                "repo://bundles" => self.yaml_of_query("query Bundles { repo { bundles { id name root sourceRoot projectType tags kind } } }")?,
                "repo://folders" => self.yaml_of_query("query Folders { repo { folders { id path name kind } } }")?,
                "repo://files" => self.yaml_of_query("query Files { repo { files { id path name kind extension } } }")?,
                "repo://tickets" => self.yaml_of_query("query Tickets { repo { tickets { id slug title status prompt interactions { prompt } } } }")?,
                "repo://policies" => self.yaml_of_query("query Policies { repo { policies { id description breachs { id } } } }")?,
                "repo://contributors" => self.yaml_of_query("query Contributors { repo { contributors { id emails name contributions { checkpoints { id } tickets { id } } } } }")?,
                "repo://goals" => {
                    let goals = self.context().goals().map_err(|error| HandlerError::tool(&error.message))?;
                    semio_framework_repo_yaml::marshal(&serde_json::to_value(goals).map_err(|error| HandlerError::tool(&error.to_string()))?).map_err(|error| HandlerError::tool(&error))?
                }
                _ => return Err(HandlerError::tool("resource not found")),
            };
            Ok(ResourceContent { uri: uri.to_string(), mime_type: "text/plain".to_string(), text, blob: String::new() })
        }

        fn prompt(&self, _context: &Context, name: &str, arguments: &BTreeMap<String, String>) -> Result<GetPromptResult, HandlerError> {
            let Some(instruction) = semio_framework_repo_mcp::prompt_instruction(name) else { return Err(HandlerError::tool("prompt not found")) };
            let given = arguments.get("prompt").cloned().unwrap_or_default();
            Ok(GetPromptResult { description: instruction.to_string(), messages: vec![("user".to_string(), Content::text(&format!("{instruction}\n\n{given}")))] })
        }
    }

    /// 🏠️ The production repository of a root, answering as the profile `SEMIO_REPO_MCP_CLIENT` names.
    pub fn real_repository(root: PathBuf) -> RepoRepository {
        let profile = Profile::parse(&std::env::var(semio_framework_repo_mcp::PROFILE_ENVIRONMENT).unwrap_or_default()).unwrap_or_default();
        RepoRepository::open(&root, profile)
    }
}

//#endregion 🔌️McpVerb
