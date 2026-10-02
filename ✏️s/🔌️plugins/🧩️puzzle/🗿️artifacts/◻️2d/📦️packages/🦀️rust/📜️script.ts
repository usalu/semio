#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ puzzle-2d Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";

/** 🔍️ Checks this owner and its independent consumer through the public facade. */
class OwnedVerifyScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    if(segments.length!==1||segments[0]!=="snapshot-sqlite-source")throw new Error("Unknown Puzzle 2D verification "+segments.join(" "));
    const snapshot=resolve(this.root,"../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
    await runRepositoryCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--skipLibCheck",resolve(snapshot,"🟦️.ts"),resolve(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts")],this.repoRoot,"puzzle2d-snapshot-sqlite-public-types");
  }
}
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-puzzle-2d", {snapshotSqliteTestFeatures:["component-app-assembly"],commands:{"graph-generate":GraphGenerateScript,"graph-wire-check":OwnerGraphWireCheckScript,verify:OwnedVerifyScript},snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"]});
