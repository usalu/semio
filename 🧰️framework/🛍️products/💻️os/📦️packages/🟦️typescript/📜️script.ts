#!/usr/bin/env bun
/** 🖥️ `@semio-tech/framework-os` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { BundleScript, ScriptRouter, getWorkspaceRoot, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "🧪️tests/🟦️.ts");
  }
}

/** 🏗️ Routes the package generator through the shared workspace implementation. */
class GenerateWgpuScript extends BundleScript {
  async run(): Promise<void> { await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "generate"); }
}
/** 🔎️ Checks the exact package artifacts without writing outputs. */
class CheckWgpuScript extends BundleScript {
  async run(): Promise<void> { await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "check"); }
}
/** 🔮️ Streams the canonical read-only package preview. */
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> { await (await import("../../../../../📜️script.ts")).runWgpuPackageGenerator(getWorkspaceRoot(), "preview"); }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("generate-wgpu", GenerateWgpuScript).register("check-wgpu", CheckWgpuScript).register("preview-generated", PreviewGeneratedScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
