#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧠️ Neural engine native and language-neutral lifecycle validation. */
import { runCargo } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

//#region 🧪️Validation
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-framework-os-kernel-neural-engine", ...rest], this.repoRoot);
  }
}
class SourceTestScript extends BundleScript {
  async run(): Promise<void> { await import("../../🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts"); }
}
/** 🏷️ Validates direct lower type ownership under the actual Neural product owner. */
class TypeOwnershipTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-type-ownership");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🏷️type/🟦️.ts")], this.repoRoot, "neural:type:ownership", 15000);
 }
}
/** 🧭️ Validates original topology ordering against Graphlib through the shared schema. */
class TopologySourceTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-topology-source");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧭️topology/🧪️tests/🟦️.ts")], this.repoRoot, "neural:topology:source", 15000);
 }
}
/** 📥️ Validates original immutable input assembly against independent JSONPatch. */
class InputSourceTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-input-source");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../📥️input/🧪️tests/🟦️.ts")], this.repoRoot, "neural:input:source", 15000);
 }
}
/** ⏱️ Validates original boundary channel semantics with independent Graphlib and JSONPatch. */
class EvaluationSourceTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-evaluation-source");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../⏱️evaluation/🧪️tests/🟦️.ts")], this.repoRoot, "neural:evaluation:source", 15000);
 }
}
/** ✅️ Validates original finished output contracts against independent strict Ajv. */
class OutputSourceTestScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-output-source");
  await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../📔️registry/✅️output/🧪️tests/🟦️.ts")], this.repoRoot, "neural:output:source", 15000);
 }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-source", SourceTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-topology-source", TopologySourceTestScript).register("test-input-source", InputSourceTestScript).register("test-evaluation-source", EvaluationSourceTestScript).register("test-output-source", OutputSourceTestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
//#endregion 🧪️Validation
