/** 🧰️Dev tooling env without IDE-injected node options. */
export function devToolingEnv(extra: NodeJS.ProcessEnv = {}): NodeJS.ProcessEnv {
  const env = { ...process.env, ...extra };
  delete env.NODE_OPTIONS;
  delete env.VSCODE_INSPECTOR_OPTIONS;
  env.NX_NATIVE_COMMAND_RUNNER ??= "false";
  env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT ??= "false";
  env.NX_TUI ??= "false";
  env.NX_ISOLATE_PLUGINS = "false";
  env.NX_VERBOSE_LOGGING ??= "false";
  env.NX_PERF_LOGGING ??= "false";
  env.NX_NATIVE_LOGGING ??= "nx=warn";
  env.RUSTC_WRAPPER ??= "";
  return env;
}
