#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 💠️ `@semio-tech/lowpoly-plugin` router: `repo ./📜️script.ts test`. */
import { runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-hub-lowpoly"], this.repoRoot, this.invocation, rest);
  }
}

/** 🧪️ Exercises the Lowpoly composition, editor commands, managed engine and interchange IO together. */
class RuntimeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-s-artifact-lowpoly-lowpoly-model", "semio-s-artifact-lowpoly-lowpoly-model-diff", "semio-s-artifact-lowpoly-lowpoly-mut-paint", "semio-s-artifact-lowpoly-lowpoly-commands", "semio-s-artifact-lowpoly-lowpoly-view", "semio-plugin-command", "semio-plugin-tests-11-2", "semio-framework-pixels", "semio-hub-lowpoly", "semio-s-artifact-lowpoly-lowpoly-editor", "semio-s-artifact-lowpoly-lowpoly-engine", "semio-s-artifact-lowpoly-lowpoly-io", "semio-s-artifact-lowpoly-lowpoly-mut-objects", "semio-s-artifact-lowpoly-lowpoly-modes", "semio-s-artifact-lowpoly-lowpoly-viewer"], this.repoRoot, this.invocation, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-runtime", RuntimeTestScript);


await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
