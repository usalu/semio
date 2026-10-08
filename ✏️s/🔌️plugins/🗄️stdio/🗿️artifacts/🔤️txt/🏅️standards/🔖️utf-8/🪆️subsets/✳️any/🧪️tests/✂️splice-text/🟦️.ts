/** 🧪️ Third-party and independent oracle of the plain-text splice corpus (`🧫️fixtures/✂️splice-text`): ajv validates every mutation payload against the leaf's JSON schema, and the text each row means is rebuilt by an independent scalar-array implementation. The Rust leaf law replays the same corpus through `splice-text` and the contract's `apply_draft_splices`. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import corpus from "../../🧫️fixtures/✂️splice-text/🔣️.json";
import schema from "../../🧬️schema/🧬️mutations/✂️splice-text/🧬️schema/🔣️.json";

type Splice = { readonly offset: number; readonly delete: number; readonly insert: string };

/** ✂️ The body the ranges mean, or `null` when a range leaves the text or overlaps the one before. */
function spliced(before: string, splices: readonly Splice[]): string | null {
  const scalars = Array.from(before);
  const parts: string[] = [];
  let position = 0;
  for (const splice of splices) {
    if (splice.offset < position || splice.offset + splice.delete > scalars.length) return null;
    parts.push(scalars.slice(position, splice.offset).join(""), splice.insert);
    position = splice.offset + splice.delete;
  }
  parts.push(scalars.slice(position).join(""));
  return parts.join("");
}

describe("plain-text splices (ajv + independent scalar oracle)", () => {
  test("every payload conforms to the leaf schema (ajv)", () => {
    const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
    for (const row of corpus.cases) expect(validate({ splices: row.splices }), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(true);
  });

  for (const row of corpus.cases) {
    test(`${row.id}: the ranges mean the corpus text`, () => {
      const text = spliced(row.before, row.splices);
      if (row.after === null) {
        // a refused row is outside the text, or switches an LF document to CRLF (the ending changes only through set-line-ending)
        if (text === null) return;
        expect(!row.before.includes("\r\n") && text.includes("\r\n")).toBe(true);
      } else {
        expect(text).toBe(row.after);
      }
    });
  }
});
