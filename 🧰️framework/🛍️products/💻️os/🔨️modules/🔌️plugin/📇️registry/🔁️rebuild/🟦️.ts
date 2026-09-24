/** 🔁️ The one registered all-plugin rebuild: every component's descriptor, the generated registry and its check, the
 * restaged `s` guests and the trusted catalog of every package, from one tree, in the order `🔣️.json` declares. Each step
 * is an existing product verb run through nx, so nx caching, per-target budgets and interruption apply unchanged; a step
 * that fails stops the chain before any later stage consumes its output. `--from <step>` resumes. */

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, getWorkspaceRoot, orchestratorBudgetOpts, runCmd } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🧱️ Where a step sits in the dependency order: descriptors feed the registry, the registry feeds the guests, the guests feed the catalog. */
export const REBUILD_STAGES = ["descriptors", "registry", "guests", "catalog"] as const;

/** 🪜️ One declared step of the chain. */
export type RebuildStepV1 = Readonly<{ id: string; stage: (typeof REBUILD_STAGES)[number]; command: readonly string[] }>;

/** 📜️ Reads and validates the declared chain: known schema, unique ids, nx commands, stages in dependency order. */
export function readRebuildChain(path = join(dirname(fileURLToPath(import.meta.url)), "🔣️.json")): readonly RebuildStepV1[] {
  const document = JSON.parse(readFileSync(path, "utf8")) as { schema?: unknown; steps?: unknown };
  if (document.schema !== "semio.plugin-registry.rebuild-chain/v1" || !Array.isArray(document.steps) || document.steps.length === 0) throw new Error("rebuild chain is not a semio.plugin-registry.rebuild-chain/v1 document");
  const ids = new Set<string>();
  let stage = 0;
  return Object.freeze(
    document.steps.map((value: any) => {
      const next = REBUILD_STAGES.indexOf(value?.stage);
      if (typeof value?.id !== "string" || ids.has(value.id) || next < stage || !Array.isArray(value.command) || value.command[0] !== "nx" || value.command.some((part: unknown) => typeof part !== "string"))
        throw new Error(`rebuild chain step ${String(value?.id)} is duplicated, out of stage order, or not an nx command`);
      ids.add(value.id);
      stage = next;
      return Object.freeze({ id: value.id, stage: value.stage, command: Object.freeze([...value.command]) }) as RebuildStepV1;
    }),
  );
}

/** 🏁️ `rebuild-all [--from <step>]`: runs every declared step in order with a numbered progress line per step. */
export class RebuildAllScript extends BundleScript {
  run(segments: string[]): void {
    const chain = readRebuildChain();
    const flag = segments.indexOf("--from");
    const from = flag === -1 ? chain[0]!.id : segments[flag + 1];
    const start = chain.findIndex((step) => step.id === from);
    if (start === -1 || (flag !== -1 && segments.length !== 2) || (flag === -1 && segments.length !== 0)) throw new Error(`usage: rebuild-all [--from <${chain.map((step) => step.id).join("|")}>]`);
    const repoRoot = getWorkspaceRoot();
    for (const [index, step] of chain.entries()) {
      if (index < start) continue;
      const started = Date.now();
      console.log(`rebuild-all ${index + 1}/${chain.length} ${step.id} (${step.stage}) started`);
      runCmd("bun", [...step.command], { cwd: repoRoot, ...orchestratorBudgetOpts() });
      console.log(`rebuild-all ${index + 1}/${chain.length} ${step.id} done in ${Math.round((Date.now() - started) / 1000)} s`);
    }
  }
}
