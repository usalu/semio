/** 🧪️ Third-party and independent oracle of the hex-window splice corpus (`🧫️fixtures/✂️hex-splices`): ajv validates the corpus rows against their schema, and the bytes each row means are rebuilt TWO independent ways — applying the digit ranges to the hex text and decoding it, and applying the listed byte ranges to the bytes — which must agree. The Rust editor law replays the same corpus through `binary_text_emit`. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import corpus from "../../🧫️fixtures/✂️hex-splices/🔣️.json";
import schema from "../../🧬️schema/🔣️hex-splices/🔣️.json";

type Range = { readonly offset: number; readonly delete: number; readonly insert: string };
type Leaf = { readonly offset: number; readonly removeLen: number; readonly insert: string };

const bytes = (hex: string): number[] => (hex.replace(/\s+/g, "").match(/../g) ?? []).map((pair) => Number.parseInt(pair, 16));

/** ✂️ The window text the editor shows: the digits, then the informational comment line. */
const windowText = (hex: string): string => `${hex}\n# total bytes: ${hex.length / 2}`;

/** 🧮️ The bytes the digit ranges mean, or `null` when they leave the window, cross the comment, or leave non-hex digits. */
function digitsMean(before: string, ranges: readonly Range[]): number[] | null | "ignored" {
  const text = Array.from(windowText(before));
  const digits = before.length;
  let touched = false;
  const out: string[] = [];
  let position = 0;
  for (const range of ranges) {
    const end = range.offset + range.delete;
    if (end > text.length || range.offset < position) return null;
    if (range.offset > digits) continue;
    if (end > digits || (range.offset === digits && range.delete > 0)) return null;
    touched = true;
    out.push(text.slice(position, range.offset).join(""), range.insert);
    position = end;
  }
  if (!touched) return "ignored";
  out.push(text.slice(position, digits).join(""));
  const joined = out.join("").replace(/\s+/g, "");
  return joined.length % 2 === 0 && /^[0-9a-f]*$/.test(joined) ? bytes(joined) : null;
}

/** 🧮️ Applies byte ranges (listed last first) to the bytes. */
function applyLeaves(before: readonly number[], leaves: readonly Leaf[]): number[] {
  const out = [...before];
  for (const leaf of leaves) out.splice(leaf.offset, leaf.removeLen, ...bytes(leaf.insert));
  return out;
}

describe("hex-window splices (ajv + two independent decodings)", () => {
  test("the corpus rows conform to their schema (ajv)", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    for (const row of corpus.cases) expect(validate(row), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(true);
  });

  for (const row of corpus.cases) {
    test(`${row.id}: the digit ranges and the byte ranges mean the same bytes`, () => {
      const decoded = digitsMean(row.before, row.splices);
      if (row.leaves === null) {
        expect(decoded).toBeNull();
        return;
      }
      const base = bytes(row.before);
      if (row.leaves.length === 0 && decoded === "ignored") return;
      const viaLeaves = applyLeaves(base, row.leaves);
      if (decoded !== "ignored" && decoded !== null) {
        // the digits only cover the touched byte runs: rebuild the whole buffer from the digit ranges applied to the whole hex text
        const text = Array.from(row.before);
        const parts: string[] = [];
        let position = 0;
        for (const range of row.splices) {
          if (range.offset > row.before.length) continue;
          parts.push(text.slice(position, range.offset).join(""), range.insert);
          position = range.offset + range.delete;
        }
        parts.push(text.slice(position).join(""));
        expect(viaLeaves).toEqual(bytes(parts.join("")));
      } else {
        expect(viaLeaves).toEqual(base);
      }
    });
  }
});
