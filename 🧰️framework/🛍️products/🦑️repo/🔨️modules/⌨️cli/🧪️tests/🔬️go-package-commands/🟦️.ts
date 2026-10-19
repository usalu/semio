import type { ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { FileLinter } from "../../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { defineLint } from "../../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";

export const policyFile = "🐹️.go";
export const policy = defineLint("repo-cli-go-godfile", (l: FileLinter) => {
  const n = l.lines().length;
  if (n > 10000) {
    return [
      l.breach({
        id: "line-budget",
        summary: `File has ${n} lines (> 10000)`,
        kind: "lint/file/line-budget",
        priority: "medium",
        reason: "Large files are harder to review",
        solution: "Split into smaller modules",
      }),
    ];
  }
  return [];
});

