/** 🧪️ The staged-arg renderer draws every mutation-input control kind of the W1-D vocabulary with the W1-E recipe
 * semantics instead of an interim fallback: a stepper (named value field, named Increase/Decrease), a dial (a detented
 * slider read in display units), a segmented choice (a radio group), a multi-line text (a textarea), vector axes (one named number field per
 * axis carrying the unit) and a reference list (chips that remove, an empty line, "use current selection" from the
 * reference's own domain, capped at `maxItems`). The controls are derived by the manifest's own `argControl` from the
 * stored schema; `dom-accessibility-api` names every field, `@testing-library` drives it, and every vector axis follows the
 * shared number-control corpus (`🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls`: all `keys`, `typed` and `limits` rows, design §18). */
import { afterAll, afterEach, describe, expect, it, vi } from "vitest";
import { createElement } from "react";
import { createRequire } from "node:module";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { argControl } from "@semio-tech/framework";
import { Stepper } from "@semio-tech/ui-react";
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import "../../../🐚️Shell/🟦️.tsx";
import { interactionSelectionIdsV1, renderStagedArgControl, stagedNumberDisplayText, stagedNumberFacetsV1, syncShellLabelLocale, type ResolvedActionArgDef, type StagedArgContextV1 } from "../../🟦️.tsx";
import numberControls from "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls/🔣️.json";
import numberFacets from "../../../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🧫️number-facets/🔣️.json";
import type { ActionArgNumberFacets } from "../../../../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import { uiNumberCrossedBound, uiNumberDisplay, uiNumberDisplayText } from "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧩️component/🟦️.ts";
import { formatUiNumber } from "../../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🔢️number-format/🟦️.ts";

const { computeAccessibleName }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");

type Def = ResolvedActionArgDef;

function mount(def: Def, value: unknown, context: StagedArgContextV1 = {}, disabled = false) {
  const changes: unknown[] = [];
  const view = render(createElement("div", null, createElement("span", { id: "field-label" }, def.label), renderStagedArgControl(def, value, (next) => changes.push(next), disabled, { id: `field.${def.id}`, labelledBy: "field-label", required: def.required }, context)));
  return { ...view, changes };
}

const quantity: Def = { id: "quantity", label: "Quantity", schema: { kind: "number", min: 1, max: 20, step: 1, integer: true }, required: true };
const angle: Def = { id: "angle", label: "Angle", schema: { kind: "number", min: 0, max: 6.283, step: 0.01, integer: false, snaps: [0, 1.5708, 3.1416], precision: 0, displayUnit: "°", displayFactor: 57.29577951308232 }, presentation: { kind: "dial" }, required: true };
const axis: Def = { id: "axis", label: "Axis", schema: { kind: "string", options: [{ value: "x", label: "X axis" }, { value: "y", label: "Y axis" }] }, presentation: { kind: "segmented" }, required: true };
const note: Def = { id: "note", label: "Note", schema: { kind: "string", options: [] }, presentation: { kind: "multiline" }, required: false };
const offset: Def = { id: "offset", label: "Offset", schema: { kind: "vector", dims: 3, unit: "mm", step: 0.5 }, required: true };
const targets: Def = { id: "targets", label: "Targets", schema: { kind: "reference", kinds: ["node"], domain: "vortex", many: true, maxItems: 2 }, required: true };
const pivot: Def = { id: "pivot", label: "Pivot", schema: { kind: "reference", kinds: ["node"], domain: "vortex", many: false }, required: true };

describe("🎛️ staged mutation-input controls", () => {
  afterEach(() => cleanup());
  afterAll(() => syncShellLabelLocale("en"));

  it("derives each case's control kind from the stored schema", () => {
    expect([quantity, angle, axis, note, offset, targets, pivot].map((def) => argControl(def as Parameters<typeof argControl>[0]).kind)).toEqual(["stepper", "dial", "segmented", "multiline", "vector", "reference", "reference"]);
  });

  it("renders an integer as a named stepper whose Increase and Decrease step the value, and disables it whole", () => {
    syncShellLabelLocale("en");
    const view = mount(quantity, 3);
    const field = view.getByRole("spinbutton");
    expect([computeAccessibleName(field), (field as HTMLInputElement).value]).toEqual(["Quantity", "3"]);
    fireEvent.mouseDown(view.getByRole("button", { name: "Increase" }));
    fireEvent.mouseUp(view.getByRole("button", { name: "Increase" }));
    fireEvent.mouseDown(view.getByRole("button", { name: "Decrease" }));
    fireEvent.mouseUp(view.getByRole("button", { name: "Decrease" }));
    expect(view.changes).toEqual([4, 3]);
    view.unmount();
    const disabled = mount(quantity, 3, {}, true);
    expect([(disabled.getByRole("spinbutton") as HTMLInputElement).disabled, (disabled.getByRole("button", { name: "Increase" }) as HTMLButtonElement).disabled]).toEqual([true, true]);
  });

  it("renders a dial as a detented slider read in display units", () => {
    const view = mount(angle, 1.5708);
    const thumb = view.getByRole("slider");
    expect([computeAccessibleName(thumb), thumb.getAttribute("aria-valuetext")]).toEqual(["Angle", "90 °"]);
    expect(view.container.querySelectorAll('[data-slot="slider-tick"]').length).toBe(3);
    expect(view.container.textContent).toContain("90 °");
  });

  it("renders a segmented choice as a named radio group that stages the picked option and never presses the chosen one off", () => {
    const view = mount(axis, "x");
    const group = view.container.querySelector('[data-staged-control="segmented"]')!;
    expect([group.getAttribute("role"), computeAccessibleName(group)]).toEqual(["radiogroup", "Axis"]);
    expect([view.getByRole("radio", { name: "X axis" }).getAttribute("aria-checked"), view.getByRole("radio", { name: "Y axis" }).getAttribute("aria-checked")]).toEqual(["true", "false"]);
    view.getByRole("radio", { name: "X axis" }).click();
    view.getByRole("radio", { name: "Y axis" }).click();
    expect(view.changes).toEqual(["y"]);
  });

  it("renders a multi-line text as a named textarea that stages every line it holds", () => {
    const view = mount(note, "one");
    const area = view.container.querySelector<HTMLTextAreaElement>('textarea[data-staged-control="multiline"]')!;
    expect([computeAccessibleName(area), area.value]).toEqual(["Note", "one"]);
    fireEvent.change(area, { target: { value: "one\ntwo" } });
    expect(view.changes).toEqual(["one\ntwo"]);
  });

  it("renders a vector as one named number field per axis carrying the unit and step", () => {
    const view = mount(offset, [1, 2, 3]);
    const group = view.container.querySelector('[data-staged-control="vector"]')!;
    expect([group.getAttribute("role"), computeAccessibleName(group)]).toEqual(["group", "Offset"]);
    const fields = [...view.container.querySelectorAll<HTMLInputElement>('input[type="number"]')];
    expect(fields.map((field) => [computeAccessibleName(field), field.value, field.step])).toEqual([["x (mm)", "1", "0.5"], ["y (mm)", "2", "0.5"], ["z (mm)", "3", "0.5"]]);
    fireEvent.change(fields[1]!, { target: { value: "4" } });
    expect(view.changes).toEqual([[1, 4, 3]]);
  });

  it("gives every vector axis the vector's facets, stages a typed value and refuses one beyond a hard bound visibly, keeping the draft", () => {
    syncShellLabelLocale("en");
    const turn: Def = { id: "turn", label: "Turn", schema: { kind: "vector", dims: 2, min: -3, max: 3, precision: 1, snaps: [-1.5, 0, 1.5], displayUnit: "°", displayFactor: 10 }, required: true };
    const view = mount(turn, [0.25, 1]);
    const fields = [...view.container.querySelectorAll<HTMLInputElement>('input[type="number"]')];
    expect(fields.map((field) => [computeAccessibleName(field), field.value, field.min, field.max, field.step])).toEqual([["x (°)", "2.5", "-30", "30", "0.1"], ["y (°)", "10.0", "-30", "30", "0.1"]]);
    fireEvent.change(fields[0]!, { target: { value: "12.34" } });
    fireEvent.change(fields[1]!, { target: { value: "99" } });
    expect(view.changes).toEqual([[1.23, 1]]);
    const refusal = view.container.querySelector<HTMLElement>("[data-staged-refusal]")!;
    expect([fields[1]!.value, fields[1]!.getAttribute("aria-invalid"), refusal.getAttribute("role"), refusal.textContent, fields[1]!.getAttribute("aria-describedby")]).toEqual(["99", "true", "alert", "Must be at most 30 °", refusal.id]);
    fireEvent.change(fields[1]!, { target: { value: "-5" } });
    expect([view.changes.at(-1), fields[1]!.getAttribute("aria-invalid"), view.container.querySelector("[data-staged-refusal]")]).toEqual([[0.25, -0.5], null, null]);
  });

  type KeyRow = { readonly case: string; readonly current: number; readonly min: number | null; readonly max: number | null; readonly step: number; readonly snaps: readonly number[]; readonly key: string; readonly large: boolean; readonly precision?: number; readonly factor?: number; readonly expected: number };
  type TypedRow = { readonly case: string; readonly typed: number; readonly factor: number | null; readonly precision: number | null; readonly candidates: readonly number[]; readonly expected: number };
  type LimitRow = { readonly case: string; readonly value: number; readonly min: number | null; readonly max: number | null; readonly limits: { readonly min?: { readonly value: number; readonly exclusive?: boolean }; readonly max?: { readonly value: number; readonly exclusive?: boolean } } | null; readonly crossed: "min" | "max" | null };
  const KEY_EVENTS: Readonly<Record<string, string>> = { increment: "ArrowUp", decrement: "ArrowDown", pageUp: "PageUp", pageDown: "PageDown", home: "Home", end: "End" };
  const axisDef = (facets: Record<string, unknown>): Def => ({ id: "axis", label: "Axis", schema: { kind: "vector", dims: 1, ...facets }, required: true });

  it("walks a vector axis by every key of the shared number-control corpus exactly as its keyboard law says", () => {
    const rows = numberControls.keys as readonly KeyRow[];
    expect(new Set(rows.map((row) => row.key))).toEqual(new Set(Object.keys(KEY_EVENTS)));
    for (const row of rows) {
      const def = axisDef({ snaps: [...row.snaps], ...(row.step > 0 ? { step: row.step } : {}), ...(row.precision === undefined ? {} : { precision: row.precision }), ...(row.factor === undefined ? {} : { displayFactor: row.factor }), ...(row.min === null ? {} : { min: row.min }), ...(row.max === null ? {} : { max: row.max }) });
      const view = mount(def, [row.current]);
      const unbound = (row.key === "home" && row.min === null) || (row.key === "end" && row.max === null);
      const pressed = fireEvent.keyDown(view.container.querySelector<HTMLInputElement>('input[type="number"]')!, { key: KEY_EVENTS[row.key], shiftKey: row.large });
      const last = view.changes.at(-1) as readonly number[] | undefined;
      expect([pressed, view.changes.length], `${row.case}: a Home/End without its bound stays the caret's key`).toEqual(unbound ? [true, 0] : [false, 1]);
      expect(last?.[0] ?? row.current, row.case).toBeCloseTo(row.expected, 9);
      view.unmount();
    }
  });

  it("reads every typed display value of the corpus back to its stored value: an exact detent or the current value kept, else rounded and divided back", () => {
    for (const row of numberControls.typed as readonly TypedRow[]) {
      const current = row.candidates[0] ?? 0;
      const view = mount(axisDef({ snaps: [...row.candidates], ...(row.factor === null ? {} : { displayFactor: row.factor }), ...(row.precision === null ? {} : { precision: row.precision }) }), [current]);
      fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="number"]')!, { target: { value: String(row.typed) } });
      const last = view.changes.at(-1) as readonly number[] | undefined;
      expect(last?.length, row.case).toBe(1);
      expect(last![0], row.case).toBeCloseTo(row.expected, 12);
      view.unmount();
    }
  });

  it("refuses exactly the typed values the corpus's hard-bound rows cross, keeping each draft, and stages the admitted ones", () => {
    syncShellLabelLocale("en");
    for (const row of numberControls.limits as readonly LimitRow[]) {
      const crossed = uiNumberCrossedBound(row.value, row.min, row.max, row.limits as Parameters<typeof uiNumberCrossedBound>[3]);
      const side = crossed === null ? null : row.limits !== null ? (crossed === row.limits.min ? "min" : "max") : crossed.value === row.min && row.value <= crossed.value ? "min" : "max";
      expect(side, `${row.case}: the bound law the axis refuses by`).toBe(row.crossed);
      const inclusiveOnly = row.limits !== null && row.min === null && row.max === null && [row.limits.min, row.limits.max].every((bound) => bound === undefined || bound.exclusive !== true);
      if (row.limits !== null && !inclusiveOnly) continue;
      const min = row.limits === null ? row.min : (row.limits.min?.value ?? null);
      const max = row.limits === null ? row.max : (row.limits.max?.value ?? null);
      const view = mount(axisDef({ ...(min === null ? {} : { min }), ...(max === null ? {} : { max }) }), [(min ?? 0) + 1]);
      const field = view.container.querySelector<HTMLInputElement>('input[type="number"]')!;
      fireEvent.change(field, { target: { value: String(row.value) } });
      const refusal = view.container.querySelector<HTMLElement>("[data-staged-refusal]");
      expect([view.changes.length, field.getAttribute("aria-invalid"), field.value, refusal?.textContent ?? null], row.case).toEqual(row.crossed === null ? [1, null, String(row.value), null] : [0, "true", String(row.value), row.crossed === "min" ? `Must be at least ${min}` : `Must be at most ${max}`]);
      view.unmount();
    }
  });

  it("names a stepper's value field only through a label that exists, so a stepper with no name is reported instead of pointing at a missing id", () => {
    const fieldOf = (props: { readonly id: string; readonly "aria-label"?: string; readonly showLabel?: boolean }) => {
      const view = render(createElement(Stepper, { value: 2, ...props }));
      const field = view.container.querySelector<HTMLInputElement>("input")!;
      const labelledBy = field.getAttribute("aria-labelledby");
      const reading = { labelledBy: labelledBy === null ? null : labelledBy.split(/\s+/u).every((ref) => document.getElementById(ref) !== null), name: computeAccessibleName(field) };
      view.unmount();
      return reading;
    };
    vi.stubGlobal("ResizeObserver", class { observe(): void {} unobserve(): void {} disconnect(): void {} });
    try {
      expect([fieldOf({ id: "stepper.named", "aria-label": "Count" }), fieldOf({ id: "stepper.bare" }), fieldOf({ id: "stepper.shown", showLabel: true }).labelledBy]).toEqual([{ labelledBy: null, name: "Count" }, { labelledBy: null, name: "" }, true]);
    } finally {
      vi.unstubAllGlobals();
    }
  });

  it("draws every staged number control with the shared number-facet corpus: travel, look, axis, detents, display units and localized limits", () => {
    type FacetCase = { readonly name: string; readonly locale: "en" | "de"; readonly def: { readonly id: string; readonly label: { readonly native: Readonly<Record<"en" | "de", string>> } } & Record<string, unknown>; readonly facets: ActionArgNumberFacets | null };
    vi.stubGlobal("ResizeObserver", class { observe(): void {} unobserve(): void {} disconnect(): void {} });
    try {
      for (const row of numberFacets.cases as readonly FacetCase[]) {
        syncShellLabelLocale(row.locale);
        const def = { ...row.def, label: row.def.label.native[row.locale] } as unknown as Def;
        if (row.facets === null) {
          expect(() => stagedNumberFacetsV1(def), row.name).toThrow();
          continue;
        }
        const facets = stagedNumberFacetsV1(def);
        expect(facets, `${row.name}: the shell's facets are the corpus's`).toEqual(row.facets);
        const value = facets.snaps[0] ?? facets.min ?? 0;
        const view = mount(def, def.schema.kind === "vector" ? Array.from({ length: def.schema.dims }, () => value) : value);
        const slider = view.container.querySelector<HTMLElement>('[role="slider"]');
        if (slider !== null) {
          const spoken = (stored: number) => String(facets.displayFactor === undefined ? stored : Number(formatUiNumber(stored * facets.displayFactor)));
          expect([view.container.querySelector('[data-slot="slider-dial"]') !== null, view.container.querySelectorAll('[data-slot="slider-tick"]').length, slider.getAttribute("aria-valuetext"), slider.getAttribute("aria-valuemin"), slider.getAttribute("aria-valuemax")], row.name).toEqual([facets.appearance === "dial", facets.snaps.length, stagedNumberDisplayText(value, facets), spoken(facets.min!), spoken(facets.max!)]);
        } else {
          const field = view.container.querySelector<HTMLInputElement>('[data-stepper-input="true"], input[type="number"]')!;
          const unit = facets.displayUnit ?? facets.unit;
          const shownUnit = view.container.querySelector('[data-slot="stepper-unit"], [data-slot="staged-unit"]')?.textContent ?? view.container.querySelector("label")?.textContent?.match(/\((.*)\)$/u)?.[1] ?? null;
          expect([field.value, shownUnit], row.name).toEqual([uiNumberDisplayText(value, facets.displayFactor, facets.precision), unit ?? null]);
          const [bound, beyond] = facets.limits.max != null ? [facets.limits.max, 1] : [facets.limits.min!, -1];
          fireEvent.change(field, { target: { value: String(uiNumberDisplay(bound.value, facets.displayFactor) + beyond) } });
          expect([view.container.querySelector('[role="alert"]')?.textContent, field.getAttribute("aria-invalid")], `${row.name}: the typed value beyond the limit is refused in ${row.locale}`).toEqual([bound.refusal, "true"]);
        }
        view.unmount();
      }
    } finally {
      vi.unstubAllGlobals();
      syncShellLabelLocale("en");
    }
  });

  it("renders a reference list as removable chips with an empty line and a use-selection button in both languages", async () => {
    const asked: (string | undefined)[] = [];
    const selection = (domain: string | undefined) => {
      asked.push(domain);
      return ["n-3", "n-4", "n-3", "n-5"];
    };
    syncShellLabelLocale("en");
    const many = mount(targets, ["n-1", "n-2"], { selection });
    expect(computeAccessibleName(many.container.querySelector("[data-staged-reference]")!)).toBe("Targets");
    many.getByRole("button", { name: "Remove n-1" }).click();
    many.getByRole("button", { name: "Use current selection" }).click();
    expect([many.changes, asked]).toEqual([[["n-2"], ["n-3", "n-4"]], ["vortex"]]);
    many.unmount();
    syncShellLabelLocale("de");
    const empty = mount(targets, [], { selection });
    expect(empty.container.querySelector("[data-staged-reference-empty]")?.textContent).toBe("Nichts ausgewählt");
    await act(async () => {
      empty.getByRole("button", { name: "Aktuelle Auswahl verwenden" }).click();
    });
    expect(empty.changes).toEqual([["n-3", "n-4"]]);
    empty.unmount();
    syncShellLabelLocale("en");
    const single = mount(pivot, "n-1", { selection });
    expect(single.getByRole("button", { name: "Remove n-1" })).toBeTruthy();
    single.getByRole("button", { name: "Use current selection" }).click();
    expect(single.changes).toEqual(["n-3"]);
    single.unmount();
    const blind = mount(pivot, "");
    expect((blind.getByRole("button", { name: "Use current selection" }) as HTMLButtonElement).disabled).toBe(true);
  });

  it("shows integer ids as chips and stages the integers a selection's id texts spell", () => {
    const zones: Def = { id: "zoneIds", label: "Zones", schema: { kind: "reference", kinds: ["zone"], domain: "energyModel", many: true, idType: "integer" }, required: true };
    const zone: Def = { id: "zoneId", label: "Zone", schema: { kind: "reference", kinds: ["zone"], domain: "energyModel", idType: "integer" }, required: true };
    const selection = () => ["7", "x", "9"];
    const many = mount(zones, [3, 4], { selection });
    many.getByRole("button", { name: "Remove 3" }).click();
    many.getByRole("button", { name: "Use current selection" }).click();
    expect(many.changes).toEqual([[4], [7, 9]]);
    many.unmount();
    const single = mount(zone, 5, { selection: () => ["x"] });
    single.getByRole("button", { name: "Remove 5" }).click();
    single.getByRole("button", { name: "Use current selection" }).click();
    expect(single.changes).toEqual([null, null]);
  });

  it.each(["en", "de"] as const)("renders a color with the color_input recipe in %s: a named swatch, a hex field and an opacity slider", (locale) => {
    syncShellLabelLocale(locale);
    const names = locale === "en" ? { hex: "Hex", alpha: "Opacity" } : { hex: "Hex", alpha: "Deckkraft" };
    const fill: Def = { id: "fill", label: "Fill", schema: { kind: "vector", dims: 4, min: 0, max: 1 }, presentation: { kind: "color" }, required: true };
    const stroke: Def = { id: "stroke", label: "Stroke", schema: { kind: "vector", dims: 3, min: 0, max: 1 }, presentation: { kind: "color" }, required: true };
    expect([argControl(fill as Parameters<typeof argControl>[0]), argControl(stroke as Parameters<typeof argControl>[0])]).toEqual([{ kind: "color", alpha: true }, { kind: "color", alpha: false }]);
    const rgba = mount(fill, [1, 0, 0, 0.5]);
    const picker = rgba.container.querySelector("input[type=color]") as HTMLInputElement;
    expect([computeAccessibleName(picker), picker.value]).toEqual(["Fill", "#ff0000"]);
    const hex = rgba.getByRole("textbox", { name: names.hex }) as HTMLInputElement;
    expect(hex.value).toBe("#ff000080");
    const opacity = rgba.getByRole("slider", { name: names.alpha });
    expect([opacity.getAttribute("aria-valuemin"), opacity.getAttribute("aria-valuemax"), opacity.getAttribute("aria-valuenow")]).toEqual(["0", "1", "0.5"]);
    fireEvent.change(picker, { target: { value: "#0000ff" } });
    fireEvent.change(hex, { target: { value: "#00ff00" } });
    fireEvent.blur(hex);
    fireEvent.change(hex, { target: { value: " 00FF0040" } });
    fireEvent.keyDown(hex, { key: "Enter" });
    fireEvent.change(hex, { target: { value: "#nope" } });
    fireEvent.blur(hex);
    fireEvent.keyDown(opacity, { key: "ArrowRight" });
    expect(rgba.changes).toEqual([[0, 0, 1, 0.5], [0, 1, 0, 0.5], [0, 1, 0, 64 / 255], [1, 0, 0, 0.51]]);
    expect(hex.value).toBe("#ff000080");
    rgba.unmount();
    const rgb = mount(stroke, [0, 1, 0]);
    expect([rgb.container.querySelector("[role=slider]"), (rgb.getByRole("textbox", { name: names.hex }) as HTMLInputElement).value]).toEqual([null, "#00ff00"]);
    fireEvent.change(rgb.container.querySelector("input[type=color]")!, { target: { value: "#ffffff" } });
    expect(rgb.changes).toEqual([[1, 1, 1]]);
  });

  it("offers a nullable input a pressed clear toggle that stages null and gives the value back", () => {
    syncShellLabelLocale("en");
    const scale: Def = { id: "newScale", label: "New Scale", schema: { kind: "number", min: 0.1, max: 10, step: 0.1, integer: false }, required: false, nullable: true };
    const set = mount(scale, 2);
    const clear = set.getByRole("button", { name: "Clear" }) as HTMLButtonElement;
    expect(clear.getAttribute("aria-pressed")).toBe("false");
    clear.click();
    set.unmount();
    const cleared = mount(scale, null);
    const pressed = cleared.getByRole("button", { name: "Clear" });
    expect(pressed.getAttribute("aria-pressed")).toBe("true");
    pressed.click();
    expect([set.changes, cleared.changes]).toEqual([[null], [undefined]]);
    cleared.unmount();
    const plain = mount({ ...scale, nullable: false }, 2);
    expect(plain.container.querySelector("[data-staged-nullable]")).toBeNull();
  });

  it("reads a domain's selection, or every domain's when the reference names none", () => {
    const state = { selection: { vortex: { granularity: "node", ids: ["n-1", "n-2"] }, region: { granularity: "region", ids: ["r-1"] } }, hover: {}, activeMode: {}, activeGranularity: {} };
    expect([interactionSelectionIdsV1(state, "vortex"), interactionSelectionIdsV1(state, "missing"), interactionSelectionIdsV1(state, undefined)]).toEqual([["n-1", "n-2"], [], ["n-1", "n-2", "r-1"]]);
  });
});
