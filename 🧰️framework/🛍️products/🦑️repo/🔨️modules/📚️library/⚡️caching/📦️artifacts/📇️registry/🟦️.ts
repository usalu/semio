import { lstat, readdir } from "node:fs/promises";
import { join, resolve, sep } from "node:path";

export type ArtifactDeclaration = { owner: string; path: string; cacheability?: boolean; consumers?: readonly string[] };
export type ArtifactCategory = "deliverable" | "task-results" | "compiler-state" | "dependency-store" | "downloaded-asset" | "report" | "service-state" | "unclassified";
export type ArtifactEntry = {
  owner: string; path: string; category: ArtifactCategory; producer: string | null; consumers: string[]; cacheability: boolean;
  portability: "host" | "unknown"; retention: { strategy: "replace-on-success" | "tool-managed" | "explicit-maintenance" | "unassigned"; budgetGroup: string | null };
  cleanup: "nx" | "native-tool" | "disabled"; activeUseLock: { resource: string; coverage: "pending" | "complete" }; namespace: boolean;
};
export type ArtifactFinding = { rule: "CACHE-03" | "CACHE-04" | "DISK-01"; owner: string; path: string; evidence: string };
export type ArtifactRegistry = { version: 1; entries: ArtifactEntry[]; findings: ArtifactFinding[] };
export type ArtifactBytes = { apparent: number; allocated: number | null; files: number; hardlinks: number; symlinks: number };
export type ArtifactMeasurement = {
  complete: boolean; totals: ArtifactBytes;
  entries: (ArtifactEntry & ArtifactBytes & { present: boolean })[];
  errors: { path: string; message: string }[];
};
export const artifactBudgets = { "task-results": 8 * 1024 ** 3, "compiler-state": 20 * 1024 ** 3, deliverable: 5 * 1024 ** 3, report: 2 * 1024 ** 3 };
const cache = ".🧬semio/🦑️repo/⚡️cache";
const stores: readonly [string, string, ArtifactCategory][] = [
  ["nx", `${cache}/nx`, "task-results"], ["nx-graph", ".nx/workspace-data", "service-state"],
  ["cargo", "target", "compiler-state"], ["bun", "node_modules", "dependency-store"], ["uv", ".venv", "dependency-store"],
  ["repo-cache", cache, "unclassified"], ["cargo-target", `${cache}/cargo/target`, "compiler-state"], ["cargo-build", `${cache}/cargo/build`, "compiler-state"],
  ["cmake", `${cache}/cmake`, "compiler-state"], ["dotnet", `${cache}/dotnet`, "compiler-state"],
  ["tools", `${cache}/tools`, "downloaded-asset"], ["tectonic", `${cache}/tectonic`, "compiler-state"],
  ["test-oracles", `${cache}/oracles`, "dependency-store"], ["tests", `${cache}/tests`, "report"],
  ["agents", `${cache}/agents`, "service-state"], ["breaches", `${cache}/breaches`, "report"],
  ["vite", `${cache}/vite`, "compiler-state"],
];
const contains = (parent: string, child: string): boolean => child === parent || child.startsWith(parent + "/");
const portablePath = (path: string): string => path.normalize("NFC").toLowerCase();
const validPath = (path: string): boolean => Boolean(path) && !/[\\:*?{}\[\]\0]/.test(path) && path.split("/").every(part => Boolean(part) && part !== "." && part !== "..");
const mutableOutput = (path: string): boolean => {
  const candidate = portablePath(path);
  return candidate.split("/").some(part => ["node_modules", "target", ".venv", ".nx", ".git"].includes(part)) || stores.some(([, store, category]) => contains(candidate, portablePath(store)) || ["compiler-state", "dependency-store", "task-results", "service-state"].includes(category) && contains(portablePath(store), candidate));
};
const order = (a: { path: string; owner: string }, b: { path: string; owner: string }): number => a.path < b.path ? -1 : a.path > b.path ? 1 : a.owner < b.owner ? -1 : a.owner > b.owner ? 1 : 0;
const bytes = (): ArtifactBytes => ({ apparent: 0, allocated: 0, files: 0, hardlinks: 0, symlinks: 0 });

/** 🧾️ Derives deliverable contracts and keeps mutable stores outside task-result ownership. */
export function createArtifactRegistry(declarations: readonly ArtifactDeclaration[]): ArtifactRegistry {
  const findings: ArtifactFinding[] = [], entries = new Map<string, ArtifactEntry>();
  for (const declaration of [...declarations].sort(order)) {
    const { owner, path } = declaration;
    if (!owner || !validPath(path) || mutableOutput(path)) {
      findings.push({ rule: "CACHE-04", owner, path, evidence: "Output must be a canonical workspace-relative deliverable path outside mutable stores" });
      continue;
    }
    const key = JSON.stringify([owner, path]), previous = entries.get(key);
    if (previous && previous.cacheability !== Boolean(declaration.cacheability)) findings.push({ rule: "CACHE-03", owner, path, evidence: "One output has conflicting cache policies" });
    entries.set(key, {
      owner, path, category: "deliverable", producer: owner, consumers: [...new Set([...(previous?.consumers ?? []), ...(declaration.consumers ?? [])])].sort(),
      cacheability: Boolean(declaration.cacheability), portability: "unknown", retention: { strategy: "replace-on-success", budgetGroup: "deliverable" },
      cleanup: "disabled", activeUseLock: { resource: `artifact:${path}`, coverage: "pending" }, namespace: false,
    });
  }
  const deliverables = [...entries.values()];
  const paths = deliverables.map((entry, index) => ({ index, parts: portablePath(entry.path).split("/") })).sort((a, b) => a.parts.length - b.parts.length || a.index - b.index);
  const ownersByPath = new Map<string, number[]>(), collisions: [number, number][] = [];
  for (const { index, parts } of paths) {
    let path = "";
    for (const part of parts) {
      path = path ? `${path}/${part}` : part;
      for (const other of ownersByPath.get(path) ?? []) collisions.push(index < other ? [index, other] : [other, index]);
    }
    if (!ownersByPath.has(path)) ownersByPath.set(path, []);
    ownersByPath.get(path)!.push(index);
  }
  for (const [first, second] of collisions.sort((a, b) => a[0] - b[0] || a[1] - b[1])) {
    const entry = deliverables[first], other = deliverables[second];
    findings.push({ rule: "CACHE-03", owner: other.owner, path: other.path, evidence: `Output overlaps ${entry.owner} at ${entry.path}` });
  }
  for (const [owner, path, category] of stores) entries.set(JSON.stringify([owner, path]), {
    owner, path, category, producer: null, consumers: [], cacheability: false, portability: "host",
    retention: { strategy: category === "task-results" || category === "dependency-store" ? "tool-managed" : category === "unclassified" ? "unassigned" : "explicit-maintenance", budgetGroup: Object.hasOwn(artifactBudgets, category) ? category : null },
    cleanup: category === "task-results" ? "nx" : category === "dependency-store" ? "native-tool" : "disabled",
    activeUseLock: { resource: `store:${path}`, coverage: "pending" }, namespace: true,
  });
  return { version: 1, entries: [...entries.values()].sort(order), findings };
}

/** 📏️ Accounts each retained inode once across owners without following links or concealing incomplete reads. */
export async function measureArtifactRegistry(workspace: string, registry: ArtifactRegistry, options: { signal: AbortSignal; onProgress?: (progress: { files: number; path: string }) => void }): Promise<ArtifactMeasurement> {
  options.signal.throwIfAborted();
  const root = resolve(workspace), entries = registry.entries.map(entry => ({ ...entry, ...bytes(), present: false })), errors: ArtifactMeasurement["errors"] = [], seen = new Set<string>();
  const owners = new Map<string, typeof entries[number]>();
  for (const entry of entries) {
    if (!validPath(entry.path)) throw new Error(`Invalid registry path: ${entry.path}`);
    if (!owners.has(entry.path)) owners.set(entry.path, entry);
  }
  const totals = bytes();
  let nextProgress = 0;
  const progress = (path: string, final = false): void => {
    options.signal.throwIfAborted();
    if (final || Date.now() >= nextProgress) { options.onProgress?.({ files: totals.files, path }); nextProgress = Date.now() + 1000; }
  };
  const errorAt = (path: string, error: unknown): void => { errors.push({ path, message: error instanceof Error ? error.message : String(error) }); };
  const add = (owner: typeof entries[number], field: Exclude<keyof ArtifactBytes, "allocated">, count: number): void => { owner[field] += count; totals[field] += count; };
  const walk = async (path: string, inherited: typeof entries[number], initial: boolean): Promise<void> => {
    progress(path);
    const owner = owners.get(path) ?? inherited;
    let stat;
    try { stat = await lstat(join(root, path), { bigint: true }); }
    catch (error) { if (!initial || (error as NodeJS.ErrnoException).code !== "ENOENT") errorAt(path, error); return; }
    owner.present = true;
    if (stat.isSymbolicLink()) {
      add(owner, "symlinks", 1);
      if (owners.has(path)) errorAt(path, new Error("Declared artifact root is a symbolic link"));
      return;
    }
    if (stat.isDirectory()) {
      let children: string[];
      try { children = await readdir(join(root, path)); } catch (error) { errorAt(path, error); return; }
      for (const child of children.sort()) await walk(path + "/" + child, owner, false);
    } else if (stat.isFile()) {
      const identity = `${stat.dev}:${stat.ino}`;
      if (seen.has(identity)) { add(owner, "hardlinks", 1); return; }
      seen.add(identity);
      add(owner, "apparent", Number(stat.size)); add(owner, "files", 1);
      const allocated = process.platform === "win32" || stat.blocks === undefined ? null : Number(stat.blocks) * 512;
      owner.allocated = owner.allocated === null || allocated === null ? null : owner.allocated + allocated;
      totals.allocated = totals.allocated === null || allocated === null ? null : totals.allocated + allocated;
    }
  };
  for (const path of [...owners.keys()].sort()) {
    if ([...owners.keys()].some(parent => parent !== path && contains(parent, path))) continue;
    let safe = true, current = root;
    for (const part of path.split("/").slice(0, -1)) {
      current = join(current, part);
      try { const stat = await lstat(current); if (stat.isSymbolicLink() || !stat.isDirectory()) throw new Error(`Unsafe registry ancestor: ${current.slice(root.length + sep.length)}`); }
      catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") break; errorAt(path, error); safe = false; break; }
    }
    if (safe) await walk(path, owners.get(path)!, true);
  }
  progress("", true);
  return { complete: errors.length === 0 && registry.findings.length === 0, totals, entries, errors };
}
