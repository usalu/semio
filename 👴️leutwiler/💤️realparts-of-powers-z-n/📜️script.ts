#!/usr/bin/env bun
/** 💤️ `@leutwiler/realparts-of-powers-z-n` task router: `bun ./📜️script.ts <deps|build|test> [lean|📯️notes|🏆️proof]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { build, prepareDependencies, verifyAxioms } from "./🟦️.ts";

class DepsScript extends BundleScript {
  async run(): Promise<void> {
    await prepareDependencies(this.root, this.repoRoot);
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await build(this.root, this.repoRoot, segments[0]);
  }
}

class TestScript extends BundleScript {
  async run(): Promise<void> {
    await verifyAxioms(this.root, this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("deps", DepsScript).register("build", BuildScript).register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
