import { canonicalPrimaryFilenameForKind, loadTaxonomy, schemaFacetFormatEntries, type BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import type { PolicyInferenceFamilySource, PolicyInferenceSourceIssue } from "../../🧱️contract/🟦️.ts";
import { policyInferenceSourceBreaches } from "../🚧️source-admission/🟦️.ts";

/** 🧩️ Requires every authored inference family to expose all configured schema-format leaves. */
export function policyInferenceFamilyRootCompletenessBreaches(repoRoot: string, families: readonly PolicyInferenceFamilySource[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const taxonomy = loadTaxonomy(),
    breaches: BreachRecord[] = [],
    issues: PolicyInferenceSourceIssue[] = [];
  for (const family of families) {
    for (const [, format] of schemaFacetFormatEntries(family.inferencesRel, taxonomy)) {
      const leaf = canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy),
        rel = `${family.inferencesRel}/${leaf}`,
        source = policySourceText(repoRoot, rel, operations);
      if (source.state === "file") continue;
      if (source.state !== "missing") {
        issues.push({ path: rel, state: source.state });
        continue;
      }
      breaches.push({
        id: `inference-family-root-leaf-missing-${rel}`,
        summary: `"${family.inferencesRel}" is missing family-root leaf ${leaf}`,
        kind: "inference-migration/family-root-completeness",
        scope: family.artifactRel,
        priority: "medium",
        reason: "Every authored inference facet root must carry every configured schema-format leaf.",
        solution: `Add handcrafted ${rel}.`,
      });
    }
  }
  return [...policyInferenceSourceBreaches(issues), ...breaches];
}
