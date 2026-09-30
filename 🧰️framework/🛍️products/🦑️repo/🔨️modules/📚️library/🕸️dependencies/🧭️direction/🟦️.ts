import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";

export type DependencyDirectionRule = Readonly<{ name: string; severity: string; from: { path: readonly string[] }; to: { path: readonly string[] } }>;
export type DependencyDirectionEdge = Readonly<{ rule: string; from: string; to: string }>;
export type DependencyDirectionGraphScope = Readonly<{ workspaceRoots: readonly string[]; workspacePackages: readonly Readonly<{ name: string; owner: string; exports: readonly string[] }>[]; excludedPaths: readonly string[]; nonFollowedPaths: readonly string[]; expectedSources: readonly string[] }>;

const sourceExtension = /\.(?:[cm]?[jt]s|[jt]sx)$/u;
const pathPatterns = new Map<string, RegExp>();
const pathMatches = (patterns: readonly string[], path: string): boolean => patterns.some((pattern) => {
  let expression = pathPatterns.get(pattern);
  if (!expression) { expression = new RegExp(pattern, "u"); pathPatterns.set(pattern, expression); }
  return expression.test(path);
});
const terminalPath = (scope: DependencyDirectionGraphScope, path: string): boolean => pathMatches([...scope.excludedPaths, ...scope.nonFollowedPaths], path);
const workspacePath = (scope: DependencyDirectionGraphScope, path: string): boolean => scope.workspaceRoots.some((root) => path === root || path.startsWith(`${root}/`));

function validateScope(scope: DependencyDirectionGraphScope): void {
  if (!scope || typeof scope !== "object") throw new Error("Dependency direction requires readable scope metadata");
  for (const key of ["workspaceRoots", "excludedPaths", "nonFollowedPaths", "expectedSources"] as const) {
    const paths = scope[key];
    if (!Array.isArray(paths) || paths.some((path) => typeof path !== "string" || !path)) throw new Error(`Dependency direction requires readable scope metadata: ${key}`);
    if (new Set(paths).size !== paths.length) throw new Error(`Dependency direction scope repeats a path: ${key}`);
  }
  if (!scope.workspaceRoots.length) throw new Error("Dependency direction requires declared workspace roots");
  if (!Array.isArray(scope.workspacePackages) || scope.workspacePackages.some((pkg) => !pkg || typeof pkg.name !== "string" || !pkg.name || typeof pkg.owner !== "string" || !workspacePath(scope, pkg.owner))) throw new Error("Dependency direction requires readable workspace package ownership");
  if (scope.workspacePackages.some((pkg) => !Array.isArray(pkg.exports) || !pkg.exports.length || pkg.exports.some((path: unknown) => typeof path !== "string" || !/^\.(?:\/.*)?$/u.test(path)) || new Set(pkg.exports).size !== pkg.exports.length)) throw new Error("Dependency direction requires readable authored package exports");
  if (new Set(scope.workspacePackages.map((pkg) => pkg.name)).size !== scope.workspacePackages.length) throw new Error("Dependency direction scope repeats a package name");
  for (const pattern of [...scope.excludedPaths, ...scope.nonFollowedPaths]) {
    try { new RegExp(pattern, "u"); } catch { throw new Error(`Dependency direction scope has an invalid path pattern: ${pattern}`); }
  }
}

/** 🗂️ Inventories every followed TypeScript/JavaScript root independently of resolver graph output. */
export function dependencyDirectionSourceInventory(root: string, roots: readonly string[], scope: DependencyDirectionGraphScope): readonly string[] {
  validateScope(scope);
  if (!roots.length || roots.some((path) => !workspacePath(scope, path))) throw new Error("Dependency direction inventory requires declared workspace roots");
  const found = new Set<string>();
  const walk = (path: string): void => {
    if (terminalPath(scope, path)) return;
    for (const entry of readdirSync(join(root, path), { withFileTypes: true })) {
      const child = `${path}/${entry.name}`;
      if (terminalPath(scope, child)) continue;
      if (entry.isSymbolicLink()) {
        if (sourceExtension.test(child) || statSync(join(root, child)).isDirectory()) throw new Error(`Dependency direction cannot inventory a linked source: ${child}`);
        continue;
      }
      if (entry.isDirectory()) walk(child);
      else if (entry.isFile() && sourceExtension.test(child)) found.add(child);
    }
  };
  roots.forEach(walk);
  return [...found].sort();
}

/** 📊️ Validates a complete resolver result and independently checks its framework dependency verdicts. */
export function dependencyDirectionEdges(report: unknown, rules: readonly DependencyDirectionRule[], scope: DependencyDirectionGraphScope): readonly DependencyDirectionEdge[] {
  const record = (value: unknown): Record<string, unknown> => {
    if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Dependency direction graph requires an object");
    return value as Record<string, unknown>;
  };
  const array = (value: unknown): unknown[] => {
    if (!Array.isArray(value)) throw new Error("Dependency direction graph requires an array");
    return value;
  };
  const text = (value: unknown): string => {
    if (typeof value !== "string" || !value) throw new Error("Dependency direction graph requires a nonempty path or rule name");
    return value;
  };
  const matches = pathMatches;
  validateScope(scope);
  if (!scope.expectedSources.length) throw new Error("Dependency direction requires source inventory");
  const graph = record(report), modules = array(graph.modules), summary = record(graph.summary);
  const names = rules.map((rule) => rule.name);
  if (!names.length || new Set(names).size !== names.length || rules.some((rule) => rule.severity !== "error")) throw new Error("Dependency direction requires distinct strict rules");
  const usedRules = array(record(summary.ruleSetUsed).forbidden).map((value) => text(record(value).name));
  if (JSON.stringify([...usedRules].sort()) !== JSON.stringify([...names].sort())) throw new Error("Dependency direction graph used a different policy");
  if (!modules.length || summary.totalCruised !== modules.length) throw new Error("Dependency direction graph is empty or incomplete");
  const edges: DependencyDirectionEdge[] = [], sources = new Map<string, string>();
  let dependencyCount = 0;
  for (const value of modules) {
    const module = record(value), from = text(module.source), dependencies = array(module.dependencies);
    const previous = sources.get(from), signature = JSON.stringify(dependencies);
    if (previous !== undefined && previous !== signature) throw new Error("Dependency direction graph repeats a source with different dependencies");
    sources.set(from, signature);
    dependencyCount += dependencies.length;
    const applicable = rules.filter((rule) => matches(rule.from.path, from));
    for (const value of dependencies) {
      const dependency = record(value), to = text(dependency.resolved);
      const specifier = text(dependency.module), pkg = scope.workspacePackages.find((pkg) => specifier === pkg.name || specifier.startsWith(`${pkg.name}/`));
      if (pkg) {
        const subpath = specifier === pkg.name ? "." : `.${specifier.slice(pkg.name.length)}`;
        if (subpath !== "." && subpath.slice(2).split(/[\\/]/u).some((segment) => !segment || segment === "." || segment === "..")) throw new Error(`Dependency direction graph uses an invalid authored package subpath: ${from} → ${specifier}`);
        const exported = pkg.exports.some((path) => new RegExp(`^${path.split("*").map((part) => part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join(".*")}$`, "u").test(subpath));
        if (!exported) throw new Error(`Dependency direction graph uses an undeclared authored package export: ${from} → ${specifier}`);
      }
      for (const rule of applicable) if (matches(rule.to.path, to) || (pkg && (matches(rule.to.path, specifier) || matches(rule.to.path, `${pkg.owner}/`)))) edges.push({ rule: rule.name, from, to });
    }
  }
  if (summary.totalDependenciesCruised !== dependencyCount) throw new Error("Dependency direction graph omits framework sources or dependencies");
  const missing = scope.expectedSources.filter((source) => !sources.has(source));
  if (missing.length) throw new Error(`Dependency direction graph omits inventoried sources: ${missing.join(", ")}`);
  for (const value of modules) {
    const module = record(value);
    for (const value of array(module.dependencies)) {
      const dependency = record(value), target = text(dependency.resolved);
      if (terminalPath(scope, target)) continue;
      const specifier = text(dependency.module);
      const local = /^(?:\.{1,2}(?:\/|$)|\/|[A-Za-z]:[\\/])/u.test(specifier) || workspacePath(scope, target);
      if (dependency.couldNotResolve === true && local) throw new Error(`Dependency direction graph has an unresolved workspace dependency: ${module.source} → ${specifier}`);
      if (workspacePath(scope, target) && (sourceExtension.test(target) || dependency.followable === true) && !sources.has(target)) throw new Error(`Dependency direction graph omits followed workspace source: ${module.source} → ${target}`);
    }
  }
  const key = (edge: DependencyDirectionEdge): string => JSON.stringify([edge.rule, edge.from, edge.to]);
  const actual = [...new Set(edges.map(key))].sort();
  const reported = [...new Set(array(summary.violations).map((value) => {
    const violation = record(value);
    const rule = text(record(violation.rule).name);
    if (!names.includes(rule)) throw new Error("Dependency direction graph reports a different policy");
    return key({ rule, from: text(violation.from), to: text(violation.to) });
  }))].sort();
  if (JSON.stringify(actual) !== JSON.stringify(reported) || summary.error !== reported.length) throw new Error("Dependency direction resolver verdict disagrees with the source graph");
  return actual.map((value) => { const [rule, from, to] = JSON.parse(value) as [string, string, string]; return { rule, from, to }; });
}
