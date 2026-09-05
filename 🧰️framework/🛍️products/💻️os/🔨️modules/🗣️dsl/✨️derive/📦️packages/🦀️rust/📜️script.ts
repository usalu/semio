#!/usr/bin/env bun
/** ✨️ `@semio-tech/dsl-derive-rs` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo, runCargoTestBudgeted } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-os-kernel-dsl-derive"], this.repoRoot, rest);
  }
}

class ExportTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "facade_exports_match_registered_macros", ...segments], this.root);
  }
}

class ExportSourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧪️tests/📤️macro-exports/📜️script.ts"); }
}

class SourceAuthorityTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧪️tests/🛂️mutation-source-authority/📜️script.ts"); }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-exports", ExportTestScript).register("test-exports-source", ExportSourceTestScript).register("test-source-authority-source", SourceAuthorityTestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
