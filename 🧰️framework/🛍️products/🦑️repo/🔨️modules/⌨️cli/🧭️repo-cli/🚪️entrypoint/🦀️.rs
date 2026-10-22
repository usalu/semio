//! 🚪️ One native repository process selects domain, build and explicit MCP ports.

/// 🧭️ The owned process route and its unchanged argument vector.
#[derive(Debug, PartialEq, Eq)]
pub struct NativeEntrypointPlan { pub kind: &'static str, pub arguments: Vec<String> }

/// 🎯️ Reads the existing repository-root option before the generic MCP protocol starts.
pub fn read_native_mcp_root_option(arguments: &[String]) -> Result<Option<String>, String> {
 match arguments {
  [] => Ok(None),
  [flag, value] if flag == "--repo" && !value.is_empty() && !value.starts_with('-') => Ok(Some(value.clone())),
  [value] if value.starts_with("--repo=") && value.len() > 7 => Ok(Some(value[7..].to_owned())),
  _ => Err("Expected mcp [--repo <directory>] / Erwartet mcp [--repo <Verzeichnis>]".into()),
 }
}

/// 🏠️ Binds a native MCP process to a validated explicit or discovered repository directory.
pub fn resolve_native_mcp_root(arguments: &[String], current: &std::path::Path) -> Result<std::path::PathBuf, String> {
 let selected = read_native_mcp_root_option(arguments)?;
 semio_framework_repo_workspace::select_repo_root(current, selected.as_deref().map(std::path::Path::new))
}

/// 🎯️ Selects the process port before domain flag parsing.
pub fn plan_native_entrypoint(argv: &[String]) -> NativeEntrypointPlan { plan_native_entrypoint_with(&build_verbs(), argv) }

/// 🎯️ Plans with explicit build verbs.
pub fn plan_native_entrypoint_with(verbs: &[String], argv: &[String]) -> NativeEntrypointPlan {
 if argv.len() >= 4 && argv[..4].iter().map(String::as_str).eq(["dev", "mcp", "stdio", "os"]) { return NativeEntrypointPlan { kind: "workspace", arguments: argv.to_vec() }; }
 let mut prefix = 0;
 while prefix < argv.len() { if argv[prefix] == "--repo" && prefix + 1 < argv.len() { prefix += 2; } else if argv[prefix].starts_with("--repo=") { prefix += 1; } else { break; } }
 if prefix > 0 && argv.get(prefix).is_some_and(|value| value == "mcp") { let mut arguments = argv[..prefix].to_vec(); arguments.extend_from_slice(&argv[prefix + 1..]); return NativeEntrypointPlan { kind: "mcp", arguments }; }
 if argv.first().is_some_and(|value| value == "mcp") { return NativeEntrypointPlan { kind: "mcp", arguments: argv[1..].to_vec() }; }
 if argv.first().is_some_and(|value| value == "dashboard") { let mut arguments: Vec<String> = ["run", "@semio-tech/repo-dashboard-rs:run", "--output=stream", "--"].into_iter().map(String::from).collect(); arguments.extend_from_slice(&argv[1..]); return NativeEntrypointPlan { kind: "build", arguments }; }
 if argv.is_empty() || argv.first().is_some_and(|value| value == "--help" || value == "-h") { return NativeEntrypointPlan { kind: "build", arguments: argv.to_vec() }; }
 if script_index(argv).is_some() { return NativeEntrypointPlan { kind: "script", arguments: argv.to_vec() }; }
 let mut index = 0;
 while index < argv.len() { if argv[index] == "--tool" { index += 2; } else if argv[index].starts_with("--tool=") { index += 1; } else if argv[index].starts_with("--service=") { index += 1; } else { break; } }
 let build = index > 0 || argv.get(index).is_some_and(|value| verbs.iter().any(|verb| verb == value));
 NativeEntrypointPlan { kind: if build { "build" } else { "domain" }, arguments: argv.to_vec() }
}

/// 📜️ The script file extensions a script route accepts (neutral vector `scriptExtensions`).
const SCRIPT_EXTENSIONS: [&str; 3] = [".ts", ".mjs", ".js"];

/// 📜️ The index of the script argument when only the neutral watch and cwd options precede it.
fn script_index(argv: &[String]) -> Option<usize> {
 let mut index = 0;
 while index < argv.len() { if argv[index] == "--watch" || argv[index].starts_with("--cwd=") { index += 1; } else if argv[index] == "--cwd" { index += 2; } else { break; } }
 (index < argv.len() && SCRIPT_EXTENSIONS.iter().any(|extension| argv[index].ends_with(extension))).then_some(index)
}

/// 🏗️ The launcher registry names the neutral operations catalogue whose operation names are the build verbs.
const LAUNCHER_REGISTRY: &str = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json";

fn read_build_verbs(root: &std::path::Path, registry: &std::path::Path) -> Option<Vec<String>> {
 let document: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(registry).ok()?).ok()?;
 let catalogue: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(root.join(document["routing"]["catalogue"].as_str()?)).ok()?).ok()?;
 Some(catalogue["operations"].as_array()?.iter().filter_map(|operation| operation["name"].as_str().map(String::from)).collect())
}

/// 🏗️ Reads the build verbs of the repository around the working directory; none outside a repository.
pub fn build_verbs() -> Vec<String> {
 let mut directory = std::env::current_dir().ok();
 while let Some(path) = directory {
  let registry = path.join(LAUNCHER_REGISTRY);
  if registry.is_file() { return read_build_verbs(&path, &registry).unwrap_or_default(); }
  directory = path.parent().map(std::path::Path::to_path_buf);
 }
 Vec::new()
}

/// 🛠️ Build, workspace and script routes are started by the one canonical launcher, the Go `repo` command (module `🚀️launcher`); this implementation plans the routes and refuses to start them.
pub const LAUNCHER_REFUSAL: &str = "Build commands are started by the canonical Go repo launcher: repo run repo-mcp:install\nBuild-Befehle startet der kanonische Go-repo-Launcher: repo run repo-mcp:install";

#[cfg(test)]
mod tests {
 #[test]
 fn native_mcp_repository_option_matches_shared_root_vectors() {
  let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🏠️workspace/🧭️root-selection/🧪️tests/🧫️fixtures/🔣️.json")).expect("shared root vectors");
  for row in fixture["optionCases"].as_array().expect("option cases") {
   let argv: Vec<String> = row["args"].as_array().expect("args").iter().map(|value| value.as_str().expect("argument").to_owned()).collect();
   let result = super::read_native_mcp_root_option(&argv);
   assert_eq!(result.is_ok(), row["accepted"].as_bool().expect("acceptance"));
   if let Ok(value) = result { assert_eq!(serde_json::json!(value), row["root"]); }
  }
 }

 #[test]
 fn native_entrypoint_matches_neutral_vectors() {
  let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚪️entrypoint/🔣️.json")).unwrap();
  for row in fixture["vectors"].as_array().unwrap() {
   let argv: Vec<String> = row["argv"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect();
   let actual = super::plan_native_entrypoint(&argv);
   assert_eq!(actual.kind, row["kind"].as_str().unwrap(), "{}", row["id"]);
   assert_eq!(serde_json::json!(actual.arguments), row["arguments"], "{}", row["id"]);
  }
 }
}
