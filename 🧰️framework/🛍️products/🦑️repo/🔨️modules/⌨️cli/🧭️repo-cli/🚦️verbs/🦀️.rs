//! 🚦️ One handler per verb, each a thin front for the domain crate that owns the aggregate.

use super::*;

//#region 🧭️RepoCli
//#region 🚦️Verbs


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

//#endregion 🚦️Verbs
//#endregion 🧭️RepoCli
