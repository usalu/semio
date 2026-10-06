/** 🎚️ Canonical numberControlsSelfTests: the TypeScript half of the shared number-control laws, answering the
 * very same `🧫️fixtures/🧫️number-controls/🔣️.json` rows the Rust `🔬️component-unit` law answers through
 * `snaps_are_valid` (limits)/`slider_axis_position`/`slider_axis_value`/`dial_angle`/`dial_position`/`slider_pointer_value`/
 * `slider_adjacent_snap`/`slider_key_value`/`ui_number_display_text`/`ui_number_typed_value`/`ui_number_crossed_bound`/
 * `format_ui_number_fixed`/`number_range_is_valid`. Ajv validates the fixture against its schema; decimal.js is the independent
 * rounding and ladder oracle (it rounds the EXACT binary expansion of every value half away from zero, so a twin that rounded the
 * shortest decimal instead would disagree on `1.005`/`2.675`); d3-scale's `scaleLog`/`scaleLinear` are the independent axis and
 * display-factor oracles. */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { scaleLinear, scaleLog } from "d3-scale";
import type { UiNumberLimits, UiNumberScale } from "@semio-tech/framework";
import { dialAngle, dialPosition, SLIDER_PAGE_STEPS, sliderAdjacentSnap, sliderAxisPosition, sliderAxisValue, sliderKeyValue, sliderPointerValue, UI_NUMBER_PRECISION_MAX, uiNumberCrossedBound, uiNumberDisplay, uiNumberDisplayText, uiNumberFieldKey, uiNumberKeyValue, uiNumberTypedValue, type SliderKey } from "../../🧩️component/🟦️.ts";
import { numberRangeIsValid, snapsAreValid } from "../../🛡️limits/🟦️.ts";
import { formatUiNumber, formatUiNumberFixed, roundUiNumber } from "../../🔢️number-format/🟦️.ts";
import { uiAccessibilityValueV1 } from "../../♿️accessibility/🟦️.ts";
import type { Component } from "../../../../🛂️manifest/🟦️.ts";

type Fixture = {
  readonly validity: readonly { readonly case: string; readonly min: number; readonly max: number; readonly snaps: readonly number[]; readonly valid: boolean }[];
  readonly axis: readonly { readonly case: string; readonly value: number; readonly min: number; readonly max: number; readonly scale: UiNumberScale; readonly position: number }[];
  readonly dial: readonly { readonly case: string; readonly position: number; readonly angle: number; readonly back: number }[];
  readonly pointer: readonly { readonly case: string; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly snaps: readonly number[]; readonly scale?: UiNumberScale; readonly expected: number }[];
  readonly display: readonly { readonly case: string; readonly stored: number; readonly factor: number | null; readonly precision: number | null; readonly text: string }[];
  readonly typed: readonly { readonly case: string; readonly typed: number; readonly factor: number | null; readonly precision: number | null; readonly candidates: readonly number[]; readonly expected: number }[];
  readonly limits: readonly { readonly case: string; readonly value: number; readonly min: number | null; readonly max: number | null; readonly limits: { readonly min?: { readonly value: number; readonly exclusive?: boolean }; readonly max?: { readonly value: number; readonly exclusive?: boolean } } | null; readonly crossed: "min" | "max" | null }[];
  readonly documents: readonly { readonly case: string; readonly component: Record<string, unknown>; readonly violation: string | null }[];
  readonly adjacent: readonly { readonly case: string; readonly current: number; readonly snaps: readonly number[]; readonly forward: boolean; readonly expected: number | null }[];
  readonly keys: readonly { readonly case: string; readonly current: number; readonly min: number | null; readonly max: number | null; readonly step: number; readonly snaps: readonly number[]; readonly key: SliderKey; readonly large: boolean; readonly precision?: number | null; readonly factor?: number | null; readonly expected: number }[];
  readonly fieldKeys: readonly { readonly case: string; readonly key: string; readonly shift: boolean; readonly min: number | null; readonly max: number | null; readonly expected: { readonly key: SliderKey; readonly large: boolean } | null }[];
  readonly fixed: readonly { readonly case: string; readonly value: number; readonly precision: number; readonly expected: string; readonly rounded: number }[];
  readonly valueTexts: readonly { readonly case: string; readonly component: Record<string, unknown>; readonly valueText: string; readonly valueNow?: number | null; readonly valueMin?: number | null; readonly valueMax?: number | null }[];
};

/** 🚧️ A sparse fixture limit object as the typed wire decodes it (absent sides `null`, absent flags `false`). */
function normalizedLimits(limits: Fixture["limits"][number]["limits"] | undefined): UiNumberLimits | null {
  if (limits == null) return null;
  const bound = (side: { readonly value: number; readonly exclusive?: boolean; readonly refusal?: string } | undefined) => (side == null ? null : { value: side.value, exclusive: side.exclusive ?? false, refusal: side.refusal ?? null });
  return { min: bound(limits.min), max: bound(limits.max) };
}

/** 📏️ The number-range verdict of a sparse fixture component, answered by the TS twin of `number_range_is_valid`. */
function numberRangeVerdict(component: Record<string, unknown>): boolean {
  const number = (key: string): number | null => (typeof component[key] === "number" ? (component[key] as number) : null);
  const scale = component.type === "slider" ? ((component.scale as UiNumberScale | undefined) ?? "linear") : "linear";
  return numberRangeIsValid(number("min"), number("max"), scale, number("displayFactor"), normalizedLimits(component.limits as Fixture["limits"][number]["limits"]), (component.snaps as number[] | undefined) ?? []);
}

/** 🧾️ Normalizes a sparse fixture component the way the typed wire decoder does (absent optionals are `null`). */
function normalizedComponent(component: Record<string, unknown>): Component {
  const numeric = { min: null, max: null, precision: null, unit: null, displayUnit: null, displayFactor: null, snaps: [], ...component, limits: normalizedLimits(component.limits as Fixture["limits"][number]["limits"]) };
  return (component.type === "input" ? { kind: "text", placeholder: null, commit: null, step: null, accept: null, ...numeric } : numeric) as unknown as Component;
}

const read = (path: string): unknown => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));

/** 🧮️ decimal.js oracle: the exact binary expansion rounded half away from zero, unsigned zero, `toFixed`'s exponent fallback. */
function decimalOracle(Decimal: { new (value: string): { toFixed(digits: number, rounding: number): string }; readonly ROUND_HALF_UP: number }, value: number, precision: number): string {
  if (Math.abs(value) >= 1e21) return formatUiNumber(value);
  const text = new Decimal(value.toPrecision(100)).toFixed(Math.min(precision, UI_NUMBER_PRECISION_MAX), Decimal.ROUND_HALF_UP);
  return /^-[0.]*$/.test(text) ? text.slice(1) : text;
}

type DecimalValue = { plus(other: DecimalValue | string): DecimalValue; minus(other: DecimalValue | string): DecimalValue; times(other: DecimalValue | string): DecimalValue; div(other: DecimalValue | string): DecimalValue; floor(): DecimalValue; ceil(): DecimalValue; isInteger(): boolean; toNumber(): number; decimalPlaces(): number; toDecimalPlaces(digits: number, rounding: number): DecimalValue };

/** 🪙️ decimal.js's half-up rounding mode constant. */
const DECIMAL_ROUND_HALF_UP = 4;

/** 🪜️ decimal.js ladder oracle for an arrow key: the step (`step`, else `10^-precision` display units over the factor, else 1), the
 * rung count (a position within the ladder tolerance of a rung is that rung) and rung value computed in exact decimal arithmetic,
 * cleaned to the decimals of the twelve-digit origin and step (at most twelve), rounded half away from zero at the precision in
 * display units and divided back, then clamped. */
function ladderOracle(Decimal: new (value: string) => DecimalValue, row: Fixture["keys"][number]): number {
  const factor = row.factor ?? null;
  const rung = Number.isFinite(row.step) && row.step > 0 ? String(row.step) : row.precision == null ? "1" : new Decimal(`1e-${row.precision}`).div(String(factor ?? 1)).toString();
  const forward = row.key === "increment";
  const origin = String(row.min ?? 0);
  const position = new Decimal(String(row.current)).minus(origin).div(rung);
  const nearest = Math.round(position.toNumber());
  const base = position.isInteger() ? position : Math.abs(position.toNumber() - nearest) <= 1e-9 * Math.max(1, Math.abs(nearest)) ? new Decimal(String(nearest)) : forward ? position.floor() : position.ceil();
  const rungs = String(row.large ? SLIDER_PAGE_STEPS : 1);
  const digits = Math.min(12, Math.max(new Decimal(formatUiNumber(Number(origin))).decimalPlaces(), new Decimal(formatUiNumber(Number(rung))).decimalPlaces()));
  const walked = new Decimal(origin).plus((forward ? base.plus(rungs) : base.minus(rungs)).times(rung)).toDecimalPlaces(digits, DECIMAL_ROUND_HALF_UP);
  const value = row.precision == null ? walked.toNumber() : new Decimal(String(walked.toNumber() * (factor ?? 1))).toDecimalPlaces(row.precision, DECIMAL_ROUND_HALF_UP).div(String(factor ?? 1)).toNumber();
  return Math.min(row.max ?? Number.POSITIVE_INFINITY, Math.max(row.min ?? Number.NEGATIVE_INFINITY, value));
}

/** 🎚️ Answers every shared fixture row, returning how many assertions the corpus carried. */
export function numberControlsSelfTests(): number {
  const require = createRequire(import.meta.url);
  
  const Decimal = require("decimal.js");
  const fixture = read("../../🧫️fixtures/🧫️number-controls/🔣️.json") as Fixture;
  
  
  let checks = 1;
  for (const row of fixture.validity) {
    assert.equal(snapsAreValid(row.snaps, row.min, row.max), row.valid, row.case);
    checks++;
  }
  for (const row of fixture.axis) {
    const position = sliderAxisPosition(row.value, row.min, row.max, row.scale);
    assert(Math.abs(position - row.position) <= 1e-12, `${row.case}: position ${position}`);
    const oracle = (row.scale === "log" ? scaleLog().domain([row.min, row.max]) : scaleLinear().domain([row.min, row.max])).range([0, 1]).clamp(true);
    assert(Math.abs(oracle(row.value) - row.position) <= 1e-12, `${row.case}: d3-scale position oracle`);
    checks += 2;
    if (row.value >= row.min && row.value <= row.max) {
      const back = sliderAxisValue(position, row.min, row.max, row.scale);
      assert(Math.abs(back - row.value) <= 1e-12 * Math.max(1, Math.abs(row.value)), `${row.case}: value ${back}`);
      assert(Math.abs(oracle.invert(row.position) - row.value) <= 1e-12 * Math.max(1, Math.abs(row.value)), `${row.case}: d3-scale inverse oracle`);
      checks += 2;
    }
  }
  for (const row of fixture.dial) {
    assert(Math.abs(dialAngle(row.position) - row.angle) <= 1e-12, `${row.case}: angle`);
    assert(Math.abs(dialPosition(row.angle) - row.back) <= 1e-12, `${row.case}: back`);
    checks += 2;
  }
  for (const row of fixture.pointer) {
    assert.equal(sliderPointerValue(row.value, row.min, row.max, row.step, row.snaps, row.scale ?? "linear"), row.expected, row.case);
    checks++;
  }
  for (const row of fixture.display) {
    assert.equal(uiNumberDisplayText(row.stored, row.factor, row.precision), row.text, row.case);
    const shown = scaleLinear().domain([0, 1]).range([0, row.factor ?? 1])(row.stored);
    assert.equal(uiNumberDisplay(row.stored, row.factor), shown, `${row.case}: d3-scale display oracle`);
    checks += 2;
  }
  for (const row of fixture.typed) {
    const stored = uiNumberTypedValue(row.typed, row.factor, row.precision, row.candidates);
    assert.equal(stored, row.expected, row.case);
    checks++;
    if (!row.candidates.includes(stored) && row.precision == null) {
      assert.equal(scaleLinear().domain([0, 1]).range([0, row.factor ?? 1]).invert(row.typed), stored, `${row.case}: d3-scale read-back oracle`);
      checks++;
    }
  }
  for (const row of fixture.limits) {
    const limits = normalizedLimits(row.limits);
    const crossed = uiNumberCrossedBound(row.value, row.min, row.max, limits);
    const lower = limits ? (limits.min ?? null) : row.min == null ? null : { value: row.min };
    assert.equal(crossed === null ? null : lower !== null && crossed.value === lower.value && (limits === null || crossed === limits.min) ? "min" : "max", row.crossed, row.case);
    checks++;
  }
  for (const row of fixture.adjacent) {
    assert.equal(sliderAdjacentSnap(row.current, row.snaps, row.forward), row.expected, row.case);
    checks++;
  }
  for (const row of fixture.keys) {
    assert.equal(uiNumberKeyValue(row.current, row.min, row.max, row.step, row.precision ?? null, row.factor ?? null, row.snaps, row.key, row.large), row.expected, row.case);
    checks++;
    if (row.min != null && row.max != null) {
      assert.equal(sliderKeyValue(row.current, row.min, row.max, row.step, row.precision ?? null, row.factor ?? null, row.snaps, row.key, row.large), row.expected, `${row.case}: the slider law is the bounded number law`);
      checks++;
    }
    if ((row.key === "increment" || row.key === "decrement") && !row.snaps.includes(row.expected)) {
      assert.equal(ladderOracle(Decimal, row), row.expected, `${row.case}: decimal.js ladder oracle`);
      checks++;
    }
  }
  for (const row of fixture.fieldKeys) {
    assert.deepEqual(uiNumberFieldKey(row.key, row.shift, row.min, row.max), row.expected, row.case);
    checks++;
  }
  for (const row of fixture.fixed) {
    assert.equal(formatUiNumberFixed(row.value, row.precision), row.expected, row.case);
    assert.equal(decimalOracle(Decimal, row.value, row.precision), row.expected, `${row.case}: decimal.js oracle`);
    assert.equal(roundUiNumber(row.value, row.precision), row.rounded, row.case);
    checks += 3;
  }
  for (const row of fixture.valueTexts) {
    const spoken = uiAccessibilityValueV1(normalizedComponent(row.component));
    assert.equal(spoken.valueText, row.valueText, row.case);
    checks++;
    for (const [field, value] of [["valueNow", spoken.valueNow], ["valueMin", spoken.valueMin], ["valueMax", spoken.valueMax], ["valueStep", spoken.valueStep]] as const) {
      if (field in row) {
        assert.equal(value, row[field], `${row.case}: ${field}`);
        checks++;
      }
    }
  }
  for (const row of fixture.documents) {
    if (row.violation === "invalidSnaps") continue;
    assert.equal(numberRangeVerdict(row.component), row.violation !== "invalidNumberRange", `${row.case}: number-range law`);
    checks++;
  }
  return checks;
}
