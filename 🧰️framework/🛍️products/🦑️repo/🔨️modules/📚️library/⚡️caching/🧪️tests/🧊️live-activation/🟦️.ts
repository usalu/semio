import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🧊️ Runs the actual preparation/activation commands with controlled publishers under native Nx. */
export async function testWgpuLiveActivation(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const root = mkdtempSync(join(output, "wgpu-live-")), owner = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
  const put = (path: string, text: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
  const project = (path: string, name: string) => put(join(path, "📋️project.json"), JSON.stringify({ name, targets: { wasm: {}, "wasm-release": {} } }));
  for (const [path, name] of [["🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust", "surface"], ["🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust", "editor"], ["🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust", "flow"], ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript", "renderer"]]) project(path!, name!);
  put(join(owner, "package.json"), JSON.stringify({ name: fixture.project, private: true }));
  put("component/Cargo.toml", `[package]\nname = "sample"\nversion = "0.1.0"\n[package.metadata.component]\npackage = "semio:${fixture.plugin}"\n[package.metadata.semio]\nrole = "plugin"\n[[package.metadata.semio.playground]]\nvariant = "${fixture.variant}"\n`);
  const { cacheInternals } = await import("../../../🟨️.mjs");
  const targets = cacheInternals.playgroundPreparationTargets(["component/Cargo.toml"], root, owner);
  for (const profile of ["dev", "release"]) assert.ok(targets[`prepare-${fixture.variant}-wgpu-${profile}`].dependsOn.includes(`renderer:${profile === "release" ? "wasm-release" : "wasm"}`), "The renderer prerequisite must select the matching compilation profile");
  for (const profile of ["dev", "release"]) for (const command of ["serve", "dev"]) {
    const target = targets[`${command}-${fixture.variant}-wgpu-${profile}`];
    assert.equal(target.options.command, `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve ${fixture.variant} ${profile}`, "The server must consume the completed profile through its narrow leaf");
    assert.deepEqual(target.dependsOn, [`activate-${fixture.variant}-wgpu-${profile}`]);
    assert.equal(target.cache, false);
    assert.equal(target.continuous, true);
  }
  const prepare = `prepare-${fixture.variant}-wgpu-${fixture.profile}`, activate = `activate-${fixture.variant}-wgpu-${fixture.profile}`;
  assert.equal(targets[prepare].cache, true);
  assert.equal(targets[activate].cache, false, "Live installation and reload publication must execute after every preparation/cache restoration");
  assert.deepEqual(targets[activate].dependsOn, [prepare]);
  const ts = require("typescript"), source = ts.createSourceFile("os-dev.ts", readFileSync(join(workspace, owner, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const names = ["PreparationScript", "ActivationScript", "publishWgpuRuntimePrerequisites"];
  const code = ts.transpileModule(source.statements.filter((node: any) => names.includes(node.name?.text)).map((node: any) => node.getText(source)).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  assert.ok(!/new PreparationScript/.test(code), "Nx owns preparation; activation must not execute that command again");
  put("package.json", JSON.stringify({ name: "wgpu-live-fixture", private: true }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put(".gitignore", "node_modules\n.nx\nmodules\nlive\n.counts\n.🧬semio\n");
  put("project.json", JSON.stringify({ name: "fixture", targets: {
    materialize: { executor: "nx:run-commands", cache: true, inputs: ["{workspaceRoot}/📜️script.ts"], outputs: ["{workspaceRoot}/modules", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/sessions", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts"], options: { command: "bun ./📜️script.ts materialize" } },
    [prepare]: { ...targets[prepare], executor: "nx:run-commands", dependsOn: ["materialize"], inputs: ["{workspaceRoot}/📜️script.ts", { dependentTasksOutputFiles: "**/*" }], options: { command: "bun ./📜️script.ts prepare" } },
    [activate]: { ...targets[activate], executor: "nx:run-commands", options: { command: "bun ./📜️script.ts activate" } }
  } }));
  put("📜️script.ts", `import assert from "node:assert/strict";
import { appendFileSync, copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
const fixture = ${JSON.stringify(fixture)}, repoRoot = process.cwd(), moduleRoot = join(repoRoot, "modules");
const FONT_ASSET = "fonts.bin", MODULE_BRIDGE_FILE = "bridge.js", MODULE_HOT_SWAP_FILE = "reload.json", PREVIEW2_VENDOR_RELATIVE = "support", MODULE_SHARD_DIRECTORY = "shards", SHARD_WORKER_FILE = "worker.js";
const put = (path, value) => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, value); };
const live = join(repoRoot, "live"), count = (name) => appendFileSync(join(repoRoot, ".counts"), name + "\\n");
const pluginModulesRoot = () => moduleRoot, moduleDirectoryName = (id) => id;
const playgroundCatalog = [{ variant: fixture.variant, pluginId: fixture.plugin }], validateFontAsset = () => 1;
const readGeneratedCatalogProjection = () => ({ entries: [{ pluginId: fixture.plugin }] });
const publishBuiltExtension = (target, source) => { mkdirSync(live, { recursive: true }); copyFileSync(join(source, MODULE_BRIDGE_FILE), join(live, "extension.js")); count("publication"); };
const ensureGuestSlimTypstFontsAsset = () => put(join(live, FONT_ASSET), "fonts");
class BundleScript { constructor(root) { this.root = root; } }
${code}
if (process.argv[2] === "materialize") {
  put(join(moduleRoot, fixture.plugin, MODULE_BRIDGE_FILE), fixture.payload);
  put(join(moduleRoot, fixture.plugin, "🔣️.json"), JSON.stringify({ manifest: { pluginId: fixture.plugin } }));
  for (const path of [join(fixture.plugin, ".nx-artifact.json"), join(PREVIEW2_VENDOR_RELATIVE, ".nx-artifact.json"), join(MODULE_SHARD_DIRECTORY, SHARD_WORKER_FILE)]) put(join(moduleRoot, path), "{}");
  put(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/sessions", fixture.variant, "🎮️playground-session/🟦️.ts"), "export const PLAYGROUND_SESSION = " + JSON.stringify({ variant: fixture.variant, registryPluginId: fixture.plugin, plugins: [{ pluginId: fixture.plugin }] }));
  put(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts", FONT_ASSET), "fonts");
} else {
  const before = existsSync(join(repoRoot, ".counts")) ? readFileSync(join(repoRoot, ".counts"), "utf8") : "";
  const Command = process.argv[2] === "prepare" ? PreparationScript : ActivationScript;
  await new Command(repoRoot).run([fixture.variant, "wgpu", fixture.profile]);
  if (process.argv[2] === "prepare") assert.equal(existsSync(join(repoRoot, ".counts")) ? readFileSync(join(repoRoot, ".counts"), "utf8") : "", before);
  count(process.argv[2]);
}
`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const env = { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache") };
  for (const cycle of fixture.cycles) {
    if (cycle.deleteLiveState) { rmSync(join(root, "live"), { recursive: true }); rmSync(join(root, "modules/reload.json")); }
    const child = Bun.spawn(["node", cli, "run", `fixture:${activate}`, "--outputStyle=static"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    assert.equal(status, 0, stdout + stderr);
    const counts = readFileSync(join(root, ".counts"), "utf8").trim().split("\n");
    assert.equal(counts.filter(name => name === "prepare").length, cycle.preparations);
    assert.equal(counts.filter(name => name === "activate").length, cycle.activations);
    assert.equal(counts.filter(name => name === "publication").length, cycle.activations);
    assert.equal(readFileSync(join(root, "live/extension.js"), "utf8"), fixture.payload);
    assert.equal(readFileSync(join(root, "live/fonts.bin"), "utf8"), "fonts");
    assert.equal(JSON.parse(readFileSync(join(root, "modules/reload.json"), "utf8")).pluginId, "*");
  }
  console.log("[DEBUG] Native Nx caches pure WGPU preparation, executes live publication once per activation and reconstructs deleted live state PASS");
  rmSync(root, { recursive: true, force: true });
}
