#!/usr/bin/env repo
/** 📦️ semio TypeScript artifact package router. */
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
/** 🧪️ Runs the artifact-owned original physical transport corpus with cancellation and progress. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-semio", {suites:[
"🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🧪️tests/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔺️geometry/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📃️media/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔗️relationships/🟦️.ts",
"🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/📝️text/📸️snapshot/🔣️json/🧪️tests/🔣️transport/🟦️.ts"
],commands: router});






