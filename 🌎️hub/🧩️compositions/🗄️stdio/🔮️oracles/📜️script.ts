#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import contract from "./🧫️fixtures/🧩️composition/🔣️.json";
import { resolve } from "node:path";
import { repoTestArtifactEnvironment } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";

/** 🧩️ Runs portable laws against the canonical oracle's actual source and contribution graph. */
class CompositionTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    process.env.SEMIO_TEST_ARTIFACT_DIR = repoTestArtifactEnvironment(resolve(this.root, "../../../.."), "hub-stdio-oracles").SEMIO_TEST_ARTIFACT_DIR;
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🧩️composition/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🖊️ Proves the actual shared DXF reader's ownership and preserved definition bodies. */
class DrawingReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    process.env.SEMIO_TEST_ARTIFACT_DIR = repoTestArtifactEnvironment(resolve(this.root, "../../../.."), "hub-stdio-oracles").SEMIO_TEST_ARTIFACT_DIR;
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🖊️drawing-reader/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🧫️ Checks exact private reader ownership and unchanged frozen caller content. */
class PrivateReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    process.env.SEMIO_TEST_ARTIFACT_DIR = repoTestArtifactEnvironment(resolve(this.root, "../../../.."), "hub-stdio-oracles").SEMIO_TEST_ARTIFACT_DIR;
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🧫️private-reader/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🖊️ Executes the complete additive lower Drawing cohort with its original oracle feature and budget. */
class NativeDrawingReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    process.env.SEMIO_TEST_ARTIFACT_DIR = repoTestArtifactEnvironment(resolve(this.root, "../../../.."), "hub-stdio-oracles").SEMIO_TEST_ARTIFACT_DIR;
    if (rest.length) throw new Error("The complete lower Drawing native gate accepts no scenario filter.");
    const family = contract.families.find(({ module }) => module === "drawing");
    if (!family) throw new Error("The original lower Drawing cohort is not registered.");
    const root = resolve(this.root, "../../../..");
    await runRepositoryTestCommand("cargo", ["test", "--manifest-path", resolve(root, family.package.path, "Cargo.toml"), "--features", "oracles"], { cwd: root, budgetMs: 900_000 });
  }
}

/** 🦀️ Executes every original provider, family, neutral-law and full assembly unit cohort. */
class NativeOracleTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    process.env.SEMIO_TEST_ARTIFACT_DIR = repoTestArtifactEnvironment(resolve(this.root, "../../../.."), "hub-stdio-oracles").SEMIO_TEST_ARTIFACT_DIR;
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

/** 🏷️ Checks the exact preserved cross-owner type law and corpus composition. */
class TypeOwnershipTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-type-ownership");
  await runRepositoryTestCommand(process.execPath, ["test", "./🧪️tests/🏷️type/🟦️.ts"], {cwd: this.root, budgetMs: 30000});
 }
}

/** 🧱️ Checks production preparation against absent and present examples. */
class ProductionPreparationTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-production-preparation");
  process.env.SEMIO_TEST_ARTIFACT_DIR ??= repoTestArtifactEnvironment(resolve(this.root, "../../../.."), "hub-stdio-production-preparation").SEMIO_TEST_ARTIFACT_DIR;
  await runRepositoryTestCommand(process.execPath, ["test", "../🧩️composition/🧪️tests/🧱️production-preparation/🟦️.ts"], {cwd: this.root, budgetMs: 30000});
 }
}

const router = new ScriptRouter(import.meta.dir).register("test-production-preparation", ProductionPreparationTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-private-reader", PrivateReaderTestScript).register("test-composition", CompositionTestScript).register("test-drawing-reader", DrawingReaderTestScript).register("test-native-drawing-reader", NativeDrawingReaderTestScript).register("test-native-oracles", NativeOracleTestScript);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test-composition" });
