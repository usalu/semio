#!/usr/bin/env bun
/** 🌍️ GIS plugin package command router. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { strict as assert } from "node:assert";
import Ajv from "ajv";
import { registerPlaygroundSiteBuildCommands, resolveTestLevel, runCargoTestBudgeted, runCmd, devToolingEnv, buildBudgetMs } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { ComponentColdMapPatchCheckScript, ComponentColdMapPatchNativeCheckScript } from "../../🧪️tests/🌉️component-cold-map-patch/🟦️.ts";
import { DurableThreeStoreAssemblyCheckScript, DurableThreeStoreAssemblyNativeCheckScript } from "../../🧪️tests/🗄️durable-three-store-assembly/🟦️.ts";
import { MapCreateRegionGroupCheckScript, MapCreateRegionGroupNativeCheckScript } from "../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🧪️tests/🧩️map-create-region-group/🟦️.ts";
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

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await proveGisNativeCodecReceipts(this.repoRoot);
    await runCargoTestBudgeted(["semio-hub-gis"], this.repoRoot, rest);
  }
}

/** 🧬️ Regenerates this plugin's committed native-codec projection `packSchemaHash` column from the live Rust
 * receipts (`os_pack::schema_hash`); the same test, run without the write mode, is the projection-equals-receipts law. */
class NativeCodecProjectionScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["test", "--manifest-path", resolve(this.root, "Cargo.toml"), "-p", "semio-hub-gis", "--no-default-features", "--test", "native_codecs", "--", "native_codec_projection_pack_schema_hashes_equal_live_receipts", "--exact"], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_NATIVE_CODEC_PROJECTION: "write", CARGO_INCREMENTAL: "0" }), budgetMs: buildBudgetMs() });
  }
}

/** 🌍️ Validates the owner's closed transport contract and the installed production manifest. */
class InferenceTransportCheckScript extends BundleScript {
  run(): void {
    const root = resolve(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🔌️client");
    const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🔣️.json"), "utf8"));
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
      runCmd("cargo", ["test", "--manifest-path", resolve(this.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/Cargo.toml"), "-p", "semio-s-artifact-gis-gismap", "--features", "component-app-assembly", "--lib", "--", law, "--exact", "--nocapture"], { cwd: this.repoRoot, env: devToolingEnv(), budgetMs: buildBudgetMs() });
    }
    console.log("GIS document-http: Ajv schema parity, owner manifest injection, typed submit/events/approval and geometry binding passed");
  }
}

const router = new ScriptRouter(import.meta.dir)
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
  .register("component-cold-map-patch-native-check", ComponentColdMapPatchNativeCheckScript);
registerPlaygroundSiteBuildCommands(router);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test" });
