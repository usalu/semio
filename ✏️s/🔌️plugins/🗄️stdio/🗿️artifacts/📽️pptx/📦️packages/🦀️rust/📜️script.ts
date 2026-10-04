#!/usr/bin/env bun
/** 📦️ pptx Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";

/** 📐️ Proves the authored signed64 wire corpus and both original mutation tables. */
class TransformWireScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("test transform-wire accepts no arguments");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("Transform wire tests require caller-owned artifacts");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧭️transform/🧪️tests/🟦️.ts")], this.repoRoot, "pptx:transform-wire", 15_000, { env: process.env });
  }
}

/** 🧾 Proves actual OPC XML outline counts and retained-owner refusals. */
class OutlineOwnershipScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test outline-ownership accepts no arguments");
  if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("Outline tests require caller-owned artifacts");
  await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧾outline/🧪️tests/🟦️.ts")],this.repoRoot,"pptx:outline-ownership",15000,{env:process.env});
 }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-stdio-pptx",{testFeatures:["component-app-assembly"],testCommands:{"transform-wire":TransformWireScript,"outline-ownership":OutlineOwnershipScript},snapshotSqliteTests:["../../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"]});
