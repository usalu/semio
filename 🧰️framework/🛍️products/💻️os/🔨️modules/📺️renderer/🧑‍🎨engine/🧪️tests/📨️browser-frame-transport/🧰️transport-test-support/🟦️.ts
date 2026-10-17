/** 🧰️ Browser frame transport test support: the resolved boot descriptor, the host appearance, platform and storage defaults, the fake Worker port and the transport factory the transport test modules share. */
import { fixtureOriginalWorkStock } from "../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts";
import { BrowserFrameTransport, type BrowserFrameUiMessage, type BrowserFrameWorkerMessage, type BrowserFrameWorkerPort } from "../../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import { resolveWgpuBootDescriptor, type WgpuBootDescriptor, type WgpuHostAppearance, type WgpuHostStorageSnapshot } from "../../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";


/** 🧭️ One resolved boot descriptor for a fixture transport — the shared resolver, never a hand
 * rolled literal, so these fixtures cannot drift from the shape the three real doors produce
 * (`🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts`). */
export function testBootDescriptor(variant: string): WgpuBootDescriptor {
  return resolveWgpuBootDescriptor({ defaultVariant: variant });
}

/** 🌓️ The appearance a realm that read nothing publishes — React's own no-window default. */
export const TEST_HOST_APPEARANCE: WgpuHostAppearance = { preference: "", systemDark: false };
export const TEST_HOST_PLATFORM = "MacIntel";
/** 🗄️ The storage snapshot a realm that persisted nothing publishes. */
export const TEST_HOST_STORAGE: WgpuHostStorageSnapshot = {};

export class FakeWorker implements BrowserFrameWorkerPort {
  onmessage: ((event: MessageEvent<BrowserFrameWorkerMessage>) => void) | null = null;
  onmessageerror: ((event: MessageEvent) => void) | null = null;
  onerror: ((event: ErrorEvent) => void) | null = null;
  readonly messages: BrowserFrameUiMessage[] = [];
  readonly transfers: Transferable[][] = [];
  terminated = false;

  postMessage(message: BrowserFrameUiMessage, transfer: Transferable[] = []): void {
    this.messages.push(message);
    this.transfers.push(transfer);
  }

  terminate(): void {
    this.terminated = true;
  }

  reply(message: BrowserFrameWorkerMessage): void {
    this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>);
  }
}

export function transport(worker: FakeWorker, hooks: { diagnostics?: string[]; directives?: number[]; faults?: string[]; turns?: string[]; now?: () => number } = {}): BrowserFrameTransport {
  return new BrowserFrameTransport({ originalWorkStock: fixtureOriginalWorkStock(),
    worker,
    boot: {
      bindingsModuleUrl: "renderer.js",
      bindingsWasmUrl: "renderer_bg.wasm",
      canvas: {} as OffscreenCanvas,
      width: 800,
      height: 600,
      dpr: 2,
      locale: "en",
      descriptor: testBootDescriptor("s"),
      appearance: TEST_HOST_APPEARANCE,
      platform: TEST_HOST_PLATFORM,
      storage: TEST_HOST_STORAGE,
      catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] },
    },
    setTimer: () => 1,
    clearTimer: () => {},
    now: hooks.now,
    onDirectives: (value) => hooks.directives?.push(value.generation),
    onDiagnostic: (value) => hooks.diagnostics?.push(`${value.channel}:${value.generation}:${value.frameSequence}:${value.json}`),
    onUiTurn: (outcome) => hooks.turns?.push(`${outcome.verdict}:${outcome.site}`),
    onFault: (code) => hooks.faults?.push(code),
  });
}
