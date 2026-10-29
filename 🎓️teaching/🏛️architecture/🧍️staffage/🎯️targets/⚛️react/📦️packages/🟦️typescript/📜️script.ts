#!/usr/bin/env repo
import { createIssuedResourceControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { typecheckAdmittedProjectV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runVitest } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
/** ⚛️ `@teaching/staffage-react` router: `repo ./📜️script.ts <test [quick|long|exhaustive] | typecheck>`. */

/** 🧪️ `test` — the web client's cases under `🧪️tests`, run in jsdom. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    await runVitest(this.root, ["--configLoader", "native", ...rest], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🪁️ `typecheck` — the renderer, the app and their cases against the compiler. */
class TypecheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await typecheckAdmittedProjectV1({ workspace: this.repoRoot, invocation: this.invocation, control: createIssuedResourceControlV1(this.invocation, "typecheck:entry", 0), cwd: this.root, project: "tsconfig.json", owner: "staffage-react:typescript", args: segments, timeoutMs: 120_000 });
  }
}

const router = new ScriptRouter(import.meta.dirname).register("test", TestScript).register("typecheck", TypecheckScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
