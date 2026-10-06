/** 🧾️ TypeScript twin of `🧪️tests/🧪️history-patch/🦀️.rs`, driven by the SAME fixture (`🧫️fixtures/🧫️history-patch/🔣️.json`).
 *
 * 🔍️ Independent where it counts: the schema of record (`🧬️schema/🔣️history-patch/🔣️.json`) is compiled by Ajv, a
 * third-party validator sharing no line with the Rust decoder, and every valid patch must also satisfy the TS twin's
 * own row key ({@link historyEntryKey}) the hosts fold under.
 */
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🧫️history-patch/🔣️.json";
import schema from "../../🧬️schema/🔣️history-patch/🔣️.json";
import { historyEntryKey, type HistoryPatch } from "../../🟦️.ts";

type ValidCase = { readonly id: string; readonly patch: HistoryPatch; readonly keys: readonly string[] };
type InvalidCase = { readonly id: string; readonly patch: unknown; readonly reason: string };

const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const schemaId = schema.$id;
const validatePatch = ajv.getSchema(`${schemaId}#/definitions/HistoryPatch`)!;

describe("history patch wire", () => {

  it("accepts every valid patch and folds its rows under the fixture keys", () => {
    for (const valid of fixture.valid as readonly ValidCase[]) {
      expect(validatePatch(valid.patch), `${valid.id}: ${JSON.stringify(validatePatch.errors)}`).toBe(true);
      expect((valid.patch.upserts ?? []).map(historyEntryKey), valid.id).toEqual(valid.keys);
    }
  });

  it("refuses every invalid patch", () => {
    for (const invalid of fixture.invalid as readonly InvalidCase[]) {
      expect(validatePatch(invalid.patch), `${invalid.id}: ${invalid.reason}`).toBe(false);
    }
  });
});
