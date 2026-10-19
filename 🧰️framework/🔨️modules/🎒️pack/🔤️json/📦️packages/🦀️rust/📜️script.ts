
import { resolveTypescriptCompilerV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { readCargoTestPolicyV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { testLevelBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runPersistentCanonicalCheckScript, runReadPolicyScript, runNativeScript, runTreeWireSourceScript, runImmutableTreeScript, runOriginalReceivingScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 📋️ Verifies original persistent canonical arrays and sealed payloads through actual native traversal. */
class PersistentCanonicalCheckScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runPersistentCanonicalCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
/** 🧩️ Runs strict schema, canonical ownership and mandatory JSON source-policy laws. */
class ReadPolicyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runReadPolicyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 📦️ Validates the neutral JSON ownership and source contract. */
class ReferenceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-reference accepts no arguments");
    const test = resolve(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts");
    await runBudgetedTestCommand(process.execPath, [resolveTypescriptCompilerV1(this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", test], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
  }
}

/** 🚮️ Runs the complete reference corpus and the physical specialization absence law. */
class OwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-ownership accepts no arguments");
    const tests = [resolve(this.root, "../../🧪️tests/🧱️ownership/🟦️.ts"), resolve(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")];
    await runBudgetedTestCommand(process.execPath, [resolveTypescriptCompilerV1(this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...tests], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
    await runBudgetedTestCommand(process.execPath, ["test", "--timeout", "30000", ...tests], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), env: {...process.env, SEMIO_TEST_ARTIFACT_DIR: readCargoTestPolicyV1(process.env).artifactDirectory}, throwOnFailure: true});
  }
}

/** 🧾️ Executes every original JSON primitive law and additive explicit-member law. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runNativeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 📨️ Differential neutral laws for the original typed Tree wire source. */
class TreeWireSourceScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runTreeWireSourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}



/** 🔒️ Validates original immutable canonical views and every funded output frontier. */
class ImmutableTreeScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runImmutableTreeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🧳️ Runs actual original JSON native receiving and canonical operation-input custody. */
class OriginalReceivingScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runOriginalReceivingScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test-tree-wire-source",TreeWireSourceScript).register("test-read-policy-source", ReadPolicyScript).register("test-reference", ReferenceScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript).register("test-immutable-tree",ImmutableTreeScript).register("test-persistent-canonical",PersistentCanonicalCheckScript).register("test-original-receiving",OriginalReceivingScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test-ownership" }));
