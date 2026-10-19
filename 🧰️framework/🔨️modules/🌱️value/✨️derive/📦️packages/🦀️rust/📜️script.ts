#!/usr/bin/env bun
import assert from "node:assert/strict";
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { scriptInvocationBudget } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { configuredExactCargoLawPolicyV1, runExactCargoLaws } from "../../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { executeCargoRequestV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { relative, resolve } from "node:path";
/** 🪄️ `@semio-tech/value-derive-rs` router: `bun ./📜️script.ts test`. */

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await executeCargoRequestV1({version:1,kind:"tests",manifest:relative(this.repoRoot,resolve(this.root,"Cargo.toml")).replaceAll("\\","/"),cwd:relative(this.repoRoot,this.root).replaceAll("\\","/"),packages:["semio-framework-value-derive"],arguments:segments},this.invocation);
  }
}

/** 🚪️ Checks schema-authored semantic admission through independent source and native laws. */
class ValueAdmissionScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || segments.length === 1 && segments[0] !== "--native") throw Error("value-admission-check accepts only --native");
    const source = resolve(this.root, "../../🧬️admission/🧪️tests/🟦️.ts");
    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", source], this.repoRoot, "value:semantic-admission:types", scriptInvocationBudget(this.invocation, 120000), { signal: this.invocation.control.signal, output: { maximumBytes: 8388608, maximumLines: 65536 } });
    (await import("../../🧬️admission/🧪️tests/🟦️.ts")).testValueSemanticAdmission();
    if (!segments.includes("--native")) return;
    const policy = readCargoTestPolicyV1(process.env), manifest = resolve(this.root, "Cargo.toml");
    assert.equal(resolve(policy.manifestPath), manifest, "Value admission laws require the original selected manifest");
    const receipts = await runExactCargoLaws({ invocation: this.invocation, policy: configuredExactCargoLawPolicyV1(), cwd: this.repoRoot, artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, manifestPaths: { "semio-framework-value-derive": manifest }, cargoTargetDir: policy.targetDirectory, groups: [{ package: "semio-framework-value-derive", target: { kind: "test", name: "value_admission" }, cargoArgs: ["--locked"], laws: ["semantic_admission_matches_neutral_schema", "controlled_admission_is_metered_and_refusals_retire_once"] }] });
    assert.equal(receipts.length, 1);
    assert.equal(receipts[0]!.assertions, 2);
    console.log(`[DEBUG] Value semantic admission passed 2 exact native laws; executable ${receipts[0]!.sha256}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("value-admission-check", ValueAdmissionScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
