import Ajv from "ajv";
import { createElement } from "react";
import { cleanup, render, waitFor } from "@semio-tech/ui-react/test";
import * as sessionLoader from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { afterEach, expect, it, vi } from "vitest";
import { uiI18n } from "@semio-tech/ui-react";
import { continuousPressIdentity } from "@semio-tech/framework";
import { uiSpacingLen } from "@semio-tech/ui-styling";
import { TextEditorHost } from "../../🧱️elements/✏️TextEditor/🟦️.tsx";
import fixture from "./🧫️fixtures/🔣️.json";

afterEach(() => { cleanup(); vi.restoreAllMocks(); vi.unstubAllGlobals(); });



for (const row of fixture.spacing) it(row.id + " keeps spacing responsive to the owned theme variable", () => {
  const own = uiSpacingLen(row.multiplier);
  const oracle = document.createElement("div");
  oracle.setAttribute("style", "gap:" + row.expectedCss);
  expect(own).toBe(row.expectedCss);
  expect(own).toBe(oracle.style.gap);
});

for (const row of fixture.presses) it(row.id + " keeps distinct page-wide press ownership at one clock instant", () => {
  vi.spyOn(Date, "now").mockReturnValue(row.clockMs);
  const ids = row.keys.map(continuousPressIdentity);
  const serials = ids.map((id, index) => {
    const parts = id.split(":");
    const serial = Number(parts.pop());
    const clock = Number(parts.pop());
    expect(parts.join(":")).toBe(row.keys[index]);
    expect(clock).toBe(row.clockMs);
    expect(Number.isSafeInteger(serial) && serial > 0).toBe(true);
    return serial;
  });
  expect(serials.slice(1).map((serial, index) => serial - serials[index]!)).toEqual(row.expectedSerialSteps);
  const oracle = new Ajv({ strict: true }).compile({ type: "array", uniqueItems: true, items: { type: "string" } });
  expect(new Set(ids).size === ids.length).toBe(row.expectedUnique);
  expect(oracle(ids)).toBe(row.expectedUnique);
  expect(oracle([ids[0], ids[0]])).toBe(false);
});

for (const row of fixture.labels) it(row.id + " names the actual text editor from explicit owned locale", async () => {
  await uiI18n.changeLanguage(row.locale as "en" | "de");
  vi.stubGlobal("ResizeObserver", class { observe() {} unobserve() {} disconnect() {} });
  const session = { attachCanvas: vi.fn(async () => {}), setCaretVisible: vi.fn(), setSize: vi.fn(), renderFrame: vi.fn(), synchronizeScene: vi.fn(), setText: vi.fn(), text: () => "", caret: () => 0, anchor: () => 0, setCanvasThemeJson: vi.fn(), free: vi.fn() };
  const factory = vi.spyOn(sessionLoader, "createEditorSession").mockResolvedValue(session as unknown as sessionLoader.EditorWasmSession);
  const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
  const view = render(createElement(TextEditorHost, {
    node: { type: "componentScene", controllerId: "fixture", surfaceId: "fixture.editor", componentKind: "text-editor", textEditor: { buffer: "", ...(row.language === null ? {} : { language: row.language }) } },
    onAction: async () => undefined,
  }));
  try {
    await waitFor(() => expect(view.container.querySelector("textarea")).not.toBeNull());
    const area = view.container.querySelector("textarea")!;
    expect(area.getAttribute("aria-label")).toBe(row.expected);
    expect(view.getByRole("textbox", { name: row.expected })).toBe(area);
  } finally {
    view.unmount();
    factory.mockRestore();
    bounds.mockRestore();
  }
});
