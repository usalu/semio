import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {cpSync,existsSync,mkdirSync,readFileSync,readdirSync,realpathSync,renameSync,rmSync,writeFileSync,statSync} from "node:fs";
import {dirname,join,relative,sep} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

/** 🧾️ The closed unit document: how a TypeScript program is cut into acyclic check units and how a unit is checked. */
export type TypeScriptUnitsDocumentV1 = {
  readonly version: 1;
  readonly seeds: {readonly markers: readonly string[]; readonly manifests: boolean; readonly roots: readonly string[]; readonly isolate: readonly string[]; readonly ambient: readonly string[]};
  readonly cycles: {readonly collapseFiles: number; readonly refineRounds: number};
  readonly coarsen: {readonly minimumFiles: number};
  readonly compiler: {readonly forced: Readonly<Record<string, boolean>>; readonly declarationCodes: readonly (readonly [number, number])[]; readonly extraTypes: readonly string[]};
  readonly messages: {readonly languages: readonly string[]; readonly texts: Readonly<Record<"planning" | "checking" | "hit" | "ran" | "failed" | "done" | "cancelled", Readonly<Record<string, string>>>>};
};
/** 📦️ One check unit: its files, the JSON files it reads from anywhere, the units whose declarations it consumes and whether nobody consumes it (a leaf is only checked: no declarations are emitted, so an unnameable inferred type in a test or script is not a finding). */
export type TypeScriptUnitV1 = {readonly id: string; readonly label: string; readonly files: readonly string[]; readonly json: readonly string[]; readonly dependencies: readonly string[]; readonly leaf: boolean};
/** 🗺️ The acyclic unit graph of one program: units in dependency order and the cyclic cores that were collapsed. */
export type TypeScriptUnitPlanV1 = {readonly version: 1; readonly family: string; readonly units: readonly TypeScriptUnitV1[]; readonly ambient: readonly string[]; readonly manifests: readonly string[]; readonly order: readonly string[]; readonly components: readonly {readonly unit: string; readonly members: number; readonly files: number}[]; readonly digest: string};
/** 🧷️ Resolved workspace-relative import edges of a program: `edges` between source files, `json` from a source file to a JSON file. */
export type TypeScriptImportsV1 = {readonly files: readonly string[]; readonly manifests: readonly string[]; readonly edges: readonly (readonly [string, string])[]; readonly json: readonly (readonly [string, string])[]};

type UnitSpec = {kind: "dir" | "self" | "file" | "group"; prefix: string; isolate?: string};
const unitId = (spec: UnitSpec): string => `${spec.kind}:${spec.prefix}${spec.isolate ? "#" + spec.isolate : ""}`;
const sha = (...parts: readonly (string | Uint8Array)[]): string => {const hash = createHash("sha256"); for (const part of parts) {hash.update(part); hash.update("\0");} return hash.digest("hex");};

/** 🔎️ Reads and closes the document against its schema. */
export function readTypeScriptUnitsDocumentV1(value: unknown): TypeScriptUnitsDocumentV1 {
  const failures = validateJsonSchemaSubset(schema, value);
  if (failures.length) throw Error("Invalid TypeScript units document: " + failures.join("; "));
  return value as TypeScriptUnitsDocumentV1;
}

/** 🔁️ Tarjan strongly connected components of a graph given as adjacency sets, iterative so deep import chains cannot overflow the stack. */
export function stronglyConnectedV1(nodes: readonly string[], adjacent: ReadonlyMap<string, ReadonlySet<string>>): string[][] {
  const index = new Map<string, number>(), low = new Map<string, number>(), onStack = new Set<string>(), stack: string[] = [], out: string[][] = [];
  let counter = 0;
  for (const start of nodes) {
    if (index.has(start)) continue;
    const work: {node: string; next: Iterator<string>}[] = [];
    const enter = (node: string): void => {index.set(node, counter); low.set(node, counter++); stack.push(node); onStack.add(node); work.push({node, next: (adjacent.get(node) ?? new Set<string>())[Symbol.iterator]()});};
    enter(start);
    while (work.length) {
      const frame = work[work.length - 1]!;
      const step = frame.next.next();
      if (!step.done) {
        const target = step.value;
        if (!index.has(target)) enter(target);
        else if (onStack.has(target)) low.set(frame.node, Math.min(low.get(frame.node)!, index.get(target)!));
        continue;
      }
      work.pop();
      if (work.length) {const parent = work[work.length - 1]!.node; low.set(parent, Math.min(low.get(parent)!, low.get(frame.node)!));}
      if (low.get(frame.node) === index.get(frame.node)) {
        const component: string[] = [];
        let member: string;
        do {member = stack.pop()!; onStack.delete(member); component.push(member);} while (member !== frame.node);
        out.push(component.sort());
      }
    }
  }
  return out;
}

/** 🧷️ Compiles a workspace glob (`*`, `**`) to the expression of the paths it selects. */
export function workspaceGlobRegExpV1(glob: string): RegExp {
  let out = "";
  for (let i = 0; i < glob.length; i++) {
    const rest = glob.slice(i);
    if (rest.startsWith("**/")) {out += "(?:.*/)?"; i += 2;}
    else if (rest.startsWith("**")) {out += ".*"; i += 1;}
    else if (glob[i] === "*") out += "[^/]*";
    else out += glob[i]!.replace(/[.+?^${}()|[\]\\]/g, "\\$&");
  }
  return new RegExp("^" + out + "$");
}

const globSegmentsMatch = (glob: readonly string[], segments: readonly string[]): boolean => glob.length === segments.length && glob.every((part, i) => part === "*" || part === segments[i]);

/** 🌱️ The unit roots of a file set: manifest directories outside a marker tree, parents of marker directories and directories matching a root glob. */
export function unitRootsV1(files: readonly string[], manifests: readonly string[], seeds: TypeScriptUnitsDocumentV1["seeds"]): ReadonlySet<string> {
  const roots = new Set<string>([""]), markers = new Set(seeds.markers), globs = seeds.roots.map(glob => glob.split("/"));
  if (seeds.manifests) for (const directory of manifests) if (!directory.split("/").some(segment => markers.has(segment))) roots.add(directory);
  const seen = new Set<string>();
  for (const file of files) {
    const segments = file.split("/").slice(0, -1);
    for (let i = 1; i <= segments.length; i++) {
      const prefix = segments.slice(0, i).join("/");
      if (seen.has(prefix)) continue;
      seen.add(prefix);
      if (markers.has(segments[i - 1]!)) roots.add(segments.slice(0, i - 1).join("/"));
      if (globs.some(glob => globSegmentsMatch(glob, segments.slice(0, i)))) roots.add(prefix);
    }
  }
  return roots;
}

const rootOf = (file: string, roots: ReadonlySet<string>): string => {
  let directory = dirname(file) === "." ? "" : dirname(file);
  while (directory && !roots.has(directory)) directory = dirname(directory) === "." ? "" : dirname(directory);
  return directory;
};

/** 🧮️ Cuts a program into acyclic units: seeds (module roots, isolated directories), refinement of cyclic cores by directory, collapse of what stays cyclic, coarsening of tiny units. Deterministic: equal inputs give an equal plan. */
export function planTypeScriptUnitsV1(imports: TypeScriptImportsV1, document: TypeScriptUnitsDocumentV1, family: string): TypeScriptUnitPlanV1 {
  const ambientMatchers = document.seeds.ambient.map(workspaceGlobRegExpV1), ambient = [...imports.files].filter(file => ambientMatchers.some(matcher => matcher.test(file))).sort(), ambientSet = new Set(ambient);
  const files = [...imports.files].filter(file => !ambientSet.has(file)).sort(), fileSet = new Set(files), roots = unitRootsV1(files, imports.manifests, document.seeds), isolate = new Set(document.seeds.isolate);
  const specs = new Map<string, UnitSpec>(), assign = new Map<string, string>(), frozen = new Set<string>();
  const declare = (spec: UnitSpec): string => {const id = unitId(spec); specs.set(id, spec); return id;};
  for (const file of files) {
    const root = rootOf(file, roots), rest = (root ? file.slice(root.length + 1) : file).split("/").slice(0, -1), name = rest.find(segment => isolate.has(segment));
    assign.set(file, declare(name ? {kind: "dir", prefix: root, isolate: name} : {kind: "dir", prefix: root}));
  }
  const stems = new Map<string, string[]>();
  for (const file of files) if (/\.tsx?$/.test(file) && !file.endsWith(".d.ts")) {const key = assign.get(file)! + "\0" + file.replace(/\.tsx?$/, ""); (stems.get(key) ?? stems.set(key, []).get(key)!).push(file);}
  for (const group of stems.values()) if (group.length > 1) for (const file of group.filter(name => name.endsWith(".tsx"))) assign.set(file, declare({kind: "file", prefix: file}));
  const edges = imports.edges.filter(([from, to]) => fileSet.has(from) && fileSet.has(to) && from !== to);
  const quotient = (): {members: Map<string, string[]>; adjacent: Map<string, Set<string>>} => {
    const members = new Map<string, string[]>(), adjacent = new Map<string, Set<string>>();
    for (const file of files) {const id = assign.get(file)!; (members.get(id) ?? members.set(id, []).get(id)!).push(file);}
    for (const id of members.keys()) adjacent.set(id, new Set());
    for (const [from, to] of edges) {const a = assign.get(from)!, b = assign.get(to)!; if (a !== b) adjacent.get(a)!.add(b);}
    return {members, adjacent};
  };
  const collapse = (component: readonly string[], members: Map<string, string[]>): void => {
    const id = declare({kind: "group", prefix: [...component].sort()[0]!.replace(/^[a-z]+:/, "")});
    for (const unit of component) for (const file of members.get(unit)!) assign.set(file, id);
    frozen.add(id);
  };
  for (let round = 1; ; round++) {
    const {members, adjacent} = quotient(), cyclic = stronglyConnectedV1([...members.keys()].sort(), adjacent).filter(component => component.length > 1);
    if (!cyclic.length) break;
    for (const component of cyclic) {
      const componentFiles = component.flatMap(unit => members.get(unit)!), clash = declarationCollision(componentFiles);
      const splittable = (unit: string): boolean => {const spec = specs.get(unit)!; return members.get(unit)!.length > 1 && spec.kind !== "file" && spec.kind !== "group" && (clash || (spec.kind !== "self" && !frozen.has(unit)));};
      const divisible = component.some(splittable);
      if (clash ? !divisible : componentFiles.length <= document.cycles.collapseFiles || round > document.cycles.refineRounds || !divisible) {collapse(component, members); continue;}
      for (const unit of component) {
        const spec = specs.get(unit)!, list = members.get(unit)!;
        if (!splittable(unit)) {frozen.add(unit); continue;}
        if (spec.kind === "self") {for (const file of list) assign.set(file, declare({kind: "file", prefix: file})); continue;}
        for (const file of list) {
          const rest = (spec.prefix ? file.slice(spec.prefix.length + 1) : file).split("/");
          assign.set(file, rest.length === 1 ? declare({kind: "self", prefix: spec.prefix, isolate: spec.isolate}) : declare({kind: "dir", prefix: spec.prefix ? spec.prefix + "/" + rest[0] : rest[0]!, isolate: spec.isolate}));
        }
      }
    }
  }
  coarsen(files, assign, specs, edges, document.coarsen.minimumFiles, declare);
  const {members, adjacent} = quotient(), ids = [...members.keys()].sort(), jsonByUnit = new Map<string, Set<string>>(), componentRows = ids.filter(id => specs.get(id)!.kind === "group").map(id => ({unit: id, members: 0, files: members.get(id)!.length}));
  for (const [from, to] of imports.json) {const unit = assign.get(from); if (unit) (jsonByUnit.get(unit) ?? jsonByUnit.set(unit, new Set()).get(unit)!).add(to);}
  const incoming = new Map<string, number>(ids.map(id => [id, 0]));
  for (const targets of adjacent.values()) for (const target of targets) incoming.set(target, incoming.get(target)! + 1);
  const dependenciesOf = (id: string): string[] => [...adjacent.get(id)!].sort(), order: string[] = [], remaining = new Map<string, number>(ids.map(id => [id, dependenciesOf(id).length])), dependents = new Map<string, string[]>();
  for (const id of ids) for (const dependency of dependenciesOf(id)) (dependents.get(dependency) ?? dependents.set(dependency, []).get(dependency)!).push(id);
  const ready = ids.filter(id => remaining.get(id) === 0);
  while (ready.length) {
    ready.sort(); const id = ready.shift()!; order.push(id);
    for (const dependent of dependents.get(id) ?? []) {remaining.set(dependent, remaining.get(dependent)! - 1); if (remaining.get(dependent) === 0) ready.push(dependent);}
  }
  if (order.length !== ids.length) throw Error("TypeScript unit graph is cyclic after refinement");
  const units = ids.map((id): TypeScriptUnitV1 => ({id, label: specs.get(id)!.prefix || ".", files: members.get(id)!, json: [...(jsonByUnit.get(id) ?? [])].sort(), dependencies: dependenciesOf(id), leaf: !dependents.has(id)}));
  const body = {version: 1 as const, family, units, ambient, manifests: [...imports.manifests].sort().map(directory => (directory ? directory + "/" : "") + "package.json"), order, components: componentRows};
  return {...body, digest: sha(JSON.stringify(body))};
}

/** 🧱️ Two sources with one stem (`x.ts`, `x.tsx`) would write one declaration path, so they may never share a unit. */
const declarationCollision = (files: readonly string[]): boolean => {const seen = new Set<string>(); for (const file of files) if (/\.tsx?$/.test(file) && !file.endsWith(".d.ts")) {const stem = file.replace(/\.tsx?$/, ""); if (seen.has(stem)) return true; seen.add(stem);} return false;};

function coarsen(files: readonly string[], assign: Map<string, string>, specs: Map<string, UnitSpec>, edges: readonly (readonly [string, string])[], minimum: number, declare: (spec: UnitSpec) => string): void {
  if (minimum < 2) return;
  const members = new Map<string, string[]>(), out = new Map<string, Set<string>>(), into = new Map<string, Set<string>>();
  for (const file of files) {const id = assign.get(file)!; (members.get(id) ?? members.set(id, []).get(id)!).push(file);}
  for (const id of members.keys()) {out.set(id, new Set()); into.set(id, new Set());}
  for (const [from, to] of edges) {const a = assign.get(from)!, b = assign.get(to)!; if (a !== b) {out.get(a)!.add(b); into.get(b)!.add(a);}}
  const reach = (starts: ReadonlySet<string>, adjacent: Map<string, Set<string>>): Set<string> => {const seen = new Set(starts), stack = [...starts]; while (stack.length) for (const next of adjacent.get(stack.pop()!) ?? []) if (!seen.has(next)) {seen.add(next); stack.push(next);} return seen;};
  const parentKey = (id: string): string => {const spec = specs.get(id)!; return (spec.prefix.includes("/") ? dirname(spec.prefix) : "") + "|" + (spec.isolate ?? "") + "|" + (spec.kind === "group" ? id : "");};
  const buckets = new Map<string, string[]>();
  for (const [id, list] of members) if (list.length < minimum && specs.get(id)!.kind !== "group") (buckets.get(parentKey(id)) ?? buckets.set(parentKey(id), []).get(parentKey(id))!).push(id);
  const merge = (cluster: readonly string[]): void => {
    const id = declare({kind: "group", prefix: "cluster/" + [...cluster].sort()[0]!.replace(/^[a-z]+:/, "")}), set = new Set(cluster), newOut = new Set<string>(), newIn = new Set<string>();
    for (const unit of cluster) {for (const file of members.get(unit)!) assign.set(file, id); for (const next of out.get(unit)!) newOut.add(next); for (const previous of into.get(unit)!) newIn.add(previous);}
    for (const unit of cluster) {members.set(id, [...(members.get(id) ?? []), ...members.get(unit)!]); members.delete(unit); out.delete(unit); into.delete(unit);}
    for (const next of set) {newOut.delete(next); newIn.delete(next);}
    out.set(id, newOut); into.set(id, newIn);
    for (const next of newOut) into.set(next, new Set([...into.get(next)!].filter(unit => !set.has(unit)).concat(id)));
    for (const previous of newIn) out.set(previous, new Set([...out.get(previous)!].filter(unit => !set.has(unit)).concat(id)));
  };
  for (const key of [...buckets.keys()].sort()) {
    let cluster: string[] = [], size = 0;
    for (const id of buckets.get(key)!.sort()) {
      if (!members.has(id)) continue;
      const trial = new Set([...cluster, id]), downstream = reach(trial, out), upstream = reach(trial, into);
      for (const unit of trial) {downstream.delete(unit); upstream.delete(unit);}
      if ([...downstream].some(unit => upstream.has(unit))) continue;
      if (declarationCollision([...trial].flatMap(unit => members.get(unit)!))) continue;
      cluster = [...cluster, id]; size += members.get(id)!.length;
      if (size >= minimum) {if (cluster.length > 1) merge(cluster); cluster = []; size = 0;}
    }
    if (cluster.length > 1) merge(cluster);
  }
}

/** 🔑️ The content address of one unit check: compiler and options identity, the digests of its files, JSON inputs, ambient files and every package manifest of the program (module resolution reads them) and the declaration digests of its dependencies. A dependency whose declarations did not change leaves the key of its dependents unchanged. */
export function typeScriptUnitKeyV1(input: {readonly compiler: string; readonly options: string; readonly environment: string; readonly unit: TypeScriptUnitV1; readonly ambient: readonly string[]; readonly manifests: readonly string[]; readonly digest: (path: string) => string; readonly dependencies: ReadonlyMap<string, string>}): string {
  const {unit} = input;
  return sha("typescript-unit-v1", input.compiler, input.options, input.environment, unit.id, String(unit.leaf), JSON.stringify(unit.files.map(file => [file, input.digest(file)])), JSON.stringify(unit.json.map(file => [file, input.digest(file)])), JSON.stringify(input.ambient.map(file => [file, input.digest(file)])), JSON.stringify(input.manifests.map(file => [file, input.digest(file)])), JSON.stringify(unit.dependencies.map(dependency => [dependency, input.dependencies.get(dependency)])));
}

/** 🧾️ What a finished unit check records: the diagnostics replayed on every hit and the digest of its declaration tree. */
export type TypeScriptUnitReceiptV1 = {readonly version: 1; readonly key: string; readonly unit: string; readonly errors: number; readonly boundary: number; readonly diagnostics: string; readonly declarations: string; readonly milliseconds: number};

/** 🧰️ The effects a run needs; the repository wires its content store, process runner and storage leases here. */
export type TypeScriptUnitsHostV1 = {
  readonly workspace: string;
  readonly compiler: string;
  readonly options: string;
  readonly environment: string;
  readonly family: string;
  readonly typeRoots: readonly string[];
  readonly typeFiles: readonly string[];
  readonly store: string;
  readonly session: string;
  readonly state: string;
  readonly digest: (path: string) => string;
  readonly tsc: (config: string, signal: AbortSignal) => Promise<{readonly code: number; readonly output: string}>;
};
export type TypeScriptUnitsProgressV1 = {readonly unit: string; readonly status: "hit" | "ran" | "failed"; readonly done: number; readonly total: number; readonly milliseconds: number; readonly errors: number};
export type TypeScriptUnitsResultV1 = {readonly errors: number; readonly boundary: number; readonly units: readonly {readonly id: string; readonly key: string; readonly status: "hit" | "ran" | "failed"; readonly errors: number; readonly boundary: number; readonly milliseconds: number}[]; readonly diagnostics: string};

const unitFileName = (id: string): string => sha(id).slice(0, 24);
const atomicWrite = (path: string, text: string): void => {mkdirSync(dirname(path), {recursive: true}); const temporary = `${path}.${process.pid}.${Date.now()}.tmp`; writeFileSync(temporary, text); renameSync(temporary, path);};
const treeDigest = (root: string): string => {
  const rows: string[] = [], walk = (directory: string): void => {for (const entry of readdirSync(directory, {withFileTypes: true}).sort((a, b) => a.name < b.name ? -1 : 1)) {const path = join(directory, entry.name); if (entry.isDirectory()) walk(path); else rows.push(relative(root, path).split(sep).join("/") + "\0" + sha(readFileSync(path)));}};
  if (existsSync(root)) walk(root);
  return sha(...rows);
};

/** 🏗️ The compiler configuration of one unit check: composite declaration-only emit into a session directory, references to the consumption views of its dependencies. `include` is emptied so the family's globs cannot widen the program. */
export function typeScriptUnitConfigV1(host: TypeScriptUnitsHostV1, document: TypeScriptUnitsDocumentV1, unit: TypeScriptUnitV1, ambient: readonly string[], references: readonly string[]): unknown {
  const forced = unit.leaf ? {...document.compiler.forced, composite: false, declaration: false, emitDeclarationOnly: false, noEmit: true} : document.compiler.forced;
  return {extends: join(host.workspace, host.family), compilerOptions: {...forced, typeRoots: host.typeRoots, rootDir: host.workspace, outDir: join(host.state, "sessions", host.session, "out", unitFileName(unit.id)), tsBuildInfoFile: join(host.state, "sessions", host.session, "incremental", unitFileName(unit.id) + ".tsbuildinfo")}, include: [], files: [...unit.files, ...unit.json, ...ambient].map(file => join(host.workspace, file)).concat(host.typeFiles), references: references.map(path => ({path}))};
}
/** 👁️ The consumption view of a finished unit: only what a dependent needs to map its sources onto the stored declarations. */
export function typeScriptUnitViewV1(host: TypeScriptUnitsHostV1, document: TypeScriptUnitsDocumentV1, unit: TypeScriptUnitV1, key: string, references: readonly string[]): unknown {
  return {extends: join(host.workspace, host.family), compilerOptions: {...document.compiler.forced, typeRoots: host.typeRoots, rootDir: host.workspace, outDir: join(host.store, "declarations", key)}, include: [], files: unit.files.map(file => join(host.workspace, file)), references: references.map(path => ({path}))};
}

/** 🚀️ Checks the units of a plan in dependency order with bounded concurrency: cached receipts replay, others run the compiler once. Declaration trees are published to the content store atomically; incremental compiler state stays in the session. */
export async function checkTypeScriptUnitsV1(plan: TypeScriptUnitPlanV1, document: TypeScriptUnitsDocumentV1, host: TypeScriptUnitsHostV1, options: {readonly concurrency: number; readonly only?: readonly string[]; readonly signal: AbortSignal; readonly onProgress?: (row: TypeScriptUnitsProgressV1) => void}): Promise<TypeScriptUnitsResultV1> {
  if (realpathSync(host.workspace) !== host.workspace) throw Error("TypeScript units need the canonical workspace path (a symlinked path hides the declaration boundary: the compiler would read dependency sources)");
  const byId = new Map(plan.units.map(unit => [unit.id, unit])), needed = new Set<string>();
  const need = (id: string): void => {if (needed.has(id)) return; needed.add(id); for (const dependency of byId.get(id)!.dependencies) need(dependency);};
  for (const id of options.only ?? plan.order) need(id);
  const order = plan.order.filter(id => needed.has(id)), done = new Map<string, Promise<{key: string; declarations: string; view: string}>>(), results: TypeScriptUnitsResultV1["units"][number][] = [], diagnostics: string[] = [];
  let finished = 0, running = 0;
  const waiting: (() => void)[] = [], slot = async (): Promise<void> => {while (running >= options.concurrency) await new Promise<void>(resolve => waiting.push(resolve)); running++;}, release = (): void => {running--; waiting.shift()?.();};
  const run = (id: string): Promise<{key: string; declarations: string; view: string}> => done.get(id) ?? done.set(id, (async () => {
    const unit = byId.get(id)!, dependencies = await Promise.all(unit.dependencies.map(run)), declarations = new Map(unit.dependencies.map((dependency, index) => [dependency, dependencies[index]!.declarations]));
    const key = typeScriptUnitKeyV1({compiler: host.compiler, options: host.options, environment: host.environment, unit, ambient: plan.ambient, manifests: plan.manifests, digest: host.digest, dependencies: declarations});
    const receiptPath = join(host.store, "receipts", key.slice(0, 2), key + ".json"), viewPath = join(host.store, "views", key + ".json"), references = dependencies.map(row => row.view);
    atomicWrite(viewPath, JSON.stringify(typeScriptUnitViewV1(host, document, unit, key, references)));
    let receipt: TypeScriptUnitReceiptV1 | undefined, status: "hit" | "ran" | "failed" = "hit";
    if (existsSync(receiptPath) && existsSync(join(host.store, "declarations", key))) try {receipt = JSON.parse(readFileSync(receiptPath, "utf8")) as TypeScriptUnitReceiptV1;} catch {receipt = undefined;}
    if (!receipt) {
      await slot();
      try {
        if (options.signal.aborted) throw Error(document.messages.texts.cancelled[Object.keys(document.messages.texts.cancelled)[0]!]);
        const started = performance.now(), configPath = join(host.state, "sessions", host.session, "configs", unitFileName(unit.id) + ".json"), out = join(host.state, "sessions", host.session, "out", unitFileName(unit.id));
        atomicWrite(configPath, JSON.stringify(typeScriptUnitConfigV1(host, document, unit, plan.ambient, references)));
        const expected = unit.leaf ? [] : unit.files.filter(file => !file.endsWith(".d.ts") && /\.(ts|tsx|mts|cts)$/.test(file)).map(file => file.replace(/\.(ts|tsx)$/, ".d.ts").replace(/\.mts$/, ".d.mts").replace(/\.cts$/, ".d.cts"));
        const incomplete = (): boolean => expected.some(file => !existsSync(join(out, file)));
        let result = await host.tsc(configPath, options.signal);
        if (!options.signal.aborted && incomplete()) {
          rmSync(out, {recursive: true, force: true}); rmSync(join(host.state, "sessions", host.session, "incremental", unitFileName(unit.id) + ".tsbuildinfo"), {force: true});
          result = await host.tsc(configPath, options.signal);
        }
        if (options.signal.aborted) throw Error("cancelled");
        const published = join(host.store, "declarations", key), staging = published + "." + host.session + ".tmp";
        rmSync(staging, {recursive: true, force: true}); mkdirSync(staging, {recursive: true});
        for (const file of expected) if (existsSync(join(out, file))) {mkdirSync(dirname(join(staging, file)), {recursive: true}); cpSync(join(out, file), join(staging, file));}
        for (const file of unit.leaf ? [] : unit.files) if (file.endsWith(".d.ts")) {mkdirSync(dirname(join(staging, file)), {recursive: true}); cpSync(join(host.workspace, file), join(staging, file));}
        const codes = [...result.output.matchAll(/error TS(\d+)/g)].map(row => Number(row[1])), boundary = codes.filter(code => document.compiler.declarationCodes.some(([from, to]) => code >= from && code <= to)).length, errors = codes.length - boundary, milliseconds = Math.round(performance.now() - started);
        if (existsSync(published)) rmSync(staging, {recursive: true, force: true}); else renameSync(staging, published);
        receipt = {version: 1, key, unit: unit.id, errors, boundary, diagnostics: result.output.trim(), declarations: treeDigest(published), milliseconds};
        atomicWrite(receiptPath, JSON.stringify(receipt));
        status = errors ? "failed" : "ran";
      } finally {release();}
    }
    finished++; if (receipt.errors || receipt.boundary) diagnostics.push(receipt.diagnostics);
    results.push({id, key, status, errors: receipt.errors, boundary: receipt.boundary, milliseconds: status === "hit" ? 0 : receipt.milliseconds});
    options.onProgress?.({unit: id, status, done: finished, total: order.length, milliseconds: receipt.milliseconds, errors: receipt.errors});
    return {key, declarations: receipt.declarations, view: viewPath};
  })()).get(id)!;
  await Promise.all(order.map(run));
  const rows = results.sort((a, b) => a.id < b.id ? -1 : 1);
  return {errors: rows.reduce((sum, row) => sum + row.errors, 0), boundary: rows.reduce((sum, row) => sum + row.boundary, 0), units: rows, diagnostics: diagnostics.join("\n")};
}

/** 🧹️ Removes session compiler state not touched for `maximumAgeMilliseconds`. Sessions never share files, so removal needs no lock. */
export function pruneTypeScriptSessionsV1(state: string, maximumAgeMilliseconds: number, now = Date.now()): readonly string[] {
  const root = join(state, "sessions"), removed: string[] = [];
  if (!existsSync(root)) return removed;
  for (const name of readdirSync(root)) if (now - statSync(join(root, name)).mtimeMs > maximumAgeMilliseconds) {rmSync(join(root, name), {recursive: true, force: true}); removed.push(name);}
  return removed;
}

/** 🧠️ Remembers the resolved workspace imports of one file for one content digest, so an edit re-derives only the files it touches. */
export type TypeScriptImportsMemoV1 = {readonly get: (file: string, digest: string) => {readonly targets: readonly string[]; readonly json: readonly string[]} | undefined; readonly set: (file: string, digest: string, row: {readonly targets: readonly string[]; readonly json: readonly string[]}) => void};

/** 🛰️ Derives the import graph of a program with the compiler's own preprocessing and module resolution: breadth-first from the root files, workspace files only; third-party resolutions end at node_modules. Manifest directories are found as ancestors of the program's files (no directory walk). */
export function deriveTypeScriptImportsV1(ts: typeof import("typescript"), options: {readonly workspace: string; readonly compilerOptions: import("typescript").CompilerOptions; readonly roots: readonly string[]; readonly exists: (path: string) => boolean; readonly read: (path: string) => string | undefined; readonly digest?: (path: string) => string; readonly memo?: TypeScriptImportsMemoV1; readonly onFile?: (done: number, derived: number) => void; readonly signal?: AbortSignal}): TypeScriptImportsV1 {
  const prefix = options.workspace.endsWith("/") ? options.workspace : options.workspace + "/", cache = ts.createModuleResolutionCache(options.workspace, name => name, options.compilerOptions);
  const seen = new Set<string>(), queue = [...options.roots], edges: [string, string][] = [], json: [string, string][] = [], files: string[] = [];
  const real = (ts.sys.realpath?.(options.workspace) ?? options.workspace).replace(/\/?$/, "/"), inside = (path: string): string | undefined => {
    if (path.includes("/node_modules/")) return undefined;
    return path.startsWith(prefix) ? path.slice(prefix.length) : path.startsWith(real) ? path.slice(real.length) : undefined;
  };
  let derived = 0;
  while (queue.length) {
    if (options.signal?.aborted) throw Error("cancelled");
    const file = queue.pop()!;
    if (seen.has(file)) continue;
    seen.add(file); files.push(file);
    const digest = options.memo && options.digest ? options.digest(file) : undefined, known = digest === undefined ? undefined : options.memo!.get(file, digest);
    let targets: readonly string[], jsonTargets: readonly string[];
    if (known) {targets = known.targets; jsonTargets = known.json;}
    else {
      const text = options.read(prefix + file), found = new Set<string>(), foundJson = new Set<string>();
      if (text !== undefined) {
        const info = ts.preProcessFile(text, true, true), specifiers = new Set<string>([...info.importedFiles.map(row => row.fileName), ...info.referencedFiles.map(row => row.fileName)]);
        for (const specifier of specifiers) {
          const resolved = ts.resolveModuleName(specifier, prefix + file, options.compilerOptions, ts.sys, cache).resolvedModule?.resolvedFileName, target = resolved ? inside(resolved) : undefined;
          if (!target || target === file) continue;
          (target.endsWith(".json") ? foundJson : found).add(target);
        }
      }
      targets = [...found].sort(); jsonTargets = [...foundJson].sort(); derived++;
      if (digest !== undefined) options.memo!.set(file, digest, {targets, json: jsonTargets});
    }
    for (const target of targets) {edges.push([file, target]); if (!seen.has(target)) queue.push(target);}
    for (const target of jsonTargets) json.push([file, target]);
    options.onFile?.(files.length, derived);
  }
  const manifests = new Set<string>(), checked = new Map<string, boolean>();
  const hasManifest = (directory: string): boolean => {let known = checked.get(directory); if (known === undefined) {known = options.exists(prefix + (directory ? directory + "/" : "") + "package.json"); checked.set(directory, known);} return known;};
  for (const file of files) for (let directory = dirname(file) === "." ? "" : dirname(file); ; directory = dirname(directory) === "." ? "" : dirname(directory)) {if (hasManifest(directory)) manifests.add(directory); if (!directory) break;}
  return {files: files.sort(), manifests: [...manifests].sort(), edges, json};
}

/** 🏃️ Runs the compiler of the repository as a child process under the host's abort signal. */
export function runTypeScriptCompilerV1(node: string, tsc: string, workspace: string, environment: Readonly<Record<string, string | undefined>>): TypeScriptUnitsHostV1["tsc"] {
  return (config, signal) => new Promise((resolve, reject) => {
    const child = spawn(node, [tsc, "-p", config, "--pretty", "false"], {cwd: workspace, env: environment as NodeJS.ProcessEnv, signal}), chunks: Buffer[] = [];
    child.stdout.on("data", chunk => chunks.push(chunk)); child.stderr.on("data", chunk => chunks.push(chunk));
    child.on("error", reject); child.on("close", code => resolve({code: code ?? 1, output: Buffer.concat(chunks).toString("utf8")}));
  });
}
