//! 🖼️ The source shapers: domain values serialised into the JSON objects the executor reads fields off.

use super::*;

//#region 🖼️Sources

/// 🔣️ Serialises a domain value into the JSON the executor reads fields off.
pub(crate) fn to_json<T: Serialize>(value: &T) -> Json {
    serde_json::to_value(value).unwrap_or(Json::Null)
}

/// 🕳️ The object body of a serialised domain value, empty when it is not an object.
pub(crate) fn body<T: Serialize>(value: &T) -> Map<String, Json> {
    match to_json(value) {
        Json::Object(fields) => fields,
        _ => Map::new(),
    }
}

/// 🏷️ Stamps the concrete type name an interface or union member is discriminated by.
pub(crate) fn tagged(mut fields: Map<String, Json>, typename: &str) -> Json {
    fields.insert("__typename".to_string(), Json::String(typename.to_string()));
    Json::Object(fields)
}

/// 📁️ Joins a repository-relative path onto the root, always with forward slashes.
pub(crate) fn file_uri(root_dir: &str, relative: &str) -> String {
    let root = root_dir.replace('\\', "/");
    let tail = relative.replace('\\', "/");
    if tail.is_empty() {
        format!("file://{root}")
    } else {
        format!("file://{}/{}", root.trim_end_matches('/'), tail.trim_start_matches('/'))
    }
}

/// 🔤️ Title-cases one dash separated slug, word by word.
fn titleize_slug(slug: &str) -> String {
    slug.split('-')
        .map(|word| match word.chars().next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + &word[first.len_utf8()..].to_lowercase(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// 📜️ The identifier value of a statute path: every segment titleized, joined with `#`.
fn statute_path_to_id_value(path: &str) -> String {
    path.split('/').map(titleize_slug).collect::<Vec<_>>().join("#")
}

/// 🪪️ The emoji a technology name derives when the record carries none.
fn technology_emoji(name: &str) -> String {
    if name.contains("repo") {
        entity("technology-infrastructure").to_string()
    } else if name.starts_with("coda") {
        entity("technology-research").to_string()
    } else {
        String::new()
    }
}

/// 🆔️ The identifier of a technology.
pub(crate) fn technology_id(technology: &Technology) -> String {
    let emoji = if technology.emoji.is_empty() { technology_emoji(&technology.name) } else { technology.emoji.clone() };
    emoji_text(&emoji) + &flat(&technology.name)
}

/// 🔵️ The emoji a bundle kind stands for.
fn bundle_kind_emoji(kind: BundleKind) -> &'static str {
    match kind {
        BundleKind::Library => entity("bundle-library"),
        BundleKind::Schema => entity("bundle-schema"),
        BundleKind::Binary => entity("bundle-binary"),
        BundleKind::Ui => entity("bundle-ui"),
        BundleKind::Site => entity("bundle-site"),
        BundleKind::Assets => entity("bundle-assets"),
        BundleKind::Repo => entity("bundle-repo"),
    }
}

/// 🔑️ The identifier of a bundle: technology emoji and code, then bundle emoji and code.
pub(crate) fn bundle_id(bundle: &Bundle, technologies: &[Technology]) -> String {
    let emoji = if bundle.emoji.is_empty() { bundle_kind_emoji(bundle.kind).to_string() } else { bundle.emoji.clone() };
    let (technology_code, bundle_code) = match bundle.name.split_once('/') {
        Some((head, tail)) => (head.to_string(), tail.to_string()),
        None => (bundle.name.clone(), bundle.name.clone()),
    };
    let technology_emoji_value = technologies
        .iter()
        .find(|candidate| candidate.name == technology_code && !candidate.emoji.is_empty())
        .map_or_else(|| derive_technology_kind(&technology_code).as_str().to_string(), |candidate| candidate.emoji.clone());
    emoji_text(&technology_emoji_value) + &flat(&technology_code) + &emoji_text(&emoji) + &flat(&bundle_code)
}

/// 💗️ The identifier of a section, falling back to the emoji form when it carries none.
fn section_id(section: &Section) -> String {
    if !section.id.is_empty() {
        return section.id.clone();
    }
    let emoji = if section.emoji.is_empty() { entity("section").to_string() } else { section.emoji.clone() };
    emoji_text(&emoji) + &flat(&section.name)
}

/// 💕️ The emoji a definition kind stands for.
fn definition_kind_emoji(kind: DefinitionKind) -> &'static str {
    match kind {
        DefinitionKind::Implementation => entity("definition-impl"),
        DefinitionKind::Interface => entity("definition-interface"),
        DefinitionKind::Constant => entity("definition-constant"),
        DefinitionKind::Test => entity("definition-test"),
    }
}

/// ⏪️ The identifier of a definition, falling back to the emoji form when it carries none.
pub(crate) fn definition_id(definition: &Definition) -> String {
    if !definition.id.is_empty() {
        return definition.id.clone();
    }
    emoji_text(definition_kind_emoji(definition.kind)) + &flat(&definition.name)
}

/// 🕰️ Normalises a recorded timestamp into the RFC 3339 rendering the `DateTime` scalar emits.
fn to_rfc3339(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.len() >= 19 && trimmed.is_char_boundary(10) && (trimmed.as_bytes()[10] == b' ' || trimmed.as_bytes()[10] == b'T') {
        let (date, rest) = trimmed.split_at(10);
        let time = rest[1..].split(['+', 'Z', '.']).next().unwrap_or("");
        if time.len() == 8 {
            return Some(format!("{date}T{time}Z"));
        }
    }
    None
}

/// 🌙️ Midnight UTC of a year/month/day triple, the zero date a ticket falls back to.
fn midnight(year: i64, month: i64, day: i64) -> String {
    format!("{year:04}-{month:02}-{day:02}T00:00:00Z")
}

/// 🎫️ Whether an interaction kind is the expected one, tolerating the `.ended` suffix.
fn is_ticket_interaction_kind(kind: &str, expected: &str) -> bool {
    let kind = kind.trim();
    kind == expected.trim() || kind.strip_suffix(".ended").is_some_and(|head| head == expected.trim())
}

/// ▶️ The date a ticket started: its open interaction, else its first, else midnight.
pub(crate) fn ticket_started(ticket: &Ticket) -> String {
    for interaction in &ticket.interactions {
        if is_ticket_interaction_kind(&interaction.kind, "ticket.open") {
            if let Some(date) = to_rfc3339(&interaction.date) {
                return date;
            }
        }
    }
    if let Some(first) = ticket.interactions.first() {
        if let Some(date) = to_rfc3339(&first.date) {
            return date;
        }
    }
    midnight(ticket.year, ticket.month, ticket.day)
}

/// 🏁️ The date a ticket finished: its last close interaction, or none.
fn ticket_finished(ticket: &Ticket) -> Option<String> {
    ticket.interactions.iter().rev().find(|interaction| is_ticket_interaction_kind(&interaction.kind, "ticket.close")).and_then(|interaction| to_rfc3339(&interaction.date))
}

/// 🔚️ The last interaction's field, then the last agent's, then blank.
fn ticket_latest(ticket: &Ticket, pick: fn(&Interaction) -> &str, fallback: fn(&TicketAgent) -> &str) -> String {
    if let Some(interaction) = ticket.interactions.last() {
        return pick(interaction).to_string();
    }
    ticket.agents.last().map(fallback).unwrap_or_default().to_string()
}

/// ✏️ The author of a ticket: its first interaction's, then its first agent's.
pub(crate) fn ticket_author(ticket: &Ticket) -> String {
    if let Some(interaction) = ticket.interactions.first() {
        return interaction.author.clone();
    }
    ticket.agents.first().map(|agent| agent.contributor.clone()).unwrap_or_default()
}

/// 🚩️ The checkpoint of a ticket: its first interaction's.
pub(crate) fn ticket_checkpoint(ticket: &Ticket) -> String {
    ticket.interactions.first().map(|interaction| interaction.checkpoint.clone()).unwrap_or_default()
}

/// ❓️ Wraps a non-empty string as JSON, mapping the empty string to null.
fn optional(value: String) -> Json {
    if value.is_empty() {
        Json::Null
    } else {
        Json::String(value)
    }
}

/// 🤝️ Splits `Name <email>` the way the reference implementation does.
pub(crate) fn parse_git_author(value: &str) -> (String, String) {
    match value.split_once(" <") {
        Some((name, rest)) => (name.trim().to_string(), rest.trim_end_matches('>').to_string()),
        None => (value.to_string(), String::new()),
    }
}

/// 🟡️ The milestone number a management link carries.
pub fn parse_milestone_number(milestone: &str) -> Option<i64> {
    milestone.parse::<i64>().ok().or_else(|| milestone.rsplit('/').next().and_then(|tail| tail.parse::<i64>().ok()))
}

/// 🪵️ The technology source object.
pub(crate) fn technology_source(technology: &Technology, technologies: &[Technology], root_dir: &str) -> Json {
    let mut fields = body(technology);
    let id = technology_id(technology);
    fields.insert("uri".to_string(), Json::String(format!("repo://technology/{id}")));
    fields.insert("id".to_string(), Json::String(id));
    fields.insert("kind".to_string(), Json::String(technology.kind.as_str().to_string()));
    let bundles = technology.bundles.clone().unwrap_or_default();
    fields.insert("bundles".to_string(), Json::Array(bundles.iter().map(|bundle| bundle_source(bundle, technologies, root_dir)).collect()));
    tagged(fields, "Technology")
}

/// 🧰️ The bundle source object.
pub(crate) fn bundle_source(bundle: &Bundle, technologies: &[Technology], root_dir: &str) -> Json {
    let mut fields = body(bundle);
    fields.insert("id".to_string(), Json::String(bundle_id(bundle, technologies)));
    fields.insert("uri".to_string(), Json::String(file_uri(root_dir, &bundle.root)));
    fields.insert("kind".to_string(), Json::String(bundle.kind.as_str().to_string()));
    fields.insert("tags".to_string(), Json::Array(bundle.tags.iter().cloned().map(Json::String).collect()));
    fields.insert("packages".to_string(), Json::Array(bundle.packages.iter().map(|package| tagged(body(package), "Package")).collect()));
    tagged(fields, "Bundle")
}

/// 🩷️ The folder source object.
pub(crate) fn folder_source(folder: &Folder) -> Json {
    let mut fields = body(folder);
    fields.insert("kind".to_string(), Json::String(folder.kind.as_str().to_string()));
    tagged(fields, "Folder")
}

/// 📄️ The file source object.
pub(crate) fn file_source(file: &File) -> Json {
    tagged(body(file), "File")
}

/// 📐️ The line range of a start/end pair.
fn range_source(start: i64, end: i64) -> Json {
    tagged(body(&Range { start, end }), "Range")
}

/// 🌳️ The section source object, recursing through children and definitions.
pub(crate) fn section_source(section: &Section) -> Json {
    let mut fields = body(section);
    fields.insert("id".to_string(), Json::String(section_id(section)));
    fields.insert("range".to_string(), range_source(section.start_line, section.end_line));
    fields.insert("children".to_string(), Json::Array(section.children.iter().map(section_source).collect()));
    fields.insert("definitions".to_string(), Json::Array(section.definitions.iter().map(definition_source).collect()));
    tagged(fields, "Section")
}

/// 📋️ The definition source object.
pub(crate) fn definition_source(definition: &Definition) -> Json {
    let mut fields = body(definition);
    fields.insert("id".to_string(), Json::String(definition_id(definition)));
    fields.insert("kind".to_string(), Json::String(definition.kind.as_str().to_string()));
    fields.insert("range".to_string(), range_source(definition.start_line, definition.end_line));
    tagged(fields, "Definition")
}

/// ⚖️ The declared metadata of one statute, or the reference implementation's unknown default.
pub(crate) fn statute_meta_of(kind: &Statute, statutes: &[StatuteMeta]) -> StatuteMeta {
    statutes.iter().find(|candidate| candidate.kind == *kind).cloned().unwrap_or(StatuteMeta {
        kind: kind.clone(),
        policy_id: String::new(),
        priority: BreachPriority::Low,
        reason: "Unknown breach".to_string(),
        solution: "Fix the breach".to_string(),
        autofixable: false,
    })
}

/// 📚️ The statute source object.
pub(crate) fn statute_source(meta: &StatuteMeta) -> Json {
    let mut fields = body(meta);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("statute")) + &statute_path_to_id_value(&meta.kind.0)));
    fields.insert("priority".to_string(), Json::String(meta.priority.as_str().to_string()));
    tagged(fields, "Statute")
}

/// 🔶️ The breach source object, with the statute metadata overlaid by the breach's own overrides.
pub(crate) fn breach_source(breach: &Breach, statutes: &[StatuteMeta]) -> Json {
    let mut meta = statute_meta_of(&breach.kind, statutes);
    if let Some(priority) = breach.lint_priority {
        meta.priority = priority;
    }
    if let Some(autofixable) = breach.lint_autofixable {
        meta.autofixable = autofixable;
    }
    if !breach.reason.is_empty() {
        meta.reason = breach.reason.clone();
    }
    if !breach.solution.is_empty() {
        meta.solution = breach.solution.clone();
    }
    let mut fields = body(breach);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("breach")) + &breach.id));
    fields.insert("kindId".to_string(), Json::String(breach.kind.0.clone()));
    fields.insert("kind".to_string(), statute_source(&meta));
    fields.insert("priority".to_string(), Json::String(meta.priority.as_str().to_string()));
    fields.insert("autofixable".to_string(), Json::Bool(meta.autofixable));
    fields.insert("line".to_string(), Json::from(breach.line));
    fields.insert("column".to_string(), Json::from(breach.column));
    fields.insert("excerpt".to_string(), optional(breach.excerpt.clone()));
    tagged(fields, "Breach")
}

/// 🟣️ The territory source object.
fn territory_source(territory: &Territory, statutes: &[StatuteMeta]) -> Json {
    let mut fields = body(territory);
    fields.insert("groups".to_string(), Json::Array(territory.groups.iter().map(|group| territory_source(group, statutes)).collect()));
    fields.insert("kinds".to_string(), Json::Array(territory.kinds.iter().map(|kind| statute_source(&statute_meta_of(kind, statutes))).collect()));
    tagged(fields, "Territory")
}

/// 👮️ The policy source object.
pub(crate) fn policy_source(policy: &Policy, statutes: &[StatuteMeta]) -> Json {
    let mut fields = body(policy);
    fields.insert("scopes".to_string(), Json::Array(policy.scopes.clone().unwrap_or_default().into_iter().map(Json::String).collect()));
    fields.insert("groups".to_string(), Json::Array(policy.groups.clone().unwrap_or_default().iter().map(|group| territory_source(group, statutes)).collect()));
    fields.insert("statutes".to_string(), Json::Array(policy.statutes.clone().unwrap_or_default().iter().map(statute_source).collect()));
    tagged(fields, "Policy")
}

/// 💬️ The interaction source object.
fn interaction_source(interaction: &Interaction) -> Json {
    let mut fields = body(interaction);
    for (key, value) in [("prompt", &interaction.prompt), ("summary", &interaction.summary), ("llm", &interaction.llm), ("effort", &interaction.effort)] {
        fields.insert(key.to_string(), Json::String(value.clone()));
    }
    tagged(fields, "Interaction")
}

/// 🎁️ The flattened interaction-resource source object.
pub(crate) fn interaction_resource_source(resource: &InteractionResource) -> Json {
    let Json::Object(mut fields) = interaction_source(&resource.interaction) else { return Json::Null };
    for (key, value) in [("goalId", &resource.goal_id), ("ticketId", &resource.ticket_id)] {
        fields.insert(key.to_string(), Json::String(value.clone()));
    }
    fields.insert("sourceKind".to_string(), Json::String(resource.source_kind.clone()));
    fields.insert("sourceId".to_string(), Json::String(resource.source_id.clone()));
    tagged(fields, "InteractionResource")
}

/// 🧬️ The ticket source object, with every derived field the reference resolvers compute.
pub(crate) fn ticket_source(ticket: &Ticket, root_dir: &str) -> Json {
    let mut fields = body(ticket);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("ticket")) + &flat(&ticket.slug)));
    fields.insert("year".to_string(), Json::from(ticket.year));
    fields.insert("month".to_string(), Json::from(ticket.month));
    fields.insert("day".to_string(), Json::from(ticket.day));
    fields.insert("slug".to_string(), Json::String(ticket.slug.clone()));
    let path = if ticket.folder_path.is_empty() { ticket.json_path.clone() } else { ticket.folder_path.clone() };
    fields.insert("path".to_string(), Json::String(path));
    fields.insert("uri".to_string(), Json::String(file_uri(root_dir, &ticket.folder_path)));
    fields.insert("title".to_string(), Json::String(ticket.title.clone()));
    fields.insert("emoji".to_string(), optional(ticket.emoji.clone()));
    let prompt = if ticket.description.is_empty() { ticket.interactions.first().map(|interaction| interaction.prompt.clone()).unwrap_or_default() } else { ticket.description.clone() };
    fields.insert("prompt".to_string(), Json::String(prompt));
    fields.insert("summary".to_string(), Json::Null);
    fields.insert("status".to_string(), Json::String(ticket.status.as_str().to_string()));
    fields.insert("llm".to_string(), optional(ticket_latest(ticket, |interaction| &interaction.llm, |agent| &agent.llm)));
    fields.insert("effort".to_string(), optional(ticket_latest(ticket, |interaction| &interaction.effort, |agent| &agent.effort)));
    fields.insert("client".to_string(), optional(ticket_latest(ticket, |interaction| &interaction.client, |agent| &agent.client)));
    fields.insert("checkpoint".to_string(), optional(ticket_checkpoint(ticket)));
    fields.insert("goal".to_string(), optional(ticket.goal.clone()));
    fields.insert("parent".to_string(), optional(ticket.parent.clone()));
    fields.insert("interactions".to_string(), Json::Array(ticket.interactions.iter().map(interaction_source).collect()));
    let mut dates = Map::new();
    dates.insert("started".to_string(), Json::String(ticket_started(ticket)));
    dates.insert("finished".to_string(), ticket_finished(ticket).map_or(Json::Null, Json::String));
    fields.insert("dates".to_string(), tagged(dates, "TicketDate"));
    tagged(fields, "Ticket")
}

/// 🎯️ The goal source object.
pub(crate) fn goal_source(goal: &Goal) -> Json {
    let mut fields = body(goal);
    fields.insert("id".to_string(), Json::String(goal.id.clone()));
    fields.insert("dueDate".to_string(), optional(goal.dates.due.clone()));
    fields.insert("createdAt".to_string(), Json::Null);
    fields.insert("status".to_string(), Json::String(goal.status.as_str().to_string()));
    let management = goal.management.clone().unwrap_or_default();
    fields.insert("milestone".to_string(), parse_milestone_number(&management.milestone).map_or(Json::Null, Json::from));
    fields.insert("issue".to_string(), optional(management.issue));
    fields.insert("parent".to_string(), optional(goal.parent.clone()));
    fields.insert("interactions".to_string(), Json::Array(Vec::new()));
    tagged(fields, "Goal")
}

/// 📝️ The draft source object.
pub(crate) fn draft_source(draft: &Draft) -> Json {
    tagged(body(draft), "Draft")
}

/// ✅️ The todo source object.
pub(crate) fn todo_source(todo: &Todo) -> Json {
    let mut fields = body(todo);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("todo")) + &flat(&todo.id)));
    fields.insert("description".to_string(), optional(todo.description.clone()));
    fields.insert("location".to_string(), todo.location.as_ref().map_or(Json::Null, |location| tagged(body(location), "Location")));
    tagged(fields, "Todo")
}

/// ✔️ The checkpoint source object.
pub(crate) fn checkpoint_source(checkpoint: &Checkpoint) -> Json {
    tagged(body(checkpoint), "Checkpoint")
}

/// 🧑️‍💻️ The contributor source object.
pub(crate) fn contributor_source(contributor: &Contributor) -> Json {
    let mut fields = body(contributor);
    fields.insert("id".to_string(), Json::String(emoji_text(entity("contributor")) + &flat(&contributor.alias)));
    let links = contributor.links.iter().map(|(name, url)| tagged(body(&ContributorLink { name: name.clone(), url: url.clone() }), "ContributorLink")).collect();
    fields.insert("links".to_string(), Json::Array(links));
    fields.insert("emoji".to_string(), optional(contributor.emoji.clone()));
    fields.insert("fingerprint".to_string(), optional(contributor.fingerprint.clone()));
    for (key, values) in [("names", &contributor.names), ("emails", &contributor.emails), ("fingerprints", &contributor.fingerprints), ("githubs", &contributor.githubs), ("aliases", &contributor.aliases)] {
        fields.insert(key.to_string(), Json::Array(values.iter().cloned().map(Json::String).collect()));
    }
    fields.insert("icons".to_string(), Json::Null);
    tagged(fields, "Contributor")
}

/// 🔬️ The analyze-result source object.
pub(crate) fn analyze_result_source(result: &AnalyzeResult, statutes: &[StatuteMeta]) -> Json {
    let mut fields = Map::new();
    fields.insert("breachs".to_string(), Json::Array(result.breachs.clone().unwrap_or_default().iter().map(|breach| breach_source(breach, statutes)).collect()));
    let metrics = result.metrics.clone().unwrap_or(AnalyzeMetrics { total: 0, by_priority: None, autofixable: 0 });
    let mut metric_fields = Map::new();
    metric_fields.insert("total".to_string(), Json::from(metrics.total));
    metric_fields.insert("autofixable".to_string(), Json::from(metrics.autofixable));
    metric_fields.insert("byPriority".to_string(), tagged(body(&metrics.by_priority.unwrap_or_default()), "PriorityCount"));
    fields.insert("metrics".to_string(), tagged(metric_fields, "AnalyzeMetrics"));
    tagged(fields, "AnalyzeResult")
}

/// 🩹️ The fix-result source object.
pub(crate) fn fix_result_source(result: &FixResult, statutes: &[StatuteMeta]) -> Json {
    let mut fields = Map::new();
    fields.insert("fixed".to_string(), Json::from(result.fixed));
    fields.insert("remaining".to_string(), Json::from(result.remaining));
    fields.insert("breachs".to_string(), Json::Array(result.breachs.clone().unwrap_or_default().iter().map(|breach| breach_source(breach, statutes)).collect()));
    tagged(fields, "FixResult")
}

//#endregion 🖼️Sources
