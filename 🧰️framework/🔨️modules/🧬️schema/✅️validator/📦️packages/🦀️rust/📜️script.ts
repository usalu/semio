#!/usr/bin/env bun
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** ✅️ `@semio-tech/schema-validator-rs` router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    const { proveSchemaValidatorOwnershipV1 } = await import("../../📏️ownership/🟦️.ts");
    proveSchemaValidatorOwnershipV1(this.repoRoot);
    const { proveJsonSchemaSubsetContractV1 } = await import("../../🧪️tests/🟦️.ts");
    console.log("schema-validator: " + await proveJsonSchemaSubsetContractV1() + " unchanged subset vectors passed");
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-schema-validator"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

class NeutralOwnerScript extends BundleScript {
  async run(): Promise<void> {
    const { runOwnedCommand } = await import("../../../../🏃️process/🎛️owned-execution/🟦️.ts");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🧩️neutral-owner/🟦️.ts")], this.repoRoot, "schema-validator:neutral-owner", 120000, { env: process.env });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-neutral-owner", NeutralOwnerScript);

await runScriptMain(router, { defaultCommand: "test" });
