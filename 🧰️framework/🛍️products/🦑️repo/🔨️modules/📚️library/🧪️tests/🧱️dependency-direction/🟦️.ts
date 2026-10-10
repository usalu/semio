import { expect, test } from "bun:test";
import { cruise } from "dependency-cruiser";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { execFileSync } from "node:child_process";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import glob from "fast-glob";
import ts from "typescript";
import { authoredPackageExportAuthority } from "../../🕸️dependencies/🧭️direction/🏗️construction/🟦️.ts";
import { loadDependencyDirectionPolicy } from "../../🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts";
import { dependencyDirectionEdges, dependencyDirectionSourceInventory, dependencyDirectionWorkspacePackages, type DependencyDirectionGraphScope } from "../../🕸️dependencies/🧭️direction/🟦️.ts";

type Edge = Readonly<{ id: string; target: string; reference: "relative" | "package" | "package-subpath" | "unresolved-package"; syntax: "static" | "type" | "dynamic" | "export" | "export-all" | "require"; package?: string; installation?: string }>;
type Case = Readonly<{ id: string; from: string; comments?: readonly string[]; dependencies: readonly Edge[]; forbidden: readonly string[]; forbiddenRules?: Readonly<Record<string, readonly string[]>> }>;
type Rule = Readonly<{ name: string; severity: string; from: { path: string[]; pathNot?: string[] }; to: { path: string[]; pathNot?: string[] } }>;
type Graph = Readonly<{ modules: readonly { source: string; dependencies: readonly { module: string; resolved: string; couldNotResolve?: boolean }[] }[]; summary: { violations: readonly { from: string; to: string; rule: { name: string } }[] } }>;
const library = resolve(import.meta.dir, "../.."), repo = resolve(library, "../../../../..");
const read = (path: string): unknown => JSON.parse(readFileSync(join(library, path), "utf8"));
type RemovabilityCase = Readonly<{ id: string; directories: readonly string[]; manifests: Readonly<Record<string, string | object>>; expectedPackages: readonly string[]; expectedPlugins: readonly string[]; accept: boolean; members?: readonly string[]; nextManifests?: Readonly<Record<string, object>>; nextExpectedPackages?: readonly string[] }>;
type InfrastructureCase = Readonly<{id:string;owner:string;name:string;commands:readonly string[];manifest:Record<string,unknown>;scriptSource:string;entrySourcePresent:boolean;project:{targets:Record<string,{executor:string;options:{command:string}}>};accept:boolean}>;
type PolicyCase = Readonly<{id:string;members:readonly string[];directories:readonly string[];manifests:Readonly<Record<string,string|object>>;accept:boolean}>;
const fixture = read("🧫️fixtures/🧱️dependency-direction/🔣️.json") as { schemaVersion: number; infrastructureCases: readonly InfrastructureCase[]; policyCases: readonly PolicyCase[]; constructionCases: readonly {id:string;packages:readonly {owner:string;name:string;exports:readonly string[];dependencyRole?:string}[];accept:boolean}[]; typeCases: readonly { id: string; declaration: string; name: string; value: unknown; accept: boolean }[]; publicExportCases: readonly { id: string; owner: string; specifier: string; accept: boolean; conditions: readonly string[]; target?: string; manifest: { name: string; exports: Record<string, unknown> }; files: Readonly<Record<string,string>> }[]; removabilityCases: readonly RemovabilityCase[]; cases: readonly Case[]; graphScope: DependencyDirectionGraphScope; resolutionCases: readonly { id: string; specifier: string; owner: string; from: string; forbidden: number; authored: boolean; accept: boolean }[]; inventoryCases: readonly { id: string; accept: boolean; sourceRole?: string; roots: readonly string[]; files: readonly string[]; links: readonly { path: string; target: string }[]; expectedSources: readonly string[] }[]; graphCases: readonly { id: string; accept: boolean }[] };
const schema = read("🧬️schema/🧱️dependency-direction/🔣️.json");
type PolicyOwner = Readonly<{ owner: string; name: string; role?: string }>;
const policyOwners = (fixture as typeof fixture & { policyOwners: readonly PolicyOwner[] }).policyOwners;
const config = portablePolicy();
const rules = config.forbidden.filter((rule) => ["framework-no-implementation", "io-renderer-independent", "repo-no-implementation", "s-modules-no-plugins", "plugin-no-extension-or-artifact-📐️cad", "framework-no-products"].includes(rule.name)).map((rule) => {
  const patterns = (value: string | string[]): string[] => typeof value === "string" ? [value] : value;
  return { ...rule, from: { path: patterns(rule.from.path), ...(rule.from.pathNot ? { pathNot: patterns(rule.from.pathNot) } : {}) }, to: { path: patterns(rule.to.path), ...(rule.to.pathNot ? { pathNot: patterns(rule.to.pathNot) } : {}) } };
});
const matches = (patterns: readonly string[], candidate: string): boolean => patterns.some((pattern) => new RegExp(pattern, "u").test(candidate));
function write(root: string, path: string, content: string): void { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
function writePolicyAuthority(root: string): void {
  write(root, "nx.json", "{}");
  write(root, "📋️project.json", JSON.stringify({ metadata: { semio: { taxonomy: "taxonomy.json" } } }));
  write(root, "taxonomy.json", JSON.stringify(JSON.parse(readFileSync(join(library,"🔣️taxonomy.json"),"utf8"))));
}

/** 🧪️ Loads the actual policy generator over closed synthetic owner declarations. */
function portablePolicy(): { forbidden: readonly Rule[]; options: { enhancedResolveOptions: { conditionNames: readonly string[] } } } {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-policy-")));
  const boundary = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs";
  try {
    writePolicyAuthority(root);
    const members = policyOwners.map((row) => row.owner);
    write(root, "package.json", JSON.stringify({ workspaces: members, semio: { workspace: { schemaVersion: 1, members, owners: [] } } }));
    for (const row of policyOwners) {
      const manifest = {name:row.name,...(row.role ? {semio:{dependencyRole:row.role}} : {})};
      write(root, `${row.owner}/package.json`, JSON.stringify(manifest));
    }
    return loadDependencyDirectionPolicy(root).policy as unknown as ReturnType<typeof portablePolicy>;
  } finally { rmSync(root, { recursive: true, force: true }); }
}

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
    "export-all": `export * from ${literal};`,
    require: `export const used = require(${literal});`,
  };
  return { specifier, source: sources[edge.syntax] };
}

/** 🧭️ Canonical package authority is unambiguous before selector or alias maps are constructed. */
test("policy construction rejects ambiguous names and divergent owner declarations",async()=>{
 const build=createRequire(import.meta.url)(join(library,"🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs")).buildDependencyDirectionPolicy;
 const taxonomy=JSON.parse(readFileSync(join(library,"🔣️taxonomy.json"),"utf8"));
 for(const row of fixture.constructionCases){
  const construct=()=>build({taxonomy,plugins:[],workspacePackages:row.packages,nodeBuiltins:["fs","path"]});
  if(row.accept)expect(construct,row.id).not.toThrow();
  else expect(construct,row.id).toThrow();
 }
 type OwnershipPatternCase=Readonly<{id:string;root:string;descendants:readonly string[];subjects:readonly Readonly<{path:string;accept:boolean}>[];maximumPatternBytes:number}>;
 const cases=(fixture as typeof fixture & {ownershipPatternCases:readonly OwnershipPatternCase[]}).ownershipPatternCases;
 const validate=new Ajv({strict:true}).compile({...schema as object,$ref:"#/$defs/AuthoredPublicOwnerRule"});
 const oracle=await import(join(repo,"node_modules/dependency-cruiser/src/validate/index.mjs")) as {validateDependency:(rules:unknown,from:unknown,to:unknown)=>{valid:boolean}};
 const escape=(value:string)=>value.replace(/[.*+?^${}()|[\]\\]/g,"\\$&");
 for(const row of cases){
  const packages=[row.root,...row.descendants].map((owner,index)=>({name:"@ownership-pattern/owner-"+index,owner:owner+"/📦️packages/🟦️typescript",sourceOwner:owner,exports:["."],exportTargets:[{subpath:".",target:"./🟦️.ts"}]}));
  const policy=build({taxonomy,plugins:[],workspacePackages:packages,nodeBuiltins:["fs","path"]});
  const rule=policy.forbidden.find((item:{name:string})=>item.name===taxonomy.dependencyDirections.publicApi.rulePrefix+packages[0]!.name);
  expect(validate(rule),JSON.stringify(validate.errors)).toBe(true);
  expect(typeof rule.from.pathNot,row.id).toBe("string");
  const expression=new RegExp(rule.from.pathNot,"u");
  const flat="^"+escape(row.root)+"/(?!(?:node_modules(?:/|$)|.*/node_modules(?:/|$)|"+row.descendants.map(owner=>escape(owner.slice(row.root.length+1))+"/").join("|")+"))";
  const reference={forbidden:[{name:"foreign-owner",severity:"error",from:{path:"^",pathNot:flat},to:{path:"^"}}]};
  for(const subject of row.subjects){
   expect(expression.test(subject.path),row.id+" authored ownership").toBe(subject.accept);
   expect(oracle.validateDependency(reference,{source:subject.path},{resolved:"other/file.ts",couldNotResolve:false,circular:false,dynamic:false,exoticallyRequired:false,dependencyTypes:["local"]}).valid,row.id+" independent flat ownership").toBe(subject.accept);
  }
  expect(Buffer.byteLength(rule.from.pathNot),row.id+" bounded shared-prefix ownership pattern").toBeLessThanOrEqual(row.maximumPatternBytes);
 }
 console.log(`[DEBUG] bounded ownership pattern cases=${cases.length}; independent predicate=dependency-cruiser flat ownership`);
});

test("schema defines unique neutral dependency cases and exact expected edges", () => {
  
  expect(fixture["schemaVersion"]).toEqual(1);
  expect(new Set(fixture.cases.map((row) => row.id)).size).toBe(fixture.cases.length);
  expect(new Set(policyOwners.map((row) => row.name)).size).toBe(policyOwners.length);
  expect(new Set(policyOwners.map((row) => row.owner)).size).toBe(policyOwners.length);
  for (const row of fixture.cases) {
    expect(new Set(row.dependencies.map((edge) => edge.id)).size).toBe(row.dependencies.length);
    expect(row.forbidden.every((id) => row.dependencies.some((edge) => edge.id === id))).toBe(true);
    expect(Object.keys(row.forbiddenRules ?? {}).every((id) => row.forbidden.includes(id))).toBe(true);
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
  const ui=policyOwners.find(row=>row.role==="ui")!;
  expect(ui).toBeDefined();
  expect(rules.some(rule=>rule.from.path.some(pattern=>new RegExp(pattern,"u").test(ui.owner))||rule.to.path.some(pattern=>new RegExp(pattern,"u").test(ui.owner)))).toBe(true);
});

test("strict taxonomy direction covers paths, package aliases, subpaths, tests and scripts", () => {
  expect(rules).toHaveLength(6);
  expect(rules.every((rule) => rule.severity === "error")).toBe(true);
  for (const row of fixture.cases) {
    const forbidden = row.dependencies.filter((edge) => rules.some((rule) => matches(rule.from.path, row.from) && !matches(rule.from.pathNot ?? [], row.from) && matches(rule.to.path, edge.reference === "relative" ? edge.target : `${edge.package}${edge.reference === "package-subpath" ? "/schema" : ""}`))).map((edge) => edge.id);
    expect(forbidden, row.id).toEqual([...row.forbidden]);
    for (const edge of row.dependencies) if (row.forbiddenRules?.[edge.id]) expect(rules.filter((rule) => matches(rule.from.path, row.from) && !matches(rule.from.pathNot ?? [], row.from) && matches(rule.to.path, edge.reference === "relative" ? edge.target : `${edge.package}${edge.reference === "package-subpath" ? "/schema" : ""}`)).map((rule) => rule.name).sort(), row.id).toEqual([...row.forbiddenRules[edge.id]!].sort());
  }
});

test("dependency-cruiser independently parses and resolves every neutral fixture verdict", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const cwd = realpathSync(mkdtempSync(join(output, "dependency-direction-")));
  try {
    const cases = fixture.cases.map((row, index) => {
      const from = row.from.includes("/") ? `${dirname(row.from)}/case-${index}/${row.from.split("/").at(-1)}` : `case-${index}.ts`;
      expect(rules.filter((rule) => matches(rule.from.path, from) && !matches(rule.from.pathNot ?? [], from)).map((rule) => rule.name), row.id).toEqual(rules.filter((rule) => matches(rule.from.path, row.from) && !matches(rule.from.pathNot ?? [], row.from)).map((rule) => rule.name));
      const inputs = row.dependencies.map((edge) => ({ edge, ...render(edge, from) }));
      write(cwd, from, [...(row.comments ?? []).map((comment) => `// ${comment}`), ...inputs.map((input) => input.source), "export {};"].join("\n"));
      write(cwd, `${dirname(from)}/package.json`, JSON.stringify({ type: "module", dependencies: Object.fromEntries(row.dependencies.filter((edge) => edge.package).map((edge) => [edge.package, "*"])) }));
      for (const { edge } of inputs) {
        const target = "export interface Value {}\nexport const value = 1;\n";
        write(cwd, edge.target, target);
        if (edge.reference === "package" || edge.reference === "package-subpath") {
          const packageRoot = edge.installation ?? `${dirname(from)}/node_modules/${edge.package}`;
          write(cwd, `${packageRoot}/package.json`, JSON.stringify({ name: edge.package, type: "module", exports: { ".": "./entry.ts", "./schema": "./schema.ts" } }));
          write(cwd, `${packageRoot}/entry.ts`, target);
          write(cwd, `${packageRoot}/schema.ts`, target);
          if (edge.installation) {
            const link = join(cwd, `${dirname(from)}/node_modules/${edge.package}`);
            mkdirSync(dirname(link), { recursive: true });
            symlinkSync(join(cwd, packageRoot), link, process.platform === "win32" ? "junction" : "dir");
          }
        }
      }
      return { row, from, inputs };
    });
    const unresolved = (row: Case): boolean => row.dependencies.some((edge) => edge.reference === "unresolved-package" && edge.package?.startsWith("@semio-tech/"));
    for (const refused of [false, true]) {
      const group = cases.filter(({ row }) => unresolved(row) === refused);
      if (!group.length) continue;
      const oracle = await cruise(group.map(({ from }) => from), { baseDir: cwd, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, doNotFollow: { path: "node_modules" } }, { exportsFields: ["exports"], conditionNames: ["import", "require", "node", "default"], modules: ["node_modules"], restrictions: [cwd], bustTheCache: true });
      expect(oracle.exitCode).toBe(0);
      const graph = (typeof oracle.output === "string" ? JSON.parse(oracle.output) : oracle.output) as Graph;
      const scope = { ...fixture.graphScope, workspaceRoots: ["🧰️framework", "🌎️hub", "✏️s", "♻️mit-bestand", "🏢️semio-tech"], expectedSources: group.map(({ from }) => from) };
      if (refused) expect(() => dependencyDirectionEdges(graph, rules, scope)).toThrow();
      else expect(dependencyDirectionEdges(graph, rules, scope)).toHaveLength(group.reduce((count, { row }) => count + row.forbidden.reduce((count, id) => count + (row.forbiddenRules?.[id]?.length ?? 1), 0), 0));
      for (const { row, from, inputs } of group) {
      const module = graph.modules.find((module) => module.source === from);
      expect(module, row.id).toBeDefined();
      expect(module!.dependencies, row.id).toHaveLength(inputs.length);
      const actual = inputs.filter(({ specifier, edge }) => {
        const dependency = module!.dependencies.find((dependency) => dependency.module === specifier);
        expect(dependency, row.id).toBeDefined();
        if (edge.installation) expect(dependency!.resolved, row.id).toBe(`${edge.installation}/${edge.reference === "package-subpath" ? "schema" : "entry"}.ts`);
        expect(dependency!.couldNotResolve === true, row.id).toBe(edge.reference === "unresolved-package");
        const verdicts = graph.summary.violations.filter((violation) => violation.from === from && violation.to === dependency!.resolved && rules.some((rule) => violation.rule.name === rule.name));
        if (row.forbiddenRules?.[edge.id]) expect(verdicts.map((violation) => violation.rule.name).sort(), row.id).toEqual([...row.forbiddenRules[edge.id]!].sort());
        return verdicts.length > 0;
      }).map(({ edge }) => edge.id);
      expect(actual, row.id).toEqual([...row.forbidden]);
    }
    }
  } finally { rmSync(cwd, { recursive: true, force: true }); }
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
    const entries = fixture.resolutionCases.map((row, index) => { const entry = `${dirname(row.from)}/resolution-${index}/${row.from.split("/").at(-1)}`; write(root, entry, `import ${JSON.stringify(row.specifier)}; export {};`); return { row, entry }; });
    const result = await cruise(entries.map(({ entry }) => entry), { baseDir: root, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", combinedDependencies: true }, { modules: [join(root, "node_modules")], restrictions: [root], bustTheCache: true });
    const report = typeof result.output === "string" ? JSON.parse(result.output) : result.output;
    expect(result.exitCode).toBe(0);
    expect(report.modules.map((module: { source: string }) => module.source).sort()).toEqual([...entries.map(({ entry }) => entry), ...new Set(entries.map(({ row }) => row.specifier))].sort());
    expect(report.summary.totalCruised).toBe(report.modules.length);
    expect(report.summary.totalDependenciesCruised).toBe(entries.length);
    for (const { row, entry } of entries) {
      const module = report.modules.find((module: { source: string }) => module.source === entry);
      expect(module.dependencies, row.id).toHaveLength(1);
      expect(module.dependencies[0].couldNotResolve, row.id).toBe(true);
      const violations = report.summary.violations.filter((violation: { from: string }) => violation.from === entry);
      const terminal = report.modules.find((leaf: { source: string }) => leaf.source === module.dependencies[0].resolved);
      expect(terminal.dependencies, row.id).toEqual([]);
      const graph = { ...report, modules: [module, terminal], summary: { ...report.summary, totalCruised: 2, totalDependenciesCruised: 1, violations, error: violations.length } };
      const scope = { ...fixture.graphScope, workspacePackages: row.authored ? [{ name: row.specifier.startsWith("@") ? row.specifier.split("/").slice(0, 2).join("/") : row.specifier.split("/")[0]!, owner: row.owner, ...authoredPackageExportAuthority(row.owner,{exports:{".":"./index.ts","./schema":"./schema.ts"}}) }] : [], expectedSources: [entry] };
      expect(violations.length, row.id).toBe(row.forbidden);
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
      const taxonomy = JSON.parse(readFileSync(join(library,"🔣️taxonomy.json"),"utf8"));
      const roots = row.sourceRole ? taxonomy.dependencyDirections.roles[row.sourceRole].ownerPaths : row.roots;
      expect(roots,row.id).toEqual([...row.roots]);
      const cwd = join(root, String(index));
      for (const path of row.files) write(cwd, path, path.endsWith(".json") ? "{}" : "export {};\n");
      for (const link of row.links) {
        mkdirSync(dirname(join(cwd, link.path)), { recursive: true });
        symlinkSync(join(cwd, link.target), join(cwd, link.path), process.platform === "win32" ? "junction" : "dir");
      }
      if (!row.accept) {
        expect(() => dependencyDirectionSourceInventory(cwd, roots, { ...fixture.graphScope, expectedSources: row.expectedSources }), row.id).toThrow();
        await expect(cruise([...roots], { baseDir: cwd, outputType: "json", exclude: { path: [...fixture.graphScope.excludedPaths] } }, { bustTheCache: true })).rejects.toThrow();
        continue;
      }
      const entry = row.expectedSources[0]!, peer = row.expectedSources[1]!;
      const target = relative(dirname(entry), peer).replaceAll("\\", "/");
      write(cwd, entry, `import "./${target}"; import "node:fs"; import "./schema.json"; export {};`);
      const scope = { ...fixture.graphScope, expectedSources: row.expectedSources };
      expect(dependencyDirectionSourceInventory(cwd, roots, scope), row.id).toEqual([...row.expectedSources]);
      const result = await cruise([...roots], { baseDir: cwd, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, exclude: { path: [...scope.excludedPaths] }, doNotFollow: { path: scope.nonFollowedPaths.join("|") } }, { bustTheCache: true });
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
    const oracleRows: { entry: string; policy: { forbidden: readonly Rule[] } }[] = [];
    for (const [index, row] of fixture.removabilityCases.entries()) {
      const cwd = join(root, String(index));
      writePolicyAuthority(cwd);
      const members = row.members ?? [owner, "✏️s/🔌️plugins/🧪️example/🗿️artifacts/🧪️example/📦️packages/🟦️typescript"];
      write(cwd, "package.json", JSON.stringify({ workspaces: members, semio: { workspace: { schemaVersion: 1, members, owners: [] } } }));
      for (const directory of row.directories) mkdirSync(join(cwd, directory), { recursive: true });
      for (const [directory, manifest] of Object.entries(row.manifests)) write(cwd, `${directory}/package.json`, typeof manifest === "string" ? manifest : JSON.stringify(manifest));
      const load = (): { forbidden: readonly Rule[] } => loadDependencyDirectionPolicy(cwd).policy as unknown as {forbidden:readonly Rule[]};
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
      oracleRows.push({ entry: `${index}/🧰️framework/🟦️.ts`, policy });
      if (row.nextManifests) {
        for (const [directory, manifest] of Object.entries(row.nextManifests)) write(cwd, `${directory}/package.json`, JSON.stringify(manifest));
        const expected = glob.sync(members.map((pattern) => `${pattern}/package.json`), { cwd, onlyFiles: true, followSymbolicLinks: false }).map((path) => JSON.parse(readFileSync(join(cwd, path), "utf8")).name).sort();
        expect(expected, row.id).toEqual([...row.nextExpectedPackages!].sort());
        expect(dependencyDirectionWorkspacePackages(cwd, []).map((pkg) => pkg.name).sort(), row.id).toEqual(expected);
      }
    }
    const oracle = await cruise(oracleRows.map(({ entry }) => entry), { baseDir: root, validate: true, ruleSet: { forbidden: oracleRows.flatMap(({ policy }, index) => policy.forbidden.map((rule) => ({ ...rule, name: `case-${index}-${rule.name}`, severity: rule.severity as "error" | "warn" }))) }, outputType: "json" }, { modules: [join(root, "node_modules")], restrictions: [root], bustTheCache: true });
    const graph = typeof oracle.output === "string" ? JSON.parse(oracle.output) : oracle.output;
    expect(oracle.exitCode).toBe(0);
    expect(graph.modules.map((module: { source: string }) => module.source).sort()).toEqual(oracleRows.map(({ entry }) => entry).sort());
    expect(graph.summary.totalDependenciesCruised).toBe(0);
    expect(graph.summary.violations).toEqual([]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

/** 🏷️ Authored public subpaths resolve independently while private and pseudo-package names fail. */
test("authored owner exports agree with independent package resolution", async () => {
  const policy = config;
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "dependency-public-exports-")));
  try {
    const entries = fixture.publicExportCases.map((row, index) => {
      const from = `♻️mit-bestand/public-${index}/consumer.ts`;
      const manifest = row.manifest;
      const owner = `${dirname(from)}/node_modules/${manifest.name}`;
      write(root, `${owner}/package.json`, JSON.stringify(manifest));
      for (const [target,content] of Object.entries(row.files)) write(root, owner+"/"+target,content);
      write(root, from, `import { value } from ${JSON.stringify(row.specifier)}; export const used = value;`);
      return { row, from, manifest, owner };
    });
    for (const conditions of new Map(entries.map(({ row }) => [JSON.stringify(row.conditions), row.conditions])).values()) {
      const group = entries.filter(({ row }) => JSON.stringify(row.conditions) === JSON.stringify(conditions));
      const result = await cruise(group.map(({ from }) => from), { baseDir: root, validate: true, ruleSet: { forbidden: rules.map((rule) => ({ ...rule, severity: "error" as const })) }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, doNotFollow: { path: "node_modules" } }, { exportsFields: ["exports"], conditionNames: [...conditions], modules: ["node_modules"], restrictions: [root], bustTheCache: true });
      const report = typeof result.output === "string" ? JSON.parse(result.output) : result.output;
      expect(result.exitCode).toBe(0);
      expect(report.summary.totalDependenciesCruised).toBe(group.length);
      expect(report.summary.totalCruised).toBe(report.modules.length);
      for (const { row, from, manifest, owner } of group) {
        const module = report.modules.find((module: { source: string }) => module.source === from);
        expect(module.dependencies, row.id).toHaveLength(1);
        const dependency = module.dependencies[0];
        expect(dependency.couldNotResolve !== true, row.id).toBe(row.accept);
        if (row.target) {
          if(row.conditions.includes("semio-source"))expect(policy.options.enhancedResolveOptions.conditionNames,row.id).toEqual([...row.conditions]);
          expect(dependency.resolved, row.id).toBe(`${owner}/${row.target.slice(2)}`);
        }
        const terminal = report.modules.find((leaf: { source: string }) => leaf.source === dependency.resolved);
        expect(terminal.dependencies, row.id).toEqual([]);
        const violations = report.summary.violations.filter((violation: { from: string }) => violation.from === from);
        const graph = { ...report, modules: [module, terminal], summary: { ...report.summary, totalCruised: 2, totalDependenciesCruised: 1, violations, error: violations.length } };
        const scope = { ...fixture.graphScope, workspaceRoots: [...fixture.graphScope.workspaceRoots, "♻️mit-bestand", "✏️s"], workspacePackages: [{ name: manifest.name, owner: row.owner, ...authoredPackageExportAuthority(row.owner,manifest) }], expectedSources: [from] };
        if (row.accept) expect(dependencyDirectionEdges(graph, rules, scope), row.id).toEqual([]);
        else expect(() => dependencyDirectionEdges(graph, rules, scope), row.id).toThrow();
      }
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
      const declaration=join(root,"declaration-"+String(index)+".d.ts");
      writeFileSync(declaration,row.declaration);
      writeFileSync(entry,"import type { "+row.name+" } from "+JSON.stringify(declaration)+"; const value: "+row.name+" = "+JSON.stringify(row.value)+"; void value;");
      const program = ts.createProgram([entry], { noEmit: true, strict: true, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ES2022, types: [] });
      const diagnostics = ts.getPreEmitDiagnostics(program);
      expect(diagnostics.length === 0, `${row.id}: ${diagnostics.map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, " ")).join("; ")}`).toBe(row.accept);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);


/** 🧪️ Command-only manifests retain exact private ownership and independently parsed registrations. */
function assertInfrastructure(row:InfrastructureCase):void {
 const manifest=row.manifest;
 expect(manifest.name,row.id).toBe(row.name);expect(manifest.private,row.id).toBe(true);
 for(const field of ["exports","main","module","types"]) expect(manifest[field],row.id+":"+field).toBeUndefined();
 expect(row.entrySourcePresent,row.id).toBe(false);
 expect(manifest.scripts,row.id).toEqual(Object.fromEntries(row.commands.map(command=>[command,"bun nx run "+row.name+":"+command])));
 const source=ts.createSourceFile("📜️script.ts",row.scriptSource,ts.ScriptTarget.Latest,true),registered=new Set<string>();
 const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isPropertyAccessExpression(node.expression)&&node.expression.name.text==="register"&&node.arguments[0]&&ts.isStringLiteral(node.arguments[0]))registered.add(node.arguments[0].text);ts.forEachChild(node,visit);};visit(source);
 expect([...registered].sort(),row.id).toEqual([...row.commands].sort());expect(Object.keys(row.project.targets).sort(),row.id).toEqual([...row.commands].sort());
 for(const command of row.commands){expect(row.project.targets[command]!.executor,row.id).toBe("nx:run-commands");expect(row.project.targets[command]!.options.command,row.id).toBe("bun ./📜️script.ts "+command);}
}
test("private owner infrastructure exposes truthful registered commands",()=>{
 for(const row of fixture.infrastructureCases) {
  if(row.accept) expect(()=>assertInfrastructure(row),row.id).not.toThrow();
  else expect(()=>assertInfrastructure(row),row.id).toThrow();
 }
});
/** 🗑️ Specific tree deletion preserves generic policy loading and all package mechanisms. */
test("synthetic owner policy accepts deletion and rejects malformed present declarations",()=>{
 const output=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir());mkdirSync(output,{recursive:true});
 const root=realpathSync(mkdtempSync(join(output,"dependency-specific-deletion-"))),boundary="🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs";
 try {
  for(const [index,row] of fixture.policyCases.entries()){
   const cwd=join(root,String(index));writePolicyAuthority(cwd);
   write(cwd,"package.json",JSON.stringify({workspaces:row.members,semio:{workspace:{schemaVersion:1,members:row.members,owners:[]}}}));
   for(const directory of row.directories)mkdirSync(join(cwd,directory),{recursive:true});
   for(const [owner,manifest] of Object.entries(row.manifests))write(cwd,owner+"/package.json",typeof manifest==="string"?manifest:JSON.stringify(manifest));
   const load=()=>loadDependencyDirectionPolicy(cwd).policy;
   if(row.accept){expect(load,row.id).not.toThrow();expect(dependencyDirectionWorkspacePackages(cwd,[]).length,row.id).toBe(Object.keys(row.manifests).length);}
   else {expect(load,row.id).toThrow(); if(row.id==="present-duplicate-name")expect(()=>dependencyDirectionWorkspacePackages(cwd,[]),row.id).toThrow();}
   if(row.id==="deleted-specific-no-owners"){expect(existsSync(join(cwd,"✏️s"))).toBe(false);for(const owner of fixture.infrastructureCases.filter(x=>x.accept))assertInfrastructure(owner);expect(fixture.publicExportCases).toHaveLength(7);expect(fixture.typeCases).toHaveLength(3);}
  }
 } finally {rmSync(root,{recursive:true,force:true});}
});

/** 🛍️ Authored export targets govern every cross-owner source form, including executable owners. */
test("cross-owner public targets agree with independent resolver and complete graph verdicts", async () => {
  type PublicCase = Readonly<{ id: string; from: string; target: string; syntax: Edge["syntax"]; accept: boolean; oracleSpecifier: string; packages: readonly {owner:string;sourceOwner:string;name:string;exports:unknown}[] }>;
  const rows = (fixture as typeof fixture & {publicOwnerCases:readonly PublicCase[]}).publicOwnerCases;
  const validate = new Ajv({strict:true}).compile({...schema as object,$ref:"#/$defs/AuthoredPublicPackage"});
  expect(new Set(rows.map(row=>row.id)).size).toBe(rows.length);
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output,{recursive:true});
  const root = realpathSync(mkdtempSync(join(output,"public-owner-")));
  try {
    for (const [index,row] of rows.entries()) {
      const cwd=join(root,String(index)),members=row.packages.map(pkg=>pkg.owner);
      write(cwd,"nx.json","{}");
      write(cwd,"📋️project.json",JSON.stringify({metadata:{semio:{taxonomy:"taxonomy.json"}}}));
      write(cwd,"taxonomy.json",readFileSync(join(library,"🔣️taxonomy.json"),"utf8"));
      write(cwd,"package.json",JSON.stringify({workspaces:members,semio:{workspace:{schemaVersion:1,members,owners:[]}}}));
      for(const pkg of row.packages) {
        write(cwd,pkg.owner+"/package.json",JSON.stringify({name:pkg.name,type:"module",exports:pkg.exports}));
        const targets:string[]=[];
        const collect=(value:unknown):void=>{if(typeof value==="string")targets.push(value.replaceAll("*","widget"));else if(value&&typeof value==="object")Object.values(value).forEach(collect);};
        collect(pkg.exports);
        for(const target of targets)write(cwd,pkg.owner+"/"+target.slice(2),target.endsWith(".json")?"{}":"export interface Value {}\nexport const value=1;\n");
        const installed=join(cwd,"node_modules",pkg.name);mkdirSync(dirname(installed),{recursive:true});symlinkSync(join(cwd,pkg.owner),installed,process.platform==="win32"?"junction":"dir");
      }
      write(cwd,row.target,row.target.endsWith(".json")?"{}":"export interface Value {}\nexport const value=1;\n");
      const edge:Edge={id:row.id,target:row.target,reference:"relative",syntax:row.syntax};
      write(cwd,row.from,render(edge,row.from).source+"\nexport {};");
      const snapshot=loadDependencyDirectionPolicy(cwd);
      for(const pkg of snapshot.workspacePackages)expect(validate(pkg),JSON.stringify(validate.errors)).toBe(true);
      const publicRules=snapshot.policy.forbidden.filter(rule=>rule.name.startsWith("no-cross-package-relative-")||rule.name==="framework-no-implementation");
      const normalized=publicRules.map(rule=>{const list=(value:string|readonly string[]|undefined):readonly string[]=>typeof value==="string"?[value]:value??[];return {...rule,from:{path:list(rule.from.path),pathNot:list(rule.from.pathNot)},to:{path:list(rule.to.path),pathNot:list(rule.to.pathNot)}};});
      const result=await cruise([row.from],{baseDir:cwd,validate:true,ruleSet:{forbidden:normalized.map(rule=>({...rule,severity:"error" as const,from:{path:[...rule.from.path],pathNot:[...rule.from.pathNot]},to:{path:[...rule.to.path],pathNot:[...rule.to.pathNot]}}))},outputType:"json",tsPreCompilationDeps:true,combinedDependencies:true},{exportsFields:[...snapshot.policy.options.enhancedResolveOptions.exportsFields],conditionNames:["semio-source","import","require","node","default"],bustTheCache:true});
      const graph=typeof result.output==="string"?JSON.parse(result.output):result.output;
      const scope={workspaceRoots:["🧰️framework"],workspacePackages:snapshot.workspacePackages,excludedPaths:[],nonFollowedPaths:["node_modules"],expectedSources:graph.modules.map((module:{source:string})=>module.source)};
      const edges=dependencyDirectionEdges(graph,normalized,scope);
      expect(edges.length===0,row.id+" independent graph verdict").toBe(row.accept);
      expect(graph.summary.error===0,row.id+" dependency-cruiser verdict").toBe(row.accept);
      expect(snapshot.workspacePackages.map(pkg=>(pkg as typeof pkg & {sourceOwner?:string}).sourceOwner),row.id+" semantic source owners").toEqual(row.packages.map(pkg=>pkg.sourceOwner));
      expect(publicRules.filter(rule=>rule.name.startsWith("no-cross-package-relative-")).length,row.id+" admitted strict owner rules").toBe(row.packages.length);
      expect(publicRules.every(rule=>rule.severity==="error"),row.id).toBe(true);
    }
    console.log(`[DEBUG] public owner cases=${rows.length}; independent resolver=dependency-cruiser; schema=Ajv2020`);
  } finally {rmSync(root,{recursive:true,force:true});}
},90_000);

/** 🔌️ Current catalogue ownership and neutral public SDK contracts remain strict. */
test("current public owner policy replaces obsolete generated registry storage and admits strict SDK",()=>{
  expect(config.forbidden.some(rule=>rule.name==="no-generated-edits-upstream")).toBe(false);
});

/** 🧭️ Plugin consumers use the strict neutral SDK without executable source exemptions. */
test("plugin public SDK policy is strict for runtime and executable owners",()=>{
  const rule=config.forbidden.find(rule=>rule.name==="plugins-framework-sdk-only");
  expect(rule?.severity).toBe("error");
  expect(rule?.from.pathNot).toBeUndefined();
});


/** 📮️ Proves actual package-local semantic facets with an independent Node export resolver. */
test("actual public package targets obey Node ownership and expose neutral Process facets", () => {
  type PublicPackageCase=Readonly<{id:string;owner:string;name:string;subpath:string;target:string;accept:boolean}>;
  const rows=(fixture as typeof fixture & {actualPublicPackageCases:readonly PublicPackageCase[]}).actualPublicPackageCases;
  const validate=new Ajv({strict:true}).compile({...schema as object,$ref:"#/$defs/AuthoredPublicPackage"});
  expect(new Set(rows.map(row=>row.id)).size).toBe(rows.length);
  const oracle=String.raw`const p=require("node:path"),m=require("node:module");const root=process.argv[1],rows=JSON.parse(process.argv[2]);process.stdout.write(JSON.stringify(rows.map(row=>{try{const target=m.createRequire(p.join(root,row.owner,"package.json")).resolve(row.name+(row.subpath==="."?"":row.subpath.slice(1)));return {id:row.id,accept:true,target};}catch(error){return {id:row.id,accept:false,error:error.code};}})));`;
  const verdicts=JSON.parse(execFileSync("node",["--input-type=commonjs","-e",oracle,repo,JSON.stringify(rows)],{encoding:"utf8",timeout:1000,maxBuffer:16777216})) as readonly Readonly<{id:string;accept:boolean;target?:string;error?:string}>[];
  expect(verdicts.length).toBe(rows.length);
  for(const [index,row] of rows.entries()){
    const verdict=verdicts[index]!;expect(verdict.id).toBe(row.id);expect(verdict.accept,row.id+" Node public package oracle "+verdict.error).toBe(row.accept);
    if(!row.accept)continue;
    expect(verdict.target).toBe(join(repo,row.owner,row.target));
    const manifest=JSON.parse(readFileSync(join(repo,row.owner,"package.json"),"utf8"));expect(manifest.name).toBe(row.name);
    const authority=authoredPackageExportAuthority(row.owner,manifest);expect(validate({owner:row.owner,name:manifest.name,...authority}),JSON.stringify(validate.errors)).toBe(true);expect(authority.exportTargets.some(target=>target.subpath===row.subpath&&target.target===row.target)).toBe(true);
    expect(authority.exportTargets.every(target=>target.target===null||target.target.startsWith("./"))).toBe(true);
  }
  console.log(`[DEBUG] actual public package cases=${rows.length}; independent oracle=Node exports; local semantic targets exact`);
});
