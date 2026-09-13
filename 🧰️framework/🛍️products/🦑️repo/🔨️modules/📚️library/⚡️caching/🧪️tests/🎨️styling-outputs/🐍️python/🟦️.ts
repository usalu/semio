import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🐍️ Restores and imports the production wheel through native Nx, with an independent uv build oracle. */
export async function testStylingPythonOutputs(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const packagePath = join(fixture.owner, fixture.package), project = JSON.parse(readFileSync(join(workspace, packagePath, "📋️project.json"), "utf8"));
  assert.equal(project.targets.build.cache, true);
  assert.deepEqual(project.targets.build.outputs, [`{projectRoot}/${fixture.output}`]);
  assert.equal(project.targets.deps.cache, false);
  assert.ok(project.targets.build.dependsOn.includes("deps"));
  assert.ok(project.targets.build.dependsOn.includes("@semio-tech/ui-styling-tokens:generate"));
  assert.ok(project.targets.build.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
  const manifest = require("@iarna/toml").parse(readFileSync(join(workspace, packagePath, "pyproject.toml"), "utf8"));
  assert.deepEqual(manifest["build-system"].requires, [fixture.backend]);
  assert.deepEqual(manifest["dependency-groups"].build, [fixture.backend]);
  for (const file of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launch = require("jsonc-parser").parse(readFileSync(join(workspace, file), "utf8"));
    for (const name of Object.keys(project.targets)) assert.ok(launch.configurations.some((row: any) => row.command === `bun nx run ${fixture.project}:${name}`), `${file} must expose ${fixture.project}:${name}`);
  }
  const compiler = join(workspace, fixture.owner, "🏗️builder/🐍️python/📜️script.ts");
  const bundle = await require("esbuild").build({ entryPoints: [compiler], absWorkingDir: workspace, bundle: true, packages: "external", platform: "node", format: "esm", write: false, metafile: true });
  assert.ok(!Object.keys(bundle.metafile.inputs).some(path => path.endsWith("🎨️styling/🏗️builder/🟦️.ts")), "Wheel compilation must not import its source and wheel tests");
  const root = mkdtempSync(join(output, "styling-python-"));
  const put = (path: string, value: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), value); };
  for (const file of ["pyproject.toml", "uv.lock", "🎨️styling/__init__.py"]) put(join(packagePath, file), readFileSync(join(workspace, packagePath, file), "utf8"));
  put(join(fixture.owner, fixture.source), readFileSync(join(workspace, fixture.owner, fixture.source), "utf8"));
  put("🧫️tokens.py", readFileSync(join(workspace, fixture.owner, fixture.source), "utf8"));
  put("package.json", JSON.stringify({ name: "styling-python-fixture", private: true }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put(".gitignore", "node_modules\n.nx\ndist\nstate\n.venv\noracle\n.runs\n.deps\n.🧬semio\n");
  put(join(packagePath, "project.json"), JSON.stringify({ name: "styling", targets: {
    deps: { executor: "nx:run-commands", cache: false, outputs: [], options: { command: "bun ./📜️script.ts deps", cwd: "." } },
    generate: { executor: "nx:run-commands", cache: true, inputs: ["{workspaceRoot}/🧫️tokens.py", "{workspaceRoot}/📜️script.ts"], outputs: [`{workspaceRoot}/${fixture.owner}/${fixture.source}`], options: { command: "bun ./📜️script.ts generate", cwd: "." } },
    build: { executor: "nx:run-commands", cache: project.targets.build.cache, dependsOn: ["deps", "generate"], outputs: project.targets.build.outputs,
      inputs: ["{projectRoot}/pyproject.toml", "{projectRoot}/uv.lock", "{projectRoot}/🎨️styling/**/*", { dependentTasksOutputFiles: "**/*" }, "{workspaceRoot}/📜️script.ts"], options: { command: "bun ./📜️script.ts build", cwd: "." } }
  } }));
  put("📜️script.ts", `import { appendFileSync, copyFileSync } from "node:fs";
import { join } from "node:path";
import { StylingPythonBuildScript, StylingPythonDepsScript } from ${JSON.stringify(compiler)};
const root = process.cwd(), command = process.argv[2];
if (command === "generate") { copyFileSync(join(root, "🧫️tokens.py"), join(root, ${JSON.stringify(join(fixture.owner, fixture.source))})); process.exit(0); }
const Command = command === "deps" ? StylingPythonDepsScript : StylingPythonBuildScript;
await new Command(join(root, ${JSON.stringify(packagePath)}), root).run([]);
appendFileSync(join(root, command === "deps" ? ".deps" : ".runs"), command + "\\n");
`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const environment = join(root, ".venv"), env = { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), SEMIO_REPO_ROOT: root, SEMIO_STYLING_PYTHON_BUILD_ROOT: join(root, "state"), UV_PROJECT_ENVIRONMENT: environment };
  const execute = async (args: string[]) => {
    const child = Bun.spawn(args, { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    assert.equal(code, 0, stdout + stderr); return stdout;
  };
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js"), python = join(environment, process.platform === "win32" ? "Scripts/python.exe" : "bin/python");
  const run = () => execute(["node", cli, "run", "styling:build", "--outputStyle=static"]);
  const runs = () => readFileSync(join(root, ".runs"), "utf8").trim().split("\n").length;
  const published = join(root, packagePath, fixture.output), wheel = join(published, fixture.wheel);
  const value = (path: string) => execute([python, "-I", "-c", `import sys; from importlib import import_module; sys.path.insert(0, sys.argv[1]); print(import_module(${JSON.stringify(fixture.module)}).STYLING_TOKENS['primary'])`, path]);
  const check = async (expected: string) => {
    await execute(["uv", "build", "--wheel", "--no-build-isolation", "--offline", "--no-python-downloads", "--project", join(root, packagePath), "--python", python, "--out-dir", join(root, "oracle")]);
    assert.equal((await value(join(root, "oracle", fixture.wheel))).trim(), expected);
    assert.equal((await value(wheel)).trim(), expected);
    assert.equal(readFileSync(wheel).equals(readFileSync(join(root, "oracle", fixture.wheel))), true, "The published wheel must equal native uv's independent build byte for byte");
  };
  console.log("[DEBUG] Styling Python fixture: preparing locked tools and building the native wheel");
  await run(); assert.equal(runs(), 1); await check(fixture.values[0]);
  assert.equal((await execute([python, "-I", "-c", "from importlib.metadata import distributions; assert not any(d.metadata['Name'] == 'semio-framework-ui-styling' for d in distributions()); print('preparation contains no first-party editable build')"])).trim(), "preparation contains no first-party editable build");
  const bytes = new Map(readdirSync(published).map(name => [name, readFileSync(join(published, name))]));
  assert.deepEqual([...bytes.keys()].sort(), [".nx-artifact.json", fixture.wheel].sort());
  await run(); assert.equal(runs(), 1);
  rmSync(published, { recursive: true });
  await run(); assert.equal(runs(), 1);
  assert.deepEqual(new Map(readdirSync(published).map(name => [name, readFileSync(join(published, name))])), bytes);
  await check(fixture.values[0]);
  put("🧫️tokens.py", readFileSync(join(root, "🧫️tokens.py"), "utf8").replace(fixture.values[0], fixture.values[1]));
  await run(); assert.equal(runs(), 2); await check(fixture.values[1]);
  assert.equal(readFileSync(join(root, ".deps"), "utf8").trim().split("\n").length, 4);
  const retained = new Map(readdirSync(published).map(name => [name, readFileSync(join(published, name))]));
  put(join(packagePath, "pyproject.toml"), readFileSync(join(root, packagePath, "pyproject.toml"), "utf8").replace("../../🔤️tokens/🐍️.py", "../../missing.py"));
  await assert.rejects(execute(["bun", join(root, "📜️script.ts"), "build"]), /styling-python-build failed/);
  assert.deepEqual(new Map(readdirSync(published).map(name => [name, readFileSync(join(published, name))])), retained, "A failed backend must preserve the previous complete wheel publication");
  assert.ok(!existsSync(join(root, "state/staging")) || readdirSync(join(root, "state/staging")).length === 0);
  console.log("[DEBUG] Styling Python native Nx cold/warm/restoration/source-change wheels execute and match independent uv bytes; failed builds preserve published bytes and preparation performs no first-party build PASS");
  rmSync(root, { recursive: true, force: true });
}
