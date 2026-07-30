#!/usr/bin/env bun
/** 🃏 `@semio-tech/graph-dsl` — shared Jack query language over queryable graphs. */
import { BundleScript, ScriptRouter, resolveTestLevel, runCargoTestBudgeted, runBundleScriptMain } from "../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["mathematical_graph_dsl"], join(this.root, "rs"), rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url);
