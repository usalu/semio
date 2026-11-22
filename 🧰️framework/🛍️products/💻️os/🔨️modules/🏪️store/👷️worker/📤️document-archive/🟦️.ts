import { encodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { DOCUMENT_ARCHIVE_WORKER_PAGE_BYTES_V1 } from "../../../../../../🔨️modules/🛰️backbone/🧱️worker-wire/🟦️.ts";
import type { BackboneWorkerRequest } from "../../../../🟦️";
import { artifactState, post } from "../🧾️document-state/🟦️.ts";

const archiveReaders = new Map<string, Readonly<{ request: Extract<BackboneWorkerRequest, { readonly kind: "document-archive-read" }>; abort: AbortController }>>();
const ARCHIVE_READER_CAPACITY = 4;

/** 📤️ Streams the actor's existing schema-owned archive to its captured renderer client. */
export async function readDocumentArchiveForRendererV1(request: Extract<BackboneWorkerRequest, { readonly kind: "document-archive-read" }>): Promise<void> {
  const { scope, clientInstanceId, requestId } = request;
  const state = artifactState(scope.documentId, scope.spaceId);
  const actor = state?.browserActorReservation;
  const abort = new AbortController();
  const current = (): boolean => state !== undefined && actor != null && !state.closed
    && artifactState(scope.documentId, scope.spaceId) === state && state.openClientInstanceId === clientInstanceId
    && state.browserActorReservation === actor && !actor.suspended && !abort.signal.aborted;
  const page = (offset: number, total: number, bytes: Uint8Array): void => post({ kind: "document-archive-page", requestId, scope, clientInstanceId, offset, total, bytes });
  if (!current() || !actor || archiveReaders.has(clientInstanceId) || archiveReaders.size >= ARCHIVE_READER_CAPACITY) return page(0, 0, new Uint8Array());
  archiveReaders.set(clientInstanceId, { request, abort });
  try {
    const archive = encodeDocumentArchiveBytes(await actor.readDocumentArchive(current));
    for (let offset = 0; offset < archive.byteLength && current(); offset += DOCUMENT_ARCHIVE_WORKER_PAGE_BYTES_V1) {
      page(offset, archive.byteLength, archive.subarray(offset, offset + DOCUMENT_ARCHIVE_WORKER_PAGE_BYTES_V1));
      await new Promise<void>(resolve => setTimeout(resolve, 0));
    }
  } catch {
    if (current()) page(0, 0, new Uint8Array());
  } finally {
    archiveReaders.delete(clientInstanceId);
  }
}


/** ⛔️ Stops publishing archive pages once the exact captured renderer cancels its transfer. */
export function cancelDocumentArchiveReadV1(request: Extract<BackboneWorkerRequest, { readonly kind: "document-archive-read-cancel" }>): void {
  const running = archiveReaders.get(request.clientInstanceId);
  if (running?.request.requestId === request.requestId && running.request.scope.spaceId === request.scope.spaceId
    && running.request.scope.documentId === request.scope.documentId) running.abort.abort();
}
