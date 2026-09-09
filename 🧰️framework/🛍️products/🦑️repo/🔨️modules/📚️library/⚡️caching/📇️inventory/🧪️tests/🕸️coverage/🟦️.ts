import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { createRequire } from "node:module";

/** 🕸️ Compares the entire audited surface with native Nx's resolved multi-provider graph. */
export async function testNativeInventory(workspace: string, inventory: (root: string) => any): Promise<any> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(schema);
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const { readCachedProjectGraph } = require("@nx/devkit"), graph = readCachedProjectGraph(), result = inventory(workspace);
  const { createTaskGraph } = require("nx/src/tasks-runner/create-task-graph"), { isCacheableTask } = require("nx/src/tasks-runner/utils");
  const names = Object.keys(graph.nodes).sort();
  assert.deepEqual(result.projects.map((project: any) => project.name).sort(), names, "Inventory must include every native Nx provider");
  for (const name of fixture.projects) assert.ok(names.includes(name), `Missing native project ${name}`);
  for (const tag of fixture.tags) assert.ok(result.projects.some((project: any) => project.tags?.includes(tag)), `Missing inferred ${tag} projects`);
  const commands = result.commands.filter((command: any) => command.target);
  assert.equal(commands.length, Object.values(graph.nodes).reduce((total: number, node: any) => total + Object.keys(node.data.targets ?? {}).length, 0));
  for (const command of commands) {
    const target = graph.nodes[command.project].data.targets[command.target];
    for (const property of fixture.properties) {
      const expected = target[property] ?? (property === "outputs" || property === "dependsOn" ? [] : property === "cache" || property === "continuous" ? false : undefined);
      assert.deepEqual(command[property], expected, `${command.project}:${command.target}.${property}`);
    }
    assert.ok(existsSync(join(workspace, command.file)), `Missing configuration source ${command.file}`);
    if (fixture.uncachedExecutors.includes(target.executor)) {
      const tasks = createTaskGraph(graph, {}, [command.project], [command.target], undefined, {}, true).tasks;
      assert.equal(isCacheableTask(tasks[`${command.project}:${command.target}`]), false, "Nx must execute publishing side effects on every requested run");
      assert.ok(!result.violations.some((finding: any) => finding.entry_point === `${command.project}:${command.target}` && finding.rule === "ORCH-01"), "A native Nx executor is already inside the required scheduler");
    }
  }
  console.log(`[DEBUG] Native Nx inventory covers all ${names.length} projects and ${commands.length} targets, preserving resolved settings and existing configuration sources PASS`);
  return result;
}
