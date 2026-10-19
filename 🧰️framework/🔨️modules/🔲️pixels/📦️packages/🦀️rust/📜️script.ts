#!/usr/bin/env bun
import{createIssuedResourceControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {configuredExactCargoLawPolicyV1} from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🔲️ The canonical native Pixels task owner retains the complete original package workload. */
import { resolve } from "node:path";
import { readCargoTestPolicyV1, runAdmittedExactCargoLawsV1 as runExactCargoLaws, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";

import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🎟️ Original image funding and physical cancellation laws use exact native targets. */
class ImageFundingScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    if(args.length||!process.env.SEMIO_TEST_ARTIFACT_DIR||!process.env.CARGO_TARGET_DIR)throw Error("image funding requires original caller artifact and target directories");
    const receipts=await runExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:3600000,listMilliseconds:60000,lawMilliseconds:120000},invocation:this.invocation,manifestPaths:{"semio-framework-pixels-tests":resolve(this.root,"../../🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-framework-pixels-tests",target:{kind:"lib"},laws:["png_decoding::tests::png_decode_original_funding_and_cancellation_match_actual_allocator","image_decoding::tests::image_sources_original_funding_and_jpeg_allocator_custody","image_decoding::tests::image_sources_decode_shared_rfc_transport_and_all_png_forms","image_decoding::tests::image_sources_exact_limits_and_live_phase_cancellation"]}]});
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    await runCargoTestsV1({ manifestPath: resolve(this.root,"Cargo.toml"),packages:["semio-framework-pixels-tests","semio-framework-deflate","semio-framework-io-base64","semio-framework-io-binary-source"],cwd:resolve(this.root,"../.."),extraArgs:segments },readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript).register("test-image-funding",ImageFundingScript), { invocation: original, ...({defaultCommand:"test"}) }));
