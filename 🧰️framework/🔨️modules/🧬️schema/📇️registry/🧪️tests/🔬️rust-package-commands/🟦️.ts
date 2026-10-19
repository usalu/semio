import type { ScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { readCargoTestPolicyV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";

/** 🏃️ Runs the `TestScript` command under the caller's roots and invocation. */
export async function runTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const cargoPhase=createIssuedResourceControlV1(context.invocation,"cargo:script-entry",0);
  const focused = segments[0] === "neutrality";
  const { rest } = resolveTestLevel(focused ? segments.slice(1) : segments);
  await runCargoTestsV1({ manifestPath: resolve(context.root, "Cargo.toml"), packages: ["semio-framework-schema-registry"], cwd: context.root, extraArgs: focused ? ["--test", "schema-registry-neutrality", ...rest] : rest }, readCargoTestPolicyV1(process.env), context.repoRoot, context.invocation,cargoPhase);
}
