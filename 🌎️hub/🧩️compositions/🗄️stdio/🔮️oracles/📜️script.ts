#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import contract from "./🧫️fixtures/🧩️composition/🔣️.json";
import { resolve } from "node:path";

/** 🧩️ Runs portable laws against the canonical oracle's actual source and contribution graph. */
class CompositionTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🧩️composition/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🖊️ Proves the actual shared DXF reader's ownership and preserved definition bodies. */
class DrawingReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🖊️drawing-reader/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🦀️ Executes every original provider, family, neutral-law and full assembly unit cohort. */
class NativeOracleTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    if (rest.length) throw new Error("The complete native oracle gate accepts no cohort or scenario filter.");
    const root = resolve(this.root, "../../../..");
    const packages = [contract.neutralLaw.source.replace(/\/🦀️\.rs$/u, "/📦️packages/🦀️rust"), ...contract.providers.map(({ package: pkg }) => pkg.path), ...contract.families.map(({ package: pkg }) => pkg.path), contract.grammar.package.path, contract.package.path];
    for (const path of packages) {
      console.info(`🦀️ Oracle unit cohort: ${path}`);
      const features = path === packages[0] ? [] : ["--features", "oracles"];
      await runRepositoryTestCommand("cargo", ["test", "--manifest-path", resolve(root, path, "Cargo.toml"), ...features], { cwd: root, budgetMs: 900_000 });
    }
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-composition", CompositionTestScript).register("test-drawing-reader", DrawingReaderTestScript).register("test-native-oracles", NativeOracleTestScript);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test-composition" });
