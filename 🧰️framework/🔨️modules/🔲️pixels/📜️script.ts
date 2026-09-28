#!/usr/bin/env bun
/** 🔲️ Pixel editing verification through the shared workspace task runner. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd, runCargoTestBudgeted } from "../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const [language = "typescript", ...rest] = segments;
    if (language === "typescript") {
      const host = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing");
      runCmd(process.execPath, [join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions",join(this.root,"✍️editing/🟦️.ts"),join(this.root,"🧩️compositing/🟦️.ts"),join(this.root,"🧩️compositing/📐️frames/🟦️.ts"),join(this.root,"🧩️compositing/🗂️layers/🟦️.ts"),join(host,"🟦️.ts")], { cwd: this.repoRoot });
      runCmd(process.execPath, ["test", join(this.root, "🧩️compositing/📐️frames/🧪️tests/🟦️.ts"), join(this.root, "✍️editing/🧪️tests/🟦️.ts"), join(this.root,"🧩️compositing/🧪️tests/🟦️.ts"),join(this.root,"🧩️compositing/🗂️layers/🧪️tests/🟦️.ts"), join(host,"🧪️tests/🟦️.ts"), ...rest], { cwd: this.repoRoot });
    }
    else if (language === "rust") await runCargoTestBudgeted(["semio-framework-pixels"], this.repoRoot, rest);
    else throw new Error("Expected test typescript or test rust");
  }
}

await runBundleScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), import.meta.url, { defaultCommand: "test" });
