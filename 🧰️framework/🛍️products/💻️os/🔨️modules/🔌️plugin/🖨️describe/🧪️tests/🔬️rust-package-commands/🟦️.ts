import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🏃️ Runs the `FreshComponentCheckScript` command under the caller's roots and invocation. */
export async function runFreshComponentCheckScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("test-fresh-component accepts no arguments");
  const { createFreshComponentTests } = await import("../🆕️fresh-component/🟦️.ts");
  const { testFreshComponentSourceEpochV1, testFreshComponentStagingV1, testFreshComponentProcessV1 } = createFreshComponentTests(context.invocation);
  await testFreshComponentSourceEpochV1(context.repoRoot);
  await testFreshComponentStagingV1(context.repoRoot);
  await testFreshComponentProcessV1(context.repoRoot);
}
