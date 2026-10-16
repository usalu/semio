//! 🔌️ Gateway verbs: `mcp`, `graphql`, `test` and `hook`, the doors to the other front ends.

use super::flag_or_arg;
use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_cli_event_stream::Format;
use semio_framework_repo_cli_mcp_verb::RepoRepository;
use semio_framework_repo_codebase::Codebase;
use serde_json::{Map, Value as Json};
use std::collections::BTreeMap;

pub fn mcp(invocation: &mut Invocation<'_>) -> Result<(), CliError> {
    if invocation.flag("dry-run") {
        return Ok(());
    }
    let raw = if invocation.args().is_empty() { std::env::var(semio_framework_repo_mcp::PROFILE_ENVIRONMENT).unwrap_or_default() } else { invocation.arg(0) };
    let profile = semio_framework_repo_mcp::Profile::parse(&raw).map_err(|error| CliError::Message(error.to_string()))?;
    semio_framework_repo_mcp::serve_stdio(RepoRepository::open(&invocation.root(), profile), profile).map_err(|error| CliError::Message(error.to_string()))
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
