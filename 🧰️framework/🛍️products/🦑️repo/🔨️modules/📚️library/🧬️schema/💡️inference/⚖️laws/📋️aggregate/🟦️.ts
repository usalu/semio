import { POLICY_SOURCE_OPERATIONS, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyDiscoverInferenceFamilies } from "../../🔍️family-discovery/🟦️.ts";
import { policyInferenceSourceBreaches } from "../🚧️source-admission/🟦️.ts";
import { policyInferenceFamilyRootCompletenessBreaches } from "../🧩️family-root-completeness/🟦️.ts";
import { policyInferenceSlugLeafPresenceBreaches } from "../🍃️slug-leaf-presence/🟦️.ts";
import { policyInferenceImplPresenceBreaches } from "../⚙️derivation-presence/🟦️.ts";
import { policyInferenceEmojiUniquenessBreaches } from "../😀️emoji-uniqueness/🟦️.ts";
import { policyInferenceAssemblyCoverageBreaches } from "../🧶️assembly-coverage/🟦️.ts";
import { policyDerivedMarkerLeakBreaches } from "../💾️state-separation/🟦️.ts";

/** 📋️ Aggregates one captured inference-family discovery across all family and state laws. */
export function policyInferenceFamilyBreaches(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS) {
  const discovery = policyDiscoverInferenceFamilies(repoRoot, operations),
    findings = [
      ...policyInferenceSourceBreaches(discovery.issues),
      ...policyInferenceFamilyRootCompletenessBreaches(repoRoot, discovery.families, operations),
      ...policyInferenceSlugLeafPresenceBreaches(repoRoot, discovery.families, operations),
      ...policyInferenceImplPresenceBreaches(repoRoot, discovery.families, operations),
      ...policyInferenceEmojiUniquenessBreaches(discovery.families),
      ...policyInferenceAssemblyCoverageBreaches(repoRoot, discovery.families, operations),
      ...policyDerivedMarkerLeakBreaches(repoRoot, operations),
    ];
  return [...new Map(findings.map((finding) => [finding.id, finding])).values()];
}
