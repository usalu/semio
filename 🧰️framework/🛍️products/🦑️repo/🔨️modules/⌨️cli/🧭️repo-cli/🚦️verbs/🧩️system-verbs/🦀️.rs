//! 🧩️ System verbs: entity emojis, configure, micro commit, update and the auth status verbs.

use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_cli_event_stream::{go_json, go_sprint, Format, EXIT_ERROR};
use serde_json::{json, Value as Json};
use std::collections::BTreeMap;

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
