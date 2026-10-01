#!/usr/bin/env bun
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { interactivityPuzzleFillP4eSelfTests } from "../../../../🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts";
import { interactivityPuzzleFillRunJobSelfTests } from "../../../../🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-run-job/🟦️.ts";
import { interactivityPuzzleFillTraceSelfTests } from "../../../../🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-trace/🟦️.ts";
class VerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "browser-contribution") {
      if (segments.length !== 1) throw Error("verify browser-contribution accepts no further arguments");
      const { testPuzzleBrowserContributionV1 } = await import("../../🧪️tests/🔗️browser-contribution/🟦️.ts");
      testPuzzleBrowserContributionV1(this.repoRoot);
      return;
    }
    if (segments[0] !== "puzzle-fill-policy-self-tests") throw new Error("Unknown Puzzle composition command");
    const checks = interactivityPuzzleFillP4eSelfTests() + interactivityPuzzleFillRunJobSelfTests() + interactivityPuzzleFillTraceSelfTests();
    console.log(`[owner-policy] Puzzle fill checks=${checks}`);
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", VerifyScript);
await runBundleScriptMain(router, import.meta.url);
