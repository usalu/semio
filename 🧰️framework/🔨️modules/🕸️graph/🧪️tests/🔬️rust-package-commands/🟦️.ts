import type { ScriptInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { runAdmittedCargoTestsV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { GRAPH_PACKAGES } from "../../🛂️manifest/🏃️execution/🟦️.ts";
import { resolve } from "node:path";

/** 🏃️ Runs the `NativeTestScript` command under the caller's roots and invocation. */
export async function runNativeTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments.length) throw new Error("test-native accepts no arguments");
  const phaseControl = createIssuedResourceControlV1(context.invocation, "graph:native-tests", 0);
  const { readCargoTestPolicyV1 } = await import("../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts");
  checkResourceLeaseControl(phaseControl);
  await runAdmittedCargoTestsV1({manifestPath:resolve(context.repoRoot,"🧰️framework/Cargo.toml"),packages:GRAPH_PACKAGES,cwd:context.root,extraArgs:["--lib","--no-fail-fast"]},readCargoTestPolicyV1(process.env),context.repoRoot,context.invocation,phaseControl);
}
