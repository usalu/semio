import type { BreachRecord } from "../../../../🟦️.ts";
import type { PolicyInferenceSourceIssue } from "../../🧱️contract/🟦️.ts";

/** 🚧️ Converts unavailable no-follow source evidence into visible inference diagnostics. */
export function policyInferenceSourceBreaches(issues: readonly PolicyInferenceSourceIssue[]): BreachRecord[] {
  return [...new Map(issues.map((issue) => [`${issue.path}:${issue.state}`, issue])).values()].map((issue) => ({
    id: `inference-source-unreadable-${issue.state}-${issue.path}`,
    summary: `"${issue.path}" cannot be admitted for inference analysis: ${issue.state}`,
    kind: "inference-migration/source-unreadable",
    scope: issue.path,
    priority: "medium",
    reason: "Unavailable or linked inference source is unresolved evidence and cannot certify a family as complete.",
    solution: `Restore readable no-follow source access to ${issue.path}.`,
  }));
}
