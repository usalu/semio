import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it, vi, afterEach, beforeEach } from "vitest";
import { JSDOM } from "jsdom";
import Ajv2020 from "ajv/dist/2020";
import { mediaTransportLabels } from "../../../../🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts";
import { collectMediaTransportBytes, type MediaTransportPort } from "../../🎬️media/🚚️lifecycle/🟦️.ts";
import { mediaTransportPort } from "../../🎬️media/📡️channel/🟦️.ts";
import { BrowserMediaRegistry, parsePresentedMediaSlots, type PresentedMediaSlot, type BrowserMediaCommand } from "../../🎬️media/🌐️browser/🟦️.ts";
import { createBrowserMediaOverlay } from "../../🎬️media/🌐️browser/🎛️host/🟦️.ts";
import { BrowserFrameTransport, type BrowserFrameUiMessage, type BrowserFrameWorkerMessage, type BrowserFrameWorkerPort } from "../../🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts";
import { resolveWgpuBootDescriptor } from "../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts";
import { createAccessibilityMirror } from "../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts";
import { AppChannelClient, AppChannelRequestSequence, decodeAppCommand, decodeAppFrame, encodeAppCommand, encodeAppFrame, type AppFrameValue } from "../../../../../🟦️.ts";
import { createTurnOutcomeBroadcast, type TurnOutcome } from "@semio-tech/framework";

const testDirectory = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(resolve(testDirectory, "../../🎬️media/🌐️browser/🧫️fixtures/🔣️.json"), "utf8"));
const identityFixtureDirectory = resolve(testDirectory, "../../../../../🔨️modules/📡️spr/🧵️channel/🧬️fixtures/🪪️document-identity-wire-v20");
const handle = { app_instance_id: 17, parent_document_id: "document-7", operation_id: BigInt(fixture.operation), base_revision: BigInt(fixture.revision), generation: BigInt(fixture.generation) };
function slot(token = "accepted:7"): PresentedMediaSlot {
  return { token, windowId: "media:main", nodeId: "7", nodeKey: "transport", rect: { x: 20, y: 30, width: 300, height: 180 }, clip: { x: 25, y: 35, width: 290, height: 170 }, paintOrder: 2, occluded: false, pluginId: "stdio.mp4", controllerId: "stdio.mp4.editor", appInstanceId: 17, parentDocumentId: "document-7", props: { schemaVersion: 1, kind: "audio", mediaType: "audio/wav", revision: fixture.revision, durationMs: 1000, positionMs: 250, selectionStartMs: 100, selectionEndMs: 900, locale: "en", labels: mediaTransportLabels("en"), resource: { kind: "artifact-media-export", controllerId: "stdio.mp4.editor", appInstanceId: 17, parentDocumentId: "document-7", outputPort: "playback:out", revision: fixture.revision, generation: fixture.generation }, capability: { status: "ready", reason: null }, hostContentHeight: 180 } };
}
function command(operation: BrowserMediaCommand["operation"], token = "accepted:7"): BrowserMediaCommand {
  return { kind: "media-command", lifecycle: 1, requestId: 1, slotToken: token, operation, ...(operation === "submit" ? {} : { handle }) } as BrowserMediaCommand;
}
function port(state: "complete" | "running" = "complete", calls: string[] = []): MediaTransportPort {
  return {
    submitMediaExport: async () => { calls.push("submit"); return handle; },
    pollMediaExport: async () => { calls.push("poll"); return { handle, state, applied_progress: 1n, checkpoint_available: false, detail: "", mime_type: "audio/wav", total_bytes: 4n }; },
    cancelMediaExport: async () => { calls.push("cancel"); },
    takeMediaExportChunk: async () => { calls.push("take"); return { handle, data: Uint8Array.from(fixture.bytes), terminal: true }; },
  };
}
const turn = () => new Promise<void>((resolve) => setTimeout(resolve, 0));
beforeEach(() => {
  const dom = new JSDOM("<body></body>");
  for (const key of ["window", "document", "HTMLElement", "HTMLMediaElement", "HTMLInputElement", "HTMLTextAreaElement", "HTMLButtonElement", "KeyboardEvent", "Event"] as const) vi.stubGlobal(key, dom.window[key]);
});
afterEach(() => { vi.restoreAllMocks(); document.body.replaceChildren(); vi.unstubAllGlobals(); });

describe("accepted browser media authority", () => {
  it("admits the neutral real-app reservation journey with independent schema and production boot decoding", () => {
    const path = resolve(testDirectory, "../../🧫️fixtures/🎬️media-app-acceptance");
    const journey = JSON.parse(readFileSync(resolve(path, "🔣️.json"), "utf8"));
    const validate = new Ajv2020().compile(JSON.parse(readFileSync(resolve(path, "🧬️schema/🔣️.json"), "utf8")));
    expect(validate(journey), JSON.stringify(validate.errors)).toBe(true);
    for (const specimen of journey.cases) {
      const query = new URLSearchParams({ plugin: journey.plugin, app: specimen.app, example: journey.example, role: journey.role, mode: journey.mode });
      const decoded = resolveWgpuBootDescriptor({ search: `?${query}`, defaultVariant: "puzzle" });
      expect(decoded.pluginVariant).toBe(journey.plugin); expect(decoded.appId).toBe(specimen.app);
    }
  });
  it("agrees with independent JSON Schema on the bounded descriptor packet", () => {
    const schema = JSON.parse(readFileSync(resolve(testDirectory, "../../🎯️targets/🧊️wgpu/🎬️media-slots/🧬️contract/🔣️.json"), "utf8"));
    const mediaSchema = JSON.parse(readFileSync(resolve(testDirectory, "../../../../🔌️plugin/🪟️window-kits/🎬️media/🧬️contract/🔣️.json"), "utf8"));
    const validate = new Ajv2020().addSchema(mediaSchema).compile(schema);
    for (const packet of [[slot()], [], [{ ...slot(), occluded: true, clip: { ...slot().clip, width: 0 } }], [{ ...slot(), windowId: "🎬".repeat(512) }], [{ ...slot(), windowId: "🎬".repeat(513) }], Array.from({ length: 33 }, (_, i) => slot(String(i))), [{ ...slot(), extra: true }], [{ ...slot(), rect: { x: 0, y: 0, width: -1, height: 10 } }]]) {
      expect(validate(packet)).toBe((() => { try { parsePresentedMediaSlots(packet); return true; } catch { return false; } })());
    }
    expect(() => parsePresentedMediaSlots([{ ...slot(), parentDocumentId: "foreign" }])).toThrow("browser-media.owner");
    expect(() => parsePresentedMediaSlots([slot(), slot()])).toThrow("browser-media.geometry");
    expect(() => parsePresentedMediaSlots([{ ...slot(), props: { ...slot().props, labels: Object.fromEntries(Object.keys(slot().props.labels).map((key) => [key, "x".repeat(5000)])) } }])).toThrow("browser-media.capacity");
  });
  for (const vector of fixture.cases) it(vector.name, async () => {
    const calls: string[] = [];
    const registry = new BrowserMediaRegistry(() => port(vector.state, calls));
    registry.accept([slot()]);
    const submitted = await registry.execute(command("submit"));
    expect(submitted).toEqual(handle);
    expect(typeof (submitted as typeof handle).generation).toBe("bigint");
    if (vector.retire) registry.accept([]);
    else {
      await registry.execute(command("poll"));
      const chunk = await registry.execute(command("take"));
      expect((chunk as { data: Uint8Array }).data).toEqual(Uint8Array.from(fixture.bytes));
    }
    await registry.close();
    expect(calls).toEqual(vector.expected);
    console.debug(`[DEBUG] browser-media ${vector.name} ${calls.join(" -> ")}`);
  });
  it("refuses candidate, retired, forged owner and foreign operation handles", async () => {
    const calls: string[] = [];
    const registry = new BrowserMediaRegistry(() => port("complete", calls));
    await expect(registry.execute(command("submit"))).rejects.toThrow("retired");
    registry.accept([slot()]);
    await registry.execute(command("submit"));
    for (const field of ["app_instance_id", "parent_document_id", "operation_id", "base_revision", "generation"] as const) {
      const value = field === "parent_document_id" ? "foreign" : field === "app_instance_id" ? 18 : handle[field] + 1n;
      await expect(registry.execute({ ...command("poll"), handle: { ...handle, [field]: value } } as BrowserMediaCommand)).rejects.toThrow("authority-mismatch");
    }
    registry.accept([]);
    await expect(registry.execute(command("poll"))).rejects.toThrow("retired");
    await registry.close();
    expect(calls).toEqual(["submit", "poll", "take"]);
  });
  it("never polls or cancels an export handle whose resource owner was refused", async () => {
    for (const field of ["app_instance_id", "parent_document_id", "base_revision", "generation"] as const) {
      const calls: string[] = [];
      const foreign = { ...handle, [field]: field === "parent_document_id" ? "foreign" : field === "app_instance_id" ? 18 : handle[field] + 1n };
      const source = { ...port("running", calls), submitMediaExport: async () => { calls.push("submit"); return foreign; } };
      const registry = new BrowserMediaRegistry(() => source);
      registry.accept([slot()]); await expect(registry.execute(command("submit"))).rejects.toThrow("authority-mismatch"); await registry.close();
      expect(calls).toEqual(["submit"]); calls.length = 0;
      await expect(collectMediaTransportBytes({ instanceId: 17, parentDocumentId: "document-7", revision: handle.base_revision, generation: handle.generation, mediaType: "audio/wav", port: source, cancelled: () => false, progress: () => {} })).rejects.toThrow("media-export.owner");
      expect(calls).toEqual(["submit"]);
    }
  });
  it("preserves the operation across geometry changes and retires an in-flight submit serially", async () => {
    const calls: string[] = [];
    let release!: (value: typeof handle) => void;
    const source = { ...port("running", calls), submitMediaExport: async () => { calls.push("submit"); return await new Promise<typeof handle>((resolve) => { release = resolve; }); } };
    const registry = new BrowserMediaRegistry(() => source);
    registry.accept([slot()]);
    const pending = registry.execute(command("submit"));
    registry.accept([{ ...slot(), rect: { ...slot().rect, x: 100 } }]);
    await expect(registry.execute(command("submit"))).rejects.toThrow("in-flight");
    registry.accept([]);
    release(handle);
    await expect(pending).rejects.toThrow("retired");
    await registry.close();
    expect(calls).toEqual(["submit", "poll", "cancel"]);
  });
  it("uses the same channel adapter and strict Done reply as React", async () => {
    const frames = { submitMediaExport: async () => [{ MediaExportSubmitted: { handle, in_reply_to: 1n } }], pollMediaExport: async () => [{ MediaExportStatus: { handle, in_reply_to: 1n, state: "complete", applied_progress: 0n, checkpoint_available: false, detail: "", mime_type: "audio/wav", total_bytes: 4n } }], cancelMediaExport: async () => [], takeMediaExportChunk: async () => [{ MediaExportChunk: { handle, in_reply_to: 1n, data: fixture.bytes, terminal: true } }] };
    const adapter = mediaTransportPort(() => frames as never);
    const bytes = await collectMediaTransportBytes({ instanceId: 17, parentDocumentId: "document-7", revision: handle.base_revision, generation: handle.generation, mediaType: "audio/wav", port: adapter, cancelled: () => false, progress: () => {} });
    expect(bytes?.chunks[0]).toEqual(Uint8Array.from(fixture.bytes));
    await expect(adapter.cancelMediaExport(17, handle)).rejects.toThrow("missing-reply");
    frames.cancelMediaExport = async () => [{ Done: { in_reply_to: 1n } }] as never;
    await expect(adapter.cancelMediaExport(17, handle)).resolves.toBeUndefined();
  });
  it("transfers owned chunk storage and retires a faulting running operation", async () => {
    const registry = new BrowserMediaRegistry(() => port());
    registry.accept([slot()]); await registry.execute(command("submit"));
    const chunk = await registry.execute(command("take")) as { handle: typeof handle; data: Uint8Array; terminal: boolean };
    const clone = structuredClone(chunk, { transfer: [chunk.data.buffer as ArrayBuffer] });
    expect(chunk.data.byteLength).toBe(0); expect(clone.data).toEqual(Uint8Array.from(fixture.bytes));
    await registry.close();
    const calls: string[] = [];
    const failed = new BrowserMediaRegistry(() => ({ ...port("running", calls), pollMediaExport: async () => { calls.push("fault"); throw new Error("guest-fault"); } }));
    failed.accept([slot()]); await failed.execute(command("submit"));
    await expect(failed.execute(command("poll"))).rejects.toThrow("guest-fault");
    await failed.close(); expect(calls).toEqual(["submit", "fault", "fault", "cancel"]);
  });
});

class Worker implements BrowserFrameWorkerPort {
  onmessage: BrowserFrameWorkerPort["onmessage"] = null;
  onmessageerror: BrowserFrameWorkerPort["onmessageerror"] = null;
  onerror: BrowserFrameWorkerPort["onerror"] = null;
  messages: BrowserFrameUiMessage[] = [];
  postMessage(message: BrowserFrameUiMessage): void { this.messages.push(structuredClone(message)); }
  terminate(): void {}
  reply(message: BrowserFrameWorkerMessage): void { this.onmessage?.({ data: message } as MessageEvent<BrowserFrameWorkerMessage>); }
}
function transport(worker: Worker): BrowserFrameTransport {
  return new BrowserFrameTransport({ worker, boot: { bindingsModuleUrl: "test.js", bindingsWasmUrl: "test.wasm", canvas: {} as OffscreenCanvas, width: 800, height: 600, dpr: 1, locale: "en", descriptor: resolveWgpuBootDescriptor({ defaultVariant: "s" }), appearance: { preference: "", systemDark: false }, platform: "MacIntel", storage: {} }, setTimer: () => 1, clearTimer: () => {} });
}
function frame(worker: Worker, slots: readonly PresentedMediaSlot[], frameSequence = 1, generation = 0): void {
  worker.reply({ kind: "frame", mediaSlots: slots, lifecycle: 1, frameSequence, generation, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted" });
}

describe("correlated media frame lane", () => {
  it("preserves bigint and refuses stale slots, foreign correlation and parallel commands", async () => {
    const worker = new Worker(); const lane = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 }); frame(worker, [slot()]);
    const media = lane.mediaPort(slot().token);
    const pending = media.submitMediaExport(17, "playback:out", "document-7", handle.base_revision);
    const request = worker.messages.at(-1)! as BrowserMediaCommand;
    await expect(media.submitMediaExport(17, "playback:out", "document-7", handle.base_revision)).rejects.toThrow("in-flight");
    worker.reply({ kind: "media-result", lifecycle: 1, requestId: request.requestId, slotToken: "foreign", result: handle });
    worker.reply({ kind: "media-result", lifecycle: 1, requestId: request.requestId, slotToken: request.slotToken, result: structuredClone(handle) });
    expect(await pending).toEqual(handle);
    const poll = media.pollMediaExport(17, handle);
    expect((worker.messages.at(-1)! as Extract<BrowserMediaCommand, { handle: unknown }>).handle.generation).toBe(handle.generation);
    frame(worker, [], 2);
    await expect(poll).rejects.toThrow("retired");
    await expect(media.pollMediaExport(17, handle)).rejects.toThrow("retired");
    lane.close();
  });
  for (const terminal of ["close", "fault", "quarantine"] as const) it(`rejects pending commands on ${terminal}`, async () => {
    const worker = new Worker(); const lane = transport(worker);
    worker.reply({ kind: "booted", lifecycle: 1 }); frame(worker, [slot()]);
    const pending = lane.mediaPort(slot().token).submitMediaExport(17, "playback:out", "document-7", handle.base_revision);
    if (terminal === "close") lane.close();
    else if (terminal === "fault") worker.reply({ kind: "fault", lifecycle: 1, code: "test", detail: "test" });
    else worker.reply({ kind: "frame", mediaSlots: [], lifecycle: 1, frameSequence: 2, generation: 0, cursor: "default", fullscreen: null, requestFrame: false, nextDeadlineDelayMs: null, progress: 1, workerDurationMs: 1, workerExecutingMs: 1, workerStepVerdict: "admitted", quarantined: true });
    await expect(pending).rejects.toThrow("closed");
    lane.close();
  });
});

describe("imperative media DOM host", () => {
  it("synchronously retires duplicate canvas AX ownership and restores the native fallback", async () => {
    const root = document.createElement("div"); document.body.append(root);
    const overlay = createBrowserMediaOverlay(root, () => port());
    const surfaces = { windows: [{ windowId: "media:main", windowGeneration: 1, nodes: [{ nodeId: 7, key: "transport", role: "status", depth: 0, live: "off", label: "Native fallback" }] }] };
    const mirror = createAccessibilityMirror(root, { introspect: async () => JSON.stringify(surfaces), enqueueLossless: () => true }, "en", undefined, (surface, node) => overlay.owns(surface.windowId, node.nodeId, node.key));
    mirror.refresh(); await new Promise<void>((resolve) => setTimeout(resolve, 450));
    expect(root.querySelector('#semio-wgpu-accessibility [data-node-id="7"]')).not.toBeNull();
    const unsupported = { ...slot(), props: { ...slot().props, resource: null, capability: { status: "unsupported" as const, reason: null } } };
    overlay.accept([unsupported]); mirror.refresh();
    expect(root.querySelector('#semio-wgpu-accessibility [data-node-id="7"]')).toBeNull();
    overlay.accept([]); mirror.refresh(); await new Promise<void>((resolve) => setTimeout(resolve, 450));
    expect(root.querySelector('#semio-wgpu-accessibility [data-node-id="7"]')).not.toBeNull();
    overlay.dispose(); mirror.dispose();
  });
  it("mounts audio controls with accepted bounds, isolated input, stable token and exactly one URL revoke", async () => {
    vi.spyOn(HTMLMediaElement.prototype, "pause").mockImplementation(() => {});
    vi.spyOn(HTMLMediaElement.prototype, "load").mockImplementation(() => {});
    const create = vi.fn(() => "blob:media"); const revoke = vi.fn();
    vi.stubGlobal("URL", class extends URL { static createObjectURL = create; static revokeObjectURL = revoke; });
    const root = document.createElement("div"); document.body.append(root);
    const overlay = createBrowserMediaOverlay(root, () => port());
    overlay.accept([slot()]); await turn();
    const host = root.querySelector<HTMLElement>("[data-media-slot]")!;
    const audio = host.querySelector("audio")!;
    expect(audio.autoplay).toBe(false);
    expect(audio.getAttribute("aria-label")).toBe("Audio");
    expect(host.style.left).toBe("20px"); expect(host.style.top).toBe("30px");
    expect(host.style.clipPath).toBe("inset(5px 5px 5px 5px)");
    expect(host.querySelectorAll('input[type="range"]')).toHaveLength(3);
    const canvasLane = vi.fn(); root.addEventListener("keydown", canvasLane);
    host.dispatchEvent(new KeyboardEvent("keydown", { key: " ", bubbles: true }));
    expect(canvasLane).not.toHaveBeenCalled();
    expect(overlay.owns("media:main", 7, "transport")).toBe(true);
    overlay.accept([{ ...slot(), rect: { ...slot().rect, x: 80 } }]);
    expect(root.querySelector("audio")).toBe(audio); expect(create).toHaveBeenCalledOnce();
    overlay.accept([{ ...slot(), occluded: true, clip: { ...slot().clip, width: 0 } }]);
    expect(host.style.visibility).toBe("hidden"); expect(host.style.pointerEvents).toBe("none"); expect(host.getAttribute("aria-hidden")).toBe("true"); expect(host.inert).toBe(true);
    expect(root.querySelector("audio")).toBe(audio); expect(revoke).not.toHaveBeenCalled();
    overlay.accept([slot()]); expect(host.style.visibility).toBe("visible"); expect(host.hasAttribute("aria-hidden")).toBe(false);
    overlay.accept([]); overlay.dispose();
    expect(revoke).toHaveBeenCalledExactlyOnceWith("blob:media");
    console.debug("[DEBUG] browser-media DOM audio bounds/control/input/URL runtime assertions passed");
  });
  it("preserves a video player after an interrupted play request and releases an actual decode fault", async () => {
    vi.spyOn(HTMLMediaElement.prototype, "pause").mockImplementation(() => {});
    vi.spyOn(HTMLMediaElement.prototype, "load").mockImplementation(() => {});
    vi.spyOn(HTMLMediaElement.prototype, "play").mockRejectedValue(new DOMException("Pause interrupted play", "AbortError"));
    const revoke = vi.fn();
    vi.stubGlobal("URL", class extends URL { static createObjectURL = () => "blob:video"; static revokeObjectURL = revoke; });
    const root = document.createElement("div"); document.body.append(root);
    const overlay = createBrowserMediaOverlay(root, () => port());
    overlay.accept([{ ...slot(), props: { ...slot().props, kind: "video" } }]); await turn();
    const video = root.querySelector("video")!;
    expect(video.autoplay).toBe(false); expect(video.getAttribute("aria-label")).toBe("Video");
    root.querySelector("button")!.click(); await turn();
    expect(root.querySelector("video")).toBe(video); expect(revoke).not.toHaveBeenCalled();
    video.dispatchEvent(new Event("error"));
    expect(root.querySelector('[data-media-state="failed"]')).not.toBeNull(); expect(revoke).toHaveBeenCalledOnce();
    overlay.dispose(); expect(revoke).toHaveBeenCalledOnce();
  });
  it("never publishes a stale completion and cancels or drains replacement before creating another URL", async () => {
    vi.spyOn(HTMLMediaElement.prototype, "pause").mockImplementation(() => {});
    vi.spyOn(HTMLMediaElement.prototype, "load").mockImplementation(() => {});
    const create = vi.fn(() => "blob:media"); const revoke = vi.fn();
    vi.stubGlobal("URL", class extends URL { static createObjectURL = create; static revokeObjectURL = revoke; });
    let release!: () => void; const calls: string[] = [];
    const source = { ...port("complete", calls), pollMediaExport: async () => { calls.push("poll"); await new Promise<void>((resolve) => { release = resolve; }); return { handle, state: "complete" as const, applied_progress: 0n, checkpoint_available: false, detail: "", mime_type: "audio/wav", total_bytes: 4n }; } };
    const root = document.createElement("div"); document.body.append(root);
    const overlay = createBrowserMediaOverlay(root, () => source);
    overlay.accept([slot()]); await turn(); overlay.accept([]); release(); await turn();
    expect(create).not.toHaveBeenCalled(); expect(revoke).not.toHaveBeenCalled();
    expect(calls).toEqual(["submit", "poll", "take"]); expect(root.querySelector("audio")).toBeNull();
    overlay.dispose();
  });
});


describe("trusted app document identity", () => {
  it("matches shared Rust vectors and independent Ajv/LEB128 without lossy u64 conversion", async () => {
    const vectors = JSON.parse(readFileSync(resolve(identityFixtureDirectory, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(resolve(identityFixtureDirectory, "🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv2020().compile(schema);
    const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
    const { default: leb } = await import(oracleModule);
    const u64 = (value: bigint): number[] => { const input = Buffer.alloc(8); input.writeBigUInt64LE(value); return Array.from(leb.encodeUIntBuffer(input)); };
    for (const vector of vectors.cases) {
      expect(validate(vector.identity)).toBe(true);
      const seq = BigInt(vector.seq);
      const command = { ReadDocumentIdentity: { seq } };
      const identity = { app_instance_id: vector.identity.appInstanceId, parent_document_id: vector.identity.parentDocumentId };
      const reply = { DocumentIdentity: { in_reply_to: seq, identity } };
      const document = identity.parent_document_id === null ? null : Buffer.from(identity.parent_document_id, "utf8");
      const independent = [vectors.frameTag, ...u64(seq), ...u64(BigInt(identity.app_instance_id)), ...(document === null ? [0] : [1, ...u64(BigInt(document.length)), ...document])];
      expect([vectors.commandTag, ...u64(seq)]).toEqual(vector.commandBytes);
      expect(independent).toEqual(vector.frameBytes);
      expect(Array.from(encodeAppCommand(command))).toEqual(vector.commandBytes);
      expect(decodeAppCommand(Uint8Array.from(vector.commandBytes))).toEqual(command);
      expect(Array.from(encodeAppFrame(reply))).toEqual(vector.frameBytes);
      expect(decodeAppFrame(Uint8Array.from(vector.frameBytes))).toEqual(reply);
    }
  });
  it("rejects malformed UTF8, owner ranges, empty/oversized documents and trailing identity authority", () => {
    for (const bytes of [[31, 1, 17, 2], [31, 1, 17, 1, 1, 255], [31, 1, 17, 1, 0], [31, 1, 17, 0, 0], [31, 1, 17]]) expect(() => decodeAppFrame(Uint8Array.from(bytes))).toThrow();
    expect(() => decodeAppCommand(Uint8Array.from([41, 1, 0]))).toThrow("trailing");
    for (const identity of [{ app_instance_id: 0x1_0000_0000, parent_document_id: null }, { app_instance_id: 17, parent_document_id: "" }, { app_instance_id: 17, parent_document_id: "x".repeat(513) }]) expect(() => encodeAppFrame({ DocumentIdentity: { in_reply_to: 1n, identity } })).toThrow("invalid owner");
    const boundary = { DocumentIdentity: { in_reply_to: 1n, identity: { app_instance_id: 17, parent_document_id: "🎬".repeat(512) } } };
    expect(decodeAppFrame(encodeAppFrame(boundary))).toEqual(boundary);
  });
  it("queries through the real AppChannelClient and rejects ambiguous/missing/foreign replies", async () => {
    for (const variant of ["valid", "ambiguous", "missing", "foreign"] as const) {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const sent: ReturnType<typeof decodeAppCommand>[] = [];
      const client = new AppChannelClient({ outcomes: broadcast.stream, enqueue: (_instance, packets) => { sent.push(...packets.map(decodeAppCommand)); } }, new AppChannelRequestSequence(), 17, "media-controller");
      try {
        const pending = client.readDocumentIdentity();
        expect(sent).toEqual([{ ReadDocumentIdentity: { seq: 1n } }]);
        const identity = { app_instance_id: variant === "foreign" ? 18 : 17, parent_document_id: "trusted-document" };
        const reply: AppFrameValue = { DocumentIdentity: { in_reply_to: 1n, identity } };
        const frames: AppFrameValue[] = variant === "missing" ? [{ Done: { in_reply_to: 1 } }] : variant === "ambiguous" ? [reply, reply] : [{ DocumentIdentity: { in_reply_to: 0xffff_ffff_ffff_ffffn, identity } }, reply];
        broadcast.push({ instanceId: 17, frames: frames.map(encodeAppFrame) });
        if (variant === "valid") expect(await pending).toEqual(identity);
        else await expect(pending).rejects.toThrow(variant === "foreign" ? "foreign instance" : "missing or ambiguous");
      } finally { client.dispose(); }
    }
    console.debug("[DEBUG] browser-media trusted document identity used exact guest/channel receipts");
  });
});

for (const kind of ["audio", "video"] as const) it(`runs the production browser ${kind} host handlers and Chromium decoder`, async () => {
  const video = kind === "video" ? readFileSync(resolve(testDirectory, "../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🎬️.mp4" )).toString("base64") : null;
  const [{ build }, { chromium }] = await Promise.all([import("esbuild"), import("playwright")]);
  const bundle = await build({ entryPoints: [resolve(testDirectory, "../../🎬️media/🌐️browser/🎛️host/🟦️.ts")], bundle: true, format: "iife", globalName: "MediaHostModule", platform: "browser", target: "chrome120", write: false, logLevel: "silent", define: { "import.meta.vitest": "undefined" } });
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const logs: string[] = [];
  page.on("console", (message) => { if (message.text().startsWith("[DEBUG]")) logs.push(message.text()); });
  try {
    await page.setContent('<div id="root" style="position:relative;width:800px;height:600px"></div>');
    await page.addScriptTag({ content: bundle.outputFiles[0]!.text });
    await page.evaluate(({ accepted, wav, video }) => {
      let wave = new Uint8Array(44 + wav.sampleCount);
      const view = new DataView(wave.buffer);
      const ascii = (offset: number, value: string) => { for (let i = 0; i < value.length; i++) wave[offset + i] = value.charCodeAt(i); };
      ascii(0, "RIFF"); view.setUint32(4, 36 + wav.sampleCount, true); ascii(8, "WAVE"); ascii(12, "fmt "); view.setUint32(16, 16, true); view.setUint16(20, 1, true); view.setUint16(22, 1, true); view.setUint32(24, wav.sampleRate, true); view.setUint32(28, wav.sampleRate, true); view.setUint16(32, 1, true); view.setUint16(34, 8, true); ascii(36, "data"); view.setUint32(40, wav.sampleCount, true); wave.fill(wav.sampleValue, 44);
      if (video) wave = Uint8Array.from(atob(video), (character) => character.charCodeAt(0));
      const owner = { app_instance_id: accepted.appInstanceId, parent_document_id: accepted.parentDocumentId, operation_id: 9n, base_revision: BigInt(accepted.props.revision), generation: BigInt(accepted.props.resource!.generation) };
      const calls: string[] = []; let offset = 0;
      const port = { submitMediaExport: async () => { calls.push("submit"); return owner; }, pollMediaExport: async () => { calls.push("poll"); return { handle: owner, state: "complete", applied_progress: 0n, checkpoint_available: false, mime_type: accepted.props.mediaType, total_bytes: BigInt(wave.length), detail: "" }; }, cancelMediaExport: async () => { calls.push("cancel"); }, takeMediaExportChunk: async () => { calls.push("take"); const data = wave.slice(offset, offset + 4096); offset += data.length; return { handle: owner, data, terminal: offset === wave.length }; } };
      const nativeRevoke = URL.revokeObjectURL.bind(URL); let revoked = 0;
      URL.revokeObjectURL = (url) => { revoked++; nativeRevoke(url); };
      const host = (window as unknown as { MediaHostModule: { createBrowserMediaOverlay: typeof createBrowserMediaOverlay } }).MediaHostModule.createBrowserMediaOverlay(document.getElementById("root")!, () => port as MediaTransportPort);
      host.accept([accepted]);
      Object.assign(window, { mediaOracle: { host, calls, accepted, revoked: () => revoked } });
    }, { accepted: { ...slot(), props: { ...slot().props, kind, mediaType: kind === "video" ? "video/mp4" : "audio/wav" } }, wav: fixture.wav, video });
    await page.waitForSelector('[data-media-state="ready"] audio,[data-media-state="ready"] video', { state: "attached" });
    await page.waitForFunction(() => document.querySelector<HTMLMediaElement>("audio,video")!.readyState >= 1);
    const state = await page.evaluate(() => { const audio = document.querySelector<HTMLMediaElement>("audio,video")!; const host = document.querySelector<HTMLElement>("[data-media-slot]")!; console.debug(`[DEBUG] browser-media Chromium decoded ${audio.duration}s ${host.style.left}/${host.style.top}`); return { paused: audio.paused, autoplay: audio.autoplay, positionMs: Math.round(audio.currentTime * 1000), durationMs: Math.round(audio.duration * 1000), left: host.style.left, top: host.style.top, selectionLabels: Array.from(host.querySelectorAll('input[type="range"]')).map((input) => input.getAttribute("aria-label")) }; });
    expect(state.durationMs).toBeGreaterThan(0);
    if (kind === "video") expect(await page.evaluate(() => { const video = document.querySelector("video")!; return video.videoWidth > 0 && video.videoHeight > 0; })).toBe(true);
    expect(state).toEqual({ paused: true, autoplay: false, positionMs: 250, durationMs: kind === "audio" ? fixture.wav.durationMs : expect.any(Number), left: "20px", top: "30px", selectionLabels: ["Seek", "Selection start", "Selection end"] });
    await page.getByRole("button", { name: "Play", exact: true }).click();
    await page.waitForFunction(() => !document.querySelector<HTMLMediaElement>("audio,video")!.paused);
    const concealed = await page.evaluate(() => {
      const oracle = (window as unknown as { mediaOracle: { host: ReturnType<typeof createBrowserMediaOverlay>; accepted: PresentedMediaSlot; calls: string[]; revoked: () => number } }).mediaOracle;
      const player = document.querySelector<HTMLMediaElement>("audio,video")!; const time = player.currentTime;
      oracle.host.accept([{ ...oracle.accepted, occluded: true, clip: { ...oracle.accepted.clip, width: 0 } }]);
      const hidden = document.querySelector<HTMLElement>("[data-media-slot]")!;
      const concealed = hidden.style.visibility === "hidden" && hidden.inert && hidden.getAttribute("aria-hidden") === "true";
      oracle.host.accept([oracle.accepted]);
      console.debug(`[DEBUG] browser-media Chromium conceal/reveal playing=${!player.paused} same-player=${player === document.querySelector<HTMLMediaElement>("audio,video")} revoke=${oracle.revoked()}`);
      return { concealed, playing: !player.paused, samePlayer: player === document.querySelector<HTMLMediaElement>("audio,video"), sameTime: player.currentTime >= time, revoked: oracle.revoked(), calls: oracle.calls };
    });
    expect(concealed).toEqual({ concealed: true, playing: true, samePlayer: true, sameTime: true, revoked: 0, calls: ["submit", "poll", ...Array.from({ length: Math.ceil((video ? Buffer.from(video, "base64").byteLength : 8044) / 4096) }, () => "take")] });
    await page.getByRole("button", { name: "Pause", exact: true }).click();
    expect(await page.evaluate(() => document.querySelector<HTMLMediaElement>("audio,video")!.paused)).toBe(true);
    await page.getByRole("slider", { name: "Seek", exact: true }).evaluate((input) => { (input as HTMLInputElement).value = "500"; input.dispatchEvent(new Event("input", { bubbles: true })); });
    expect(await page.evaluate(() => document.querySelector<HTMLMediaElement>("audio,video")!.currentTime)).toBeCloseTo(0.5, 2);
    await page.getByRole("slider", { name: "Selection start", exact: true }).evaluate((input) => { (input as HTMLInputElement).value = "200"; input.dispatchEvent(new Event("input", { bubbles: true })); });
    expect(await page.getByRole("slider", { name: "Selection end", exact: true }).getAttribute("min")).toBe("201");
    const cleanup = await page.evaluate(() => { const oracle = (window as unknown as { mediaOracle: { host: ReturnType<typeof createBrowserMediaOverlay>; calls: string[]; revoked: () => number } }).mediaOracle; oracle.host.accept([]); oracle.host.dispose(); console.debug(`[DEBUG] browser-media Chromium retired ${oracle.calls.join(" -> ")} revoke=${oracle.revoked()}`); return { revoked: oracle.revoked(), calls: oracle.calls, audio: document.querySelector<HTMLMediaElement>("audio,video") !== null }; });
    expect(cleanup).toEqual({ revoked: 1, calls: ["submit", "poll", ...Array.from({ length: Math.ceil((video ? Buffer.from(video, "base64").byteLength : 8044) / 4096) }, () => "take")], audio: false });
    expect(logs).toHaveLength(3);
    for (const log of logs) console.debug(log);
  } finally { await browser.close(); }
}, 30_000);
