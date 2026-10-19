#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runVitestV1, readVitestPolicyV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🧪️vitest/🟦️.ts";
/** 🖥️ `@semio-tech/framework-replication` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitestV1(readVitestPolicyV1(process.env,this.root), rest, "../../🧪️tests/🎚️config/🟦️.ts", process.env);
  }
}

/** 🌱️ Runs the original fold-index lazy entry neutral oracle. */
class IndexEntryTestScript extends BundleScript {
  async run():Promise<void>{await import("../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🌱️entry/🟦️.ts");}
}

/** 🧺️ Runs the original generic set custody neutral oracle. */
class IndexSetTestScript extends BundleScript {
  async run():Promise<void>{await import("../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🧺️set/🟦️.ts");}
}

/** 🎟️ Exercises independently authored original index insertion references. */
class IndexInsertionTestScript extends BundleScript{async run():Promise<void>{const {testHistoryIndexInsertion}=await import("../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🎟️insertion/🟦️.ts");testHistoryIndexInsertion();}}
/** 🗂️ Runs shared map transport, composition and inverse laws against Ajv and RFC6902. */
class MapDeltaTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("test-map-delta accepts no arguments");const{testSharedMapDeltaOracle}=await import("../../🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-source/🟦️.ts");testSharedMapDeltaOracle();}
}/** 🧳️ Runs the required retained semantic owner neutral SQLite oracle. */
class RetainedMutationTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("test-retained-mutation accepts no arguments");const{testRetainedMutationOracle}=await import("../../🎮️mutation/🧳️retained/🧪️tests/🟦️.ts");testRetainedMutationOracle();}
}
/** 📤️ Verifies original diagnostic bundle moves against an independent JSON Patch oracle. */
class DiagnosticTransferTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{if(args.length)throw new Error("test-diagnostic-transfer accepts no arguments");const{testDiagnosticTransferOracle}=await import("../../🎮️mutation/📨️messages/♻️retire/🧪️tests/📤️transfer/🟦️.ts");testDiagnosticTransferOracle();}
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-map-delta",MapDeltaTestScript).register("test-diagnostic-transfer",DiagnosticTransferTestScript).register("test-retained-mutation",RetainedMutationTestScript).register("test-index-entry",IndexEntryTestScript).register("test-index-insertion",IndexInsertionTestScript).register("test-index-set",IndexSetTestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
