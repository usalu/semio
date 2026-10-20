/** 🔶️ The renders text editor host tests of the framework renderer hosts suite. */
import { editorTextInputFixture as textInputFixture } from "@semio-tech/framework-editor-rs/testing/fixtures";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import * as flowSessionLoader from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { afterEach, expect, it, vi } from "vitest";
import { TextEditorHost } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 🔷 Registers the renders text editor host tests of the framework renderer hosts suite. */
export function registerRendersTextEditorHostTests(): void {
  afterEach(() => cleanup());
  it("renders text editor host", () => {
    const markup = renderToStaticMarkup(
      createElement(TextEditorHost, {
        node: {
          type: "componentScene",
          surfaceId: "writer.play.editor",
          controllerId: "writer-play",
          componentKind: "text-editor",
          textEditor: {
            buffer: "hello",
            language: "jack",
            tokensJson: JSON.stringify([{ class: "ident", start: 0, end: 5 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-text-editor-host");
    expect(markup).toContain("hello");
  });
  it("retires a mounted text selection drag without synthesizing an up or a second selection action", async () => {
    const attachCanvas = vi.fn(async () => {});
    const pointerDownScreen = vi.fn();
    const pointerUpScreen = vi.fn();
    const pointerCancelScreen = vi.fn();
    const onAction = vi.fn();
    const session = {
      attachCanvas,
      setCaretVisible: vi.fn(),
      setSize: () => {},
      renderFrame: vi.fn(),

      synchronizeScene: () => {},
      setText: () => {},
      text: () => "hello",
      caret: () => 2,
      anchor: () => 1,
      pointerDownScreen,
      pointerMoveScreen: () => {},
      pointerUpScreen,
      pointerCancelScreen,
      wheelScrollScreen: () => {},
      insertText: () => {},
      backspace: () => {},
      deleteForward: () => {},
      selectAll: () => {},
      replaceSelection: () => {},
      selectionText: () => "e",
      hoverTokenRangeJson: () => "null",
      setHoverRange: () => {},
      cameraJson: () => "{}",
      setCanvasThemeJson: () => {},
      moveLeft: () => {},
      moveRight: () => {},
      moveUp: () => {},
      moveDown: () => {},
      moveLineStart: () => {},
      moveLineEnd: () => {},
      tabInsertText: () => "  ",
      setSelectionRange: () => {},
      selectSpanAt: () => {},
      selectSpanAtScreen: () => {},
      pickTargetsAtScreenJson: () => "[]",
      free: vi.fn(),
    } as unknown as flowSessionLoader.EditorWasmSession;
    const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const view = render(
      createElement(TextEditorHost, {
        node: { type: "componentScene", surfaceId: "writer.cancel", controllerId: "writer", componentKind: "text-editor", textEditor: { buffer: "hello", language: "jack" } },
        onAction,
      }),
    );
    try {
      await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
      const surfaces = view.container.querySelectorAll(".semio-text-editor-host div.absolute.inset-0");
      const surface = surfaces.item(surfaces.length - 1) as HTMLElement;
      await waitFor(() => {
        fireEvent.pointerDown(surface, { pointerId: 4, button: 0, clientX: 12, clientY: 14 });
        expect(pointerDownScreen).toHaveBeenCalled();
      });
      const downsBeforeCancel = pointerDownScreen.mock.calls.length;
      const actionsBeforeCancel = onAction.mock.calls.length;
      fireEvent.pointerCancel(surface, { pointerId: 4, clientX: 20, clientY: 14 });
      expect(pointerCancelScreen).toHaveBeenCalledOnce();
      expect(pointerUpScreen).not.toHaveBeenCalled();
      expect(onAction).toHaveBeenCalledTimes(actionsBeforeCancel);
      fireEvent.pointerDown(surface, { pointerId: 5, button: 0, clientX: 24, clientY: 14 });
      expect(pointerDownScreen).toHaveBeenCalledTimes(downsBeforeCancel + 1);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
    }
  });
  it("retires only the closed React text editor and remounts a fresh sibling generation", async () => {
    const law = textInputFixture.hostLifecycle;
    const sessions = [law.text, law.text, law.expect.successorText].map((text) => ({
      attachCanvas: vi.fn(async () => {}),
      setCaretVisible: vi.fn(),
      setSize: () => {},
      renderFrame: () => {},

      synchronizeScene: () => {},
      setText: () => {},
      text: () => text,
      caret: () => text.length,
      anchor: () => text.length,
      setCanvasThemeJson: () => {},
      free: vi.fn(),
    }));
    let index = 0;
    const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockImplementation(async () => sessions[index++] as unknown as flowSessionLoader.EditorWasmSession);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const editor = (window: string, buffer: string) =>
      createElement(TextEditorHost, {
        key: window,
        node: { type: "componentScene", surfaceId: law.surfaceId, controllerId: window, componentKind: "text-editor", textEditor: { buffer, language: "jack" } },
        onAction: noopAction,
      });
    const view = render(createElement("div", null, editor(law.closingWindow, law.text), editor(law.siblingWindow, law.text)));
    try {
      await waitFor(() => expect(sessions[1].attachCanvas).toHaveBeenCalledOnce());
      const before = view.container.querySelectorAll(".semio-text-editor-host").length;
      view.rerender(createElement("div", null, editor(law.siblingWindow, law.text)));
      expect(view.container.querySelectorAll(".semio-text-editor-host").length).toBe(law.expect.survivingHosts);
      expect(before - view.container.querySelectorAll(".semio-text-editor-host").length).toBe(law.expect.closedHosts);
      expect(view.container.querySelector("textarea")?.value).toBe(law.expect.siblingText);
      expect(sessions[1].attachCanvas).toHaveBeenCalledOnce();
      view.rerender(createElement("div", null, editor(law.closingWindow, law.expect.successorText), editor(law.siblingWindow, law.text)));
      await waitFor(() => expect(sessions[2].attachCanvas).toHaveBeenCalledOnce());
      expect(Array.from(view.container.querySelectorAll("textarea"), (area) => area.value)).toEqual([law.expect.successorText, law.expect.siblingText]);
      expect(sessions[1].attachCanvas).toHaveBeenCalledOnce();
      expect(factory).toHaveBeenCalledTimes(3);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
    }
  });
}
