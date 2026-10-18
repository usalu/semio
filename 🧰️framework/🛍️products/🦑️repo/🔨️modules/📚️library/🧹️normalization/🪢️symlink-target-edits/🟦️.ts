import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { sha256 } from "../📁️input/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { posix } from "node:path";
import type { TaxonomyInventory, TaxonomyPathPreimage, TaxonomyPlanOptions, TaxonomySymlinkTargetEdit, TaxonomyViolation } from "../📜️plan-contracts/🟦️.ts";
import { isExcluded, resolveFileKind, stableViolations, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { dispositionOperationId, generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { logicalRepositorySymlinkTargetPath, projectedPath } from "../➰️symlink-targets/🟦️.ts";
import { taxonomyFixturePayloadPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { referenceEntry } from "../🔗️reference-inventory/🟦️.ts";
import { incomingReferenceSnapshot } from "../📥️incoming-references/🟦️.ts";

export function planSymlinkTargetEdits(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy, options: TaxonomyPlanOptions): { readonly edits: TaxonomySymlinkTargetEdit[]; readonly violations: TaxonomyViolation[] } {
  const edits: TaxonomySymlinkTargetEdit[] = [];
  const violations: TaxonomyViolation[] = [];
  const bySource = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  const links = [...inventory.entries, ...incomingReferenceSnapshot(inventory, taxonomy, options).entries].filter((candidate) => candidate.nodeKind === "symlink" && candidate.symlinkTarget !== undefined && !taxonomyFixturePayloadPath(candidate.sourcePath, taxonomy.discoverySchema));
  for (const entry of links) {
    const oldTarget = entry.symlinkTarget!;
    const logicalTargetSourcePath = logicalRepositorySymlinkTargetPath(inventory.repoRoot, entry.sourcePath, oldTarget);
    if (logicalTargetSourcePath === null || isExcluded(logicalTargetSourcePath, taxonomy)) {
      violations.push(violation("symlink-target-authority-invalid", entry.sourcePath, "Symlink target is external, escaping, or opaque"));
      continue;
    }
    const finalPath = projectedPath(entry.sourcePath, inventory.entries);
    const logicalTargetFinalPath = projectedPath(logicalTargetSourcePath, inventory.entries);
    const targetEntry = bySource.get(logicalTargetSourcePath) ?? referenceEntry(inventory.repoRoot, logicalTargetSourcePath, taxonomy);
    if (targetEntry?.nodeKind === "directory") {
      violations.push(violation("symlink-target-directory-authority-unresolved", entry.sourcePath, `Directory target requires a recursive no-follow authority: ${logicalTargetSourcePath}`));
      continue;
    }
    const logicalTargetPreimage: TaxonomyPathPreimage = !targetEntry ? { state: "absent" } : targetEntry.nodeKind === "symlink" ? { state: "symlink", contentHash: targetEntry.contentHash, mode: targetEntry.mode, size: targetEntry.size, target: targetEntry.symlinkTarget! } : { state: "file", contentHash: targetEntry.contentHash, mode: targetEntry.mode, size: targetEntry.size };
    const extension = resolveFileKind(logicalTargetSourcePath, taxonomy, undefined, []).kind;
    if (!targetEntry && !extension) {
      violations.push(violation("symlink-target-kind-unresolved", entry.sourcePath, `Broken target kind cannot be proven: ${logicalTargetSourcePath}`));
      continue;
    }
    const newTarget = posix.relative(posix.dirname(finalPath), logicalTargetFinalPath);
    if (!newTarget || newTarget.startsWith("/") || isExcluded(posix.normalize(posix.join(posix.dirname(finalPath), newTarget)), taxonomy)) {
      violations.push(violation("symlink-target-render-invalid", entry.sourcePath, "Relative target rendering is empty, absolute, or opaque"));
      continue;
    }
    if (newTarget === oldTarget) continue;
    const targetDigestible = { sourcePath: entry.sourcePath, finalPath, oldTarget, newTarget, logicalTargetSourcePath, logicalTargetFinalPath, logicalTargetPreimage };
    const provisional = { sourcePath: entry.sourcePath, finalPath, oldTarget, newTarget, oldTargetHash: sha256(oldTarget), newTargetHash: sha256(newTarget), logicalTargetSourcePath, logicalTargetFinalPath, logicalTargetPreimage, windowsLinkType: "file" as const, sourceTargetDigest: sha256(canonicalJson(targetDigestible)), rationaleRule: "repository-local-symlink-target-v2" as const, ownerId: entry.ownerId };
    edits.push({ operationId: dispositionOperationId("symlink-target-edit", provisional), ...provisional });
  }
  return { edits: edits.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: [...stableViolations(violations)] };
}
