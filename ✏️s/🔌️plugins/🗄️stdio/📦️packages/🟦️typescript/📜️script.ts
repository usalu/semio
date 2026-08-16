#!/usr/bin/env bun
/** 🗄️ stdio TypeScript package */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    resolveTestLevel(segments);
    const catalog = JSON.parse(readFileSync(join(this.root, "../../📇️registry/📇️catalog.json"), "utf8")) as { stdio_roster: Record<string, unknown> };
    const facade = await import("./📦️index.ts");
    const missing = Object.keys(catalog.stdio_roster).filter((id) => !(id in facade));
    if (missing.length > 0) throw new Error(`[stdio] missing TypeScript artifact exports: ${missing.join(", ")}`);
    console.log(`[stdio] TypeScript facade exposes ${Object.keys(catalog.stdio_roster).length} catalog artifacts.`);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
