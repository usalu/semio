#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runOwnedCommand } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runRepositoryCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
/** 🌍️ GIS plugin package command router. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { strict as assert } from "node:assert";
import Ajv from "ajv";
import { runRepositoryCargoTests, runCmd, devToolingEnv } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ComponentColdMapPatchCheckScript, ComponentColdMapPatchNativeCheckScript } from "../../🧪️tests/🌉️component-cold-map-patch/🟦️.ts";
import { DurableThreeStoreAssemblyCheckScript, DurableThreeStoreAssemblyNativeCheckScript } from "../../🧪️tests/🗄️durable-three-store-assembly/🟦️.ts";
import { MapCreateRegionGroupCheckScript, MapCreateRegionGroupNativeCheckScript } from "../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/🧪️testing/🧩️map-create-region-group/🟦️.ts";
import { NativeCodecCheckScript, proveGisNativeCodecReceipts } from "../../🧪️tests/📇️native-codecs/🟦️.ts";
import { proveGisControlledProposal } from "../../🧪️tests/💡️inference-control/🟦️.ts";
import { InferenceDiscoveryOracleScript, InferenceDiscoveryCheckScript } from "../../🧪️tests/💡️inference-discovery/🟦️.ts";

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
    await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,...receivers],this.repoRoot,"gis-installed-service-strict",TEST_LEVEL_BUDGET_MS.long,options);
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
  run(): void {
    const root = resolve(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🔌️client");
    const fixture = JSON.parse(readFileSync(resolve(root, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    const validators = new Map<string, ReturnType<Ajv["compile"]>>();
    const ajv = new Ajv({ strict: false });
    for (const vector of fixture.schemaVectors) {
      let validate = validators.get(vector.schema);
      if (!validate) {
        validate = ajv.compile(JSON.parse(readFileSync(resolve(root, "🧬️schema", vector.schema), "utf8")));
        validators.set(vector.schema, validate);
      }
      assert.equal(validate(vector.value), vector.valid, vector.name);
    }
    for (const law of ["inference_client::tests::production_manifest_installs_owner_transport_and_schema_vectors", "inference_client::tests::typed_owner_transport_preserves_proposal_binding_and_closed_geometry"]) {
      runCmd("cargo", ["test", "--manifest-path", resolve(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml"), "-p", "semio-s-artifact-gis-gismap", "--lib", "--", law, "--exact", "--nocapture"], { cwd: this.repoRoot, env: devToolingEnv(), budgetMs: buildBudgetMs() });
    }
    console.log("GIS document-http: Ajv schema parity, owner manifest injection, typed submit/events/approval and geometry binding passed");
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
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
