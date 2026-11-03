// @vitest-environment jsdom
/** ✏️ The React half of the `focusRowControl` law. `F2` on a focused tree row moves the keyboard into the first editable cell of that row, a row without one is refused politely in both languages, and the cell answers the shared typing law: Escape shows the committed value again, Enter commits.
 *
 * 🧫️ Read from `🐚️Shell/🧪️tests/🧫️fixtures/🎯️focus-row-control/🔣️.json` — the SAME file the wgpu laws read (`🔬️wgpu-shell-chrome-parity/🎯️focus-row-control/🦀️.rs` and the retained-canvas cell law of `🖱️ui`), so the rule, the key and the texts cannot drift between the implementations. The browser's own focus model (jsdom `document.activeElement`) judges where the focus landed. */
import { describe, expect, it } from "vitest";
import { FOCUS_ROW_CONTROL_ACTION_ID, FOCUS_ROW_CONTROL_ARG_ROW, FOCUS_ROW_CONTROL_EDITABLE_KINDS, FOCUS_ROW_CONTROL_KEY, type ShellLocale } from "@semio-tech/framework";
import { focusedRowKeyV1, focusRowControlArgsV1, focusRowControlRefusalV1, focusRowControlV1, isFocusRowControlKeyV1, planRowControlFocusV1, rowControlKindOfElementV1, rowKeyOfDomIdV1 } from "../../🧱️elements/🏛️ShellHost/🎯️focus-row-control/🟦️.ts";
import fixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/🎯️focus-row-control/🔣️.json" with { type: "json" };

const ROW_ID = "table.marker.m0";
const row = (inner: string): string => `<div role="tree"><div role="treeitem" tabindex="0" id="surface/${ROW_ID}">${inner}</div></div>`;

describe("✏️ focusRowControl", () => {
  it("shares its action, argument, key and editable kinds with the Rust shell", () => {
    expect(fixture.action).toBe(FOCUS_ROW_CONTROL_ACTION_ID);
    expect(fixture.argument).toBe(FOCUS_ROW_CONTROL_ARG_ROW);
    expect(fixture.key).toBe(FOCUS_ROW_CONTROL_KEY);
    expect(fixture.editableKinds).toEqual([...FOCUS_ROW_CONTROL_EDITABLE_KINDS]);
    expect(focusRowControlArgsV1({ [FOCUS_ROW_CONTROL_ARG_ROW]: ROW_ID })).toBe(ROW_ID);
    expect(focusRowControlArgsV1({})).toBeNull();
    expect(focusRowControlArgsV1(undefined)).toBeNull();
    expect(rowKeyOfDomIdV1(`surface/${ROW_ID}`)).toBe(ROW_ID);
  });

  for (const planRow of fixture.plans) {
    it(`plans: ${planRow.id}`, () => {
      expect(planRowControlFocusV1(planRow.controls)).toBe(planRow.target);
    });
  }

  it("raises the verb only for a bare F2", () => {
    const key = (init: Partial<KeyboardEventInit> & { key: string }) => new KeyboardEvent("keydown", init);
    expect(isFocusRowControlKeyV1(key({ key: "F2" }))).toBe(true);
    for (const modified of [{ ctrlKey: true }, { metaKey: true }, { altKey: true }, { shiftKey: true }]) expect(isFocusRowControlKeyV1(key({ key: "F2", ...modified }))).toBe(false);
    expect(isFocusRowControlKeyV1(key({ key: "F3" }))).toBe(false);
  });

  it("names the kind of every DOM control like the retained document does", () => {
    document.body.innerHTML = `<input id="t"><input id="n" type="number"><input id="d" type="date"><input id="c" type="color"><input id="f" type="file"><input id="k" type="checkbox"><textarea id="a"></textarea><select id="s"></select><button id="b"></button>`;
    const kinds = Object.fromEntries(["t", "n", "d", "c", "f", "k", "a", "s", "b"].map((id) => [id, rowControlKindOfElementV1(document.getElementById(id)!)]));
    expect(kinds).toEqual({ t: "text", n: "number", d: "date", c: "color", f: "file", k: "toggle", a: "longText", s: "select", b: "button" });
  });

  for (const origin of fixture.focusOrigins) {
    it(`F2 origin: ${origin.id}`, () => {
      document.body.innerHTML = row(`<span>Marker</span><input id="cell" type="text" value="350 cm">`);
      const focused = origin.focus === "row" ? document.querySelector('[role="treeitem"]') : origin.focus === "cell" ? document.getElementById("cell") : document.body;
      expect(focusedRowKeyV1(focused)).toBe(origin.rowId);
    });
  }

  it("focuses the first editable cell of the row and selects its text so typing replaces it", () => {
    document.body.innerHTML = row(`<button id="label">Marker</button><input id="locked" type="text" disabled><input id="cell" type="text" value="350 cm"><input id="second" type="number">`);
    const target = focusRowControlV1(document, ROW_ID);
    expect(target?.id).toBe("cell");
    expect(document.activeElement?.id).toBe("cell");
    const cell = document.getElementById("cell") as HTMLInputElement;
    expect([cell.selectionStart, cell.selectionEnd]).toEqual([0, "350 cm".length]);
  });

  it("leaves the cells of a nested row to that row", () => {
    document.body.innerHTML = row(`<span>Outer</span><div role="treeitem" id="surface/inner"><input id="inner-cell" type="text"></div>`);
    expect(focusRowControlV1(document, ROW_ID)).toBeNull();
    expect(focusRowControlV1(document, "inner")?.id).toBe("inner-cell");
  });

  it("refuses a row nobody presents and a row without an editable cell", () => {
    document.body.innerHTML = row(`<input type="color"><input type="checkbox">`);
    expect(focusRowControlV1(document, ROW_ID)).toBeNull();
    expect(focusRowControlV1(document, "elsewhere")).toBeNull();
  });

  for (const announcement of fixture.announcements) {
    it(`announces the refusal in ${announcement.locale}`, () => {
      expect(focusRowControlRefusalV1(announcement.locale as ShellLocale)).toEqual({ text: announcement.message, severity: announcement.severity, code: announcement.code });
    });
  }
});
