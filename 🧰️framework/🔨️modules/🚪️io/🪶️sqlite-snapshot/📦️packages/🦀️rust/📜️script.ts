
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRefusalScript, runNativeScript, runReceivingSchemaScript, runOperationScript, runOwnershipScript, runOriginalFixedLimitsScript, runOriginalPhysicalScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** ⚠️ Checks owned SQLite refusal producers against the closed schema and independent SQLite. */
class RefusalScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runRefusalScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🪶️ Executes the complete owned SQLite native law roster without selected filters. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runNativeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 📑️ Proves original paid borrowed schema validation against independent SQLite. */
class ReceivingSchemaScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runReceivingSchemaScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 💰️ Checks the complete neutral SQLite operation and frontier laws. */
class OperationScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runOperationScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🏛️ Checks the single neutral SQLite owner and complete public transfer contract. */
class OwnershipScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runOwnershipScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 📜️ Runs the closed original SQLite fixed ceiling contract and selected allocator laws. */
class OriginalFixedLimitsScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runOriginalFixedLimitsScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 🫴️ Executes original physical file custody demands before the whole typed file producer. */
class OriginalPhysicalScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runOriginalPhysicalScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}


const router = new ScriptRouter(import.meta.dirname).register("test-original-fixed-limits",OriginalFixedLimitsScript).register("test-original-physical",OriginalPhysicalScript).register("test-ownership",OwnershipScript).register("test-operation",OperationScript).register("test-refusal", RefusalScript).register("test-native", NativeScript).register("test-receiving-schema",ReceivingSchemaScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test-refusal" }) }));

