import { describe, it, expect } from "vitest";
import { act, render } from "@testing-library/react";
import { Diagram, type Node } from "../../../🟦️.tsx";
import { DIAGRAM_KEYBOARD_FOCUS_CLASS, type DiagramNavigableNode, type DiagramKeyboardStep, diagramKeyboardStep, diagramReadingOrderPosition, diagramKeyboardAnnouncement, diagramArrowNavigationTarget, DIAGRAM_ARROW_KEYS, diagramActivateSelection } from "../../../⌨️keyboard/🟦️.tsx";
import keyboardFixture from "../../🧫️fixtures/⌨️keyboard.json";
import { flowCapture } from "../🧰️harness/🟦️.ts";

describe("♿️ diagram keyboard and ARIA affordances", () => {
  const grid = [
    { id: "a", position: { x: 0, y: 0 }, data: {} },
    { id: "b", position: { x: 200, y: 0 }, data: {} },
    { id: "c", position: { x: 0, y: 200 }, data: {} },
    { id: "d", position: { x: 200, y: 200 }, data: {} },
  ];

  const surfaceOf = (container: HTMLElement) => container.querySelector('[role="application"]') as HTMLElement;

  const press = (element: HTMLElement, key: string, init: KeyboardEventInit = {}) =>
    act(() => {
      element.dispatchEvent(new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true, ...init }));
    });

  it("exposes an application-role canvas that is a real tab stop, labelled and self-describing", () => {
    const view = render(<Diagram nodeTypes={{}} initialNodes={grid} initialEdges={[{ id: "a-b", source: "a", target: "b" }]} forceConfig={{ enabled: false }} />);
    const surface = surfaceOf(view.container);
    expect(surface).toBeTruthy();
    expect(surface.getAttribute("tabindex")).toBe("0");
    expect(surface.getAttribute("aria-roledescription")).toBeTruthy();
    expect(surface.getAttribute("aria-label")).toContain("4");
    expect(surface.getAttribute("aria-label")).toContain("1");
    const describedBy = surface.getAttribute("aria-describedby");
    expect(describedBy).toBeTruthy();
    const help = [...view.container.querySelectorAll("[id]")].find((element) => element.id === describedBy);
    expect(help?.textContent ?? "").toMatch(/Arrow keys|Pfeiltasten/u);
  });

  const announcementOf = (container: HTMLElement) => container.querySelector("[data-diagram-announcement]") as HTMLElement;

  it("walks nodes with the arrow keys, rings the focused node and speaks it through a polite live region", () => {
    const view = render(<Diagram nodeTypes={{}} initialNodes={grid} initialEdges={[]} forceConfig={{ enabled: false }} />);
    const surface = surfaceOf(view.container);
    const live = announcementOf(view.container);
    expect(live.getAttribute("role")).toBe("status");
    expect(live.getAttribute("aria-live")).toBe("polite");
    expect(surface.getAttribute("data-diagram-focused-node")).toBeNull();
    expect(surface.hasAttribute("aria-activedescendant")).toBe(false);
    press(surface, "ArrowRight");
    expect(surface.getAttribute("data-diagram-focused-node")).toBe("a");
    expect(live.textContent).toMatch(/^a, 1 (of|von) 4$/u);
    const ringed = (node: Node) => DIAGRAM_KEYBOARD_FOCUS_CLASS.split(" ").every((token) => (node.className ?? "").split(" ").includes(token));
    expect(ringed(flowCapture.props?.nodes.find((node: Node) => node.id === "a"))).toBe(true);
    press(surface, "ArrowRight");
    expect(surface.getAttribute("data-diagram-focused-node")).toBe("b");
    press(surface, "ArrowDown");
    expect(surface.getAttribute("data-diagram-focused-node")).toBe("d");
    expect(live.textContent).toMatch(/^d, 4 (of|von) 4$/u);
    press(surface, "ArrowLeft");
    expect(surface.getAttribute("data-diagram-focused-node")).toBe("c");
    press(surface, "ArrowUp");
    expect(surface.getAttribute("data-diagram-focused-node")).toBe("a");
    expect(flowCapture.props?.nodes.filter(ringed).map((node: Node) => node.id)).toEqual(["a"]);
  });

  it("selects through the SAME node-change lane a mouse pick uses, and Shift+Enter accumulates", () => {
    const changes: any[][] = [];
    const view = render(<Diagram nodeTypes={{}} initialNodes={grid} initialEdges={[]} forceConfig={{ enabled: false }} onNodesChangeReactFlow={(batch) => changes.push(batch)} />);
    const surface = surfaceOf(view.container);
    press(surface, "ArrowRight");
    press(surface, "Enter");
    expect(changes.at(-1)).toEqual([{ id: "a", type: "select", selected: true }]);
    press(surface, "ArrowRight");
    press(surface, "Enter", { shiftKey: true });
    expect(changes.at(-1)).toEqual([{ id: "b", type: "select", selected: true }]);
    expect(announcementOf(view.container).textContent).toMatch(/^b (selected|ausgewählt)/u);
    press(surface, "Escape");
    expect(changes.at(-1)!.every((change: any) => change.selected === false)).toBe(true);
    expect(surface.getAttribute("data-diagram-focused-node")).toBeNull();
    expect(announcementOf(view.container).textContent).toMatch(/^(Selection cleared|Auswahl aufgehoben)$/u);
  });

  it("stays out of the way of text entry inside the canvas", () => {
    const changes: any[][] = [];
    const view = render(<Diagram nodeTypes={{}} initialNodes={grid} initialEdges={[]} forceConfig={{ enabled: false }} onNodesChangeReactFlow={(batch) => changes.push(batch)} />);
    const surface = surfaceOf(view.container);
    const input = document.createElement("input");
    surface.appendChild(input);
    const before = changes.length;
    act(() => {
      input.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowRight", bubbles: true, cancelable: true }));
      input.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true, cancelable: true }));
    });
    expect(changes.length).toBe(before);
    expect(surface.getAttribute("data-diagram-focused-node")).toBeNull();
  });
});

describe("♿️ diagram keyboard law — 🧫️fixtures/⌨️keyboard.json", () => {
  const fixture = keyboardFixture as unknown as {
    readonly nodes: readonly DiagramNavigableNode[];
    readonly sequences: readonly { readonly name: string; readonly editable: boolean; readonly presses: readonly { readonly key: string; readonly shift?: boolean; readonly step: DiagramKeyboardStep; readonly position?: number }[] }[];
  };
  for (const sequence of fixture.sequences) {
    it(sequence.name, () => {
      let focusedId: string | null = null;
      let selectedIds: readonly string[] = [];
      sequence.presses.forEach((press, index) => {
        const step = diagramKeyboardStep(fixture.nodes, focusedId, selectedIds, press.key, press.shift ?? false, sequence.editable);
        expect(step, `${sequence.name} press ${index} (${JSON.stringify(press.key)})`).toEqual(press.step);
        if (step?.kind === "focus") {
          focusedId = step.focusedId;
          expect(diagramReadingOrderPosition(fixture.nodes, step.focusedId)).toBe(press.position);
        } else if (step?.kind === "select") {
          selectedIds = step.selectedIds;
        } else if (step?.kind === "clear") {
          focusedId = null;
          selectedIds = [];
        }
      });
    });
  }

  it("announces through the caller's translator only — no language is baked in", () => {
    const calls: [string, Record<string, unknown> | undefined][] = [];
    const translate = (key: string, options?: Record<string, unknown>) => {
      calls.push([key, options]);
      return `${key}|${JSON.stringify(options ?? {})}`;
    };
    const labelOf = (id: string) => `node ${id}`;
    expect(diagramKeyboardAnnouncement({ kind: "focus", focusedId: "d" }, fixture.nodes, labelOf, translate)).toBe('ui.diagram.focusedNode|{"node":"node d","position":4,"count":4}');
    expect(diagramKeyboardAnnouncement({ kind: "select", focusedId: "b", selectedIds: ["a", "b"], added: true }, fixture.nodes, labelOf, translate)).toBe('ui.diagram.selectedNode|{"node":"node b","count":2}');
    expect(diagramKeyboardAnnouncement({ kind: "select", focusedId: "b", selectedIds: ["a"], added: false }, fixture.nodes, labelOf, translate)).toBe('ui.diagram.deselectedNode|{"node":"node b","count":1}');
    expect(diagramKeyboardAnnouncement({ kind: "clear" }, fixture.nodes, labelOf, translate)).toBe("ui.diagram.selectionCleared|{}");
    expect(calls).toHaveLength(4);
  });
});

describe("♿️ diagram arrow navigation math", () => {
  const nodes = [
    { id: "a", position: { x: 0, y: 0 } },
    { id: "b", position: { x: 200, y: 0 } },
    { id: "c", position: { x: 0, y: 200 } },
  ];

  it("enters at the extreme node opposite the direction when nothing is focused", () => {
    expect(diagramArrowNavigationTarget(nodes, null, "right")).toBe("a");
    expect(diagramArrowNavigationTarget(nodes, null, "left")).toBe("b");
    expect(diagramArrowNavigationTarget(nodes, undefined, "down")).toBe("a");
  });

  it("prefers the node in the pressed half-plane over a merely closer diagonal one", () => {
    const skewed = [
      { id: "origin", position: { x: 0, y: 0 } },
      { id: "right", position: { x: 120, y: 0 } },
      { id: "diagonal", position: { x: 40, y: 90 } },
    ];
    expect(diagramArrowNavigationTarget(skewed, "origin", "right")).toBe("right");
    expect(diagramArrowNavigationTarget(skewed, "origin", "down")).toBe("diagonal");
  });

  it("answers null at the edge of the graph and for an empty graph", () => {
    expect(diagramArrowNavigationTarget(nodes, "b", "right")).toBeNull();
    expect(diagramArrowNavigationTarget(nodes, "a", "up")).toBeNull();
    expect(diagramArrowNavigationTarget([], null, "right")).toBeNull();
  });

  it("maps exactly the four arrow keys and nothing else", () => {
    expect(DIAGRAM_ARROW_KEYS).toEqual({ ArrowUp: "up", ArrowDown: "down", ArrowLeft: "left", ArrowRight: "right" });
    expect(DIAGRAM_ARROW_KEYS.Tab).toBeUndefined();
  });

  it("Enter replaces the selection and Shift+Enter toggles membership", () => {
    expect(diagramActivateSelection(["x", "y"], "z", false)).toEqual(["z"]);
    expect(diagramActivateSelection(["x", "y"], "z", true)).toEqual(["x", "y", "z"]);
    expect(diagramActivateSelection(["x", "y"], "y", true)).toEqual(["x"]);
  });
});
