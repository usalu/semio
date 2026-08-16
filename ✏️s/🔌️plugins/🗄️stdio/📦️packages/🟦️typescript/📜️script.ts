#!/usr/bin/env bun
/** 🗄️ stdio TypeScript package */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    resolveTestLevel(segments);
    const facade = await import("./📦️index.ts");
    const entries = Object.entries(facade);
    if (entries.length !== 36) throw new Error(`[stdio] expected 36 TypeScript definitions, got ${entries.length}.`);
    for (const [artifact, namespace] of entries) {
      const definition = (namespace as { definition?: { id?: string } }).definition;
      if (!definition || definition.id !== `s.stdio.${artifact}`) throw new Error(`[stdio] ${artifact} does not export its schema-owned ArtifactDefinition.`);
    }
    console.log(`[stdio] TypeScript facade exposes ${entries.length} schema-owned artifact definitions.`);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
