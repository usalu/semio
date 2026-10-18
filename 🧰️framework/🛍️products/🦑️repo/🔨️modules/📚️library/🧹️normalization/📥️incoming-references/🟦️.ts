import { type TaxonomyProgress, report } from "../🏃️operation/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, createLexicalInputSession, lstatOrNull } from "../📁️input/🟦️.ts";
import { execFileSync, spawnSync } from "node:child_process";
import { lstatSync, readFileSync, readlinkSync } from "node:fs";
import type { Stats } from "node:fs";
import { dirname, posix, relative, resolve } from "node:path";
import type { TaxonomyInventory, TaxonomyInventoryEntry, TaxonomyPlanOptions } from "../📜️plan-contracts/🟦️.ts";
import { areaId, checkCancellation, isExcluded, ownerId } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { opaqueLeafNames, referenceResolutionReach } from "../✂️reference-edits/🟦️.ts";
import { logicalRepositorySymlinkTargetPath, projectedPath } from "../➰️symlink-targets/🟦️.ts";
import { taxonomyFixturePayloadPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { type IncomingReferenceScan, type IncomingReferenceSnapshot, ancestorReferenceCoordinateRoot, incomingReferenceLexicalAdmission, incomingReferenceSnapshots, inventoryCaptures, referenceCandidatesWithProgress, referenceEntry, referenceInventoryContexts, repositoryReferenceCandidatePaths } from "../🔗️reference-inventory/🟦️.ts";
import { isFrozenSourceCoordinateToken, validateObservedFrozenEvidenceNodes } from "../❄️frozen-evidence/🟦️.ts";
import { referencePathIndex, resolveReferenceTokenPath } from "../🧭️reference-resolution/🟦️.ts";
import { referenceDigestMayResolveInto, referenceTokenStore, textualPath } from "../🧮️reference-digests/🟦️.ts";
import { referenceTokensIncludingUnsupported, storedReferenceAnalysis } from "../🔬️reference-analysis/🟦️.ts";

export function referenceCoordinateRoots(repoRoot: string, paths: Iterable<string>, taxonomy: LoadedTaxonomy, cancelFile?: string, observe?: (path: string, stat: Stats | null, bytes?: Uint8Array) => void, progress?: TaxonomyPlanOptions["progress"], operation: TaxonomyProgress["operation"] = "plan"): readonly string[] {
  checkCancellation(repoRoot, cancelFile);
  const directories = new Set<string>(), roots = new Set<string>(), admit = createLexicalInputSession();
  let callingGitDirectory: string | undefined;
  for (const path of paths) for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) directories.add(parent);
  const ordered = [...directories].sort((left, right) => left.split("/").length - right.split("/").length || generatorPathCompare(left, right));
  if (ordered.length === 0) report(progress, operation, "incoming-coordinate-roots", 0, 0);
  for (const [index, path] of ordered.entries()) {
    report(progress, operation, "incoming-coordinate-roots", index, ordered.length, path);
    checkCancellation(repoRoot, cancelFile);
    if (isExcluded(path, taxonomy) || taxonomyFixturePayloadPath(path, taxonomy.discoverySchema) || path.split("/").includes(".git") || ancestorReferenceCoordinateRoot(path, roots)) continue;
    const marker = admit(repoRoot, `${path}/.git`, "Reference repository marker"), stat = lstatOrNull(marker);
    if (!stat) { observe?.(`${path}/.git`, null); continue; }
    if (stat.isSymbolicLink()) throw new Error(`Reference repository marker is a symlink: ${path}/.git`);
    if (stat.isFile()) {
      const bytes = readFileSync(marker), content = bytes.toString("utf8"), target = content.match(/^gitdir:\s*([^\r\n]+)\r?\n?$/u)?.[1];
      observe?.(`${path}/.git`, stat, bytes);
      if (!target) continue;
      const gitdir = assertLexicalInputOutsideOpaque(repoRoot, resolve(dirname(marker), target), "Reference repository gitdir", true);
      const gitdirStat = lstatOrNull(gitdir);
      observe?.(relative(repoRoot, gitdir).replaceAll("\\", "/"), gitdirStat);
      if (!gitdirStat?.isDirectory()) throw new Error(`Reference repository gitdir is not a local directory: ${path}/.git`);
      callingGitDirectory ??= resolve(repoRoot, execFileSync("git", ["rev-parse", "--absolute-git-dir"], { cwd: repoRoot, encoding: "utf8" }).trim());
      if (resolve(gitdir) === callingGitDirectory) continue;
    } else {
      observe?.(`${path}/.git`, stat);
      if (!stat.isDirectory()) continue;
    }
    const parent = assertLexicalInputOutsideOpaque(repoRoot, path, "Reference repository owner", true);
    const result = spawnSync("git", ["rev-parse", "--show-toplevel"], { cwd: parent, encoding: "utf8" });
    if (result.status === 0 && resolve(result.stdout.trim()) === resolve(parent)) roots.add(path);
  }
  checkCancellation(repoRoot, cancelFile);
  if (ordered.length > 0) report(progress, operation, "incoming-coordinate-roots", ordered.length, ordered.length);
  return [...roots].sort((left, right) => right.split("/").length - left.split("/").length || generatorPathCompare(left, right));
}

/** 👁️ One incoming-reference candidate as the snapshot reads it: a symlink with its target, a textual file with its bytes, or `undefined`
 * for anything else. An invalid frozen evidence document is reported by the reference planner, never thrown here. */
function incomingReferenceObservation(repoRoot: string, path: string, taxonomy: LoadedTaxonomy, admit: typeof assertLexicalInputOutsideOpaque): Readonly<{ stat: Stats; target?: string; bytes?: Buffer }> | undefined {
  if (taxonomyFixturePayloadPath(path, taxonomy.discoverySchema)) return undefined;
  const absolute = admit(repoRoot, path, "Incoming reference candidate"), stat = lstatOrNull(absolute);
  if (!stat || stat.isDirectory()) return undefined;
  if (stat.isSymbolicLink()) return { stat, target: readlinkSync(absolute) };
  if (!stat.isFile() || !textualPath(path)) return undefined;
  return { stat, bytes: readFileSync(absolute) };
}

/** 🔦️ Reads the incoming-reference candidates once for every inventory of a multi-scope run (see {@link IncomingReferenceScan}); keeps a
 * textual file only when the union of all inventories' changing paths lexically admits it, a superset of what each scope admits. */
export function incomingReferenceScan(inventories: readonly TaxonomyInventory[], taxonomy: LoadedTaxonomy, options: Pick<TaxonomyPlanOptions, "cancelFile" | "progress">): IncomingReferenceScan | undefined {
  const changing = new Set(inventories.flatMap((inventory) => inventory.entries.filter((entry) => entry.sourcePath !== entry.normalizedPath).map((entry) => entry.sourcePath)));
  if (changing.size === 0 || inventories.length === 0) return undefined;
  const repoRoot = inventories[0]!.repoRoot;
  const paths = repositoryReferenceCandidatePaths(repoRoot, taxonomy, referenceInventoryContexts.get(inventories[0]!), options.cancelFile);
  const coordinateRoots = referenceCoordinateRoots(repoRoot, paths, taxonomy, options.cancelFile, undefined, options.progress, "plan");
  const admits = incomingReferenceLexicalAdmission(changing), observed = new Map<string, Readonly<{ stat: Stats; target?: string; bytes?: Buffer }>>(), admit = createLexicalInputSession();
  for (const [, path] of referenceCandidatesWithProgress(paths, "plan", options.progress)) {
    checkCancellation(repoRoot, options.cancelFile);
    const row = incomingReferenceObservation(repoRoot, path, taxonomy, admit);
    if (row && (row.target !== undefined || admits(row.bytes!.toString("utf8")))) observed.set(path, row);
  }
  return { paths, coordinateRoots, observed };
}

export function incomingReferenceSnapshot(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy, options: Pick<TaxonomyPlanOptions, "cancelFile" | "progress"> = {}): IncomingReferenceSnapshot {
  checkCancellation(inventory.repoRoot, options.cancelFile);
  const cached = incomingReferenceSnapshots.get(inventory);
  if (cached) return cached;
  const changing = new Set(inventory.entries.filter((entry) => entry.sourcePath !== entry.normalizedPath).map((entry) => entry.sourcePath));
  const context = referenceInventoryContexts.get(inventory);
  const scan = changing.size > 0 ? inventoryCaptures.get(inventory)?.incoming : undefined;
  const paths = scan?.paths ?? (changing.size > 0 ? repositoryReferenceCandidatePaths(inventory.repoRoot, taxonomy, context, options.cancelFile) : []);
  const knownPaths = new Set([...paths, ...inventory.entries.map((entry) => entry.sourcePath)]);
  for (const path of paths) for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) knownPaths.add(parent);
  validateObservedFrozenEvidenceNodes(inventory.repoRoot, knownPaths, taxonomy);
  const coordinateRoots = scan?.coordinateRoots ?? referenceCoordinateRoots(inventory.repoRoot, paths, taxonomy, options.cancelFile, undefined, options.progress, "plan");
  const known = referencePathIndex(knownPaths, inventory.repoRoot, coordinateRoots, undefined, options.cancelFile, changing), admitted = new Set(inventory.entries.map((entry) => entry.sourcePath));
  const admitsText = incomingReferenceLexicalAdmission(changing), reach = referenceResolutionReach(changing), store = referenceTokenStore(inventory.repoRoot), opaqueLeaves = opaqueLeafNames(taxonomy);
  const entries: TaxonomyInventoryEntry[] = [], contents = new Map<string, string>(), admit = createLexicalInputSession();
  for (const [index, path] of referenceCandidatesWithProgress(scan ? [...scan.observed.keys()] : paths, "plan", options.progress)) {
    checkCancellation(inventory.repoRoot, options.cancelFile);
    if (admitted.has(path) || taxonomyFixturePayloadPath(path, taxonomy.discoverySchema)) continue;
    const observed = scan ? scan.observed.get(path)! : incomingReferenceObservation(inventory.repoRoot, path, taxonomy, admit);
    if (!observed) continue;
    const stat = observed.stat;
    if (observed.target !== undefined) {
      const logical = logicalRepositorySymlinkTargetPath(inventory.repoRoot, path, observed.target);
      if (logical && !isExcluded(logical, taxonomy) && projectedPath(logical, inventory.entries) !== logical) entries.push(referenceEntry(inventory.repoRoot, path, taxonomy)!);
    } else if (observed.bytes !== undefined) {
      const bytes = observed.bytes, content = bytes.toString("utf8");
      if (!admitsText(content)) continue;
      report(options.progress, "plan", "incoming-parse", index + 1, paths.length, path);
      const analysis = storedReferenceAnalysis(store, inventory.repoRoot, path, sha256(bytes), opaqueLeaves, () => ({ text: content, matches: true }));
      if (!referenceDigestMayResolveInto(analysis.digest, reach)) continue;
      const relevant = referenceTokensIncludingUnsupported(path, content, known, analysis).some((token) => {
        if (token.unsupportedReason && token.physicalTargets?.some((target) => changing.has(target))) return true;
        const target = resolveReferenceTokenPath(path, token, known);
        return target !== null && changing.has(target) && !isFrozenSourceCoordinateToken(path, bytes, token, target, taxonomy, inventory.repoRoot);
      });
      if (relevant) {
        const current = lstatSync(assertLexicalInputOutsideOpaque(inventory.repoRoot, path, "Incoming reference preimage", true));
        if (!current.isFile() || current.mode !== stat.mode || current.size !== stat.size || current.mtimeMs !== stat.mtimeMs || bytes.byteLength !== stat.size) throw new Error(`Incoming reference changed during its snapshot: ${path}`);
        entries.push({ sourcePath: path, normalizedPath: path, nodeKind: "file", ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength, referencesIn: [], referencesOut: [], violations: [] });
        contents.set(path, content);
      }
    }
  }
  checkCancellation(inventory.repoRoot, options.cancelFile);
  const result = { paths: [...knownPaths].sort(generatorPathCompare), coordinateRoots, entries: entries.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), contents };
  incomingReferenceSnapshots.set(inventory, result);
  return result;
}
