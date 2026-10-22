//! 🚀️ The unified repository command entrypoint.
fn main() {
 let argv: Vec<String> = std::env::args().skip(1).collect();
 let plan = semio_framework_repo_cli::repo_cli::entrypoint::plan_native_entrypoint(&argv);
 let code = match plan.kind {
  "mcp" => {
   let root = match std::env::current_dir().map_err(|error| error.to_string()).and_then(|current| semio_framework_repo_cli::repo_cli::entrypoint::resolve_native_mcp_root(&plan.arguments, &current)) { Ok(root) => root, Err(error) => { eprintln!("{error}"); std::process::exit(1); } };
   semio_framework_repo_mcp::run_with(&[], semio_framework_repo_cli::mcp_verb::real_repository(root))
  },
  "workspace" | "build" | "script" => {
   eprintln!("{}", semio_framework_repo_cli::repo_cli::entrypoint::LAUNCHER_REFUSAL);
   2
  },
  _ => semio_framework_repo_cli::repo_cli::run(&argv),
 };
 std::process::exit(code);
}
