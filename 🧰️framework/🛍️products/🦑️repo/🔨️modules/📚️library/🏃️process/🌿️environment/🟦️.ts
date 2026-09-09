/** 🧰️Dev tooling env without IDE-injected node options. Plugin isolation is deliberately left at Nx's
 * own default: this workspace's inference plugin
 * (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`) is an async ES module — it top-level
 * `await import()`s the runtime-component closure under a revision query — so `NX_ISOLATE_PLUGINS=false`
 * makes Nx's in-process `require()` path (`runPreTasksExecution` → `getPluginsSeparated`) reject it with
 * "require() async module … is unsupported", which fails every `nx run` spawned with this env even
 * though the daemon-served project graph itself resolves. */
export function devToolingEnv(extra: NodeJS.ProcessEnv = {}): NodeJS.ProcessEnv {
  const env = { ...process.env, ...extra };
  delete env.NODE_OPTIONS;
  delete env.VSCODE_INSPECTOR_OPTIONS;
  env.NX_NATIVE_COMMAND_RUNNER ??= "false";
  env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT ??= "false";
  env.NX_TUI ??= "false";
  env.NX_VERBOSE_LOGGING ??= "false";
  env.NX_PERF_LOGGING ??= "false";
  env.NX_NATIVE_LOGGING ??= "nx=warn";
  env.RUSTC_WRAPPER ??= "";
  return env;
}
