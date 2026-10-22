import { BUILD_OWNER_REGISTRY_FILE, checkBuildOwnerRegistry, type OwnerFindingKind } from "../../🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
import type { BreachRecord } from "../../🚨️policy/🔖️breach/🟦️.ts";

const AUTHORED: readonly OwnerFindingKind[] = ["registry-invalid", "missing-native-root", "row-in-excluded-tree", "provider-owner-root", "provider-source", "provider-consumer-root"];

/** 📋️ Fails when a module has no row in `📋️build-owners.json` or a row names a module, manifest, name or package list the tree no longer has; the sync target repairs every finding except the authored ones. */
export function policyBuildOwnerRegistryBreaches(repoRoot: string): BreachRecord[] {
  return checkBuildOwnerRegistry(repoRoot).findings.map((finding) => ({
    id: `build-owner-registry-${finding.kind}-${finding.owner ?? finding.path}`,
    summary: `${finding.kind} ${finding.owner ?? ""} ${finding.path ?? ""}: ${finding.detail}`.replace(/ {2,}/gu, " "),
    kind: "repo/build-owner-registry",
    scope: finding.path ?? finding.owner ?? BUILD_OWNER_REGISTRY_FILE,
    priority: "high",
    autofixable: !AUTHORED.includes(finding.kind),
    reason: "The build graph admits only modules that have exactly one registry row, so a stale or missing row drops targets or breaks admission.",
    solution: AUTHORED.includes(finding.kind) ? `Fix ${BUILD_OWNER_REGISTRY_FILE} by hand.` : "Run `repo run workspace:verify-build-owners-sync`.",
  }));
}
