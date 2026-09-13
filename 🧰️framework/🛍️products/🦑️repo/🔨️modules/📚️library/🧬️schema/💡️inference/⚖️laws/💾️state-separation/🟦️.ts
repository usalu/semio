import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyWalkRelFileSources } from "../../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { policyLineOfIndex } from "../../../../🔍️discovery/📍️source-coordinate/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { POLICY_DERIVED_MARKER, POLICY_INFERENCES_FACET, type PolicyInferenceSourceIssue } from "../../🧱️contract/🟦️.ts";
import { policyInferenceSourceBreaches } from "../🚧️source-admission/🟦️.ts";

/** 💾️ Keeps derived markers in inference concerns and out of persisted snapshot components. */
export function policyDerivedMarkerLeakBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const walk = policyWalkRelFileSources(
    repoRoot,
    ["✏️s"],
    (relPath, name) => name === POLICY_RS_COMPONENT_LEAF_NAME && relPath.replaceAll("\\", "/").includes("/📸️snapshot/") && !relPath.replaceAll("\\", "/").includes(`/${POLICY_INFERENCES_FACET}/`),
    undefined,
    operations,
  );
  const breaches: BreachRecord[] = [],
    issues: PolicyInferenceSourceIssue[] = [...walk.issues];
  for (const rel of walk.files) {
    const source = policySourceText(repoRoot, rel, operations);
    if (source.state !== "file") {
      if (source.state !== "missing") issues.push({ path: rel, state: source.state });
      continue;
    }
    const index = source.text.indexOf(POLICY_DERIVED_MARKER);
    if (index < 0) continue;
    breaches.push({
      id: `derived-marker-leak-${rel}`,
      summary: `"${rel}" declares ${POLICY_DERIVED_MARKER} inside a snapshot facet`,
      kind: "inference-migration/state-leak",
      scope: rel,
      line: policyLineOfIndex(source.text, index),
      priority: "medium",
      reason: "Derived values are computed inference output and cannot be persisted snapshot input.",
      solution: `Move the derived field from ${rel} into an inference slug.`,
    });
  }
  return [...policyInferenceSourceBreaches(issues), ...breaches];
}
