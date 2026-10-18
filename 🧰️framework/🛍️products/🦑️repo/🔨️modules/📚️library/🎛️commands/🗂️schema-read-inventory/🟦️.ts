import { resolve } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🗂️ Produces original reader evidence with caller-owned reporting and interruptible discovery. */
export class SchemaReadInventoryScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length && (segments.length !== 2 || segments[0] !== "--report")) throw new Error("Expected schema-read-inventory [--report <path>]");
    const { inventorySchemaValidationReads } = await import("../../🔍️discovery/🧾️schema-scope-catalog/🟦️.ts"), controller = new AbortController(), cancel = (): void => controller.abort(new Error("Schema reader inventory cancelled"));
    process.once("SIGINT", cancel);
    try {
      const index = await inventorySchemaValidationReads(this.repoRoot, undefined, { checkCancellation: () => { if (controller.signal.aborted) throw controller.signal.reason; }, onProgress: row => { if (row.completed === 0 || row.completed % 1024 === 0 || row.phase === "inspect" && row.completed === row.total) console.log(`[schema-read-inventory] ${row.phase} completed=${row.completed} total=${row.total ?? "unknown"}`); } });
      if (segments.length) {
        const target = resolve(this.repoRoot, segments[1]!);
        if (target.length > 256 || [...target].length > 256) throw new Error("Schema reader report path exceeds 256 characters");
        mkdirSync(resolve(target, ".."), { recursive: true }); writeFileSync(target, JSON.stringify(index, null, 2) + "\n");
      }
    } finally { process.removeListener("SIGINT", cancel); }
  }
}
