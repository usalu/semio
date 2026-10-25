import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import fixture from "../🧫️fixtures/🧮️plain-elements/🔣️.json";
import schema from "../../🧬️schema/🧮️plain-elements/🔣️.json";

/** 🧮️ Validates padded plain-composite retirement with an independent relational oracle. */
test("plain composite retirement preserves full padded width and owning controls", () => {
  const valid = new Ajv({ strict: true }).compile(schema);
  expect(valid(fixture)).toBe(true);
  expect(valid({ ...fixture, extra: true })).toBe(false);
  expect(valid({ ...fixture, widthCases: [{ ...fixture.widthCases[0], needsDrop: 0 }] })).toBe(false);
  const db = new Database(":memory:");
  try {
    const width = db.query<{ width: number }, { $size: number; $drop: number; $components: string }>(`
      SELECT CASE WHEN $drop OR EXISTS(SELECT 1 FROM json_each($components) WHERE value = 0)
        THEN 0 ELSE $size END AS width
    `);
    const pages = db.query<{ pages: number }, { $count: number; $width: number; $work: number }>(`
      SELECT ($count * $width + $work - 1) / $work AS pages
    `);
    for (const row of fixture.widthCases) {
      const actual = row.needsDrop || row.componentBytes.includes(0) ? 0 : row.sizeBytes;
      expect(actual).toBe(row.expectedBytes);
      expect(width.get({ $size: row.sizeBytes, $drop: Number(row.needsDrop), $components: JSON.stringify(row.componentBytes) })?.width).toBe(row.expectedBytes);
      if (row.expectedBytes) for (const count of fixture.counts) {
        expect(pages.get({ $count: count, $width: row.expectedBytes, $work: fixture.workBytes })?.pages).toBe(Math.ceil(count * row.expectedBytes / fixture.workBytes));
      }
    }
  } finally {
    db.close();
  }
});

/** 🪄️ Pins the stash behavior at the generalized defining derive owner. */
test("RetireOwned derive declares actual composite width through its selected value crate", () => {
  const source = readFileSync(new URL("../../../✨️derive/🧬️retained-clone/🦀️.rs", import.meta.url), "utf8");
  const derive = source.slice(source.indexOf("pub fn expand_retire_owned("), source.indexOf("fn destructure_fields"));
  expect(derive).toContain("let components = field_types(input);");
  expect(derive).toContain("fn retirement_element_copy_bytes() -> usize");
  expect(derive).toContain("#value_crate::retirement::composite_copy_bytes::<Self>");
  expect(derive).toContain("#(<#components as #value_crate::retirement::RetireOwned>::retirement_element_copy_bytes()),*");
  expect(derive).toContain("crate::component::declared_value_crate(input)?");
});

/** 🧭️ Admits the original invocation only after every registered class is initialized. */
test("Value owner initializes every script before receiving the original invocation", () => {
  const source = readFileSync(new URL("../../../📦️packages/🦀️rust/📜️script.ts", import.meta.url), "utf8");
  const receive = source.lastIndexOf("await receiveScriptProcessInvocation(");
  const definitions = [...source.matchAll(/^class \w+/gm)].map(match => match.index);
  expect(definitions.length).toBeGreaterThan(0);
  expect(receive).toBeGreaterThan(Math.max(...definitions));
  expect(source).toContain('.register("test-plain-element-retirement-source",PlainElementRetirementSourceScript)');
});
