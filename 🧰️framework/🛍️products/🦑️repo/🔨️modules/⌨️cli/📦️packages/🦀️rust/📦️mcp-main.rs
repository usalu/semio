//! 🚀️ The unified repository command entrypoint.
fn main() {
 let argv: Vec<String> = std::env::args().skip(1).collect();
 let plan = semio_framework_repo_cli::repo_cli::entrypoint::plan_native_entrypoint(&argv);
 let code = match plan.kind {
  "mcp" => {
   let root = semio_framework_repo_workspace::find_repo_root(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")));
   semio_framework_repo_mcp::run_with(&plan.arguments, semio_framework_repo_cli::mcp_verb::real_repository(root))
  },
  "workspace" => semio_framework_repo_cli::repo_cli::entrypoint::run_native_workspace(&plan.arguments),
  "build" => semio_framework_repo_cli::repo_cli::entrypoint::run_native_build(&plan.arguments),
  _ => semio_framework_repo_cli::repo_cli::run(&argv),
 };
 std::process::exit(code);
}
