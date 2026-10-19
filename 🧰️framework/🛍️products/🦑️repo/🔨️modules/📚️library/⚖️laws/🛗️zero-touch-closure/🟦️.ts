import { checkZeroTouchClosuresV1, ZERO_TOUCH_CLOSURES_FILE } from "../../🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
import type { BreachRecord } from "../../🚨️policy/🔖️breach/🟦️.ts";

/** 🪫️ Fails when an import closure that runs before the dependencies are installed names a workspace package, an external package or an eager post-install module; each finding names the file, the specifier and the repair. */
export function policyZeroTouchClosureBreaches(repoRoot: string): BreachRecord[] {
  return checkZeroTouchClosuresV1(repoRoot).findings.map((finding) => ({
    id: `zero-touch-${finding.closure}-${finding.kind}-${finding.file}-${finding.specifier}`,
    summary: `${finding.closure} ${finding.kind} ${finding.file} imports ${finding.specifier}: ${finding.detail}`,
    kind: "repo/zero-touch-closure",
    scope: finding.file,
    priority: "high",
    autofixable: false,
    reason: "The repo CLI, the dependency installers and the setup routes run on a checkout without node_modules, so every import they reach must be relative and free of external packages.",
    solution: `Import the owner module by a relative path, move the external use behind a dynamic import of a seam listed in ${ZERO_TOUCH_CLOSURES_FILE}, or add the closure's seam there with its reason.`,
  }));
}
