#!/usr/bin/env repo
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepoScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";
import { runFixtureScript, runGenerateScript, runRetainedAuditScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, ["--configLoader", "runner", ...rest], "../../🧪️tests/🎚️config/🟦️.ts");
  }
}

class FixtureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runFixtureScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class GenerateScript extends BundleScript {
  async run(extra: string[]): Promise<void> {
    await runGenerateScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, extra);
  }
}

class RetainedAuditScript extends BundleScript {
  async run(): Promise<void> {
    await runRetainedAuditScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("fixture", FixtureScript).register("generate", GenerateScript).register("retained-audit", RetainedAuditScript);

await runRepoScriptMain(router, import.meta.url, { defaultCommand: "test" });
