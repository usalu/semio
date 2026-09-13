import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { BundleScript } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import plugin from "../../../🟨️.mjs";
import type { InventoryProject } from "../../📇️inventory/🟦️.ts";
import { sourceFiles } from "../../🔍️discovery/📂️source/🟦️.ts";


export class GraphScript extends BundleScript {
  run(): void {
    const { readCachedProjectGraph } = createRequire(import.meta.url)("@nx/devkit");
    const graph = readCachedProjectGraph();
    const edges = Object.values(graph.dependencies).flat() as any[];
    assert.ok(edges.length > 0, "A monorepo graph must contain dependency edges");
    for (const edge of edges) assert.ok(graph.nodes[edge.target] || graph.externalNodes?.[edge.target], `Unknown dependency ${edge.target}`);
    const files = sourceFiles(this.repoRoot);
    const declared = (plugin.createNodesV2 as any)[1](files.filter(file => file.endsWith("📋️project.json") || file.endsWith("Cargo.toml")), {}, { workspaceRoot: this.repoRoot }).flatMap(([, result]: any) => Object.values(result.projects)) as InventoryProject[];
    for (const project of declared) {
      const resolved = graph.nodes[project.name]?.data;
      assert.ok(resolved, `Nx omitted ${project.name}`);
      for (const [name, target] of Object.entries(project.targets)) {
        const actual = resolved.targets?.[name];
        assert.ok(actual, `Nx omitted ${project.name}:${name}`);
        for (const property of ["cache", "outputs", "dependsOn"]) if (Object.hasOwn(target, property)) assert.deepEqual(actual[property], target[property], `Nx precedence changed ${project.name}:${name}.${property}`);
      }
    }
    console.log(`[nx-graph] projects=${Object.keys(graph.nodes).length} edges=${edges.length}`);
  }
}
