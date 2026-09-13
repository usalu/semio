import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, policyStripEmoji } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import type { PolicyInferenceFamilySource, PolicyInferenceSourceIssue } from "../../🧱️contract/🟦️.ts";
import { policyInferenceSourceBreaches } from "../🚧️source-admission/🟦️.ts";

/** ⚙️ Accepts either an entity InferredField implementation or a public whole-snapshot function. */
export function policyInferenceImplPresenceBreaches(repoRoot: string, families: readonly PolicyInferenceFamilySource[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [],
    issues: PolicyInferenceSourceIssue[] = [],
    inferredField = /\bimpl\b[^\n{]*\bInferredField\s*</,
    publicFunction = /\bpub\s+fn\s+\w+/;
  for (const family of families)
    for (const slug of family.slugs) {
      const rel = `${family.inferencesRel}/${slug}/${POLICY_RS_COMPONENT_LEAF_NAME}`,
        source = policySourceText(repoRoot, rel, operations);
      if (source.state === "missing") continue;
      if (source.state !== "file") {
        issues.push({ path: rel, state: source.state });
        continue;
      }
      if (inferredField.test(source.text) || publicFunction.test(source.text)) continue;
      breaches.push({
        id: `inference-impl-missing-${rel}`,
        summary: `"${rel}" has neither an InferredField implementation nor a public derivation function`,
        kind: "inference-migration/impl-presence",
        scope: family.artifactRel,
        priority: "medium",
        reason: "A per-entity derivation must implement InferredField, while a whole-snapshot fold may expose a public function.",
        solution: `Implement InferredField or add compute_${policyStripEmoji(slug).replaceAll("-", "_")} in ${rel}.`,
      });
    }
  return [...policyInferenceSourceBreaches(issues), ...breaches];
}
