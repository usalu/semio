// @vitest-environment jsdom
/** ⏪️ The keyboard route to Undo and Redo as laws over ONE language-neutral fixture
 * (`🏛️ShellHost/🧪️tests/🧫️fixtures/⌨️history-key-route/🔣️.json`):
 *   1. which verb a keydown asks for under the user's overrides (default chords, moved chords, freed defaults, unbound);
 *   2. which focused element keeps the chord for itself — only text editing surfaces do; a canvas, a button, a tree row and
 *      every field that edits no text (a `select` left focused by an earlier edit, a checkbox, a slider) let it through to the history.
 *
 * ⚖️ Every chord row runs through the shipped matcher, through an independent in-file oracle and, for the defaults, through real
 * events minted by the third-party `@testing-library/user-event` keyboard model. */

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it } from "vitest";
import { HISTORY_VERB_CHORDS_V1, effectiveChordsV1, historyVerbForChordV1 } from "../../🧱️elements/🏛️ShellHost/⌨️window-scope/🟦️.ts";
import { controlOwnsKeyboardEvent, keyboardEventMatchesChord } from "../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts";

type ChordRow = { readonly id: string; readonly key: string; readonly ctrl: boolean; readonly meta: boolean; readonly shift: boolean; readonly alt: boolean; readonly overrides: Readonly<Record<string, string>>; readonly expected: "undo" | "redo" | null };
type FocusRow = { readonly id: string; readonly target: string; readonly ownsChord: boolean };
type Fixture = { readonly defaults: { readonly undo: string; readonly redo: string }; readonly chords: readonly ChordRow[]; readonly focus: readonly FocusRow[] };

const fixture = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/⌨️history-key-route/🔣️.json"), "utf8")) as Fixture;

/** ⚖️ The same decision written without the shipped helpers: split, trim, lower-case, compare modifiers one by one. */
function oracleVerb(row: Pick<ChordRow, "key" | "ctrl" | "meta" | "shift" | "alt" | "overrides">): "undo" | "redo" | null {
  for (const verb of ["undo", "redo"] as const) {
    for (const raw of (row.overrides[verb] ?? fixture.defaults[verb]).split(",")) {
      const parts = raw.trim().toLowerCase().split("+").filter(Boolean);
      if (parts.length === 0) continue;
      const wantsMod = parts.includes("mod");
      const modifiersMatch = wantsMod ? row.ctrl !== row.meta : row.ctrl === parts.includes("ctrl") && row.meta === parts.includes("meta");
      if (modifiersMatch && row.shift === parts.includes("shift") && row.alt === parts.includes("alt") && row.key.toLowerCase() === parts[parts.length - 1]) return verb;
    }
  }
  return null;
}

function mount(kind: string): HTMLElement {
  const make = (tag: string, configure: (element: HTMLElement) => void = () => undefined) => {
    const element = document.createElement(tag);
    configure(element);
    document.body.append(element);
    return element;
  };
  const input = (type: string, configure: (element: HTMLInputElement) => void = () => undefined) => make("input", (element) => { (element as HTMLInputElement).type = type; configure(element as HTMLInputElement); });
  switch (kind) {
    case "canvas": return make("canvas", (element) => { element.tabIndex = 0; });
    case "body": return document.body;
    case "button": return make("button");
    case "treeitem": return make("div", (element) => { element.setAttribute("role", "treeitem"); element.tabIndex = 0; });
    case "select": return make("select");
    case "checkbox": return input("checkbox");
    case "range": return input("range");
    case "interpreter-clean": return input("text", (element) => { element.setAttribute("data-ui-node-id", "field"); element.setAttribute("data-draft-dirty", "false"); });
    case "interpreter-dirty": return input("text", (element) => { element.setAttribute("data-ui-node-id", "field"); element.setAttribute("data-draft-dirty", "true"); });
    case "text": return input("text");
    case "number": return input("number");
    case "textarea": return make("textarea");
    case "contenteditable": return make("div", (element) => { element.setAttribute("contenteditable", "true"); });
    case "textbox": return make("div", (element) => { element.setAttribute("role", "textbox"); element.tabIndex = 0; });
    default: throw new Error(`unknown focus target ${kind}`);
  }
}

afterEach(() => document.body.replaceChildren());

describe("⏪️ history key route", () => {
  it("pins the declared defaults to the fixture", () => {
    expect(HISTORY_VERB_CHORDS_V1).toEqual(fixture.defaults);
  });

  for (const row of fixture.chords) {
    it(`${row.id}: the shipped matcher, the oracle and the fixture agree`, () => {
      const event = { key: row.key, ctrlKey: row.ctrl, metaKey: row.meta, shiftKey: row.shift, altKey: row.alt };
      expect(historyVerbForChordV1(event, row.overrides, keyboardEventMatchesChord)).toBe(row.expected);
      expect(oracleVerb(row)).toBe(row.expected);
    });
  }

  it("an override replaces the declared chord of its control id and nothing else", () => {
    expect(effectiveChordsV1("mod+z", "undo", {})).toEqual(["mod+z"]);
    expect(effectiveChordsV1("mod+z", "undo", { undo: "Mod+U, mod+q" })).toEqual(["mod+u", "mod+q"]);
    expect(effectiveChordsV1("mod+z", "undo", { redo: "mod+r" })).toEqual(["mod+z"]);
    expect(effectiveChordsV1("mod+z", "undo", { undo: "" })).toEqual([]);
  });

  it("real keyboard events from the user-event model resolve to the same verbs", async () => {
    const seen: ("undo" | "redo" | null)[] = [];
    const user = userEvent.setup();
    mount("canvas").focus();
    const listener = (event: KeyboardEvent) => { if (!["Control", "Shift"].includes(event.key)) seen.push(historyVerbForChordV1(event, {}, keyboardEventMatchesChord)); };
    document.addEventListener("keydown", listener);
    await user.keyboard("{Control>}z{/Control}");
    await user.keyboard("{Control>}{Shift>}z{/Shift}{/Control}");
    await user.keyboard("{Control>}y{/Control}");
    await user.keyboard("{Control>}{Shift>}y{/Shift}{/Control}");
    document.removeEventListener("keydown", listener);
    expect(seen).toEqual(["undo", "redo", "redo", null]);
  });

  for (const row of fixture.focus) {
    it(`${row.id}: ${row.ownsChord ? "keeps" : "yields"} Ctrl+Z, Ctrl+Shift+Z and Ctrl+Y`, () => {
      const target = mount(row.target);
      for (const key of [{ key: "z", ctrlKey: true, shiftKey: false }, { key: "Z", ctrlKey: true, shiftKey: true }, { key: "y", ctrlKey: true, shiftKey: false }]) {
        expect(controlOwnsKeyboardEvent({ ...key, metaKey: false, altKey: false, target }), `${row.id} ${key.key}`).toBe(row.ownsChord);
      }
    });
  }
});
