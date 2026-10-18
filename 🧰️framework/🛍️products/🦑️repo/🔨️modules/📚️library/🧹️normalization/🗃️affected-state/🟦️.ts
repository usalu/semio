import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { splitLeadingEmoji } from "../🛣️path/🟦️.ts";
import { sha256, lstatOrNull } from "../📁️input/🟦️.ts";
import { readFileSync, readdirSync, readlinkSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { join, posix } from "node:path";
import { type TaxonomyAffectedStateRow, type TaxonomyDestinationAncestorPreimage, type TaxonomyInventory, type TaxonomyInventoryEntry, type TaxonomyPathPreimage, type TaxonomyPlan, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { generatorTreeInventory } from "../🏭️generator-inputs/🟦️.ts";
import { inventoryEntriesByNormalizedPath, referenceEditsByPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { removalAuthorityPaths } from "../🧐️preimage-assertions/🟦️.ts";
import { noFollowTreeDigest } from "../🌳️tree-digests/🟦️.ts";
import { TRANSACTION_LEASE_TOKEN, parseTransactionLease } from "../🔐️transaction-lease/🟦️.ts";
import { recoverCanonicalJsonCandidates } from "../🔄️journal-reconciliation/🟦️.ts";

/** 🧮️ The affected path-state digest over one row per path, plus every path two rows disagree on (first row kept, in path order). */
function affectedStateSummary(rows: readonly TaxonomyAffectedStateRow[]): Readonly<{ digest: string; conflicts: readonly Readonly<{ path: string; rows: readonly string[] }>[] }> {
  const sorted = [...rows].sort((left, right) => generatorPathCompare(left.path, right.path));
  const unique = new Map<string, TaxonomyAffectedStateRow>(), conflicts = new Map<string, Set<string>>();
  for (const row of sorted) {
    const prior = unique.get(row.path);
    if (!prior) { unique.set(row.path, row); continue; }
    const kept = canonicalJson(prior), other = canonicalJson(row);
    if (kept !== other) conflicts.set(row.path, (conflicts.get(row.path) ?? new Set([kept])).add(other));
  }
  return { digest: sha256(`sha256-affected-path-state-v2\u0000${canonicalJson([...unique.values()])}`), conflicts: [...conflicts].map(([path, states]) => ({ path, rows: [...states] })) };
}

function affectedStateDigest(rows: readonly TaxonomyAffectedStateRow[]): string {
  const { digest, conflicts } = affectedStateSummary(rows);
  if (conflicts.length > 0) throw new Error(`Conflicting affected path-state rows at ${conflicts[0]!.path}`);
  return digest;
}

function entryStateRow(path: string, entry?: TaxonomyInventoryEntry): TaxonomyAffectedStateRow {
  if (!entry) return { path, state: "absent" };
  if (entry.nodeKind === "symlink") return { path, state: "symlink", targetHash: entry.contentHash, targetSize: entry.size };
  if (entry.nodeKind === "file") return { path, state: "file", contentHash: entry.contentHash, mode: entry.mode, size: entry.size };
  throw new Error(`Affected directory requires an explicit no-follow tree digest: ${path}`);
}

function pathPreimageRow(path: string, preimage: TaxonomyPathPreimage): TaxonomyAffectedStateRow {
  if (preimage.state === "absent") return { path, state: "absent" };
  if (preimage.state === "directory") throw new Error(`Directory logical target requires recursive tree authority: ${path}`);
  return preimage.state === "symlink" ? { path, state: "symlink", targetHash: preimage.contentHash, targetSize: preimage.size } : { path, state: "file", contentHash: preimage.contentHash, mode: preimage.mode, size: preimage.size };
}

export function destinationAncestorPreimages(repoRoot: string, destinations: readonly string[]): readonly TaxonomyDestinationAncestorPreimage[] {
  const rows = new Map<string, TaxonomyDestinationAncestorPreimage>();
  for (const destination of destinations) {
    for (let path = posix.dirname(destination); path !== "." && path !== ""; path = posix.dirname(path)) {
      const stat = lstatOrNull(absolutePath(repoRoot, path));
      if (stat?.isSymbolicLink() || stat && !stat.isDirectory()) throw new Error(`Mutation destination ancestor is not a no-follow directory: ${path}`);
      rows.set(path, { path, state: stat ? "directory" : "absent" });
    }
  }
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.path, right.path));
}

export function plannedAffectedStateDigests(inventory: TaxonomyInventory, plan: Pick<TaxonomyPlan, "moves" | "embeddedTicketRoots" | "embeddedTicketRootRelocations" | "symlinkTargetEdits" | "evidenceRemovals" | "destinationAncestorPreimages" | "edits" | "regenerations">, resultHashes: ReadonlyMap<string, string>, resultSizes: ReadonlyMap<string, number>): { readonly pre: string; readonly post: string; readonly conflicts: readonly Readonly<{ path: string; side: "pre" | "post"; rows: readonly string[] }>[] } {
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  const authorityStateRow = (path: string): TaxonomyAffectedStateRow => {
    const entry = entries.get(path);
    if (entry) return entryStateRow(path, entry);
    const absolute = absolutePath(inventory.repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat) return { path, state: "absent" };
    if (stat.isSymbolicLink()) { const target = readlinkSync(absolute); return { path, state: "symlink", targetHash: sha256(target), targetSize: Buffer.byteLength(target) }; }
    if (stat.isFile()) return { path, state: "file", contentHash: sha256(readFileSync(absolute)), mode: stat.mode & 0o7777, size: stat.size };
    throw new Error(`Affected authority path must be a no-follow leaf: ${path}`);
  };
  const pre: TaxonomyAffectedStateRow[] = [];
  const post: TaxonomyAffectedStateRow[] = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    pre.push(ancestor);
    post.push({ path: ancestor.path, state: "directory" });
  }
  for (const move of plan.moves) {
    const source = entries.get(move.sourcePath);
    const targetEdit = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath);
    const postSource = source && targetEdit ? { ...source, sourcePath: move.destinationPath, contentHash: targetEdit.newTargetHash, size: Buffer.byteLength(targetEdit.newTarget), symlinkTarget: targetEdit.newTarget } : source ? { ...source, sourcePath: move.destinationPath, contentHash: resultHashes.get(move.destinationPath) ?? source.contentHash, size: resultSizes.get(move.destinationPath) ?? source.size } : undefined;
    pre.push(entryStateRow(move.sourcePath, source), entryStateRow(move.destinationPath, entries.get(move.destinationPath)));
    post.push({ path: move.sourcePath, state: "absent" }, entryStateRow(move.destinationPath, postSource));
  }
  for (const relocation of plan.embeddedTicketRootRelocations) {
    pre.push(entryStateRow(relocation.sourcePath, entries.get(relocation.sourcePath)), entryStateRow(relocation.destinationPath, entries.get(relocation.destinationPath)));
    post.push({ path: relocation.sourcePath, state: "absent" }, relocation.preimage.nodeKind === "symlink" ? { path: relocation.destinationPath, state: "symlink", targetHash: relocation.preimage.contentHash, targetSize: relocation.preimage.size } : { path: relocation.destinationPath, state: "file", contentHash: relocation.preimage.contentHash, mode: relocation.preimage.mode, size: relocation.preimage.size });
  }
  for (const removal of plan.evidenceRemovals) {
    pre.push(entryStateRow(removal.sourcePath, entries.get(removal.sourcePath)));
    post.push({ path: removal.sourcePath, state: "absent" });
    if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members.filter((member) => member.disposition !== "remove")) {
      pre.push(entryStateRow(member.sourcePath, entries.get(member.sourcePath)));
      post.push(member.preimage.nodeKind === "symlink" ? { path: member.finalPath, state: "symlink", targetHash: member.preimage.contentHash, targetSize: member.preimage.size } : { path: member.finalPath, state: "file", contentHash: member.preimage.contentHash, mode: member.preimage.mode, size: member.preimage.size });
    }
    if (removal.authority.kind === "owner-manifest-status") { const manifest = authorityStateRow(removal.authority.manifestPath); pre.push(manifest); post.push(manifest); }
    if (removal.authority.kind === "exact-path-mutation" || removal.authority.kind === "exact-owner-generated-source" || removal.authority.kind === "nested-cargo-generated-source") { const catalog = authorityStateRow(removal.authority.catalogPath); pre.push(catalog); post.push(catalog); }
    
  }
  for (const root of plan.embeddedTicketRoots) { pre.push({ path: root.sourceMetadataRoot, state: "directory-tree", tree: root.sourceTreeDigest }); post.push({ path: root.sourceMetadataRoot, state: "absent" }); }
  for (const edit of plan.symlinkTargetEdits) {
    pre.push({ path: edit.sourcePath, state: "symlink", targetHash: edit.oldTargetHash, targetSize: Buffer.byteLength(edit.oldTarget) });
    pre.push(pathPreimageRow(edit.logicalTargetSourcePath, edit.logicalTargetPreimage));
    post.push({ path: edit.finalPath, state: "symlink", targetHash: edit.newTargetHash, targetSize: Buffer.byteLength(edit.newTarget) });
    const logicalPost = pathPreimageRow(edit.logicalTargetFinalPath, edit.logicalTargetPreimage);
    const logicalTargetEdit = plan.symlinkTargetEdits.find((candidate) => candidate.sourcePath === edit.logicalTargetSourcePath && candidate.finalPath === edit.logicalTargetFinalPath);
    post.push(logicalPost.state === "symlink" && logicalTargetEdit ? { ...logicalPost, targetHash: logicalTargetEdit.newTargetHash, targetSize: Buffer.byteLength(logicalTargetEdit.newTarget) } : logicalPost.state === "file" && resultHashes.has(edit.logicalTargetFinalPath) ? { ...logicalPost, contentHash: resultHashes.get(edit.logicalTargetFinalPath)!, size: resultSizes.get(edit.logicalTargetFinalPath) ?? logicalPost.size } : logicalPost);
  }
  const normalizedEntries = inventoryEntriesByNormalizedPath(inventory), editsAt = referenceEditsByPath(plan.edits);
  for (const [path, hash] of new Map(plan.edits.map((edit) => [edit.path, resultHashes.get(edit.path) ?? edit.preimage.contentHash]))) {
    const entry = entries.get(path) ?? normalizedEntries.get(path);
    if (entry?.nodeKind === "file") { pre.push(entryStateRow(entry.sourcePath, entry)); post.push({ path, state: "file", contentHash: hash, mode: entry.mode, size: resultSizes.get(path) ?? entry.size }); }
    else {
      const preimage = editsAt.get(path)![0]!.preimage;
      pre.push({ path, state: "file", contentHash: preimage.contentHash, mode: preimage.mode, size: preimage.size });
      post.push({ path, state: "file", contentHash: hash, mode: preimage.mode, size: resultSizes.get(path) ?? preimage.size });
    }
  }
  for (const regeneration of plan.regenerations) {
    pre.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.preOutputs)) });
    post.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.outputs)) });
  }
  const before = affectedStateSummary(pre), after = affectedStateSummary(post);
  return { pre: before.digest, post: after.digest, conflicts: [...before.conflicts.map((conflict) => ({ ...conflict, side: "pre" as const })), ...after.conflicts.map((conflict) => ({ ...conflict, side: "post" as const }))] };
}

/** 📜️ Hashes a canonical plan while excluding its self-referential planDigest field. */
export function taxonomyPlanDigest(plan: TaxonomyPlan): string {
  const { planDigest: _planDigest, ...digestible } = plan;
  return sha256(canonicalJson(digestible));
}

export function validateLeasePreparationEvidence(
  backupRoot: string,
  leasePreparationName: (pid: number, token: string, state: "preparing" | "stale") => string,
  jsonWritePreparationName: (pid: number, token: string) => string,
  filename: string,
  previousName: string,
  planDigest: string,
  attemptOrdinal: string,
): void {
  for (const name of readdirSync(backupRoot).sort(generatorPathCompare)) {
    const match = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match) continue;
    const pid = Number.parseInt(requireTaxonomyValue(match[1],"match[1]"), 10), token = requireTaxonomyValue(match[2],"match[2]"), state = match[3] as "preparing" | "stale";
    if (!TRANSACTION_LEASE_TOKEN.test(token) || name !== leasePreparationName(pid, token, state)) throw new Error(`Transaction lease preparation name is invalid: ${name}`);
    const root = join(backupRoot, name), stat = lstatOrNull(root);
    if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction lease preparation must be a no-follow directory: ${name}`);
    recoverCanonicalJsonCandidates(root, filename, previousName, jsonWritePreparationName, (path) => {
      const record = parseTransactionLease(path, planDigest, attemptOrdinal, token);
      if (record.pid !== pid) throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
    }, false, true);
    const canonical = join(root, filename);
    if (lstatOrNull(canonical)) {
      const record = parseTransactionLease(canonical, planDigest, attemptOrdinal, token);
      if (record.pid !== pid) throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
    }
  }
}

export function actualAffectedDigest(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy): string {
  const row = (path: string): TaxonomyAffectedStateRow => {
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat) return { path, state: "absent" };
    if (stat.isSymbolicLink()) { const target = readlinkSync(absolute); return { path, state: "symlink", targetHash: sha256(target), targetSize: Buffer.byteLength(target) }; }
    if (stat.isFile()) return { path, state: "file", contentHash: sha256(readFileSync(absolute)), mode: stat.mode & 0o7777, size: stat.size };
    return { path, state: "directory-tree", tree: noFollowTreeDigest(repoRoot, path) };
  };
  const rows: TaxonomyAffectedStateRow[] = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    const stat = lstatOrNull(absolutePath(repoRoot, ancestor.path));
    rows.push(!stat ? { path: ancestor.path, state: "absent" } : stat.isDirectory() && !stat.isSymbolicLink() ? { path: ancestor.path, state: "directory" } : row(ancestor.path));
  }
  for (const move of plan.moves) rows.push(row(move.sourcePath), row(move.destinationPath));
  for (const relocation of plan.embeddedTicketRootRelocations) rows.push(row(relocation.sourcePath), row(relocation.destinationPath));
  for (const removal of plan.evidenceRemovals) { rows.push(row(removal.sourcePath)); if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members.filter((member) => member.disposition !== "remove")) rows.push(row(member.finalPath)); else for (const path of removalAuthorityPaths(removal.authority)) rows.push(row(path)); }
  for (const root of plan.embeddedTicketRoots) rows.push(row(root.sourceMetadataRoot));
  for (const edit of plan.symlinkTargetEdits) rows.push(row(edit.finalPath), row(edit.logicalTargetFinalPath));
  for (const path of new Set(plan.edits.map((edit) => edit.path))) rows.push(row(path));
  for (const regeneration of plan.regenerations) rows.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy))) });
  return affectedStateDigest(rows);
}

export function actualAffectedPreDigest(repoRoot: string, plan: TaxonomyPlan): string {
  const row = (path: string): TaxonomyAffectedStateRow => {
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat) return { path, state: "absent" };
    if (stat.isSymbolicLink()) { const target = readlinkSync(absolute); return { path, state: "symlink", targetHash: sha256(target), targetSize: Buffer.byteLength(target) }; }
    if (stat.isFile()) return { path, state: "file", contentHash: sha256(readFileSync(absolute)), mode: stat.mode & 0o7777, size: stat.size };
    return { path, state: "directory-tree", tree: noFollowTreeDigest(repoRoot, path) };
  };
  const rows: TaxonomyAffectedStateRow[] = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    const stat = lstatOrNull(absolutePath(repoRoot, ancestor.path));
    rows.push(!stat ? { path: ancestor.path, state: "absent" } : stat.isDirectory() && !stat.isSymbolicLink() ? { path: ancestor.path, state: "directory" } : row(ancestor.path));
  }
  for (const move of plan.moves) rows.push(row(move.sourcePath), row(move.destinationPath));
  for (const relocation of plan.embeddedTicketRootRelocations) rows.push(row(relocation.sourcePath), row(relocation.destinationPath));
  for (const removal of plan.evidenceRemovals) { rows.push(row(removal.sourcePath)); if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members.filter((member) => member.disposition !== "remove")) rows.push(row(member.sourcePath)); else for (const path of removalAuthorityPaths(removal.authority)) rows.push(row(path)); }
  for (const root of plan.embeddedTicketRoots) rows.push(row(root.sourceMetadataRoot));
  for (const edit of plan.symlinkTargetEdits) rows.push(row(edit.sourcePath), row(edit.logicalTargetSourcePath));
  for (const path of new Set(plan.edits.map((edit) => plan.moves.find((move) => move.destinationPath === edit.path)?.sourcePath ?? edit.path))) rows.push(row(path));
  for (const regeneration of plan.regenerations) rows.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.preOutputs)) });
  return affectedStateDigest(rows);
}
