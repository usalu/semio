// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createElement } from "react";
import Ajv2020 from "ajv/dist/2020.js";
import { afterEach, describe, expect, it, vi } from "vitest";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import { cleanup, render } from "@semio-tech/ui-react/test";
import type { Component, UiNodeRecord, UiSnapshot } from "@semio-tech/framework";
import { UiDocumentStore } from "../../🧱️elements/📃️UiDocumentStore/🟦️.tsx";
import { interpretUiNode, type UiInterpreterContext } from "../../🧱️elements/🗣️Interpreter/🟦️.tsx";
import { createAccessibilityMirror } from "../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts";
import { uiAccessibilityProjectionNodeV1 } from "../../../../../../../🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🟦️.ts";

type ToggleCase = {
  readonly id: string;
  readonly component: Component;
  readonly accessibleLabel: string;
  readonly expected: {
    readonly role: "button" | "checkbox";
    readonly elementType: "button" | "input";
    readonly stateAttribute: "aria-pressed" | "aria-checked";
    readonly stateValue: boolean;
    readonly absentAttribute: "aria-pressed" | "aria-checked";
  };
};

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/♿️retained-toggle-semantics/🔣️.json"), "utf8")) as { readonly cases: readonly ToggleCase[] };
const schema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/♿️retained-toggle-semantics/🔣️.json"), "utf8"));
const { computeAccessibleName, getRole }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");

function record(row: ToggleCase, id: number): UiNodeRecord {
  return {
    id,
    key: `#toggle-${id}`,
    component: row.component,
    layout: { kind: "leaf", width: "hug", height: "hug" },
    style: {},
    activity: "idle",
    disabled: false,
    transition: null,
    accessibility: { label: row.accessibleLabel, description: null, live: "off", shortcut: null, hidden: false },
    bindings: [],
    menu: null,
    children: [],
  } as UiNodeRecord;
}

function renderInterpreter(row: ToggleCase, id: number): HTMLElement {
  const store = new UiDocumentStore("toggle.semantic.fixture");
  const node = record(row, id);
  store.loadSnapshot({ surface: "toggle.semantic.fixture", revision: 0, root: id, layoutEpoch: 0n, nodes: [node] } as UiSnapshot);
  const context: UiInterpreterContext = { store, onAction: () => {}, onIntent: () => {} };
  const mounted = render(createElement("div", {}, interpretUiNode(store, context)));
  const element = mounted.container.querySelector<HTMLElement>(row.expected.elementType);
  if (!element) throw new Error(`${row.id}: Interpreter did not render ${row.expected.elementType}`);
  return element;
}

afterEach(() => {
  vi.useRealTimers();
  cleanup();
  document.body.replaceChildren();
});

describe("retained Toggle semantics", () => {
  it("validates the language-neutral appearance and state-channel corpus", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.cases.map((row) => row.component.appearance)).toEqual(["button", "checkbox"]);
  });

  it("matches the actual React Interpreter and WGPU mirror by native role and exclusive state channel", async () => {
    for (const [index, row] of fixture.cases.entries()) {
      const element = renderInterpreter(row, index);
      expect(getRole(element), row.id).toBe(row.expected.role);
      expect(computeAccessibleName(element), row.id).toBe(row.accessibleLabel);
      expect(element.hasAttribute(row.expected.absentAttribute), row.id).toBe(false);
      if (row.expected.stateAttribute === "aria-pressed") {
        expect(element.getAttribute("aria-pressed"), row.id).toBe(String(row.expected.stateValue));
      } else {
        expect((element as HTMLInputElement).checked, row.id).toBe(row.expected.stateValue);
      }

      const projected = uiAccessibilityProjectionNodeV1(record(row, index), 0);
      const root = document.createElement("div");
      document.body.append(root);
      vi.useFakeTimers();
      const mirror = createAccessibilityMirror(root, {
        introspect: async () => JSON.stringify({ windows: [{ windowId: "toggle.semantic.fixture", windowGeneration: 1, nodes: [{ ...projected, label: projected.label ?? undefined, description: undefined, shortcut: undefined, checked: projected.checked ?? undefined, pressed: projected.pressed ?? undefined }] }] }),
        enqueueLossless: () => true,
      }, "en");
      mirror.refresh();
      await vi.runAllTimersAsync();
      const mirrored = root.querySelector<HTMLElement>(`[data-node-key="${projected.key}"]`);
      expect(mirrored, row.id).not.toBeNull();
      expect(getRole(mirrored!), row.id).toBe(row.expected.role);
      expect(mirrored!.getAttribute(row.expected.stateAttribute), row.id).toBe(String(row.expected.stateValue));
      expect(mirrored!.hasAttribute(row.expected.absentAttribute), row.id).toBe(false);
      mirror.dispose();
      root.remove();
      vi.useRealTimers();
      cleanup();
    }
  });
});
