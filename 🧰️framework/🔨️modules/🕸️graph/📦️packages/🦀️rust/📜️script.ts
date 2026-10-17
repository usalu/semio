#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 📜️ `@semio-tech/framework-graph` task router. */
import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createIssuedResourceControlV1, runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { runRepositoryCargoTestsV1 } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { CheckGeneratedScript, GRAPH_PACKAGES, GenerateScript, LintScript, ManifestContractScript, PreviewGeneratedScript, TestScript } from "../../🛂️manifest/🏃️execution/🟦️.ts";

import { resolve } from "node:path";

/** 🦀️ Runs the complete default Graph native library cohort through its managed owner. */
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-native accepts no arguments");
    const phaseControl = createIssuedResourceControlV1(this.invocation, "graph:native-tests", 0);
    const { readCargoTestPolicyV1 } = await import("../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts");
    checkResourceLeaseControl(phaseControl);
    await runRepositoryCargoTestsV1({manifestPath:resolve(this.repoRoot,"🧰️framework/Cargo.toml"),packages:GRAPH_PACKAGES,cwd:this.root,extraArgs:["--lib","--no-fail-fast"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation,phaseControl);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-generated", CheckGeneratedScript)
  .register("test", TestScript)
  .register("test-native", NativeTestScript)
  .register("test-manifest-contract", ManifestContractScript)
  .register("lint", LintScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "generate" }) }));
