import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { join, relative } from "node:path";
import { FileLinter } from "../../../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { defineLint } from "../../../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { goLevelTestArgs, runCanonicalGoTests } from "../../../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { readEntityCatalog } from "../../../../../../../🔨️modules/🧬️schema/📦️packages/🟦️typescript/🏷️entity-kinds/📥️source/🟦️.ts";
import { GENERATOR_ID, REPO_CLI_GO, repoEntityKindTarget } from "../../🏷️entity-kinds/📽️projection/🟦️.ts";

export const policyFile = "🐹️.go";
export const policy = defineLint("repo-client-cli-main-go", (l: FileLinter) => {
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

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const { level, rest } = resolveTestLevel(segments);
  const oracle = await import("../🃏️glob/🟦️.ts");
  oracle.verifyFixtureGlobOracle();
  oracle.verifyTicketRetentionOracle();
  await runCanonicalGoTests(join(context.repoRoot, REPO_CLI_GO), [...goLevelTestArgs(level), ...rest], context.invocation, {
    env: { ...process.env, GOWORK: join(context.repoRoot, "go.work") },
  });
}

/** 🏃️ Runs the `PreviewEntityKindsScript` command under the caller's roots and invocation. */
export function runPreviewEntityKindsScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): void {
  const target = repoEntityKindTarget(context.repoRoot, readEntityCatalog(context.repoRoot));
  const nodes = [{ bytesBase64: Buffer.from(target.content).toString("base64"), mode: 0o644, nodeKind: "file", path: relative(context.repoRoot, target.path).replaceAll("\\", "/").normalize("NFC") }];
  process.stdout.write(`${JSON.stringify({ contractId: GENERATOR_ID, nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
}
