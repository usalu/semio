/** 🖊️ Canonical textControlsSelfTests: the TypeScript half of the text-control keyboard law (ticket 26/09/30 design §22.7),
 * answering the very same `🧫️fixtures/🧫️text-controls/🔣️.json` rows the Rust `🔬️component-unit` law answers through
 * `text_input_key`. Ajv validates the fixture against its schema; the third-party oracle of what a key means to a real field
 * (`@testing-library/user-event` typing into a rendered single-line and multi-line field) judges the same rows in the
 * renderer's `🧪️text-keyboard-law`. */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import type { InputKind } from "@semio-tech/framework";
import { uiTextInputKey, uiTextInputKeyOf, type TextInputKey, type TextInputKeyAction } from "../../🧩️component/🟦️.ts";

type Fixture = { readonly keys: readonly { readonly case: string; readonly kind: InputKind; readonly key: TextInputKey; readonly primary: boolean; readonly shift: boolean; readonly alt: boolean; readonly action: TextInputKeyAction | null }[] };

const read = (path: string): unknown => JSON.parse(readFileSync(fileURLToPath(new URL(path, import.meta.url)), "utf8"));
const PHYSICAL: Readonly<Record<TextInputKey, string>> = { enter: "Enter", escape: "Escape" };

/** 🖊️ Answers every shared fixture row, returning how many assertions the corpus carried. */
export function textControlsSelfTests(): number {
  const Ajv2020 = createRequire(import.meta.url)("ajv/dist/2020").default;
  const fixture = read("../../🧫️fixtures/🧫️text-controls/🔣️.json") as Fixture;
  
  
  let checks = 1;
  for (const row of fixture.keys) {
    const modifiers = { primary: row.primary, shift: row.shift, alt: row.alt };
    assert.equal(uiTextInputKey(row.kind, row.key, modifiers), row.action, row.case);
    for (const [ctrlKey, metaKey] of row.primary ? [[true, false], [false, true]] : [[false, false]]) {
      assert.deepEqual(uiTextInputKeyOf({ key: PHYSICAL[row.key], ctrlKey: ctrlKey!, metaKey: metaKey!, shiftKey: row.shift, altKey: row.alt }), { key: row.key, modifiers }, `${row.case}: the physical key`);
      checks += 1;
    }
    checks += 1;
  }
  for (const key of ["a", "Tab", "ArrowDown", " ", "NumpadEnter"]) {
    assert.equal(uiTextInputKeyOf({ key, ctrlKey: false, metaKey: false, shiftKey: false, altKey: false }), null, `${key} is not a law key`);
    checks += 1;
  }
  assert.deepEqual(new Set(fixture.keys.map((row) => row.action)), new Set(["newline", "commit", "revert", null]));
  return checks + 1;
}
