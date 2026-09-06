#!/usr/bin/env bun
/** 📸️ `@semio-tech/remodel-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-remodel"], this.repoRoot);
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-remodel", join(this.root, "..", "..")));
  }
}

/** @emoji 🛰️ Regenerates the committed `📚️examples/🛰️synthetic-orbit` assets (ten rendered PNG views,
 * their ground-truth JSON and the remodeling document) from the seeded generator in that example's
 * `🧪️tests/🦀️.rs`. Deterministic: a second run leaves the working tree unchanged. */
class RegenerateExampleScript extends BundleScript {
  run(segments: string[]): void {
    const example = segments[0] ?? "synthetic-orbit";
    if (example !== "synthetic-orbit") throw new Error(`unknown remodel example \`${example}\`; known: synthetic-orbit`);
    runCargo(["test", "-p", "semio-s-plugin-remodel", "--lib", "regenerates_the_synthetic_orbit_example", "--", "--ignored", "--nocapture"], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("describe", DescribeScript).register("regenerate-example", RegenerateExampleScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
