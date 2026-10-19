
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { runCompositionTestScript, runDrawingReaderTestScript, runPrivateReaderTestScript, runNativeDrawingReaderTestScript, runNativeOracleTestScript, runProductionPreparationTestScript } from "./🟦️.ts";

/** 🧩️ Runs portable laws against the canonical oracle's actual source and contribution graph. */
class CompositionTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCompositionTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🖊️ Proves the actual shared DXF reader's ownership and preserved definition bodies. */
class DrawingReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runDrawingReaderTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🧫️ Checks exact private reader ownership and unchanged frozen caller content. */
class PrivateReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runPrivateReaderTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🖊️ Executes the complete additive lower Drawing cohort with its original budget. */
class NativeDrawingReaderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runNativeDrawingReaderTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🦀️ Executes every original provider, family, neutral-law and full assembly unit cohort. */
class NativeOracleTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runNativeOracleTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🏷️ Checks the exact preserved cross-owner type law and corpus composition. */
class TypeOwnershipTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw new Error("Expected test-type-ownership");
  await runRepositoryTestCommand(process.execPath, ["test", "./🧪️tests/🏷️type/🟦️.ts"], {cwd: this.root, budgetMs: 30000});
 }
}

/** 🧱️ Checks production preparation against absent and present examples. */
class ProductionPreparationTestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runProductionPreparationTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-production-preparation", ProductionPreparationTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-private-reader", PrivateReaderTestScript).register("test-composition", CompositionTestScript).register("test-drawing-reader", DrawingReaderTestScript).register("test-native-drawing-reader", NativeDrawingReaderTestScript).register("test-native-oracles", NativeOracleTestScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test-composition" }));
