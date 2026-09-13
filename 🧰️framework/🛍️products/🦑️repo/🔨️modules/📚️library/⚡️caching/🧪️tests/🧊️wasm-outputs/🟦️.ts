import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import path, { delimiter, dirname, join } from "node:path";

/** 🧊️ Executes restored renderer artifacts and compares them with an independent native Trunk build. */
export async function testWgpuWasmOutputs(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const packagePath = join(fixture.owner, "📦️packages/🦀️rust"), project = JSON.parse(readFileSync(join(workspace, fixture.owner, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  assert.equal(project.metadata.nativeRoot, packagePath, "Renderer compiler inputs must resolve from the Cargo owner");
  const ts = require("typescript"), consumerText = readFileSync(join(workspace, fixture.consumer), "utf8"), consumer = ts.createSourceFile("vite.ts", consumerText, ts.ScriptTarget.Latest, true);
  let directoryExpression = "";
  const visit = (node: any) => { if (ts.isVariableDeclaration(node) && node.name.getText(consumer) === "rendererModulesDir") directoryExpression = node.initializer.getText(consumer); ts.forEachChild(node, visit); };
  visit(consumer);
  assert.match(consumerText, /find: "\/renderer-modules\/wgpu", replacement: rendererModulesDir/);
  const rendererDirectory = new Function("path", "playDir", "profile", `return ${directoryExpression};`);
  const consumerRoot = path.resolve(workspace, dirname(fixture.consumer), "../..");
  for (const row of fixture.profiles) {
    const target = project.targets[row.target];
    assert.equal(target?.cache, true, `${row.target} must cache its compiler deliverables`);
    assert.deepEqual(target.outputs, [`{workspaceRoot}/${packagePath}/${row.output}`]);
    assert.ok(target.dependsOn.includes("workspace:deps-trunk"));
    assert.ok(target.dependsOn.includes("workspace:deps-wasm-opt"));
    assert.ok(target.options.command.endsWith(`build ${row.profile}`));
    assert.equal(rendererDirectory(path, consumerRoot, row.profile), join(workspace, packagePath, row.output));
    for (const file of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) assert.ok(require("jsonc-parser").parse(readFileSync(join(workspace, file), "utf8")).configurations.some((entry: any) => entry.command === `bun nx run ${fixture.project}:${row.target}`), `${file} must expose the compiler profile`);
  }
  const compiler = join(workspace, fixture.owner, "🏗️compiler/🌐️wasm/📜️script.ts");
  const bundle = await require("esbuild").build({ entryPoints: [compiler], absWorkingDir: workspace, bundle: true, packages: "external", platform: "node", format: "esm", write: false, metafile: true });
  assert.ok(!Object.keys(bundle.metafile.inputs).some(path => path.endsWith("🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts")), "Finite renderer compilation must not import browser/application tests or live stores");
  const root = mkdtempSync(join(output, "wgpu-wasm-"));
  const put = (path: string, value: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), value); };
  const bindgen = (Bun.TOML.parse(readFileSync(join(workspace, "Cargo.lock"), "utf8")) as any).package.find((row: any) => row.name === "wasm-bindgen").version;
  put("Cargo.toml", `[package]\nname=${JSON.stringify(fixture.crate)}\nversion="0.1.0"\nedition="2024"\n[lib]\npath="🦀️.rs"\ncrate-type=["cdylib"]\n[dependencies]\nwasm-bindgen="=${bindgen}"\n[workspace]\n`);
  put("🦀️.rs", fixture.rust.replace("VALUE", String(fixture.values[0])));
  put("package.json", JSON.stringify({ name: "wgpu-wasm-fixture", private: true, type: "module" }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put(".gitignore", "node_modules\n.nx\ndist\nstate\noracle\n.runs\n.🧬semio\n");
  const targets = Object.fromEntries(fixture.profiles.map((row: any) => [row.target, { executor: "nx:run-commands", cache: true, inputs: ["{projectRoot}/Cargo.toml", "{projectRoot}/Cargo.lock", "{projectRoot}/🦀️.rs", "{projectRoot}/📜️script.ts"], outputs: [`{projectRoot}/${row.output}`], options: { command: `bun ./📜️script.ts build ${row.profile}`, cwd: "." } }]));
  put("project.json", JSON.stringify({ name: "renderer", targets }));
  put("📜️script.ts", `import { appendFileSync } from "node:fs";\nimport { buildTrunkRenderer } from ${JSON.stringify(compiler)};\nawait buildTrunkRenderer({ rustPackageRoot: process.cwd(), workspace: process.cwd(), toolWorkspace: ${JSON.stringify(workspace)}, profile: process.argv[3], stateRoot: "state" });\nappendFileSync(".runs", process.argv[3] + "\\n");\n`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const env = { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), SEMIO_REPO_ROOT: root, CARGO_TARGET_DIR: join(root, "state/target"), CARGO_BUILD_BUILD_DIR: join(root, "state/build"), TRUNK_BUILD_DIST: join(root, "forbidden"), TRUNK_BUILD_RELEASE: "true", TRUNK_TOOLS_WASM_BINDGEN: "0.0.0" };
  const execute = async (args: string[], environment = env) => {
    const child = Bun.spawn(args, { cwd: root, env: environment, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    assert.equal(code, 0, stdout + stderr); return stdout;
  };
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js"), runs = () => readFileSync(join(root, ".runs"), "utf8").trim().split("\n").length;
  await execute(["cargo", "generate-lockfile", "--offline"]);
  const run = (target: string) => execute(["node", cli, "run", `renderer:${target}`, "--outputStyle=static"]);
  const files = (directory: string): Map<string, Buffer> => new Map(readdirSync(directory, { recursive: true, withFileTypes: true }).filter(row => row.isFile()).map(row => [join(row.parentPath, row.name).slice(directory.length + 1), readFileSync(join(row.parentPath, row.name))]));
  const value = async (path: string) => JSON.parse(await execute(["node", "--input-type=module", "-e", `import { readFileSync } from "node:fs"; import { pathToFileURL } from "node:url"; const module = await import(pathToFileURL(process.argv[1].replace(/_bg.wasm$/, ".js")).href); await module.default({ module_or_path: readFileSync(process.argv[1]) }); console.log(JSON.stringify({ value: module.answer() }));`, path])).value;
  const { preparedBinaryen } = await import("../../🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts");
  const oracleEnv = { ...env, PATH: `${dirname(preparedBinaryen(workspace))}${delimiter}${env.PATH}`, CARGO_TARGET_DIR: join(root, "state/oracle-target") };
  for (const key of Object.keys(oracleEnv)) if (key.startsWith("TRUNK_") || ["NO_COLOR", "FORCE_COLOR"].includes(key)) delete oracleEnv[key];
  const oracle = async (profile: string, expected: number) => {
    put("oracle/index.html", '<!doctype html><html><head><link data-trunk rel="rust" href="../Cargo.toml" data-wasm-opt="z" data-type="worker" data-bindgen-target="web" /></head><body></body></html>\n');
    put("oracle/Trunk.toml", `required_version="=0.21.14"\noffline=true\n[build]\ntarget="index.html"\ndist="dist"\nlocked=true\nfrozen=true\nfilehash=false\nrelease=${profile === "release"}\n[tools]\nwasm_bindgen="${bindgen}"\nwasm_opt="version_130"\n`);
    await execute(["trunk", "build", "--config", join(root, "oracle/Trunk.toml"), "--skip-version-check", "--offline", "true"], oracleEnv);
    assert.equal(await value(join(root, "oracle/dist", `${fixture.crate}_bg.wasm`)), expected);
    for (const extension of [".js", "_bg.wasm"]) assert.deepEqual(readFileSync(join(root, "oracle/dist", `${fixture.crate}${extension}`)), readFileSync(join(root, `dist/wasm-${profile}`, `${fixture.crate}${extension}`)), "Renderer bytes must match the independent native Trunk oracle");
  };
  console.log("[DEBUG] Native Trunk renderer fixture: cold compile, warm reuse and deleted-output restoration");
  let count = 0;
  for (const row of fixture.profiles) {
    await run(row.target); assert.equal(runs(), ++count);
    const published = join(root, row.output), bytes = files(published), wasm = join(published, `${fixture.crate}_bg.wasm`);
    assert.equal(await value(wasm), fixture.values[0]);
    assert.deepEqual([...bytes.keys()].filter(name => name !== ".nx-artifact.json").sort(), fixture.deliverables.map((suffix: string) => fixture.crate + suffix).sort(), "Publish each compiler artifact once without duplicate aliases");
    assert.ok(![...bytes.keys()].some(name => /plugin-modules|extension-modules|assets|target|staging/.test(name)));
    await run(row.target); assert.equal(runs(), count);
    rmSync(published, { recursive: true });
    await run(row.target); assert.equal(runs(), count);
    assert.deepEqual(files(published), bytes);
    assert.equal(await value(wasm), fixture.values[0]);
    await oracle(row.profile, fixture.values[0]);
    const fixtureConsumer = join(root, path.relative(workspace, consumerRoot)), fixtureRenderer = rendererDirectory(path, fixtureConsumer, row.profile);
    if (!existsSync(dirname(fixtureRenderer))) {
      mkdirSync(dirname(dirname(fixtureRenderer)), { recursive: true });
      symlinkSync(join(root, "dist"), dirname(fixtureRenderer), process.platform === "win32" ? "junction" : "dir");
    }
    const server = await require("vite").createServer({ configFile: false, root, publicDir: false, cacheDir: join(root, "state/vite"), resolve: { alias: [{ find: "/renderer-modules/wgpu", replacement: fixtureRenderer }] }, server: { port: 0, host: "127.0.0.1", watch: null, fs: { allow: [root] } }, logLevel: "silent" });
    try {
      await server.listen();
      const address = server.httpServer.address(), response = await fetch(`http://127.0.0.1:${address.port}/renderer-modules/wgpu/${fixture.crate}_bg.wasm`);
      assert.equal(response.status, 200);
      assert.deepEqual(Buffer.from(await response.arrayBuffer()), readFileSync(wasm), "The real Vite consumer must serve restored compiler bytes");
    } finally { await server.close(); }
  }
  put("🦀️.rs", fixture.rust.replace("VALUE", String(fixture.values[1])));
  await run("wasm"); assert.equal(runs(), ++count);
  const published = join(root, "dist/wasm-dev"), wasm = join(published, `${fixture.crate}_bg.wasm`);
  assert.equal(await value(wasm), fixture.values[1]);
  await oracle("dev", fixture.values[1]);
  assert.equal(existsSync(join(root, "forbidden")), false, "Ambient Trunk configuration must not redirect owned outputs");
  const retained = files(published);
  put("🦀️.rs", "compile_error!(\"preserve the last successful renderer\");\n");
  await assert.rejects(run("wasm"), /preserve the last successful renderer/);
  assert.deepEqual(files(published), retained);
  assert.ok(!existsSync(join(root, "state/staging")) || readdirSync(join(root, "state/staging")).length === 0);
  console.log("[DEBUG] Native Nx restores executable WGPU WASM outputs for both profiles, matches independent Trunk bytes, reuses warm results and serves restored bytes through Vite and preserves publication after compiler failure PASS");
  rmSync(root, { recursive: true, force: true });
}
