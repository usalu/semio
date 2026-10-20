/** 🔸️ The framework renderer hosts tests of the framework renderer hosts suite. */
import editorDeliveryFixture from "../../../🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📮️delivery/🔣️.json";
import { act as reactAct, createElement } from "react";
import * as flowSessionLoader from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { expect, it, vi } from "vitest";
import { type ActionDescriptor } from "@semio-tech/framework";
import { TextEditorHost } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔹 Registers the framework renderer hosts tests of the framework renderer hosts suite. */
export function registerFrameworkRendererHostsTests(): void {
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
}
