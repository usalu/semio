#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🔱️ Trinity TypeScript package. */
import { join } from "node:path";
import { runCmd } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class TestScript extends BundleScript {
  run(): void {
    const artifacts = join(this.repoRoot, "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts");
    const tests = ["🔌️jack", "♻️rewriting"].flatMap((artifact) => {
      const subset = join(artifacts, artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any");
      return [join(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"), join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts")];
    });
    runCmd(process.execPath, ["test", ...tests]);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
