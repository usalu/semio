import type { ScriptInvocation } from "@semio-tech/process/routing";
import { interactivityPuzzleFillP4eSelfTests } from "../../../../🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️testing/🔬️interactivity-puzzle-fill-p4e/🟦️.ts";
import { interactivityPuzzleFillRunJobSelfTests } from "../../../../🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️testing/🔬️interactivity-puzzle-fill-run-job/🟦️.ts";
import { interactivityPuzzleFillTraceSelfTests } from "../../../../🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️testing/🔬️interactivity-puzzle-fill-trace/🟦️.ts";
import { interactivityPuzzleFillP4eFailures, interactivityPuzzleFillRunJobFailures, interactivityPuzzleFillTraceFailures } from "@semio-tech/repo-lib/laws";

/** 🏃️ Runs the `VerifyScript` command under the caller's roots and invocation. */
export async function runVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments[0] === "browser-contribution") {
    if (segments.length !== 1) throw new Error("verify browser-contribution accepts no further arguments");
    const { testPuzzleBrowserContributionV1 } = await import("../🔗️browser-contribution/🟦️.ts");
    testPuzzleBrowserContributionV1(context.repoRoot);
    return;
  }
  if (segments[0] !== "puzzle-fill-policy-self-tests") throw new Error("Unknown Puzzle composition command");
  const checks = interactivityPuzzleFillP4eSelfTests(interactivityPuzzleFillP4eFailures) + interactivityPuzzleFillRunJobSelfTests(interactivityPuzzleFillRunJobFailures) + interactivityPuzzleFillTraceSelfTests(interactivityPuzzleFillTraceFailures);
  console.log(`[owner-policy] Puzzle fill checks=${checks}`);
}
