/** ♿️ The native accessibility bridge's language-agnostic contract (`🧬️schema/♿️native-accessibility-tree`) held against the
 * TypeScript UI contract: Ajv holds the fixture to its schema, and every role the shared `uiAccessibilityRoleV1` (the React
 * and wgpu shells' one role rule) or the shell chrome can announce has a platform role in the fixture's table — the table the
 * Rust bridge's law replays with AccessKit's consumer tree as the oracle (`🧪️tests/♿️native-accessibility/🦀️.rs`). */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { describe, expect, test } from "vitest";
import { uiAccessibilityRoleV1 } from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🟦️.ts";
import type { Component } from "../../../../../../../🔨️modules/🛂️manifest/🟦️.ts";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "♿️native-accessibility-tree", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "♿️native-accessibility-tree", "🔣️.json"), "utf8"));

/** 🎭️ Every role the shared contract derives, over every component type and every container role, pressed or not. */
function contractRoles(): Set<string> {
  const roles = new Set<string>();
  const types = ["text", "button", "separator", "input", "select", "keyValueList", "slider", "ring", "numberStepper", "iconSelect", "progress", "tree", "treeSection", "treeItem", "image", "surface", "extension", "table", "tableRow"];
  for (const activatable of [false, true]) {
    for (const role of ["plain", "group", "field", "section", "form", "toolbar"]) roles.add(uiAccessibilityRoleV1({ type: "container", role } as unknown as Component, activatable));
    for (const appearance of ["checkbox", "switch"]) roles.add(uiAccessibilityRoleV1({ type: "toggle", appearance } as unknown as Component, activatable));
    for (const type of types) roles.add(uiAccessibilityRoleV1({ type } as unknown as Component, activatable));
  }
  return roles;
}

/** 🧭️ The roles the wgpu shell chrome announces beside the contract's (`chrome_accessibility_role`, footer status chips). */
const CHROME_ROLES = ["button", "switch", "combobox", "option", "slider", "textbox", "menuitem", "status"];

describe("♿️ native accessibility tree", () => {
  test("the fixture satisfies its schema", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("every contract and chrome role has a platform role in the table", () => {
    const table = new Set<string>(fixture.roles.map((row: { role: string }) => row.role));
    const missing = [...contractRoles(), ...CHROME_ROLES].filter((role) => !table.has(role));
    expect(missing).toEqual([]);
  });

  test("every fixture node's authored role is either in the table or deliberately outside it", () => {
    const table = new Set<string>(fixture.roles.map((row: { role: string }) => row.role));
    const outside = fixture.windows.flatMap((window: { nodes: { role: string; key: string }[] }) => window.nodes.filter((node) => !table.has(node.role)).map((node) => node.key));
    expect(outside).toEqual(["future"]);
  });
});
