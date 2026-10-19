/** 📝 Text editor host, selection retirement and span edits. */
import { editorTextInputFixture as textInputFixture } from "@semio-tech/framework-editor-rs/testing/fixtures";
import editorDeliveryFixture from "../../🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📮️delivery/🔣️.json";
import { act as reactAct, createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { uiI18n } from "@semio-tech/ui-react";
import * as flowSessionLoader from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { type ActionDescriptor } from "@semio-tech/framework";
import { TextEditorHost, lineRangeAt, multiSpanReplace } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
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

  for (const law of editorDeliveryFixture.cases)
    it(`settles actual React editor delivery: ${law.id}`, async () => {
      const { decodePackValue } = await import("@semio-tech/framework-os");
      let text = law.initial;
      let caret = text.length;
      const session = {
        attachCanvas: vi.fn(async () => {}),
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},

        setText: () => {},
        synchronizeScene: (pack: Uint8Array) => {
          const scene = decodePackValue(pack) as { buffer?: string; selection?: { end: number } };
          if (scene.buffer !== undefined) text = scene.buffer;
          if (scene.selection !== undefined) caret = scene.selection.end;
        },
        text: () => text,
        caret: () => caret,
        anchor: () => caret,
        setCanvasThemeJson: () => {},
        free: () => {},
        insertText: (value: string) => {
          text = text.slice(0, caret) + value + text.slice(caret);
          caret += value.length;
        },
      };
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session as unknown as flowSessionLoader.EditorWasmSession);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const completions: Array<(outcome: unknown) => void> = [];
      const dispatched: string[] = [];
      const onAction = vi.fn((action: ActionDescriptor) => {
        const args = action.args as { readonly text?: string; readonly start?: number; readonly end?: number } | undefined;
        dispatched.push(action.action === "textEdit" ? `textEdit:${args?.text}` : `textSelect:${args?.start}:${args?.end}`);
        return action.action === "textEdit" ? new Promise((resolve) => completions.push(resolve)) : Promise.resolve(undefined);
      });
      const view = render(
        createElement(TextEditorHost, {
          node: {
            type: "componentScene",
            surfaceId: "writer.delivery",
            controllerId: "writer",
            componentKind: "text-editor",
            textEditor: {
              buffer: law.initial,
              selectionJson: JSON.stringify({ start: caret, end: caret }),
            },
          },
          onAction,
        }),
      );
      try {
        await waitFor(() => expect(session.attachCanvas).toHaveBeenCalledOnce());
        await reactAct(async () => {
          await Promise.resolve();
        });
        const area = view.container.querySelector("textarea")!;
        for (const key of law.typed) fireEvent.keyDown(area, { key });
        const typedAfterNewOwner = (law as { readonly typedAfterNewOwner?: readonly string[] }).typedAfterNewOwner ?? [];
        if (typedAfterNewOwner.length > 0) {
          view.rerender(createElement(TextEditorHost, { node: { type: "componentScene", surfaceId: "writer.delivery", controllerId: "writer", componentKind: "text-editor", textEditor: { buffer: law.initial, selectionJson: JSON.stringify({ start: law.initial.length, end: law.initial.length }) } }, onAction: (action: ActionDescriptor) => onAction(action) } as never));
          for (const key of typedAfterNewOwner) fireEvent.keyDown(view.container.querySelector("textarea")!, { key });
        }
        expect(dispatched).toEqual(law.expected[0]);
        await reactAct(async () => {
          completions.shift()!(law.outcome === "accepted" ? undefined : { kind: "refused", reason: law.outcome });
        });
        await waitFor(() => expect(dispatched).toEqual(law.expected[1]));
        if (completions.length)
          await reactAct(async () => {
            completions.shift()!(undefined);
          });
        await waitFor(() => expect(dispatched).toEqual(law.expected[2]));
        expect(area.getAttribute("aria-readonly")).toBe(String(law.readOnly));
        expect(text).toBe(law.outcome === "accepted" ? law.initial + [...law.typed, ...typedAfterNewOwner].join("") : law.initial);
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });

  for (const law of editorDeliveryFixture.spliceCases)
    it(`delivers splice typing against what the actual React editor shows at delivery: ${law.id}`, async () => {
      const { decodePackValue } = await import("@semio-tech/framework-os");
      let text = law.initial;
      let caret = text.length;
      const session = {
        attachCanvas: vi.fn(async () => {}),
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},

        setText: (value: string) => {
          text = value;
        },
        setSelectionRange: (_anchor: number, next: number) => {
          caret = next;
        },
        synchronizeScene: (pack: Uint8Array) => {
          const scene = decodePackValue(pack) as { buffer?: string };
          if (scene.buffer !== undefined) text = scene.buffer;
        },
        text: () => text,
        caret: () => caret,
        anchor: () => caret,
        setCanvasThemeJson: () => {},
        free: () => {},
        insertText: (value: string) => {
          text = text.slice(0, caret) + value + text.slice(caret);
          caret += value.length;
        },
      };
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session as unknown as flowSessionLoader.EditorWasmSession);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const completions: Array<(outcome: unknown) => void> = [];
      const dispatched: string[] = [];
      const onAction = vi.fn((action: ActionDescriptor) => {
        if (action.action !== "textSplice") return Promise.resolve(undefined);
        const args = action.args as { readonly seq: number; readonly start: number; readonly deleted: string; readonly insert: string; readonly before: string; readonly after: string };
        dispatched.push(`textSplice:${args.seq}:${args.start}:${args.deleted}:${args.insert}:${args.before}:${args.after}`);
        return new Promise((resolve) => completions.push(resolve));
      });
      const node = (buffer: string, applied: number) => ({
        type: "componentScene",
        surfaceId: "writer.splice",
        controllerId: "writer",
        componentKind: "text-editor",
        textEditor: { buffer, selectionJson: JSON.stringify({ start: buffer.length, end: buffer.length, splice: applied }), settingsJson: JSON.stringify({ typing: { mode: "splice" } }) },
      });
      const view = render(createElement(TextEditorHost, { node: node(law.initial, 0), onAction } as never));
      try {
        await waitFor(() => expect(session.attachCanvas).toHaveBeenCalledOnce());
        await reactAct(async () => {
          await Promise.resolve();
        });
        const area = view.container.querySelector("textarea")!;
        for (const key of law.typedFirst) fireEvent.keyDown(area, { key });
        for (const key of law.typedWhileInFlight) fireEvent.keyDown(area, { key });
        await reactAct(async () => {
          view.rerender(createElement(TextEditorHost, { node: node(law.published.buffer, law.published.applied), onAction } as never));
          await Promise.resolve();
        });
        for (let turn = 0; turn < 16 && (completions.length > 0 || dispatched.length < law.expected.length); turn += 1)
          await reactAct(async () => {
            completions.shift()?.({ kind: "applied", inputSeq: 0 });
            await Promise.resolve();
          });
        await waitFor(() => expect(dispatched).toEqual(law.expected));
        expect(text).toBe(law.finalText);
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });

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

  it("multiSpanReplace renames every occurrence and remaps spans", () => {
    const result = multiSpanReplace(
      "MATCH (a:Piece) RETURN a.name",
      [
        { start: 7, end: 8 },
        { start: 23, end: 24 },
      ],
      "piece",
    );
    expect(result.text).toBe("MATCH (piece:Piece) RETURN piece.name");
    expect(result.occurrences).toEqual([
      { start: 7, end: 12 },
      { start: 23, end: 28 },
    ]);
  });

  it("lineRangeAt finds the line containing an offset", () => {
    const text = "MATCH (a)\nWHERE a.x = 1\nRETURN a";
    const range = lineRangeAt(text, 15);
    expect(text.slice(range.start, range.end)).toBe("WHERE a.x = 1");
  });
});
