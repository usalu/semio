/** 🧪️ Third-party oracle of the plain-text net-leaves corpus (`🧫️fixtures/🧫️net-leaves`): ajv validates individual mutation records and jsdiff's array diff finds the lines both texts share at either end; the leaves an Apply means follow from those ends
 * alone (paired lines that differ are re-set, surplus lines removed last first or inserted), and an Apply that changes the line
 * ending or the terminator is the whole-buffer lowering (`null`). The Rust editor law replays the same corpus through `txt_emit`. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { diffArrays } from "diff";
import corpus from "../../🧫️fixtures/🧫️net-leaves/🔣️.json";
import schema from "../../🧬️schema/🔣️net-leaves/🔣️.json";

type Leaf = { readonly kind: "set-line" | "insert-line"; readonly index: number; readonly text: string } | { readonly kind: "remove-line"; readonly index: number };

/** 📄️ The lines, terminator and line ending of a body. */
function shape(body: string): { readonly lines: string[]; readonly trailing: boolean; readonly ending: string } {
  const ending = body.includes("\r\n") ? "\r\n" : "\n";
  const trailing = body !== "" && body.endsWith(ending);
  const lines = body === "" ? [] : (trailing ? body.slice(0, -ending.length) : body).split(ending);
  return { lines, trailing, ending };
}

/** ✂️ How many lines the two bodies share at their start and at their end, by jsdiff's own array diff. */
function sharedEnds(before: readonly string[], after: readonly string[]): { readonly prefix: number; readonly suffix: number } {
  const chunks = diffArrays(before as string[], after as string[]);
  const kept = (chunk: (typeof chunks)[number] | undefined) => (chunk !== undefined && !chunk.added && !chunk.removed ? chunk.count ?? 0 : 0);
  return { prefix: kept(chunks[0]), suffix: chunks.length > 1 ? kept(chunks.at(-1)) : 0 };
}

/** 🧮️ The net line leaves the shared ends mean, or `null` for a change of shape. */
function netLeaves(beforeBody: string, afterBody: string): Leaf[] | null {
  const before = shape(beforeBody);
  const after = shape(afterBody);
  if (before.trailing !== after.trailing || before.ending !== after.ending) return null;
  const { prefix, suffix } = sharedEnds(before.lines, after.lines);
  const oldMiddle = before.lines.slice(prefix, before.lines.length - suffix);
  const newMiddle = after.lines.slice(prefix, after.lines.length - suffix);
  const paired = Math.min(oldMiddle.length, newMiddle.length);
  const leaves: Leaf[] = [];
  for (let offset = 0; offset < paired; offset += 1) if (oldMiddle[offset] !== newMiddle[offset]) leaves.push({ kind: "set-line", index: prefix + offset, text: newMiddle[offset]! });
  for (let offset = oldMiddle.length - 1; offset >= paired; offset -= 1) leaves.push({ kind: "remove-line", index: prefix + offset });
  for (let offset = paired; offset < newMiddle.length; offset += 1) leaves.push({ kind: "insert-line", index: prefix + offset, text: newMiddle[offset]! });
  return leaves;
}

describe("plain-text net leaves (jsdiff oracle)", () => {
  test("actual mutation records conform to their domain schema (ajv)", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    for (const row of corpus.cases) for (const leaf of row.leaves ?? []) expect(validate(leaf), JSON.stringify(validate.errors)).toBe(true);
  });

  for (const row of corpus.cases) {
    test(`${row.id}: the corpus leaves are the leaves the shared ends mean`, () => {
      expect(netLeaves(row.before, row.after)).toEqual(row.leaves as Leaf[] | null);
    });
  }
});
