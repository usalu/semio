/** 🎨️ Canonical colorInputSelfTests: the TypeScript half of the colour law of the `color_input` recipe, answering the
 * very same `🧫️fixtures/🧫️color-input/🔣️.json` rows the Rust `🔬️component-unit` law answers through
 * `ui_color_hex`/`parse_ui_color_hex`. Ajv validates the fixture against its schema and color-string is the independent
 * oracle: it parses every accepted text to the same channels and alpha, refuses every refused one, and prints the same
 * hex (upper-case, alpha omitted when opaque). */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { parseUiColorHex, uiColorHex } from "../../🧩️component/🟦️.ts";

type Fixture = {
  readonly hex: readonly { readonly case: string; readonly rgba: readonly number[]; readonly alpha: boolean; readonly expected: string }[];
  readonly parse: readonly { readonly case: string; readonly text: string; readonly expected: readonly number[] | null }[];
};

type ColorString = { readonly get: { rgb(text: string): number[] | null }; readonly to: { hex(red: number, green: number, blue: number, alpha?: number): string } };

const read = (path: string): unknown => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));

/** 🎨️ Answers every shared fixture row, returning how many assertions the corpus carried. */
export function colorInputSelfTests(): number {
  const require = createRequire(import.meta.url);
  const Ajv2020 = require("ajv/dist/2020").default;
  const colorString = require("color-string") as ColorString;
  const fixture = read("../../🧫️fixtures/🧫️color-input/🔣️.json") as Fixture;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(read("../../🧫️fixtures/🧫️color-input/🧬️schema/🔣️.json"));
  assert(validate(fixture), JSON.stringify(validate.errors));
  let checks = 1;
  for (const row of fixture.hex) {
    assert.equal(uiColorHex(row.rgba, row.alpha), row.expected, row.case);
    const channel = (index: number) => Math.round(Math.min(1, Math.max(0, row.rgba[index] ?? (index === 3 ? 1 : 0))) * 255);
    const opaque = channel(3) === 255;
    const oracle = colorString.to.hex(channel(0), channel(1), channel(2), row.alpha ? channel(3) / 255 : 1) + (row.alpha && opaque ? "FF" : "");
    assert.equal(uiColorHex(row.rgba, row.alpha).toUpperCase(), oracle, `${row.case}: color-string oracle`);
    checks += 2;
  }
  for (const row of fixture.parse) {
    assert.deepEqual(parseUiColorHex(row.text), row.expected, row.case);
    const trimmed = row.text.trim();
    const oracle = colorString.get.rgb(`#${trimmed.startsWith("#") ? trimmed.slice(1) : trimmed}`);
    assert.deepEqual(oracle && [oracle[0]! / 255, oracle[1]! / 255, oracle[2]! / 255, oracle[3]!], row.expected, `${row.case}: color-string oracle`);
    checks += 2;
  }
  return checks;
}
