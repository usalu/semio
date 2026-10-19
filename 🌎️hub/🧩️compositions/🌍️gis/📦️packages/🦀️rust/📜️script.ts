#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runOwnedCommand, resolveTypescriptCompilerV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runRepositoryCommand, runCmd, devToolingEnv } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolve } from "node:path";
import { runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ComponentColdMapPatchCheckScript, ComponentColdMapPatchNativeCheckScript } from "../../🧪️tests/🌉️component-cold-map-patch/🟦️.ts";
import { DurableThreeStoreAssemblyCheckScript, DurableThreeStoreAssemblyNativeCheckScript } from "../../🧪️tests/🗄️durable-three-store-assembly/🟦️.ts";
import { MapCreateRegionGroupCheckScript, MapCreateRegionGroupNativeCheckScript } from "../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/🧪️testing/🧩️map-create-region-group/🟦️.ts";
import { NativeCodecCheckScript, proveGisNativeCodecReceipts } from "../../🧪️tests/📇️native-codecs/🟦️.ts";
import { proveGisControlledProposal } from "../../🧪️tests/💡️inference-control/🟦️.ts";
import { InferenceDiscoveryOracleScript, InferenceDiscoveryCheckScript } from "../../🧪️tests/💡️inference-discovery/🟦️.ts";
import { runInferenceTransportCheckScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧾️ Validates the authored composition manifest and protocol commitments independently. */
class NativeCodecOracleScript extends BundleScript {
  async run(): Promise<void> {
    await proveGisNativeCodecReceipts(this.repoRoot);
    await proveGisControlledProposal(this.repoRoot);
  }
}

/** 🧱️ Verifies GIS service composition and each concrete receiver at its owning deployment. */
class InstalledServiceSourceScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length)throw new Error("installed-service-source accepts no arguments");
    const tests=resolve(this.root,"../../🧪️tests");
    const source=resolve(tests,"🧱️io-boundary/🟦️.ts");
    const receivers=["🌉️component-cold-map-patch","💡️inference-control","📇️native-codecs"].map(name=>resolve(tests,name,"🟦️.ts"));
    const options={signal:this.invocation.control.signal};
    await runOwnedCommand(process.execPath,[resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,...receivers],this.repoRoot,"gis-installed-service-strict",TEST_LEVEL_BUDGET_MS.long,options);
    await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"gis-installed-service-source",TEST_LEVEL_BUDGET_MS.long,options);
  }
}

/** 🤝️ Runs the hub edit durability gate: every accepted hub edit is durable collaboration state across a hub restart. */
class HubEditDurabilityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("hub-edit-durability-check accepts no arguments");
    await runRepositoryCommand(this.invocation, "bun", [resolve(this.root, "../../🧪️tests/🤝️hub-edit-durability/🟦️.ts")], this.repoRoot, "os-mcp-hub-edit-durability", { version: 1, maximumElapsedMilliseconds: 1_800_000, output: { maximumBytes: 134217728, maximumLines: 65536 } });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await proveGisNativeCodecReceipts(this.repoRoot);
    await runRepositoryCargoTests(["semio-hub-gis"], this.repoRoot, this.invocation, rest);
  }
}

/** 🧬️ Regenerates this plugin's committed native-codec projection `packSchemaHash` column from the live Rust
 * receipts (`pack::record::schema_hash`); the same test, run without the write mode, is the projection-equals-receipts law. */
class NativeCodecProjectionScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["test", "--manifest-path", resolve(this.root, "Cargo.toml"), "-p", "semio-hub-gis", "--test", "native_codecs", "--", "native_codec_projection_pack_schema_hashes_equal_live_receipts", "--exact"], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_NATIVE_CODEC_PROJECTION: "write", CARGO_INCREMENTAL: "0" }), budgetMs: buildBudgetMs() });
  }
}

/** 🌍️ Validates the owner's closed transport contract and the installed production manifest. */
class InferenceTransportCheckScript extends BundleScript {
  async run(): Promise<void> {
    await runInferenceTransportCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("installed-service-source", InstalledServiceSourceScript)
  .register("inference-discovery-oracle", InferenceDiscoveryOracleScript)
  .register("inference-discovery-check", InferenceDiscoveryCheckScript)
  .register("native-codec-oracle", NativeCodecOracleScript)
  .register("inference-transport-check", InferenceTransportCheckScript)
  .register("native-codec-projection", NativeCodecProjectionScript)
  .register("test", TestScript)
  .register("native-codec-check", NativeCodecCheckScript)
  .register("map-create-region-group-check", MapCreateRegionGroupCheckScript)
  .register("map-create-region-group-native-check", MapCreateRegionGroupNativeCheckScript)
  .register("durable-three-store-assembly-check", DurableThreeStoreAssemblyCheckScript)
  .register("durable-three-store-assembly-native-check", DurableThreeStoreAssemblyNativeCheckScript)
  .register("component-cold-map-patch-check", ComponentColdMapPatchCheckScript)
  .register("component-cold-map-patch-native-check", ComponentColdMapPatchNativeCheckScript)
  .register("hub-edit-durability-check", HubEditDurabilityCheckScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
