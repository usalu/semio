import { expect, test } from "vitest";
import { sha256 } from "@noble/hashes/sha2.js";
import { operationEntityId } from "../../🧬️schema/🪪️entity-id/🟦️.ts";
import fixture from "../../🧫️fixtures/🪪️entity-id/🔣️.json";

test("operation entity authority distinguishes writers and replays the same admission", () => {
  const ids = new Set<string>();
  const encoder = new TextEncoder();
  for (const row of fixture.rows) {
    const authority = { parentDocumentId: row.documentId, authoringSeed: row.authoringSeed };
    const id = operationEntityId(authority, row.prefix, row.localKey);
    const field = (value: string) => encoder.encode(value).length + ":" + value;
    const bytes = encoder.encode(field(row.documentId) + field(row.authoringSeed) + field(row.localKey));
    const oracle = row.prefix + "-" + Buffer.from(sha256(bytes)).toString("hex").slice(0, 16);
    expect(id).toBe(row.expected);
    expect(id).toBe(oracle);
    expect(operationEntityId(authority, row.prefix, row.localKey)).toBe(id);
    expect(ids.has(id)).toBe(false);
    ids.add(id);
  }
  expect(() => operationEntityId({ parentDocumentId: "doc", authoringSeed: "" }, "n", "nodes")).toThrow("admitted authoring seed");
});
