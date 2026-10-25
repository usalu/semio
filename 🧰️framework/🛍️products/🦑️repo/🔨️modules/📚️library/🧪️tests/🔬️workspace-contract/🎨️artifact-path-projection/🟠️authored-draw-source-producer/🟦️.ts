/** 🟠️ The authored draw source producer tests of the artifact path projection authority suite. */
import toml from "@iarna/toml";
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join, posix, relative, resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, validateTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { inspectRustModuleGraphFacts } from "../../../../🔍️discovery/🌐️rust-module-graph/🟦️.ts";
import { semanticPathProjectionAuthority } from "../../../../🔍️discovery/🪞️path-projection/🟦️.ts";
import { type Taxonomy } from "../../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { ARTIFACT_PROJECTION_GOLDEN, ARTIFACT_PROJECTION_RUN_PARENT, type ArtifactProducerInput, DRAW_SOURCE_SCENARIO, DRAW_SOURCE_SCENARIO_ROOT, type DrawSourceScenario, NORMALIZATION_RUN_DIRECTORY, NORMALIZATION_RUN_ROOT, NORMALIZATION_SCHEMA_REL, artifactProjectionFixtureLocations, artifactProjectionFixturePath, artifactProjectionProducerInputs, artifactProjectionSourceFiles, drawSourceScenarioContent, normalizationRetainedRunParent, projectionAuthorityNodes, projectionGolden } from "../../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../../🏗️shared-fixtures/🟦️.ts";
import { projectionAuthority, projectionTaxonomy } from "../🟣️harness/🟦️.ts";

/** 🟡 Registers the authored draw source producer tests of the artifact path projection authority suite. */
export function registerAuthoredDrawSourceProducerTests(): void {
  test("authored Draw source producer retains imported JSON bytes and rejects invalid data", () => {
    const vector = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🌐️registry-import-language/🧪️imported-data/🔣️.json"), "utf8")) as { graph: { entries: string[]; dataPath: string; files: { path: string; content: string; mode: number }[] } };
    const taxonomy = JSON.parse(readFileSync(join(getWorkspaceRoot(), NORMALIZATION_SCHEMA_REL), "utf8")) as Taxonomy;
    Object.assign(taxonomy.generatorContracts["plugin-registry"]!.inputDiscovery!, { implementationEntryPaths: vector.graph.entries, workspaceImports: {} });
    const source = new Map(vector.graph.files.map((row) => [row.path, row])), reads: string[] = [];
    const read = (path: string): ArtifactProducerInput => {
      reads.push(path);
      const row = source.get(path);
      if (!row && ![...DRAW_SOURCE_SCENARIO.producerContext.workspaceInputs, ...taxonomy.generatorContracts["plugin-registry"]!.inputPatterns].includes(path)) throw new Error("Missing authored compiler input: " + path);
      const content = row?.content ?? (path === "nx.json" ? '{"plugins":[]}' : "{}"), mode = row?.mode ?? 0o644;
      return { content, mode, sha256: createHash("sha256").update(content).digest("hex"), origin: "authored-scenario" };
    };
    const context = { ...DRAW_SOURCE_SCENARIO.producerContext, compilerRoots: [], runtimeModules: [], runtimeData: [], runtimeReceiptCatalogs: [] };
    const captured = artifactProjectionProducerInputs(taxonomy, read, context);
    const data = source.get(vector.graph.dataPath)!;
    expect(captured.files[data.path]).toEqual({ content: data.content, mode: data.mode, sha256: createHash("sha256").update(data.content).digest("hex"), origin: "authored-scenario" });
    expect(captured.modules.find(({ path }) => path === data.path)).toEqual({ path: data.path, kind: "json-data", imports: [] });
    expect(reads.filter((path) => path === data.path)).toHaveLength(1);
    expect(reads.some((path) => path.includes("never.ts"))).toBe(false);
    source.set(data.path, { ...data, content: "{" });
    expect(() => artifactProjectionProducerInputs(taxonomy, read, context)).toThrow("imported JSON is invalid");
    source.delete(data.path);
    expect(() => artifactProjectionProducerInputs(taxonomy, read, context)).toThrow("Missing authored compiler input");
    source.set(data.path.replace(".json", ".txt"), { ...data, path: data.path.replace(".json", ".txt") });
    const entry = source.get(vector.graph.entries[0]!)!;
    source.set(entry.path, { ...entry, content: entry.content.replaceAll(".json", ".txt") });
    expect(() => artifactProjectionProducerInputs(taxonomy, read, context)).toThrow("language is not supported");
  });
  test("authored Draw source authority never reads live Draw leaves or reconstructs canonical input", () => {
    const projection = projectionGolden("artifact-editor-command-bundle-v1");
    const nodes = projectionAuthorityNodes(projection, "authored-draw", () => { throw new Error("Live Draw input must not be read by a source scenario"); });
    expect(nodes.filter(({ nodeKind }) => nodeKind === "directory")).toHaveLength(DRAW_SOURCE_SCENARIO.oracle.sourceDirectoryCount);
    for (const row of DRAW_SOURCE_SCENARIO.members) {
      const expected = row.content.replaceAll("{{sourceRoot}}", projection.sourceRoot).replaceAll("{{artifactSourceRoot}}", artifactProjectionFixturePath(projection.sourceRoot));
      expect(nodes.find(({ path }) => path === `${projection.sourceRoot}/${row.path}`)?.content).toBe(expected);
    }
    const authority = projectionAuthority(projection, nodes);
    expect(authority.problems).toEqual([]);
    expect(authority.mappings).toEqual(projection.mappings);
    expect(authority.mappingDigest).toBe(projection.mappingDigest);
    expect(authority.destinationDirectoryCount).toBe(DRAW_SOURCE_SCENARIO.oracle.destinationDirectoryCount);
    expect(authority.destinationNodeCount).toBe(DRAW_SOURCE_SCENARIO.oracle.destinationNodeCount);
    for (const manifest of DRAW_SOURCE_SCENARIO.members.filter(({ path }) => path.endsWith("/📦️packages/🦀️rust/Cargo.toml"))) {
      const entry = posix.join(dirname(manifest.path), (toml.parse(manifest.content).lib as { path: string }).path);
      expect(projection.mappings.map(({ sourcePath }) => sourcePath)).toContain(`${projection.sourceRoot}/${entry}`);
      expect(projection.mappings.map(({ destinationPath }) => destinationPath)).toContain(`${projection.destinationRoot}/${entry}`);
    }
    expect(() => projectionAuthorityNodes(projectionGolden("artifact-example-model-catalog-v1"), "authored-draw")).toThrow("cannot supply another projection");
    expect(() => drawSourceScenarioContent("{{unknown}}", projection)).toThrow("unknown binding");
    expect(() => projectionAuthorityNodes({ ...projection, mappings: [{ sourcePath: `${projection.sourceRoot}/🧪️unknown`, destinationPath: `${projection.destinationRoot}/🧪️unknown` }] }, "authored-draw")).toThrow("does not own");
    expect(() => projectionAuthorityNodes(projection, "live", () => { throw new Error("Missing live source input"); })).toThrow("Missing live source input");
  });
  test("authored Draw source Cargo mount agrees with independent TOML and Rust parser ownership", async () => {
    const { cargoModuleRoot: root, oracle: { cargoModuleMount: mount } } = DRAW_SOURCE_SCENARIO;
    const files = artifactProjectionSourceFiles(true);
    expect(files[root.path]).toBe(root.content);
    const manifest = toml.parse(files[mount.manifestPath]!);
    expect((manifest.package as { name: string }).name).toBe(mount.packageName);
    expect((manifest.lib as { path: string }).path).toBe(mount.libraryEntry);
    const module = inspectRustModuleGraphFacts(root.content).modules;
    expect(module.map(({ name, pathTarget }) => ({ name, pathTarget }))).toEqual([{ name: mount.moduleName, pathTarget: mount.moduleTarget }]);
    const virtualRoot = resolve("/draw-source-fixture");
    expect(resolve(virtualRoot, dirname(mount.manifestPath), mount.libraryEntry)).toBe(resolve(virtualRoot, root.path));
    const modulePath = relative(virtualRoot, resolve(virtualRoot, dirname(root.path), mount.moduleTarget)).replaceAll("\\", "/");
    const { inspectRustModuleGraph } = await import("../../../../🔍️discovery/🌐️rust-module-graph/🟦️.ts"), { inspectRustJoinArgumentSpans, inspectRustManifestPathReferences } = await import("../../../../🔍️discovery/🧷️rust-path-references/🟦️.ts");
    const graph = inspectRustModuleGraph(Object.keys(files), (path) => files[path], { conventionalRoots: false, strictManifests: true });
    expect([...new Set(graph.contexts.get(modulePath)?.map(({ manifestPath }) => manifestPath))]).toEqual([mount.manifestPath]);
    expect(graph.contexts.get(modulePath)?.filter(({ modulePath: segments }) => segments.join("::") === mount.moduleName).map(({ manifestPath }) => manifestPath)).toEqual([mount.manifestPath]);
    const cad = projectionGolden("artifact-example-model-catalog-v1"), testsPath = posix.join(posix.dirname(modulePath), "🧪️tests/🔬️unit/🦀️.rs");
    const expectedRoot = `../../${posix.relative(cad.sourceRoot.slice(0, cad.sourceRoot.indexOf("/🏅️standards/")), cad.sourceRoot)}`;
    expect(inspectRustManifestPathReferences(files[modulePath]!)).toEqual([]);
    expect([...new Set(graph.contexts.get(testsPath)?.map(({ manifestPath }) => manifestPath))]).toEqual([mount.manifestPath]);
    expect(inspectRustManifestPathReferences(files[testsPath]!).map(({ value, base }) => ({ value, base }))).toEqual([{ value: expectedRoot, base: [] }]);
    expect(inspectRustJoinArgumentSpans(files[testsPath]!).map(({ value }) => value)).toEqual([expectedRoot, "\\n"]);
    const parser = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-name", "draw_source_mount", "--edition=2021", "--crate-type=lib", "-"], { input: root.content, encoding: "utf8", timeout: 10_000 });
    expect({ status: parser.status, signal: parser.signal, stderr: parser.stderr }).toEqual({ status: 0, signal: null, stderr: "" });
    expect(parser.stdout.match(/kind: Mod\(/gu)).toHaveLength(1);
    const name = parser.stdout.match(/kind: Mod\(\s*Default,\s*([a-z_][a-z_0-9]*)#[0-9]+,\s*Unloaded,/u)?.[1];
    const literal = parser.stdout.match(/symbol: ("(?:[^"\\]|\\.)*")/u)?.[1];
    const pathTarget = literal === undefined ? undefined : JSON.parse(literal.replace(/\\u\{([0-9a-f]+)\}/gu, (_, code: string) => String.fromCodePoint(Number.parseInt(code, 16))));
    expect([{ name, pathTarget }]).toEqual(module.map(({ name, pathTarget }) => ({ name, pathTarget })));
  });
  test("authored Draw source scenarios isolate every member and external consumer from live Draw reads", () => {
    const projection = projectionGolden("artifact-editor-command-bundle-v1"), reads: string[] = [];
    const files = artifactProjectionSourceFiles(true, (path) => {
      if (path.startsWith("✏️s/🔌️plugins/🖍️draw/") || path === "Cargo.toml" || path === "🔒️dependencies.json") throw new Error(`Unexpected live Draw source-scenario read: ${path}`);
      reads.push(path);
      return readFileSync(join(getWorkspaceRoot(), path), "utf8");
    });
    expect(reads).toHaveLength(projectionGolden("artifact-example-model-catalog-v1").sourceFileCount + 5);
    for (const row of DRAW_SOURCE_SCENARIO.members) expect(files[`${artifactProjectionFixturePath(projection.sourceRoot)}/${row.path}`]).toBe(drawSourceScenarioContent(row.content, projection));
    for (const row of DRAW_SOURCE_SCENARIO.consumers) expect(files[row.path]).toBe(drawSourceScenarioContent(row.content, projection));
    expect(files[DRAW_SOURCE_SCENARIO.cadConsumerMount.path]).toBeDefined();
    expect(createHash("sha256").update(files[DRAW_SOURCE_SCENARIO.cadConsumerMount.path]!).digest("hex")).toBe(createHash("sha256").update(readFileSync(join(getWorkspaceRoot(), DRAW_SOURCE_SCENARIO.cadConsumerMount.path))).digest("hex"));
  });
  test("authored Draw source run parent is exact and rejects unsafe ancestry before allocation", async () => {
    const expected = [NORMALIZATION_RUN_DIRECTORY, ...DRAW_SOURCE_SCENARIO.retention.parentSegments].join("/");
    expect(expected).toBe(ARTIFACT_PROJECTION_RUN_PARENT);
    const taxonomy = loadTaxonomy(), { registryCatalogPathMayAffect } = await import("../../../../🔍️discovery/🧲️registry-catalog-projection/🟦️.ts");
    const fixtureOwner = DRAW_SOURCE_SCENARIO.retention.fixtureOwnerSegments.join("/");
    expect(fixtureOwner).toBe("🧪️tests");
    expect(registryCatalogPathMayAffect(`${fixtureOwner}/🧪️fixture/🗿️artifacts/🖍️draw/🦀️.rs`, taxonomy)).toBe(true);
    const parsed = (await import("jsonc-parser")).parse(readFileSync(join(DRAW_SOURCE_SCENARIO_ROOT, "🔣️.json"), "utf8")) as DrawSourceScenario;
    expect(parsed.retention).toEqual(DRAW_SOURCE_SCENARIO.retention);
    const virtualRoot = resolve("/draw-source-ownership"), locations = artifactProjectionFixtureLocations(virtualRoot);
    expect(relative(virtualRoot, locations.workspace).replaceAll("\\", "/")).toBe(`${parsed.retention.fixtureOwnerSegments.join("/")}/🧪️fixture`);
    expect(relative(virtualRoot, locations.ticketDir).replaceAll("\\", "/")).toBe(parsed.retention.transactionTicketSegments.join("/"));
    expect(registryCatalogPathMayAffect(`${parsed.retention.transactionTicketSegments.join("/")}/🧾️taxonomy-transaction`, taxonomy)).toBe(false);
    for (const vector of DRAW_SOURCE_SCENARIO.retention.cases) {
      let created = 0;
      const inspect = (path: string) => {
        const selected = path === resolve(NORMALIZATION_RUN_ROOT, expected);
        if (selected && (vector.kind === "unreadable" || vector.kind === "missing" && created === 0)) throw Object.assign(new Error(vector.kind), { code: vector.kind === "missing" ? "ENOENT" : "EACCES" });
        return { isDirectory: () => !selected || vector.kind === "directory" || vector.kind === "missing" && created === 1, isSymbolicLink: () => selected && vector.kind === "symlink" };
      };
      const run = () => normalizationRetainedRunParent(expected, inspect, (path) => { expect(path).toBe(resolve(NORMALIZATION_RUN_ROOT, expected)); created++; });
      if (vector.allowed) expect(run()).toBe(resolve(NORMALIZATION_RUN_ROOT, expected));
      else expect(run).toThrow();
      expect(created).toBe(vector.creates);
    }
    expect(() => normalizationRetainedRunParent(`${expected}/../🧪️elsewhere`, () => { throw new Error("must not inspect"); }, () => { throw new Error("must not create"); })).toThrow("exact artifact run parent");
  });
  test("canonical CAD and Draw authority retains the exact language-neutral mapping digests", () => {
    for (const projection of ARTIFACT_PROJECTION_GOLDEN.projections) {
      const source = projectionAuthorityNodes(projection, projection.contractId === "artifact-editor-command-bundle-v1" ? "authored-draw" : "live");
      const directories = new Set<string>([projection.destinationRoot]);
      const files = projection.mappings.map(({ sourcePath, destinationPath }) => {
        const content = source.find(({ path }) => path === sourcePath)!.content!;
        for (let path = dirname(destinationPath); path !== dirname(projection.destinationRoot); path = dirname(path)) directories.add(path);
        return { path: destinationPath, nodeKind: "file" as const, content };
      });
      const nodes = [...[...directories].map((path) => ({ path, nodeKind: "directory" as const })), ...files];
      const result = semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes, layout: "destination" }, projectionTaxonomy(projection));
      expect(result.problems).toEqual([]);
      expect(result.mappings).toEqual(projection.mappings);
      expect(result.mappingDigest).toBe(projection.mappingDigest);
      const invalid = nodes.map((node) => node.nodeKind === "file" && node.path === files[0]!.path ? { ...node, path: `${node.path}.unexpected` } : node);
      expect(semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes: invalid, layout: "destination" }, projectionTaxonomy(projection)).problems.length).toBeGreaterThan(0);
      const symlink = nodes.map((node) => node.path === files[0]!.path ? { path: node.path, nodeKind: "symlink" as const } : node);
      expect(semanticPathProjectionAuthority({ artifactRoot: projection.sourceRoot.slice(0, projection.sourceRoot.indexOf("/🏅️standards/")), contractId: projection.contractId, sourceRoot: projection.sourceRoot, nodes: symlink, layout: "destination" }, projectionTaxonomy(projection)).problems.some((problem) => problem.includes("forbidden symlink"))).toBe(true);
    }
  });
  test("CAD profile vectors reject missing, duplicate, and ambiguous forward authority", () => {
    const taxonomy = loadTaxonomy();
    const catalog = taxonomy.semanticPathProjectionCatalogContracts["cad-model-catalog-v1"]!;
    expect("profileVectors" in catalog && catalog.profileVectors).toEqual([{ artifactId: "📐️cad", standardVersion: "1", subsetId: "any" }]);
    for (const vectors of [[], [{ artifactId: "📐️cad", standardVersion: "1", subsetId: "any" }, { artifactId: "📐️cad", standardVersion: "1", subsetId: "any" }], [{ artifactId: "📐️cad", standardVersion: "1", subsetId: "any/nested" }]]) {
      const changed = structuredClone(taxonomy);
      (changed.semanticPathProjectionCatalogContracts["cad-model-catalog-v1"] as { profileVectors: unknown }).profileVectors = vectors;
      expect(validateTaxonomy(changed).some((problem) => /profile|tuple/u.test(problem))).toBe(true);
    }
  });
}
