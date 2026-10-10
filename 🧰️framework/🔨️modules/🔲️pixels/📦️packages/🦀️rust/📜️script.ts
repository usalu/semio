#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 🔲️ The canonical native Pixels task owner retains the complete original package workload. */
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {runExactCargoLaws} from "../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

/** 🎟️ Original image funding and physical cancellation laws use exact native targets. */
class ImageFundingScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    if(args.length||!process.env.SEMIO_TEST_ARTIFACT_DIR||!process.env.CARGO_TARGET_DIR)throw Error("image funding requires original caller artifact and target directories");
    const receipts=await runExactCargoLaws({manifestPaths:{"semio-framework-pixels":resolve(this.root,"Cargo.toml")},cargoTargetDir:process.env.CARGO_TARGET_DIR,cwd:this.repoRoot,groups:[{package:"semio-framework-pixels",target:{kind:"lib"},laws:["png_decoding::tests::png_decode_original_funding_and_cancellation_match_actual_allocator","image_decoding::tests::image_sources_original_funding_and_jpeg_allocator_custody","image_decoding::tests::image_sources_decode_shared_rfc_transport_and_all_png_forms","image_decoding::tests::image_sources_exact_limits_and_live_phase_cancellation"]}],artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,buildBudgetMs:3600000,listBudgetMs:60000,lawBudgetMs:120000});console.log(`[DEBUG] Original PNG/JPEG funding native receipts=${receipts.length}`);
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargoTestsV1({ manifestPath: resolve(this.root,"Cargo.toml"),packages:["semio-framework-pixels","semio-framework-deflate","semio-framework-io-base64","semio-framework-io-binary-source"],cwd:resolve(this.root,"../.."),extraArgs:segments },readCargoTestPolicyV1(process.env));
  }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript).register("test-image-funding",ImageFundingScript), { invocation: original, ...({defaultCommand:"test"}) }));
