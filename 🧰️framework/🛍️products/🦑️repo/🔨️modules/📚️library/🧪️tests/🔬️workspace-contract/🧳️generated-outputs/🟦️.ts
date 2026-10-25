import { wasmOutputDirectory } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, readFileSync, readdirSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, validateTaxonomy, loadCatalogTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { fixedDirectoryContractIdsForPath, fixedFilenameContractIdsForPath } from "../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts";
import { type FixedDirectoryContract, type FixedFilenameContract, type PackageSourceDisposition } from "../../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";
import { type Taxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { normalizationEngineDeclarations } from "../../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { probeClone, readToml } from "../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

/** 🦀️ One member manifest, read only for the identity fields the installation owner projects. */
interface CargoMemberManifest {
  readonly package: {
    readonly name: string;
    readonly metadata: { readonly component: { readonly package: string }; readonly semio: { readonly extends: string; readonly contributes: unknown } };
  };
  readonly lib: { readonly path: string };
}

/** 🔒️ The exact-parent admission vectors driving the fixed-directory-contract-set probes. */
interface FixedParentCases {
  readonly parents: Readonly<Record<string, string>>;
  readonly accepted: { readonly kind: "fixed-directory-contract-set"; readonly fixedDirectoryContractIds: readonly string[] };
  readonly namedSets: Readonly<Record<string, readonly string[]>>;
  readonly namedAccepted: { readonly kind: "named-fixed-directory-contract-set"; readonly fixedDirectoryContractSetId: string };
  readonly namedRejected: readonly unknown[];
  readonly rejected: readonly unknown[];
  readonly invalidLiteralBasenames: readonly string[];
  readonly matches: readonly { readonly parent: string; readonly filename: string; readonly accepted: boolean }[];
}

describe("current JCO destination authority", () => {
  test("preserves tool-owned declaration pairing and the exact vendored shim payloads", async () => {
    const ts = await import("typescript");
    const root = findWorkspaceRoot(contractRoot);
    const fixtureRoot = join(root, "🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe"), host = join(fixtureRoot, "🌐️browser-host"), bundles = join(fixtureRoot, "🌐️browser-bundles");
    const options = { allowJs: true, moduleResolution: ts.ModuleResolutionKind.Bundler, module: ts.ModuleKind.ESNext };
    const taxonomy = loadTaxonomy();
    const fixed = Object.entries(taxonomy.fixedFilenameContracts).filter(([id]) => id.startsWith("jco-"));
    expect(fixed).toHaveLength(48);
    for (const [id, contract] of fixed) {
      expect(contract.pathPattern).not.toMatch(/[*?\[\]{}]/u);
      expect(fixedFilenameContractIdsForPath(contract.pathPattern, taxonomy)).toEqual([id]);
      expect(fixedFilenameContractIdsForPath(contract.pathPattern + ".extra", taxonomy)).toEqual([]);
      expect(fixedFilenameContractIdsForPath("other/" + contract.pathPattern, taxonomy)).toEqual([]);
      expect(lstatSync(join(root, contract.pathPattern)).isFile()).toBe(true);
    }
    for (const variant of ["📞️out-callback", "⚡️out-jspi-explicit"]) {
      const owner = join(bundles, variant);
      expect(ts.resolveModuleName("./jcoprobe.js", join(owner, "consumer.ts"), options, ts.sys).resolvedModule?.resolvedFileName.replaceAll("\\", "/")).toBe(join(owner, "jcoprobe.d.ts").replaceAll("\\", "/"));
      const interfacePath = join(owner, "interfaces").slice(root.length + 1).replaceAll("\\", "/");
      expect(fixedDirectoryContractIdsForPath(interfacePath, taxonomy)).toHaveLength(1);
      expect(fixedDirectoryContractIdsForPath(interfacePath + "-other", taxonomy)).toEqual([]);
      expect(fixedFilenameContractIdsForPath(interfacePath + "/custom.d.ts", taxonomy)).toEqual([]);
      const declarations = readdirSync(join(owner, "interfaces")).filter((name) => name.endsWith(".d.ts"));
      expect(declarations).toHaveLength(16);
      for (const name of ["jcoprobe.d.ts", ...declarations.map((name) => "interfaces/" + name)]) {
        const path = join(owner, name);
        for (const specifier of ts.preProcessFile(readFileSync(path, "utf8"), true, true).importedFiles) expect(ts.resolveModuleName(specifier.fileName, path, options, ts.sys).resolvedModule, specifier.fileName).toBeDefined();
      }
      expect(readFileSync(join(owner, "jcoprobe.js"), "utf8")).toContain("new URL('./jcoprobe.core.wasm', import.meta.url)");
      expect(readFileSync(join(owner, "jcoprobe.core.wasm")).subarray(0, 4)).toEqual(Buffer.from([0, 97, 115, 109]));
    }
    const shim = join(host, "🪞️preview2-shim");
    const tool = join(root, "node_modules/@bytecodealliance/preview2-shim/dist/browser"), names = readdirSync(tool).filter((name) => name.endsWith(".js")).sort();
    expect(names).toEqual(expect.arrayContaining(["cli.js", "clocks.js", "config.js", "environment.js", "filesystem.js", "http.js", "index.js", "io.js", "random.js", "sockets.js"]));
    expect(readdirSync(shim).sort()).toEqual(names);
    for (const name of names) expect(readFileSync(join(shim, name))).toEqual(readFileSync(join(tool, name)));
  });

  test("separates current paths from immutable evidence and verifies current package ownership", async () => {
    const { parseCurrentJcoPackageDestination } = await import("../../../🔍️discovery/📦️package-projection-contracts/🟦️.ts"), { semanticPackageAdapterPreview } = await import("../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts");
    const taxonomy = JSON.parse(readFileSync(join(contractRoot, "../../🔣️taxonomy.json"), "utf8"));
    const root = findWorkspaceRoot(contractRoot);
    const inputRoot = join(root, "🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe");
    const current = taxonomy.generatorContracts["jco-package-adapter"].currentPackageDestination;
    expect(parseCurrentJcoPackageDestination(current)).toEqual(current);
    for (const row of JSON.parse(readFileSync(join(inputRoot, "🧪️destination-cases.json"), "utf8")).invalid) {
      const candidate = structuredClone(current);
      if (row.remove) delete candidate[row.field];
      else candidate[row.field] = row.valueFrom ? candidate[row.valueFrom] : row.value;
      expect(() => parseCurrentJcoPackageDestination(candidate), row.id).toThrow();
    }
    const contract = taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
    expect(createHash("sha256").update(readFileSync(join(root, contract.authorityCatalogPath))).digest("hex")).toBe(contract.authorityCatalogSha256);
    const adapters = semanticPackageAdapterPreview(root, "jcoprobe-guest", taxonomy);
    expect(adapters).toHaveLength(1);
    expect(adapters[0].path).toBe(current.adapterPath);
    expect(adapters[0].targetPaths).toEqual([current.componentPath]);
    expect(adapters[0].content).toBe(readFileSync(join(root, current.adapterPath), "utf8"));
    const manifest = readToml<CargoMemberManifest>(readFileSync(join(root, current.cargoManifestPath), "utf8"));
    expect(manifest.package.name).toBe("semio-jcoprobe-guest");
    expect(join(current.packageRoot, manifest.lib.path).replaceAll("\\", "/")).toBe(current.adapterPath);
  });
});

describe("finite exact fixed parent scope", () => {
  test("admits only declared parents and one exact leaf through both classifiers", async () => {
    const { parseFixedDirectoryContractSetScope, parseNamedFixedDirectoryContractSetScope, taxonomyPathPatternMatches } = await import("../../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts"), { createFixedContractResolver } = await import("../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts");
    const ts = await import("typescript");
    const root = findWorkspaceRoot(contractRoot);
    const vector: FixedParentCases = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧫️fixtures/📦️deployment/🔒️fixed-parent-cases.json"), "utf8"));
    const common = { authority: "fixture compiler", reason: "exact parent fixture", configurability: "unconfigurable", verification: "finite exact fixed parent scope", expires: null } as const;
    const parents: Readonly<Record<string, FixedDirectoryContract>> = Object.fromEntries(Object.entries(vector.parents).map(([id, path]): [string, FixedDirectoryContract] => [id, { ...common, pathPattern: path, scope: { kind: "exact-path", path } }]));
    const scopeSchema = { type: "object", additionalProperties: false, required: ["kind", "fixedDirectoryContractIds"], properties: { kind: { const: "fixed-directory-contract-set" }, fixedDirectoryContractIds: { type: "array", minItems: 1, maxItems: 256, uniqueItems: true, items: { enum: Object.keys(parents) } } } };
    const validate = new Ajv({ strict: true }).compile(scopeSchema);
    for (const candidate of [vector.accepted, ...vector.rejected]) {
      if (validate(candidate)) expect(parseFixedDirectoryContractSetScope(candidate, parents)).toEqual<typeof candidate>(candidate);
      else expect(() => parseFixedDirectoryContractSetScope(candidate, parents)).toThrow();
    }
    expect(() => parseFixedDirectoryContractSetScope(vector.accepted, { ...parents, right: { ...parents.right, pathPattern: "**/interfaces" } })).toThrow();
    expect(() => parseFixedDirectoryContractSetScope(vector.accepted, { ...parents, right: { ...parents.left } })).toThrow();
    expect(parseNamedFixedDirectoryContractSetScope(vector.namedAccepted, parents, vector.namedSets)).toEqual(vector.accepted);
    const namedValidate = new Ajv({ strict: true }).compile({ type: "object", additionalProperties: false, required: ["kind", "fixedDirectoryContractSetId"], properties: { kind: { const: "named-fixed-directory-contract-set" }, fixedDirectoryContractSetId: { enum: Object.keys(vector.namedSets) } } });
    expect(namedValidate(vector.namedAccepted)).toBe(true);
    for (const candidate of vector.namedRejected) {
      expect(namedValidate(candidate)).toBe(false);
      expect(() => parseNamedFixedDirectoryContractSetScope(candidate, parents, vector.namedSets)).toThrow();
    }
    for (const ids of [[], ["left", "left"], ["left", "unknown"]]) expect(() => parseNamedFixedDirectoryContractSetScope(vector.namedAccepted, parents, { "fixture-compilers": ids })).toThrow();
    const taxonomy = probeClone(loadTaxonomy());
    Object.assign(taxonomy.fixedDirectoryContracts, parents);
    const contract: FixedFilenameContract = { ...common, pathPattern: "**/wasi-io-poll.d.ts", scope: vector.accepted };
    taxonomy.fixedFilenameContracts["fixture-interface"] = contract;
    taxonomy.packageSourceDispositions["fixture-interface"] = { contractKind: "fixed", disposition: "adapter-source", validator: "package-glue", authority: "fixture", verification: "fixture" } satisfies PackageSourceDisposition;
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    const source = normalizationEngineDeclarations(join(contractRoot, "../../🧹️normalization"));
    const syntax = ts.createSourceFile("🟦️.ts", source, ts.ScriptTarget.Latest, true);
    const declaration = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "fixedScopeMatches");
    expect(declaration).toBeDefined();
    const normalizer = new Function("dirname", new Bun.Transpiler({ loader: "ts" }).transformSync(declaration!.getText(syntax) + "\nreturn fixedScopeMatches;"))(dirname);
    for (const row of vector.matches) {
      const path = (vector.parents[row.parent] ?? "other/interfaces") + "/" + row.filename;
      const ids = fixedFilenameContractIdsForPath(path, taxonomy, { parentFixedDirectoryContractIds: [row.parent] });
      expect(ids.includes("fixture-interface")).toBe(row.accepted);
      expect(taxonomyPathPatternMatches(path, contract.pathPattern) && normalizer(contract, path, null, undefined, row.parent)).toBe(row.accepted);
    }
    expect(fixedFilenameContractIdsForPath(vector.parents.left + "/wasi-io-poll.d.ts", taxonomy)).toEqual([]);
    taxonomy.fixedDirectoryContractSets = { ...taxonomy.fixedDirectoryContractSets, ...vector.namedSets };
    taxonomy.fixedFilenameContracts["fixture-interface"].scope = vector.namedAccepted;
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    const resolver = createFixedContractResolver(taxonomy);
    for (const row of vector.matches) {
      const path = (vector.parents[row.parent] ?? "other/interfaces") + "/" + row.filename;
      const context = { parentFixedDirectoryContractIds: [row.parent] };
      expect(fixedFilenameContractIdsForPath(path, taxonomy, context).includes("fixture-interface")).toBe(row.accepted);
      expect(resolver.filenameIdsForPath(path, context).includes("fixture-interface")).toBe(row.accepted);
      const normalized = { ...contract, scope: parseNamedFixedDirectoryContractSetScope(vector.namedAccepted, parents, vector.namedSets) };
      expect(taxonomyPathPatternMatches(path, normalized.pathPattern) && normalizer(normalized, path, null, undefined, row.parent)).toBe(row.accepted);
    }
    for (const name of vector.invalidLiteralBasenames) {
      const invalid = structuredClone(taxonomy);
      invalid.fixedFilenameContracts["fixture-interface"].pathPattern = "**/" + name;
      expect(validateTaxonomy(invalid).some((problem) => problem.includes("one exact literal basename"))).toBe(true);
    }
    taxonomy.fixedFilenameContracts["fixture-interface"].scope = vector.accepted;
    taxonomy.fixedFilenameContracts["ambiguous-interface"] = structuredClone(contract);
    expect(() => fixedFilenameContractIdsForPath(vector.parents.left + "/wasi-io-poll.d.ts", taxonomy, { parentFixedDirectoryContractIds: ["left"] })).toThrow(/equal-specificity/);
  }, 30_000);
});

describe("fresh-clone generator outputs", () => {
  test("every declared-tracked output is visible to a clone and every external input is tracked", () => {
    const taxonomy = loadCatalogTaxonomy();
    expect(validateTaxonomy(taxonomy).filter((problem) => problem.includes("no clone has it"))).toEqual([]);
    const outputs = Object.values(taxonomy.generatorContracts).flatMap((contract) => contract.outputRoots.map((output) => ({ ownership: contract.ownership, ...output })));
    const tracked = outputs.filter((output) => output.inclusion === "tracked").map((output) => output.path);
    expect(outputs.filter((output) => output.ownership === "external" && output.inclusion !== "tracked")).toEqual([]);
    const probes = tracked.map((path) => (existsSync(join(getWorkspaceRoot(), path)) && lstatSync(join(getWorkspaceRoot(), path)).isDirectory() ? `${path}/probe` : path));
    const oracle = Bun.spawnSync(["git", "check-ignore", "--no-index", "--stdin"], { cwd: getWorkspaceRoot(), stdin: new TextEncoder().encode(`${probes.join("\n")}\n`), stdout: "pipe", stderr: "pipe" });
    expect(oracle.stdout.toString().split("\n").filter(Boolean)).toEqual([]);
    const violated = probeClone(taxonomy) as unknown as Taxonomy;
    const scale = violated.generatorContracts["scale-fixture"]!.outputRoots[0] as { inclusion: string };
    scale.inclusion = "tracked";
    const shortcodes = violated.generatorContracts["external-emoji-shortcodes"]!.outputRoots[0] as { inclusion: string };
    shortcodes.inclusion = "ignored";
    const problems = validateTaxonomy(violated).filter((problem) => problem.includes("no clone has it"));
    expect(problems.some((problem) => problem.includes("scale-fixture") && problem.includes("inside 🤖️generated"))).toBe(true);
    expect(problems.some((problem) => problem.includes("external-emoji-shortcodes") && problem.includes("external input"))).toBe(true);
  }, 60_000);
});

describe("materialized JCO interface filename boundaries", () => {
  test("admits the exact physical matrix through indexed and direct resolvers", async () => {
    const discovery = { ...(await import("../../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts")), ...(await import("../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts")) }, ts = await import("typescript"), picomatch = (await import("picomatch")).default;
    const taxonomy = loadTaxonomy(), root = findWorkspaceRoot(contractRoot), resolver = discovery.createFixedContractResolver(taxonomy), pathMatcher = discovery.createTaxonomyPathMatcher();
    const contracts = Object.entries(taxonomy.fixedFilenameContracts).filter(([id]) => id.startsWith("dev-jco-interface-"));
    const parentIds = taxonomy.fixedDirectoryContractSets!["dev-jco-all-interfaces"];
    expect(contracts).toHaveLength(31);
    expect(parentIds).toHaveLength(69);
    const normalSource = normalizationEngineDeclarations(join(contractRoot, "../../🧹️normalization"));
    const syntax = ts.createSourceFile("🟦️.ts", normalSource, ts.ScriptTarget.Latest, true);
    const declaration = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "fixedScopeMatches");
    const normalizer = new Function("dirname", new Bun.Transpiler({ loader: "ts" }).transformSync(declaration!.getText(syntax) + "\nreturn fixedScopeMatches;"))(dirname);
    const options = { allowJs: true, moduleResolution: ts.ModuleResolutionKind.Bundler, module: ts.ModuleKind.ESNext };
    let physicalFiles = 0;
    for (const parentId of parentIds) {
      const parent = taxonomy.fixedDirectoryContracts[parentId].pathPattern, physical = readdirSync(join(root, parent)).sort();
      expect(resolver.directoryIdsForPath(parent)).toEqual([parentId]);
      const names: string[] = [];
      for (const [id, contract] of contracts) {
        const filename = discovery.fixedContractFilename(contract), path = parent + "/" + filename;
        const expected = physical.includes(filename), context = { parentFixedDirectoryContractIds: [parentId], pathMatcher };
        const resolvedScope = discovery.parseNamedFixedDirectoryContractSetScope(contract.scope, taxonomy.fixedDirectoryContracts, taxonomy.fixedDirectoryContractSets!);
        expect(picomatch(contract.pathPattern)(path) && resolvedScope.fixedDirectoryContractIds.includes(parentId)).toBe(expected);
        expect(discovery.fixedFilenameContractIdsForPath(path, taxonomy, context).includes(id)).toBe(expected);
        expect(resolver.filenameIdsForPath(path, context).includes(id)).toBe(expected);
        expect(discovery.taxonomyPathPatternMatches(path, contract.pathPattern) && normalizer({ ...contract, scope: resolvedScope }, path, null, undefined, parentId)).toBe(expected);
        if (expected) names.push(filename);
      }
      expect(names.sort()).toEqual(physical);
      physicalFiles += physical.length;
      for (const filename of physical) {
        const stem = filename.slice(0, -".d.ts".length);
        expect(ts.resolveModuleName("./" + stem + ".js", join(root, parent, "consumer.ts"), options, ts.sys).resolvedModule?.resolvedFileName.replaceAll("\\", "/")).toBe(join(root, parent, filename).replaceAll("\\", "/"));
      }
      for (const filename of ["custom.d.ts", "semio-framework-types.d.ts.extra", "🧪️custom.d.ts"]) expect(resolver.filenameIdsForPath(parent + "/" + filename, { parentFixedDirectoryContractIds: [parentId] })).toEqual([]);
    }
    expect(physicalFiles).toBe(1717);
    for (const [id, contract] of contracts) {
      const path = "unknown/interfaces/" + discovery.fixedContractFilename(contract);
      expect(resolver.filenameIdsForPath(path).includes(id)).toBe(false);
      expect(discovery.fixedFilenameContractIdsForPath(path, taxonomy).includes(id)).toBe(false);
    }
  }, 20_000);
});

describe("materialized JCO companion boundaries", () => {
  test("preserves only the 69 exact compiler-linked triples", async () => {
    const discovery = await import("../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts"), ts = await import("typescript"), picomatch = (await import("picomatch")).default;
    const taxonomy = loadTaxonomy(), root = findWorkspaceRoot(contractRoot), resolver = discovery.createFixedContractResolver(taxonomy);
    const contracts = Object.entries(taxonomy.fixedFilenameContracts).filter(([id]) => /^dev-(plugin|extension)-component-/u.test(id));
    expect(contracts).toHaveLength(207);
    const options = { allowJs: true, moduleResolution: ts.ModuleResolutionKind.Bundler, module: ts.ModuleKind.ESNext };
    for (const [id, contract] of contracts) {
      const path = contract.pathPattern;
      expect(existsSync(join(root, path)), id).toBe(true);
      expect(resolver.filenameIdsForPath(path)).toEqual([id]);
      for (const candidate of [path, "elsewhere/" + basename(path), path + ".backup", dirname(path) + "/🧾️custom.js", dirname(path) + "/🌉️bridge.js"]) {
        expect(picomatch(contract.pathPattern)(candidate)).toBe(candidate === path);
        expect(resolver.filenameIdsForPath(candidate).includes(id)).toBe(candidate === path);
      }
      if (id.endsWith("-js")) {
        const declaration = path.slice(0, -3) + ".d.ts", core = basename(path).slice(0, -3) + ".core.wasm";
        expect(ts.resolveModuleName("./" + basename(path), join(root, dirname(path), "consumer.ts"), options, ts.sys).resolvedModule?.resolvedFileName.replaceAll("\\", "/")).toBe(join(root, declaration).replaceAll("\\", "/"));
        expect(readFileSync(join(root, path), "utf8")).toContain(core);
      }
      if (id.endsWith("-wasm")) expect(new Uint8Array(await Bun.file(join(root, path)).slice(0, 8).arrayBuffer())).toEqual(new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]));
      else expect(taxonomy.packageSourceDispositions[id]).toMatchObject({ contractKind: "fixed", disposition: "adapter-source", validator: "package-glue" });
    }
    for (const parentId of taxonomy.fixedDirectoryContractSets!["dev-jco-all-interfaces"]) {
      const parent = dirname(taxonomy.fixedDirectoryContracts[parentId].pathPattern);
      const physical = readdirSync(join(root, parent)).filter((name) => name.includes("_component.")).sort();
      expect(contracts.filter(([, row]) => dirname(row.pathPattern) === parent).map(([, row]) => basename(row.pathPattern)).sort()).toEqual(physical);
    }
  }, 20_000);
});

describe("Flow compiler output boundaries", () => {
  test("uses handpicked output owners and preserves exact compiler companion pairing", async () => {
    const ts = await import("typescript");
    const { parse: parseJsonc } = await import("jsonc-parser");
    const root = findWorkspaceRoot(contractRoot);
    const flow = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow");
    const text = readFileSync(join(flow, "🎁️browser-package/🧪️bindings.json"), "utf8"), vector = JSON.parse(text);
    expect(parseJsonc(text)).toEqual(vector);
    expect(wasmOutputDirectory(flow, vector.outputDirectory)).toBe(join(flow, vector.outputDirectory));
    for (const outputDirectory of vector.invalidDirectories) expect(() => wasmOutputDirectory(flow, outputDirectory)).toThrow();
    const taxonomy = loadTaxonomy();
    const options = { allowJs: true, moduleResolution: ts.ModuleResolutionKind.Bundler, module: ts.ModuleKind.ESNext };
    for (const parent of vector.owners) {
      const owner = join(flow, parent, vector.outputDirectory);
      expect(existsSync(join(flow, parent, "pkg"))).toBe(false);
      expect(ts.resolveModuleName("./flow_core.js", join(owner, "consumer.ts"), options, ts.sys).resolvedModule?.resolvedFileName.replaceAll("\\", "/")).toBe(join(owner, "flow_core.d.ts").replaceAll("\\", "/"));
      expect(readFileSync(join(owner, "flow_core.js"), "utf8")).toContain("flow_core_bg.wasm");
      for (const name of vector.companions) {
        const path = relative(root, join(owner, name)).replaceAll("\\", "/");
        expect(fixedFilenameContractIdsForPath(path, taxonomy)).toHaveLength(1);
        expect(fixedFilenameContractIdsForPath(path + ".extra", taxonomy)).toEqual([]);
        expect(fixedFilenameContractIdsForPath("other/" + path, taxonomy)).toEqual([]);
        expect(readFileSync(join(owner, name))).toEqual(readFileSync(join(flow, vector.owners[0], vector.outputDirectory, name)));
      }
      expect(fixedFilenameContractIdsForPath(relative(root, join(owner, "custom.js")).replaceAll("\\", "/"), taxonomy)).toEqual([]);
    }
  });
});
