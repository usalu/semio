/** 📖️ Reads one immutable native snapshot through its exact original media-export owner. */
import type { MediaExportHandle, MediaExportStatus } from "../../../../../../../🔨️modules/🪢️wire-contracts/🟦️.ts";
import policy from "./🔣️.json";

export type SnapshotPackPort = Readonly<{
 submit(port: string, documentId: string, revision: bigint): Promise<MediaExportHandle>;
 poll(handle: MediaExportHandle): Promise<MediaExportStatus>;
 take(handle: MediaExportHandle): Promise<Readonly<{handle: MediaExportHandle; data: Uint8Array; terminal: boolean}>>;
 cancel(handle: MediaExportHandle): Promise<void>;
}>;
export type SnapshotPackControl = Readonly<{
 signal: AbortSignal;
 deadlineMs: number;
 assertCurrent(): void;
 progress(completed: bigint): void;
 yieldTurn(): Promise<void>;
}>;
export type SnapshotPackIdentity = Readonly<{instance: number; documentId: string; revision: bigint}>;
function same(left: MediaExportHandle, right: MediaExportHandle): boolean {
 return left.app_instance_id === right.app_instance_id && left.parent_document_id === right.parent_document_id && left.operation_id === right.operation_id && left.base_revision === right.base_revision && left.generation === right.generation;
}
function current(control: SnapshotPackControl): void {
 if (!Number.isSafeInteger(control.deadlineMs) || control.signal.aborted || Date.now() >= control.deadlineMs) throw Error("snapshot-read.cancelled");
 control.assertCurrent();
}

/** 🌊️ Keeps the native handle through chunk EOF or original cancellation before publishing bytes. */
export async function readSnapshotPack(port: SnapshotPackPort, identity: SnapshotPackIdentity, control: SnapshotPackControl): Promise<Uint8Array> {
 current(control);
 const handle = await port.submit(policy.port, identity.documentId, identity.revision);
 const chunks: Uint8Array[] = [];
 let complete = false;
 let admitted = false;
 let failed = false;
 let failure: unknown;
 try {
  if (handle.app_instance_id !== identity.instance || handle.parent_document_id !== identity.documentId || handle.base_revision !== identity.revision) throw Error("snapshot-read.foreign-handle");
  admitted = true;
  current(control);
  let status: MediaExportStatus | undefined;
  for (let turn = 0; turn < policy.maximumTurns; turn++) {
   current(control);
   status = await port.poll(handle);
   current(control);
   if (!same(status.handle, handle)) throw Error("snapshot-read.foreign-handle");
   control.progress(status.applied_progress);
   if (status.state !== "running") break;
   await control.yieldTurn();
  }
  if (!status || status.state === "running") throw Error("snapshot-read.stalled");
  if (status.state !== "complete") throw Error(status.detail || "snapshot-read.failed");
  if (status.mime_type !== policy.mediaType) throw Error("snapshot-read.media-type");
  if (status.total_bytes < 1n || status.total_bytes > BigInt(policy.maximumBytes)) throw Error("snapshot-read.capacity");
  let length = 0;
  for (let page = 0; page < policy.maximumTurns; page++) {
   current(control);
   const chunk = await port.take(handle);
   chunks.push(chunk.data);
   current(control);
   if (!same(chunk.handle, handle)) throw Error("snapshot-read.foreign-handle");
   length += chunk.data.byteLength;
   if (chunk.data.byteLength > policy.maximumChunkBytes || BigInt(length) > status.total_bytes) throw Error("snapshot-read.capacity");
   if (chunk.terminal) {
    if (BigInt(length) !== status.total_bytes) throw Error("snapshot-read.length");
    const bytes = new Uint8Array(length);
    let offset = 0;
    for (const original of chunks) { bytes.set(original, offset); offset += original.byteLength; original.fill(0); }
    chunks.length = 0;
    complete = true;
    return bytes;
   }
   if (chunk.data.byteLength === 0) throw Error("snapshot-read.no-progress");
   await control.yieldTurn();
  }
  throw Error("snapshot-read.stalled");
 } catch(error) {
  failed = true;
  failure = error;
  throw error;
 } finally {
  for (const original of chunks) original.fill(0);
  if (admitted && !complete) {
   try {await port.cancel(handle);}
   catch(error) {throw failed ? new AggregateError([failure,error],"snapshot-read.retirement-failed") : error;}
  }
 }
}
