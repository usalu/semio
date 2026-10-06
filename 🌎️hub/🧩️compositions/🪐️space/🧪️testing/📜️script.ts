#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import contract from "./🧫️fixtures/🔮️ownership/🔣️.json";
import { resolve } from "node:path";

/** 🪐️ Proves source ownership and caller-owned document admission against real inputs. */
class OwnershipTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🔮️ownership/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🦀️ Requires all original host, Space, Home and higher composition native cohorts. */
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    if (rest.length) throw new Error("The complete Space document native gate accepts no cohort filters.");
    for (const project of contract.nativeProjects) {
      const features = project === "@semio-tech/framework-os-host-rs" ? ["--", "rust", "--features", "os-host-full"] : project === "@semio-tech/space-space-rs" || project === "@semio-tech/space-home-rs" ? ["--", "--features", "component-app-assembly"] : [];
      await runRepositoryTestCommand(process.execPath, ["nx", "run", `${project}:test`, "--skip-nx-cache", ...features], { cwd: resolve(this.root, "../../../.."), budgetMs: 3_600_000 });
    }
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-ownership", OwnershipTestScript).register("test-native", NativeTestScript);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test-ownership" });
