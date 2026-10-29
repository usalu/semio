import { spawn } from "node:child_process";
import { createRequire } from "node:module";
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
  test: ["-m", "unittest", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.📚️closure.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🧪️tests.📑️heading.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.💾️persisted.🐍️", "🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧬️algorithms.🧪️tests.🧩️widgets.🐍️", "🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧬️algorithms.🧪️tests.📈️comparison.🐍️", "🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧪️experiments.🧪️tests.🧊️consumed.🐍️", "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🎭️presentation.🐍️", "🎓️teaching.🏛️architecture.🏢️nakagin.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.🎭️presentation.🐍️", "🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.📇️registry.🐍️","🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧪️tests.🎭️presentation.🐍️", "🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🧪️tests.📇️registry.🐍️","🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔗️consistency.🧪️tests.🔄️lifecycle.🐍️", "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🧪️tests.📏️footer.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🖼️stream.🐍️", "🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.👥️audience.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖱️interaction.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.📐️geometry.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🌐️browser.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🧪️tests.🧩️chapters.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.📟️readout.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️silhouettes", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🎨️semio-style.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️integration", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️scenes", "🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.📦️volume.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️transforms", "🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚖️regulations.🧪️tests.☀️sun.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🏃️activity.🧪️tests.🐍️", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️sections", "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️section_consumers", ...args],
};
if (!commands[command]) throw new Error("Expected render or test");
if (command === "test") {
  const require = createRequire(import.meta.url);
  for (const subject of ["🏗️bim", "🎯️optimization", "🦗️grasshopper", "📊️excel"]) require(join(workspace, "🎓️teaching/🏛️architecture", subject, "🎬️clip/🧪️tests/🟨️.cjs"));
  require(join(workspace, "🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🟨️.cjs"));
  require(join(workspace, "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧬️loss-selector/🟦️.ts"));
}
const cache = resolve(workspace, process.env.SEMIO_TEACHING_CACHE_ROOT ?? join(workspace, ".🧬semio/🎓️teaching/⚡️cache"));
const outputRoot = resolve(workspace, process.env.SEMIO_TEACHING_OUTPUT_ROOT ?? join(cache, command === "test" ? "tests" : "videos"));
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
