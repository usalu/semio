#!/usr/bin/env repo
/** 📦️ pdf TypeScript artifact package router. */
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
await runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/stdio-pdf", { bundleRuntime: "runtime", suites: [
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🧪️tests/🟦️.ts",
  "🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧩️cos/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🌈️color/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🔤️font/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖋️content/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🖼️resource/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🎯️navigation/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📌️annotation/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📇️metadata/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📝️form/🟦️.ts",
  "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📄️document/🟦️.ts",
"🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🧪️tests/🔣️transport/🟦️.ts"
],commands: router});






