import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};
import {resolveBuildHelpLanguageV1} from "../../⚙️operations/📖️help/🟦️.ts";
import {cargoLockClosure} from "../🔒️lock/🟨️.mjs";

/** 🧾️ One parsed TOML table: the shape every manifest, lock and configuration has once a parser has read it. */
export type CargoTomlV1 = {readonly [key: string]: unknown};
/** 📄️ One parsed file together with the digest of its bytes. */
export type CargoFileV1 = {readonly value: CargoTomlV1; readonly sha: string};
/** 🧰️ The only I/O the resolution identity owns: read one repository-relative TOML file with the digest of its bytes (`null` when it does not exist) and digest a file without parsing it. */
export type CargoReadHostV1 = {readonly read: (path: string) => Promise<CargoFileV1 | null>; readonly digest: (path: string) => Promise<string | null>};
/** 📖️ One file a derivation read and the digest of its bytes when it did (`null`: it did not exist). */
export type CargoReadV1 = {readonly path: string; readonly sha: string | null};
/** 🧬️ What one crate resolved to for one run kind (`b` build, `t` test root): the files it read, what it inherits, what its workspace fixes, the path dependencies with the identities they had and the folded identity. */
export type CargoUnitV1 = {readonly id: string; readonly root: string | null; readonly reads: readonly CargoReadV1[]; readonly own: unknown; readonly fixed: unknown; readonly children: readonly {readonly path: string; readonly id: string}[]};
/** 🔒️ The locked slice of a crate and what it was derived from: the lock file's digest and the digest of the files the crate's unit read. */
export type CargoLockReceiptV1 = {readonly path: string; readonly sha: string | null; readonly basis: string; readonly digest: string};
/** 📇️ The persisted receipt of one crate: its units and its locked slice. */
export type CargoLeafV1 = {readonly version: 1; readonly crate: string; readonly b?: CargoUnitV1; readonly t?: CargoUnitV1; readonly lock?: CargoLockReceiptV1};
/** 💾️ Where receipts live: read is mandatory, write only where a writer custody exists (the graph build); a reader never writes. */
export type CargoReceiptStoreV1 = {readonly read: (crate: string) => CargoLeafV1 | undefined | Promise<CargoLeafV1 | undefined>; readonly write?: (leaf: CargoLeafV1) => void | Promise<void>};
/** 🗣️ The refusal and finding texts in every language of the document. */
export type CargoResolutionMessagesV1 = {readonly languages: readonly string[]; readonly texts: Readonly<Record<"workspaceKey" | "cycle" | "escape" | "workspace", Readonly<Record<string, string>>>>};
/** 📕️ The closed document: which toolchain files the identity digests semantically, which keys may never be whole-workspace and the reasons that exempt a target. */
export type CargoResolutionPolicyV1 = {
  readonly version: 1;
  readonly toolchain: {readonly pin: string; readonly config: string; readonly configExcluded: readonly string[]};
  readonly workspaceKeys: {readonly literals: readonly string[]; readonly tails: readonly string[]; readonly exempt: readonly {readonly command: string; readonly reason: string}[]; readonly exemptTargets: readonly {readonly targets: readonly string[]; readonly reason: string}[]};
  readonly messages: CargoResolutionMessagesV1;
};
/** 🏷️ The digests one crate resolves to, kept apart so a finding can say which part moved. */
export type CargoResolutionPartsV1 = {readonly lock: string; readonly workspace: string; readonly toolchain: string; readonly config: string};

/** 🚫️ Refusal of a resolution that cannot be bounded: a path dependency cycle, a path that leaves the repository or a workspace the crate does not have. */
export class CargoResolutionErrorV1 extends Error {
  constructor(message: string) {
    super(message);
    this.name = "CargoResolutionErrorV1";
  }
}

const DEPENDENCY_KINDS = ["dependencies", "build-dependencies", "dev-dependencies"] as const;
const sha = (value: string): string => createHash("sha256").update(value).digest("hex");
const bytes = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));
const table = (value: unknown): CargoTomlV1 | undefined => (typeof value === "object" && value !== null && !Array.isArray(value) ? (value as CargoTomlV1) : undefined);
const at = (directory: string, name: string): string => (directory === "" ? name : `${directory}/${name}`);
const parent = (directory: string): string => (directory.includes("/") ? directory.slice(0, directory.lastIndexOf("/")) : "");
const resolvePath = (base: string, relative: string): string | undefined => {
  const parts = base === "" ? [] : base.split("/");
  for (const part of relative.split("/")) {
    if (part === "" || part === ".") continue;
    if (part === "..") {
      if (!parts.length) return undefined;
      parts.pop();
    } else parts.push(part);
  }
  return parts.join("/");
};

/** 🔠️ Serializes a value with sorted object keys (byte order) and no whitespace, so equal data always has equal text in every language. */
export function canonicalJsonV1(value: unknown): string {
  if (value === null || typeof value !== "object") return JSON.stringify(value) ?? "null";
  if (Array.isArray(value)) return `[${value.map((item) => canonicalJsonV1(item)).join(",")}]`;
  const row = value as Record<string, unknown>;
  return `{${Object.keys(row).filter((key) => row[key] !== undefined).sort(bytes).map((key) => `${JSON.stringify(key)}:${canonicalJsonV1(row[key])}`).join(",")}}`;
}

/** ✅️ Admits a document against its closed schema. */
export function readCargoResolutionPolicyV1(value: unknown): CargoResolutionPolicyV1 {
  const failures = validateJsonSchemaSubset(schema, value, schema);
  if (failures.length) throw new CargoResolutionErrorV1(`Cargo resolution document refused: ${failures.slice(0, 3).join("; ")}`);
  return value as CargoResolutionPolicyV1;
}

/** 🍃️ Admits one persisted receipt against its closed schema. */
export function readCargoLeafV1(value: unknown): CargoLeafV1 {
  const failures = validateJsonSchemaSubset(schema.definitions.Leaf as never, value as never);
  if (failures.length) throw new CargoResolutionErrorV1(`Cargo receipt refused: ${failures.slice(0, 3).join("; ")}`);
  return value as CargoLeafV1;
}

/** 💬️ The text of one message in a language, with its `{name}` holes filled. */
export function cargoResolutionTextV1(policy: Pick<CargoResolutionPolicyV1, "messages">, key: keyof CargoResolutionMessagesV1["texts"], language: string, values: Readonly<Record<string, string>>): string {
  const texts = policy.messages.texts[key];
  return (texts[language] as string).replace(/\{(\w+)\}/gu, (_, name: string) => values[name] ?? "");
}
/** 🌐️ The language of a message: the user's own when the document has it, else its first. */
export const cargoResolutionLanguageV1 = (policy: Pick<CargoResolutionPolicyV1, "messages">, environment: Readonly<Record<string, string | undefined>>): string => resolveBuildHelpLanguageV1(policy.messages.languages, [], environment);

/** 🧱️ The Cargo workspace that owns a crate and every file the search read: the explicit `package.workspace` path or the nearest ancestor (the crate itself first) whose manifest declares `[workspace]`. */
export async function locateWorkspaceV1(host: CargoReadHostV1, crate: string): Promise<{readonly workspace: {readonly root: string; readonly manifest: CargoTomlV1} | null; readonly manifest: CargoFileV1 | null; readonly reads: readonly CargoReadV1[]}> {
  const reads: CargoReadV1[] = [], own = await host.read(at(crate, "Cargo.toml"));
  reads.push({path: at(crate, "Cargo.toml"), sha: own?.sha ?? null});
  const explicit = table(own?.value.package)?.workspace;
  let root = typeof explicit === "string" ? resolvePath(crate, explicit) : crate;
  if (root === undefined) throw new CargoResolutionErrorV1(`Cargo workspace path escapes the repository: ${crate}`);
  for (;;) {
    const path = at(root, "Cargo.toml"), file = path === reads[0]!.path ? own : await host.read(path);
    if (path !== reads[0]!.path) reads.push({path, sha: file?.sha ?? null});
    if (file && table(file.value.workspace)) return {workspace: {root, manifest: file.value}, manifest: own, reads};
    if (typeof explicit === "string") throw new CargoResolutionErrorV1(`Cargo workspace path names no [workspace]: ${crate} -> ${root}`);
    if (root === "") return {workspace: null, manifest: own, reads};
    root = parent(root);
  }
}

/** 📁️ The Cargo workspace that owns a crate. */
export async function owningWorkspaceV1(host: CargoReadHostV1, crate: string): Promise<{readonly root: string; readonly manifest: CargoTomlV1} | null> {
  return (await locateWorkspaceV1(host, crate)).workspace;
}

/** 📦️ One dependency declaration of a manifest: the table it sits in, its kind, its alias and the path it resolves to (relative to the repository) when it is a path dependency. */
export type CargoDependencyV1 = {readonly table: string; readonly kind: (typeof DEPENDENCY_KINDS)[number]; readonly alias: string; readonly inherited: boolean; readonly path?: string};

/** ⛓️ Every table dependency of a crate in the order Cargo reads them; development dependencies only when the crate is the root of a test run. */
export function cargoDependenciesV1(crate: string, manifest: CargoTomlV1, workspace: {readonly root: string; readonly manifest: CargoTomlV1} | null, tests: boolean): CargoDependencyV1[] {
  const rows: CargoDependencyV1[] = [], inherited = table(table(workspace?.manifest.workspace)?.dependencies) ?? {};
  const tables: [string, CargoTomlV1][] = [["", manifest], ...Object.entries(table(manifest.target) ?? {}).flatMap(([name, value]): [string, CargoTomlV1][] => (table(value) ? [[`target.${name}`, table(value)!]] : []))];
  for (const [name, source] of tables) {
    for (const kind of DEPENDENCY_KINDS) {
      if (kind === "dev-dependencies" && !tests) continue;
      for (const [alias, declaration] of Object.entries(table(source[kind]) ?? {})) {
        const row = table(declaration);
        if (!row) continue;
        const through = row.workspace === true, resolved = through ? table(inherited[alias]) : row, path = typeof resolved?.path === "string" ? resolvePath(through ? workspace!.root : crate, resolved.path) : undefined;
        if (typeof resolved?.path === "string" && path === undefined) throw new CargoResolutionErrorV1(`Cargo path dependency escapes the repository: ${crate} -> ${resolved.path}`);
        rows.push({table: name, kind, alias, inherited: through, ...(path === undefined ? {} : {path})});
      }
    }
  }
  return rows.sort((left, right) => bytes(`${left.table}\0${left.kind}\0${left.alias}`, `${right.table}\0${right.kind}\0${right.alias}`));
}

/** ✂️ What one crate inherits from its workspace: the `[workspace.package]` fields it names, the lints it takes and the `[workspace.dependencies]` entries it uses. */
export function inheritedSliceV1(manifest: CargoTomlV1, workspace: CargoTomlV1 | null, dependencies: readonly CargoDependencyV1[]): Record<string, unknown> {
  const fields = table(workspace?.package) ?? {}, wanted = table(manifest.package) ?? {}, shared = table(workspace?.dependencies) ?? {};
  const taken = Object.keys(wanted).filter((key) => table(wanted[key])?.workspace === true).sort(bytes);
  return {
    package: Object.fromEntries(taken.map((key) => [key, fields[key] ?? null])),
    lints: table(manifest.lints)?.workspace === true ? (workspace?.lints ?? null) : null,
    dependencies: dependencies.filter((row) => row.inherited).map((row) => [row.table, row.kind, row.alias, shared[row.alias] ?? null]),
  };
}

/** 🧊️ What every crate of one workspace shares and Cargo applies to all of them: the unstable feature gates, the resolver, the profiles and the patches. */
export function workspaceFixedV1(manifest: CargoTomlV1): Record<string, unknown> {
  return {features: manifest["cargo-features"] ?? null, resolver: table(manifest.workspace)?.resolver ?? null, profile: manifest.profile ?? null, patch: manifest.patch ?? null, replace: manifest.replace ?? null};
}

/** 📌️ The pinned toolchain without its comments and its ordering: the channel, profile, components and targets. */
export function toolchainPinV1(pin: CargoTomlV1 | null): unknown {
  if (!pin) return "missing";
  const row = table(pin.toolchain) ?? {}, sorted = (value: unknown) => (Array.isArray(value) ? value.map(String).sort(bytes) : value);
  return {...row, components: sorted(row.components), targets: sorted(row.targets)};
}

/** 🛠️ The Cargo configuration without its comments and the keys that only choose where artifacts are stored. */
export function cargoConfigV1(config: CargoTomlV1 | null, excluded: readonly string[]): unknown {
  if (!config) return "missing";
  const copy = structuredClone(config) as Record<string, any>;
  for (const key of excluded) {
    const parts = key.split(".");
    let node: any = copy;
    for (const part of parts.slice(0, -1)) node = node?.[part];
    if (node && typeof node === "object") delete node[parts[parts.length - 1]!];
  }
  return copy;
}

/** 🔑️ The state one resolution run shares: every file read or digested once, the owning workspace and the identity of every crate already folded, and the receipts read and changed, so a thousand crates of one graph read each manifest once and a warm run parses none. */
export type CargoResolutionContextV1 = {
  readonly host: CargoReadHostV1;
  readonly policy: CargoResolutionPolicyV1;
  readonly store?: CargoReceiptStoreV1;
  readonly files: Map<string, Promise<CargoFileV1 | null>>;
  readonly digests: Map<string, Promise<string | null>>;
  readonly leaves: Map<string, Promise<CargoLeafV1 | undefined>>;
  readonly dirty: Set<string>;
  readonly crates: Map<string, Promise<string>>;
  readonly counts: {parsed: number; digested: number; reused: number; derived: number};
  toolchain?: Promise<{toolchain: string; config: string}>;
};

/** 🆕️ Opens a resolution run over a host and, optionally, a receipt store. */
export function createCargoResolutionContextV1(host: CargoReadHostV1, policy: CargoResolutionPolicyV1, store?: CargoReceiptStoreV1): CargoResolutionContextV1 {
  const files = new Map<string, Promise<CargoFileV1 | null>>(), digests = new Map<string, Promise<string | null>>(), counts = {parsed: 0, digested: 0, reused: 0, derived: 0};
  const read = (path: string): Promise<CargoFileV1 | null> => {
    let known = files.get(path);
    if (!known) {
      counts.parsed++;
      files.set(path, (known = host.read(path)));
      digests.set(path, known.then((file) => file?.sha ?? null));
    }
    return known;
  };
  const digest = (path: string): Promise<string | null> => {
    let known = digests.get(path);
    if (!known) {
      counts.digested++;
      digests.set(path, (known = host.digest(path)));
    }
    return known;
  };
  return {host: {read, digest}, policy, ...(store ? {store} : {}), files, digests, leaves: new Map(), dirty: new Set(), crates: new Map(), counts};
}

const leafOf = (context: CargoResolutionContextV1, crate: string): Promise<CargoLeafV1 | undefined> => {
  let known = context.leaves.get(crate);
  if (!known) context.leaves.set(crate, (known = Promise.resolve(context.store?.read(crate))));
  return known;
};
const change = async (context: CargoResolutionContextV1, crate: string, update: (leaf: CargoLeafV1) => CargoLeafV1): Promise<void> => {
  const leaf = (await leafOf(context, crate)) ?? {version: 1 as const, crate};
  context.leaves.set(crate, Promise.resolve(update(leaf)));
  context.dirty.add(crate);
};
const holds = async (context: CargoResolutionContextV1, reads: readonly CargoReadV1[]): Promise<boolean> => {
  for (const read of reads) if ((await context.host.digest(read.path)) !== read.sha) return false;
  return true;
};
const fold = (own: unknown, fixed: unknown, children: readonly string[]): string => sha(canonicalJsonV1({own, fixed, children: [...children].sort(bytes)}));

/** 📝️ Writes the receipts this run created or changed through the store's writer, when it has one; a reader's changes stay in memory. */
export async function flushCargoReceiptsV1(context: CargoResolutionContextV1): Promise<number> {
  let written = 0;
  if (context.store?.write) for (const crate of [...context.dirty].sort(bytes)) { await context.store.write((await context.leaves.get(crate))!); written++; }
  context.dirty.clear();
  return written;
}

/** 🪪️ The identity of one crate folded over its path-dependency closure: what each crate inherits from its own workspace, hashed with the identities of the crates it compiles with, so a change reaches exactly the crates that compile with the changed one. Development dependencies count only at the root of a test run; a path cycle is refused. A receipt whose recorded files still have their recorded digests answers without parsing anything: only the identities of the path dependencies are asked again, so the identity moves exactly with a dependency's. */
export async function crateIdentityV1(context: CargoResolutionContextV1, crate: string, tests: boolean, stack: readonly string[] = []): Promise<string> {
  const flag = tests ? "t" : "b", key = `${flag}\0${crate}`;
  if (stack.includes(key)) throw new CargoResolutionErrorV1(`Cargo path dependency cycle at ${crate}`);
  const known = context.crates.get(key);
  if (known) return known;
  const run = (async () => {
    const stored = (await leafOf(context, crate))?.[flag];
    if (stored && (await holds(context, stored.reads))) {
      const children: {path: string; id: string}[] = [];
      for (const child of stored.children) children.push({path: child.path, id: await crateIdentityV1(context, child.path, false, [...stack, key])});
      const id = stored.own === null ? stored.id : fold(stored.own, stored.fixed, children.map((child) => child.id));
      context.counts.reused++;
      if (id !== stored.id) await change(context, crate, (leaf) => ({...leaf, [flag]: {...stored, id, children}}));
      return id;
    }
    context.counts.derived++;
    const located = await locateWorkspaceV1(context.host, crate), manifest = located.manifest, reads = located.reads;
    if (!manifest) {
      const id = sha(canonicalJsonV1({missing: crate}));
      await change(context, crate, (leaf) => ({...leaf, [flag]: {id, root: null, reads, own: null, fixed: null, children: []}}));
      return id;
    }
    const owner = located.workspace, rows = cargoDependenciesV1(crate, manifest.value, owner, tests), children: {path: string; id: string}[] = [];
    for (const path of new Set(rows.flatMap((row) => (row.path === undefined ? [] : [row.path])))) children.push({path, id: await crateIdentityV1(context, path, false, [...stack, key])});
    const own = inheritedSliceV1(manifest.value, table(owner?.manifest.workspace) ?? null, rows), fixed = owner ? workspaceFixedV1(owner.manifest) : null, id = fold(own, fixed, children.map((child) => child.id));
    await change(context, crate, (leaf) => ({...leaf, [flag]: {id, root: owner?.root ?? null, reads, own, fixed, children}}));
    return id;
  })();
  context.crates.set(key, run);
  return run;
}

/** 🔧️ The toolchain and configuration digests of the repository, read once per run. */
export function toolchainIdentityV1(context: CargoResolutionContextV1): Promise<{toolchain: string; config: string}> {
  return (context.toolchain ??= (async () => {
    const {pin, config, configExcluded} = context.policy.toolchain;
    return {toolchain: sha(canonicalJsonV1(toolchainPinV1((await context.host.read(pin))?.value ?? null))), config: sha(canonicalJsonV1(cargoConfigV1((await context.host.read(config))?.value ?? null, configExcluded)))};
  })());
}

/** 🍕️ The locked slice of one crate: the digest of what the lock file records for the crate and everything it compiles with, reused while the lock file and the files the crate's identity read keep their digests; a missing lock file digests as such and a lock that refuses the crate throws. */
export async function cargoLockSliceV1(context: CargoResolutionContextV1, crate: string): Promise<string> {
  await crateIdentityV1(context, crate, true);
  const leaf = await leafOf(context, crate), unit = leaf!.t!;
  if (unit.root === null || unit.own === null) throw new CargoResolutionErrorV1(unit.own === null ? "Selected Cargo package manifest required" : "Selected Cargo workspace owner required");
  const path = at(unit.root, "Cargo.lock"), basis = sha(canonicalJsonV1(unit.reads)), lockSha = await context.host.digest(path);
  if (leaf!.lock && leaf!.lock.path === path && leaf!.lock.sha === lockSha && leaf!.lock.basis === basis) return leaf!.lock.digest;
  const selected = (await context.host.read(at(crate, "Cargo.toml")))!.value, name = table(selected.package)?.name;
  if (typeof name !== "string" || !name) throw new CargoResolutionErrorV1("Selected Cargo package manifest required");
  let digest: string;
  if (lockSha === null) digest = sha("missing\0Cargo.lock");
  else {
    const workspace = (await context.host.read(at(unit.root, "Cargo.toml")))!.value, version = table(table(selected.package)?.version)?.workspace ? table(table(workspace.workspace)?.package)?.version : table(selected.package)?.version;
    digest = sha(JSON.stringify(cargoLockClosure((await context.host.read(path))!.value as never, [{name, version: version as string | undefined}])));
  }
  await change(context, crate, (current) => ({...current, lock: {path, sha: lockSha, basis, digest}}));
  return digest;
}

/** 🧮️ Folds the locked resolution, the workspace identity and the toolchain digests into the one digest a task key carries; nx and Bazel call this one function. */
export function composeCargoResolutionV1(parts: CargoResolutionPartsV1): string {
  return sha(canonicalJsonV1({version: 1, ...parts}));
}

/** 〽️ The locked, workspace, toolchain and configuration parts of a crate; with `tolerant`, a crate whose lock refuses it keeps the refusal in its lock part instead of failing, because the action itself fails under `--locked`. Reads are strictly sequential: the live Source batch serializes descriptor operations. */
export async function cargoResolutionPartsV1(context: CargoResolutionContextV1, manifest: string, options: {readonly tolerant?: boolean} = {}): Promise<CargoResolutionPartsV1> {
  if (!manifest.endsWith("Cargo.toml")) throw new CargoResolutionErrorV1(`Cargo manifest path required: ${manifest}`);
  const crate = manifest === "Cargo.toml" ? "" : manifest.slice(0, -"/Cargo.toml".length), workspace = await crateIdentityV1(context, crate, true);
  let lock: string;
  try { lock = await cargoLockSliceV1(context, crate); } catch (error) { if (!options.tolerant) throw error; lock = "unresolved:" + (error instanceof Error ? error.message : String(error)); }
  return {lock, workspace, ...(await toolchainIdentityV1(context))};
}

const normalizeKey = (pattern: string): string => pattern.replace(/^!?\{(?:workspaceRoot|projectRoot)\}\//u, "").replace(/^(?:\.\/)+/u, "");
const expandBraces = (pattern: string): string[] => {
  const open = pattern.indexOf("{");
  if (open < 0) return [pattern];
  const close = pattern.indexOf("}", open);
  if (close < 0) return [pattern];
  return pattern.slice(open + 1, close).split(",").flatMap((choice) => expandBraces(pattern.slice(0, open) + choice + pattern.slice(close + 1)));
};

/** 🗝️ Whether one declared input is a whole-workspace Rust or schema key: a root manifest, lock or toolchain file named literally, or a pattern whose directory part is the whole workspace and whose tail names Cargo manifests, locks, Rust sources or schema trees. */
export function workspaceKeyViolationV1(policy: Pick<CargoResolutionPolicyV1, "workspaceKeys">, input: string): string | undefined {
  if (input.startsWith("!") || !input.startsWith("{workspaceRoot}/")) return undefined;
  const bare = normalizeKey(input);
  if (policy.workspaceKeys.literals.includes(bare)) return bare;
  for (const alternative of expandBraces(bare)) for (const tail of policy.workspaceKeys.tails) if (alternative === `**/${tail}`) return `**/${tail}`;
  return undefined;
}

/** ⌨️ Every whole-workspace Rust or schema key among the declared inputs of one target (`project:target`), unless a recorded reason exempts its command or the target itself. */
export function workspaceKeyViolationsV1(policy: CargoResolutionPolicyV1, inputs: readonly unknown[], command?: string, id?: string): string[] {
  if (command !== undefined && policy.workspaceKeys.exempt.some((row) => command.includes(row.command))) return [];
  if (id !== undefined && policy.workspaceKeys.exemptTargets.some((row) => row.targets.includes(id))) return [];
  return [...new Set(inputs.flatMap((input) => (typeof input === "string" ? [workspaceKeyViolationV1(policy, input)] : [])).filter((hit): hit is string => hit !== undefined))].sort(bytes);
}
