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
  assert.ok(!source.statements.some((node: any) => ts.isClassDeclaration(node) && node.name?.text === "BuildScript"), "Hub must not retain a second build implementation");
  const imports = await require("esbuild").build({ absWorkingDir: workspace, entryPoints: [fixture.entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
  assert.ok(!Object.keys(imports.metafile.inputs).some(path => path.startsWith("🌎️hub/")), "The native producer must not eagerly import the Hub application/test script");
  console.log("[DEBUG] Hub build uses one native producer, one outer admin prerequisite and one deliverable owner PASS");
}
