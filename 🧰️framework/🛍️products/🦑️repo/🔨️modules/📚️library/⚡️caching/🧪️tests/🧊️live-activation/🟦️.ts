import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🧊️ Runs the actual preparation/activation commands with controlled publishers under native Nx. */
export async function testWgpuLiveActivation(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const activationModule = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts");
  const { developmentRuntimeRoot } = await import(activationModule);
  const namespaces = fixture.renderers.flatMap((renderer: string) => fixture.profiles.flatMap((profile: string) => fixture.variants.map((variant: string) => ({ renderer, profile, variant }))));
  const expectedRoots = require("lodash").map(namespaces, (row: any) => join(output, "dist/runtime", row.renderer, row.profile, row.variant));
  assert.deepEqual(namespaces.map((row: any) => developmentRuntimeRoot(output, row.variant, row.profile, row.renderer)), expectedRoots, "Every renderer, variant and profile needs an isolated runtime installation");
  assert.equal(new Set(expectedRoots).size, 8);
  assert.throws(() => developmentRuntimeRoot(output, fixture.variant, fixture.profile, "unknown"), /renderer/i);
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
  assert.deepEqual(targets[activate].outputs, [`{projectRoot}/dist/runtime/wgpu/${fixture.profile}/${fixture.variant}`], "Uncached activation still owns its complete live output namespace");
  assert.deepEqual(targets[activate].dependsOn, [prepare]);
  const ts = require("typescript"), activationOwner = dirname(activationModule);
  const definitions = [["🧰️preparation/🟦️.ts", ["PreparationScript"]], ["🏃️execution/🟦️.ts", ["ActivationScript"]], ["📥️installation/🟦️.ts", ["activationFilesDigest", "publishActivatedExtension"]]] as const;
  const code = definitions.map(([file, names]) => {
    const source = ts.createSourceFile(file, readFileSync(join(activationOwner, file), "utf8"), ts.ScriptTarget.Latest, true);
    const nodes = source.statements.filter((node: any) => names.includes(node.name?.text));
    assert.equal(nodes.length, names.length, `Every tested definition must come from ${file}`);
    return ts.transpileModule(nodes.map((node: any) => node.getText(source)).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  }).join("\n");
  assert.ok(!/new PreparationScript/.test(code), "Nx owns preparation; activation must not execute that command again");
  assert.ok(!/publishBuiltExtension|ensureGuestSlimTypstFontsAsset|publishWgpuRuntimePrerequisites/.test(code), "Activation must neither publish globally nor rebuild fonts");
  put("package.json", JSON.stringify({ name: "wgpu-live-fixture", private: true }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put(".gitignore", "node_modules\n.nx\nmodules\ndist\n.counts\n.🧬semio\n");
  put("project.json", JSON.stringify({ name: "fixture", targets: {
    materialize: { executor: "nx:run-commands", cache: true, inputs: ["{workspaceRoot}/📜️script.ts"], outputs: ["{workspaceRoot}/modules", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/sessions", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts"], options: { command: "bun ./📜️script.ts materialize" } },
    [prepare]: { ...targets[prepare], executor: "nx:run-commands", dependsOn: ["materialize"], inputs: ["{workspaceRoot}/📜️script.ts", { dependentTasksOutputFiles: "**/*" }], options: { command: "bun ./📜️script.ts prepare" } },
    [activate]: { ...targets[activate], executor: "nx:run-commands", options: { command: "bun ./📜️script.ts activate" } },
    cancel: { executor: "nx:run-commands", cache: false, outputs: [], dependsOn: ["materialize"], options: { command: "bun ./📜️script.ts cancel" } },
    ...Object.fromEntries(namespaces.map((row: any) => [`install-${row.variant}-${row.renderer}-${row.profile}`, { executor: "nx:run-commands", cache: false, outputs: [], dependsOn: ["materialize"], options: { command: `bun ./📜️script.ts activate ${row.variant} ${row.renderer} ${row.profile}` } }]))
  } }));
  put("📜️script.ts", `import assert from "node:assert/strict";
import { appendFileSync, createReadStream, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import { ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot, nextActivationReceipt, publishActivationReceipt, readActivationReceipt } from ${JSON.stringify(activationModule)};
import { artifactFiles } from ${JSON.stringify(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts"))};
import { stageArtifacts } from ${JSON.stringify(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts"))};
const fixture = ${JSON.stringify(fixture)}, repoRoot = process.cwd();
const [operation, variant = fixture.variant, renderer = "wgpu", profile = fixture.profile] = process.argv.slice(2), moduleRoot = join(repoRoot, "modules", profile);
const FONT_ASSET = "fonts.bin", MODULE_BRIDGE_FILE = "bridge.js", PREVIEW2_VENDOR_RELATIVE = "support", MODULE_SHARD_DIRECTORY = "shards", SHARD_WORKER_FILE = "worker.js";
const EXTENSION_INSTALL_META = "install.json", MODULE_PLUGIN_ROUTE = "/plugins", MODULE_EXTENSION_ROUTE = "/extensions", rewritePreview2ShimImportSource = (text) => text;
const put = (path, value) => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, value); };
const count = (name) => appendFileSync(join(repoRoot, ".counts"), name + "\\n");
const pluginModulesRoot = () => moduleRoot, moduleDirectoryName = (id) => id;
const playgroundCatalog = fixture.variants.map(variant => ({ variant, pluginId: fixture.plugin })), validateFontAsset = () => 1;
const readGeneratedCatalogProjection = () => ({ entries: [{ pluginId: fixture.plugin, role: "extension" }] });
class BundleScript { constructor(root) { this.root = root; } }
${code}
if (operation === "materialize") {
  for (const profile of fixture.profiles) {
    const moduleRoot = join(repoRoot, "modules", profile);
    put(join(moduleRoot, fixture.plugin, MODULE_BRIDGE_FILE), fixture.payload + " " + profile);
    put(join(moduleRoot, fixture.plugin, "🔣️.json"), JSON.stringify({ manifest: { pluginId: fixture.plugin } }));
    for (const path of [join(fixture.plugin, ".nx-artifact.json"), join(PREVIEW2_VENDOR_RELATIVE, ".nx-artifact.json"), join(MODULE_SHARD_DIRECTORY, SHARD_WORKER_FILE)]) put(join(moduleRoot, path), "{}");
  }
  for (const variant of fixture.variants) put(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/dist/sessions", variant, "🎮️playground-session/🟦️.ts"), "export const PLAYGROUND_SESSION = " + JSON.stringify({ variant, registryPluginId: fixture.plugin, plugins: [{ pluginId: fixture.plugin }] }));
  put(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts", FONT_ASSET), "fonts");
} else if (operation === "cancel") {
  const destination = join(repoRoot, "cancelled"), signal = AbortSignal.abort(new Error("Activation cancelled"));
  await assert.rejects(() => publishActivatedExtension({ pluginId: fixture.plugin, role: "extension" }, join(moduleRoot, fixture.plugin), destination, "a".repeat(64), 1, signal), /Activation cancelled/);
  assert.equal(existsSync(destination), false);
} else {
  const before = existsSync(join(repoRoot, ".counts")) ? readFileSync(join(repoRoot, ".counts"), "utf8") : "";
  const Command = process.argv[2] === "prepare" ? PreparationScript : ActivationScript;
  await new Command(repoRoot).run([variant, renderer, profile]);
  if (process.argv[2] === "prepare") assert.equal(existsSync(join(repoRoot, ".counts")) ? readFileSync(join(repoRoot, ".counts"), "utf8") : "", before);
  count(process.argv[2]);
}
`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const env = { ...process.env, SEMIO_REPO_ROOT: root, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache") };
  const run = async (args: string[]) => {
    const child = Bun.spawn(["node", cli, ...args, "--outputStyle=static"], { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const cancel = () => child.kill("SIGTERM"), deadline = setTimeout(cancel, 60000);
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
      assert.equal(status, 0, stdout + stderr);
      return stdout;
    } finally { clearTimeout(deadline); process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  };
  let previous: { receipt: string; mtime: number } | undefined;
  for (const cycle of fixture.cycles) {
    const live = developmentRuntimeRoot(root, fixture.variant, fixture.profile, "wgpu");
    if (cycle.deleteLiveState) rmSync(live, { recursive: true });
    await run(["run", `fixture:${activate}`]);
    const counts = readFileSync(join(root, ".counts"), "utf8").trim().split("\n");
    assert.equal(counts.filter(name => name === "prepare").length, cycle.preparations);
    assert.equal(counts.filter(name => name === "activate").length, cycle.activations);
    assert.equal(readFileSync(join(live, "extensions", fixture.plugin, "bridge.js"), "utf8"), fixture.payload + " " + fixture.profile);
    const receiptText = readFileSync(join(live, "activation/🔣️receipt.json"), "utf8"), receipt = JSON.parse(receiptText);
    assert.equal(receipt.plugins[0].pluginId, fixture.plugin);
    assert.equal(JSON.parse(readFileSync(join(live, "extensions", fixture.plugin, "install.json"), "utf8")).packageHash, receipt.plugins[0].artifactSha256);
    const current = { receipt: receiptText, mtime: statSync(join(live, "extensions", fixture.plugin, "bridge.js")).mtimeMs };
    if (previous && !cycle.deleteLiveState) assert.deepEqual(current, previous, "Warm activation preserves installation and receipt bytes without a spurious reload");
    previous = current;
  }
  await run(["run-many", "--projects=fixture", `--targets=${namespaces.map((row: any) => `install-${row.variant}-${row.renderer}-${row.profile}`).join(",")}`, "--parallel=4"]);
  for (const row of namespaces) {
    const live = developmentRuntimeRoot(root, row.variant, row.profile, row.renderer);
    assert.equal(readFileSync(join(live, "extensions", fixture.plugin, "bridge.js"), "utf8"), fixture.payload + " " + row.profile);
    assert.equal(JSON.parse(readFileSync(join(live, "activation/🔣️receipt.json"), "utf8")).variant, row.variant);
  }
  assert.equal(existsSync(join(root, "modules", fixture.profile, "reload.json")), false, "Activation cannot mutate the shared compiler/materializer outputs");
  assert.equal(existsSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️extension-modules")), false);
  await run(["run", "fixture:cancel"]);
  console.log("[DEBUG] Native Nx preserves warm receipts, reconstructs deleted live state and publishes eight renderer/variant/profile installations independently; lodash/Ajv contracts agree PASS");
  rmSync(root, { recursive: true, force: true });
}
