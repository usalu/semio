import { test, expect } from "bun:test";
import { createRequire } from "node:module";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, resolve, join } from "node:path";
import { pathToFileURL } from "node:url";

type Phase = { file: string; projects: Record<string, string>; packages: Record<string, string>; locked: Record<string, string> };
const caching = resolve(import.meta.dir, "../../../.."), workspace = process.cwd();
const fixture = JSON.parse(readFileSync(join(caching, "🧫️fixtures/import-edges/🔁️context/🔣️.json"), "utf8")) as { graphAuthority: { module: string; export: string }; contexts: { id: string; phases: Phase[] }[]; lockScopes: { id: string; file: string; source: string; before: string; after: string; beforeTarget: string; afterTarget: string }[]; authorityInputs: { id: string; files: Record<string, string>; path: string; after: string }[] };

test("retained Nx reuse admits only current roots manifests lock scope and implementation authority", async () => {
  const nxJson = JSON.parse(readFileSync(join(workspace, "nx.json"), "utf8"));
  expect(nxJson.pluginsConfig["@repo/emoji-project-json"]?.dependencyResolutionAuthority).toEqual(fixture.graphAuthority);
  const owner = await import(pathToFileURL(join(workspace, fixture.graphAuthority.module)).href);
  expect(typeof owner[fixture.graphAuthority.export]).toBe("function");
  const selectedManifest = existsSync(join(workspace, ".nx/installation/package.json")) ? join(workspace, ".nx/installation/package.json") : join(workspace, "package.json");
  const require = createRequire(selectedManifest), { createProjectFileMapCache, shouldRecomputeWholeGraph } = require("nx/src/project-graph/nx-deps-cache");
  const artifact = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifact) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(artifact, { recursive: true });
  const root = mkdtempSync(join(artifact, "retained-context-authority-"));
  try {
    for (const vector of fixture.contexts) {
      const directory = join(root, vector.id);
      let previous: string | undefined;
      for (const phase of vector.phases) {
        const projects = Object.fromEntries(Object.entries(phase.projects).map(([name, root]) => [name, { root }]));
        for (const [name, project] of Object.entries(projects)) {
          const manifest = join(directory, project.root, "package.json");
          mkdirSync(dirname(manifest), { recursive: true });
          writeFileSync(manifest, JSON.stringify({ name: Object.entries(phase.packages).find(([, target]) => target === name)?.[0] ?? `__unused_${name}` }));
        }
        writeFileSync(join(directory, "bun.lock"), JSON.stringify({ lockfileVersion: 3, workspaces: { "": { name: "authority-fixture" } }, packages: Object.fromEntries(Object.entries(phase.locked).map(([name, version]) => [name, [`${name}@${version}`, "", {}]])) }));
        const authority: string = await owner[fixture.graphAuthority.export](directory, projects);
        expect(authority).toMatch(/^[a-f0-9]{64}$/);
        expect(await owner[fixture.graphAuthority.export](directory, structuredClone(projects))).toBe(authority);
        const fileMap = { projectFileMap: Object.fromEntries(Object.keys(projects).map(name => [name, []])), nonProjectFiles: [] }, cache = createProjectFileMapCache(nxJson, {}, fileMap, {}, "same-external-node-identity", previous ?? authority);
        expect(shouldRecomputeWholeGraph(cache, {}, projects, nxJson, {}, "same-external-node-identity", authority), vector.id).toBe(previous !== undefined && previous !== authority);
        if (previous !== undefined) expect(authority === previous, vector.id).toBe(vector.id === "old-resolved-cache-isolation");
        previous = authority;
      }
    }
    const { cacheInternals } = await import("../../../../../🟨️.mjs");
    const { TargetProjectLocator } = require("nx/src/plugins/js/project-graph/build-dependencies/target-project-locator");
    for (const row of fixture.lockScopes) {
      const directory = join(root, row.id), source = join(directory, row.file), projects = { caller: { root: dirname(row.file) } };
      mkdirSync(dirname(source), { recursive: true });
      writeFileSync(source, row.source);
      let previous: string | undefined, externalIdentity: string | undefined;
      for (const [index, text] of [row.before, row.after].entries()) {
        writeFileSync(join(directory, "bun.lock"), text);
        const lock = cacheInternals.bunLockGraph(JSON.parse(text)), target = index === 0 ? row.beforeTarget : row.afterTarget;
        const identity = JSON.stringify(lock.externalNodes);
        if (externalIdentity !== undefined) expect(identity).toBe(externalIdentity);
        externalIdentity = identity;
        expect(`npm:${lock.resolveImport(row.file, "semio-import-fixture-lock")}`).toBe(target);
        const selected = lock.externalNodes[target], packageFile = join(dirname(source), "node_modules/semio-import-fixture-lock/package.json");
        mkdirSync(dirname(packageFile), { recursive: true });
        writeFileSync(packageFile, JSON.stringify({ name: selected.data.packageName, version: selected.data.version, main: "index.js" }));
        const locator = new TargetProjectLocator({}, lock.externalNodes, new Map(), new Map());
        expect(locator.findProjectFromImport("semio-import-fixture-lock", source)).toBe(target);
        const authority: string = await owner[fixture.graphAuthority.export](directory, projects);
        const cache = createProjectFileMapCache(nxJson, {}, { projectFileMap: { caller: [] }, nonProjectFiles: [] }, {}, externalIdentity, previous ?? authority);
        expect(shouldRecomputeWholeGraph(cache, {}, projects, nxJson, {}, externalIdentity, authority)).toBe(previous !== undefined);
        if (previous !== undefined) expect(authority).not.toBe(previous);
        previous = authority;
      }
    }
    for (const row of fixture.authorityInputs) {
      const directory = join(root, row.id), projects = { caller: { root: "caller" } };
      for (const [path, source] of Object.entries(row.files)) { const file = join(directory, path); mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, source); }
      const before = await owner[fixture.graphAuthority.export](directory, projects);
      writeFileSync(join(directory, row.path), row.after);
      const after = await owner[fixture.graphAuthority.export](directory, projects);
      expect(after, row.id).not.toBe(before);
      const cache = createProjectFileMapCache(nxJson, {}, { projectFileMap: { caller: [] }, nonProjectFiles: [] }, {}, "same-external-node-identity", before);
      expect(shouldRecomputeWholeGraph(cache, {}, projects, nxJson, {}, "same-external-node-identity", after), row.id).toBe(true);
    }
    console.log(`[DEBUG] retained Nx context authority: ${fixture.contexts.length} replay pairs plus ${fixture.lockScopes.length} source-scoped locks and ${fixture.authorityInputs.length} immutable recipe inputs with unchanged external nodes; actual selected Nx cache admission`);
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 120000);
