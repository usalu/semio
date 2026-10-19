import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runAdmittedExactCargoLawsV1 as runExactCargoLaws } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolve } from "node:path";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runCargo } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if(segments[0]==="original-registry-admission-source"||segments[0]==="original-registry-admission-native"){
    if(segments.length!==1)throw new Error("Original registry admission requires exact one cohort");const remaining=context.invocation.control.remainingMilliseconds();if(remaining===null||remaining<=0)throw new Error("Original registry admission requires finite original deadline");
    await runOwnedCommand(process.execPath,["test",resolve(context.root,"../../📔️registry/🏗️admission/🧪️tests/🟦️.ts")],context.repoRoot,"neural:original-registry-admission-source",remaining,{env:process.env});
    if(segments[0]==="original-registry-admission-native"){const left=context.invocation.control.remainingMilliseconds();if(left===null||left<=0)throw new Error("Original registry admission deadline expired");await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation:context.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:left,listMilliseconds:left,lawMilliseconds:left},manifestPaths:{"semio-framework-os-kernel-neural-engine":resolve(context.root,"Cargo.toml")},cwd:context.repoRoot,groups:[{package:"semio-framework-os-kernel-neural-engine",target:{kind:"test",name:"registry_admission"},laws:["original_registry_issuer_reserves_two_shells_before_same_source_donation","original_registry_issuer_cancellation_returns_same_payload_after_born_shell_close"]}]});}return;
  }
  const { rest } = resolveTestLevel(segments);
  for (const crate of ["model", "registry", "budgeted", ""]) await runCargo(["test", "-p", `semio-framework-os-kernel-neural-engine${crate === "" ? "" : `-${crate}`}`, ...rest], context.repoRoot);
}
