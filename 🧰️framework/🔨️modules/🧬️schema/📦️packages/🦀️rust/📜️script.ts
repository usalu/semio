#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { cmdBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 📜️ `@semio-tech/framework-schema` task router. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, type ScriptCommand } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { CheckScript, GenerateScript, PreviewGeneratedScript } from "../../🏷️entity-kinds/🏃️execution/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const Contract = contractTests.get(segments[0] ?? "");
    if (Contract) return await new Contract(this.root, this.repoRoot).run(segments.slice(1));
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-schema"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🩹️fragment-validation-oracle/🟦️.ts")], this.repoRoot, "tool:owner", cmdBudgetMs(), {env: process.env});
  }
}

/** 🧩️ Executes portable schema-subset laws through Bun, Node and AJV. */
class SubsetContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test subset-contract accepts no arguments");
    const { proveJsonSchemaSubsetContractV1 } = await import("../../✅️validator/🧪️tests/🟦️.ts");
    console.log("schema-subset-contract: " + await proveJsonSchemaSubsetContractV1() + " vectors passed");
  }
}


/** 🧱️ Tests schema-first output ownership after removing every product from the loader. */
class EntityOwnershipScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test entity-ownership accepts no arguments");
    const { proveEntityCatalogOwnership } = await import("../../🧪️tests/🏷️entity-kinds/📏️ownership/🟦️.ts");
    await proveEntityCatalogOwnership(this.repoRoot);
    const { runVitestV1, readVitestPolicyV1 } = await import("../../../🏃️process/🧪️testing/🧪️vitest/🟦️.ts");
    await runVitestV1(readVitestPolicyV1(process.env, this.root), [resolve(this.root, "../../🧪️tests/🏷️entity-kinds/🟦️.ts")], resolve(this.root, "../../🧪️tests/🎚️config/🟦️.ts"), process.env);
  }
}

/** 🧺️ Verifies physical mutation schema registration with the independent Ajv oracle. */
class MutationLeafRegistrationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test mutation-leaf-registration accepts no arguments");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("Mutation registration requires caller-owned SEMIO_TEST_ARTIFACT_DIR");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🔮️oracles/✅️validator/🧪️tests/🟦️.ts")], this.repoRoot, "schema:mutation-registration", 15_000, { env: process.env });
  }
}

/** 🧱️ Executes closed neutral schema contracts and current physical owner refusals. */
class NeutralityScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test neutrality accepts no arguments");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("Schema neutrality requires caller-owned SEMIO_TEST_ARTIFACT_DIR");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🧱️neutrality/🟦️.ts")], this.repoRoot, "schema:neutrality", 15_000, { env: process.env });
  }
}

const contractTests = new Map<string, ScriptCommand>([
  ["entity-ownership", EntityOwnershipScript],
  ["subset-contract", SubsetContractScript],
  ["mutation-leaf-registration", MutationLeafRegistrationScript],
  ["neutrality", NeutralityScript],
]);

const router = new ScriptRouter(import.meta.dir)
  .register("generate", GenerateScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check", CheckScript)
  .register("test", TestScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "generate" }) }));
