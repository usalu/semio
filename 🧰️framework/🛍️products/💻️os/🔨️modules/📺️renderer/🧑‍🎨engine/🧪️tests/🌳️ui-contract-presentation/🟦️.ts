/** 🌳️ Language-neutral wire laws for compact trees and checkbox toggles. */
import type { Component, ToggleAppearance, TreePresentation } from "@semio-tech/framework";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

const engineRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const fixture = JSON.parse(readFileSync(join(engineRoot, "🧫️fixtures", "🌳️ui-contract-presentation", "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(engineRoot, "🧬️schema", "🌳️ui-contract-presentation", "🔣️.json"), "utf8"));
const typedComponents: readonly Component[] = [
  { type: "tree", interactionDomain: null },
  { type: "tree", presentation: "compact", interactionDomain: null },
  { type: "toggle", on: true, icon: "check", text: "Enabled" },
  { type: "toggle", appearance: "checkbox", on: true, icon: "check", text: "Enabled" },
];

const treePresentation = (component: Extract<Component, { type: "tree" }>): TreePresentation => component.presentation ?? "standard";
const toggleAppearance = (component: Extract<Component, { type: "toggle" }>): ToggleAppearance => component.appearance ?? "button";

describe("semantic UI presentation wire", () => {
  test("the neutral fixture rejects unknown members and values", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, unknown: true })).toBe(false);
    const invalid = structuredClone(fixture);
    invalid.cases[1].toggle.appearance = "switch";
    expect(validate(invalid)).toBe(false);
  });

  test("generated TypeScript consumers preserve optional defaults and explicit compact checkbox values", () => {
    for (const [index, row] of fixture.cases.entries()) {
      const tree = { type: "tree", ...row.tree } as Extract<Component, { type: "tree" }>;
      const toggle = { type: "toggle", ...row.toggle } as Extract<Component, { type: "toggle" }>;
      expect(treePresentation(tree)).toBe(row.expected.treePresentation);
      expect(toggleAppearance(toggle)).toBe(row.expected.toggleAppearance);
      expect(JSON.stringify(tree)).toBe(JSON.stringify(typedComponents[index]));
      expect(JSON.stringify(toggle)).toBe(JSON.stringify(typedComponents[index + 2]));
    }
  });
});
