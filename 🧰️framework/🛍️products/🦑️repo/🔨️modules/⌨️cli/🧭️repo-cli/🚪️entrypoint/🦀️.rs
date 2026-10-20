//! 🚪️ One native repository process selects domain, build and explicit MCP ports.

/// 🧭️ The owned process route and its unchanged argument vector.
#[derive(Debug, PartialEq, Eq)]
pub struct NativeEntrypointPlan { pub kind: &'static str, pub arguments: Vec<String> }

/// 🎯️ Selects the process port before domain flag parsing.
pub fn plan_native_entrypoint(argv: &[String]) -> NativeEntrypointPlan {
 if argv.len() >= 4 && argv[..4].iter().map(String::as_str).eq(["dev", "mcp", "stdio", "os"]) { return NativeEntrypointPlan { kind: "workspace", arguments: argv.to_vec() }; }
 if argv.first().is_some_and(|value| value == "mcp") { return NativeEntrypointPlan { kind: "mcp", arguments: argv[1..].to_vec() }; }
 if argv.first().is_some_and(|value| value == "dashboard") { let mut arguments: Vec<String> = ["run", "@semio-tech/repo-dashboard-rs:run", "--output=stream", "--"].into_iter().map(String::from).collect(); arguments.extend_from_slice(&argv[1..]); return NativeEntrypointPlan { kind: "build", arguments }; }
 let mut index = 0;
 while index < argv.len() { if argv[index] == "--tool" { index += 2; } else if argv[index].starts_with("--tool=") { index += 1; } else { break; } }
 let build = index > 0 || argv.get(index).is_some_and(|value| matches!(value.as_str(), "run" | "run-many" | "affected" | "exec" | "watch" | "dependencies" | "cargo" | "tools" | "reset" | "cache" | "preflight" | "show" | "graph" | "inputs" | "incremental" | "concurrency" | "capacity" | "leases" | "remote-execution"));
 NativeEntrypointPlan { kind: if build { "build" } else { "domain" }, arguments: argv.to_vec() }
}

/// 🛠️ The tool-independent build process boundary.
pub trait NativeScriptTool { fn run(&self, root: &std::path::Path, arguments: &[String]) -> Result<i32, String>; }

struct BunScriptTool { entry: &'static str, prefix: &'static [&'static str] }
impl NativeScriptTool for BunScriptTool {
 fn run(&self, root: &std::path::Path, arguments: &[String]) -> Result<i32, String> {
  let mut command = std::process::Command::new("bun");
  command.arg(root.join(self.entry)).args(self.prefix).args(arguments);
  #[cfg(unix)] { use std::os::unix::process::CommandExt; Err(command.exec().to_string()) }
  #[cfg(not(unix))] { command.status().map(|status| status.code().unwrap_or(1)).map_err(|error| error.to_string()) }
 }
}

/// 🏗️ Runs the owned script entry with inherited environment, streams and process authority.
fn run_native_script(arguments: &[String], tool: &dyn NativeScriptTool) -> i32 {
 let current = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
 let root = semio_framework_repo_workspace::find_repo_root(&current);
 match tool.run(&root, arguments) { Ok(code) => code, Err(error) => { eprintln!("{error}"); 1 } }
}

/// 🏗️ Enters the authored build owner through the shared native script port.
pub fn run_native_build(arguments: &[String]) -> i32 { run_native_script(arguments, &BunScriptTool { entry: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts", prefix: &["repo"] }) }

/// 🌉️ Enters the existing OS MCP workspace owner without generic build logging.
pub fn run_native_workspace(arguments: &[String]) -> i32 { run_native_script(arguments, &BunScriptTool { entry: "📜️script.ts", prefix: &[] }) }

#[cfg(test)]
mod tests {
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
