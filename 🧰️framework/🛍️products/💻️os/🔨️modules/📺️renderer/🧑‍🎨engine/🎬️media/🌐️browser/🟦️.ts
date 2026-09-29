import type { MediaExportHandle, MediaExportStatus } from "@semio-tech/framework-os";
import { parseMediaTransportProps, type MediaTransportProps } from "../../../../🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts";
import { assertMediaExportAuthority, type MediaExportChunk, type MediaTransportPort } from "../🚚️lifecycle/🟦️.ts";
import { SEGMENTED_DOWNLOAD_CONTRACT } from "../../🧱️elements/📤️SegmentedDownload/🟦️.ts";

export const BROWSER_MEDIA_CAPACITY = { slots: 32, descriptorBytes: 65_536, commandBytes: 8192, commandTimeoutMs: 30_000 } as const;
export type MediaRect = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };
export type PresentedMediaSlot = {
  readonly token: string;
  readonly windowId: string;
  readonly nodeId: string;
  readonly nodeKey: string;
  readonly rect: MediaRect;
  readonly clip: MediaRect;
  readonly paintOrder: number;
  readonly occluded: boolean;
  readonly pluginId: string;
  readonly controllerId: string;
  readonly appInstanceId: number;
  readonly parentDocumentId: string;
  readonly props: MediaTransportProps;
};
export type BrowserMediaOperation = { readonly operation: "submit" } | { readonly operation: "poll" | "cancel" | "take"; readonly handle: MediaExportHandle };
export type BrowserMediaCommand = BrowserMediaOperation & { readonly kind: "media-command"; readonly lifecycle: number; readonly requestId: number; readonly slotToken: string };
export type BrowserMediaRelease = { readonly kind: "media-release"; readonly lifecycle: number; readonly slotToken: string };
export type BrowserMediaValue = MediaExportHandle | MediaExportStatus | MediaExportChunk | null;
export type BrowserMediaResult = { readonly kind: "media-result"; readonly lifecycle: number; readonly requestId: number; readonly slotToken: string; readonly result?: BrowserMediaValue; readonly fault?: string };

function exact(value: Record<string, unknown>, keys: readonly string[]): boolean {
  return Object.keys(value).length === keys.length && keys.every((key) => Object.hasOwn(value, key));
}
function rect(value: unknown): value is MediaRect {
  if (!value || typeof value !== "object") return false;
  const record = value as Record<string, unknown>;
  return exact(record, ["x", "y", "width", "height"]) && [record.x, record.y, record.width, record.height].every((number) => typeof number === "number" && Number.isFinite(number)) && (record.width as number) >= 0 && (record.height as number) >= 0;
}

/** 🧬️ Admits bounded canonical descriptors and exact shell/resource ownership together. */
export function parsePresentedMediaSlots(value: unknown): readonly PresentedMediaSlot[] {
  if (!Array.isArray(value) || value.length > BROWSER_MEDIA_CAPACITY.slots || new TextEncoder().encode(JSON.stringify(value)).byteLength > BROWSER_MEDIA_CAPACITY.descriptorBytes) throw new Error("browser-media.capacity");
  const tokens = new Set<string>();
  return value.map((candidate) => {
    if (!candidate || typeof candidate !== "object" || !exact(candidate, ["token", "windowId", "nodeId", "nodeKey", "rect", "clip", "paintOrder", "occluded", "pluginId", "controllerId", "appInstanceId", "parentDocumentId", "props"])) throw new Error("browser-media.descriptor");
    const slot = candidate as PresentedMediaSlot;
    for (const [key, maximum] of [["token", 4096], ["windowId", 512], ["nodeId", 32], ["nodeKey", 512], ["pluginId", 256], ["controllerId", 256], ["parentDocumentId", 512]] as const) if (typeof slot[key] !== "string" || slot[key].length === 0 || [...slot[key]].length > maximum) throw new Error("browser-media.identity");
    if (!Number.isSafeInteger(slot.appInstanceId) || slot.appInstanceId < 0 || slot.appInstanceId > 0xffff_ffff || !Number.isSafeInteger(slot.paintOrder) || slot.paintOrder < 0 || slot.paintOrder >= BROWSER_MEDIA_CAPACITY.slots || typeof slot.occluded !== "boolean" || !rect(slot.rect) || !rect(slot.clip) || (!slot.occluded && (slot.rect.width === 0 || slot.rect.height === 0 || slot.clip.width === 0 || slot.clip.height === 0)) || tokens.has(slot.token)) throw new Error("browser-media.geometry");
    if (!/^(0|[1-9][0-9]{0,19})$/.test(slot.nodeId) || BigInt(slot.nodeId) > 0xffff_ffff_ffff_ffffn) throw new Error("browser-media.node");
    const props = parseMediaTransportProps(slot.props);
    const resource = props.resource;
    if (resource && (resource.controllerId !== slot.controllerId || resource.appInstanceId !== slot.appInstanceId || resource.parentDocumentId !== slot.parentDocumentId)) throw new Error("browser-media.owner");
    tokens.add(slot.token);
    return slot;
  });
}

function ownedHandle(slot: PresentedMediaSlot, handle: MediaExportHandle): void {
  const resource = slot.props.resource;
  if (!resource) throw new Error("browser-media.resource");
  assertMediaExportAuthority(handle, { app_instance_id: slot.appInstanceId, parent_document_id: slot.parentDocumentId, base_revision: BigInt(resource.revision), generation: BigInt(resource.generation) });
}

/** 🔑️ Geometry may move while the accepted resource authority remains unchanged. */
export function mediaSlotAuthorityKey(slot: PresentedMediaSlot): string {
  return JSON.stringify([slot.windowId, slot.nodeId, slot.nodeKey, slot.pluginId, slot.controllerId, slot.appInstanceId, slot.parentDocumentId, slot.props.resource, slot.props.kind, slot.props.mediaType]);
}

type WorkerRun = { readonly slot: PresentedMediaSlot; readonly port: MediaTransportPort; active: boolean; busy: boolean; handle: MediaExportHandle | null; tail: Promise<unknown> };

/** 🔐️ The worker owns accepted descriptors and serial export retirement, independently of page claims. */
export class BrowserMediaRegistry {
  private readonly runs = new Map<string, WorkerRun>();
  private readonly retired = new Set<Promise<void>>();
  private closed = false;
  constructor(private readonly resolvePort: (pluginId: string) => MediaTransportPort | undefined) {}

  accept(value: unknown): readonly PresentedMediaSlot[] {
    if (this.closed) return [];
    const slots = parsePresentedMediaSlots(value);
    const next = new Set(slots.map((slot) => slot.token));
    for (const [token, run] of this.runs) if (!next.has(token) || mediaSlotAuthorityKey(slots.find((slot) => slot.token === token)!) !== mediaSlotAuthorityKey(run.slot)) this.retire(token);
    for (const slot of slots) {
      if (this.runs.has(slot.token) || slot.props.capability.status !== "ready" || !slot.props.resource) continue;
      const port = this.resolvePort(slot.pluginId);
      if (port && this.runs.size + this.retired.size < BROWSER_MEDIA_CAPACITY.slots) this.runs.set(slot.token, { slot, port, active: true, busy: false, handle: null, tail: Promise.resolve() });
    }
    return slots;
  }

  async execute(command: BrowserMediaCommand): Promise<BrowserMediaValue> {
    if (!Number.isSafeInteger(command.requestId) || command.requestId < 1 || typeof command.slotToken !== "string" || command.slotToken.length > 4096 || !["submit", "poll", "cancel", "take"].includes(command.operation)) throw new Error("browser-media.command");
    const fields = command.operation === "submit" ? ["kind", "lifecycle", "requestId", "slotToken", "operation"] : ["kind", "lifecycle", "requestId", "slotToken", "operation", "handle"];
    if (!exact(command as unknown as Record<string, unknown>, fields) || new TextEncoder().encode(JSON.stringify(command, (_, value) => typeof value === "bigint" ? String(value) : value)).byteLength > BROWSER_MEDIA_CAPACITY.commandBytes) throw new Error("browser-media.command-capacity");
    if (command.operation !== "submit") {
      const handle = command.handle;
      if (!handle || !exact(handle as unknown as Record<string, unknown>, ["app_instance_id", "parent_document_id", "operation_id", "base_revision", "generation"]) || typeof handle.parent_document_id !== "string" || [...handle.parent_document_id].length > 512 || ![handle.operation_id, handle.base_revision, handle.generation].every((value) => typeof value === "bigint" && value >= 0n && value <= 0xffff_ffff_ffff_ffffn)) throw new Error("browser-media.handle");
    }
    const run = this.runs.get(command.slotToken);
    if (this.closed || !run?.active) throw new Error("browser-media.retired");
    if (run.busy) throw new Error("browser-media.in-flight");
    if (command.operation !== "submit") {
      if (!run.handle) throw new Error("browser-media.missing-operation");
      ownedHandle(run.slot, command.handle);
      assertMediaExportAuthority(command.handle, run.handle);
    } else if (run.handle) throw new Error("browser-media.duplicate-submit");
    run.busy = true;
    const work = async (): Promise<BrowserMediaValue> => {
      const slot = run.slot;
      if (command.operation === "submit") {
        const handle = await run.port.submitMediaExport(slot.appInstanceId, slot.props.resource!.outputPort, slot.parentDocumentId, BigInt(slot.props.revision));
        ownedHandle(slot, handle);
        run.handle = handle;
        return handle;
      }
      if (command.operation === "cancel") {
        await run.port.cancelMediaExport(slot.appInstanceId, command.handle);
        run.handle = null;
        return null;
      }
      if (command.operation === "poll") {
        const status = await run.port.pollMediaExport(slot.appInstanceId, command.handle);
        assertMediaExportAuthority(status.handle, command.handle);
        if (status.state === "failed" || status.state === "cancelled") run.handle = null;
        return status;
      }
      const chunk = await run.port.takeMediaExportChunk(slot.appInstanceId, command.handle);
      assertMediaExportAuthority(chunk.handle, command.handle);
      if (chunk.data.byteLength > SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes) throw new Error("browser-media.chunk-capacity");
      if (chunk.terminal) run.handle = null;
      return chunk;
    };
    run.tail = work().finally(() => { run.busy = false; });
    try {
      const result = await run.tail as BrowserMediaValue;
      if (!run.active || this.closed) throw new Error("browser-media.retired");
      return result;
    } catch (error) {
      if (run.active) this.retire(command.slotToken);
      throw error;
    }
  }

  release(token: string): void { this.retire(token); }

  private retire(token: string): void {
    const run = this.runs.get(token);
    if (!run) return;
    this.runs.delete(token);
    run.active = false;
    const cleanup = run.tail.catch(() => {}).then(async () => {
      if (!run.handle) return;
      const handle = run.handle;
      let status: MediaExportStatus;
      try {
        status = await run.port.pollMediaExport(run.slot.appInstanceId, handle);
        assertMediaExportAuthority(status.handle, handle);
      } catch {
        await run.port.cancelMediaExport(run.slot.appInstanceId, handle);
        run.handle = null;
        return;
      }
      if (status.state === "running") await run.port.cancelMediaExport(run.slot.appInstanceId, handle);
      else if (status.state === "complete") for (let page = 0; page < SEGMENTED_DOWNLOAD_CONTRACT.maximumOutstandingChunks; page++) {
        const chunk = await run.port.takeMediaExportChunk(run.slot.appInstanceId, handle);
        assertMediaExportAuthority(chunk.handle, handle);
        if (chunk.terminal) break;
        await new Promise<void>((resolve) => setTimeout(resolve, 0));
      }
      run.handle = null;
    }).catch(() => {}).finally(() => { this.retired.delete(cleanup); });
    this.retired.add(cleanup);
  }

  async close(): Promise<void> {
    this.closed = true;
    for (const token of this.runs.keys()) this.retire(token);
    await Promise.all(this.retired);
  }
}
