import { test, expect } from "bun:test";
import { createRequire } from "node:module";
import { existsSync, readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { resolve, join } from "node:path";
import { createHash } from "node:crypto";

const caching = resolve(import.meta.dir, "../../.."), workspace = process.cwd();
const fixture = JSON.parse(readFileSync(join(caching, "🧫️fixtures/import-edges/🔁️context/🔣️.json"), "utf8")) as { repository: { owner: string; sources: string[] } };
type Edge = { source: string; target: string; type: string; sourceFile?: string };
type File = { file: string; hash: string; deps?: (string | string[])[] };

/** 🏛️ Captures the ordinary published graph and rebuilds canonical dependencies without using retained resolved edges. */
async function graphEpoch() {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true });
  const directory = process.env.NX_WORKSPACE_DATA_DIRECTORY ?? join(workspace, ".nx/workspace-data");
  const graphSource = readFileSync(join(directory, "project-graph.json"), "utf8"), mapSource = readFileSync(join(directory, "file-map.json"), "utf8");
  const graph = JSON.parse(graphSource), fileMap = JSON.parse(mapSource).fileMap;
  const projects = Object.fromEntries(Object.entries(graph.nodes).map(([name, node]: any) => [name, { ...node.data, name }]));
  const files = Object.fromEntries(Object.entries(fileMap.projectFileMap).map(([name, rows]: [string, any]) => [name, rows.map(({ file, hash }: File) => ({ file, hash }))]));
  const { cacheInternals } = await import("../../../../🟨️.mjs");
  const fresh: Edge[] = await cacheInternals.createDependenciesImplementation({ analyzeLockfile: true }, { workspaceRoot: workspace, projects, fileMap: { projectFileMap: files, nonProjectFiles: [] }, filesToProcess: { projectFileMap: files, nonProjectFiles: [] }, externalNodes: graph.externalNodes, nxJsonConfiguration: JSON.parse(readFileSync(join(workspace, "nx.json"), "utf8")) });
  const selectedManifest = existsSync(join(workspace, ".nx/installation/package.json")) ? join(workspace, ".nx/installation/package.json") : join(workspace, "package.json");
  const require = createRequire(selectedManifest), { getPluginsSeparated } = require("nx/src/project-graph/plugins/get-plugins"), { buildProjectGraphUsingProjectFileMap } = require("nx/src/project-graph/build-project-graph");
  const nxJson = JSON.parse(readFileSync(join(workspace, "nx.json"), "utf8")), plugins = await getPluginsSeparated(nxJson, workspace);
  const roots = Object.fromEntries(Object.values(projects).map((project: any) => [project.root, project]));
  const built = await buildProjectGraphUsingProjectFileMap(roots, graph.externalNodes, { projectFileMap: files, nonProjectFiles: fileMap.nonProjectFiles.map(({ file, hash }: File) => ({ file, hash })) }, [], null, [...plugins.specifiedPlugins, ...plugins.defaultPlugins], {});
  const canonical = built.projectGraph, admittedCache = JSON.stringify(built.projectFileMapCache);
  const warm = await buildProjectGraphUsingProjectFileMap(roots, graph.externalNodes, built.projectFileMapCache.fileMap, [], built.projectFileMapCache, [...plugins.specifiedPlugins, ...plugins.defaultPlugins], {});
  expect(JSON.stringify(built.projectFileMapCache), "admitted cache records remain immutable under shared file-map aliasing").toBe(admittedCache);
  const key = (edge: Edge) => `${edge.source}\0${edge.target}\0${edge.type}`;
  const actual = new Set<string>((Object.values(graph.dependencies).flat() as Edge[]).map(key)), expected = new Set<string>((Object.values(canonical.dependencies).flat() as Edge[]).map(key));
  const warmEdges = new Set<string>((Object.values(warm.projectGraph.dependencies).flat() as Edge[]).map(key));
  expect([...warmEdges].sort(), "unchanged-authority warm graph equals fresh source resolution").toEqual([...expected].sort());
  console.log(`[DEBUG] unchanged-authority Nx warm graph: ${warmEdges.size} edges; admitted cache remains immutable under shared input aliasing`);
  const excess = [...actual].filter(edge => !expected.has(edge)).sort(), missing = [...expected].filter(edge => !actual.has(edge)).sort();
  if (excess.length || missing.length) console.log(`[DEBUG] graph mismatch authority: ${JSON.stringify({ selectedManifest, pluginNames: [...plugins.specifiedPlugins, ...plugins.defaultPlugins].map(plugin => plugin.name), freshWs: fresh.filter(edge => edge.target === "npm:ws"), canonicalWs: Object.values(canonical.dependencies).flat().filter((edge: any) => edge.target === "npm:ws") })}`);
  const selected = fixture.repository.sources.map(source => {
    const row: File | undefined = fileMap.projectFileMap[fixture.repository.owner]?.find((file: File) => file.file === source);
    if (!row) throw Error(`Current ordinary Nx file map is missing ${source}`);
    const actual = [...new Set((row.deps ?? []).map(dep => typeof dep === "string" ? dep : dep[0]))].sort();
    const expected = [...new Set(fresh.filter(edge => edge.source === fixture.repository.owner && edge.sourceFile === source).map(edge => edge.target))].sort();
    return { source, hash: row.hash, actual, expected };
  });
  if (readFileSync(join(directory, "project-graph.json"), "utf8") !== graphSource || readFileSync(join(directory, "file-map.json"), "utf8") !== mapSource) throw Error("Ordinary Nx graph epoch changed during the canonical comparison");
  writeFileSync(join(output, "ordinary-retained-graph-equality.json"), JSON.stringify({ publishedGraphHash: createHash("sha256").update(graphSource).digest("hex"), publishedMapHash: createHash("sha256").update(mapSource).digest("hex"), selected, actualEdges: actual.size, canonicalEdges: expected.size, excess, missing }, null, 2));
  return { selected, excess, missing, actualEdges: actual.size, canonicalEdges: expected.size };
}

let epoch: ReturnType<typeof graphEpoch> | undefined;
const current = () => epoch ??= graphEpoch();

test("ordinary Nx retained Process sources agree with canonical current dependency authority", async () => {
  const { selected } = await current();
  console.log(`[DEBUG] ordinary retained Process source authority: ${JSON.stringify(selected)}`);
  for (const row of selected) expect(row.actual, row.source).toEqual(row.expected);
}, 120000);

test("ordinary Nx published dependency graph equals the full canonical current graph", async () => {
  const { excess, missing, actualEdges, canonicalEdges } = await current();
  console.log(`[DEBUG] ordinary Nx canonical graph equality: actual ${actualEdges}, canonical ${canonicalEdges}, excess ${excess.length}, missing ${missing.length}`);
  expect({ excess, missing }).toEqual({ excess: [], missing: [] });
}, 120000);
