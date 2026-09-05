import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import emojiRegex from "emoji-regex";
import picomatch from "picomatch";
import ts from "typescript";
import { parseTree, type Node as JsonNode, type ParseError } from "jsonc-parser";
import { createValidFileMatcher } from "next/dist/server/lib/find-page-file.js";
import { createTaxonomyPathMatcher, fixedDirectoryContractIdsForPath, fixedFilenameContractIdsForPath, leadingEmojiIdentity, loadCatalogTaxonomy, mutationDomainOwnersProblems, mutationOwnerIdentity, mutationOwnerRelativePath, pathEmojiStatuteFindings, reservedDocumentationBasename, semanticDirectoryKindId, semanticManifestFilenameForCollection, semanticProjectionCatalogProblems, subsetDirectoryNameForId, subsetIdForDirectoryName, validateTaxonomy } from "../../🔍️discovery/🟦️.ts";
import { inventoryTaxonomySources } from "../../🧹️normalization/🟦️.ts";
import { jsonDocumentDuplicateKeys, mutationPayloadSchemaProblems, mutationPayloadSchemaDocumentProblems, semanticExactOwnedFileCatalog } from "../../🔍️discovery/🟦️.ts";
import { mutationCatalogSourceOwner, mutationCatalogSourceOwnersProblems } from "../../🔍️discovery/🟦️.ts";
import { mutationCatalogProblems } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const root = import.meta.dir;
const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));

test("mutation catalogs resolve only explicitly registered same-artifact and same-standard source owners", () => {
  const contract = fixture.mutationCatalogSourceOwnership, sourceRoot = `${contract.source}/🧬️schema/🧬️mutations`;
  for (const row of contract.cases) {
    const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [sourceRoot]: contract.domains }, mutationCatalogSourceOwners: { [row.owner]: row.source } };
    const parts = row.owner.split("/"), target = row.source.split("/");
    const oracle = picomatch("*/🏅️standards/🔖️1/🪆️subsets/*")(row.owner) && parts.length === 5 && parts.every((part: string) => part !== "." && part !== "..") && parts.slice(0, 4).join("/") === target.slice(0, 4).join("/") && row.owner !== row.source && row.source === contract.source;
    expect(oracle, row.id).toBe(row.valid);
    expect(mutationCatalogSourceOwnersProblems(taxonomy).length === 0, row.id).toBe(row.valid);
    expect(mutationCatalogSourceOwner(row.owner, taxonomy), row.id).toBe(row.valid ? row.source : null);
  }
  const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [sourceRoot]: contract.domains }, mutationCatalogSourceOwners: { [contract.catalog]: contract.source } };
  expect(mutationCatalogSourceOwner(contract.source, taxonomy)).toBe(contract.source);
  expect(mutationCatalogSourceOwner(`${contract.catalog}/✏️editor`, taxonomy)).toBe(`${contract.catalog}/✏️editor`);
  const chained = { ...taxonomy, mutationCatalogSourceOwners: { ...taxonomy.mutationCatalogSourceOwners, [contract.source]: contract.catalog } };
  expect(mutationCatalogSourceOwnersProblems(chained).length).toBeGreaterThan(0);
  const vectors = [
    { mutationId: "create-camera", sourceMutationDirectoryName: "🌱️create", mutationDirectoryName: "🌱️create", scenarios: [{ id: "applied", directoryName: "✅️applied" }] },
    { mutationId: "create-node", sourceMutationDirectoryName: "🌱️create", mutationDirectoryName: "🌱️create", scenarios: [{ id: "applied", directoryName: "🌱️applied" }] },
    { mutationId: "reorder-cameras", sourceMutationDirectoryName: "🔀️reorder", mutationDirectoryName: "🔀️reorder", scenarios: [{ id: "applied", directoryName: "🔀️applied" }] }
  ];
  expect(semanticProjectionCatalogProblems([{ ownerPath: contract.catalog, catalogId: "camera", vectors }], taxonomy)).toEqual([]);
  const catalog = { id: "camera", capability: "mutation", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️camera", kinds: vectors.map((vector) => vector.mutationId), vectors };
  expect(mutationCatalogProblems(catalog, contract.catalog, taxonomy)).toEqual([]);
  expect(mutationCatalogProblems({ ...catalog, vectors: [{ ...vectors[0], mutationId: "create-unknown" }] }, contract.catalog, taxonomy).some((problem) => problem.includes("registered domain-operation"))).toBe(true);
  expect(mutationCatalogProblems({ ...catalog, vectors: [{ ...vectors[0], scenarios: [{ id: "applied", directoryName: "🌱️✅️applied" }] }] }, contract.catalog, taxonomy).some((problem) => problem.includes("canonical NFC test-case"))).toBe(true);
  expect(semanticProjectionCatalogProblems([{ ownerPath: contract.catalog, catalogId: "camera", vectors: [{ ...vectors[0], mutationId: "create-unknown" }] }], taxonomy).some((problem) => problem.includes("registered domain-operation"))).toBe(true);
});

test("mutation vector discovery audits exact grouped owners and declared scenario names across catalogs", () => {
  const contract = fixture.mutationCatalogSourceOwnership, sourceRoot = `${contract.source}/🧬️schema/🧬️mutations`;
  const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [sourceRoot]: contract.domains }, mutationCatalogSourceOwners: { [contract.catalog]: contract.source } };
  const source = readFileSync(resolve(root, "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts"), "utf8");
  const syntax = ts.createSourceFile("test.ts", source, ts.ScriptTarget.Latest, true);
  const definition = syntax.statements.filter(ts.isFunctionDeclaration).find((node) => node.name?.text === "mutationVectorRegistryBreaches")!;
  const camera = { mutationId: "create-camera", sourceMutationDirectoryName: "🌱️create", mutationDirectoryName: "🌱️create", scenarios: [{ id: "applied", directoryName: "✅️applied" }] };
  const node = { ...camera, mutationId: "create-node", scenarios: [{ id: "applied", directoryName: "🌱️applied" }] };
  const catalog = { id: "camera", capability: "mutation", standardDirectoryName: "🔖️1", subsetDirectoryName: "✳️camera", kinds: ["create-camera"], vectors: [camera] };
  const other = { ...catalog, id: "node", subsetDirectoryName: "✳️any", kinds: ["create-node"], vectors: [node] };
  const registry = { contributions: [{ owner: contract.catalog, manifestPath: `${contract.catalog}/🔮️oracle/🔣️.json`, mutationCatalogs: [catalog] }, { owner: contract.source, manifestPath: `${contract.source}/🔮️oracle/🔣️.json`, mutationCatalogs: [other] }] };
  const sourceCases = [`${sourceRoot}/🎥️camera/🌱️create/🧪️tests/✅️applied`, `${sourceRoot}/🌳️node/🌱️create/🧪️tests/🌱️applied`];
  for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const nodes = new Set(sourceCases.flatMap((path) => path.split("/").map((_part, index, parts) => parts.slice(0, index + 1).join("/"))));
    const checked: string[] = [];
    const support = { join, relative, sep: "/", basename, PROFILE_MARKER: "/🏅️standards/", testTaxonomy: () => taxonomy, mutationCatalogProblems, mutationCatalogSourceOwner, mutationOwnerRelativePath, childDirectories: (path: string) => [...nodes].filter((node) => dirname(node) === path).map((node) => basename(node)), bundleBreach: (path: string) => { checked.push(path); return null; }, breach: (_statute: string, id: string, scope: string) => ({ id, scope }) };
    const audit = new Function(...Object.keys(support), `${compile(definition.getText(syntax).replace(/^export /u, ""))}\nreturn mutationVectorRegistryBreaches;`)(...Object.values(support));
    expect(audit(".", registry, taxonomy)).toEqual([]);
    expect(checked.sort()).toEqual(sourceCases.sort());
    nodes.add(`${sourceRoot}/🎥️camera/🛸️unregistered/🧪️tests/🛸️applied`);
    nodes.add(`${sourceRoot}/🎥️camera/🛸️unregistered/🧪️tests`);
    nodes.add(`${sourceRoot}/🎥️camera/🛸️unregistered`);
    expect(audit(".", registry, taxonomy).some((row: { id: string }) => row.id === "mutation-vector-unregistered")).toBe(true);
    nodes.delete(sourceCases[0]);
    expect(audit(".", registry, taxonomy).some((row: { id: string }) => row.id === "mutation-vector-missing")).toBe(true);
  }
});

test("normalization reads explicit cross-subset catalogs without searching unrelated ancestors", () => {
  const contract = fixture.mutationCatalogSourceOwnership, sourceRoot = `${contract.source}/🧬️schema/🧬️mutations`;
  const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [sourceRoot]: contract.domains }, mutationCatalogSourceOwners: { [contract.catalog]: contract.source } };
  const source = readFileSync(join(root, "../../🧹️normalization/🟦️.ts"), "utf8");
  const syntax = ts.createSourceFile("normalization.ts", source, ts.ScriptTarget.Latest, true);
  const names = ["projectionCatalogVectors", "projectionCatalogEntryForSubset", "projectionCatalogsForMutationSource", "canonicalProjectedMutationOwner", "projectMutationTestBundles"];
  const definitions = names.map((name) => syntax.statements.filter(ts.isFunctionDeclaration).find((node) => node.name?.text === name));
  expect(definitions.every(Boolean)).toBe(true);
  const entries = new Map<string, any>(), documents = new Map<string, string>();
  for (const [owner, mutationId] of [[contract.source, "create-node"], [contract.catalog, "create-camera"]]) {
    const path = `${owner}/🔮️oracle/🔣️.json`;
    entries.set(path, { sourcePath: path, normalizedPath: path, nodeKind: "file", fileKind: "json", violations: [] });
    documents.set(path, JSON.stringify({ mutationCatalogs: [{ id: mutationId, capability: "mutation", standardDirectoryName: "🔖️1", subsetDirectoryName: basename(owner), kinds: [mutationId], vectors: [{ mutationId, sourceMutationDirectoryName: "🌱️create", mutationDirectoryName: "🌱️create", scenarios: [{ id: "applied", directoryName: "✅️applied" }] }] }] }));
  }
  const scenarioSources = [
    { mutationId: "create-camera", owner: "🎥️camera/🌱️create", subset: "camera" },
    { mutationId: "create-node", owner: "🌳️node/🌱️create", subset: "any" }
  ].map((row) => ({ artifactRoot: "🗿️sample", artifactId: "sample", standardVersion: "1", standardDirectoryName: "🔖️1", subsetId: "any", subsetDirectoryName: "✳️any", mutationId: row.mutationId, mutationDirectoryName: "🌱️create", sourceScenarioId: "applied", sourceScenarioDirectoryName: "✅️applied", subsetRoot: contract.source, mutationRoot: `${sourceRoot}/${row.owner}`, scenarioRoot: `${sourceRoot}/${row.owner}/🧪️tests/✅️applied`, expected: `🗿️sample/🧪️tests/🪆️1-${row.subset}/${row.owner}/✅️applied` }));
  for (const row of scenarioSources) entries.set(row.scenarioRoot, { sourcePath: row.scenarioRoot, normalizedPath: row.scenarioRoot, nodeKind: "directory", violations: [] });
  const support = { basename, dirname, Buffer, mutationOwnerRelativePath, mutationCatalogSourceOwner, mutationCatalogSourceOwnersProblems, absolutePath: (_root: string, path: string) => path, readFileSync: (path: string) => documents.get(path), record: (value: unknown) => value, requiredString: (value: unknown) => { if (typeof value !== "string" || !value) throw new Error("string required"); return value; }, stringArray: (value: unknown) => value, splitLeadingEmoji: leadingEmojiIdentity, emojiFold: (value: string) => value.replaceAll("\uFE0F", ""), canonicalProjectedMemberName: () => null, projectionSourceAt: (path: string) => scenarioSources.find((row) => row.scenarioRoot === path) ?? null, mutationDescendantContract: () => ({ pathBudgetReserve: { bytes: 0 } }), projectionBundleProblem: () => null, setProjectedPath: (entry: any, path: string) => { entry.normalizedPath = path; }, violation: (code: string, path: string, detail: string) => ({ code, path, detail }) };
  for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const readers = new Function(...Object.keys(support), `${compile(definitions.map((node) => node!.getText(syntax)).join("\n"))}\nreturn {catalogs: projectionCatalogsForMutationSource, owner: canonicalProjectedMutationOwner, project: projectMutationTestBundles};`)(...Object.values(support));
    const catalogs = readers.catalogs(".", entries, contract.source, { discoverySchema: taxonomy });
    expect(catalogs.map((catalog: any) => catalog.owner).sort()).toEqual([contract.source, contract.catalog].sort());
    expect(catalogs.flatMap((catalog: any) => catalog.vectors).map((vector: any) => vector.mutationId).sort()).toEqual(["create-camera", "create-node"]);
    expect(catalogs.every((catalog: any) => !catalog.error)).toBe(true);
    const projected = structuredClone(entries);
    readers.project(".", "🗿️sample", projected, new Map(), { schema: taxonomy, discoverySchema: taxonomy });
    for (const row of scenarioSources) expect(projected.get(row.scenarioRoot).normalizedPath).toBe(row.expected);
    expect([...projected.values()].flatMap((entry: any) => entry.violations)).toEqual([]);
    expect(readers.owner("🌱️create", "create-camera", contract.catalog, { discoverySchema: taxonomy })).toBe("🎥️camera/🌱️create");
    expect(readers.owner("🌱️create", "create-unknown", contract.catalog, { discoverySchema: taxonomy })).toBeNull();
    expect(readers.catalogs(".", new Map(), contract.source, { discoverySchema: taxonomy }).every((catalog: any) => Boolean(catalog.error))).toBe(true);
    const malformed = { ...taxonomy, mutationCatalogSourceOwners: { [contract.catalog]: `${contract.source}/✏️editor` } };
    expect(() => readers.catalogs(".", entries, contract.source, { discoverySchema: malformed })).toThrow();
  }
});

test("glTF generator follows exact fixture-manifest roles and handpicked file coordinates", () => {
  const contract = fixture.gltfFixtureCoordinates, repoRoot = resolve(root, "../../../../../../..");
  const source = readFileSync(join(repoRoot, contract.owner, "🏭️generator/📜️script.ts"), "utf8");
  const syntax = ts.createSourceFile("script.ts", source, ts.ScriptTarget.Latest, true);
  const definition = syntax.statements.filter(ts.isFunctionDeclaration).find((node) => node.name?.text === "gltfFixtureOutputPaths");
  expect(definition).toBeDefined();
  const catalogDir = join(repoRoot, contract.owner, "🔮️oracle");
  const live = JSON.parse(readFileSync(join(catalogDir, "🔣️.json"), "utf8"));
  expect(parseTree(JSON.stringify(live.fixtureManifests))?.type).toBe("array");
  for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const paths = new Function("basename", "dirname", "isAbsolute", "join", "relative", "resolve", `${compile(definition!.getText(syntax).replace(/^export /u, ""))}\nreturn gltfFixtureOutputPaths;`)(basename, dirname, isAbsolute, join, relative, resolve);
    for (const item of contract.cases) {
      const record = { id: contract.id, files: item.paths.map((path: string, index: number) => ({ role: index === 0 ? "expected-before-gltf" : "expected-after-gltf", path })) };
      if (item.valid) {
        for (const path of item.paths) expect(picomatch(contract.pathShapes)(relative(resolve(catalogDir, "../.."), resolve(catalogDir, path)).replaceAll("\\", "/"))).toBe(true);
        expect(paths([record], contract.id, catalogDir)).toEqual({ before: resolve(catalogDir, item.paths[0]), after: resolve(catalogDir, item.paths[1]) });
        expect(paths([record], contract.id, catalogDir, "scratch")).toEqual({ before: join("scratch", contract.id, basename(item.paths[0])), after: join("scratch", contract.id, basename(item.paths[1])) });
        expect(() => paths([record, record], contract.id, catalogDir)).toThrow();
        expect(() => paths([record], "../escape", catalogDir)).toThrow();
        expect(() => paths([{ ...record, files: [...record.files, record.files[0]] }], contract.id, catalogDir)).toThrow();
      } else expect(() => paths([record], contract.id, catalogDir)).toThrow();
    }
    const current = paths(live.fixtureManifests, contract.id, catalogDir);
    for (const file of live.fixtureManifests.find((record: { id: string }) => record.id === contract.id).files) {
      const target = file.role === "expected-before-gltf" ? current.before : current.after;
      const bytes = readFileSync(target);
      expect(bytes.byteLength).toBe(file.bytes);
      expect(`sha256:${new Bun.CryptoHasher("sha256").update(bytes).digest("hex")}`).toBe(file.sha256);
    }
  }
  expect(source.includes("gltfFixtureOutputPaths(catalog.fixtureManifests, id, catalogDir, outRoot)")).toBe(true);
  expect(source.includes('join(dir, "before.gltf")')).toBe(false);
});

test("logo animation uses all six explicitly handpicked keyframe paths in order", () => {
  const source = readFileSync(resolve(root, "../../../../../../..", "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  const syntax = ts.createSourceFile("script.ts", source, ts.ScriptTarget.Latest, true);
  const definition = syntax.statements.filter(ts.isFunctionDeclaration).find((node) => node.name?.text === "logoKeyframePaths");
  expect(definition).toBeDefined();
  expect(fixture.assetLogoKeyframes.every(picomatch("🎞️animation/*/🖋️vector.svg"))).toBe(true);
  for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const paths = new Function("join", `${compile(definition!.getText(syntax).replace(/^export /u, ""))}\nreturn logoKeyframePaths;`)(join);
    expect(paths("logos")).toEqual(fixture.assetLogoKeyframes.map((path: string) => join("logos", path)));
  }
});

test("generated asset documentation keeps README literal without rewriting frozen owner evidence", () => {
  const taxonomy = loadCatalogTaxonomy(), contract = fixture.assetDocumentation;
  expect(taxonomy.generatorContracts[contract.generatorId].outputRoots.some((row) => row.path === contract.outputPath)).toBe(true);
  const evidence = taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  const repoRoot = resolve(root, "../../../../../../..");
  const before = readFileSync(join(repoRoot, evidence.authorityCatalogPath));
  const catalog = semanticExactOwnedFileCatalog(repoRoot, taxonomy);
  expect(catalog?.cases.some((row) => row.sourcePath === contract.outputPath)).toBe(true);
  expect(readFileSync(join(repoRoot, evidence.authorityCatalogPath))).toEqual(before);
  expect(parseTree(before.toString("utf8"))?.type).toBe("object");
  const source = readFileSync(join(repoRoot, taxonomy.generatorContracts[contract.generatorId].ownerPath, "📜️script.ts"), "utf8");
  const syntax = ts.createSourceFile("script.ts", source, ts.ScriptTarget.Latest, true);
  const definition = syntax.statements.filter(ts.isFunctionDeclaration).find((node) => node.name?.text === "renderCatalogReadme")!.getText(syntax);
  for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const render = new Function("join", `${compile(definition)}\nreturn renderCatalogReadme;`)(join);
    expect(render(dirname(contract.outputPath), []).path).toBe(contract.outputPath);
  }
});

test("asset SVG identities preserve handpicked paths independently of language bindings", () => {
  const source = readFileSync(resolve(root, "../../../../../../..", "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  const definition = source.match(/^export function catalogSvgSources\([\s\S]*?^\}/mu)?.[0];
  expect(definition).toBeDefined();
  for (const row of fixture.assetIconPaths) {
    const entries = row.paths.map((path: string) => {
      const parts = path.split("/");
      if (!picomatch("**/*.svg")(path) || /[\\:]/u.test(path)) return null;
      for (const part of parts) {
        const identity = part.replaceAll("\uFE0F", "").match(emojiRegex())?.[0];
        if (!identity || !part.replaceAll("\uFE0F", "").startsWith(identity)) return null;
        const tail = part.replaceAll("\uFE0F", "").slice(identity.length);
        if (!/^[a-zA-Z0-9]+(?:[-_][a-zA-Z0-9]+)*(?:\.svg)?$/u.test(tail)) return null;
      }
      return { id: parts.at(-1)!.replaceAll("\uFE0F", "").replace(emojiRegex(), "").slice(0, -4), path };
    });
    const oracle = entries.some((entry: unknown) => entry === null) || new Set(entries.map((entry: any) => entry?.id)).size !== entries.length ? null : entries;
    expect(oracle).toEqual(row.expected);
    for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText]) {
      const parse = new Function("leadingEmojiIdentity", `${compile(definition!.replace(/^export /u, ""))}\nreturn catalogSvgSources;`)(leadingEmojiIdentity);
      if (row.expected === null) expect(() => parse(row.paths)).toThrow();
      else expect(parse(row.paths)).toEqual(row.expected);
    }
  }
});

test("Rust icon bindings retain the source path and the public identity separately", () => {
  const source = readFileSync(resolve(root, "../../../../../../..", "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  const syntax = ts.createSourceFile("script.ts", source, ts.ScriptTarget.Latest, true);
  const definitions = syntax.statements.filter(ts.isFunctionDeclaration).filter((node) => ["renderRust", "renderRustMetabolism", "iconIdToRustVariant"].includes(node.name?.text ?? "")).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  for (const row of fixture.assetIconPaths.filter((row: any) => row.expected !== null)) {
    const icons = Object.fromEntries(row.expected.map(({ id }: { id: string }) => [id, `<svg id="${id}" />`]));
    for (const [renderer, mirror] of [["renderRust", "🖼️icon_svgs"], ["renderRustMetabolism", "🌱️metabolism_svgs"]]) {
      const expected = row.expected.map(({ id, path }: { id: string; path: string }) => ({ path: join("output", mirror, path), content: icons[id] })).sort((left: any, right: any) => left.path.localeCompare(right.path));
      for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
        const render = new Function("join", `${compile(definitions)}\nreturn ${renderer};`)(join);
        const artifacts = render(icons, "output", row.expected);
        expect(artifacts.filter((artifact: any) => artifact.path.endsWith(".svg")).sort((left: any, right: any) => left.path.localeCompare(right.path))).toEqual(expected);
        const binding = artifacts.find((artifact: any) => artifact.path.endsWith(".rs")).content;
        for (const { id, path } of row.expected) {
          expect(binding).toContain(`#[serde(rename = "${id}")]`);
          expect(binding).toContain(`include_str!("${mirror}/${path}")`);
        }
      }
    }
  }
});

test("descriptor-linked payload schemas stay inside their exact owner with handpicked paths", () => {
  const contract = fixture.mutationPayloadOwnership;
  for (const row of contract.cases) {
    const target = `${contract.owner}/${row.path}`, ancestor = dirname(target);
    const state = (path: string) => path === target ? { kind: row.state, content: JSON.stringify(contract.document) } : { kind: row.linkedAncestor && path === ancestor ? "symlink" : "directory", repositoryBoundary: row.repositoryBoundary && path === ancestor };
    const oracle = !/[\\:#\u0000-\u001F]/u.test(row.path) && !row.path.startsWith("/") && row.path.split("/").every((part: string) => part !== ".." && part !== "." && part !== "") && picomatch("**/*.json")(row.path) && row.state === "file" && !row.linkedAncestor && !row.repositoryBoundary;
    expect(oracle, row.path).toBe(row.expected);
    expect(mutationPayloadSchemaProblems(contract.owner, row.path, state).length === 0, row.path).toBe(row.expected);
  }
});

test("payload authority validates actual Draft-07 schema shape against Ajv", () => {
  const ajv = new Ajv({ strict: false });
  for (const row of fixture.mutationPayloadOwnership.documents) {
    const declaresDialect = typeof row.value === "object" && row.value !== null && !Array.isArray(row.value) && row.value.$schema === "http://json-schema.org/draft-07/schema#";
    const oracle = declaresDialect && ajv.validateSchema(row.value) === true;
    expect(oracle, JSON.stringify(row.value)).toBe(row.expected);
    expect(mutationPayloadSchemaDocumentProblems(row.value).length === 0, JSON.stringify(row.value)).toBe(row.expected);
  }
});

test("TSV mutation payload schemas resolve with camel-case language-neutral contracts", () => {
  const mutations = resolve(root, "../../../../../../..", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
  const cases = [
    { directory: "➕insert-row", positive: { index: 0, row: ["a"] }, negative: { index: -1, row: ["a"] } },
    { directory: "➖remove-row", positive: { index: 0 }, negative: { index: 0.5 } },
    { directory: "🔲set-cell", positive: { rowIndex: 0, fieldIndex: 1, value: "a" }, negative: { row_index: 0, field_index: 1, value: "a" } },
    { directory: "📸️set-snapshot", positive: { snapshot: { schema: "stdio.tsv", records: [], trailingNewline: false, lineEnding: "lf" } }, negative: { snapshot: { schema: "stdio.tsv", records: [], trailing_newline: false, line_ending: "lf" } } },
    { directory: "🔀set-line-ending", positive: { lineEnding: "crlf" }, negative: { lineEnding: "cr" } },
    { directory: "🔚set-trailing-newline", positive: { trailingNewline: false }, negative: { trailingNewline: "false" } },
  ];
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(JSON.parse(readFileSync(join(dirname(mutations), "📸️snapshot/🔣️.json"), "utf8")));
  for (const row of cases) {
    const owner = join(mutations, row.directory);
    const descriptor = JSON.parse(readFileSync(join(owner, "🔣️.json"), "utf8"));
    expect(descriptor.payloadSchema, row.directory).toBe("🧬️.schema.json");
    const schemaPath = join(owner, descriptor.payloadSchema);
    expect(existsSync(schemaPath), row.directory).toBe(true);
    const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
    expect(mutationPayloadSchemaDocumentProblems(schema), row.directory).toEqual([]);
    const validate = ajv.compile(schema);
    expect(validate(row.positive), `${row.directory}: ${JSON.stringify(validate.errors)}`).toBe(true);
    expect(validate(row.negative), row.directory).toBe(false);
    expect(readFileSync(join(owner, "🦀️.rs"), "utf8"), row.directory).toContain('#[value(rename_all = "camelCase")]');
  }
  const oracle = JSON.parse(readFileSync(join(dirname(dirname(mutations)), "🔮️oracle/🔣️.json"), "utf8"));
  const declared = oracle.mutationManifests.flatMap((manifest: any) => manifest.mutations).filter((mutation: any) => cases.some((row) => row.directory.endsWith(mutation.id)));
  expect(declared.map((mutation: any) => mutation.id).sort()).toEqual(["insert-row", "remove-row", "set-cell", "set-line-ending", "set-snapshot", "set-trailing-newline"]);
  expect(declared.every((mutation: any) => mutation.payloadSchema === "🧬️.schema.json")).toBe(true);
});

test("payload schema authority rejects duplicate decoded JSON members", () => {
  const contract = fixture.mutationPayloadOwnership, ajv = new Ajv({ strict: false });
  for (const row of contract.rawDocuments) {
    const errors: ParseError[] = [], tree = parseTree(row.content, errors, { disallowComments: true, allowTrailingComma: false });
    const uniqueMembers = (node: JsonNode): boolean => (node.type !== "object" || new Set(node.children?.map((child) => child.children?.[0]?.value)).size === node.children?.length) && (node.children ?? []).every(uniqueMembers);
    const oracle = errors.length === 0 && tree !== undefined && uniqueMembers(tree) && ajv.validateSchema(JSON.parse(row.content));
    expect(Boolean(oracle)).toBe(row.expected);
    const target = `${contract.owner}/🧬️.schema.json`;
    expect(mutationPayloadSchemaProblems(contract.owner, "🧬️.schema.json", (path) => path === target ? { kind: "file", content: row.content } : { kind: "directory" }).length === 0).toBe(row.expected);
  }
});

test("normalization validates descriptor authority for flat and grouped owners without schema moves", () => {
  const taxonomy = loadCatalogTaxonomy(), contract = fixture.mutationPayloadOwnership;
  const source = readFileSync(join(root, "../../🧹️normalization/🟦️.ts"), "utf8");
  const definition = source.match(/^function validateMutationPayloadSchemas\([\s\S]*?^\}/mu)?.[0];
  expect(definition).toBeDefined();
  const mutationRoot = "🗿️fixture/🧬️schema/🧬️mutations";
  const owners = [`${mutationRoot}/🌱️create-access-rule`, `${mutationRoot}/🔑️access-rule/🌱️create`];
  for (const owner of owners) for (const row of contract.cases) {
    const discoverySchema = { ...taxonomy, mutationDomainOwners: owner === owners[1] ? { [mutationRoot]: fixture.mutationDomainContract.domains } : {} };
    const descriptorPath = `${owner}/🔣️.json`, target = `${owner}/${row.path}`;
    const descriptor = { schemaVersion: 1, owner, semanticKind: "create-access-rule", payloadSchema: row.path, requiredLanguageSurfaces: ["json-schema"] };
    const documents = new Map([[descriptorPath, JSON.stringify(descriptor)], [target, JSON.stringify(contract.document)]]);
    const entries = new Map<string, any>();
    for (const path of [descriptorPath, target]) {
      const parts = path.split("/");
      for (let index = 1; index < parts.length; index++) {
        const ancestor = parts.slice(0, index).join("/");
        entries.set(ancestor, { sourcePath: ancestor, normalizedPath: ancestor, nodeKind: row.linkedAncestor && ancestor === dirname(target) ? "symlink" : "directory", violations: [] });
      }
    }
    entries.set(descriptorPath, { sourcePath: descriptorPath, normalizedPath: descriptorPath, nodeKind: "file", violations: [] });
    if (row.state !== "absent") entries.set(target, { sourcePath: target, normalizedPath: target, nodeKind: row.state, violations: [] });
    const support = { basename, dirname, Buffer, jsonDocumentDuplicateKeys, mutationOwnerIdentity, mutationPayloadSchemaProblems, record: (value: unknown) => value, readFileSync: (path: string) => Buffer.from(documents.get(path)!), assertLexicalInputOutsideOpaque: (_root: string, path: string) => path, isExcluded: (path: string) => Boolean(row.repositoryBoundary && path === dirname(target)), violation: (code: string, path: string, detail: string) => ({ code, path, detail }) };
    for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
      const rows = structuredClone(entries);
      new Function(...Object.keys(support), `${compile(definition!)}\nreturn validateMutationPayloadSchemas;`)(...Object.values(support))(".", rows, { discoverySchema });
      expect(rows.get(descriptorPath).violations.length === 0, `${owner}/${row.path}`).toBe(row.expected);
      expect([...rows.values()].every((entry) => entry.sourcePath === entry.normalizedPath)).toBe(true);
    }
  }
});

test("normalization requires one exact descriptor authority per admitted mutation owner", () => {
  const taxonomy = loadCatalogTaxonomy(), contract = fixture.mutationPayloadOwnership;
  const source = readFileSync(join(root, "../../🧹️normalization/🟦️.ts"), "utf8");
  const definition = source.match(/^function validateMutationPayloadSchemas\([\s\S]*?^\}/mu)![0];
  const mutationRoot = "🗿️fixture/🧬️schema/🧬️mutations";
  for (const owner of [`${mutationRoot}/🌱️create-access-rule`, `${mutationRoot}/🔑️access-rule/🌱️create`]) for (const row of contract.descriptors) {
    const discoverySchema = { ...taxonomy, mutationDomainOwners: owner.endsWith("/🌱️create") ? { [mutationRoot]: fixture.mutationDomainContract.domains } : {} };
    const descriptorPath = `${owner}/🔣️.json`, target = `${owner}/🧬️.schema.json`, extra = `${owner}/🪪️descriptor.json`;
    const descriptor = { schemaVersion: 1, owner, semanticKind: "create-access-rule", payloadSchema: "🧬️.schema.json", requiredLanguageSurfaces: ["json-schema"] };
    const documents = new Map([[target, JSON.stringify(contract.document)]]);
    if (row.present) documents.set(descriptorPath, JSON.stringify(descriptor));
    if (row.extra !== "absent") documents.set(extra, JSON.stringify(row.extra === "descriptor" ? descriptor : contract.document));
    const authorityCount = [...documents.values()].map((content) => parseTree(content)!).filter((tree) => [["owner", owner], ["semanticKind", "create-access-rule"]].every(([key, value]) => tree.children?.some((property) => property.children?.[0]?.value === key && property.children?.[1]?.value === value))).length;
    expect(row.present && authorityCount === 1, row.id).toBe(row.expected);
    const entries = new Map<string, any>([[owner, { sourcePath: owner, normalizedPath: owner, nodeKind: "directory", violations: [] }], ...[...documents.keys()].map((path) => [path, { sourcePath: path, normalizedPath: path, nodeKind: "file", violations: [] }] as const)]);
    const segments = owner.split("/");
    for (let index = 1; index < segments.length; index++) {
      const path = segments.slice(0, index).join("/");
      entries.set(path, { sourcePath: path, normalizedPath: path, nodeKind: "directory", violations: [] });
    }
    const support = { basename, dirname, Buffer, jsonDocumentDuplicateKeys, mutationOwnerIdentity, mutationPayloadSchemaProblems, record: (value: unknown) => value, readFileSync: (path: string) => Buffer.from(documents.get(path)!), assertLexicalInputOutsideOpaque: (_root: string, path: string) => path, isExcluded: () => false, violation: (code: string, path: string, detail: string) => ({ code, path, detail }) };
    for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
      const rows = structuredClone(entries);
      new Function(...Object.keys(support), `${compile(definition)}\nreturn validateMutationPayloadSchemas;`)(...Object.values(support))(".", rows, { discoverySchema });
      expect([...rows.values()].every((entry) => entry.violations.length === 0), row.id).toBe(row.expected);
      expect([...rows.values()].every((entry) => entry.sourcePath === entry.normalizedPath)).toBe(true);
    }
  }
});

test("captured structural schema checks reject linked and unadmitted authority", () => {
  const contract = fixture.mutationPayloadOwnership, taxonomy = loadCatalogTaxonomy();
  const source = readFileSync(resolve(root, "../../../../../../..", "📜️script.ts"), "utf8");
  const definitions = ["policyStructuralSource", "policyStructuralNodeState", "policyMutationPayloadSchemaProblems"].map((name) => source.match(new RegExp(`^function ${name}\\([\\s\\S]*?^}`, "m"))![0]).join("\n");
  for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const validate = new Function("mutationPayloadSchemaProblems", `${compile(definitions)}\nreturn policyMutationPayloadSchemaProblems;`)(mutationPayloadSchemaProblems);
    for (const row of contract.cases) {
      const target = `${contract.owner}/${row.path}`, parts = target.split("/");
      const directories = new Map(parts.slice(0, -1).map((_part: string, index: number) => [parts.slice(0, index + 1).join("/"), {}]));
      const contents = row.state === "file" ? new Map([[target, JSON.stringify(contract.document)]]) : new Map();
      if (row.state === "directory") directories.set(target, {});
      const observations = [{ sourcePath: target, observedKind: row.state }, ...(row.linkedAncestor ? [{ sourcePath: dirname(target), observedKind: "symlink" }] : []), ...(row.repositoryBoundary ? [{ sourcePath: dirname(target), observedKind: "directory", repositoryBoundary: "gitlink" }] : [])];
      expect(validate({ contents, directories, admission: { observations } }, contract.owner, row.path, taxonomy).length === 0, row.path).toBe(row.expected);
    }
  }
});

test("domain test reference coordinates retain their declared owner depth and semantic scenario", () => {
  const artifactRoot = "🗿️fixture", profile = "🏅️standards/🔖️1/🪆️subsets/✳️any", mutationRoot = `${artifactRoot}/${profile}/🧬️schema/🧬️mutations`;
  const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [mutationRoot]: fixture.mutationDomainContract.domains } };
  const source = readFileSync(join(root, "../../🧹️normalization/🟦️.ts"), "utf8");
  const constants = ["MUTATION_SOURCE_TEST_PREFIX", "MUTATION_SOURCE_STRUCTURE"].map((name) => source.match(new RegExp(`^const ${name} = .*;`, "m"))![0]).join("\n");
  const definitions = ["mutationStructuralPaths", "mutationProjectionRationale"].map((name) => source.match(new RegExp(`^function ${name}\\([\\s\\S]*?^}`, "m"))![0]).join("\n");
  const support = { basename, artifactRootForPath: () => artifactRoot, mutationCatalogSourceOwner, mutationOwnerIdentity, mutationOwnerRelativePath, splitLeadingEmoji: leadingEmojiIdentity, pathEmojiStatuteFindings, canonicalProjectedMemberName: () => null };
  for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const readers = new Function(...Object.keys(support), `${compile(`${constants}\n${definitions}`)}\nreturn {paths: mutationStructuralPaths, rationale: mutationProjectionRationale};`)(...Object.values(support));
    for (const row of fixture.mutationDomainContract.cases) {
      const from = `${mutationRoot}/${row.path}/🧪️tests/📨️sample/🔣️.json`, to = `${artifactRoot}/🧪️tests/🪆️1-any/${row.path}/📨️sample/🔣️.json`;
      const result = readers.rationale(from, to, { schema: taxonomy, discoverySchema: taxonomy });
      expect(result !== null, row.path).toBe(row.expected !== null);
      if (row.expected !== null) expect(readers.paths(from)[0].mutation).toBe(row.path);
    }
  }
});

test("cross-subset reference rationale accepts only the explicit catalog profile", () => {
  const contract = fixture.mutationCatalogSourceOwnership, sourceRoot = `${contract.source}/🧬️schema/🧬️mutations`;
  const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [sourceRoot]: contract.domains }, mutationCatalogSourceOwners: { [contract.catalog]: contract.source } };
  const source = readFileSync(join(root, "../../🧹️normalization/🟦️.ts"), "utf8");
  const constants = ["MUTATION_SOURCE_TEST_PREFIX", "MUTATION_SOURCE_STRUCTURE"].map((name) => source.match(new RegExp(`^const ${name} = .*;`, "m"))![0]).join("\n");
  const definitions = ["mutationStructuralPaths", "mutationProjectionRationale"].map((name) => source.match(new RegExp(`^function ${name}\\([\\s\\S]*?^}`, "m"))![0]).join("\n");
  const support = { basename, artifactRootForPath: () => "🗿️sample", mutationCatalogSourceOwner, mutationOwnerIdentity, mutationOwnerRelativePath, splitLeadingEmoji: leadingEmojiIdentity, pathEmojiStatuteFindings, canonicalProjectedMemberName: () => null };
  for (const compile of [(code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code), (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const rationale = new Function(...Object.keys(support), `${compile(`${constants}\n${definitions}`)}\nreturn mutationProjectionRationale;`)(...Object.values(support));
    const from = `${sourceRoot}/🎥️camera/🌱️create/🧪️tests/✅️applied/🔣️.json`;
    expect(rationale(from, "🗿️sample/🧪️tests/🪆️1-camera/🎥️camera/🌱️create/✅️applied/🔣️.json", { schema: taxonomy, discoverySchema: taxonomy })).toBe("artifact-mutation-test-projection-v1");
    expect(rationale(from, "🗿️sample/🧪️tests/🪆️1-unknown/🎥️camera/🌱️create/✅️applied/🔣️.json", { schema: taxonomy, discoverySchema: taxonomy })).toBeNull();
  }
});

test("domain-owned mutations keep explicit identities with short unique operation siblings", () => {
  const contract = fixture.mutationDomainContract;
  const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [contract.root]: contract.domains } };
  const validateDomains = new Ajv().compile(schema.properties.mutationDomainContract.properties.domains);
  expect(validateDomains(contract.domains)).toBe(true);
  expect(mutationDomainOwnersProblems(taxonomy.mutationDomainOwners, fixture.genericEmojiIdentities)).toEqual([]);
  const explicit = Object.entries(contract.domains).flatMap(([domain, operations]) => Object.entries(operations as Record<string, string>).map(([operation, identity]) => ({ path: `${domain}/${operation}`, identity })));
  for (const row of contract.cases) {
    const oracle = explicit.find((entry) => picomatch(entry.path, { literalBrackets: true })(row.path))?.identity ?? null;
    expect(oracle, row.path).toBe(row.expected);
    expect(mutationOwnerIdentity(contract.root, row.path, taxonomy), row.path).toBe(row.expected);
    if (row.expected !== null) expect(mutationOwnerRelativePath(contract.root, row.expected, taxonomy)).toBe(row.path);
  }
  expect(mutationOwnerIdentity("🗿️other/🧬️schema/🧬️mutations", "🌱️create-access-rule", taxonomy)).toBe("create-access-rule");
  expect(mutationOwnerIdentity("🗿️other/🧬️schema/🧬️mutations", "🔑️access-rule/🌱️create", taxonomy)).toBeNull();
  expect(mutationOwnerRelativePath(contract.root, "change-unknown-name", taxonomy)).toBeNull();
  const duplicate = structuredClone(taxonomy.mutationDomainOwners);
  duplicate[contract.root]["🔑️access-rule"]["🌱️spawn"] = "spawn-access-rule";
  expect(mutationDomainOwnersProblems(duplicate, fixture.genericEmojiIdentities).some((problem) => problem.includes("duplicate"))).toBe(true);
  const wrongIdentity = structuredClone(taxonomy.mutationDomainOwners);
  wrongIdentity[contract.root]["🔑️access-rule"]["🌱️create"] = "create-privacy-requirement";
  expect(mutationDomainOwnersProblems(wrongIdentity, fixture.genericEmojiIdentities).length).toBeGreaterThan(0);
  for (const row of contract.invalidDefinitions) {
    const invalid = structuredClone(taxonomy.mutationDomainOwners) as Record<string, Record<string, Record<string, unknown>>>;
    invalid[contract.root]![row.domain] ??= {};
    invalid[contract.root]![row.domain]![row.operation] = row.identity;
    expect(mutationDomainOwnersProblems(invalid as typeof taxonomy.mutationDomainOwners, fixture.genericEmojiIdentities).length, JSON.stringify(row)).toBeGreaterThan(0);
    const lexicalIdentity = new Ajv().compile(schema.properties.mutationDomainContract.properties.domains.additionalProperties.additionalProperties);
    expect(lexicalIdentity(row.identity)).toBe(typeof row.identity === "string" && /^[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/u.test(row.identity));
  }
  const ownerPath = "🗿️fixture/🏅️standards/🔖️1/🪆️subsets/✳️any";
  const catalogTaxonomy = { ...taxonomy, mutationDomainOwners: { [`${ownerPath}/🧬️schema/🧬️mutations`]: contract.domains } };
  const vectors = [["create-access-rule", "🌱️create"], ["create-privacy-requirement", "🌱️create"], ["reorder-cameras", "🔀️reorder"], ["change-node-name", "🏷️change-name"], ["bind-node-camera", "🎥️bind-camera"]].map(([mutationId, name]) => ({ mutationId, sourceMutationDirectoryName: name, mutationDirectoryName: name, scenarios: [{ id: "sample", directoryName: "📨️sample" }] }));
  expect(semanticProjectionCatalogProblems([{ ownerPath, catalogId: "domain-catalog", vectors }], catalogTaxonomy)).toEqual([]);
  expect(semanticProjectionCatalogProblems([{ ownerPath, catalogId: "domain-catalog", vectors: [{ ...vectors[0], mutationId: "create-unknown" }] }], catalogTaxonomy).some((problem) => problem.includes("no exact registered"))).toBe(true);
});

test("mutation projection catalog lookup respects exact oracle owner overrides", () => {
  const source = readFileSync(join(root, "../../🧹️normalization/🟦️.ts"), "utf8");
  const definition = source.match(/^function projectionCatalogEntryForSubset\([\s\S]*?^\}/mu)?.[0];
  expect(definition).toBeDefined();
  for (const compile of [
    (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code),
    (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText,
  ]) {
    const lookup = new Function("basename", "dirname", `${compile(definition!)}\nreturn projectionCatalogEntryForSubset;`)(basename, dirname);
    for (const row of fixture.projectionOracleDirectories) {
      const entry = { nodeKind: "file", fileKind: "json", sourcePath: row.candidate, normalizedPath: row.candidate };
      const overrides = row.override === null ? {} : { [row.owner]: row.override };
      const result = lookup(new Map([[row.candidate, entry]]), row.owner, { discoverySchema: { testContributionDirectoryOverrides: overrides, testContributionDirName: "🔮️oracle" } });
      expect(result !== null, row.candidate).toBe(row.expected);
    }
  }
});

test("subset directory overrides preserve logical ids while giving every sibling a semantic identity", () => {
  const vector = fixture.subsetDirectoryOverrides;
  const taxonomy = { ...loadCatalogTaxonomy(), subsetDirectoryOverrides: { [vector.owner]: vector.directories } };
  for (const row of vector.cases) {
    expect(subsetDirectoryNameForId(vector.owner, row.id, taxonomy), row.id).toBe(row.directory);
    expect(subsetIdForDirectoryName(vector.owner, row.directory, taxonomy), row.directory).toBe(row.id);
  }
  expect(subsetDirectoryNameForId(vector.owner, "unknown", taxonomy)).toBeNull();
  expect(subsetIdForDirectoryName(vector.owner, "✳️unknown", taxonomy)).toBeNull();
  expect(subsetDirectoryNameForId("unregistered/🪆️subsets", "probe", taxonomy)).toBe("✳️probe");
  expect(subsetIdForDirectoryName("unregistered/🪆️subsets", "✳️probe", taxonomy)).toBe("probe");
  expect(validateTaxonomy(taxonomy)).toEqual([]);
  const duplicate = structuredClone(taxonomy);
  duplicate.subsetDirectoryOverrides[vector.owner].analysis = duplicate.subsetDirectoryOverrides[vector.owner].mesh;
  expect(validateTaxonomy(duplicate).some((problem) => problem.includes("duplicate physical directory"))).toBe(true);
  const multiple = structuredClone(taxonomy);
  multiple.subsetDirectoryOverrides[vector.owner].mesh = "🧪️🕸️mesh";
  expect(validateTaxonomy(multiple).some((problem) => problem.includes("one canonical semantic emoji"))).toBe(true);
});

test("semantic collection manifest overrides use exact owners without ancestor fallback", () => {
  const vector = fixture.semanticManifestFilenameOverrides;
  const taxonomy = { ...loadCatalogTaxonomy(), semanticManifestFilenameOverrides: vector.overrides };
  for (const row of vector.cases) expect(semanticManifestFilenameForCollection(row.owner, taxonomy), row.owner).toBe(row.expected);
  expect(validateTaxonomy(taxonomy)).toEqual([]);
  const multiple = structuredClone(taxonomy);
  multiple.semanticManifestFilenameOverrides[Object.keys(multiple.semanticManifestFilenameOverrides)[0]] = "📥️📦️manifest.json";
  expect(validateTaxonomy(multiple).some((problem) => problem.includes("one canonical semantic emoji"))).toBe(true);
});

test("both mutation inventories enumerate only registered two-tier operation owners", () => {
  const contract = fixture.mutationDomainContract;
  const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [contract.root]: contract.domains } };
  const source = readFileSync(resolve(root, "../../../../../../..", "📜️script.ts"), "utf8");
  const definitions = ["policyListMutationDirs", "policyStructuralMutationChildren", "policyStructuralMutationDirs"].map((name) => source.match(new RegExp(`^function ${name}\\([\\s\\S]*?^}`, "m"))![0]).join("\n");
  const expected = Object.entries(contract.domains).flatMap(([domain, operations]) => Object.keys(operations as Record<string, string>).map((operation) => `${domain}/${operation}`)).sort();
  const paths = [contract.root, ...Object.keys(contract.domains).map((domain) => `${contract.root}/${domain}`), ...expected.map((owner) => `${contract.root}/${owner}`), `${contract.root}/🌱️create-access-rule`];
  const directories = new Map(paths.map((path) => [path, {}]));
  const readdir = (_root: string, path: string) => paths.filter((entry) => dirname(entry) === path).map((entry) => ({ name: basename(entry), isDirectory: true }));
  const support = { loadTaxonomy: () => taxonomy, policyReaddirSafe: readdir, mutationOwnerIdentity, mutationTaxonomyCompare: (left: string, right: string) => left.localeCompare(right) };
  for (const compile of [
    (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code),
    (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText,
  ]) {
    const readers = new Function(...Object.keys(support), `${compile(definitions)}\nreturn {live: policyListMutationDirs, captured: policyStructuralMutationDirs, children: policyStructuralMutationChildren};`)(...Object.values(support));
    const view = { taxonomySchema: { bytes: Buffer.from(JSON.stringify(taxonomy)) }, directories, admission: { observations: [] } };
    expect(readers.live(".", contract.root)).toEqual(expected);
    expect(readers.captured(view, contract.root).sort()).toEqual(expected);
    expect(readers.children(view, contract.root).filter((entry: { classification: string }) => entry.classification === "domain-owner")).toHaveLength(Object.keys(contract.domains).length);
    expect(readers.children(view, contract.root).find((entry: { name: string }) => entry.name === "🌱️create-access-rule")?.classification).toBe("malformed-child");
  }
});

test("normalization discovers exact domain mutation scenarios without flattening their owners", () => {
  const artifactRoot = "🗿️fixture", subsetRoot = `${artifactRoot}/🏅️standards/🔖️1/🪆️subsets/✳️any`, mutationRoot = `${subsetRoot}/🧬️schema/🧬️mutations`;
  const taxonomy = { ...loadCatalogTaxonomy(), mutationDomainOwners: { [mutationRoot]: fixture.mutationDomainContract.domains } };
  const source = readFileSync(join(root, "../../🧹️normalization/🟦️.ts"), "utf8");
  const definitions = ["mutationDomainOwnerLocation", "projectionDirectorySlug", "projectionSourceAt", "canonicalProjectedMemberName", "canonicalProjectedMutationOwner"].map((name) => source.match(new RegExp(`^function ${name}\\([\\s\\S]*?^}`, "m"))![0]).join("\n");
  const support = { basename, mutationCatalogSourceOwner, mutationOwnerIdentity, mutationOwnerRelativePath, pathEmojiStatuteFindings, splitLeadingEmoji: leadingEmojiIdentity, emojiFold: (value: string) => value.replaceAll("\uFE0F", "") };
  const literalKinds: Record<string, string> = { "🏅️standards": "standards", "🔖️1": "standard", "🪆️subsets": "subsets", "✳️any": "subset", "🧬️schema": "schema", "🧬️mutations": "schema", "🧪️tests": "tests" };
  for (const compile of [
    (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code),
    (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText,
  ]) {
    const readers = new Function(...Object.keys(support), `${compile(definitions)}\nreturn {source: projectionSourceAt, owner: canonicalProjectedMutationOwner};`)(...Object.values(support));
    for (const row of fixture.mutationDomainContract.cases) {
      const scenario = `${mutationRoot}/${row.path}/🧪️tests/📨️sample`, parts = scenario.split("/");
      const entries = new Map(parts.map((_part: string, index: number) => { const path = parts.slice(0, index + 1).join("/"); return [path, { sourcePath: path, normalizedPath: path, nodeKind: "directory" }]; }));
      const kinds = new Map([...entries.keys()].map((path) => [path, literalKinds[basename(path)] ?? "members-of-schema"]));
      const observed = readers.source(scenario, artifactRoot, entries, kinds, { schema: taxonomy, discoverySchema: taxonomy });
      expect(observed?.mutationId ?? null, row.path).toBe(row.expected);
      if (row.expected !== null) {
        expect(observed.mutationRoot).toBe(`${mutationRoot}/${row.path}`);
        expect(observed.subsetRoot).toBe(subsetRoot);
        expect(readers.owner(basename(row.path), row.expected, subsetRoot, { schema: taxonomy, discoverySchema: taxonomy })).toBe(row.path);
      }
    }
  }
});

test("projected scenarios retain individually chosen single-emoji identities", () => {
  const taxonomy = loadCatalogTaxonomy();
  const projected = taxonomy.semanticProjectedMemberKinds[taxonomy.mutationCatalogProjection.projectedMemberKindId];
  const mutationDirectoryName = taxonomy.semanticDirectoryMemberKinds[projected.sourceMemberKindId].memberNames[0]!;
  const mutationId = leadingEmojiIdentity(mutationDirectoryName).rest;
  for (const row of fixture.projectionScenarios) {
    const matches = [...row.directoryName.matchAll(emojiRegex())];
    const oracle = matches.length === 1 && matches[0]?.index === 0 && row.directoryName.slice(matches[0][0].length).replace(/^\uFE0F/u, "") === row.id && !fixture.genericEmojiIdentities.includes(matches[0][0].replace(/\uFE0F$/u, "") + "\uFE0F");
    expect(oracle, row.directoryName).toBe(row.expected);
    const problems = semanticProjectionCatalogProblems([{
      ownerPath: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any",
      catalogId: "handpicked-scenarios",
      vectors: [{ mutationDirectoryName, mutationId, sourceMutationDirectoryName: mutationDirectoryName, scenarios: [{ id: row.id, directoryName: row.directoryName }] }],
    }], taxonomy);
    expect(problems.filter((problem) => problem.includes("test-case identity")).length === 0, row.directoryName).toBe(row.expected);
  }
});

test("Storybook discovers every handpicked UI story by its semantic suffix", () => {
  const source = readFileSync(resolve(root, "../../../../../../..", ".storybook/scopes.ts"), "utf8");
  const pattern = source.match(/"(\.\.\/🧰️framework\/🔨️modules\/🖱️ui\/🧱️elements\/[^"]+\.story\.tsx)"/u)?.[1];
  expect(pattern).toBeDefined();
  const glob = pattern!.slice(3);
  const matcher = createTaxonomyPathMatcher();
  const oracle = picomatch(glob);
  for (const row of fixture.storyNames) {
    const path = `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧩️Example/${row.name}`;
    expect(oracle(path), row.name).toBe(row.expected);
    expect(matcher.matches(path, glob), row.name).toBe(row.expected);
  }
});

test("graph manifest discovery uses its semantic filename rather than one shared emoji", () => {
  const source = readFileSync(resolve(root, "../../../../../../..", "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts"), "utf8");
  const watcher = readFileSync(resolve(root, "../../../../../../..", "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/build.rs"), "utf8");
  expect(/else if ([^\n]+) \{\n\s+println!\("cargo:rerun-if-changed=/u.exec(watcher)?.[1]).toBe('name.ends_with("manifest.json")');
  const expression = /else if \(([^\n]+)\) \{\n\s+out\.push\(path\)/u.exec(source)?.[1];
  expect(expression).toBeDefined();
  const accepts = new Function("name", `return ${expression};`);
  const oracle = picomatch("*manifest.json");
  for (const row of fixture.graphManifestNames) {
    expect(oracle(row.name)).toBe(row.expected);
    expect(accepts(row.name), row.name).toBe(row.expected);
  }
});

test("normalization preserves handpicked file identities and rejects stacked names through both compilers", () => {
  const taxonomy = loadCatalogTaxonomy();
  const source = readFileSync(join(root, "../../🧹️normalization/🟦️.ts"), "utf8");
  const names = ["canonicalFile", "splitLeadingEmoji", "splitLeadingEmojiIdentity", "isEmojiGrapheme", "emojiFold"];
  const definitions = names.map((name) => {
    const matches = [...source.matchAll(new RegExp(`^function ${name}\\([\\s\\S]*?^}`, "gm"))];
    expect(matches, name).toHaveLength(1);
    return matches[0]![0];
  }).join("\n");
  const support = {
    basename, dirname,
    SEGMENTER: new Intl.Segmenter("und", { granularity: "grapheme" }),
    packageLocation: () => null,
    configurableContract: () => null,
    matchingFixedContracts: (path: string) => {
      const ids = fixedFilenameContractIdsForPath(path, taxonomy);
      return { selected: ids.length === 1 ? [ids[0], taxonomy.fixedFilenameContracts[ids[0]!]] : null, ambiguous: ids.length > 1 ? ids : [] };
    },
    resolveFileKind: (path: string) => ({ kind: { id: "json", emoji: "🔣️", role: "data" }, extension: path.endsWith(".schema.json") ? ".schema.json" : ".json", stem: basename(path).replace(/(?:\.schema)?\.json$/u, "") }),
    matchDirectoryKind: () => ({ kind: null, ambiguous: [] }),
    GENERIC_SEMANTIC_STEMS: new Set(),
    pathEmojiStatuteFindings,
    reservedDocumentationBasename,
    violation: (code: string, path: string, message: string) => ({ code, path, message, severity: "error" }),
  };
  for (const compile of [
    (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code),
    (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText,
  ]) {
    const canonical = new Function(...Object.keys(support), `${compile(definitions)}\nreturn canonicalFile;`)(...Object.values(support));
    for (const scenario of fixture.normalization) {
      const result = canonical(scenario.name, "", undefined, [], new Map(), new Map(), new Map(), { schema: { fixedFilenameContracts: {} } });
      expect(result.path, scenario.name).toBe(scenario.expectedName);
      expect(result.violations.map((row: { code: string }) => row.code), scenario.name).toEqual(scenario.expectedViolations);
    }
  }
}, 30000);

test("the normalization reader accepts the same current reserved-name contracts", () => {
  expect(() => inventoryTaxonomySources({ repoRoot: resolve(root, "../../../../../../.."), scope: "README.md" })).not.toThrow();
}, 30000);

test("taxonomy accepts single keycaps and pictographic sequences without accepting stacked identities", () => {
  const baseline = loadCatalogTaxonomy();
  const probes = fixture.taxonomyEmojiIdentities.map((row, index) => ({ row, kindId: `identity-probe-${index}`, memberId: `identity-member-probe-${index}` }));
  const taxonomy = {
    ...baseline,
    semanticDirectoryKinds: Object.fromEntries([
      ...Object.entries(baseline.semanticDirectoryKinds),
      ...probes.map(({ row, kindId }) => [kindId, { emoji: row.emoji, slugPattern: `^identity-probe-${kindId.slice("identity-probe-".length)}$`, allowEmojiOnly: false, parentKindIds: ["modules"] }]),
    ]),
    semanticDirectoryMemberKinds: Object.fromEntries([
      ...Object.entries(baseline.semanticDirectoryMemberKinds),
      ...probes.map(({ row, memberId }, index) => [memberId, { source: "registry", ownerKindIds: ["modules"], memberNames: [`${row.emoji}member-probe-${index}`] }]),
    ]),
  };
  const problems = validateTaxonomy(taxonomy as typeof baseline);
  for (const { row, kindId, memberId } of probes) {
    const matches = [...row.emoji.matchAll(emojiRegex())];
    const exactSequence = matches.length === 1 && matches[0]![0] === row.emoji;
    const explicitlyPresentedSingle = matches.length === 1 && `${matches[0]![0]}\uFE0F` === row.emoji;
    const legacyKeycap = /^[#*0-9]\u20E3$/u.test(row.emoji);
    const oracle = ((exactSequence && !legacyKeycap) || explicitlyPresentedSingle) && row.emoji === row.emoji.normalize("NFC");
    expect(oracle, row.emoji).toBe(row.expected);
    expect(problems.some((problem) => problem.includes(`semanticDirectoryKinds["${kindId}"].emoji`)), row.emoji).toBe(!row.expected);
    if (row.expected) {
      expect(problems.some((problem) => problem.includes(`semanticDirectoryMemberKinds["${memberId}"] has invalid exact member`)), row.emoji).toBe(false);
      expect(semanticDirectoryKindId(`${row.emoji}${kindId}`, taxonomy as typeof baseline, { parentKindId: "modules" })).toBe(kindId);
    }
  }
});

test("selector-free joined identities are the only live spellings for the repaired owners", () => {
  const repoRoot = resolve(root, "../../../../../../..");
  for (const row of fixture.selectorFreeZwjOwners) {
    expect(existsSync(join(repoRoot, row.path)), row.path).toBe(true);
    expect(existsSync(join(repoRoot, row.legacyPath)), row.legacyPath).toBe(false);
    const name = basename(row.path), identity = leadingEmojiIdentity(name).emoji, oracle = emojiRegex().exec(name);
    expect(oracle?.index, row.path).toBe(0);
    expect(identity, row.path).toBe(oracle?.[0]);
    expect(identity.includes("\uFE0F"), row.path).toBe(false);
    expect(pathEmojiStatuteFindings([{ path: row.path, nodeKind: row.nodeKind }], fixture.genericEmojiIdentities), row.path).toEqual([]);
  }
});

test("path emoji statutes share a language-neutral contract", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const scenario of fixture.cases) {
    expect(pathEmojiStatuteFindings(scenario.entries, fixture.genericEmojiIdentities)).toEqual(scenario.expected);
  }
});

test("leading emoji identities agree with the independent Unicode emoji oracle", () => {
  for (const scenario of fixture.cases) for (const entry of scenario.entries) {
    const name = entry.path.split("/").at(-1)!;
    const oracle = emojiRegex().exec(name);
    const observed = leadingEmojiIdentity(name).emoji;
    expect(Boolean(observed)).toBe(Boolean(oracle?.index === 0));
    if (oracle?.index === 0) expect(observed.startsWith(oracle[0]) || oracle[0].startsWith(observed)).toBe(true);
  }
});

test("stacked emoji prefixes are rejected even when a structural role can still be diagnosed", () => {
  for (const name of ["🎮️🔎️commands", "📦️📦️packages", "🧪️✅️tests"]) {
    expect(pathEmojiStatuteFindings([{ path: name, nodeKind: "directory" }], [])).toContainEqual({ kind: "multiple", path: name, emoji: leadingEmojiIdentity(name).emoji });
  }
  expect(semanticDirectoryKindId("🎮️commands")).toBe("commands");
});

test("standard documentation basenames remain reserved outside package roots too", () => {
  const taxonomy = loadCatalogTaxonomy();
  expect(validateTaxonomy(taxonomy)).toEqual([]);
  const context = { packageRoot: true, ecosystemId: "🟦️typescript" } as const;
  expect(fixedFilenameContractIdsForPath("owner/README.md", taxonomy, context)).toEqual(["bun-package-readme"]);
  expect(fixedFilenameContractIdsForPath("owner/LICENSE.md", taxonomy, context)).toEqual(["bun-package-license"]);
  expect(fixedFilenameContractIdsForPath("owner/README.md", taxonomy)).toEqual(["reserved-readme-markdown"]);
  expect(fixedFilenameContractIdsForPath("owner/README", taxonomy)).toEqual(["reserved-readme"]);
  expect(fixedFilenameContractIdsForPath("owner/LICENSE", taxonomy)).toEqual(["reserved-license"]);
  expect(fixedFilenameContractIdsForPath("owner/LICENSE.md", taxonomy)).toEqual(["reserved-license-markdown"]);
});

test("Cargo conventional source entrypoints remain literal reserved basenames", () => {
  const taxonomy = loadCatalogTaxonomy();
  expect(fixedDirectoryContractIdsForPath("owner/🏗️generator/🦀️json-engine/src", taxonomy)).toEqual(["cargo-build-source-directory"]);
  expect(fixedFilenameContractIdsForPath("owner/src/lib.rs", taxonomy)).toEqual(["cargo-conventional-library-entry"]);
  expect(fixedFilenameContractIdsForPath("owner/src/main.rs", taxonomy)).toEqual(["cargo-conventional-binary-entry"]);
});

test("OS semantic-stem owners use canonical leaves and exact tool authority", () => {
  const taxonomy = loadCatalogTaxonomy();
  const scenario = fixture.cases.find((row: { id: string }) => row.id === "os-semantic-stem-owners-use-canonical-leaves-and-exact-tool-authority");
  expect(pathEmojiStatuteFindings(scenario.entries, taxonomy.pathEmojiPolicy.genericEmojiIdentities)).toEqual([]);
  const coreManifest = scenario.entries.find((row: { path: string }) => row.path.endsWith("/package.json")).path;
  const familyManifest = coreManifest.replace("/🫀️core/🕸️bindings/", "/📦️packages/🦀️rust/🕸️bindings/");
  expect(fixedFilenameContractIdsForPath(coreManifest, taxonomy)).toEqual(["flow-core-package-manifest"]);
  expect(fixedFilenameContractIdsForPath(familyManifest, taxonomy)).toEqual(["flow-family-package-manifest"]);
  expect(fixedFilenameContractIdsForPath(coreManifest.replace("/🕸️bindings/", "/🕸️bindings/🧪️nested/"), taxonomy)).toEqual([]);
  expect(fixedFilenameContractIdsForPath(`foreign/${coreManifest}`, taxonomy)).toEqual([]);
  const outline = basename(scenario.entries.find((row: { path: string }) => row.path.endsWith(".ttf")).path).replace(/\.ttf$/u, "");
  expect(semanticDirectoryKindId(outline, taxonomy)).toBe("distribution-font-outline-asset");
  for (const row of scenario.entries.filter((entry: { path: string; reserved?: boolean }) => !entry.reserved && !entry.path.endsWith(".ttf"))) {
    const name = basename(row.path), identity = leadingEmojiIdentity(name).emoji;
    expect(name.slice(identity.length).startsWith("."), row.path).toBe(true);
    expect([...identity.replaceAll("\uFE0F", "").matchAll(emojiRegex())], row.path).toHaveLength(1);
  }
});

test("authored artifact taxonomies remain governed below artifact owners", () => {
  const taxonomy = loadCatalogTaxonomy();
  for (const name of fixture.authoredSubtreeNames) expect(taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames).not.toContain(name);
  expect(taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames).not.toContain("🗿️artifacts");
  expect(taxonomy.pathEmojiPolicy.reservedSubtreeDirectoryNames).not.toContain("📄️artifacts");
  const commandRoot = "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🎮️commands";
  expect(pathEmojiStatuteFindings([
    { path: `${commandRoot}/🧬️add-generation`, nodeKind: "directory" },
    { path: `${commandRoot}/🧬️remove-generation`, nodeKind: "directory" },
  ], taxonomy.pathEmojiPolicy.genericEmojiIdentities)).toEqual([
    { kind: "duplicate", path: `${commandRoot}/🧬️remove-generation`, sibling: `${commandRoot}/🧬️add-generation`, emoji: "🧬" },
  ]);
});

test("framework-owned Next entry identities stay reserved without exempting authored helpers", () => {
  const taxonomy = loadCatalogTaxonomy();
  const root = "owner/📦️packages/🟦️typescript/app";
  expect(fixedDirectoryContractIdsForPath(root, taxonomy)).toEqual(["next-app-router-root"]);
  expect(taxonomy.fixedDirectoryContracts["next-app-router-root"]?.descendants).toBeUndefined();
  expect(fixedDirectoryContractIdsForPath(`${root}/api`, taxonomy)).toEqual([]);
  const oracle = createValidFileMatcher(["ts", "tsx"], root);
  for (const row of fixture.nextEntryNames) {
    const path = `${root}/${row.path}`;
    expect(oracle.isAppRouterPage(path) || oracle.isAppLayoutPage(path), row.path).toBe(row.expected);
    expect(fixedFilenameContractIdsForPath(path, taxonomy).length > 0, row.path).toBe(row.expected);
  }
});
