import { expect, test } from "bun:test";
import { cruise } from "dependency-cruiser";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import ts from "typescript";
import { dependencyDirectionEdges, dependencyDirectionSourceInventory, dependencyDirectionWorkspacePackages, type DependencyDirectionGraphScope } from "../../🕸️dependencies/🧭️direction/🟦️.ts";

type Edge = Readonly<{ id: string; target: string; reference: "relative" | "package" | "package-subpath" | "unresolved-package"; syntax: "static" | "type" | "dynamic" | "export" | "require"; package?: string }>;
type Case = Readonly<{ id: string; from: string; comments?: readonly string[]; dependencies: readonly Edge[]; forbidden: readonly string[] }>;
type Rule = Readonly<{ name: string; severity: string; from: { path: string[]; pathNot?: string[] }; to: { path: string[]; pathNot?: string[] } }>;
type Graph = Readonly<{ modules: readonly { source: string; dependencies: readonly { module: string; resolved: string; couldNotResolve?: boolean }[] }[]; summary: { violations: readonly { from: string; to: string; rule: { name: string } }[] } }>;
const library = resolve(import.meta.dir, "../.."), repo = resolve(library, "../../../../..");
const read = (path: string): unknown => JSON.parse(readFileSync(join(library, path), "utf8"));
type RemovabilityCase = Readonly<{ id: string; directories: readonly string[]; manifests: Readonly<Record<string, string | object>>; expectedPackages: readonly string[]; expectedPlugins: readonly string[]; accept: boolean }>;
const fixture = read("🧫️fixtures/🧱️dependency-direction/🔣️.json") as { schemaVersion: number; infrastructureCases: readonly { owner: string; name: string; commands: readonly string[] }[]; typeCases: readonly { id: string; source: string; name: string; value: unknown; accept: boolean }[]; publicExportCases: readonly { id: string; owner: string; specifier: string; accept: boolean; conditions: readonly string[]; target?: string }[]; removabilityCases: readonly RemovabilityCase[]; cases: readonly Case[]; graphScope: DependencyDirectionGraphScope; resolutionCases: readonly { id: string; specifier: string; owner: string; from: string; forbidden: number; authored: boolean; accept: boolean }[]; inventoryCases: readonly { id: string; accept: boolean; roots: readonly string[]; files: readonly string[]; links: readonly { path: string; target: string }[]; expectedSources: readonly string[] }[]; graphCases: readonly { id: string; accept: boolean }[] };
const schema = read("🧬️schema/🧱️dependency-direction/🔣️.json");
const config = createRequire(import.meta.url)(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs")) as { forbidden: readonly Rule[] };
const rules = config.forbidden.filter((rule) => ["framework-no-implementation", "io-renderer-independent", "repo-no-implementation", "s-modules-no-plugins", "plugin-no-extension-or-artifact-📐️cad"].includes(rule.name)).map((rule) => {
  const patterns = (value: string | string[]): string[] => typeof value === "string" ? [value] : value;
  return { ...rule, from: { path: patterns(rule.from.path), ...(rule.from.pathNot ? { pathNot: patterns(rule.from.pathNot) } : {}) }, to: { path: patterns(rule.to.path), ...(rule.to.pathNot ? { pathNot: patterns(rule.to.pathNot) } : {}) } };
});
const matches = (patterns: readonly string[], candidate: string): boolean => patterns.some((pattern) => new RegExp(pattern, "u").test(candidate));
const write = (root: string, path: string, content: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); };

/** 🧭️ Renders the same neutral dependency intent into the JavaScript parser's supported import forms. */
function render(edge: Edge, from: string): { specifier: string; source: string } {
  const local = relative(dirname(from), edge.target).split("\\").join("/");
  const specifier = edge.reference === "relative" ? local.startsWith(".") ? local : `./${local}` : `${edge.package}${edge.reference === "package-subpath" ? "/schema" : ""}`;
  const literal = JSON.stringify(specifier);
  const sources = {
    static: `import { value } from ${literal}; export const used = value;`,
    type: `import type { Value } from ${literal}; export type Used = Value;`,
    dynamic: `export const used = import(${literal});`,
    export: `export { value } from ${literal};`,
    require: `export const used = require(${literal});`,
  };
  return { specifier, source: sources[edge.syntax] };
}

test("schema defines unique neutral dependency cases and exact expected edges", () => {
  const validate = new Ajv({ strict: true }).compile(schema as object);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map((row) => row.id)).size).toBe(fixture.cases.length);
  for (const row of fixture.cases) {
    expect(new Set(row.dependencies.map((edge) => edge.id)).size).toBe(row.dependencies.length);
    expect(row.forbidden.every((id) => row.dependencies.some((edge) => edge.id === id))).toBe(true);
  }
});

test("declared semantic roles satisfy the portable schema and package contributions", () => {
  const taxonomy = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8"));
  const policySchema = (schema as { $defs: { DependencyDirections: object } }).$defs.DependencyDirections;
  const validate = new Ajv({ strict: true }).compile(policySchema);
  expect(validate(taxonomy.dependencyDirections), JSON.stringify(validate.errors)).toBe(true);
  for (const rule of Object.values(taxonomy.dependencyDirections.rules) as { fromRoles: string[]; toRoles: string[] }[]) {
    expect([...rule.fromRoles, ...rule.toRoles].every((role) => taxonomy.dependencyDirections.roles[role])).toBe(true);
  }
  const manifest = JSON.parse(readFileSync(join(repo, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json"), "utf8"));
  expect(manifest.semio.dependencyRole).toBe("ui");
});

test("strict taxonomy direction covers paths, package aliases, subpaths, tests and scripts", () => {
  expect(rules).toHaveLength(5);
  expect(rules.every((rule) => rule.severity === "error")).toBe(true);
  for (const row of fixture.cases) {
    const forbidden = row.dependencies.filter((edge) => rules.some((rule) => matches(rule.from.path, row.from) && !matches(rule.from.pathNot ?? [], row.from) && matches(rule.to.path, edge.reference === "relative" ? edge.target : `${edge.package}${edge.reference === "package-subpath" ? "/schema" : ""}`))).map((edge) => edge.id);
    expect(forbidden, row.id).toEqual([...row.forbidden]);
  }
});

test("dependency-cruiser independently parses and resolves every neutral fixture verdict", async () => {
  expect(rules).toHaveLength(5);
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-direction-")));
  try {
    for (const [index, row] of fixture.cases.entries()) {
      const cwd = join(root, String(index));
      const inputs = row.dependencies.map((edge) => ({ edge, ...render(edge, row.from) }));
      write(cwd, row.from, [...(row.comments ?? []).map((comment) => `// ${comment}`), ...inputs.map((input) => input.source), "export {};"].join("\n"));
      write(cwd, "package.json", JSON.stringify({ type: "module", dependencies: Object.fromEntries(row.dependencies.filter((edge) => edge.package).map((edge) => [edge.package, "*"])) }));
      for (const { edge } of inputs) {
        const target = "export interface Value {}\nexport const value = 1;\n";
        write(cwd, edge.target, target);
        if (edge.reference === "package" || edge.reference === "package-subpath") {
          const packageRoot = `node_modules/${edge.package}`;
          write(cwd, `${packageRoot}/package.json`, JSON.stringify({ name: edge.package, type: "module", exports: { ".": "./entry.ts", "./schema": "./schema.ts" } }));
          write(cwd, `${packageRoot}/entry.ts`, target);
          write(cwd, `${packageRoot}/schema.ts`, target);
        }
      }
      const oracle = await cruise([row.from], { baseDir: cwd, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, doNotFollow: { path: "node_modules" } }, { exportsFields: ["exports"], conditionNames: ["import", "require", "node", "default"], modules: [join(cwd, "node_modules")], bustTheCache: true });
      expect(oracle.exitCode, row.id).toBe(0);
      const graph = (typeof oracle.output === "string" ? JSON.parse(oracle.output) : oracle.output) as Graph;
      const scope = { ...fixture.graphScope, workspaceRoots: ["🧰️framework", "🌎️hub", "✏️s", "♻️mit-bestand"], expectedSources: [row.from] };
      if (inputs.some((input) => input.edge.reference === "unresolved-package" && input.edge.package?.startsWith("@semio-tech/"))) expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).toThrow();
      else expect(dependencyDirectionEdges(graph, rules, scope), row.id).toHaveLength(row.forbidden.length);
      const module = graph.modules.find((module) => module.source === row.from);
      expect(module, row.id).toBeDefined();
      expect(module!.dependencies, row.id).toHaveLength(inputs.length);
      const actual = inputs.filter(({ specifier }) => {
        const dependency = module!.dependencies.find((dependency) => dependency.module === specifier);
        expect(dependency, row.id).toBeDefined();
        expect(dependency!.couldNotResolve === true, row.id).toBe(inputs.find((input) => input.specifier === specifier)!.edge.reference === "unresolved-package");
        return graph.summary.violations.some((violation) => violation.from === row.from && violation.to === dependency!.resolved && rules.some((rule) => violation.rule.name === rule.name));
      }).map(({ edge }) => edge.id);
      expect(actual, row.id).toEqual([...row.forbidden]);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);


test("resolver graph validation rejects omissions, corrupt totals and contradictory verdicts", () => {
  const from = "🧰️framework/🔨️modules/🧵️job/🟦️.ts", peer = "🧰️framework/🔨️modules/🧬️schema/🟦️.ts", implementation = "🌎️hub/🟦️.ts";
  for (const row of fixture.graphCases) {
    const graph: any = { modules: [{ source: from, dependencies: [{ module: "./peer", resolved: peer }] }, { source: peer, dependencies: [] }], summary: { totalCruised: 2, totalDependenciesCruised: 1, error: 0, violations: [], ruleSetUsed: { forbidden: rules } } };
    if (row.id === "complete-forbidden" || row.id === "unreported-forbidden-edge") {
      graph.modules[0].dependencies[0].resolved = implementation;
      graph.modules.push({ source: implementation, dependencies: [] });
      graph.summary.totalCruised++;
    }
    if (row.id === "complete-forbidden" || row.id === "phantom-violation") { graph.summary.violations = [{ from, to: implementation, rule: { name: rules[0]!.name } }]; graph.summary.error = 1; }
    if (row.id === "missing-modules") delete graph.modules;
    if (row.id === "empty-modules") { graph.modules = []; graph.summary.totalCruised = 0; }
    if (row.id === "wrong-module-total") graph.summary.totalCruised++;
    if (row.id === "wrong-dependency-total") graph.summary.totalDependenciesCruised++;
    if (row.id === "missing-resolved-path") delete graph.modules[0].dependencies[0].resolved;
    if (row.id === "missing-import-specifier") delete graph.modules[0].dependencies[0].module;
    if (row.id === "unknown-policy") graph.summary.ruleSetUsed.forbidden = [{ name: "other" }];
    if (row.id === "duplicate-source") graph.modules[1].source = from;
    if (row.id === "complete-two-rules") {
      const io = "🧰️framework/🔨️modules/🚪️io/🟦️.ts", ui = "🧰️framework/🔨️modules/🖱️ui/🟦️.ts";
      graph.modules[0] = { source: io, dependencies: [{ module: "react", resolved: ui }] };
      graph.modules[1] = { source: from, dependencies: [{ module: "hub", resolved: implementation }] };
      graph.modules.push({ source: peer, dependencies: [] }, { source: ui, dependencies: [] }, { source: implementation, dependencies: [] });
      graph.summary.totalCruised = graph.modules.length;
      graph.summary.totalDependenciesCruised = 2;
      graph.summary.error = 2;
      graph.summary.violations = [{ from: io, to: ui, rule: { name: "io-renderer-independent" } }, { from, to: implementation, rule: { name: "framework-no-implementation" } }];
    }
    let scope = fixture.graphScope;
    if (row.id === "missing-followed-local-source" || row.id === "fake-nonfollowed-local-source") {
      graph.modules.pop(); graph.summary.totalCruised--;
      scope = { ...scope, expectedSources: [from] };
      if (row.id === "fake-nonfollowed-local-source") Object.assign(graph.modules[0].dependencies[0], { followable: false, matchesDoNotFollow: true });
    }
    if (row.id === "missing-disconnected-root") scope = { ...scope, expectedSources: [...scope.expectedSources, "🧰️framework/orphan.ts"] };
    if (row.id === "unresolved-local-source") {
      Object.assign(graph.modules[0].dependencies[0], { couldNotResolve: true });
      graph.modules.pop(); graph.summary.totalCruised--;
      scope = { ...scope, expectedSources: [from] };
    }
    if (row.id === "missing-source-inventory") scope = { ...scope, expectedSources: [] };
    if (row.id === "missing-scope-metadata") scope = undefined as unknown as DependencyDirectionGraphScope;
    if (row.id === "invalid-scope-pattern") scope = { ...scope, excludedPaths: ["["] };
    const terminal: Record<string, { module: string; resolved: string; couldNotResolve?: boolean; coreModule?: boolean; followable?: boolean }> = {
      "terminal-vendor": { module: "vendor", resolved: "node_modules/vendor/entry.ts", followable: false },
      "terminal-builtin": { module: "node:fs", resolved: "fs", coreModule: true, followable: false },
      "terminal-unresolved-relative": { module: "./absent.json", resolved: "./absent.json", couldNotResolve: true, followable: false },
      "terminal-unresolved-package": { module: "absent-package", resolved: "absent-package", couldNotResolve: true, followable: false },
      "terminal-json": { module: "./schema.json", resolved: "🧰️framework/schema.json", followable: false },
      "terminal-policy-exclusion": { module: "./generated.ts", resolved: "🧰️framework/🗑️generated/leaf.ts" },
      "terminal-policy-nonfollow": { module: "./nonfollow.ts", resolved: "🧰️framework/nonfollowed/leaf.ts" },
      "unresolved-workspace-alias": { module: "@neutral/framework", resolved: "@neutral/framework", couldNotResolve: true, followable: false },
      "unresolved-workspace-subpath": { module: "@neutral/framework/schema", resolved: "@neutral/framework/schema", couldNotResolve: true, followable: false },
    };
    if (terminal[row.id]) graph.modules[0].dependencies[0] = terminal[row.id];
    if (row.id === "multiple-unresolved-local-sources") {
      graph.modules[0].dependencies = [{ module: "./missing-first.ts", resolved: "./missing-first.ts", couldNotResolve: true }, { module: "./missing-second.ts", resolved: "./missing-second.ts", couldNotResolve: true }];
      graph.summary.totalDependenciesCruised = 2;
      for (const target of ["missing-first.ts", "missing-second.ts"]) expect(() => dependencyDirectionEdges(graph, rules, scope), target).toThrow(target);
    }
    if (row.id === "identical-leaf-records") { graph.modules.push({ source: peer, dependencies: [] }); graph.summary.totalCruised++; }
    if (row.accept) expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).not.toThrow();
    else expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).toThrow();
  }
});

test("dependency-cruiser unresolved local terminals fail and authored aliases retain ownership", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-resolution-")));
  try {
    for (const [index, row] of fixture.resolutionCases.entries()) {
      const cwd = join(root, String(index)), entry = row.from;
      write(cwd, entry, `import ${JSON.stringify(row.specifier)}; export {};`);
      const result = await cruise([entry], { baseDir: cwd, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", combinedDependencies: true }, { bustTheCache: true });
      const graph = typeof result.output === "string" ? JSON.parse(result.output) : result.output;
      expect(result.exitCode, row.id).toBe(0);
      expect(graph.modules.find((module: { source: string }) => module.source === entry).dependencies[0].couldNotResolve, row.id).toBe(true);
      const scope = { ...fixture.graphScope, workspacePackages: row.authored ? [{ name: row.specifier.startsWith("@") ? row.specifier.split("/").slice(0, 2).join("/") : row.specifier.split("/")[0]!, owner: row.owner, exports: [".", "./schema"] }] : [], expectedSources: [entry] };
      expect(graph.summary.violations.length, row.id).toBe(row.forbidden);
      if (row.accept) expect(dependencyDirectionEdges(graph, rules, scope), row.id).toHaveLength(row.forbidden);
      else expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).toThrow();
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);


test("independent source inventory and dependency-cruiser reject self-consistent root and target omissions", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-completeness-")));
  try {
    for (const [index, row] of fixture.inventoryCases.entries()) {
      const cwd = join(root, String(index));
      for (const path of row.files) write(cwd, path, path.endsWith(".json") ? "{}" : "export {};\n");
      for (const link of row.links) {
        mkdirSync(dirname(join(cwd, link.path)), { recursive: true });
        symlinkSync(join(cwd, link.target), join(cwd, link.path), process.platform === "win32" ? "junction" : "dir");
      }
      if (!row.accept) {
        expect(() => dependencyDirectionSourceInventory(cwd, row.roots, { ...fixture.graphScope, expectedSources: row.expectedSources }), row.id).toThrow();
        await expect(cruise([...row.roots], { baseDir: cwd, outputType: "json", exclude: { path: [...fixture.graphScope.excludedPaths] } }, { bustTheCache: true })).rejects.toThrow();
        continue;
      }
      const entry = row.expectedSources[0]!, peer = row.expectedSources[1]!;
      const target = relative(dirname(entry), peer).replaceAll("\\", "/");
      write(cwd, entry, `import "./${target}"; import "node:fs"; import "./schema.json"; export {};`);
      const scope = { ...fixture.graphScope, expectedSources: row.expectedSources };
      expect(dependencyDirectionSourceInventory(cwd, row.roots, scope), row.id).toEqual([...row.expectedSources]);
      const result = await cruise([...row.roots], { baseDir: cwd, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, exclude: { path: [...scope.excludedPaths] }, doNotFollow: { path: scope.nonFollowedPaths.join("|") } }, { bustTheCache: true });
      const graph = typeof result.output === "string" ? JSON.parse(result.output) : result.output;
      expect(result.exitCode).toBe(0);
      expect(dependencyDirectionEdges(graph, rules, scope)).toEqual([]);
      expect(graph.modules.map((module: { source: string }) => module.source).filter((source: string) => row.expectedSources.includes(source)).sort()).toEqual([...row.expectedSources]);
      for (const missing of row.expectedSources.slice(1)) {
        const incomplete = structuredClone(graph);
        incomplete.modules = incomplete.modules.filter((module: { source: string }) => module.source !== missing);
        incomplete.summary.totalCruised = incomplete.modules.length;
        incomplete.summary.totalDependenciesCruised = incomplete.modules.reduce((count: number, module: { dependencies: unknown[] }) => count + module.dependencies.length, 0);
        expect(() => dependencyDirectionEdges(incomplete, rules, scope), `${row.id}: missing ${missing}`).toThrow();
        if (missing === peer) expect(() => dependencyDirectionEdges(incomplete, rules, { ...scope, expectedSources: [entry] }), "followed target closure independently of inventory").toThrow();
      }
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);


test("portable removability cases agree with actual boundary policy loading", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-removability-")));
  const boundary = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs";
  const owner = "✏️s/🔌️plugins/🧪️example/📦️packages/🟦️typescript";
  try {
    for (const [index, row] of fixture.removabilityCases.entries()) {
      const cwd = join(root, String(index));
      write(cwd, boundary, readFileSync(join(repo, boundary), "utf8"));
      write(cwd, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", readFileSync(join(library, "🔣️taxonomy.json"), "utf8"));
      write(cwd, "package.json", JSON.stringify({ workspaces: [owner, "✏️s/🔌️plugins/🧪️example/🗿️artifacts/🧪️example/📦️packages/🟦️typescript"] }));
      for (const directory of row.directories) mkdirSync(join(cwd, directory), { recursive: true });
      for (const [directory, manifest] of Object.entries(row.manifests)) write(cwd, `${directory}/package.json`, typeof manifest === "string" ? manifest : JSON.stringify(manifest));
      const load = (): { forbidden: readonly Rule[] } => createRequire(import.meta.url)(join(cwd, boundary));
      if (!row.accept) {
        expect(() => dependencyDirectionWorkspacePackages(cwd, []), row.id).toThrow();
        expect(load, row.id).toThrow();
        continue;
      }
      expect(dependencyDirectionWorkspacePackages(cwd, []).map((pkg) => pkg.name), row.id).toEqual([...row.expectedPackages]);
      const policy = load();
      expect(policy.forbidden.filter((rule) => rule.name.startsWith("plugin-no-extension-or-artifact-")).map((rule) => rule.name.slice("plugin-no-extension-or-artifact-".length)), row.id).toEqual([...row.expectedPlugins]);
      const neutralRule = policy.forbidden.find((rule) => rule.name === "framework-no-implementation")!;
      expect(neutralRule.severity, row.id).toBe("error");
      write(cwd, "🧰️framework/🟦️.ts", "export {};\n");
      const oracle = await cruise(["🧰️framework/🟦️.ts"], { baseDir: cwd, validate: true, ruleSet: { forbidden: policy.forbidden.map((rule) => ({ ...rule, severity: rule.severity as "error" | "warn" })) }, outputType: "json" }, { bustTheCache: true });
      const graph = typeof oracle.output === "string" ? JSON.parse(oracle.output) : oracle.output;
      expect(oracle.exitCode, row.id).toBe(0);
      expect(graph.summary.violations, row.id).toEqual([]);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});

/** 🏷️ Authored public subpaths resolve independently while private and pseudo-package names fail. */
test("authored owner exports agree with independent package resolution", async () => {
  const policy = createRequire(import.meta.url)(join(repo, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs"));
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-public-exports-")));
  try {
    for (const [index, row] of fixture.publicExportCases.entries()) {
      const cwd = join(root, String(index)), from = "♻️mit-bestand/consumer.ts";
      const manifest = JSON.parse(readFileSync(join(repo, row.owner, "package.json"), "utf8"));
      const owner = `node_modules/${manifest.name}`;
      write(cwd, `${owner}/package.json`, JSON.stringify(manifest));
      const targets = (entry: unknown): string[] => typeof entry === "string" ? [entry] : entry && typeof entry === "object" ? Object.values(entry).flatMap(targets) : [];
      for (const target of targets(manifest.exports)) write(cwd, `${owner}/${target}`, "export const value = 1;\n");
      write(cwd, from, `import { value } from ${JSON.stringify(row.specifier)}; export const used = value;`);
      const result = await cruise([from], { baseDir: cwd, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, doNotFollow: { path: "node_modules" } }, { exportsFields: ["exports"], conditionNames: [...row.conditions], modules: [join(cwd, "node_modules")], bustTheCache: true });
      const graph = typeof result.output === "string" ? JSON.parse(result.output) : result.output;
      expect(result.exitCode, row.id).toBe(0);
      const dependency = graph.modules.find((module: { source: string }) => module.source === from).dependencies[0];
      expect(dependency.couldNotResolve !== true, row.id).toBe(row.accept);
      if (row.target) {
        expect(policy.options.enhancedResolveOptions.conditionNames, row.id).toEqual([...row.conditions]);
        expect(dependency.resolved, row.id).toBe(`${owner}/${row.target.slice(2)}`);
      }
      const scope = { ...fixture.graphScope, workspaceRoots: [...fixture.graphScope.workspaceRoots, "♻️mit-bestand", "✏️s"], workspacePackages: [{ name: manifest.name, owner: row.owner, exports: Object.keys(manifest.exports) }], expectedSources: [from] };
      if (row.accept) expect(dependencyDirectionEdges(graph, rules, scope), row.id).toEqual([]);
      else expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).toThrow();
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

/** 🔬️ An independent compiler checks authored wire declarations against portable JSON values. */
test("authored schema type declarations accept the portable wire values", () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-wire-types-")));
  try {
    for (const [index, row] of fixture.typeCases.entries()) {
      const entry = join(root, `${index}.ts`);
      writeFileSync(entry, `import type { ${row.name} } from ${JSON.stringify(join(repo, row.source))}; const value: ${row.name} = ${JSON.stringify(row.value)}; void value;`);
      const program = ts.createProgram([entry], { noEmit: true, strict: true, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ES2022, types: [] });
      const diagnostics = ts.getPreEmitDiagnostics(program);
      expect(diagnostics.length === 0, `${row.id}: ${diagnostics.map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, " ")).join("; ")}`).toBe(row.accept);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

/** 🧪️ Private owner infrastructure routes agree with the independent TypeScript registration parser. */
test("private owner infrastructure exposes truthful registered commands", () => {
  for (const row of fixture.infrastructureCases) {
    const manifest = JSON.parse(readFileSync(join(repo, row.owner, "package.json"), "utf8"));
    const source = ts.createSourceFile("📜️script.ts", readFileSync(join(repo, row.owner, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    const registered = new Set<string>();
    const visit = (node: ts.Node): void => {
      if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) registered.add(node.arguments[0].text);
      ts.forEachChild(node, visit);
    };
    visit(source);
    expect(manifest.name, row.name).toBe(row.name);
    expect(manifest.private, row.name).toBe(true);
    for (const field of ["exports", "main", "module", "types"]) expect(manifest[field], `${row.name}: ${field}`).toBeUndefined();
    expect(existsSync(join(repo, row.owner, "🟦️.ts")), row.name).toBe(false);
    expect(manifest.scripts, row.name).toEqual(Object.fromEntries(row.commands.map((command) => [command, `bun nx run ${row.name}:${command}`])));
    for (const command of row.commands) expect(registered.has(command), `${row.name}: ${command}`).toBe(true);
  }
});
