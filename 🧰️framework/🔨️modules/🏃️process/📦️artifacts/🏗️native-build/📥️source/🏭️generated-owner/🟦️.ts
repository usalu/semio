import { access, readFile } from "node:fs/promises";
import { dirname, relative, resolve, sep } from "node:path";
import type { ScriptProgress } from "../../../../🧭️routing/📥️invocation/🟦️.ts";
import { deriveWorkspaceIndexedV1, listWorkspaceIndexedV1, type WorkspaceChangeFeedV1, type WorkspaceIndexControlV1 } from "./🗂️index/🟦️.ts";
import suites from "../../../../🧪️testing/🗂️suites/🧬️schema/🔣️.json" with { type: "json" };

type ProjectTargetsV1 = Readonly<{ name?: string; targets?: Readonly<Record<string, Readonly<{ outputs?: readonly unknown[] }>>> }>;
type ProjectManifestV1 = Readonly<{ path: string; directory: string; name: string; targets: NonNullable<ProjectTargetsV1["targets"]> }>;
/** 🎛️ Cancellation and progress ports of the caller; the scan never blocks the event loop and checks the signal between directories and files. */
export type GeneratedScanControlV1 = Readonly<{ signal?: AbortSignal; report?: (stage: ScriptProgress["stage"]) => void | Promise<void>; changes?: WorkspaceChangeFeedV1; store?: string }>;

const PROJECT_MANIFEST_V1 = "📋️project.json";
/** 🚫️ Ambient trees the scan never enters: dependency, tool and build state (the same trees the input precision exclusions name, and the Bazel output trees the change journal never reports either), and the test scratch root, whose sandboxes every test process creates and deletes while the scan runs. @see ../../../../🧪️testing/🗂️suites/🧬️schema/🔣️.json */
const SKIPPED_DIRECTORIES_V1 = new Set(["node_modules", ".git", ".cache", "target", ".🧬semio", "🗑️generated", ".nx", ".venv", "__pycache__", ".pytest_cache", ".r", "bazel-out", "bazel-bin", "bazel-testlogs", "bazel-semio", suites.definitions.Protocol.const.treeScratchDirectory]);
const VANISHED_V1 = new Set(["ENOENT", "ENOTDIR"]);
const GENERATED_SEGMENT_V1 = "🤖️generated";
const ANCESTOR_LEVELS_V1 = 6, MANIFEST_DEPTH_V1 = 4, READ_CONCURRENCY_V1 = 64;
const MOUNT_PATTERN_V1 = /#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+[A-Za-z_][A-Za-z0-9_]*\s*;/gu;

const within = (base: string, path: string): boolean => path === base || path.startsWith(base.endsWith(sep) ? base : `${base}${sep}`);
const present = (path: string): Promise<boolean> => access(path).then(() => true, () => false);

/** 🧵️ Maps the items with at most `limit` reads in flight, so the scan overlaps its file system waits without exhausting descriptors. */
async function mapLimitV1<T, R>(items: readonly T[], limit: number, map: (item: T) => Promise<R>): Promise<R[]> {
  const results: R[] = new Array(items.length);
  let next = 0;
  await Promise.all(Array.from({ length: Math.min(limit, items.length) }, async () => {
    while (next < items.length) { const index = next++; results[index] = await map(items[index]!); }
  }));
  return results;
}

const QUERY_V1 = Object.freeze({ skipped: SKIPPED_DIRECTORIES_V1, suffixes: Object.freeze([".rs"]), names: Object.freeze([PROJECT_MANIFEST_V1]) });
const listingControlV1 = (control: GeneratedScanControlV1): WorkspaceIndexControlV1 => ({ signal: control.signal, report: () => control.report?.("loading"), changes: control.changes, store: control.store });

/** 🗂️ Lists the workspace's Rust sources and project manifests, skipping ignored and generated-output directories; the answer equals a fresh full walk, and a root this process walked before only re-lists the directories the change feed names, or else whose stamp changed. */
async function inventoryV1(root: string, control: GeneratedScanControlV1): Promise<Readonly<{ sources: string[]; manifests: string[] }>> {
  const listing = await listWorkspaceIndexedV1(root, QUERY_V1, listingControlV1(control));
  return { sources: [...listing.suffixed], manifests: [...listing.named] };
}

/** 📋️ Reads the declared project names and targets of the discovered manifests, remembered while a manifest's stamp is unchanged; unreadable manifests are skipped. */
async function projectsV1(root: string, manifests: readonly string[], control: GeneratedScanControlV1): Promise<ProjectManifestV1[]> {
  control.signal?.throwIfAborted();
  const read = await deriveWorkspaceIndexedV1(root, QUERY_V1, "manifest", manifests, async path => {
    try { return JSON.parse(await readFile(path, "utf8")) as ProjectTargetsV1; } catch { return undefined; }
  }, listingControlV1(control));
  return read.flatMap((value, index) => value?.name !== undefined && value.targets !== undefined ? [{ path: manifests[index]!, directory: dirname(manifests[index]!), name: value.name, targets: value.targets }] : []);
}

/** 🔎️ Lists the Rust mount targets under the sources that do not exist yet; the mounts of a source are remembered while its stamp is unchanged, a target the listing names exists, any other is checked live. */
async function missingMountsV1(root: string, sources: readonly string[], control: GeneratedScanControlV1): Promise<string[]> {
  control.signal?.throwIfAborted();
  const missing = new Set<string>();
  const targets = await deriveWorkspaceIndexedV1(root, QUERY_V1, "mounts", sources, async file => {
    const text = await readFile(file, "utf8").catch(error => { if (VANISHED_V1.has((error as NodeJS.ErrnoException).code ?? "")) return ""; throw error; });
    return [...text.matchAll(MOUNT_PATTERN_V1)].map(match => resolve(dirname(file), match[1]!));
  }, listingControlV1(control));
  const listed = new Set(sources);
  const candidates = [...new Set(targets.flatMap(row => row ?? []))].filter(target => !listed.has(target));
  const absent = await mapLimitV1(candidates, READ_CONCURRENCY_V1, async target => (await present(target)) ? undefined : target);
  for (const target of absent) if (target !== undefined) missing.add(target);
  return [...missing].sort();
}

const outputCoversV1 = (root: string, manifest: string, output: string, absolute: string): boolean =>
  within(resolve(root, output.replaceAll("{projectRoot}", dirname(manifest)).replaceAll("{workspaceRoot}", root)), absolute);

const ownersOfV1 = (root: string, missing: string, projects: readonly ProjectManifestV1[]): string[] => {
  const absolute = resolve(missing), generated = absolute.split(sep).includes(GENERATED_SEGMENT_V1);
  const owners = new Set<string>(), fallback = new Set<string>();
  let directory = dirname(absolute);
  for (let level = 0; level < ANCESTOR_LEVELS_V1 && within(root, directory) && directory !== root && owners.size === 0 && fallback.size === 0; level++, directory = dirname(directory)) {
    for (const project of projects) {
      if (!within(directory, project.directory) || relative(directory, project.directory).split(sep).filter(Boolean).length > MANIFEST_DEPTH_V1) continue;
      for (const [target, declaration] of Object.entries(project.targets)) {
        const produces = (declaration.outputs ?? []).some(output => typeof output === "string" && outputCoversV1(root, project.path, output, absolute));
        if (produces) owners.add(`${project.name}:${target}`);
        else if (generated && target === "generate") fallback.add(`${project.name}:${target}`);
      }
    }
  }
  return [...(owners.size > 0 ? owners : fallback)].sort();
};

/** 🔎️ Lists the files that Rust `#[path]` module mounts under `root` point to and that do not exist yet. */
export async function missingMountedModulesV1(root: string, control: GeneratedScanControlV1 = {}): Promise<readonly string[]> {
  const base = resolve(root);
  return missingMountsV1(base, (await inventoryV1(base, control)).sources, control);
}

/** 🏭️ Names the `project:target` pairs that produce a missing Rust module: a target declaring the file among its outputs, or else the `generate` target of a project near it, when the file is generated output.
 * @see https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute */
export async function generatedModuleOwnersV1(root: string, missing: string, control: GeneratedScanControlV1 = {}): Promise<readonly string[]> {
  const base = resolve(root);
  return ownersOfV1(base, missing, await projectsV1(base, (await inventoryV1(base, control)).manifests, control));
}

/** 🧱️ Neutral `run` requests, one per owning `project:target`, that must succeed before any Rust source closure of the workspace is read; the workspace is scanned once. */
export async function generatedPrerequisiteArgumentsV1(root: string, control: GeneratedScanControlV1 = {}): Promise<readonly (readonly string[])[]> {
  const base = resolve(root);
  const inventory = await inventoryV1(base, control);
  const missing = await missingMountsV1(base, inventory.sources, control);
  await control.report?.("running");
  const projects = await projectsV1(base, inventory.manifests, control);
  const targets = new Set(missing.flatMap(path => ownersOfV1(base, path, projects)));
  return [...targets].sort().map(target => Object.freeze(["run", target]));
}
