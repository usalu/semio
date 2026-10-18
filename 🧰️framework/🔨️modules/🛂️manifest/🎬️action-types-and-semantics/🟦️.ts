/** 🎬️ The action definition, address, invocation and argument schema types, the reference id codec, the capability and semantics types and the argument control and number facets. */
import { type ShellLocale } from "../🤖️generated/🎚️ui-axes/🟦️.ts";
import { UI_NUMBER_PRECISION_MAX, uiNumberCrossedBound, uiNumberDisplayText } from "../../🖱️ui/🧬️contract/🧩️component/🟦️.ts";
import type { ActionKind as GeneratedActionKind, ActionDefinition as GeneratedActionDefinition, ActionAddress as GeneratedActionAddress, ActionInvocation as GeneratedActionInvocation, ActionArgDef as GeneratedActionArgDef, ActionArgControl as GeneratedActionArgControl, ActionArgOption as GeneratedActionArgOption, ArgSchema as GeneratedArgSchema, ArgFormat as GeneratedArgFormat, ArgPresentation as GeneratedArgPresentation, SnapSource as GeneratedSnapSource, OptionSource as GeneratedOptionSource, NumberScale as GeneratedNumberScale, ReferenceIdType as GeneratedReferenceIdType, ResourceSelector as GeneratedResourceSelector, CapabilityEffects as GeneratedCapabilityEffects, ApprovalMode as GeneratedApprovalMode, CapabilityPolicy as GeneratedCapabilityPolicy, PreviewMode as GeneratedPreviewMode, UndoMode as GeneratedUndoMode, IdempotencyMode as GeneratedIdempotencyMode, ExecutionClass as GeneratedExecutionClass, CapabilityExecution as GeneratedCapabilityExecution, ActionSemantics as GeneratedActionSemantics } from "../🤖️generated/🪪️manifest/🟦️.ts";
import type { SliderAppearance, UiNumberBound, UiNumberLimits, UiNumberScale } from "../🏷️ui-contract-and-canvas-types/🟦️.ts";

// #region 🛂️Manifest
//#region 🔌️PluginAndAppContract
//#region PluginRuntime
/** 🧬️ Generated from Rust `ActionKind`/`ActionDefinition` (`framework/core/rs/lib.rs`) — see `js/generated/manifest.ts`. */
export type ActionKind = GeneratedActionKind;
export type ActionDefinition = GeneratedActionDefinition;
export type ActionAddress = GeneratedActionAddress;
export type ActionInvocation = GeneratedActionInvocation;
export type ActionArgDef = GeneratedActionArgDef;
export type ActionArgControl = GeneratedActionArgControl;
export type ActionArgOption = GeneratedActionArgOption;

/** 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P3-manifest-schema, D6: the
 * stored, engine-neutral shape of one `ActionArgDef`'s value (see Rust `🔖️ArgSchema`) — the sole
 * persisted truth; `ActionArgControl` above (unchanged) is now DERIVED from it by {@link argControl}. */
export type ArgSchema = GeneratedArgSchema;
export type ArgFormat = GeneratedArgFormat;
export type ArgPresentation = GeneratedArgPresentation;
export type SnapSource = GeneratedSnapSource;
export type OptionSource = GeneratedOptionSource;
export type NumberScale = GeneratedNumberScale;
export type ReferenceIdType = GeneratedReferenceIdType;

/** 🔢️ The largest integer id magnitude a reference admits — the Rust `REFERENCE_ID_INTEGER_MAX` twin. */
export const REFERENCE_ID_INTEGER_MAX = Number.MAX_SAFE_INTEGER;

/** 🎯️ The payload value of one selected id: the text itself, or the integer it spells — `undefined` for empty text or, for an
 * integer reference, text that is not a decimal integer within ±(2^53 − 1). The Rust `ReferenceIdType::id_value` twin. */
export function referenceIdValue(idType: ReferenceIdType | undefined, id: string): string | number | undefined {
  if (id === "") return undefined;
  if (idType !== "integer") return id;
  if (!/^-?[0-9]+$/u.test(id)) return undefined;
  const value = Number(id);
  return Number.isSafeInteger(value) ? value : undefined;
}

/** 🏷️ The text of one reference id value — a non-empty string id as it is, an integer id in decimal (the spelling a selection
 * carries); `undefined` for anything else. The Rust `reference_id_text` twin. */
export function referenceIdText(value: unknown): string | undefined {
  if (typeof value === "string") return value === "" ? undefined : value;
  return typeof value === "number" && Number.isSafeInteger(value) ? String(value) : undefined;
}

/** 🎯️ Generated from Rust `🔖️ActionSemantics` (`🛂️manifest/🦀️.rs`) — effects/policy/
 * execution + natural-language framing carried on every `ActionDefinition`/`CommandDefinition`. */
export type ResourceSelector = GeneratedResourceSelector;
export type CapabilityEffects = GeneratedCapabilityEffects;
export type ApprovalMode = GeneratedApprovalMode;
export type CapabilityPolicy = GeneratedCapabilityPolicy;
export type PreviewMode = GeneratedPreviewMode;
export type UndoMode = GeneratedUndoMode;
export type IdempotencyMode = GeneratedIdempotencyMode;
export type ExecutionClass = GeneratedExecutionClass;
export type CapabilityExecution = GeneratedCapabilityExecution;
export type ActionSemantics = GeneratedActionSemantics;

//#region 🎯️ActionSemanticsDefaults
/** 🏭️ Mirrors native `ActionSemantics::for_kind`; defaults never constitute an interactive-job migration proof. */
export function actionSemanticsForKind(kind: ActionKind): ActionSemantics {
  const mutation = kind === "mutation";
  const observes = kind === "view" || kind === "interaction";
  return {
    effects: { reads: observes ? ["config:{self}"] : [], writes: mutation ? ["artifact:{self}"] : [], external: false, destructive: false, reversible: mutation },
    policy: { scopes: mutation || kind === "history" ? ["artifacts.write"] : observes ? ["artifacts.read", "shell.observe"] : kind === "clipboard" ? ["shell.clipboard"] : ["shell.navigate"], approval: mutation ? "whenDestructive" : "never" },
    execution: { preview: mutation ? "diff" : "none", undo: { kind: mutation ? "inverse" : "none" }, idempotency: "none", expectedRevision: mutation, cancellable: false, class: "interactive", interactiveJob: "unclassified" },
    useWhen: [], examples: [],
  };
}

//#endregion 🎯️ActionSemanticsDefaults
/** 🎛️ Mirrors Rust `ActionArgDef::control()` exactly (D6): derives the renderer-facing
 * `ActionArgControl` from `def.schema`/`def.presentation` — the ONLY place a TS reader should reach
 * for an argument's widget kind; never reconstructs `ActionArgControl` from `schema` by hand.
 * Priority matches Rust: non-empty `options` always wins Select (or Segmented); a plain string declared multiline is
 * Multiline; a number's presentation wins, else an integer steps, else a fully bounded number slides, else it is a plain number field. */
export function argControl(def: ActionArgDef): ActionArgControl {
  const schema = def.schema;
  switch (schema.kind) {
    case "string": {
      const options = schema.options ?? [];
      if (options.length > 0 || schema.optionSource !== undefined) return def.presentation?.kind === "segmented" ? { kind: "segmented", options } : { kind: "select", options };
      const format = schema.format;
      if (format?.kind === "iconId") return { kind: "iconSelect", classifierKind: "icon" };
      if (format?.kind === "artifactKind") return { kind: "artifactKind", roles: format.roles };
      if (format?.kind === "surfaceApp") return { kind: "surfaceApp", roles: format.roles, dialectArg: format.dialectArg };
      return def.presentation?.kind === "multiline" ? { kind: "multiline" } : { kind: "text" };
    }
    case "number": {
      const facets = { step: schema.step, unit: schema.unit, precision: schema.precision, displayUnit: schema.displayUnit, displayFactor: schema.displayFactor };
      const snapping = { snaps: schema.snaps, snapSource: schema.snapSource };
      const travel = { min: schema.softMin ?? schema.min ?? 0, max: schema.softMax ?? schema.max ?? 0 };
      const presentation = def.presentation?.kind;
      if (presentation === "slider" || (presentation !== "dial" && presentation !== "stepper" && !schema.integer && schema.min !== undefined && schema.max !== undefined)) return { kind: "slider", ...travel, ...facets, ...snapping, scale: schema.scale };
      if (presentation === "dial") return { kind: "dial", ...travel, ...facets, ...snapping };
      if (presentation === "stepper" || schema.integer) return { kind: "stepper", min: schema.min, max: schema.max, ...facets, ...snapping };
      return { kind: "number", min: schema.min, max: schema.max, ...facets };
    }
    case "boolean":
      return { kind: "toggle" };
    case "vector":
      if (def.presentation?.kind === "color") return { kind: "color", alpha: schema.dims === 4 };
      return { kind: "vector", dims: schema.dims, min: schema.min, max: schema.max, unit: schema.unit, step: schema.step, snaps: schema.snaps, snapSource: schema.snapSource, precision: schema.precision, displayUnit: schema.displayUnit, displayFactor: schema.displayFactor };
    case "reference":
      return { kind: "reference", kinds: schema.kinds, domain: schema.domain, granularity: schema.granularity, many: schema.many, minItems: schema.minItems, maxItems: schema.maxItems, ...(schema.idType === "integer" ? { idType: schema.idType } : {}) };
    case "array":
    case "object":
    case "any":
      return { kind: "text" };
  }
}

//#region 🔖️ActionArgFacets
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
//#endregion 🔖️ActionArgFacets
//#endregion PluginRuntime
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest
