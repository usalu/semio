#!/usr/bin/env repo
import { runDeclaredRustBuildV1, runRepositoryCargoTests } from "../../../📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** ⚙️ Builds and tests the `semio-framework-repo-test-runner` crate (owned build route for `repo/test-runner/rs`). */
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

const CRATE = "semio-framework-repo-test-runner";

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runDeclaredRustBuildV1(this.repoRoot, "@semio-tech/repo-test-runner-rs", this.root, this.invocation, segments);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests([CRATE], this.repoRoot, this.invocation, rest);
  }
}

if (isMainModuleV1(import.meta.url)) {
  const router = new ScriptRouter(import.meta.dirname).register("build", BuildScript).register("test", TestScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
