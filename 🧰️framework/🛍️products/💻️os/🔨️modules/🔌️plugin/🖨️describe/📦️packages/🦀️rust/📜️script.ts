#!/usr/bin/env bun
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { DescriptorBuildScript, DescriptorTestScript } from "../../🏗️component-build/🟦️.ts";
import { DescribeScript } from "../../🛂️descriptor-emission/🟦️.ts";
import { DescribeComponentScript, testFreshComponentSourceEpochV1, testFreshComponentStagingV1, testFreshComponentProcessV1 } from "../../🏭️fresh-component/🟦️.ts";

/** 🧪️ Runs the portable source, staging and actual process lifetime proofs for fresh producers. */
class FreshComponentCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-fresh-component accepts no arguments");
    await testFreshComponentSourceEpochV1(this.repoRoot);
    await testFreshComponentStagingV1(this.repoRoot);
    await testFreshComponentProcessV1(this.repoRoot);
  }
}

if (import.meta.main) await runScriptMain(new ScriptRouter(import.meta.dir).register("build", DescriptorBuildScript).register("test", DescriptorTestScript).register("describe", DescribeScript).register("component", DescribeComponentScript).register("test-fresh-component", FreshComponentCheckScript));
