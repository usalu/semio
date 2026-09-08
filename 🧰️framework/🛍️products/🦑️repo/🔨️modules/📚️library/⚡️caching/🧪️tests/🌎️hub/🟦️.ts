import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createRequire } from "node:module";

/** 🌎️ Uses native Nx task construction and esbuild's import graph to verify the Hub build boundary. */
export async function testHubBuild(workspace: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), directory = resolve(import.meta.dirname, "../../🧫️fixtures/hub-build");
  const fixture = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
  assert.equal(require("jsonschema").validate(fixture, JSON.parse(readFileSync(join(directory, "🛂️schema/🔣️.json"), "utf8"))).valid, true);
  const project = JSON.parse(readFileSync(join(workspace, fixture.projectRoot, "📋️project.json"), "utf8")), build = project.targets.build;
  assert.deepEqual(build.outputs, [fixture.output]);
  assert.equal(build.cache, true); assert.equal(build.options.cwd, ".");
  const command = ["bun", JSON.stringify(fixture.entry), ...fixture.arguments.map((argument: string) => argument.includes("/") ? JSON.stringify(argument) : argument)].join(" ");
  assert.equal(build.options.command, command);
  const admin = fixture.prerequisite.split(":")[0];
  const graph = { nodes: {
    [project.name]: { name: project.name, type: "app", data: { ...project, root: fixture.projectRoot } },
    [admin]: { name: admin, type: "app", data: { name: admin, root: fixture.prerequisiteRoot, targets: { build: { executor: "nx:run-commands", options: { command: "bun ./📜️script.ts build" } } } } },
  }, dependencies: { [project.name]: [], [admin]: [] } };
  const tasks = require("nx/src/tasks-runner/create-task-graph").createTaskGraph(graph, {}, [project.name], ["build"], undefined, {});
  assert.deepEqual(Object.keys(tasks.tasks).sort(), [`${project.name}:build`, fixture.prerequisite].sort());
  assert.deepEqual(tasks.dependencies[`${project.name}:build`], [fixture.prerequisite]);
  const script = readFileSync(join(workspace, fixture.projectRoot, "📜️script.ts"), "utf8"), ts = require("typescript");
  const source = ts.createSourceFile("📜️script.ts", script, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const launch = ts.parseConfigFileTextToJson("launch.seed.jsonc", readFileSync(join(workspace, ".vscode/🧩️launch.seed.jsonc"), "utf8"));
  assert.equal(launch.error, undefined);
  for (const target of [`${fixture.project}:build`, fixture.prerequisite]) assert.ok(launch.config.configurations.some((entry: any) => entry.command === `bun nx run ${target}`), `Missing editor build command: ${target}`);
  assert.ok(!source.statements.some((node: any) => ts.isClassDeclaration(node) && node.name?.text === "BuildScript"), "Hub must not retain a second build implementation");
  const imports = await require("esbuild").build({ absWorkingDir: workspace, entryPoints: [fixture.entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
  assert.ok(!Object.keys(imports.metafile.inputs).some(path => path.startsWith("🌎️hub/")), "The native producer must not eagerly import the Hub application/test script");
  const config = ts.createSourceFile("⚙️vite.config.ts", readFileSync(join(workspace, fixture.prerequisiteRoot, "⚙️vite.config.ts"), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const definition = config.statements.find((node: any) => ts.isExportAssignment(node))?.expression.arguments?.[0];
  const properties = definition?.properties?.find((node: any) => node.name?.text === "define")?.initializer.properties ?? [];
  const define = Object.fromEntries(properties.map((node: any) => [node.name.text, node.initializer.text]));
  const browser = await require("esbuild").build({ stdin: { contents: fixture.browserProbe, sourcefile: "browser-probe.ts", loader: "ts" }, define, bundle: true, write: false, platform: "browser", format: "esm", logLevel: "silent" });
  assert.equal((await import("data:text/javascript;base64," + Buffer.from(browser.outputFiles[0].text).toString("base64"))).value, 42);
  console.log("[DEBUG] Hub build uses one native producer, one outer admin prerequisite and one deliverable owner PASS");
}
