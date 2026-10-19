import { spawnSync } from "node:child_process";
import { existsSync, lstatSync, readdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import cachingPolicy from "../../../⚡️caching/🔣️policy.json" with { type: "json" };
import ownershipPolicy from "../../../🗂️workspaces/🔣️policy.json" with { type: "json" };
import { readBuildOwnerRegistryV1, type BuildOwnerRegistryV1 } from "../🚪️admission/🟦️.ts";

/** 📋️ One row of the build owner registry. */
export type OwnerRow = BuildOwnerRegistryV1["owners"][number];

/** 🚫️ The example-collection and generated-directory names the owner discovery excludes, as read from the workspace and caching policies. */
export type OwnerTreePolicy = Readonly<{ collections: readonly string[]; moduleMember: string; generated: readonly string[] }>;

/** 🧾️ The finding kinds of `findOwnerFindings`, mirrored by the `Finding` pattern of `🧬️schema/🧾️owner-consistency.json`. */
export type OwnerFindingKind = "registry-invalid" | "missing-root" | "missing-manifest" | "missing-native-root" | "cargo-row-shadows-project" | "row-in-excluded-tree" | "manifest-not-file" | "name-mismatch" | "package-missing" | "package-stale" | "provider-owner-root" | "provider-source" | "provider-consumer-root" | "project-without-row" | "cargo-without-row";

/** 🔎️ One inconsistency between the registry and the workspace tree. */
export type OwnerFinding = Readonly<{ kind: OwnerFindingKind; detail: string; owner?: string; path?: string }>;

/** 🌲️ The workspace as the consistency law sees it: manifest candidates, file reads, existence and ignore state. */
export interface OwnerTree {
  readonly manifests: readonly string[];
  exists(path: string): boolean;
  file(path: string): boolean;
  tracked(path: string): boolean;
  read(path: string): string | undefined;
  ignored(path: string): boolean;
}

/** 🎯️ One path prefix or several path prefixes that restrict a check or sync to subtrees. */
export type OwnerScope = string | readonly string[];

/** 🔧️ What `syncOwnerRegistry` changed. */
export type OwnerSync = Readonly<{
  registry: BuildOwnerRegistryV1;
  removed: { name: string; root: string; nativeRoots: string[] }[];
  added: string[];
  renamed: { from: string; to: string }[];
  relocated: { name: string; from: string; to: string }[];
  repackaged: string[];
}>;

export const BUILD_OWNER_REGISTRY_FILE = "📋️build-owners.json";
export const OWNER_TREE_POLICY: OwnerTreePolicy = { collections: ownershipPolicy.collections.names, moduleMember: ownershipPolicy.collections.moduleMember, generated: cachingPolicy.generatedDirectories };

const PROJECT = "📋️project.json";
const CARGO = "Cargo.toml";
const at = (root: string, name: string): string => (root === "." ? name : `${root}/${name}`);
const compareBytes = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));
const scopesOf = (scope?: OwnerScope): readonly string[] | undefined => (scope === undefined ? undefined : typeof scope === "string" ? [scope] : scope);
const within = (path: string, scope?: OwnerScope): boolean => scopesOf(scope)?.some((prefix) => prefix === "." || path === prefix || path.startsWith(`${prefix}/`)) ?? true;

/** 🚫️ Mirrors the discovery exclusions: the ticket tree, generated directories and example collections outside a module member. */
export function excludedFromOwners(path: string, policy: OwnerTreePolicy): boolean {
  const parts = path.split("/");
  return path.startsWith(".🧬semio/") || parts.some((part, index) => policy.generated.includes(part) || (policy.collections.includes(part) && parts[index - 1] !== policy.moduleMember));
}

/** 🦀️ Reads `[package] name` of a Cargo manifest, or undefined for virtual manifests. */
export function cargoPackageName(text: string): string | undefined {
  const section = /^\[package\]\s*$([\s\S]*?)(?=^\[|(?![\s\S]))/m.exec(text);
  return section ? /^name\s*=\s*"([^"]+)"/m.exec(section[1] ?? "")?.[1] : undefined;
}

/** 🪪️ Reads the name a manifest declares, or undefined when it is absent, unreadable or virtual. */
export function ownerManifestName(tree: OwnerTree, manifest: string): string | undefined {
  const text = tree.read(manifest);
  if (text === undefined) return undefined;
  if (manifest.endsWith(CARGO)) return cargoPackageName(text);
  try { return (JSON.parse(text) as { name?: unknown }).name as string | undefined; } catch { return undefined; }
}

/** 📦️ Reads the npm package name declared beside an owner root. */
export function ownerPackageName(tree: OwnerTree, root: string): string | undefined {
  const text = tree.read(at(root, "package.json"));
  if (text === undefined) return undefined;
  try { return (JSON.parse(text) as { name?: string }).name; } catch { return undefined; }
}

/** ✅️ Returns every inconsistency between the registry rows and the tree below `scope`; a scope skips the catalog providers. */
export function findOwnerFindings(registry: BuildOwnerRegistryV1, tree: OwnerTree, policy: OwnerTreePolicy, scope?: OwnerScope): OwnerFinding[] {
  const out: OwnerFinding[] = [];
  const add = (kind: OwnerFindingKind, detail: string, owner?: string, path?: string): void => { out.push({ kind, detail, owner, path }); };
  const covered = new Set(registry.owners.flatMap((owner) => [owner.root, ...(owner.nativeRoots ?? [])]));
  for (const owner of registry.owners.filter((row) => within(row.root, scope))) {
    for (const native of owner.nativeRoots ?? []) if (!tree.exists(at(native, CARGO)) && !tree.ignored(native)) add("missing-native-root", `${native}/${CARGO} absent`, owner.name, native);
    if (!tree.exists(owner.root) || !tree.exists(owner.manifest)) {
      if (!tree.ignored(owner.manifest)) add(tree.exists(owner.root) ? "missing-manifest" : "missing-root", tree.exists(owner.root) ? "manifest absent" : "root directory absent", owner.name, tree.exists(owner.root) ? owner.manifest : owner.root);
      continue;
    }
    if (!tree.file(owner.manifest)) add("manifest-not-file", "manifest is not an ordinary file", owner.name, owner.manifest);
    if (owner.manifest.endsWith(CARGO) && tree.exists(at(owner.root, PROJECT))) add("cargo-row-shadows-project", `${PROJECT} beside the Cargo row is ignored by admission`, owner.name, at(owner.root, PROJECT));
    if (excludedFromOwners(owner.manifest, policy)) add("row-in-excluded-tree", "manifest sits in an excluded tree", owner.name, owner.manifest);
    const actual = ownerManifestName(tree, owner.manifest);
    if (actual !== owner.name) add("name-mismatch", `manifest declares ${actual ?? "(unreadable)"}`, owner.name, owner.manifest);
    const declared = ownerPackageName(tree, owner.root);
    if (declared && !owner.packages.includes(declared)) add("package-missing", `package.json declares ${declared}`, owner.name, at(owner.root, "package.json"));
    for (const name of owner.packages) if (name !== declared) add("package-stale", `${name} not declared by package.json`, owner.name, at(owner.root, "package.json"));
  }
  if (scope === undefined) for (const provider of registry.catalogProviders) {
    if (!tree.exists(provider.ownerRoot)) add("provider-owner-root", "absent", undefined, provider.ownerRoot);
    if (!tree.exists(provider.sourcePath)) add("provider-source", "absent", undefined, provider.sourcePath);
    for (const consumer of provider.consumerRoots) if (!tree.exists(consumer)) add("provider-consumer-root", "absent", undefined, consumer);
  }
  const projectDirs = new Set(tree.manifests.filter((path) => basename(path) === PROJECT).map(dirname));
  for (const path of tree.manifests.filter((candidate) => within(candidate, scope) && !excludedFromOwners(candidate, policy))) {
    const directory = dirname(path);
    if (basename(path) === PROJECT) {
      if (!registry.owners.some((owner) => owner.root === directory && owner.manifest === path)) add("project-without-row", "project manifest has no owner row", undefined, path);
    } else if (!projectDirs.has(directory) && !covered.has(directory) && ownerManifestName(tree, path) !== undefined) add("cargo-without-row", "package manifest has no owner row or native root", undefined, path);
  }
  return out;
}

/** 🔧️ Brings the rows below `scope` in line with the tree: rows of vanished modules are dropped only when no manifest exists and git tracks no file at the missing path (a tracked path that is merely absent is a concurrent move in progress), the Cargo packages under their native roots become rows of their own, rows whose module moved follow it, rows take the name and package list their manifest declares, Cargo rows beside a project manifest hand over to it, and every unregistered module gets a row at its byte-order position. */
export function syncOwnerRegistry(registry: BuildOwnerRegistryV1, tree: OwnerTree, policy: OwnerTreePolicy, scope?: OwnerScope): OwnerSync {
  const findings = findOwnerFindings(registry, tree, policy, scope);
  const owners = structuredClone(registry.owners);
  const missing = findings.filter((finding) => finding.kind === "missing-root" || finding.kind === "missing-manifest");
  const gone = new Set(missing.map((finding) => finding.owner));
  const absent = new Set(missing.filter((finding) => !tree.tracked(finding.path!)).map((finding) => finding.owner));
  const found: { path: string; root: string; name: string }[] = findings.filter((finding) => finding.kind === "project-without-row" || finding.kind === "cargo-without-row").map((finding) => ({ path: finding.path!, root: dirname(finding.path!), name: ownerManifestName(tree, finding.path!) })).filter((entry): entry is { path: string; root: string; name: string } => entry.name !== undefined);
  const claimed = new Set<string>();
  const removed: OwnerSync["removed"] = [], relocated: OwnerSync["relocated"] = [], renamed: OwnerSync["renamed"] = [], repackaged: string[] = [], added: string[] = [];
  const place = (row: OwnerRow): void => {
    const index = owners.findIndex((other) => compareBytes(other.name, row.name) > 0);
    owners.splice(index < 0 ? owners.length : index, 0, row);
  };
  const taken = (name: string, except: OwnerRow): boolean => owners.some((other) => other !== except && other.name === name);
  for (const owner of [...owners]) {
    if (!gone.has(owner.name)) continue;
    const target = found.find((entry) => entry.name === owner.name && !claimed.has(entry.path));
    if (!target) {
      if (!absent.has(owner.name)) continue;
      owners.splice(owners.indexOf(owner), 1);
      removed.push({ name: owner.name, root: owner.root, nativeRoots: [...owner.nativeRoots ?? []] });
      for (const native of owner.nativeRoots ?? []) for (const manifest of [at(native, PROJECT), at(native, CARGO)]) {
        const name = tree.exists(manifest) ? ownerManifestName(tree, manifest) : undefined;
        if (name !== undefined) { found.push({ path: manifest, root: native, name }); break; }
      }
      continue;
    }
    claimed.add(target.path);
    const from = owner.root;
    owner.root = target.root;
    owner.manifest = target.path;
    if (owner.nativeRoots) owner.nativeRoots = owner.nativeRoots.map((native) => (native === from ? target.root : native.startsWith(`${from}/`) ? at(target.root, native.slice(from.length + 1)) : native));
    relocated.push({ name: owner.name, from, to: target.root });
  }
  for (const owner of [...owners].filter((row) => within(row.root, scope) && tree.exists(row.manifest))) {
    const sibling = owner.manifest.endsWith(CARGO) ? at(owner.root, PROJECT) : undefined;
    const handover = sibling !== undefined ? ownerManifestName(tree, sibling) : undefined;
    if (sibling !== undefined && handover !== undefined && !taken(handover, owner)) owner.manifest = sibling;
    const name = ownerManifestName(tree, owner.manifest);
    if (name !== undefined && name !== owner.name && !taken(name, owner)) {
      const from = owner.name;
      owners.splice(owners.indexOf(owner), 1);
      owner.name = name;
      place(owner);
      renamed.push({ from, to: name });
    }
    const declared = ownerPackageName(tree, owner.root);
    const packages = declared && !owners.some((other) => other !== owner && other.packages.includes(declared)) ? [declared] : declared ? owner.packages : [];
    if (JSON.stringify(packages) !== JSON.stringify(owner.packages)) { owner.packages = packages; repackaged.push(owner.name); }
  }
  for (const entry of found) {
    if (claimed.has(entry.path) || owners.some((owner) => owner.name === entry.name || owner.root === entry.root || owner.nativeRoots?.includes(entry.root))) continue;
    const declared = ownerPackageName(tree, entry.root);
    place({ name: entry.name, root: entry.root, manifest: entry.path, packages: declared && !owners.some((owner) => owner.packages.includes(declared)) ? [declared] : [] });
    added.push(entry.name);
  }
  return { registry: readBuildOwnerRegistryV1({ ...registry, owners }), removed, added, renamed, relocated, repackaged };
}

/** 🧠️ Builds an in-memory tree from a path-to-text map, a list of ignored path prefixes and a list of path prefixes that git tracks even when the file is absent; the language-agnostic fixtures run on it. */
export function memoryOwnerTree(files: Readonly<Record<string, string>>, policy: OwnerTreePolicy, ignored: readonly string[] = [], tracked: readonly string[] = []): OwnerTree {
  const paths = Object.keys(files);
  const under = (path: string, prefix: string): boolean => path === prefix || path.startsWith(`${prefix}/`);
  return {
    manifests: paths.filter((path) => [PROJECT, CARGO].includes(basename(path)) && !excludedFromOwners(path, policy)).sort(compareBytes),
    exists: (path) => path === "." || path in files || paths.some((candidate) => candidate.startsWith(`${path}/`)),
    file: (path) => path in files,
    tracked: (path) => tracked.some((prefix) => under(prefix, path) || under(path, prefix)),
    read: (path) => files[path],
    ignored: (path) => ignored.some((prefix) => under(path, prefix)),
  };
}

function listManifests(root: string, policy: OwnerTreePolicy, scope?: OwnerScope): string[] {
  const scopes = scopesOf(scope);
  const listed = spawnSync("git", ["ls-files", "-z", "-c", "-o", "--exclude-standard", ...(scopes && !scopes.includes(".") ? ["--", ...scopes.map((prefix) => `:(literal)${prefix}`)] : [])], { cwd: root, maxBuffer: 1 << 29 });
  const names = [PROJECT, CARGO];
  if (listed.status === 0) return listed.stdout.toString("utf8").split("\0").filter((path) => names.includes(basename(path)) && !excludedFromOwners(path, policy) && existsSync(join(root, path))).sort(compareBytes);
  const found: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(join(root, directory), { withFileTypes: true })) {
      const path = at(directory, entry.name);
      if (entry.isDirectory()) { if (!policy.generated.includes(entry.name) && !excludedFromOwners(`${path}/`, policy)) walk(path); } else if (names.includes(entry.name) && !excludedFromOwners(path, policy)) found.push(path);
    }
  };
  for (const prefix of scopes ?? ["."]) walk(prefix);
  return [...new Set(found)].sort(compareBytes);
}

/** 💽️ Reads the workspace below `root` from the file system; git lists the manifest candidates and decides ignore state, and a directory without git is walked instead. */
export function readOwnerTree(root: string, policy: OwnerTreePolicy = OWNER_TREE_POLICY, scope?: OwnerScope): OwnerTree {
  const ignoredCache = new Map<string, boolean>();
  const trackedCache = new Map<string, boolean>();
  return {
    manifests: listManifests(root, policy, scope),
    exists: (path) => existsSync(join(root, path)),
    file: (path) => { const facts = lstatSync(join(root, path), { throwIfNoEntry: false }); return facts !== undefined && facts.isFile(); },
    tracked: (path) => {
      if (!trackedCache.has(path)) { const listed = spawnSync("git", ["ls-files", "-z", "-c", "--", `:(literal)${path}`], { cwd: root, maxBuffer: 1 << 29 }); trackedCache.set(path, listed.status === 0 && listed.stdout.length > 0); }
      return trackedCache.get(path)!;
    },
    read: (path) => { try { return readFileSync(join(root, path), "utf8"); } catch { return undefined; } },
    ignored: (path) => {
      if (!ignoredCache.has(path)) ignoredCache.set(path, spawnSync("git", ["check-ignore", "-q", "--", path], { cwd: root }).status === 0);
      return ignoredCache.get(path)!;
    },
  };
}

/** ⚖️ Checks the registry file below `repoRoot` and returns its findings; an invalid registry is the one finding. */
export function checkBuildOwnerRegistry(repoRoot: string, scope?: OwnerScope, policy: OwnerTreePolicy = OWNER_TREE_POLICY): { owners: number; findings: OwnerFinding[] } {
  let registry: BuildOwnerRegistryV1;
  try { registry = readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(repoRoot, BUILD_OWNER_REGISTRY_FILE), "utf8"))); } catch (error) { return { owners: 0, findings: [{ kind: "registry-invalid", detail: (error as Error).message, path: BUILD_OWNER_REGISTRY_FILE }] }; }
  return { owners: registry.owners.length, findings: findOwnerFindings(registry, readOwnerTree(repoRoot, policy, scope), policy, scope) };
}

/** 🚚️ Names the subtrees an applied move plan can have changed owners in: the directories of every moved owner or package manifest on either side, collapsed to the outermost ones, or the whole workspace when more than `limit` remain. */
export function moveOwnerScopes(moves: readonly Readonly<{ sourcePath: string; destinationPath: string }>[], limit = 256): string[] {
  const names = [PROJECT, CARGO, "package.json"];
  const directories = [...new Set(moves.flatMap((move) => [move.sourcePath, move.destinationPath]).filter((path) => names.includes(basename(path))).map(dirname))].sort(compareBytes);
  const outermost = directories.filter((path, index) => !directories.slice(0, index).some((prefix) => prefix === "." || path.startsWith(`${prefix}/`)));
  return outermost.length > limit ? ["."] : outermost;
}

/** 🔄️ Re-reads the registry file at call time because peers edit it concurrently, syncs the rows below `scope`, replaces the file atomically when anything changed, and lists the findings that remain (catalog providers are authored, not synced; native roots follow their row). */
export function reconcileBuildOwnerRegistry(repoRoot: string, options: { scope?: OwnerScope; write: boolean }, policy: OwnerTreePolicy = OWNER_TREE_POLICY): OwnerSync & { changed: boolean; findings: OwnerFinding[] } {
  const file = join(repoRoot, BUILD_OWNER_REGISTRY_FILE);
  const registry = readBuildOwnerRegistryV1(JSON.parse(readFileSync(file, "utf8")));
  const tree = readOwnerTree(repoRoot, policy, options.scope);
  const result = syncOwnerRegistry(registry, tree, policy, options.scope);
  const text = `${JSON.stringify(result.registry, null, 2)}\n`;
  const changed = JSON.stringify(result.registry) !== JSON.stringify(registry);
  if (changed && options.write) {
    const pending = `${file}.${process.pid}.tmp`;
    writeFileSync(pending, text);
    renameSync(pending, file);
  }
  return { ...result, changed, findings: findOwnerFindings(result.registry, tree, policy, options.scope) };
}
