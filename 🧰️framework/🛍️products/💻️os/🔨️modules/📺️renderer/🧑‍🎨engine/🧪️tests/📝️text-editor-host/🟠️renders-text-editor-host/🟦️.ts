/** 🟠️ The renders text editor host tests of the framework renderer hosts suite. */
import { editorTextInputFixture as textInputFixture } from "@semio-tech/framework-editor-rs/testing/fixtures";
import { act as reactAct, createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { uiI18n } from "@semio-tech/ui-react";
import * as flowSessionLoader from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { expect, it, vi } from "vitest";
import { type ActionDescriptor } from "@semio-tech/framework";
import { TextEditorHost } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 🟡 Registers the renders text editor host tests of the framework renderer hosts suite. */
export function registerRendersTextEditorHostTestsMore(): void {
  for (const law of textInputFixture.rendererKeys)
    it(`routes the actual React text editor key: ${law.id}`, async () => {
      const attachCanvas = vi.fn(async () => {});
      let text = law.text;
      let selection = law.selection;
      const operations = Object.fromEntries(
        ["moveLeft", "moveRight", "moveUp", "moveDown", "moveLineStart", "moveLineEnd", "insertText"].map((name) => [
          name,
          vi.fn(() => {
            text = law.expect.text;
            selection = law.expect.selection;
          }),
        ]),
      );
      const session = {
        attachCanvas,
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},

        synchronizeScene: () => {},
        setText: () => {},
        text: () => text,
        caret: () => selection[1],
        anchor: () => selection[0],
        setCanvasThemeJson: () => {},
        free: () => {},
        tabInsertText: () => " ".repeat(law.tabSize ?? 2),
        ...operations,
      } as unknown as flowSessionLoader.EditorWasmSession;
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const onAction = vi.fn(async (_action: ActionDescriptor) => undefined);
      const view = render(
        createElement(TextEditorHost, {
          node: {
            type: "componentScene",
            surfaceId: "writer.keys",
            controllerId: "writer",
            componentKind: "text-editor",
            textEditor: {
              buffer: law.text,
              language: "jack",
              selectionJson: JSON.stringify({ start: law.selection[0], end: law.selection[1] }),
              newlineGatesJson: law.newlineGates === undefined ? undefined : JSON.stringify(law.newlineGates),
            },
          },
          onAction,
        }),
      );
      try {
        await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
        await reactAct(async () => {
          await Promise.resolve();
        });
        const area = view.container.querySelector("textarea")!;
        fireEvent.keyDown(area, { key: law.key, shiftKey: law.shift ?? false, altKey: law.alt ?? false });
        if (law.operation !== null) {
          expect(operations[law.operation]).toHaveBeenCalledWith(law.argument);
          await waitFor(() => expect(onAction).toHaveBeenCalledWith({ controllerId: "writer", action: "textSelect", args: { surfaceId: "writer.keys", start: law.expect.selection[0], end: law.expect.selection[1] } }));
          const actions = onAction.mock.calls.map(([action]) => action);
          expect(actions.map((action) => action.action)).toEqual(law.operation === "insertText" ? ["textEdit", "textSelect"] : ["textSelect"]);
          if (law.operation === "insertText") expect(actions[0]?.args).toEqual({ surfaceId: "writer.keys", text: law.expect.text, typing: "writer.keys" });
        } else {
          for (const operation of Object.values(operations)) expect(operation).not.toHaveBeenCalled();
          expect(onAction).not.toHaveBeenCalled();
        }
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });
  for (const stepKind of ["paste", "compose"] as const)
    it(`routes the actual React text editor ${stepKind} event through the shared text-input fixture`, async () => {
      await uiI18n.changeLanguage("en");
      const stepText = (step: (typeof textInputFixture.sequences)[number]["steps"][number]): string | undefined => (stepKind === "paste" ? ("paste" in step ? step.paste : undefined) : "compose" in step ? step.compose : undefined);
      const law = textInputFixture.sequences.find((sequence) => sequence.steps.some((step) => stepText(step) !== undefined))!;
      const committed = law.steps.map(stepText).find((text) => text !== undefined)!;
      let text = law.text;
      let selection = law.selection;
      const commit = vi.fn(() => {
        text = law.expect.text;
        selection = law.expect.selection;
      });
      const session = {
        attachCanvas: vi.fn(async () => {}),
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},

        synchronizeScene: () => {},
        setText: () => {},
        text: () => text,
        caret: () => selection[1],
        anchor: () => selection[0],
        setCanvasThemeJson: () => {},
        free: () => {},
        replaceSelection: commit,
        insertText: commit,
      } as unknown as flowSessionLoader.EditorWasmSession;
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const onAction = vi.fn(async (_action: ActionDescriptor) => undefined);
      const view = render(
        createElement(TextEditorHost, {
          node: { type: "componentScene", surfaceId: `writer.${stepKind}`, controllerId: "writer", componentKind: "text-editor", textEditor: { buffer: law.text, selectionJson: JSON.stringify({ start: law.selection[0], end: law.selection[1] }) } },
          onAction,
        }),
      );
      try {
        await waitFor(() => expect(session.attachCanvas).toHaveBeenCalledOnce());
        const area = view.container.querySelector("textarea")!;
        expect(area.getAttribute("aria-label")).toBe("Artifact");
        expect(area.getAttribute("aria-readonly")).toBe("false");
        const event = new Event(stepKind === "paste" ? "paste" : "compositionend", { bubbles: true, cancelable: true });
        if (stepKind === "paste") Object.defineProperty(event, "clipboardData", { value: { getData: (type: string) => (type === "text/plain" ? committed : "") } });
        else Object.defineProperty(event, "data", { value: committed });
        await reactAct(async () => {
          area.dispatchEvent(event);
        });
        await waitFor(() => expect(onAction).toHaveBeenCalledTimes(2));
        expect(commit).toHaveBeenCalledWith(committed);
        expect(onAction.mock.calls.map(([action]) => action)).toEqual([
          { controllerId: "writer", action: "textEdit", args: { surfaceId: `writer.${stepKind}`, text: law.expect.text, typing: `writer.${stepKind}` } },
          { controllerId: "writer", action: "textSelect", args: { surfaceId: `writer.${stepKind}`, start: law.expect.selection[0], end: law.expect.selection[1] } },
        ]);
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });
  it("renders text editor host with hover/newline/rename scene fields", () => {
    const markup = renderToStaticMarkup(
      createElement(TextEditorHost, {
        node: {
          type: "componentScene",
          surfaceId: "writer.play.editor",
          controllerId: "writer-play",
          componentKind: "text-editor",
          textEditor: {
            buffer: "MATCH (a:Piece) RETURN a.name",
            language: "jack",
            hoverJson: '{"start":0,"end":5}',
            newlineGatesJson: "[30]",
            renameJson: '{"name":"a","occurrences":[{"start":7,"end":8}]}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-text-editor-host");
  });
}
