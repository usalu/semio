import { describe, expect, it } from "vitest";
import { keepArtifactOpeningV1 } from "../../🧱️elements/🏛️ShellHost/🗂️local-catalog/📥️artifact/💾️opening/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import opening from "./🧫️fixtures/💾️opening.json";

describe("durable natural-file opening", () => {
  for (const row of opening.cases) it(row.name, async () => {
    const operations: string[] = [], controller = new AbortController();
    let current = true;
    const result = await keepArtifactOpeningV1({ name: fixture.name, schema: fixture.schema, surface: fixture.surface, dataDir: "/device", nowMs: 10,
      signal: controller.signal, current: () => current, progress: () => undefined,
      identity: async () => { operations.push("identity"); return { parent_document_id: row.failure === "identity" ? null : fixture.documentId }; },
      archive: async () => { operations.push("archive"); return fixture.archive; },
      write: async () => { operations.push("write"); if (row.failure === "cancel") controller.abort(); return row.failure !== "write"; },
      attach: async () => { operations.push("attach"); if (row.failure === "stale") current = false; return row.failure === "attach" ? null : { runtimeKey: fixture.documentId, clientInstanceId: "opening" }; },
      record: async () => { operations.push("record"); return row.failure !== "record"; },
      publish: () => { operations.push("publish"); return row.failure !== "publish"; },
      close: async () => { operations.push("close"); }, unlist: async () => { operations.push("unlist"); }, report: () => undefined,
    }).catch(() => false);
    expect(result).toBe(row.opened);
    expect(operations).toEqual(row.operations);
  });
});
