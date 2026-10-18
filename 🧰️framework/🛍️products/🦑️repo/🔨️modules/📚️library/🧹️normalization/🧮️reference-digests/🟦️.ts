import { report } from "../🏃️operation/🟦️.ts";
import { sourceRelative } from "../🛣️path/🟦️.ts";
import { spawnSync } from "node:child_process";
import { randomUUID } from "node:crypto";
import { mkdirSync, readFileSync, rmSync } from "node:fs";
import { basename, join, posix, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { moduleSourceClosureDigest, openRecordStore, pruneRecordStoreNamespaces, recordKey, type RecordStore } from "../../⚡️caching/🗃️records/🟦️.ts";
import { repoCacheDirectory } from "../../⚡️caching/🟦️.ts";
import type { TaxonomyInventoryOptions } from "../📜️plan-contracts/🟦️.ts";
import { checkCancellation } from "../🏷️file-kind-matching/🟦️.ts";
import { type ReferenceToken, cmakeTokens, embeddedArgumentTokens, gherkinTokens, goTokens, htmlTokens, jsonTokens, markdownInlineTokens, markdownReferenceDefinitionTokens, pythonTokens, regexTokens, tomlTokens } from "../🔤️reference-tokenizers/🟦️.ts";
import { typescriptTokens } from "../🧷️typescript-reference-authority/🟦️.ts";
import { type ReferenceResolutionReach, referenceSignatureMayResolveInto } from "../✂️reference-edits/🟦️.ts";
import { type ReferencePathIndex, type RustManifestInspection, type StandaloneReferenceTokens, markdownCommentPathReferenceAuthority, rustTokens } from "../🦀️rust-reference-graph/🟦️.ts";
import { resolveReferenceTokenPath } from "../🧭️reference-resolution/🟦️.ts";

export const REFERENCE_DIGEST_NAMESPACE = "taxonomy-reference-digest-v1";

export let sourceClosureRevision: string | undefined;

/** 🧬️ This module's source closure digest: every record computed by its code is keyed or namespaced by it. */
export function normalizationRevision(): string {
  return sourceClosureRevision ??= moduleSourceClosureDigest(fileURLToPath(import.meta.url));
}

const referenceTokenStores = new Map<string, RecordStore | null>();

/** 🗃️ The persistent reference digest store of one repository, or `null` when its directory is not Git-ignored there (a cache must never
 * become an inventory input). Its namespace is this module's source closure digest, so any change to the tokenizers starts empty. */
export function referenceTokenStore(repoRoot: string): RecordStore | null {
  if (referenceTokenStores.has(repoRoot)) return referenceTokenStores.get(repoRoot)!;
  const root = repoCacheDirectory(repoRoot, "taxonomy", "reference-digests");
  let ignored = false;
  try { ignored = spawnSync("git", ["check-ignore", "-q", "--no-index", sourceRelative(relative(repoRoot, join(root, "probe")))], { cwd: repoRoot, stdio: "ignore" }).status === 0; } catch {}
  let store: RecordStore | null = null;
  if (ignored) {
    const namespace = normalizationRevision().slice(0, 32);
    pruneRecordStoreNamespaces(root, namespace);
    store = openRecordStore(join(root, namespace));
  }
  referenceTokenStores.set(repoRoot, store);
  return store;
}

/**
 * 🧾️ Everything the inventory and the planner's skip test need from one file's index-free reference tokens, without the tokens:
 * - `queries`: each distinct resolution input of a supported token (`["p", ...physicalTargets]` or `["v", ...targetValues ?? [value]]`);
 * - `unsupportedQueries`: the same for the standalone tokens that carry an `unsupportedReason`, which the planner still resolves;
 * - `opaque`: every supported token whose values pass {@link opaqueReferenceCandidate} for the digest's opaque leaves (query index, adapter,
 *   location, values), the only tokens {@link lexicalOpaqueReferenceTarget} can report;
 * - `supported`, `unsupported`: the reach signatures of the supported and the unsupported tokens (see {@link referenceTokenMayResolveInto});
 * - `rustManifest`: whether index-dependent rust manifest tokens can exist, which are then derived live from the content.
 */
export interface ReferenceDigest {
  readonly queries: readonly (readonly string[])[];
  readonly unsupportedQueries: readonly (readonly string[])[];
  readonly opaque: readonly (readonly [query: number, adapter: string, location: string, values: readonly string[]])[];
  readonly supported: Readonly<{ leaves: readonly string[]; nullLeaf: boolean; python: boolean }>;
  readonly unsupported: Readonly<{ leaves: readonly string[]; nullLeaf: boolean; python: boolean }>;
  readonly rustManifest: boolean;
  readonly rustInspection?: RustManifestInspection;
}

/** 🔎️ The resolution input of one token: its physical targets when it has them (`["p", ...]`), otherwise its target values (`["v", ...]`). */
export function referenceTokenQuery(token: ReferenceToken): string[] {
  return token.physicalTargets !== undefined ? ["p", ...token.physicalTargets] : ["v", ...(token.targetValues ?? [token.value])];
}

/** 🪪️ A token that {@link resolveReferenceTokenPath} resolves exactly like every token with this resolution input. */
export function referenceQueryToken(query: readonly string[]): ReferenceToken {
  return query[0] === "p" ? { adapter: "rust", structuredLocation: "", start: 0, end: 0, value: "", physicalTargets: query.slice(1) } : { adapter: "rust", structuredLocation: "", start: 0, end: 0, value: query[1]!, targetValues: query.slice(1) };
}

/** 🎯️ True when some standalone token of the digested file, resolved from `path` through `index`, names a key of `targets`, or carries an
 * `unsupportedReason` and physical targets among them: exactly the tokens a staying file can turn into edits or unsupported-syntax violations.
 * Only queries that pass `reachable` (a conservative pre-test the planner applies to every token before resolving it) are resolved. */
export function referenceDigestResolvesInto(path: string, digest: ReferenceDigest, index: ReferencePathIndex, targets: ReadonlyMap<string, string>, reachable: (token: ReferenceToken) => boolean): boolean {
  for (const query of digest.unsupportedQueries) if (query[0] === "p" && query.slice(1).some((target) => targets.has(target))) return true;
  for (const query of [...digest.queries, ...digest.unsupportedQueries]) {
    const token = referenceQueryToken(query);
    if (!reachable(token)) continue;
    const target = resolveReferenceTokenPath(path, token, index);
    if (target !== null && targets.has(target)) return true;
  }
  return false;
}

/** 🔬️ A {@link ReferenceDigest} plus, when it was just computed rather than read from the store, the standalone and unsupported tokens it
 * was computed from, so a caller that needs the tokens too never tokenizes the same content twice. */
export interface ReferenceAnalysis { readonly digest: ReferenceDigest; readonly standalone?: StandaloneReferenceTokens; readonly unsupported?: readonly ReferenceToken[] }

/**
 * 🧶️ Fills the store with the reference digests of every leaf it lacks using up to `workers` threads that claim the leaves, largest first,
 * from one shared cursor (so a few huge files never leave the other threads idle) and publish their records through files; the caller
 * blocks on shared status words, reporting progress and honouring cancellation. Too few misses to amortize worker start-up leave the work
 * to the caller's sequential path.
 */
export function precomputeReferenceDigests(store: RecordStore, repoRoot: string, rows: readonly (readonly [path: string, contentHash: string, size: number])[], opaqueLeaves: readonly string[], workers: number, progress?: TaxonomyInventoryOptions["progress"], cancelFile?: string): void {
  const missing = rows.filter(([path, contentHash]) => !store.has(recordKey(REFERENCE_DIGEST_NAMESPACE, repoRoot, path, contentHash, ...opaqueLeaves))).sort((left, right) => right[2] - left[2]).map(([path, contentHash]) => [path, contentHash] as const);
  const count = Math.min(workers, Math.floor(missing.length / 256));
  if (count < 2) return;
  const directory = repoCacheDirectory(repoRoot, "taxonomy", "work");
  mkdirSync(directory, { recursive: true });
  const cursor = new SharedArrayBuffer(4);
  const tasks = Array.from({ length: count }, (_, index) => {
    const status = new Int32Array(new SharedArrayBuffer(8)), out = join(directory, `digests-${process.pid}-${randomUUID()}-${index}.tsv`), worker = new Worker(new URL("../🧵️digests/🟦️.ts", import.meta.url).href);
    worker.postMessage({ repoRoot, rows: missing, opaqueLeaves, out, status: status.buffer, cursor });
    return { status, out, worker };
  });
  try {
    report(progress, "inventory", "reference-digests", 0, missing.length);
    while (tasks.some((task) => Atomics.load(task.status, 0) === 0)) {
      for (const task of tasks) if (Atomics.load(task.status, 0) === 0) Atomics.wait(task.status, 0, 0, 250);
      checkCancellation(repoRoot, cancelFile);
      report(progress, "inventory", "reference-digests", tasks.reduce((sum, task) => sum + Atomics.load(task.status, 1), 0), missing.length);
    }
    const failed = tasks.find((task) => Atomics.load(task.status, 0) === 2);
    if (failed) throw new Error(`Reference digest worker failed: ${readFileSync(`${failed.out}.error`, "utf8")}`);
    for (const task of tasks) for (const line of readFileSync(task.out, "utf8").split("\n")) {
      const tab = line.indexOf("\t");
      if (tab > 0) store.put(line.slice(0, tab), Buffer.from(line.slice(tab + 1), "utf8"));
    }
  } finally {
    for (const task of tasks) {
      task.worker.terminate();
      rmSync(task.out, { force: true });
      rmSync(`${task.out}.error`, { force: true });
    }
  }
}

/** 🧲️ False only when no token of the file can resolve into the reach's target set. `rootedNullLeaves` is false when no ancestor of the file
 * is a target: a value without a promisable final segment (`.`, `..`, `./`) only ever resolves to an ancestor of the referencing file's
 * directory, of its coordinate root (itself such an ancestor) or to the repository root, so it can then be ignored. */
export function referenceDigestMayResolveInto(digest: ReferenceDigest, reach: ReferenceResolutionReach, rootedNullLeaves = true): boolean {
  const relevant = (signature: ReferenceDigest["supported"]): boolean => referenceSignatureMayResolveInto({ ...signature, nullLeaf: signature.nullLeaf && rootedNullLeaves }, reach);
  return digest.rustManifest || relevant(digest.supported) || relevant(digest.unsupported);
}

/** 🌳️ True when the repository root or some ancestor directory of `path` is in `targets`. */
export function ancestorInTargets(path: string, targets: ReadonlyMap<string, string> | ReadonlySet<string>): boolean {
  for (let parent = posix.dirname(path); ; parent = posix.dirname(parent)) {
    if (targets.has(parent === "." ? "" : parent)) return true;
    if (parent === "." || parent === "/" || parent === "") return false;
  }
}

export function referenceTokens(path: string, content: string, index?: ReferencePathIndex): readonly ReferenceToken[] {
  const lower = path.toLowerCase();
  if (lower.endsWith(".rs")) return rustTokens(path, content, index);
  if (lower.endsWith(".feature")) return gherkinTokens(path, content);
  if (/\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts)$/u.test(lower)) return typescriptTokens(path, content);
  if (/\.(?:go|mod|work)$/u.test(lower) || /(?:^|\/)go\.(?:mod|work)$/u.test(lower)) return goTokens(path, content);
  if (lower.endsWith(".py")) return pythonTokens(path, content);
  if (/\.(?:csproj|fsproj|vbproj|sln|props|targets|cs|fs|vb)$/u.test(lower)) return regexTokens(content, "dotnet", "dotnet-reference", [/(?:Include|Update|Remove|Link|HintPath)\s*=\s*["']([^"']+)["']/giu, /^Project\([^\r\n]+?=\s*[^,]+,\s*"([^"]+)"/gmu, /\b(?:GetManifestResourceStream|ReadAllText|ReadAllBytes)\s*\(\s*["']([^"']+)["']/gu]);
  if (/\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|cmake)$/u.test(lower) || basename(path) === "CMakeLists.txt") return [...regexTokens(content, "native", "native-path", [/^\s*#\s*include\s*[<"]([^>"]+)[>"]/gmu, /["']([^"']+\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx))["']/gu]), ...cmakeTokens(content)];
  if (lower.endsWith(".json")) return jsonTokens(path, content, "json");
  if (lower.endsWith(".jsonc")) return jsonTokens(path, content, "jsonc");
  if (lower.endsWith(".toml")) return tomlTokens(path, content);
  if (/\.ya?ml$/u.test(lower)) {
    const direct = regexTokens(content, "yaml", "yaml-value", [/^\s*(?:-\s*)?[\w.-]+\s*:\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu, /^\s*-\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu]);
    const embeddedArgv = /(?:workflow|action|launch|task|project|(?:^|\/)ci(?:\/|$))/iu.test(path);
    return embeddedArgv ? [...direct, ...direct.flatMap((token) => embeddedArgumentTokens(content, token.value, token.start, "yaml", "embedded-argv", false))] : direct;
  }
  if (/\.(?:xml|html|htm)$/u.test(lower)) return [...regexTokens(content, "xml", "xml-attribute", [/(?:href|src|path|include|file|link|hintpath)\s*=\s*["']([^"']+)["']/giu]), ...htmlTokens(content, "xml")];
  if (/\.(?:md|mdx)$/u.test(lower)) return [...markdownInlineTokens(content), ...markdownReferenceDefinitionTokens(content), ...htmlTokens(content, "markdown"), ...markdownCommentPathReferenceAuthority(content)];
  return [];
}

export function textualPath(path: string): boolean {
  return /(?:\.rs|\.tsx?|\.jsx?|\.mjs|\.cjs|\.mts|\.cts|\.go|\.mod|\.work|\.py|\.cs|\.fs|\.vb|\.csproj|\.fsproj|\.vbproj|\.sln|\.props|\.targets|\.c|\.cc|\.cpp|\.cxx|\.h|\.hh|\.hpp|\.hxx|\.cmake|\.jsonc?|\.toml|\.ya?ml|\.xml|\.html?|\.mdx?|\.feature)$/iu.test(path) || basename(path) === "CMakeLists.txt";
}
