//! 🎫️ Ticket verbs: open, close, reopen, change, purge, list, show and files.

use super::{client_enum, extract_client, extract_effort, extract_llm, missing_client, ticket_path};
use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use serde_json::{json, Map, Value as Json};

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

/// 🧹️ Plans, and only with `--apply` performs, the removal of oversized `🗑️generated` output of closed tickets.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json (PurgePolicy)
pub fn ticket_purge_artifacts(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    let context = invocation.context();
    let apply = invocation.flag("apply");
    let options = semio_framework_repo_tickets::PurgeOptions {
        root: context.root().to_path_buf(),
        tickets_dir: semio_framework_repo_workspace::tickets_dir_for_root(context.root()),
        selection: semio_framework_repo_tickets::PurgeSelection::Oversized,
        apply,
        now: std::time::SystemTime::now(),
        tracked: &semio_framework_repo_tickets::GitTrackedFiles,
    };
    let plan = if invocation.args().is_empty() {
        semio_framework_repo_tickets::purge_closed_ticket_generated_output(&options)
    } else {
        let id = semio_framework_repo_tickets::TicketId::parse(&invocation.arg(0)).map_err(|error| CliError::Message(error.message))?;
        let ticket = context.with_tickets(semio_framework_repo_providers::McpClientKind::Generic, |service| service.read(&id)).map_err(|error| CliError::Message(error.message))?;
        semio_framework_repo_tickets::purge_ticket_generated_output(&options, &[std::path::PathBuf::from(&ticket.folder_path)])
    }
    .map_err(CliError::Message)?;
    let freed: u64 = plan.removals.iter().map(|removal| removal.bytes).sum();
    for removal in &plan.removals {
        invocation.println(&format!("{} {} ({} bytes)", if apply { "removed / entfernt" } else { "would remove / würde entfernen" }, removal.path, removal.bytes));
    }
    for skip in &plan.skips {
        invocation.println(&format!("skipped / übersprungen {}: {}", skip.ticket, skip.reason));
    }
    invocation.println(&if apply {
        format!("Removed {} paths ({freed} bytes) / {} Pfade entfernt ({freed} Bytes)", plan.removals.len(), plan.removals.len())
    } else {
        format!("Dry run: {} paths ({freed} bytes) would be removed; repeat with --apply / Probelauf: {} Pfade ({freed} Bytes) würden entfernt; mit --apply wiederholen", plan.removals.len(), plan.removals.len())
    });
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
