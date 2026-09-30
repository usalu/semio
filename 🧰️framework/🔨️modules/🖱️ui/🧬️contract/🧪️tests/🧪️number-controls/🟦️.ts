/** 🎚️ Canonical numberControlsSelfTests: the TypeScript half of the shared number-control laws, answering the
 * very same `🧫️fixtures/🧫️number-controls/🔣️.json` rows the Rust `🔬️component-unit` law answers through
 * `snaps_are_valid` (limits)/`slider_pointer_value`/`slider_adjacent_snap`/`slider_key_value`/`format_ui_number_fixed`.
 * Ajv validates the fixture against its schema and decimal.js is the independent rounding and ladder oracle: it rounds the EXACT binary
 * expansion of every value half away from zero, so a twin that rounded the shortest decimal instead would disagree
 * on `1.005`/`2.675`. */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { SLIDER_PAGE_STEPS, sliderAdjacentSnap, sliderKeyValue, sliderPointerValue, UI_NUMBER_PRECISION_MAX, uiNumberKeyValue, type SliderKey } from "../../🧩️component/🟦️.ts";
import { snapsAreValid } from "../../🛡️limits/🟦️.ts";
import { formatUiNumber, formatUiNumberFixed, roundUiNumber } from "../../🔢️number-format/🟦️.ts";
import { uiAccessibilityValueV1 } from "../../♿️accessibility/🟦️.ts";
import type { Component } from "../../../../🛂️manifest/🟦️.ts";

type Fixture = {
  readonly validity: readonly { readonly case: string; readonly min: number; readonly max: number; readonly snaps: readonly number[]; readonly valid: boolean }[];
  readonly pointer: readonly { readonly case: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly snaps: readonly number[]; readonly expected: number }[];
  readonly adjacent: readonly { readonly case: string; readonly current: number; readonly snaps: readonly number[]; readonly forward: boolean; readonly expected: number | null }[];
  readonly keys: readonly { readonly case: string; readonly current: number; readonly min: number | null; readonly max: number | null; readonly step: number; readonly snaps: readonly number[]; readonly key: SliderKey; readonly large: boolean; readonly expected: number }[];
  readonly fixed: readonly { readonly case: string; readonly value: number; readonly precision: number; readonly expected: string; readonly rounded: number }[];
  readonly valueTexts: readonly { readonly case: string; readonly component: Record<string, unknown>; readonly valueText: string }[];
};

/** 🧾️ Normalizes a sparse fixture component the way the typed wire decoder does (absent optionals are `null`). */
function normalizedComponent(component: Record<string, unknown>): Component {
  return (component.type === "input" ? { kind: "text", placeholder: null, commit: null, min: null, max: null, step: null, accept: null, precision: null, ...component } : { min: null, max: null, precision: null, ...component }) as Component;
}

const read = (path: string): unknown => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));

/** 🧮️ decimal.js oracle: the exact binary expansion rounded half away from zero, unsigned zero, `toFixed`'s exponent fallback. */
function decimalOracle(Decimal: { new (value: string): { toFixed(digits: number, rounding: number): string }; readonly ROUND_HALF_UP: number }, value: number, precision: number): string {
  if (Math.abs(value) >= 1e21) return formatUiNumber(value);
  const text = new Decimal(value.toPrecision(100)).toFixed(Math.min(precision, UI_NUMBER_PRECISION_MAX), Decimal.ROUND_HALF_UP);
  return /^-[0.]*$/.test(text) ? text.slice(1) : text;
}

type DecimalValue = { plus(other: DecimalValue | string): DecimalValue; minus(other: DecimalValue | string): DecimalValue; times(other: DecimalValue | string): DecimalValue; div(other: DecimalValue | string): DecimalValue; floor(): DecimalValue; ceil(): DecimalValue; isInteger(): boolean; toNumber(): number };

/** 🪜️ decimal.js ladder oracle for an arrow key: the rung count and rung value computed in exact decimal arithmetic, then clamped. */
function ladderOracle(Decimal: new (value: string) => DecimalValue, row: Fixture["keys"][number]): number {
  const rung = Number.isFinite(row.step) && row.step > 0 ? String(row.step) : "1";
  const forward = row.key === "increment";
  const origin = String(row.min ?? 0);
  const position = new Decimal(String(row.current)).minus(origin).div(rung);
  const base = position.isInteger() ? position : forward ? position.floor() : position.ceil();
  const rungs = String(row.large ? SLIDER_PAGE_STEPS : 1);
  const value = new Decimal(origin).plus((forward ? base.plus(rungs) : base.minus(rungs)).times(rung)).toNumber();
  return Math.min(row.max ?? Number.POSITIVE_INFINITY, Math.max(row.min ?? Number.NEGATIVE_INFINITY, value));
}

/** 🎚️ Answers every shared fixture row, returning how many assertions the corpus carried. */
export function numberControlsSelfTests(): number {
  const require = createRequire(import.meta.url);
  const Ajv2020 = require("ajv/dist/2020").default;
  const Decimal = require("decimal.js");
  const fixture = read("../../🧫️fixtures/🧫️number-controls/🔣️.json") as Fixture;
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(read("../../🧫️fixtures/🧫️number-controls/🧬️schema/🔣️.json"));
  assert(validate(fixture), JSON.stringify(validate.errors));
  let checks = 1;
  for (const row of fixture.validity) {
    assert.equal(snapsAreValid(row.snaps, row.min, row.max), row.valid, row.case);
    checks++;
  }
  for (const row of fixture.pointer) {
    assert.equal(sliderPointerValue(row.value, row.min, row.max, row.step, row.snaps), row.expected, row.case);
    checks++;
  }
  for (const row of fixture.adjacent) {
    assert.equal(sliderAdjacentSnap(row.current, row.snaps, row.forward), row.expected, row.case);
    checks++;
  }
  for (const row of fixture.keys) {
    assert.equal(uiNumberKeyValue(row.current, row.min, row.max, row.step, row.snaps, row.key, row.large), row.expected, row.case);
    checks++;
    if (row.min != null && row.max != null) {
      assert.equal(sliderKeyValue(row.current, row.min, row.max, row.step, row.snaps, row.key, row.large), row.expected, `${row.case}: the slider law is the bounded number law`);
      checks++;
    }
    if (row.key === "increment" || row.key === "decrement") {
      assert.equal(ladderOracle(Decimal, row), row.expected, `${row.case}: decimal.js ladder oracle`);
      checks++;
    }
  }
  for (const row of fixture.fixed) {
    assert.equal(formatUiNumberFixed(row.value, row.precision), row.expected, row.case);
    assert.equal(decimalOracle(Decimal, row.value, row.precision), row.expected, `${row.case}: decimal.js oracle`);
    assert.equal(roundUiNumber(row.value, row.precision), row.rounded, row.case);
    checks += 3;
  }
  for (const row of fixture.valueTexts) {
    assert.equal(uiAccessibilityValueV1(normalizedComponent(row.component)).valueText, row.valueText, row.case);
    checks++;
  }
  return checks;
}
