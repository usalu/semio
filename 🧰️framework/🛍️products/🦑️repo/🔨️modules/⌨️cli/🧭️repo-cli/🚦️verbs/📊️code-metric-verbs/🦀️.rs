//! 📊️ Code metric verbs: the mermaid charts, `loc`, technologies, bundles and `benchmark`.

use semio_framework_repo_cli_command_framework::{CliError, Invocation};
use semio_framework_repo_graphql::RepoContext;
use semio_framework_repo_cli_event_stream::{format_markdown_result, Format};
use semio_framework_repo_cli_tree_source_and_search::{status_filter, StreamOptions};
use semio_framework_repo_codebase::{Codebase, Scope as CodebaseScope};
use semio_framework_repo_model::{TicketStatus};
use semio_framework_repo_model_entity::entity;
use serde_json::{json, Value as Json};
use std::collections::{BTreeMap, BTreeSet};

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
