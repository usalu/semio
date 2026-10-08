#!/usr/bin/env bun
/** 📦️ Block 3D TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts";
import {BundleScript} from "./../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {resolve} from "node:path";
import {runOwnedCommand} from "./../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
/** 🧪️ Runs the artifact-owned original physical transport corpus with cancellation and progress. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length === 1 && segments[0] === "physical-codecs") {
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "./../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🧪️tests/🔣️transport/🟦️.ts")], this.repoRoot, "owned-physical-codecs", 120_000);
      return;
    }
    throw Error("verify physical-codecs");
  }
}

await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/block-3d", {suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🧪️tests/🔣️transport/🟦️.ts"],commands:{verify:OwnedVerifyScript}});






