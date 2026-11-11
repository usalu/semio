#!/usr/bin/env repo
/** 📦️ txt TypeScript artifact package router. */
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
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-txt", {suites:["🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/✏️set-line/🧪️tests/🔣️transport/🟦️.ts"],commands: router});






