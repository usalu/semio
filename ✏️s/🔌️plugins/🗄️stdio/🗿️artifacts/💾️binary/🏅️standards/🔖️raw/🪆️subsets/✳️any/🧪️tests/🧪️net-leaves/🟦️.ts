/** 🧪️ Third-party oracle of the hex-window net-leaves corpus (`🧫️fixtures/🧫️net-leaves`): ajv validates individual mutation records and jsdiff's array diff finds the bytes both buffers share at either end; the ONE range replacement an Apply means
 * follows from those ends alone, and an Apply that changes no byte means none (`null`). The Rust editor law replays the same
 * corpus through `binary_text_emit`. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { diffArrays } from "diff";
import corpus from "../../🧫️fixtures/🧫️net-leaves/🔣️.json";
import schema from "../../🧬️schema/🔣️net-leaves/🔣️.json";

type Leaf = { readonly kind: "replace-byte-range"; readonly offset: number; readonly removeLen: number; readonly insert: string };

const bytes = (hex: string): number[] => (hex.match(/../g) ?? []).map((pair) => Number.parseInt(pair, 16));
const hex = (values: readonly number[]): string => values.map((value) => value.toString(16).padStart(2, "0")).join("");

/** 🧮️ The ONE range replacement the shared ends of the two buffers mean, or `null` when no byte changed. */
function netLeaf(beforeHex: string, afterHex: string): Leaf | null {
  const before = bytes(beforeHex);
  const after = bytes(afterHex);
  const chunks = diffArrays(before, after);
  const kept = (chunk: (typeof chunks)[number] | undefined) => (chunk !== undefined && !chunk.added && !chunk.removed ? chunk.count ?? 0 : 0);
  const prefix = kept(chunks[0]);
  const suffix = chunks.length > 1 ? kept(chunks.at(-1)) : 0;
  const removeLen = before.length - prefix - suffix;
  const insert = after.slice(prefix, after.length - suffix);
  return removeLen === 0 && insert.length === 0 ? null : { kind: "replace-byte-range", offset: prefix, removeLen, insert: hex(insert) };
}

describe("hex-window net leaf (jsdiff oracle)", () => {
  test("actual mutation records conform to their domain schema (ajv)", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    for (const row of corpus.cases) for (const leaf of row.leaf === null ? [] : [row.leaf]) expect(validate(leaf), JSON.stringify(validate.errors)).toBe(true);
  });

  for (const row of corpus.cases) {
    test(`${row.id}: the corpus leaf is the range the shared ends mean`, () => {
      expect(netLeaf(row.before, row.after)).toEqual(row.leaf as Leaf | null);
    });
  }
});
