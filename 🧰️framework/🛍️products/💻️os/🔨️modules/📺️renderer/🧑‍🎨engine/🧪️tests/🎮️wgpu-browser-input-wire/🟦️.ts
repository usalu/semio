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
  type BrowserFrameDomEvent,
  type BrowserFrameUiMessage,
  type BrowserFrameWorkerMessage,
  type BrowserFrameWorkerPort,
} from "../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";

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
    boot: { bindingsModuleUrl: "renderer.js", bindingsWasmUrl: "renderer_bg.wasm", canvas: {} as OffscreenCanvas, width: 1434, height: 836, dpr: 1, pluginVariant: "generation3d", locale: "en", appRole: "editor", appMode: "generate", appExample: "" },
    setTimer: () => 1,
    clearTimer: () => {},
    now: () => 0,
  });
  worker.reply({ kind: "booted", lifecycle: 1 } as BrowserFrameWorkerMessage);
  return transport;
}

describe("wgpu browser input wire", () => {
  it("covers every input the shell admits", () => {
    expect(fixture.rows.length).toBeGreaterThanOrEqual(12);
    const kinds = new Set(fixture.rows.map((row) => String(row.wire.kind)));
    expect([...kinds].sort()).toEqual(["key-down", "key-up", "pointer-down", "pointer-move", "pointer-up", "resize", "wheel"]);
    for (const row of fixture.rows) expect(row.why, `${row.id} must say why it exists`).toBeTruthy();
  });

  for (const row of fixture.rows) {
    it(`projects ${row.id} onto its wire event`, () => {
      const { event, dpr } = domEvent(row);
      expect(browserFrameEventFromDom(event, dpr)).toEqual(row.wire);
    });
  }

  it("scales coordinates by the device pixel ratio exactly once", () => {
    const retina = fixture.rows.find((row) => row.id === "pointer-move-on-a-retina-surface")!;
    const { event } = domEvent(retina);
    expect(browserFrameEventFromDom(event, 1)).toMatchObject({ x: 160.5, y: 69 });
    expect(browserFrameEventFromDom(event, 2)).toMatchObject({ x: 321, y: 138 });
    expect(browserFrameEventFromDom(event, 3)).toMatchObject({ x: 481.5, y: 207 });
  });

  it("never scales a scroll delta", () => {
    const wheel = fixture.rows.find((row) => row.id === "wheel-over-the-preview")!;
    const { event } = domEvent(wheel);
    for (const dpr of [1, 2, 3]) expect(browserFrameEventFromDom(event, dpr)).toMatchObject({ deltaX: 0, deltaY: 120 });
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
    expect(browserFrameEventFromDom({ type: "pointerdown", pointerId: 1, pointerType: "mouse", offsetX: 1, offsetY: 2, button: 4 }, 1)).toMatchObject({ button: "primary" });
    expect(browserFrameEventFromDom({ type: "pointerdown", pointerId: 1, pointerType: "mouse", offsetX: 1, offsetY: 2 }, 1)).toMatchObject({ button: "primary" });
  });

  it("treats an unknown pointer type as a mouse", () => {
    expect(browserFrameEventFromDom({ type: "pointermove", pointerId: 1, pointerType: "gamepad", offsetX: 1, offsetY: 2 }, 1)).toMatchObject({ pointerKind: "mouse" });
  });
});
