//! 🔎️ Query verbs: search, list, query, the trees and export.

use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_cli_event_stream::{go_json, Format};
use semio_framework_repo_cli_tree_source_and_search::{export_entities, flatten_tree, searched_tree, tree_filter, write_tree_forest, write_tree_node, FsTreeSource};
use semio_framework_repo_model::{entity, TreeNodeKind};
use serde_json::{json, Map, Value as Json};

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
