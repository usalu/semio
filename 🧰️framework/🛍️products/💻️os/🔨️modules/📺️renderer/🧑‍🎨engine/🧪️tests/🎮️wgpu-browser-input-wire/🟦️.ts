// @vitest-environment jsdom
/**
 * 🎮️ TypeScript twin over `🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json`.
 *
 * Answers the UI-isolate half of the same oracle the Rust law
 * (`🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs`) answers on the Worker side: a DOM input observed on
 * `#semio-wgpu-canvas` becomes exactly the wire event the fixture names, it travels on the lane the
 * fixture implies, and a real `BrowserFrameTransport` actually transfers those bytes in one batch.
 *
 * The third-party twin is `playwright`'s own `page.mouse`/`page.keyboard`, which is what
 * `<ticket>/🐍️wgpu-wire-probe.mjs` drives against the live 6118 surface — the rows here are the same
 * points and keys that probe uses, so a green suite and a silent browser cannot both be true.
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  BrowserFrameTransport,
  browserFrameEventFromDom,
  browserFrameEventIsReplaceable,
  browserFramePointerDomEvent,
  browserFrameWheelDomEvent,
  type BrowserFrameDomEvent,
  type BrowserFrameUiMessage,
  type BrowserFrameWorkerMessage,
  type BrowserFrameWorkerPort,
} from "../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import { resolveWgpuBootDescriptor, type WgpuBootDescriptor, type WgpuHostAppearance, type WgpuHostStorageSnapshot } from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";
import { browserClipboardPasteCandidate } from "../../🎯️targets/🧊️wgpu/🎮️input-wire/🟦️.ts";

/** @emoji 🧭️ One resolved boot descriptor for a fixture transport — the shared resolver, never a hand
 * rolled literal, so these fixtures cannot drift from the shape the three real doors produce
 * (`🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts`). */
function testBootDescriptor(variant: string): WgpuBootDescriptor {
  return resolveWgpuBootDescriptor({ defaultVariant: variant });
}

/** @emoji 🌓️ The appearance a realm that read nothing publishes — React's own no-window default. */
const TEST_HOST_APPEARANCE: WgpuHostAppearance = { preference: "", systemDark: false };
const TEST_HOST_PLATFORM = "MacIntel";
const TEST_HOST_STORAGE: WgpuHostStorageSnapshot = {};

type FixtureRow = { readonly id: string; readonly why: string; readonly dom: Record<string, unknown>; readonly wire: Record<string, unknown>; readonly dispatch: Record<string, unknown> | null };

const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json"), "utf8")) as { readonly rows: readonly FixtureRow[] };

/** 🖱️ The fixture's `dom` column minus the `devicePixelRatio` the surface, not the event, owns. */
function domEvent(row: FixtureRow): { event: BrowserFrameDomEvent; dpr: number } {
  const { devicePixelRatio, ...rest } = row.dom as { devicePixelRatio?: number } & Record<string, unknown>;
  return { event: rest as unknown as BrowserFrameDomEvent, dpr: devicePixelRatio ?? 1 };
}

class FakeWorker implements BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  readonly messages: BrowserFrameUiMessage[] = [];
  terminated = false;
  postMessage(message: BrowserFrameUiMessage): void {
    this.messages.push(message);
  }
  terminate(): void {
    this.terminated = true;
  }
  reply(message: BrowserFrameWorkerMessage): void {
    this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>);
  }
}

function readyTransport(worker: FakeWorker): BrowserFrameTransport {
  const transport = new BrowserFrameTransport({
    worker,
    boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer_bg.wasm", canvas: {} as OffscreenCanvas, width: 1434, height: 836, dpr: 1, locale: "en", descriptor: testBootDescriptor("generation3d"), appearance: TEST_HOST_APPEARANCE, platform: TEST_HOST_PLATFORM, storage: TEST_HOST_STORAGE },
    setTimer: () => 1,
    clearTimer: () => {},
    now: () => 0,
  });
  worker.reply({ kind: "booted", lifecycle: 1 } as BrowserFrameWorkerMessage);
  return transport;
}

describe("wgpu browser input wire", () => {
  it("carries an addressed accessibility blur on the bounded lossless lane", () => {
    const worker = new FakeWorker();
    const transport = readyTransport(worker);
    expect(transport.enqueueLossless({ kind: "accessibility-blur", windowId: "framework.settings.general", windowGeneration: 7, nodeId: 21, nodeKey: "framework.settings.driver.saveLabel" })).toBe(true);
    transport.flush();
    const batch = worker.messages.findLast((message): message is Extract<BrowserFrameUiMessage, { kind: "batch" }> => message.kind === "batch");
    expect(batch?.lossless).toContainEqual(expect.objectContaining({ kind: "accessibility-blur", windowId: "framework.settings.general", windowGeneration: 7, nodeId: 21, nodeKey: "framework.settings.driver.saveLabel" }));
  });

  it("covers every input the shell admits", () => {
    expect(fixture.rows.length).toBeGreaterThanOrEqual(12);
    const kinds = new Set(fixture.rows.map((row) => String(row.wire.kind)));
    expect([...kinds].sort()).toEqual(["key-down", "key-up", "pointer-cancel", "pointer-down", "pointer-move", "pointer-up", "resize", "wheel"]);
    for (const row of fixture.rows) expect(row.why, `${row.id} must say why it exists`).toBeTruthy();
  });

  for (const row of fixture.rows) {
    it(`projects ${row.id} onto its wire event`, () => {
      const { event, dpr } = domEvent(row);
      expect(browserFrameEventFromDom(event, dpr)).toEqual(row.wire);
    });
  }

  /** 📐️ Pointer coordinates are CSS pixels on BOTH sides of this wire: the renderer lays out,
   * hit-tests and paints in logical pixels exactly like the React host's DOM, so a density change
   * must not move a single coordinate. Scaling here was the HiDPI placement bug
   * (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g). */
  it("never scales a pointer coordinate by the device pixel ratio", () => {
    const retina = fixture.rows.find((row) => row.id === "pointer-move-on-a-retina-surface")!;
    const { event } = domEvent(retina);
    for (const dpr of [1, 2, 3]) expect(browserFrameEventFromDom(event, dpr), `dpr ${dpr}`).toMatchObject({ x: 160.5, y: 69 });
  });

  /** 📐️ The ONE event that carries physical pixels: a resize sizes the GPU surface, and ships the
   * dpr the renderer divides by to recover the logical extent layout consumes. */
  it("scales only the resize extent by the device pixel ratio", () => {
    const dom: BrowserFrameDomEvent = { type: "resize", clientWidth: 717, clientHeight: 418 };
    expect(browserFrameEventFromDom(dom, 1)).toEqual({ kind: "resize", width: 717, height: 418, dpr: 1 });
    expect(browserFrameEventFromDom(dom, 2)).toEqual({ kind: "resize", width: 1434, height: 836, dpr: 2 });
    expect(browserFrameEventFromDom(dom, 3)).toEqual({ kind: "resize", width: 2151, height: 1254, dpr: 3 });
  });

  it("never scales a scroll delta or its position", () => {
    const wheel = fixture.rows.find((row) => row.id === "wheel-over-the-preview")!;
    const { event } = domEvent(wheel);
    for (const dpr of [1, 2, 3]) expect(browserFrameEventFromDom(event, dpr)).toMatchObject({ x: 250.88, y: 406.8, deltaX: 0, deltaY: 120 });
  });

  it("reads modifier snapshots from actual DOM pointer and wheel events", () => {
    const pointer = new MouseEvent("pointerdown", { shiftKey: true, ctrlKey: true, altKey: false, metaKey: true }) as PointerEvent;
    Object.defineProperties(pointer, {
      pointerId: { value: 41 },
      pointerType: { value: "mouse" },
      offsetX: { value: 12 },
      offsetY: { value: 34 },
      pressure: { value: 0 },
      tiltX: { value: 0 },
      tiltY: { value: 0 },
      button: { value: 0 },
    });
    expect(browserFrameEventFromDom(browserFramePointerDomEvent(pointer, "pointerdown"), 1)).toEqual({
      kind: "pointer-down", pointerId: 41, pointerKind: "mouse", x: 12, y: 34, button: "primary", shift: true, ctrl: true, alt: false, meta: true,
    });

    const wheel = new WheelEvent("wheel", { deltaX: 2, deltaY: 7, altKey: true, metaKey: true });
    Object.defineProperties(wheel, { offsetX: { value: 55 }, offsetY: { value: 66 } });
    expect(browserFrameEventFromDom(browserFrameWheelDomEvent(wheel), 1)).toEqual({
      kind: "wheel", x: 55, y: 66, deltaX: 2, deltaY: 7, shift: false, ctrl: false, alt: true, meta: true,
    });
  });

  it("keeps the newest pointer modifier snapshot when moves coalesce", () => {
    const worker = new FakeWorker();
    const transport = readyTransport(worker);
    transport.enqueueReplaceable({ kind: "pointer-move", pointerId: 1, pointerKind: "mouse", x: 1, y: 1, shift: true, ctrl: false, alt: false, meta: false });
    transport.enqueueReplaceable({ kind: "pointer-move", pointerId: 1, pointerKind: "mouse", x: 2, y: 2, shift: false, ctrl: false, alt: false, meta: false });
    transport.flush();
    const batch = worker.messages.findLast((message): message is Extract<BrowserFrameUiMessage, { kind: "batch" }> => message.kind === "batch");
    const move = batch?.replaceable.find((event) => event.kind === "pointer-move");
    expect(move).toMatchObject({ x: 2, y: 2, shift: false });
  });

  it("puts pointer moves, wheels and resizes on the coalescing lane and transitions on the lossless one", () => {
    for (const row of fixture.rows) {
      const { event, dpr } = domEvent(row);
      const projected = browserFrameEventFromDom(event, dpr);
      expect(browserFrameEventIsReplaceable(projected), row.id).toBe(["pointer-move", "wheel", "resize"].includes(String(row.wire.kind)));
    }
  });

  it("transfers the fixture's own wire bytes through a live transport", () => {
    const worker = new FakeWorker();
    const transport = readyTransport(worker);
    const admitted: Record<string, unknown>[] = [];
    for (const row of fixture.rows) {
      const { event, dpr } = domEvent(row);
      const projected = browserFrameEventFromDom(event, dpr);
      admitted.push(projected as unknown as Record<string, unknown>);
      if (browserFrameEventIsReplaceable(projected)) transport.enqueueReplaceable(projected);
      else transport.enqueueLossless(projected);
    }
    transport.flush();
    const batch = worker.messages.filter((message): message is Extract<BrowserFrameUiMessage, { kind: "batch" }> => message.kind === "batch").at(-1);
    expect(batch, "one batch must carry the admitted inputs").toBeTruthy();
    const transferred = [...batch!.replaceable, ...batch!.lossless] as unknown as Record<string, unknown>[];
    /** ⏱️ The transport stamps `timestampMs` on what it carries; the fixture pins the meaning, not the clock. */
    const carries = (wire: Record<string, unknown>) => transferred.some((event) => Object.entries(wire).every(([field, value]) => JSON.stringify(event[field]) === JSON.stringify(value)));
    for (const row of fixture.rows) {
      if (row.wire.kind === "pointer-move") continue;
      expect(carries(row.wire), `${row.id} must cross the wire`).toBe(true);
    }
    expect(admitted.filter((event) => !["pointer-move", "resize"].includes(String(event.kind))).length, "every non-coalescing input is carried").toBe(transferred.filter((event) => !["pointer-move", "resize"].includes(String(event.kind))).length);
    const moves = batch!.replaceable.filter((event) => event.kind === "pointer-move");
    expect(moves.map((event) => (event as { pointerId: number }).pointerId).sort(), "one coalesced move per pointer identity").toEqual([1, 3, 7]);
  });

  it("names an unknown DOM button the primary one rather than dropping the press", () => {
    expect(browserFrameEventFromDom({ type: "pointerdown", pointerId: 1, pointerType: "mouse", offsetX: 1, offsetY: 2, button: 4, shift: false, ctrl: false, alt: false, meta: false }, 1)).toMatchObject({ button: "primary" });
    expect(browserFrameEventFromDom({ type: "pointerdown", pointerId: 1, pointerType: "mouse", offsetX: 1, offsetY: 2, shift: false, ctrl: false, alt: false, meta: false }, 1)).toMatchObject({ button: "primary" });
  });

  it("treats an unknown pointer type as a mouse", () => {
    expect(browserFrameEventFromDom({ type: "pointermove", pointerId: 1, pointerType: "gamepad", offsetX: 1, offsetY: 2, shift: false, ctrl: false, alt: false, meta: false }, 1)).toMatchObject({ pointerKind: "mouse" });
  });

  it("selects the first valid image before text and refuses items beyond the bounded scan", () => {
    const text = { kind: "string", type: "text/plain", getAsString: () => {}, getAsFile: () => null } as unknown as DataTransferItem;
    const png = new File([new Uint8Array([1, 2, 3, 4])], "pixel.png", { type: "image/png" });
    const image = { kind: "file", type: "image/png", getAsFile: () => png } as unknown as DataTransferItem;
    expect(browserClipboardPasteCandidate([text, image])).toEqual({ kind: "image", file: png });
    expect(browserClipboardPasteCandidate([text])).toEqual({ kind: "text", item: text });
    expect(browserClipboardPasteCandidate([...Array.from({ length: 16 }, () => text), image])).toEqual({ kind: "text", item: text });
    expect(browserClipboardPasteCandidate([])).toBeUndefined();
  });
});
