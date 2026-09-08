#!/usr/bin/env bun
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package|describe>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runExtensionComponentPackage } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describeExtensionComponent } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-imperative-math"], this.repoRoot);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the Nx-owned deliverable and accepts no output override");
    await runExtensionComponentPackage({ rsDir: import.meta.dir, repoRoot: this.repoRoot });
  }
}

class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describeExtensionComponent(this.repoRoot, import.meta.dir));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("package", PackageScript).register("describe", DescribeScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
