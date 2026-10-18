/** 🧪️ The GIF 89a diff parser accepts and refuses exactly what the independent JSON Schema oracle (Ajv over the committed schema and its snapshot sibling) accepts and refuses. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../../🔣️.json";
import snapshot from "../../../📸️snapshot/🔣️.json";
import vectors from "../🧫️fixtures/🔣️.json";
import { parseGifDiff } from "../../🟦️.ts";

test("GIF 89a diff parser agrees with the independent JSON Schema oracle on every vector", () => {
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(snapshot);
  const oracle = ajv.compile(schema);
  for (const row of vectors.valid) {
    expect(oracle(row.value), `${row.name}: ${JSON.stringify(oracle.errors)}`).toBe(true);
    expect(parseGifDiff(row.value), row.name).toEqual<typeof row.value>(row.value);
  }
  for (const row of vectors.invalid) {
    expect(oracle(row.value), row.name).toBe(false);
    expect(() => parseGifDiff(row.value), row.name).toThrow();
  }
});
