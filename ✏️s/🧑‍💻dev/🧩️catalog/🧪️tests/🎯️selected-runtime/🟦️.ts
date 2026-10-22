import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createRequire} from "node:module";
import {join} from "node:path";
/** 🎯️ Checks selected app prerequisites against neutral cases and the installed Nx scheduler. */
export function testSelectedRuntimeDependencies(targets: Record<string, any>, graph?: any): void {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "./🧫️fixtures/🔣️.json"), "utf8"));
  const owner = "@semio-tech/framework-os-dev", rendererProject = "@semio-tech/framework-renderer-wgpu";
  for (const row of fixture.variants) for (const profile of fixture.profiles) for (const renderer of fixture.renderers) {
    const name = `prepare-${row.variant}-${renderer}-${profile}`, target = targets[name];
    assert.ok(target, name);
    const engines = target.dependsOn.filter((id: string) => /:wasm(?:-release)?$/.test(id));
    const expected = [...fixture.sharedEngines,...(renderer === "react" ? row.engines : [`${rendererProject}:wasm`])];
    assert.deepEqual(require("lodash").sortBy(engines), require("lodash").sortBy(expected), name + ": only declared runtime engines");
    if (!graph) continue;
    const tasks = createRequire(join(process.cwd(), ".nx/installation/package.json"))("nx/src/tasks-runner/create-task-graph").createTaskGraph(graph, {}, [owner], [`dev-${row.variant}-${renderer}-${profile}`], undefined, {}, false);
    const ids = Object.keys(tasks.tasks);
    if (renderer === "react") assert.ok(!ids.some(id => id.startsWith(rendererProject + ":")), name + ": React must not compile the WGPU renderer");
    const fontTask = fixture.fontTool.project + ":build";
    assert.ok(ids.includes(fontTask), name + ": standalone font prerequisite");
    assert.ok(!ids.some(id => /^semio-framework-os-infinite:(?:build|font-tool)$/.test(id)), name + ": fonts must not compile Infinite");
    for (const forbidden of row.forbiddenProjects) assert.ok(!ids.some(id => id === forbidden + ":wasm" || id === forbidden + ":wasm-release"), name + ": unwanted engine " + forbidden);
  }
}

