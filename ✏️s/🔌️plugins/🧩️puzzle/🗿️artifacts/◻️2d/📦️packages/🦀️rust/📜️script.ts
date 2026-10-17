#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ puzzle-2d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";

/** 🔍️ Checks this owner and its independent consumer through the public facade. */
class OwnedVerifyScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length!==1||segments[0]!=="snapshot-sqlite-source")throw new Error("Unknown Puzzle 2D verification "+segments.join(" "));
    const snapshot=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
    await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",resolve(snapshot,"🟦️.ts"),resolve(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts")],this.repoRoot,"puzzle2d-snapshot-sqlite-public-types",{version:1,maximumElapsedMilliseconds:0,output:null});
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-2d", { packages: ["semio-s-artifact-puzzle-2d", "semio-s-artifact-puzzle-2d-board-host", "semio-s-artifact-puzzle-2d-connect-handles-preparation", "semio-s-artifact-puzzle-2d-engine", "semio-s-artifact-puzzle-2d-graph-manifest", "semio-s-artifact-puzzle-2d-history-edit-runtime-tests", "semio-s-artifact-puzzle-2d-inferences", "semio-s-artifact-puzzle-2d-interaction", "semio-s-artifact-puzzle-2d-mut-any", "semio-s-artifact-puzzle-2d-mut-any-10", "semio-s-artifact-puzzle-2d-mut-any-2", "semio-s-artifact-puzzle-2d-mut-any-3", "semio-s-artifact-puzzle-2d-mut-any-4", "semio-s-artifact-puzzle-2d-mut-any-5", "semio-s-artifact-puzzle-2d-mut-any-6", "semio-s-artifact-puzzle-2d-mut-any-7", "semio-s-artifact-puzzle-2d-mut-any-8", "semio-s-artifact-puzzle-2d-mut-any-9", "semio-s-artifact-puzzle-2d-mutations", "semio-s-artifact-puzzle-2d-records", "semio-s-artifact-puzzle-2d-retained", "semio-s-artifact-puzzle-2d-schema", "semio-s-artifact-puzzle-2d-snapshot", "semio-s-artifact-puzzle-2d-tambour-end-storeys", "semio-s-artifact-puzzle-2d-tambour-single-storey", "semio-s-artifact-puzzle-2d-tests", "semio-s-artifact-puzzle-2d-text-inverse", "semio-s-artifact-puzzle-2d-unit-tests", "semio-s-artifact-puzzle-2d-v1"],commands:{"graph-generate":GraphGenerateScript,"preview-generated":GraphPreviewScript,"graph-wire-check":OwnerGraphWireCheckScript,verify:OwnedVerifyScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
