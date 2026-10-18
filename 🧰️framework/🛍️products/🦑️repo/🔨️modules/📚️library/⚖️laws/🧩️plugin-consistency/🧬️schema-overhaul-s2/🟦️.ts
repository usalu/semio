import type { BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { policyAllRustFiles, policyNormalizeRelPath } from "../🏛️foundation/🟦️.ts";
import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyDiffAlgebraBreaches, policyStdioVcsMachineryBanBreaches } from "./🚫️machinery-bans/🟦️.ts";
import { policyFacetMirrorDriftBreaches } from "./🪞️facet-mirror-drift/🟦️.ts";
import { policyGrammarHonestyBreaches } from "./🧱️grammar-honesty/🟦️.ts";

/**
 * 🎫️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 4: a
 * `fn sniff(...)` whose parameter is underscore-prefixed (Rust's own "deliberately unused"
 * convention) cannot possibly inspect the bytes/text it was handed — it can only ever return a
 * constant confidence, which defeats the entire point of format sniffing. Seeded at V0 with every
 * file that fails this today (shrink-only allowlist, same pattern as
 * `POLICY_PACK_COMPLETENESS_ALLOWLIST`): as each artifact's sniff is made real (see the codec
 * uplift waves), remove its entry here — an allowlisted entry whose sniff has ALREADY been fixed
 * is flagged separately below as a stale-acceptance breach, so cleanup isn't silently forgotten.
 */
const POLICY_SNIFF_REALITY_ALLOWLIST = new Set<string>([
  "writer/standards#1-subsets-any-analyzer-component",
  "mathematical/equation/standards#1-subsets-any-analyzer-component",
  "procedural/generation2d/standards#1-subsets-any-analyzer-component",
  "procedural/generation3d/standards#1-subsets-any-analyzer-component",
  "flow/standards#1-subsets-any-analyzer-component",
  "gis/gisterrain/standards#1-subsets-any-analyzer-component",
  "gis/gismap/standards#1-subsets-any-analyzer-component",
  "vcs/standards#1-subsets-any-analyzer-component",
  "animate/presentation/standards#1-subsets-any-analyzer-component",
  "shooting/standards#1-subsets-any-analyzer-component",
  "demonstrator/playground/standards#1-subsets-any-analyzer-component",
  "sequence/standards#1-subsets-any-analyzer-component",
  "fem/2d/standards#1-subsets-any-analyzer-component",
  "fem/3d/standards#1-subsets-any-analyzer-component",
  "architect/program/standards#1-subsets-any-analyzer-component",
  "process/process3d/standards#1-subsets-any-analyzer-component",
  "lowpoly/standards#1-subsets-any-analyzer-component",
  "reasoning/wires/standards#1-subsets-any-analyzer-component",
  "forms/standards#1-subsets-any-analyzer-component",
  "layout/standards#1-subsets-any-analyzer-component",
  "cad/standards#1-subsets-any-analyzer-component",
  "norm/iso16757/standards#1-subsets-any-analyzer-component",
  "norm/vdi3805/standards#1-subsets-any-analyzer-component",
  "norm/din4108/standards#1-subsets-any-analyzer-component",
  "norm/din16798/standards#1-subsets-any-analyzer-component",
  "norm/en1990/standards#1-subsets-any-analyzer-component",
  "norm/en1991/standards#1-subsets-any-analyzer-component",
  "norm/en1992/standards#1-subsets-any-analyzer-component",
  "norm/en1993/standards#1-subsets-any-analyzer-component",
  "norm/en1994/standards#1-subsets-any-analyzer-component",
  "norm/en1995/standards#1-subsets-any-analyzer-component",
  "norm/en1996/standards#1-subsets-any-analyzer-component",
  "norm/en1997/standards#1-subsets-any-analyzer-component",
  "norm/en1998/standards#1-subsets-any-analyzer-component",
  "norm/en1999/standards#1-subsets-any-analyzer-component",
  "norm/din18599/standards#1-subsets-any-analyzer-component",
  "playbook/standards#1-subsets-any-analyzer-component",
  "imperative/standards#1-subsets-any-analyzer-component",
  "remodel/standards#1-subsets-any-analyzer-component",
  "energy/model/standards#1-subsets-any-analyzer-component",
  "trinity/rewriting/standards#1-subsets-any-analyzer-component",
  "trinity/jack/standards#1-subsets-any-analyzer-component",
  "dag/standards#1-subsets-any-analyzer-component",
  "draw/standards#1-subsets-any-analyzer-component",
  "raster/standards#1-subsets-any-analyzer-component",
  "stdio/ifc/standards#4-subsets-any-analyzer-component",
  "stdio/binary/standards#raw-subsets-any-analyzer-component",
  "stdio/pdf/standards#1.4-subsets-any-analyzer-component",
  "stdio/step/standards#ap214-subsets-any-analyzer-component",
  "stdio/xml/standards#1.0-subsets-any-analyzer-component",
  "stdio/dwg/standards#ac1018-subsets-any-analyzer-component",
  "stdio/dwg/standards#ac1024-subsets-any-analyzer-component",
  "stdio/deflate/standards#rfc1950-subsets-any-analyzer-component",
  "note/standards#1-subsets-any-analyzer-component",
  "puzzle/2d/standards#1-subsets-any-analyzer-component",
  "puzzle/5d/standards#1-subsets-any-analyzer-component",
  "puzzle/3d/standards#1-subsets-any-analyzer-component",
  "block/2d/standards#1-subsets-any-analyzer-component",
  "block/5d/standards#1-subsets-any-analyzer-component",
  "block/3d/standards#1-subsets-any-analyzer-component",
  "space/home/standards#1-subsets-any-analyzer-component",
  "sourcing/curate/standards#1-subsets-any-analyzer-component",
]);

export function policySniffRealityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content.includes("fn sniff(")) continue;
    const hasUnusedParam = /fn\s+sniff\(\s*_[A-Za-z0-9_]*\s*:/.test(content);
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_SNIFF_REALITY_ALLOWLIST.has(normalized);
    if (hasUnusedParam) {
      if (allowlisted) continue;
      breaches.push({
        id: `sniff-reality-${relPath}`,
        summary: `"${relPath}" declares fn sniff(...) with an underscore-prefixed (unused) parameter`,
        kind: "artifact-io/sniff-reality",
        scope: relPath,
        priority: "high",
        reason: "sniff() must inspect the bytes/text it's handed to produce a real confidence signal — an unused parameter means it can only ever return a hardcoded constant, defeating format detection.",
        solution: `Make ${relPath}'s sniff() branch on its argument (magic bytes / structural check), or if this artifact hasn't been reached by the codec uplift yet, add "${normalized}" to POLICY_SNIFF_REALITY_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `sniff-reality-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_SNIFF_REALITY_ALLOWLIST but its sniff() already uses its argument`,
        kind: "artifact-io/sniff-reality",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed, or the allowlist silently drifts into meaninglessness.",
        solution: `Remove "${normalized}" from POLICY_SNIFF_REALITY_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** ⚖️Aggregates S2's S-7 ban + all four S-8 drift/honesty/coverage rules. */
export function policySchemaOverhaulS2Breaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyStdioVcsMachineryBanBreaches(repoRoot),
    ...policyFacetMirrorDriftBreaches(repoRoot),
    ...policyGrammarHonestyBreaches(repoRoot),
    ...policyDiffAlgebraBreaches(repoRoot),
  ];
}
