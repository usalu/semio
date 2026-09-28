#!/usr/bin/env bun
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runExtensionComponentPackage } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-plugin-playbook-procedural"], this.repoRoot, rest);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the Nx-owned deliverable and accepts no output override");
    await runExtensionComponentPackage({ rsDir: import.meta.dir, repoRoot: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("package", PackageScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
