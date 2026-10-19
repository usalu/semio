#!/usr/bin/env bun
import{createIssuedResourceControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { readCargoTestPolicyV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** ✅️ `@semio-tech/schema-validator-rs` router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runNeutralOwnerScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    const { rest } = resolveTestLevel(segments);
    const { proveSchemaValidatorOwnershipV1 } = await import("../../🧪️tests/📏️ownership/🟦️.ts");
    proveSchemaValidatorOwnershipV1(this.repoRoot);
    const { proveJsonSchemaSubsetContractV1 } = await import("../../🧪️tests/🟦️.ts");
    console.log("schema-validator: " + await proveJsonSchemaSubsetContractV1() + " unchanged subset vectors passed");
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-schema-validator"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}

class NeutralOwnerScript extends BundleScript {
  async run(): Promise<void> {
    await runNeutralOwnerScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-neutral-owner", NeutralOwnerScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
