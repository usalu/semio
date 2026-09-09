import { existsSync, realpathSync } from "node:fs";
import { createRequire } from "node:module";
import { relative, resolve, sep } from "node:path";

export type InventoryProject = { name: string; root: string; targets: Record<string, any>; namedInputs?: Record<string, any>; [key: string]: any };
export type InventoryGraph = { projects: InventoryProject[]; sources: Record<string, Record<string, string>> };

/** 🧭️ Reads the graph already resolved by the outer Nx invocation, with every provider and target's provenance. */
export function readInventoryGraph(workspace: string): InventoryGraph {
  const require = createRequire(import.meta.url), root = realpathSync(workspace);
  if (root !== realpathSync(require("nx/src/utils/workspace-root").workspaceRoot)) throw new Error("Inventory must use the active Nx workspace");
  const graph = require("@nx/devkit").readCachedProjectGraph(), sourceMaps = require("nx/src/project-graph/nx-deps-cache").readSourceMapsCache();
  if (!sourceMaps) throw new Error("Nx configuration provenance is unavailable; run the audit through Nx");
  const projects: InventoryProject[] = Object.values(graph.nodes).map((node: any) => ({ ...node.data, targets: node.data.targets ?? {} }));
  projects.sort((a, b) => a.name < b.name ? -1 : a.name > b.name ? 1 : 0);
  const sources: InventoryGraph["sources"] = {};
  for (const project of projects) {
    const map = sourceMaps[project.root] ?? {}, targets: Record<string, string> = {};
    for (const name of Object.keys(project.targets)) {
      const prefix = `targets.${name}`;
      const candidates = [`${prefix}.options.command`, `${prefix}.executor`, prefix, "targets", "name", "root"].map(key => map[key]?.[0]);
      const source = candidates.find(path => typeof path === "string" && existsSync(resolve(root, path)));
      if (!source) throw new Error(`Nx configuration source is missing for ${project.name}:${name}`);
      const path = relative(root, resolve(root, source)).split(sep).join("/");
      if (path === ".." || path.startsWith("../")) throw new Error(`Nx configuration source escapes the workspace: ${source}`);
      targets[name] = path;
    }
    sources[project.name] = targets;
  }
  return { projects, sources };
}
