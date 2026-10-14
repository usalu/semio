#!/usr/bin/env bun
import { resolve as compilerStatePath } from "node:path";
import { typecheckProject } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🐾️ `@semio-tech/pets` router: `bun ./📜️script.ts <test [fundamental|quick|long|exhaustive] | typecheck>`. */
import { join } from "node:path";
import { resolveTestLevel } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCmd, runVitest } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🧪️ `test` — the unit suites of the core. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🪁️ `typecheck` — the core, its unit suites and its Protocol v2 adapters against the compiler. */
class TypecheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await typecheckProject({ workspace: this.repoRoot, stateRoot: compilerStatePath(this.repoRoot, ".cache", "framework"), invocation: this.invocation, cwd: this.root, project: "tsconfig.json", owner: "pets:typescript", args: segments, timeoutMs: 120_000 });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
