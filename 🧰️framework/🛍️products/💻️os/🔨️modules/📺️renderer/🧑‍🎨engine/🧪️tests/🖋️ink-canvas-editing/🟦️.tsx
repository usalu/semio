/**
 * 🖋️ React host oracle for the shared InkCanvas editing law consumed by native wgpu tests.
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createElement } from "react";
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { InkCanvasHost } from "../../🧱️elements/🖋️InkCanvasHost/🟦️.tsx";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const law = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖋️ink-canvas-editing/🔣️.json"), "utf8")) as any;

function actionEvents(actions: readonly any[]): any[] {
  return actions.filter((action) => action.action === "inkApplyEvents").map((action) => ({ phase: action.args.phase, events: JSON.parse(action.args.eventsJson) }));
}

describe("InkCanvas text and table-cell editing", () => {
  afterEach(() => cleanup());

  it("commits text on blur, advances table cells on Enter and Tab, and cancels text on Escape", () => {
    const actions: any[] = [];
    const node = { type: "componentScene", ...law.scene, inkCanvas: { ...law.scene.inkCanvas, documentJson: JSON.stringify(law.document) } };
    const rendered = render(createElement(InkCanvasHost, { node, onAction: (action: unknown) => actions.push(action) } as any));
    const root = rendered.container.querySelector(`[data-surface-id="${law.scene.surfaceId}"]`) as HTMLDivElement;
    Object.defineProperty(root, "getBoundingClientRect", { value: () => ({ x: 0, y: 0, left: 0, top: 0, right: law.viewport.width, bottom: law.viewport.height, width: law.viewport.width, height: law.viewport.height, toJSON: () => ({}) }) });

    fireEvent.doubleClick(root, { clientX: law.gestures.text.x, clientY: law.gestures.text.y });
    const textEditor = root.querySelector('[contenteditable="true"]') as HTMLDivElement;
    expect(textEditor).not.toBeNull();
    textEditor.innerHTML = `<div>${law.gestures.text.replacement}</div>`;
    fireEvent.blur(textEditor);
    expect(actionEvents(actions)[0]).toEqual({ phase: law.expected.phase, events: [law.expected.textEvent] });

    fireEvent.doubleClick(root, { clientX: law.gestures.table.x, clientY: law.gestures.table.y });
    const tableEditor = root.querySelector("input") as HTMLInputElement;
    expect(tableEditor.value).toBe("b");
    fireEvent.change(tableEditor, { target: { value: law.gestures.table.replacement } });
    fireEvent.keyDown(tableEditor, { key: "Enter" });
    expect(actionEvents(actions)[1]).toEqual({ phase: law.expected.phase, events: [law.expected.tableEvent] });
    expect((root.querySelector("input") as HTMLInputElement).value).toBe("c");

    const enteredDocument = { ...law.document, blocks: law.document.blocks.map((block: any) => (block.id === law.expected.tableEvent.blockId ? law.expected.tableEvent.block : block)) };
    const advancedNode = { ...node, inkCanvas: { ...node.inkCanvas, documentJson: JSON.stringify(enteredDocument) } };
    rendered.rerender(createElement(InkCanvasHost, { node: advancedNode, onAction: (action: unknown) => actions.push(action) } as any));
    const advancedTableEditor = root.querySelector("input") as HTMLInputElement;
    fireEvent.change(advancedTableEditor, { target: { value: "z" } });
    fireEvent.keyDown(advancedTableEditor, { key: "Tab" });
    expect(actionEvents(actions)[2]).toEqual({ phase: law.expected.phase, events: [law.expected.tableTabEvent] });
    expect(root.querySelector("input")).toBeNull();

    const beforeCancel = actionEvents(actions).length;
    fireEvent.doubleClick(root, { clientX: law.gestures.text.x, clientY: law.gestures.text.y });
    const cancelled = root.querySelector('[contenteditable="true"]') as HTMLDivElement;
    cancelled.innerHTML = "<div>cancelled</div>";
    fireEvent.keyDown(cancelled, { key: "Escape" });
    expect(root.querySelector('[contenteditable="true"]')).toBeNull();
    expect(actionEvents(actions)).toHaveLength(beforeCancel);
  });
});
