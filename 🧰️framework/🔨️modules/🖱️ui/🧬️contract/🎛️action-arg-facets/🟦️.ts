/** 🎛️ The number facets of an action argument as the UI contract needs them: the one mapping every renderer shares, projected from the manifest's argument definition. */
import { type ShellLocale } from "@semio-tech/framework-manifest/generated/ui-axes";
import { argControl, type ActionArgDef } from "@semio-tech/framework-manifest/action-types-and-semantics";
import type { SliderAppearance, UiNumberBound, UiNumberLimits, UiNumberScale } from "@semio-tech/framework-manifest/ui-contract-and-canvas-types";
import { UI_NUMBER_PRECISION_MAX, uiNumberCrossedBound, uiNumberDisplayText } from "../🧩️component/🟦️.ts";

/** 🎛️ The UI-contract number facets of one input — the Rust `ActionArgNumberFacets` twin, the single mapping every renderer
 * of an `ActionArgDef` shares: key range (a slider's or dial's travel; a stepper's, number field's or vector axis' hard
 * bounds, an excluded bound absent), step, look, axis, unit symbols, display factor, precision, the admitted detents and
 * the schema's hard limits with refusals naming the bound in display units. Fixture `🧫️fixtures/🧫️number-facets`. */
export type ActionArgNumberFacets = {
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
  readonly appearance: SliderAppearance;
  readonly scale: UiNumberScale;
  readonly unit?: string;
  readonly displayUnit?: string;
  readonly displayFactor?: number;
  readonly precision?: number;
  readonly snaps: readonly number[];
  readonly limits: UiNumberLimits;
};

/** 🔣️ The symbol a descriptor's unit name shows as (`deg` reads `°`, `percent` reads `%`) — the Rust `action_arg_unit_symbol` twin. */
export function actionArgUnitSymbol(unit: string): string {
  return unit === "deg" || unit === "degree" || unit === "degrees" ? "°" : unit === "percent" ? "%" : unit;
}

/** 🚧️ The refusal of a typed value crossing a hard bound, `{bound}` naming it — framework copy, EN and DE, no default. */
const ACTION_ARG_BOUND_REFUSALS: Readonly<Record<ShellLocale, { readonly atLeast: string; readonly greaterThan: string; readonly atMost: string; readonly lessThan: string }>> = {
  en: { atLeast: "Must be at least {bound}", greaterThan: "Must be greater than {bound}", atMost: "Must be at most {bound}", lessThan: "Must be less than {bound}" },
  de: { atLeast: "Muss mindestens {bound} sein", greaterThan: "Muss größer als {bound} sein", atMost: "Darf höchstens {bound} sein", lessThan: "Muss kleiner als {bound} sein" },
};

/** 🎛️ The {@link ActionArgNumberFacets} of the control {@link argControl} derives, its refusals in `locale` — `null` unless the
 * input is a number field, stepper, slider, dial or vector. The Rust `ActionArgDef::number_facets` twin. */
export function actionArgNumberFacets(def: ActionArgDef, locale: ShellLocale): ActionArgNumberFacets | null {
  const schema = def.schema;
  if (schema.kind !== "number" && schema.kind !== "vector") return null;
  const minExclusive = schema.kind === "number" && schema.minExclusive === true;
  const maxExclusive = schema.kind === "number" && schema.maxExclusive === true;
  const control = argControl(def);
  if (control.kind !== "slider" && control.kind !== "dial" && control.kind !== "stepper" && control.kind !== "number" && control.kind !== "vector") return null;
  const keyed = control.kind === "stepper" || control.kind === "number";
  const min = keyed && minExclusive ? undefined : control.min;
  const max = keyed && maxExclusive ? undefined : control.max;
  const step = control.step !== undefined && Number.isFinite(control.step) && control.step > 0 ? control.step : undefined;
  const unit = control.unit === undefined ? undefined : actionArgUnitSymbol(control.unit);
  const displayUnit = control.displayUnit === undefined ? undefined : actionArgUnitSymbol(control.displayUnit);
  const shownUnit = displayUnit ?? unit;
  const refusals = ACTION_ARG_BOUND_REFUSALS[locale];
  const bound = (value: number, exclusive: boolean, below: boolean): UiNumberBound => {
    const text = uiNumberDisplayText(value, control.displayFactor, null);
    const refusal = (below ? (exclusive ? refusals.greaterThan : refusals.atLeast) : exclusive ? refusals.lessThan : refusals.atMost).replace("{bound}", shownUnit === undefined ? text : `${text} ${shownUnit}`);
    return exclusive ? { value, exclusive, refusal } : { value, refusal };
  };
  const limits: UiNumberLimits = { ...(schema.min === undefined ? {} : { min: bound(schema.min, minExclusive, true) }), ...(schema.max === undefined ? {} : { max: bound(schema.max, maxExclusive, false) }) };
  const [low, high] = [min ?? -Infinity, max ?? Infinity];
  const snaps: number[] = [];
  for (const snap of control.kind === "number" ? [] : (control.snaps ?? [])) {
    if (Number.isFinite(snap) && snap > (snaps.at(-1) ?? -Infinity) && snap >= low && snap <= high && uiNumberCrossedBound(snap, null, null, limits) === null) snaps.push(snap);
  }
  return {
    ...(min === undefined ? {} : { min }),
    ...(max === undefined ? {} : { max }),
    ...(step === undefined ? {} : { step }),
    appearance: control.kind === "dial" ? "dial" : "track",
    scale: control.kind === "slider" && control.scale === "log" ? "log" : "linear",
    ...(unit === undefined ? {} : { unit }),
    ...(displayUnit === undefined ? {} : { displayUnit }),
    ...(control.displayFactor === undefined ? {} : { displayFactor: control.displayFactor }),
    ...(control.precision === undefined ? {} : { precision: Math.min(control.precision, UI_NUMBER_PRECISION_MAX) }),
    snaps,
    limits,
  };
}
