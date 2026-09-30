/** 🧪️ The staged-arg renderer draws every mutation-input control kind of the W1-D vocabulary with the W1-E recipe
 * semantics instead of an interim fallback: a stepper (named value field, named Increase/Decrease), a dial (a detented
 * slider read in display units), a segmented choice (pressed toggle buttons), vector axes (one named number field per
 * axis carrying the unit) and a reference list (chips that remove, an empty line, "use current selection" from the
 * reference's own domain, capped at `maxItems`). The controls are derived by the manifest's own `argControl` from the
 * stored schema; `dom-accessibility-api` names every field and `@testing-library` drives it. */
import { afterAll, afterEach, describe, expect, it } from "vitest";
import { createElement } from "react";
import { createRequire } from "node:module";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { argControl } from "@semio-tech/framework";
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import "../../../🐚️Shell/🟦️.tsx";
import { interactionSelectionIdsV1, renderStagedArgControl, syncShellLabelLocale, type ResolvedActionArgDef, type StagedArgContextV1 } from "../../🟦️.tsx";

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
const offset: Def = { id: "offset", label: "Offset", schema: { kind: "vector", dims: 3, unit: "mm", step: 0.5 }, required: true };
const targets: Def = { id: "targets", label: "Targets", schema: { kind: "reference", kinds: ["node"], domain: "vortex", many: true, maxItems: 2 }, required: true };
const pivot: Def = { id: "pivot", label: "Pivot", schema: { kind: "reference", kinds: ["node"], domain: "vortex", many: false }, required: true };

describe("🎛️ staged mutation-input controls", () => {
  afterEach(() => cleanup());
  afterAll(() => syncShellLabelLocale("en"));

  it("derives each case's control kind from the stored schema", () => {
    expect([quantity, angle, axis, offset, targets, pivot].map((def) => argControl(def as Parameters<typeof argControl>[0]).kind)).toEqual(["stepper", "dial", "segmented", "vector", "reference", "reference"]);
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

  it("renders a segmented choice as pressed toggle buttons that stage the picked value", () => {
    const view = mount(axis, "x");
    expect(computeAccessibleName(view.container.querySelector('[data-staged-control="segmented"]')!)).toBe("Axis");
    expect([view.getByRole("button", { name: "X axis" }).getAttribute("aria-pressed"), view.getByRole("button", { name: "Y axis" }).getAttribute("aria-pressed")]).toEqual(["true", "false"]);
    view.getByRole("button", { name: "Y axis" }).click();
    expect(view.changes).toEqual(["y"]);
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

  it("gives every vector axis the vector's facets: bounds, precision, display factor and detents on the page keys", () => {
    const turn: Def = { id: "turn", label: "Turn", schema: { kind: "vector", dims: 2, min: -3, max: 3, precision: 1, snaps: [-1.5, 0, 1.5], displayUnit: "°", displayFactor: 10 }, required: true };
    const view = mount(turn, [0.25, 1]);
    const fields = [...view.container.querySelectorAll<HTMLInputElement>('input[type="number"]')];
    expect(fields.map((field) => [computeAccessibleName(field), field.value, field.min, field.max, field.step])).toEqual([["x (°)", "2.5", "-30", "30", "1"], ["y (°)", "10", "-30", "30", "1"]]);
    fireEvent.keyDown(fields[0]!, { key: "PageUp" });
    fireEvent.keyDown(fields[1]!, { key: "PageDown" });
    fireEvent.change(fields[0]!, { target: { value: "12.34" } });
    fireEvent.change(fields[1]!, { target: { value: "99" } });
    expect(view.changes).toEqual([[1.5, 1], [0.25, 0], [1.23, 1], [0.25, 3]]);
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
