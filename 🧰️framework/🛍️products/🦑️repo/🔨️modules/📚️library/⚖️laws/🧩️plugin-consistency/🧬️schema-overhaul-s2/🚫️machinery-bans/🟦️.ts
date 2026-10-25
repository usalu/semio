import { existsSync } from "node:fs";
import { join } from "node:path";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { POLICY_STDIO_ARTIFACTS_REL, policyRustFileHasRealTraitImpl } from "../../🚪️artifact-io/📋️owner-table/🟦️.ts";
import { policyAllRustFiles, policyNormalizeRelPath } from "../../🏛️foundation/🟦️.ts";
import { policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";
import { policyListStdioSchemaOwningEntries } from "../📇️stdio-entries/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";

/**
 * 📏️S-7: the vcs plugin's generic `CollectionDiff`/`CollectionMutation`/`Patchable`/`Identified`
 * machinery has real users (🌊️flow's `FlowMutation`, 🌿️vcs itself, 🏪️store's re-export — verified by
 * S1) but ZERO stdio artifact users today (confirmed by grep at S1 and re-confirmed here). This
 * ticket's design has every stdio artifact-standard handcraft its own snapshot/diff/mutation triple
 * per `🧬️schema-design.md` instead of reaching for the generic collection machinery — this rule
 * keeps that true going forward. Seeded EMPTY: any hit is a real regression, not a backlog item.
 */
const POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST = new Set<string>([]);

const POLICY_STDIO_VCS_MACHINERY_BAN_MARKERS = ["CollectionDiff", "CollectionMutation", "Patchable", "Identified"] as const;

const POLICY_STDIO_VCS_MACHINERY_BAN_RE = new RegExp(`\\b(${POLICY_STDIO_VCS_MACHINERY_BAN_MARKERS.join("|")})\\b`);

export function policyStdioVcsMachineryBanBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const prefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(prefix)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const stripped = policyStripRustCommentsAndStrings(content);
    const hit = stripped.match(POLICY_STDIO_VCS_MACHINERY_BAN_RE)?.[1];
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST.has(normalized);
    if (hit) {
      if (allowlisted) continue;
      breaches.push({
        id: `stdio-vcs-machinery-ban-${relPath}`,
        summary: `"${relPath}" references vcs collection machinery (${hit}) — banned under stdio artifacts`,
        kind: "stdio-artifacts/vcs-machinery-ban",
        scope: relPath,
        priority: "high",
        reason: "Stdio artifacts handcraft their own snapshot/diff/mutation triples per artifact (see 🧬️schema-design.md); the generic vcs CollectionDiff/CollectionMutation/Patchable/Identified machinery is reserved for flow/vcs/store and must never leak into stdio.",
        solution: `Remove the ${hit} reference from ${relPath} and handcraft this artifact's own diff/mutation type instead, or add "${normalized}" to POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST citing a real, reviewed justification (expected to stay empty forever).`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `stdio-vcs-machinery-ban-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST but no longer references vcs collection machinery`,
        kind: "stdio-artifacts/vcs-machinery-ban",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_STDIO_VCS_MACHINERY_BAN_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 🆚️S-8 rule 3 (`POLICY_DIFF_ALGEBRA`): every stdio artifact-standard's `🔺️diff/🦀️.rs` must
 * carry a real `impl DiffAlgebra<...> for ...` block (S1 added the trait, deliberately implemented it
 * for nothing — see `s1-spine-report.md`). Seeded with all 31 current standards; F-wave agents shrink
 * this to zero as they land real diffs with handcrafted `inverse`/`is_empty`.
 */
const POLICY_DIFF_ALGEBRA_ALLOWLIST = new Set<string>([]);

export function policyDiffAlgebraBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const entry of policyListStdioSchemaOwningEntries(repoRoot)) {
    const rustRel = `${entry.subsetRel}/🧬️schema/🔺️diff/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    if (!existsSync(join(repoRoot, rustRel))) continue;
    const content = policyReadFileSafe(repoRoot, rustRel);
    const hasImpl = policyRustFileHasRealTraitImpl(content, "DiffAlgebra");
    const normalized = policyNormalizeRelPath(rustRel);
    const allowlisted = POLICY_DIFF_ALGEBRA_ALLOWLIST.has(normalized);
    if (!hasImpl) {
      if (allowlisted) continue;
      breaches.push({
        id: `diff-algebra-missing-${rustRel}`,
        summary: `"${rustRel}" has no impl DiffAlgebra<...> for ... block`,
        kind: "stdio-artifacts/diff-algebra",
        scope: entry.artRel,
        priority: "medium",
        reason: "Every stdio diff type must implement DiffAlgebra (inverse/is_empty) alongside MutationDiff — see 🧬️schema-design.md's Verb set per artifact.",
        solution: `Implement DiffAlgebra<${entry.artifactId}Snapshot> for the diff type in ${rustRel}, or if this standard hasn't been reached yet, add "${normalized}" to POLICY_DIFF_ALGEBRA_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `diff-algebra-stale-${rustRel}`,
        summary: `"${rustRel}" is allowlisted in POLICY_DIFF_ALGEBRA_ALLOWLIST but already implements DiffAlgebra`,
        kind: "stdio-artifacts/diff-algebra",
        scope: entry.artRel,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_DIFF_ALGEBRA_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}
