import assert from "node:assert/strict";
import { existsSync, mkdtempSync, mkdirSync, readFileSync, cpSync, rmSync } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";

/** 🔗️ Fixture + repo import edges from the full scan equal the incremental filesToProcess path. */
export async function testImportEdgeEquality(workspace: string, output: string): Promise<void> {
  const { cacheInternals } = await import("../../../🟨️.mjs");
  const fixtureRoot = join(import.meta.dir, "../../🧫️fixtures/import-edges");
  const cases = JSON.parse(readFileSync(join(fixtureRoot, "cases.json"), "utf8"));
  const root = mkdtempSync(join(output, "import-edges-"));
  const dataDir = join(root, ".nx/workspace-data");
  mkdirSync(dataDir, { recursive: true });
  process.env.NX_WORKSPACE_DATA_DIRECTORY = dataDir;
  const key = (edge: { source: string; target: string; sourceFile?: string; type: string }) =>
    `${edge.source}\0${edge.target}\0${edge.sourceFile ?? ""}\0${edge.type}`;
  try {
    for (const name of ["a", "b"]) cpSync(join(fixtureRoot, name), join(root, name), { recursive: true });
    const projects = {
      "fixture-a": { name: "fixture-a", root: "a", targets: {} },
      "fixture-b": { name: "fixture-b", root: "b", targets: {} },
    };
    const hash = (file: string) => createHash("sha256").update(readFileSync(join(root, file))).digest("hex");
    const fileMap = {
      projectFileMap: {
        "fixture-a": [
          { file: "a/package.json", hash: hash("a/package.json") },
          { file: "a/index.js", hash: hash("a/index.js") },
          { file: "a/deep/util.js", hash: hash("a/deep/util.js") },
        ],
        "fixture-b": [
          { file: "b/package.json", hash: hash("b/package.json") },
          { file: "b/lib.js", hash: hash("b/lib.js") },
        ],
      },
      nonProjectFiles: [],
    };
    const context = {
      workspaceRoot: root,
      projects,
      fileMap,
      filesToProcess: fileMap,
      externalNodes: {},
      nxJsonConfiguration: {},
    };
    const full = await cacheInternals.createDependenciesImplementation({ analyzeLockfile: false }, context);
    const fullKeys = new Set(full.map(key));
    for (const expected of cases.expectedEdges) assert.ok(fullKeys.has(key(expected)), JSON.stringify(expected));

    const again = await cacheInternals.createDependenciesImplementation({ analyzeLockfile: false }, context);
    assert.deepEqual([...again.map(key)].sort(), [...full.map(key)].sort());

    const utilOnly = {
      ...context,
      filesToProcess: {
        projectFileMap: { "fixture-a": [{ file: "a/deep/util.js", hash: hash("a/deep/util.js") }] },
        nonProjectFiles: [],
      },
    };
    const partial = await cacheInternals.createDependenciesImplementation({ analyzeLockfile: false }, utilOnly);
    assert.ok(partial.some((edge) => edge.source === "fixture-a" && edge.target === "fixture-b" && edge.sourceFile === "a/deep/util.js"));

    const pgPath = join(workspace, ".tmp-ticket/wp-o2c/generated/nx-iso3/ws-data/project-graph.json");
    const fmPath = join(workspace, ".tmp-ticket/wp-o2c/generated/nx-iso3/ws-data/file-map.json");
    assert.ok(existsSync(pgPath) && existsSync(fmPath), "repo file-map/project-graph snapshots required for equality");
    const pg = JSON.parse(readFileSync(pgPath, "utf8"));
    const fm = JSON.parse(readFileSync(fmPath, "utf8"));
    const nodes = pg.nodes || pg.graph?.nodes || {};
    const repoProjects = Object.fromEntries(Object.entries(nodes).map(([name, node]: any) => [name, { name, root: node.data.root, targets: node.data.targets || {} }]));
    const fileMapRepo = fm.fileMap || fm;
    const repoData = join(output, "import-edges-repo-cache");
    rmSync(repoData, { recursive: true, force: true });
    mkdirSync(repoData, { recursive: true });
    process.env.NX_WORKSPACE_DATA_DIRECTORY = repoData;
    const repoContext = {
      workspaceRoot: workspace,
      projects: repoProjects,
      fileMap: fileMapRepo,
      filesToProcess: fileMapRepo,
      externalNodes: {},
      nxJsonConfiguration: {},
    };
    const cold = await cacheInternals.createDependenciesImplementation({ analyzeLockfile: false }, repoContext);
    const warm = await cacheInternals.createDependenciesImplementation({ analyzeLockfile: false }, repoContext);
    assert.deepEqual([...warm.map(key)].sort(), [...cold.map(key)].sort(), "warm hash cache must preserve the full edge set");

    const byPackage = new Map<string, string>();
    for (const [name, project] of Object.entries(repoProjects) as [string, { root: string }][]) {
      const packageFile = join(workspace, project.root, "package.json");
      if (!existsSync(packageFile)) continue;
      try {
        const manifest = JSON.parse(readFileSync(packageFile, "utf8"));
        if (manifest.name) byPackage.set(manifest.name, name);
      } catch { /* ignore malformed manifests in snapshot */ }
    }
    const importOnly = new Map<string, { source: string; target: string; sourceFile: string; type: string }>();
    const add = (source: string, target: string | undefined, sourceFile: string) => {
      if (target && source !== target) importOnly.set(`${source}\0${target}\0${sourceFile}`, { source, target, sourceFile, type: "static" });
    };
    await cacheInternals.collectImportEdges(workspace, fileMapRepo.projectFileMap, repoProjects, byPackage, undefined, add);
    const fromCreate = cold.filter((edge) => /\.[cm]?[jt]sx?$/.test(edge.sourceFile ?? ""));
    assert.deepEqual([...fromCreate.map(key)].sort(), [...importOnly.values()].map(key).sort(), "createDependencies import edges must equal full collectImportEdges scan");
    console.log(`[DEBUG] Import edge equality PASS fixture+repo cold=${cold.length} import=${importOnly.size}`);
  } finally {
    delete process.env.NX_WORKSPACE_DATA_DIRECTORY;
    rmSync(root, { recursive: true, force: true });
  }
}
