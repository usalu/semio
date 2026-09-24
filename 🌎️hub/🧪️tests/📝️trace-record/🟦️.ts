// #region Header
/**
 * 📝️ Third-party oracle for the hub's structured trace lines: every record-line vector the Rust
 * renderer is held to (`⏱️trace/📝️record/🧫️fixtures`) validates with Ajv against the language-agnostic
 * record schema (`⏱️trace/📝️record/🧬️schema`), every invalid vector is refused, and the vocabulary's
 * events all fit the schema's event grammar.
 * @see ../../../🧰️framework/🔨️modules/⏱️trace/📝️record/🦀️.rs
 */
// #endregion Header

import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const trace = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "🧰️framework", "🔨️modules", "⏱️trace");
const read = (...path: string[]) => JSON.parse(readFileSync(join(trace, ...path), "utf8"));

describe("structured trace record schema", () => {
  const schema = read("📝️record", "🧬️schema", "🔣️.json");
  const vectors = read("📝️record", "🧫️fixtures", "🔣️.json") as { valid: Array<{ record: unknown; line: string }>; invalid: string[] };
  const vocabulary = read("🧫️fixtures", "🛰️span-vocabulary", "🔣️.json") as { events: string[]; outcomes: Array<{ name: string }> };
  const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);

  it("admits every rendered vector and reads it back as its record", () => {
    for (const vector of vectors.valid) {
      const parsed = JSON.parse(vector.line);
      expect(validate(parsed), JSON.stringify(validate.errors)).toBe(true);
      expect(parsed).toEqual(vector.record);
    }
  });

  it("refuses every invalid vector", () => {
    for (const line of vectors.invalid) expect(validate(JSON.parse(line)), line).toBe(false);
  });

  it("admits every declared event and outcome", () => {
    for (const event of vocabulary.events) for (const { name } of vocabulary.outcomes) expect(validate({ level: "info", event, outcome: name })).toBe(true);
  });
});
