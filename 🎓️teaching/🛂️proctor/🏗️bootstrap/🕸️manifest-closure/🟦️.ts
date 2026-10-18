/** 🕸️ Which crates of this repository a Cargo package is built from, read from the manifests with the repository's own
 * Cargo TOML parser — the launcher asks no `cargo metadata` for it, which the repository's Cargo ingress refuses outside
 * its own launchers.
 *
 * The answer is the one `cargo metadata` gives for the workspace: every member resolves with its default features, the
 * features any dependency edge requests (a member's dev-dependencies included) unify across the whole graph, an optional
 * dependency joins when a feature of its package enables it (`dep:name`, an implicit `name`, `name/feature`, and
 * `name?/feature` only once `name` is enabled), every platform's tables count, and a package is reached when an enabled
 * normal or build edge leads to it. Only dependencies with a `path` can be crates of this repository; registry and git
 * dependencies cannot lead back to one.
 *
 * @see ../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📖️toml/🟦️.ts — the parser
 * @see ../🟦️.ts — `proctorSourceDirectories`, its caller
 * @see https://doc.rust-lang.org/cargo/reference/features.html
 * @see https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table */
import { readFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import type { CargoControl } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🎛️control/🟦️.ts";
import { CargoTomlWorkspace, parseCargoToml, type CargoTomlTable, type CargoTomlValue } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📖️toml/🟦️.ts";

/** 📦️ One crate of this repository the closure reached. */
export interface CargoCrate {
  readonly name: string;
  readonly manifest: string;
}

/** 🎛️ How a closure run may be stopped and followed. */
export interface ManifestClosureOptions {
  readonly signal?: AbortSignal;
  readonly onManifest?: (manifest: string) => void;
}

type DependencyKind = "normal" | "build" | "dev";

interface Edge {
  readonly key: string;
  readonly kind: DependencyKind;
  readonly optional: boolean;
  readonly defaultFeatures: boolean;
  readonly features: readonly string[];
  readonly target: Crate;
}

class Crate {
  name = "";
  features = new Map<string, readonly string[]>();
  enabledByDep = new Set<string>();
  edges: Edge[] = [];
  loaded = false;
  constructor(readonly manifest: string, readonly member: boolean) {}
}

const TABLES = [["dependencies", "normal"], ["build-dependencies", "build"], ["dev-dependencies", "dev"]] as const;
const MANIFEST_TIMEOUT_MS = 60_000;

const asTable = (value: CargoTomlValue | undefined): CargoTomlTable | undefined => (value !== null && typeof value === "object" && !Array.isArray(value) ? value : undefined);
const asText = (value: CargoTomlValue | undefined): string | undefined => (typeof value === "string" ? value : undefined);
const asTexts = (value: CargoTomlValue | undefined): string[] => (Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : []);

/** 📖️ Reads and parses manifests once, and finds the workspace root a package inherits its dependencies from. */
class Manifests {
  private readonly documents = new Map<string, Promise<CargoTomlTable>>();
  private readonly control: CargoControl;

  constructor(private readonly options: ManifestClosureOptions) {
    const deadline = Date.now() + MANIFEST_TIMEOUT_MS;
    this.control = {
      signal: options.signal ?? new AbortController().signal,
      maximumUnits: 50_000_000,
      maximumOwnedBytes: 1 << 30,
      maximumDepth: 64,
      remainingMilliseconds: () => Math.max(1, deadline - Date.now()),
      onProgress: () => undefined,
      yieldContinuation: () => new Promise<void>((accept) => setImmediate(accept)),
    };
  }

  read(manifest: string): Promise<CargoTomlTable> {
    const known = this.documents.get(manifest);
    if (known !== undefined) return known;
    const parsed = (async () => {
      this.options.signal?.throwIfAborted();
      const text = await readFile(manifest, "utf8");
      this.options.onManifest?.(manifest);
      return parseCargoToml(text, new CargoTomlWorkspace(), this.control, manifest);
    })();
    this.documents.set(manifest, parsed);
    return parsed;
  }

  async root(manifest: string, document: CargoTomlTable): Promise<string> {
    if (document.workspace !== undefined) return manifest;
    const named = asText(asTable(document.package)?.workspace);
    if (named !== undefined) return resolve(dirname(manifest), named, "Cargo.toml");
    for (let directory = dirname(dirname(manifest)); ; directory = dirname(directory)) {
      const candidate = join(directory, "Cargo.toml");
      const found = await this.read(candidate).catch(() => undefined);
      if (found?.workspace !== undefined) return candidate;
      if (dirname(directory) === directory) throw new Error(`${manifest} inherits a workspace dependency but no workspace root holds it`);
    }
  }
}

/** 🧭️ The dependency tables of a manifest, the platform tables included. */
function dependencyTables(document: CargoTomlTable): [DependencyKind, CargoTomlTable][] {
  const scopes = [document, ...Object.values(asTable(document.target) ?? {}).flatMap((scope) => asTable(scope) ?? [])];
  return scopes.flatMap((scope) => TABLES.flatMap(([name, kind]) => {
    const table = asTable(scope[name]);
    return table === undefined ? [] : [[kind, table] as [DependencyKind, CargoTomlTable]];
  }));
}

/** 🕸️ Loads every crate reachable by a path dependency from the workspace members, dev-dependencies of members only. */
async function loadCrates(manifests: Manifests, members: readonly string[]): Promise<Map<string, Crate>> {
  const crates = new Map(members.map((manifest) => [manifest, new Crate(manifest, true)] as const));
  const inherited = new Map<string, Promise<CargoTomlTable>>();
  const dependencies = (root: string): Promise<CargoTomlTable> => {
    if (!inherited.has(root)) inherited.set(root, manifests.read(root).then((document) => asTable(asTable(document.workspace)?.dependencies) ?? (Object.create(null) as CargoTomlTable)));
    return inherited.get(root)!;
  };
  const load = async (crate: Crate): Promise<void> => {
    if (crate.loaded) return;
    crate.loaded = true;
    const document = await manifests.read(crate.manifest);
    crate.name = asText(asTable(document.package)?.name) ?? crate.manifest;
    for (const [name, listed] of Object.entries(asTable(document.features) ?? {})) {
      const items = asTexts(listed);
      crate.features.set(name, items);
      for (const item of items) if (item.startsWith("dep:")) crate.enabledByDep.add(item.slice(4).split("/")[0]!);
    }
    let root: string | undefined;
    const rootOf = async (): Promise<string> => (root ??= await manifests.root(crate.manifest, document));
    const pending: Crate[] = [];
    for (const [kind, table] of dependencyTables(document)) {
      if (kind === "dev" && !crate.member) continue;
      for (const [key, raw] of Object.entries(table)) {
        const spec = asTable(raw);
        if (spec === undefined) continue;
        const shared = spec.workspace === true ? asTable((await dependencies(await rootOf()))[key]) : undefined;
        const path = asText(spec.path) ?? asText(shared?.path);
        if (path === undefined) continue;
        const target = resolve(asText(spec.path) === undefined ? dirname(await rootOf()) : dirname(crate.manifest), path, "Cargo.toml");
        const known = crates.get(target) ?? new Crate(target, false);
        crates.set(target, known);
        crate.edges.push({
          key,
          kind,
          optional: spec.optional === true,
          defaultFeatures: spec["default-features"] !== false && shared?.["default-features"] !== false,
          features: [...asTexts(shared?.features), ...asTexts(spec.features)],
          target: known,
        });
        pending.push(known);
      }
    }
    for (const next of pending) await load(next);
  };
  for (const crate of [...crates.values()]) await load(crate);
  return crates;
}

/** 🧮️ The features and optional dependencies enabled once every request of the graph is unified. */
function unify(members: readonly Crate[]): (crate: Crate, edge: Edge) => boolean {
  const features = new Map<Crate, Set<string>>();
  const activated = new Map<Crate, Set<string>>();
  const weak = new Map<Crate, Map<string, Set<string>>>();
  const present = new Set<Crate>();
  const setOf = (map: Map<Crate, Set<string>>, crate: Crate): Set<string> => map.get(crate) ?? map.set(crate, new Set()).get(crate)!;
  const live = (crate: Crate, edge: Edge): boolean => !edge.optional || setOf(activated, crate).has(edge.key);
  const include = (crate: Crate): void => {
    if (present.has(crate)) return;
    present.add(crate);
    for (const edge of crate.edges) if (live(crate, edge)) follow(edge);
  };
  const follow = (edge: Edge): void => {
    include(edge.target);
    if (edge.defaultFeatures) enable(edge.target, "default");
    for (const name of edge.features) enable(edge.target, name);
  };
  const request = (crate: Crate, key: string, name: string): void => {
    for (const edge of crate.edges) if (edge.key === key && live(crate, edge)) enable(edge.target, name);
  };
  const activate = (crate: Crate, key: string): void => {
    const on = setOf(activated, crate);
    if (on.has(key)) return;
    on.add(key);
    for (const edge of crate.edges) if (edge.key === key && edge.optional) follow(edge);
    for (const name of weak.get(crate)?.get(key) ?? []) request(crate, key, name);
  };
  const apply = (crate: Crate, item: string): void => {
    if (item.startsWith("dep:")) return activate(crate, item.slice(4));
    const slash = item.indexOf("/");
    if (slash < 0) return enable(crate, item);
    const left = item.slice(0, slash);
    const key = left.endsWith("?") ? left.slice(0, -1) : left;
    const name = item.slice(slash + 1);
    if (left === key) activate(crate, key);
    else if (!crate.edges.some((edge) => edge.key === key && live(crate, edge))) {
      const pending = weak.get(crate) ?? weak.set(crate, new Map()).get(crate)!;
      pending.set(key, (pending.get(key) ?? new Set()).add(name));
      return;
    }
    request(crate, key, name);
  };
  const enable = (crate: Crate, name: string): void => {
    const on = setOf(features, crate);
    if (on.has(name)) return;
    const listed = crate.features.get(name);
    if (listed === undefined && name !== "default" && crate.edges.some((edge) => edge.key === name && edge.optional) && !crate.enabledByDep.has(name)) {
      on.add(name);
      activate(crate, name);
    } else if (listed !== undefined) {
      on.add(name);
      for (const item of listed) apply(crate, item);
    }
  };
  for (const member of members) {
    include(member);
    enable(member, "default");
  }
  return live;
}

/** 🕸️ The crates of this repository the package `packageName` of the workspace at `workspaceManifest` is built from, itself included. */
export async function cargoPackageClosure(workspaceManifest: string, packageName: string, options: ManifestClosureOptions = {}): Promise<readonly CargoCrate[]> {
  const manifests = new Manifests(options);
  const document = await manifests.read(workspaceManifest);
  const members = asTexts(asTable(document.workspace)?.members).map((member) => {
    if (/[*?[]/u.test(member)) throw new Error(`${workspaceManifest} lists the workspace member glob ${JSON.stringify(member)}, which the manifest reader does not expand`);
    return resolve(dirname(workspaceManifest), member, "Cargo.toml");
  });
  const crates = await loadCrates(manifests, members);
  const live = unify(members.map((member) => crates.get(member)!));
  const start = members.map((member) => crates.get(member)!).find((crate) => crate.name === packageName);
  if (start === undefined) throw new Error(`${workspaceManifest} has no member package ${packageName}`);
  const reached = new Set<Crate>();
  for (const pending = [start]; pending.length > 0; ) {
    const crate = pending.pop()!;
    if (reached.has(crate)) continue;
    reached.add(crate);
    for (const edge of crate.edges) if (edge.kind !== "dev" && live(crate, edge)) pending.push(edge.target);
  }
  return [...reached].map((crate) => ({ name: crate.name, manifest: crate.manifest }));
}
