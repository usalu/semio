import { lstatSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { authoredPackageExportAuthority } from "./🏗️construction/🟦️.ts";
import { bunRepositoryMembership } from "../../🗂️workspaces/🟦️bun/🟦️.ts";

export type DependencyDirectionRule = Readonly<{ name: string; severity: string; from: { path: readonly string[]; pathNot?: readonly string[] }; to: { path: readonly string[]; pathNot?: readonly string[] } }>;
export type DependencyDirectionEdge = Readonly<{ rule: string; from: string; to: string }>;
export type DependencyDirectionGraphScope = Readonly<{ workspaceRoots: readonly string[]; workspacePackages: readonly Readonly<{ name: string; owner: string; sourceOwner:string; exports: readonly string[]; exportTargets:readonly Readonly<{subpath:string;target:string|null}>[] }>[]; excludedPaths: readonly string[]; nonFollowedPaths: readonly string[]; expectedSources: readonly string[] }>;

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
  if (scope.workspacePackages.some((pkg) => !Array.isArray(pkg.exports) || pkg.exports.some((path: unknown) => typeof path !== "string" || !/^\.(?:\/.*)?$/u.test(path)) || new Set(pkg.exports).size !== pkg.exports.length)) throw new Error("Dependency direction requires readable authored package exports");
  for (const pkg of scope.workspacePackages) {
    if (pkg.sourceOwner !== (pkg.owner.includes("/📦️packages/") ? pkg.owner.split("/📦️packages/")[0] : pkg.owner) || !workspacePath(scope,pkg.sourceOwner)) throw Error("Dependency direction requires canonical semantic package ownership");
    if (!Array.isArray(pkg.exportTargets) || pkg.exportTargets.some((row: Readonly<{subpath:string;target:string|null}>)=>!row || typeof row.subpath!=="string" || !/^\.(?:\/.*)?$/u.test(row.subpath) || (row.target!==null && (typeof row.target!=="string" || !row.target.startsWith("./"))))) throw Error("Dependency direction requires readable authored export targets");
    const targets:Record<string,(string|null)[]> = {};
    for (const row of pkg.exportTargets) { if (Object.keys(row).sort().join("|") !== "subpath|target") throw Error("Dependency direction requires exact target metadata"); (targets[row.subpath] ??= []).push(row.target); }
    const canonical = authoredPackageExportAuthority(pkg.owner,{exports:targets});
    if (new Set(pkg.exportTargets.map((row:Readonly<{subpath:string;target:string|null}>)=>JSON.stringify(row))).size !== pkg.exportTargets.length || pkg.exports.length !== canonical.exports.length || pkg.exports.some((key:string)=>!canonical.exports.includes(key))) throw Error("Dependency direction has contradictory authored public exports");
  }
  if (new Set(scope.workspacePackages.map((pkg) => pkg.name)).size !== scope.workspacePackages.length) throw new Error("Dependency direction scope repeats a package name");
  for (const pattern of [...scope.excludedPaths, ...scope.nonFollowedPaths]) {
    try { new RegExp(pattern, "u"); } catch { throw new Error(`Dependency direction scope has an invalid path pattern: ${pattern}`); }
  }
}

/** 📦️ Reads present authored workspace owners; deleted owners contribute no packages. */
export function dependencyDirectionWorkspacePackages(root: string, excludedPaths: readonly string[]): DependencyDirectionGraphScope["workspacePackages"] {
  const membership = bunRepositoryMembership(root);
  for (const scope of membership.scopes) for (const path of scope.declaration.members) {
    if (path.startsWith("!") || /[*?]/u.test(path)) continue;
    const owner = join(root, scope.directory, path);
    let entry;
    try { entry = lstatSync(owner); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") continue; throw error; }
    if (entry.isDirectory()) lstatSync(join(owner, "package.json"));
  }
  const workspaces = membership.packages, names = new Map<string,string>();
  return workspaces.filter((path) => !pathMatches(excludedPaths, path)).flatMap((owner) => {
    let directory;
    try { directory = lstatSync(join(root, owner)); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return []; throw error; }
    if (!directory.isDirectory()) throw new Error(`Dependency direction workspace owner must be a directory: ${owner}`);
    const manifest = JSON.parse(readFileSync(join(root, owner, "package.json"), "utf8"));
    if (typeof manifest.name !== "string" || !manifest.name) throw new Error(`Dependency direction requires an authored package name: ${owner}`);
    const previous = names.get(manifest.name);
    if (previous !== undefined && previous !== owner) throw new Error("Dependency direction package name has distinct owners: "+manifest.name+" ("+previous+", "+owner+")");
    names.set(manifest.name,owner);
    return [{ name: manifest.name, owner, ...authoredPackageExportAuthority(owner, manifest) }];
  });
}

/** 🗂️ Inventories every followed TypeScript/JavaScript root independently of resolver graph output. */
export function dependencyDirectionSourceInventory(root: string, roots: readonly string[], scope: DependencyDirectionGraphScope): readonly string[] {
  validateScope(scope);
  if (!roots.length || roots.some((path) => !workspacePath(scope, path) && (path.includes("/") || !sourceExtension.test(path)))) throw new Error("Dependency direction inventory requires declared workspace roots");
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
  roots.forEach((path) => {
    const entry = lstatSync(join(root, path));
    if (entry.isSymbolicLink()) throw new Error(`Dependency direction cannot inventory a linked root: ${path}`);
    if (entry.isDirectory()) walk(path);
    else if (entry.isFile() && sourceExtension.test(path) && !terminalPath(scope, path)) found.add(path);
    else throw new Error(`Dependency direction inventory requires a source root: ${path}`);
  });
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
  const edges: DependencyDirectionEdge[] = [], problems: string[] = [], sources = new Map<string, string>();
  let dependencyCount = 0;
  for (const value of modules) {
    const module = record(value), from = text(module.source), dependencies = array(module.dependencies);
    const previous = sources.get(from), signature = JSON.stringify(dependencies);
    if (previous !== undefined && previous !== signature) throw new Error(`Dependency direction graph repeats a source with different dependencies: ${from}`);
    sources.set(from, signature);
    dependencyCount += dependencies.length;
    const applicable = rules.filter((rule) => matches(rule.from.path, from) && !matches(rule.from.pathNot ?? [], from));
    for (const value of dependencies) {
      const dependency = record(value), to = text(dependency.resolved);
      const specifier = text(dependency.module), pkg = scope.workspacePackages.find((pkg) => specifier === pkg.name || specifier.startsWith(`${pkg.name}/`));
      if (pkg) {
        const subpath = specifier === pkg.name ? "." : `.${specifier.slice(pkg.name.length)}`;
        if (subpath !== "." && subpath.slice(2).split(/[\\/]/u).some((segment) => !segment || segment === "." || segment === "..")) throw new Error(`Dependency direction graph uses an invalid authored package subpath: ${from} → ${specifier}`);
        const exported = pkg.exports.some((path) => new RegExp(`^${path.split("*").map((part) => part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join(".*")}$`, "u").test(subpath));
        if (!exported) problems.push(`Dependency direction graph uses an undeclared authored package export: ${from} → ${specifier}`);
        const candidates=pkg.exportTargets.flatMap(row=>{
          const expression=new RegExp(`^${row.subpath.split("*").map(part=>part.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("(.*)")}$`,"u"),match=expression.exec(subpath);
          return match?[{...row,capture:match[1]??"",rank:row.subpath.includes("*")?row.subpath.indexOf("*"):Number.MAX_SAFE_INTEGER}]:[];
        }).sort((a,b)=>b.rank-a.rank || b.subpath.length-a.subpath.length);
        const chosen=candidates.filter(row=>row.subpath===candidates[0]?.subpath);
        const installedTarget=to.includes("/node_modules/")?to.slice(to.lastIndexOf("/node_modules/")+1):to;
        if (!chosen.some(row=>{if(row.target===null)return false;const target=row.target.slice(2).replaceAll("*",row.capture);return to===`${pkg.owner}/${target}` || installedTarget===`node_modules/${pkg.name}/${target}`;})) problems.push(`Dependency direction graph resolves an authored package export to a different target: ${from} → ${specifier} → ${to}`);

      }
      const installed = to.split("/").lastIndexOf("node_modules"), ownership = installed < 0 ? to : to.split("/").slice(installed).join("/");
      for (const rule of applicable) if (!matches(rule.to.pathNot ?? [], to) && (matches(rule.to.path, ownership) || (pkg && (matches(rule.to.path, specifier) || matches(rule.to.path, `${pkg.owner}/`))))) edges.push({ rule: rule.name, from, to });
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
      const authored = scope.workspacePackages.some((pkg) => specifier === pkg.name || specifier.startsWith(`${pkg.name}/`)) || specifier.startsWith("@semio-tech/");
      const local = /^(?:\.{1,2}(?:\/|$)|\/|[A-Za-z]:[\\/])/u.test(specifier) || workspacePath(scope, target);
      if (dependency.couldNotResolve === true && (local || authored)) problems.push(`Dependency direction graph has an unresolved workspace dependency: ${module.source} → ${specifier}`);
      if (workspacePath(scope, target) && (sourceExtension.test(target) || dependency.followable === true) && !sources.has(target)) problems.push(`Dependency direction graph omits followed workspace source: ${module.source} → ${target}`);
    }
  }
  if (problems.length) throw new Error([...new Set(problems)].join("\n"));
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
