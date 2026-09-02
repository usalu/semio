#!/usr/bin/env bun
/** @emoji 🪟️ `@semio-tech/plugin-window-kits` task router: `bun ./📜️script.ts test|typecheck`. */

// 🏃️ `bun-types` isn't installed in this workspace; `import.meta.dir` (Bun's own runtime global) needs an
// ambient declaration rather than an `as any` cast at each call site — same gap `@semio-tech/ui-react`'s
// own `📜️script.ts` documents and works around identically.
declare global {
  interface ImportMeta {
    readonly dir: string;
  }
}
import { BundleScript, ScriptRouter, resolveTestLevel, runBunx, runBundleScriptMain, runVitest } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
