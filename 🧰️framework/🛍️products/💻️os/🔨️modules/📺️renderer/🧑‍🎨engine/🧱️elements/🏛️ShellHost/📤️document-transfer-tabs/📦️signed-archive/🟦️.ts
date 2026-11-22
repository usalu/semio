import { decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import type { DocumentScope } from "@semio-tech/framework/document/scope";

/** 📦️ Collects ordered archive pages returned by one exact mounted browser actor. */
export async function readSignedDocumentArchiveV1(worker: Worker, scope: DocumentScope, clientInstanceId: string, current: () => boolean,
  signal: AbortSignal, progress: (completed: number, total: number | null) => void): Promise<Uint8Array> {
  signal.throwIfAborted();
  if (!current()) throw new Error("document-archive.owner-retired");
  const owner = { scope, clientInstanceId, requestId: crypto.randomUUID().replaceAll("-", "") };
  return await new Promise<Uint8Array>((resolve, reject) => {
    let archive: Uint8Array | undefined;
    let received = 0;
    const dispose = (): void => {
      clearTimeout(deadline);
      clearInterval(guard);
      signal.removeEventListener("abort", cancel);
      worker.removeEventListener("message", receive);
    };
    const fail = (error: unknown): void => {
      dispose();
      try {
        worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "document-archive-read-cancel", ...owner }) });
      } finally {
        reject(error);
      }
    };
    const cancel = (): void => fail(signal.reason);
    const receive = (event: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>): void => {
      const message = "wire" in event.data ? decodeBackboneWorkerResponse(event.data.wire) : event.data;
      if (message.kind !== "document-archive-page" || message.requestId !== owner.requestId || message.clientInstanceId !== clientInstanceId
        || message.scope.spaceId !== scope.spaceId || message.scope.documentId !== scope.documentId) return;
      if (!current() || message.total === 0 || message.offset !== received || (archive !== undefined && message.total !== archive.byteLength)) {
        fail(new Error("document-archive.invalid-page"));
        return;
      }
      archive ??= new Uint8Array(message.total);
      archive.set(message.bytes, received);
      received += message.bytes.length;
      progress(received, archive.byteLength);
      if (received === archive.byteLength) {
        dispose();
        resolve(archive);
      }
    };
    const guard = setInterval(() => { if (!current()) fail(new Error("document-archive.owner-retired")); }, 100);
    const deadline = setTimeout(() => fail(new Error("document-archive.timeout")), 120_000);
    signal.addEventListener("abort", cancel, { once: true });
    worker.addEventListener("message", receive);
    try {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "document-archive-read", ...owner }) });
    } catch (error) {
      dispose();
      reject(error);
    }
  });
}
