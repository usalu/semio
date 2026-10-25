import { lstatSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { authoredPackageExportAuthority } from "./🏗️construction/🟦️.ts";
import { bunRepositoryMembership } from "../../🗂️workspaces/🟦️bun/🟦️.ts";
import type { DependencyPolicySnapshot } from "./🚀️bootstrap/🟦️.ts";
import { refuseDirectionV1 } from "./📖️texts/🟦️.ts";

/** 🧭️ One enforced path rule: an absent selector path is normalized to the empty pattern that matches every path; `dependencyTypes`, `dependencyTypesNot` and `circular` constrain the dependency itself, with the semantics of the resolver's own rule matching. */
export type DependencyDirectionRule = Readonly<{ name: string; severity: string; from: { path: readonly string[]; pathNot?: readonly string[] }; to: { path: readonly string[]; pathNot?: readonly string[]; dependencyTypes?: readonly string[]; dependencyTypesNot?: readonly string[]; circular?: boolean } }>;
export type DependencyDirectionEdge = Readonly<{ rule: string; from: string; to: string }>;
export type DependencyDirectionGraphScope = Readonly<{ workspaceRoots: readonly string[]; workspacePackages: readonly Readonly<{ name: string; owner: string; sourceOwner:string; exports: readonly string[]; exportTargets:readonly Readonly<{subpath:string;target:string|null}>[] }>[]; excludedPaths: readonly string[]; nonFollowedPaths: readonly string[]; expectedSources: readonly string[] }>;

const sourceExtension = /\.(?:[cm]?[jt]s|[jt]sx)$/u;
const pathPatterns = new Map<string, RegExp>();
/** 🔎️ Reuses immutable compiled selector patterns across source and runtime path checks. */
export const dependencyDirectionPathMatches = (patterns: readonly string[], path: string): boolean => patterns.some((pattern) => {
  let expression = pathPatterns.get(pattern);
  if (!expression) { expression = new RegExp(pattern, "u"); pathPatterns.set(pattern, expression); }
  return expression.test(path);
});
const terminalPath = (scope: DependencyDirectionGraphScope, path: string): boolean => dependencyDirectionPathMatches([...scope.excludedPaths, ...scope.nonFollowedPaths], path);
const workspacePath = (scope: DependencyDirectionGraphScope, path: string): boolean => scope.workspaceRoots.some((root) => path === root || path.startsWith(`${root}/`));

/** 🧾️ Admits the exact rule subset shared by the resolver and the independent graph verifier: path selectors, dependency types and the circular flag, nothing else. */
function strictRule(value: unknown): DependencyDirectionRule {
  const record=(value:unknown):Record<string,unknown>=>{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Dependency direction requires exact strict rule authority");return value as Record<string,unknown>;};
  const rule=record(value);
  if(typeof rule.name!=="string"||!rule.name||Object.keys(rule).some(key=>!["name","severity","comment","scope","from","to"].includes(key))||rule.severity!=="error"||rule.scope!==undefined&&rule.scope!=="module"||rule.comment!==undefined&&typeof rule.comment!=="string")throw Error("Dependency direction requires exact strict rule authority");
  const patterns=(value:unknown,empty=false):readonly string[]=>{
    const paths=typeof value==="string"?[value]:value;
    if(!Array.isArray(paths)||!empty&&!paths.length||paths.some(path=>typeof path!=="string"))throw Error("Dependency direction requires readable rule paths");
    for(const path of paths)try{new RegExp(path,"u");}catch{throw Error("Dependency direction requires valid rule path patterns");}
    return paths;
  };
  const types=(value:unknown):readonly string[]=>{if(!Array.isArray(value)||!value.length||value.some(type=>typeof type!=="string"||!type))throw refuseDirectionV1("invalid","rule.dependency-types-readable");return value;};
  const selector=(value:unknown,allowed:readonly string[]):Record<string,unknown>=>{
    const selected=record(value);
    if(Object.keys(selected).some(key=>!allowed.includes(key)))throw Error("Dependency direction cannot enforce undeclared rule selectors");
    return selected;
  };
  const from=selector(rule.from,["path","pathNot"]),to=selector(rule.to,["path","pathNot","dependencyTypes","dependencyTypesNot","circular"]);
  if(!Object.keys(from).length&&!Object.keys(to).length)throw refuseDirectionV1("invalid","rule.selects-nothing");
  if(Object.hasOwn(to,"circular")&&typeof to.circular!=="boolean")throw refuseDirectionV1("invalid","rule.circular-boolean");
  return {name:rule.name,severity:"error",
    from:{path:Object.hasOwn(from,"path")?patterns(from.path):[""],...(Object.hasOwn(from,"pathNot")?{pathNot:patterns(from.pathNot,true)}:{})},
    to:{path:Object.hasOwn(to,"path")?patterns(to.path):[""],...(Object.hasOwn(to,"pathNot")?{pathNot:patterns(to.pathNot,true)}:{}),...(Object.hasOwn(to,"dependencyTypes")?{dependencyTypes:types(to.dependencyTypes)}:{}),...(Object.hasOwn(to,"dependencyTypesNot")?{dependencyTypesNot:types(to.dependencyTypesNot)}:{}),...(Object.hasOwn(to,"circular")?{circular:to.circular as boolean}:{})}};
}

/** 🪪️ Admits one authored rule or refuses it; the one place that defines which selectors the independent verifier can enforce. */
export function dependencyDirectionRule(value: unknown): DependencyDirectionRule {
  return strictRule(value);
}

/** 📤️ Renders enforced rules in the resolver's own configuration dialect: a selector path that selects every path is omitted again. */
export function dependencyDirectionResolverRules(rules: readonly DependencyDirectionRule[]): readonly Record<string, unknown>[] {
  const open = (path: readonly string[]): boolean => path.length === 1 && path[0] === "";
  const selector = (value: DependencyDirectionRule["from"] | DependencyDirectionRule["to"]): Record<string, unknown> => ({ ...(open(value.path) ? {} : { path: value.path }), ...Object.fromEntries(Object.entries(value).filter(([key]) => key !== "path")) });
  return rules.map((rule) => ({ name: rule.name, severity: rule.severity, from: selector(rule.from), to: selector(rule.to) }));
}

/** 🧮️ Selects complete mandatory semantic authority before any graph is inspected. */
function selectedStrictRules(snapshot: Pick<DependencyPolicySnapshot,"taxonomy"|"plugins"|"policy">,publicApi:boolean): readonly DependencyDirectionRule[] {
  const {taxonomy,plugins,policy}=snapshot;
  if(new Set(plugins).size!==plugins.length||plugins.some(plugin=>typeof plugin!=="string"||!plugin))throw Error("Dependency direction requires distinct present plugin owners");
  const names=new Set(["framework-no-implementation","repo-no-implementation","s-modules-no-plugins",...(publicApi?["plugins-framework-sdk-only"]:[]),...Object.keys(taxonomy.dependencyDirections.rules),...plugins.flatMap(from=>plugins.filter(to=>to!==from).map(to=>`no-cross-plugin-${from}-to-${to}`))]);
  const rules=policy.forbidden.filter(rule=>publicApi?rule.severity==="error":names.has(rule.name)||rule.name.startsWith("plugin-no-extension-or-artifact-")).map(strictRule);
  const present=new Set(rules.map(rule=>rule.name));
  if(rules.length!==present.size||[...names].some(name=>!present.has(name)))throw Error("Canonical architecture is missing or repeats a declared strict rule");
  return rules;
}

/** 🌳️ Requires every intrinsic layer and removable plugin boundary for native source ownership. */
export function dependencyDirectionLayerRules(snapshot: Pick<DependencyPolicySnapshot,"taxonomy"|"plugins"|"policy">): readonly DependencyDirectionRule[] {
  return selectedStrictRules(snapshot,false);
}

/** 🛡️ Extends layer authority with strict authored TypeScript/JavaScript public export contracts. */
export function dependencyDirectionStrictRules(snapshot: Pick<DependencyPolicySnapshot,"taxonomy"|"plugins"|"policy">): readonly DependencyDirectionRule[] {
  return selectedStrictRules(snapshot,true);
}

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
  return workspaces.filter((path) => !dependencyDirectionPathMatches(excludedPaths, path)).flatMap((owner) => {
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

/** 🔗️ Whether a symbolic link leads to a directory; a link whose target is gone leads nowhere and holds no source. */
function linkedDirectory(path: string): boolean {
  try { return statSync(path).isDirectory(); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return false; throw error; }
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
        if (sourceExtension.test(child) || linkedDirectory(join(root, child))) throw new Error(`Dependency direction cannot inventory a linked source: ${child}`);
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
  const matches = dependencyDirectionPathMatches;
  validateScope(scope);
  if (!scope.expectedSources.length) throw new Error("Dependency direction requires source inventory");
  const graph = record(report), modules = array(graph.modules), summary = record(graph.summary);
  const names = rules.map((rule) => rule.name);
  if (!names.length || new Set(names).size !== names.length || rules.some((rule) => rule.severity !== "error")) throw new Error("Dependency direction requires distinct strict rules");
  const authority = (value: unknown): string => {
    const rule = strictRule(value);
    const selector=(value:DependencyDirectionRule["from"]):readonly (string|null)[]=>[value.path.join("|"),value.pathNot?.join("|")??null];
    return JSON.stringify([rule.name,rule.severity,selector(rule.from),selector(rule.to),rule.to.dependencyTypes?.join("|")??null,rule.to.dependencyTypesNot?.join("|")??null,rule.to.circular??null]);
  };
  const usedRules = array(record(summary.ruleSetUsed).forbidden).map(authority).sort();
  if (JSON.stringify(usedRules) !== JSON.stringify(rules.map(authority).sort())) throw new Error("Dependency direction graph used different authored rule authority");
  if (!modules.length || summary.totalCruised !== modules.length) throw new Error("Dependency direction graph is empty or incomplete");
  const edges: DependencyDirectionEdge[] = [], problems: string[] = [], sources = new Map<string, string>(), typed = rules.some((rule) => rule.to.dependencyTypes !== undefined || rule.to.dependencyTypesNot !== undefined), builtOutputs = new WeakSet<object>(), inventoried = new Set(scope.expectedSources), cycleEdges = new Map<string, DependencyDirectionEdge[]>(rules.filter((rule) => rule.to.circular === true).map((rule) => [rule.name, []]));
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
      const specifier = text(dependency.module), bare = specifier.replace(/[?#].*$/u, ""), pkg = scope.workspacePackages.find((pkg) => bare === pkg.name || bare.startsWith(`${pkg.name}/`));
      if (pkg) {
        const subpath = bare === pkg.name ? "." : `.${bare.slice(pkg.name.length)}`;
        if (subpath !== "." && subpath.slice(2).split(/[\\/]/u).some((segment) => !segment || segment === "." || segment === "..")) throw new Error(`Dependency direction graph uses an invalid authored package subpath: ${from} → ${specifier}`);
        const exported = pkg.exports.some((path) => new RegExp(`^${path.split("*").map((part) => part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join(".*")}$`, "u").test(subpath));
        if (!exported) problems.push(`Dependency direction graph uses an undeclared authored package export: ${from} → ${specifier}`);
        const candidates=pkg.exportTargets.flatMap(row=>{
          const expression=new RegExp(`^${row.subpath.split("*").map(part=>part.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("(.*)")}$`,"u"),match=expression.exec(subpath);
          return match?[{...row,capture:match[1]??"",rank:row.subpath.includes("*")?row.subpath.indexOf("*"):Number.MAX_SAFE_INTEGER}]:[];
        }).sort((a,b)=>b.rank-a.rank || b.subpath.length-a.subpath.length);
        const chosen=candidates.filter(row=>row.subpath===candidates[0]?.subpath);
        const installedTarget=to.includes("/node_modules/")?to.slice(to.lastIndexOf("/node_modules/")+1):to;
        const exportPaths=chosen.flatMap(row=>row.target===null?[]:[`${pkg.owner}/${row.target.slice(2).replaceAll("*",row.capture)}`]);
        const builtOutput=dependency.couldNotResolve===true&&chosen.length>0&&exportPaths.length===chosen.length&&(specifier!==bare||exportPaths.some(path=>terminalPath(scope,path)))&&exportPaths.every(path=>terminalPath(scope,path)||/\.d\.[cm]?ts$/u.test(path)||specifier!==bare&&inventoried.has(path));
        if (builtOutput) builtOutputs.add(dependency);
        else if (!chosen.some(row=>{if(row.target===null)return false;const target=row.target.slice(2).replaceAll("*",row.capture);return to===`${pkg.owner}/${target}` || installedTarget===`node_modules/${pkg.name}/${target}`;})) problems.push(`Dependency direction graph resolves an authored package export to a different target: ${from} → ${specifier} → ${to}`);

      }
      const installed = to.split("/").lastIndexOf("node_modules"), ownership = installed < 0 ? to : to.split("/").slice(installed).join("/");
      const kinds = dependency.dependencyTypes;
      if (typed && (!Array.isArray(kinds) || kinds.some((kind) => typeof kind !== "string"))) throw refuseDirectionV1("invalid","graph.dependency-types-required",{from,specifier});
      const typesOf = (kinds ?? []) as readonly string[];
      for (const rule of applicable) if (!matches(rule.to.pathNot ?? [], to) && (matches(rule.to.path, ownership) || (pkg && dependency.couldNotResolve === true && (matches(rule.to.path, specifier) || matches(rule.to.path, `${pkg.owner}/`))))
        && (rule.to.dependencyTypes === undefined || rule.to.dependencyTypes.some((kind) => typesOf.includes(kind)))
        && (rule.to.dependencyTypesNot === undefined || !rule.to.dependencyTypesNot.some((kind) => typesOf.includes(kind)))
        && (rule.to.circular === undefined || (dependency.circular === true) === rule.to.circular)) (rule.to.circular === true ? cycleEdges.get(rule.name)! : edges).push({ rule: rule.name, from, to });
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
      if (dependency.couldNotResolve === true && (local || authored) && !builtOutputs.has(dependency)) problems.push(`Dependency direction graph has an unresolved workspace dependency: ${module.source} → ${specifier}`);
      if (workspacePath(scope, target) && (sourceExtension.test(target) || dependency.followable === true) && !sources.has(target)) problems.push(`Dependency direction graph omits followed workspace source: ${module.source} → ${target}`);
    }
  }
  if (problems.length) throw new Error([...new Set(problems)].join("\n"));
  const key = (edge: DependencyDirectionEdge): string => JSON.stringify([edge.rule, edge.from, edge.to]);
  for (const [rule, candidates] of cycleEdges) {
    const found = new Set(candidates.map((edge) => `${edge.from}\0${edge.to}`)), covered = new Set<string>(), representatives: DependencyDirectionEdge[] = [];
    for (const value of array(summary.violations)) {
      const violation = record(value);
      if (text(record(violation.rule).name) !== rule) continue;
      const from = text(violation.from), to = text(violation.to), cycle = array(violation.cycle).map((member) => text(record(member).name));
      if (!found.has(`${from}\0${to}`) || !cycle.length || cycle[0] !== to || cycle.at(-1) !== from) throw refuseDirectionV1("failed","graph.cycle-not-in-source",{rule,from,to});
      covered.add(`${from}\0${to}`);
      for (let index = 0; index + 1 < cycle.length; index++) covered.add(`${cycle[index]}\0${cycle[index + 1]}`);
      representatives.push({ rule, from, to });
    }
    const uncovered = [...found].filter((edge) => !covered.has(edge));
    if (uncovered.length) throw refuseDirectionV1("failed","graph.cycle-uncovered",{rule,edges:uncovered.map((edge) => edge.replace("\0", " → ")).join(", ")});
    edges.push(...representatives);
  }
  const actual = [...new Set(edges.map(key))].sort();
  const reported = [...new Set(array(summary.violations).map((value) => {
    const violation = record(value);
    const rule = text(record(violation.rule).name);
    if (!names.includes(rule)) throw new Error("Dependency direction graph reports a different policy");
    return key({ rule, from: text(violation.from), to: text(violation.to) });
  }))].sort();
  if (JSON.stringify(actual) !== JSON.stringify(reported) || summary.error !== reported.length) {
    const only = (left: readonly string[], right: readonly string[]): string => { const known = new Set(right), rows = left.filter((row) => !known.has(row)); return `${rows.length}${rows.length ? ` e.g. ${rows.slice(0, 3).join(" ")}` : ""}`; };
    throw new Error(`Dependency direction resolver verdict disagrees with the source graph: only in the verifier ${only(actual, reported)}; only in the resolver ${only(reported, actual)}; resolver error count ${String(summary.error)} for ${reported.length} distinct edges`);
  }
  return actual.map((value) => { const [rule, from, to] = JSON.parse(value) as [string, string, string]; return { rule, from, to }; });
}
