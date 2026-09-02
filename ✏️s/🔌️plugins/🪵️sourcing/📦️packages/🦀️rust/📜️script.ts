#!/usr/bin/env bun
/** 🪵️ `@semio-tech/sourcing-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-sourcing"], this.repoRoot);
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-sourcing", join(this.root, "..", "..")));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("describe", DescribeScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
