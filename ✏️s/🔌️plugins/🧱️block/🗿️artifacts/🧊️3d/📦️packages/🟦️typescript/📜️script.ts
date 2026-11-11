#!/usr/bin/env repo
/** 📦️ Block 3D TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🟦️typescript/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
/** 🧪️ Runs the artifact-owned original physical transport corpus with cancellation and progress. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/block-3d", {suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🧪️tests/🔣️transport/🟦️.ts"],commands: router});






