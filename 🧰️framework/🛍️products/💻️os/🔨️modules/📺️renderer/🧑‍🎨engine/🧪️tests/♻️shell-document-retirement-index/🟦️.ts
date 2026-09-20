/** ♻️ Neutral oracle for the bounded Shell retained-document occupied index. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, test } from "vitest";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "♻️shell-document-retirement-index", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "♻️shell-document-retirement-index", "🔣️.json"), "utf8"));

const firstAtOrAfter = (occupied: Set<number>, cursor: number, capacity: number): number | undefined => {
  for (let offset = 0; offset < capacity; offset += 1) {
    const index = (cursor + offset) % capacity;
    if (occupied.has(index)) return index;
  }
  return undefined;
};

const firstVacant = (occupied: Set<number>, capacity: number): number | undefined => {
  for (let index = 0; index < capacity; index += 1) if (!occupied.has(index)) return index;
  return undefined;
};

describe("♻️ Shell document retirement occupied index", () => {
  test("the language-neutral lifecycle satisfies its schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.wordCount).toBe(Math.ceil(fixture.capacity / fixture.wordBits));
    expect(fixture.indexBytes).toBe(fixture.wordCount * BigUint64Array.BYTES_PER_ELEMENT + BigUint64Array.BYTES_PER_ELEMENT);
  });

  test("an independent Set oracle preserves cursor fairness, vacancy reuse, epochs, and terminal count", () => {
    const occupied = new Set<number>(fixture.initial.occupied);
    const epochs = new Map<number, number>(fixture.initial.occupied.map((slot: number) => [slot, 1]));
    let cursor = fixture.initial.cursor;
    for (const step of fixture.steps) {
      if (step.op === "closePage") {
        expect(firstAtOrAfter(occupied, cursor, fixture.capacity)).toBe(step.slot);
        cursor = (step.slot + 1) % fixture.capacity;
        if (step.terminal) occupied.delete(step.slot);
        expect(cursor).toBe(step.nextCursor);
      } else {
        expect(firstVacant(occupied, fixture.capacity)).toBe(step.slot);
        const epoch = (epochs.get(step.slot) ?? 0) + 1;
        epochs.set(step.slot, epoch);
        occupied.add(step.slot);
        expect(epoch).toBe(step.epoch);
      }
      expect(occupied.size).toBe(step.occupiedCount);
    }
    expect(occupied.size).toBe(0);
  });
});
