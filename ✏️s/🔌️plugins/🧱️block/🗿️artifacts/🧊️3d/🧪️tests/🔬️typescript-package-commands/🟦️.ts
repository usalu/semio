import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { resolve } from "node:path";
import { runOwnedCommand } from "@semio-tech/framework/process/execution";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length === 1 && segments[0] === "physical-codecs") {
    await runOwnedCommand(process.execPath, ["test", resolve(context.root, "./../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🧪️tests/🔣️transport/🟦️.ts")], context.repoRoot, "owned-physical-codecs", 120_000);
    return;
  }
  throw new Error("verify physical-codecs");
}
