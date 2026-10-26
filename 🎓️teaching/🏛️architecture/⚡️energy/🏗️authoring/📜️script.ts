import { spawn } from "node:child_process";
import { dirname, join, resolve } from "node:path";
import { existsSync, mkdirSync } from "node:fs";

const root = import.meta.dir;
let workspace = root;
while (!existsSync(join(workspace, "🧰️framework"))) {
  const parent = dirname(workspace);
  if (parent === workspace) throw new Error("Workspace root not found");
  workspace = parent;
}
const project = join(root, "📦️packages", "🐍️python");
const command = process.argv[2] ?? "render";
const args = process.argv.slice(3);
const commands: Record<string, string[]> = {
  render: [join(root, "🎬️clip", "🖌️render", "🐍️.py"), ...args],
  test: ["-m", "unittest", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🧪️tests.🐍️", ...args],
};
if (!commands[command]) throw new Error("Expected render or test");
const cache = resolve(process.env.SEMIO_TEACHING_CACHE_ROOT ?? join(workspace, ".🧬semio/🎓️teaching/⚡️cache"));
const outputRoot = process.env.SEMIO_TEACHING_OUTPUT_ROOT ?? (command === "test" ? join(cache, "tests") : undefined);
if (outputRoot) mkdirSync(outputRoot, { recursive: true });
const env = { ...process.env, PYTHONUTF8: "1", PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1", SEMIO_TEST_MANIM: command === "test" ? "1" : process.env.SEMIO_TEST_MANIM,
  UV_PROJECT_ENVIRONMENT: process.env.SEMIO_TEACHING_ENV ?? join(cache, "python"),
  SEMIO_TEACHING_CACHE_DIR: process.env.SEMIO_TEACHING_CACHE_DIR ?? join(cache, "fonts"),
  SEMIO_TEACHING_OUTPUT_ROOT: outputRoot };
const child = spawn("uv", ["run", "--project", project, "--python", "3.12", ...(command === "test" ? ["--group", "test"] : []), "python", ...commands[command]],
  { cwd: workspace, env, stdio: "inherit", windowsHide: true, detached: process.platform !== "win32" });
/** 🛑 Cancel the complete render process tree on each supported host. */
function cancel(signal: "SIGINT" | "SIGTERM") {
  if (!child.pid) return;
  if (process.platform === "win32") {
    spawn("taskkill", ["/PID", String(child.pid), "/T", "/F"], { stdio: "ignore", windowsHide: true });
  } else {
    try { process.kill(-child.pid, signal); } catch { child.kill(signal); }
  }
}
for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => cancel(signal));
child.on("error", error => { console.error(error.message); process.exitCode = 1; });
child.on("exit", code => { process.exitCode = code ?? 1; });
