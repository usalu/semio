#!/usr/bin/env bun
/** 📊️ Print native and shared chart SQLite verification through the repository execution policy. */
import {runArtifactRustPackageMain} from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {runArtifactRustTests} from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {BundleScript} from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
class LogicalOwnerScript extends BundleScript {
 async run(segments:string[]):Promise<void>{const mode=segments[0];if(segments.length!==1||(mode!=="produce"&&mode!=="check"))throw Error("Expected logical-owner produce or check");const prior=process.env.SEMIO_CHART_FIXTURE_PRODUCE;if(mode==="produce")process.env.SEMIO_CHART_FIXTURE_PRODUCE="1";try{await runArtifactRustTests("semio-framework-print",this.repoRoot,["--lib",mode==="produce"?"chart_logical_owner_fixture_producer":"sqlite_snapshot_chart_canonical_logical_owner_assets", "--no-fail-fast","--","--nocapture"]);}finally{if(prior===undefined)delete process.env.SEMIO_CHART_FIXTURE_PRODUCE;else process.env.SEMIO_CHART_FIXTURE_PRODUCE=prior;}}
}
await runArtifactRustPackageMain(import.meta.dir,"semio-framework-print",{commands:{"logical-owner":LogicalOwnerScript},snapshotSqliteTests:["../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
