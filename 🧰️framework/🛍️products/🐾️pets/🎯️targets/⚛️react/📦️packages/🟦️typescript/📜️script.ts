#!/usr/bin/env bun
import { resolve as compilerStatePath } from "node:path";
import { typecheckProject } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** ⚛️ `@semio-tech/pets-react` router: `bun ./📜️script.ts <test [quick|long|exhaustive] | typecheck | dev>`. */
import { join } from "node:path";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCmd, runViteBunxDev, runVitest } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🧪️ `test` — the React suites of the pets product, in jsdom. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments, "quick");
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

/** 🪁️ `typecheck` — the target, its stories and its suites against the compiler. */
class TypecheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await typecheckProject({ workspace: this.repoRoot, stateRoot: compilerStatePath(this.repoRoot, ".cache", "framework"), invocation: this.invocation, cwd: this.root, project: "tsconfig.json", owner: "pets-react:typescript", args: segments, timeoutMs: 120_000 });
  }
}

/** 📖️ `dev` — the stories gallery on `PETS_STORIES_PORT` (default 6069) for the menagerie named by `PETS_MENAGERIE`. */
class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runViteBunxDev(this.root, segments, { config: "../../🏗️builder/🌐️vite/🟦️.ts", portEnv: "PETS_STORIES_PORT", defaultPort: "6069", fixedPort: true });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript).register("dev", DevScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
