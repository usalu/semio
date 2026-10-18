//! ⚖️ Statute verbs: analyze, autofix and the statute catalog listings.

use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_cli_event_stream::{graphql_events, Event, Format, EXIT_OK, KIND_ERROR, KIND_RESULT, KIND_START};
use semio_framework_repo_cli_tree_source_and_search::{text_tree_lines, StreamOptions};
use semio_framework_repo_model::{TreeNode};
use semio_framework_repo_model_entity::entity;
use serde_json::{json, Map, Value as Json};

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
