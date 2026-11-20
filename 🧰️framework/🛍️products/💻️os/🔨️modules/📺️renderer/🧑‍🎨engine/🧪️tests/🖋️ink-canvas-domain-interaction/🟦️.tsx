/** 🖋️ Mounted React oracle for InkCanvas's canonical interaction-domain wire contract. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { createElement } from "react";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { InkCanvasHost } from "../../🧱️elements/🖋️InkCanvasHost/🟦️.tsx";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const engineRoot = resolve(suiteRoot, "../..");
const fixture = JSON.parse(readFileSync(resolve(engineRoot, "🧪️tests/🧫️fixtures/🖋️ink-canvas-domain-interaction/🔣️.json"), "utf8")) as any;

function mountHost(activeUtility = "selectDirect", interactive = true) {
  const actions: any[] = [];
  const node = {
    type: "componentScene",
    surfaceId: fixture.scene.surfaceId,
    controllerId: fixture.scene.controllerId,
    componentKind: "ink-canvas",
    inkCanvas: {
      documentJson: JSON.stringify(fixture.document),
      selectionJson: fixture.scene.selectionJson,
      hoveredId: fixture.scene.hoveredId,
      activeUtility,
      viewMode: "composite",
      interactive,
      interactionDomain: fixture.scene.interactionDomain,
    },
  };
  const mounted = render(createElement(InkCanvasHost, { node, onAction: (action: unknown) => actions.push(action) } as any));
  const root = mounted.container.querySelector(`[data-surface-id="${fixture.scene.surfaceId}"]`) as HTMLDivElement;
  Object.defineProperty(root, "getBoundingClientRect", {
    value: () => ({ x: 0, y: 0, left: 0, top: 0, right: fixture.viewport.width, bottom: fixture.viewport.height, width: fixture.viewport.width, height: fixture.viewport.height, toJSON: () => ({}) }),
  });
  return { actions, mounted, root };
}

function expectedAction(testCase: any): any {
  return {
    controllerId: fixture.scene.controllerId,
    action: testCase.action,
    args: { surfaceId: fixture.scene.surfaceId, ...testCase.args },
  };
}

function caseById(id: string): any {
  return fixture.cases.find((entry: any) => entry.id === id);
}

afterEach(() => cleanup());

describe("InkCanvas canonical interaction domain", () => {
  it("validates the neutral cross-renderer contract and forbids removed app-private selection verbs", () => {
    expect(fixture.identity).toEqual({ requiresInteractionId: true, missingInteractionId: "refuse-target", rawIdFallback: false, incomingPaintIds: "raw" });
    expect(fixture.forbiddenActions).toEqual(["setHover", "setSelection"]);
    const validateDomain = new Ajv().compile(fixture.interactionDomainSchema);
    expect(validateDomain(fixture.scene.interactionDomain)).toBe(true);
    for (const invalid of fixture.invalidInteractionDomains) {
      expect(validateDomain(invalid)).toBe(false);
    }
  });

  it("publishes canonical topology hover and an exact empty hover clear", () => {
    const { actions, root } = mountHost();
    for (const id of ["pointer-hover", "pointer-background"]) {
      const testCase = caseById(id);
      fireEvent.pointerMove(root, { clientX: testCase.gesture.x, clientY: testCase.gesture.y, pointerId: 7 });
      expect(actions.at(-1)).toEqual(expectedAction(testCase));
    }
    expect(actions.every((entry) => !fixture.forbiddenActions.includes(entry.action))).toBe(true);
  });

  it("clears the pointer hover when the pointer leaves directly from a block", () => {
    const { actions, root } = mountHost();
    const hover = caseById("pointer-hover");
    fireEvent.pointerMove(root, { clientX: hover.gesture.x, clientY: hover.gesture.y, pointerId: 7 });
    fireEvent.pointerLeave(root, { pointerId: 7 });
    expect(actions).toEqual([expectedAction(hover), expectedAction(caseById("pointer-leave"))]);
  });

  it("does not publish pointer hover from a read-only scene", () => {
    const { actions, root } = mountHost("selectDirect", false);
    fireEvent.pointerLeave(root, { pointerId: 7 });
    expect(actions).toEqual([]);
  });

  it("publishes replace, additive and empty picks through interactionSelect", () => {
    for (const id of ["replace-pick", "additive-pick", "background-clear"]) {
      const { actions, root } = mountHost();
      const testCase = caseById(id);
      const target = testCase.gesture.blockId ? root.querySelector(`[data-ink-block-id="${testCase.gesture.blockId}"]`) : root;
      expect(target).not.toBeNull();
      fireEvent.pointerDown(target!, { button: 0, pointerId: 11, shiftKey: testCase.gesture.shiftKey, clientX: testCase.gesture.x ?? 40, clientY: testCase.gesture.y ?? 40 });
      expect(actions.at(-1)).toEqual(expectedAction(testCase));
      cleanup();
    }
  });

  it("maps a completed rectangle to canonical topology ids", () => {
    const { actions, root } = mountHost("selectMarquee");
    const testCase = caseById("rectangle-marquee");
    fireEvent.pointerDown(root, { button: 0, pointerId: 13, clientX: testCase.gesture.from.x, clientY: testCase.gesture.from.y });
    fireEvent.pointerMove(root, { pointerId: 13, clientX: testCase.gesture.to.x, clientY: testCase.gesture.to.y });
    fireEvent.pointerUp(root, { button: 0, pointerId: 13, clientX: testCase.gesture.to.x, clientY: testCase.gesture.to.y });
    expect(actions.at(-1)).toEqual(expectedAction(testCase));
  });

  it("paints incoming projected raw selection and hover ids", () => {
    const { root } = mountHost();
    const selected = root.querySelector('[data-ink-block-id="text-a"]') as HTMLDivElement;
    const hovered = root.querySelector('[data-ink-block-id="image-b"]') as HTMLDivElement;
    expect(selected.getAttribute("data-shared-border-state")).toBe("active");
    expect(hovered.getAttribute("data-shared-border-state")).toBe("hover");
    for (const block of [selected, hovered]) {
      expect(block.getAttribute("data-shared-border-edges")).toBe("");
      expect(block.className).not.toMatch(/\bring/u);
    }
  });
});
