//! 📄️ The Go-compatible JSON writer and the ticket document codec.

use super::*;

//#region 🔤️GoJson

/// 🔤️ Encodes one string the way Go's `encoding/json` does, HTML escaping included.
///
/// Divergence note: `<`, `>` and `&` become `\u003c`, `\u003e` and `\u0026` because Go's encoder
/// sets `SetEscapeHTML(true)` by default, which is observable in the committed documents — see
/// `🎆️25/🌙️11/☀️24/LOG-SYSTEM/🎫️ticket.json`, whose title is stored as `"\u003ee-"`.
pub fn go_json_string(value: &str) -> String {
    let mut text = String::with_capacity(value.len() + 2);
    text.push('"');
    for character in value.chars() {
        match character {
            '"' => text.push_str("\\\""),
            '\\' => text.push_str("\\\\"),
            '\n' => text.push_str("\\n"),
            '\r' => text.push_str("\\r"),
            '\t' => text.push_str("\\t"),
            '<' => text.push_str("\\u003c"),
            '>' => text.push_str("\\u003e"),
            '&' => text.push_str("\\u0026"),
            '\u{2028}' => text.push_str("\\u2028"),
            '\u{2029}' => text.push_str("\\u2029"),
            control if (control as u32) < 0x20 => text.push_str(&format!("\\u{:04x}", control as u32)),
            other => text.push(other),
        }
    }
    text.push('"');
    text
}

/// 🧱️ One member of a Go-shaped JSON object, in declaration order.
enum GoMember {
    Text(&'static str, String),
    Object(&'static str, Vec<GoMember>),
    Strings(&'static str, Vec<String>),
    Objects(&'static str, Vec<Vec<GoMember>>),
}

/// 🖨️ Renders members the way `json.MarshalIndent(value, "", "  ")` renders a struct.
fn render_members(members: &[GoMember], depth: usize) -> String {
    if members.is_empty() {
        return "{}".to_string();
    }
    let indent = "  ".repeat(depth + 1);
    let closing = "  ".repeat(depth);
    let mut lines: Vec<String> = Vec::with_capacity(members.len());
    for member in members {
        match member {
            GoMember::Text(name, value) => lines.push(format!("{indent}{}: {}", go_json_string(name), go_json_string(value))),
            GoMember::Object(name, nested) => lines.push(format!("{indent}{}: {}", go_json_string(name), render_members(nested, depth + 1))),
            GoMember::Objects(name, values) => {
                if values.is_empty() {
                    lines.push(format!("{indent}{}: []", go_json_string(name)));
                } else {
                    let element_indent = "  ".repeat(depth + 2);
                    let elements: Vec<String> = values.iter().map(|value| format!("{element_indent}{}", render_members(value, depth + 2))).collect();
                    lines.push(format!("{indent}{}: [\n{}\n{indent}]", go_json_string(name), elements.join(",\n")));
                }
            }
            GoMember::Strings(name, values) => {
                if values.is_empty() {
                    lines.push(format!("{indent}{}: []", go_json_string(name)));
                } else {
                    let element_indent = "  ".repeat(depth + 2);
                    let elements: Vec<String> = values.iter().map(|value| format!("{element_indent}{}", go_json_string(value))).collect();
                    lines.push(format!("{indent}{}: [\n{}\n{indent}]", go_json_string(name), elements.join(",\n")));
                }
            }
        }
    }
    format!("{{\n{}\n{closing}}}", lines.join(",\n"))
}

//#endregion 🔤️GoJson

//#region 📄️Codec

/// 🧺️ Decodes every element of an array member, skipping anything that is not an object.
fn decode_array<T>(value: Option<&Value>, decode: fn(&serde_json::Map<String, Value>) -> T) -> Vec<T> {
    let Some(Value::Array(entries)) = value else { return Vec::new() };
    entries.iter().filter_map(Value::as_object).map(decode).collect()
}

/// 🔤️ A text member, or the empty string when it is absent or not a string. This is Go's zero value.
pub(super) fn text_of(members: &serde_json::Map<String, Value>, name: &str) -> String {
    members.get(name).and_then(Value::as_str).unwrap_or_default().to_string()
}

/// 🟫️ Decodes one interaction leniently: every absent member takes its zero value, as in Go.
fn decode_interaction(members: &serde_json::Map<String, Value>) -> Interaction {
    Interaction {
        kind: text_of(members, "kind"),
        date: text_of(members, "date"),
        author: text_of(members, "author"),
        system: text_of(members, "system"),
        client: text_of(members, "client"),
        checkpoint: text_of(members, "checkpoint"),
        prompt: text_of(members, "prompt"),
        summary: text_of(members, "summary"),
        llm: text_of(members, "llm"),
        effort: text_of(members, "effort"),
        files: decode_array(members.get("files"), |file| InteractionFile { path: text_of(file, "path"), id: text_of(file, "id"), uri: text_of(file, "uri") }),
    }
}

/// 🔳️ Decodes one agent record leniently, plan included only when it decodes cleanly.
fn decode_agent(members: &serde_json::Map<String, Value>) -> TicketAgent {
    TicketAgent {
        session: text_of(members, "session"),
        contributor: text_of(members, "contributor"),
        system: text_of(members, "system"),
        client: text_of(members, "client"),
        llm: text_of(members, "llm"),
        effort: text_of(members, "effort"),
        transcript: text_of(members, "transcript"),
        plan: members.get("plan").and_then(|value| serde_json::from_value(value.clone()).ok()),
    }
}

/// 📖️ Decodes one `🎫️ticket.json` document.
///
/// The status member is mandatory and closed: a document without one, or with any spelling other
/// than `open`/`closed`, is refused rather than defaulted. Three pre-schema spellings are absorbed:
/// a `prompt` member fills an absent `description`, a `sessions` array of agent objects contributes
/// its session ids, and a closing interaction's summary fills an absent `summary`.
///
/// Members the shape does not declare (`note`, `closedAt`, `files`, `contributors`, …) are read and
/// dropped; re-encoding a document therefore loses them. That is the Go behaviour verbatim and it is
/// exercised by `🧪️tests/🎫️ticket-document-codec`.
pub fn decode_ticket_document(text: &str) -> TicketResult<Ticket> {
    let value: Value = serde_json::from_str(text).map_err(|error| TicketError::invalid(error.to_string()))?;
    let Value::Object(members) = value else {
        return Err(TicketError::invalid("ticket document must be a JSON object"));
    };
    let text_member = |name: &str| -> String { members.get(name).and_then(Value::as_str).unwrap_or_default().to_string() };
    let status = match members.get("status").and_then(Value::as_str) {
        Some("open") => TicketStatus::Open,
        Some("closed") => TicketStatus::Closed,
        _ => return Err(TicketError::invalid("ticket status must be explicitly \"open\" or \"closed\"")),
    };
    let management = match members.get("github") {
        Some(Value::Object(_)) => Some(TicketManagementData { issue: members["github"].get("issue").and_then(Value::as_str).unwrap_or_default().to_string() }),
        _ => None,
    };
    let plan = match members.get("plan") {
        Some(value @ Value::Object(_)) => Some(TicketPlan {
            client: value.get("client").and_then(Value::as_str).unwrap_or_default().to_string(),
            id: value.get("id").and_then(Value::as_str).unwrap_or_default().to_string(),
            source: value.get("source").and_then(Value::as_str).unwrap_or_default().to_string(),
            local: value.get("local").and_then(Value::as_str).unwrap_or_default().to_string(),
        }),
        _ => None,
    };
    let interactions: Vec<Interaction> = decode_array(members.get("interactions"), decode_interaction);
    let agents: Vec<TicketAgent> = decode_array(members.get("agents"), decode_agent);

    let mut ticket = Ticket {
        year: 0,
        month: 0,
        day: 0,
        slug: String::new(),
        title: text_member("title"),
        emoji: text_member("emoji"),
        status,
        description: text_member("description"),
        summary: text_member("summary"),
        management,
        goal: text_member("goal"),
        parent: text_member("parent"),
        plan,
        sessions: Vec::new(),
        interactions,
        agents,
        folder_path: String::new(),
        json_path: String::new(),
        important_path: String::new(),
    };
    if let Some(Value::Array(entries)) = members.get("sessions") {
        for entry in entries.iter().filter_map(Value::as_str) {
            append_session_id(&mut ticket, entry);
        }
    }
    if ticket.summary.is_empty() {
        for interaction in ticket.interactions.iter().rev() {
            if is_ticket_interaction_kind(&interaction.kind, "ticket.close") && !interaction.summary.is_empty() {
                ticket.summary = interaction.summary.clone();
                break;
            }
        }
    }
    let agent_sessions: Vec<String> = ticket.agents.iter().map(|agent| agent.session.clone()).collect();
    for session in agent_sessions {
        append_session_id(&mut ticket, &session);
    }
    Ok(ticket)
}

/// 🖨️ Encodes the durable ticket fields in canonical declaration order.
pub fn encode_ticket_document(ticket: &Ticket) -> String {
    let mut members: Vec<GoMember> = Vec::with_capacity(12);
    members.push(GoMember::Text("title", ticket.title.clone()));
    if !ticket.emoji.is_empty() {
        members.push(GoMember::Text("emoji", ticket.emoji.clone()));
    }
    members.push(GoMember::Text("status", ticket.status.as_str().to_string()));
    if !ticket.description.is_empty() {
        members.push(GoMember::Text("description", ticket.description.clone()));
    }
    if !ticket.summary.is_empty() {
        members.push(GoMember::Text("summary", ticket.summary.clone()));
    }
    if let Some(management) = &ticket.management {
        let mut nested: Vec<GoMember> = Vec::with_capacity(1);
        if !management.issue.is_empty() {
            nested.push(GoMember::Text("issue", management.issue.clone()));
        }
        members.push(GoMember::Object("github", nested));
    }
    if !ticket.goal.is_empty() {
        members.push(GoMember::Text("goal", ticket.goal.clone()));
    }
    if !ticket.parent.is_empty() {
        members.push(GoMember::Text("parent", ticket.parent.clone()));
    }
    if let Some(plan) = &ticket.plan {
        let mut nested: Vec<GoMember> = Vec::with_capacity(4);
        if !plan.client.is_empty() {
            nested.push(GoMember::Text("client", plan.client.clone()));
        }
        if !plan.id.is_empty() {
            nested.push(GoMember::Text("id", plan.id.clone()));
        }
        if !plan.source.is_empty() {
            nested.push(GoMember::Text("source", plan.source.clone()));
        }
        if !plan.local.is_empty() {
            nested.push(GoMember::Text("local", plan.local.clone()));
        }
        members.push(GoMember::Object("plan", nested));
    }
    if !ticket.sessions.is_empty() {
        members.push(GoMember::Strings("sessions", ticket.sessions.clone()));
    }
    if !ticket.interactions.is_empty() {
        members.push(GoMember::Objects("interactions", ticket.interactions.iter().map(interaction_members).collect()));
    }
    if !ticket.agents.is_empty() {
        members.push(GoMember::Objects("agents", ticket.agents.iter().map(agent_members).collect()));
    }
    render_members(&members, 0)
}

/// 🧵️ Appends nonempty text fields without changing declaration order.
fn optional_text_members(members: &mut Vec<GoMember>, fields: &[(&'static str, &str)]) {
    members.extend(fields.iter().filter(|(_, value)| !value.is_empty()).map(|(name, value)| GoMember::Text(name, (*value).to_string())));
}

/// 🧾️ Encodes ordered durable interactions and their file identities.
fn interaction_members(interaction: &Interaction) -> Vec<GoMember> {
    let mut members = vec![GoMember::Text("kind", interaction.kind.clone()), GoMember::Text("date", interaction.date.clone()), GoMember::Text("author", interaction.author.clone()), GoMember::Text("system", interaction.system.clone()), GoMember::Text("client", interaction.client.clone()), GoMember::Text("checkpoint", interaction.checkpoint.clone())];
    optional_text_members(&mut members, &[("prompt", &interaction.prompt), ("summary", &interaction.summary), ("llm", &interaction.llm), ("effort", &interaction.effort)]);
    if !interaction.files.is_empty() {
        let files = interaction.files.iter().map(|file| {
            let mut fields = vec![GoMember::Text("path", file.path.clone())];
            optional_text_members(&mut fields, &[("id", &file.id), ("uri", &file.uri)]);
            fields
        }).collect();
        members.push(GoMember::Objects("files", files));
    }
    members
}

/// 🤖️ Encodes ordered agent metadata and its nested plan without losing empty plan objects.
fn agent_members(agent: &TicketAgent) -> Vec<GoMember> {
    let mut members = vec![GoMember::Text("session", agent.session.clone())];
    optional_text_members(&mut members, &[("contributor", &agent.contributor), ("system", &agent.system), ("client", &agent.client), ("llm", &agent.llm), ("effort", &agent.effort), ("transcript", &agent.transcript)]);
    if let Some(plan) = &agent.plan {
        let mut fields = Vec::new();
        if !plan.steps.is_empty() {
            let steps = plan.steps.iter().map(|step| {
                let mut fields = vec![GoMember::Text("id", step.id.clone()), GoMember::Text("name", step.name.clone())];
                optional_text_members(&mut fields, &[("description", &step.description), ("status", &step.status), ("ideated", &step.ideated), ("started", &step.started), ("completed", &step.completed), ("abandoned", &step.abandoned)]);
                fields
            }).collect();
            fields.push(GoMember::Objects("steps", steps));
        }
        members.push(GoMember::Object("plan", fields));
    }
    members
}

/// ➕️ Appends a session id once, ignoring blanks and duplicates.
pub fn append_session_id(ticket: &mut Ticket, session_id: &str) {
    let session_id = session_id.trim();
    if session_id.is_empty() || ticket.sessions.iter().any(|existing| existing == session_id) {
        return;
    }
    ticket.sessions.push(session_id.to_string());
}

/// 🔍️ Whether an interaction kind names an operation, with or without its `.ended` suffix.
pub fn is_ticket_interaction_kind(kind: &str, expected: &str) -> bool {
    let kind = kind.trim();
    let expected = expected.trim();
    kind == expected || kind.strip_suffix(".ended").is_some_and(|stem| stem == expected)
}

//#endregion 📄️Codec
