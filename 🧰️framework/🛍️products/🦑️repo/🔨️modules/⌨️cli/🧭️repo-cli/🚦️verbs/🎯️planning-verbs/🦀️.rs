//! 🎯️ Planning verbs: todos, goals and contributors.

use super::{extract_client, extract_effort, extract_llm, flag_or_arg, missing_client, missing_llm};
use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use serde_json::{json, Value as Json};

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
