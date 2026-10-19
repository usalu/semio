#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runVitestV1, readVitestPolicyV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
/** 🎠️ `@semio-tech/framework-kernel` (TS surface) router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand, resolveTypescriptCompilerV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { scriptInvocationBudget } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "receipt-port") {
      if (segments.length !== 1) throw new Error("test receipt-port accepts no arguments");
      await runOwnedCommand(process.execPath, [resolveTypescriptCompilerV1(this.root), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", resolve(this.root, "../../🧪️tests/📦️receipt/🟦️.ts")], this.repoRoot, "kernel:receipt-port:types", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/📦️receipt/🟦️.ts")], this.repoRoot, "kernel:receipt-port", scriptInvocationBudget(this.invocation, 30000), { signal: this.invocation.control.signal });
      return;
    }
    if (segments[0] === "localized-label") {
      if (segments.length !== 1) throw new Error("test localized-label accepts no arguments");
      await runVitestV1(readVitestPolicyV1(process.env, this.root), [], "../../🧪️tests/🏷️localized-label-fixture/🎚️config/🟦️.ts", process.env);
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runVitestV1(readVitestPolicyV1(process.env, this.root), rest, "../../🧪️tests/🎚️config/🟦️.ts", process.env);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
