import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, POLICY_TS_COMPONENT_LEAF } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import type { PolicyInferenceFamilySource, PolicyInferenceSourceIssue } from "../../🧱️contract/🟦️.ts";
import { policyInferenceSourceBreaches } from "../🚧️source-admission/🟦️.ts";

/** 🍃️ Requires real Rust and TypeScript leaves for every concrete inference slug. */
export function policyInferenceSlugLeafPresenceBreaches(repoRoot: string, families: readonly PolicyInferenceFamilySource[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [],
    issues: PolicyInferenceSourceIssue[] = [];
  for (const family of families) {
    for (const slug of family.slugs) {
      const slugRel = `${family.inferencesRel}/${slug}`,
        rsRel = `${slugRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`,
        rs = policySourceText(repoRoot, rsRel, operations);
      if (rs.state === "missing")
        breaches.push({
          id: `inference-slug-rs-missing-${slugRel}`,
          summary: `"${slugRel}" has no ${POLICY_RS_COMPONENT_LEAF_NAME}`,
          kind: "inference-migration/slug-leaf-presence",
          scope: family.artifactRel,
          priority: "medium",
          reason: "Every inference slug must carry a real Rust derivation leaf.",
          solution: `Add ${rsRel} with an InferredField implementation or public snapshot function.`,
        });
      else if (rs.state !== "file") issues.push({ path: rsRel, state: rs.state });
      const tsRel = `${slugRel}/${POLICY_TS_COMPONENT_LEAF}`,
        source = policySourceText(repoRoot, tsRel, operations);
      if (source.state === "missing") {
        breaches.push({
          id: `inference-slug-ts-missing-${slugRel}`,
          summary: `"${slugRel}" has no ${POLICY_TS_COMPONENT_LEAF} mirror`,
          kind: "inference-migration/slug-leaf-presence",
          scope: family.artifactRel,
          priority: "medium",
          reason: "Every inference slug must carry a real TypeScript mirror beside its Rust leaf.",
          solution: `Create ${tsRel} with the matching derivation contract.`,
        });
        continue;
      }
      if (source.state !== "file") {
        issues.push({ path: tsRel, state: source.state });
        continue;
      }
      const stripped = source.text
        .replace(/\/\*[\s\S]*?\*\//g, "")
        .replace(/\/\/.*$/gm, "")
        .trim();
      if (stripped === "" || stripped === "export {};")
        breaches.push({
          id: `inference-slug-ts-stub-${slugRel}`,
          summary: `"${tsRel}" is an empty mirror`,
          kind: "inference-migration/slug-leaf-presence",
          scope: family.artifactRel,
          priority: "medium",
          reason: "An inference TypeScript mirror must contain a real contract or implementation.",
          solution: `Give ${tsRel} real content matching its Rust sibling.`,
        });
    }
  }
  return [...policyInferenceSourceBreaches(issues), ...breaches];
}
