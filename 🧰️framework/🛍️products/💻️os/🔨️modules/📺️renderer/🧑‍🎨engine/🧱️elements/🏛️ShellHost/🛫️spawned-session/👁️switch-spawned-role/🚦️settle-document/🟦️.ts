import { decodeBackboneWorkerResponse, encodeBackboneWorkerRequest, type BackboneWorkerResponse } from "@semio-tech/framework/backbone";
import type { DocumentScope } from "@semio-tech/framework/document/scope";

/** 🚦️ Waits for the exact predecessor's acknowledged Hub head before opening its successor. */
export function settleRoleDocumentV1(worker: Worker, scope: DocumentScope, clientInstanceId: string, current: () => boolean): Promise<boolean> {
  if (!current()) return Promise.resolve(false);
  const requestId = crypto.randomUUID().replaceAll("-", "");
  return new Promise((resolve, reject) => {
    const finish = (settled: boolean): void => {
      clearInterval(check);
      clearTimeout(deadline);
      worker.removeEventListener("message", receive);
      resolve(settled);
    };
    const receive = (event: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>): void => {
      const message = "wire" in event.data ? decodeBackboneWorkerResponse(event.data.wire) : event.data;
      if (message.kind === "document-settled" && message.requestId === requestId && message.clientInstanceId === clientInstanceId
        && message.scope.spaceId === scope.spaceId && message.scope.documentId === scope.documentId) finish(current() && message.settled);
    };
    const check = setInterval(() => { if (!current()) finish(false); }, 100);
    const deadline = setTimeout(() => finish(false), 16_000);
    worker.addEventListener("message", receive);
    try {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "document-settle", requestId, clientInstanceId, scope }) });
    } catch (error) {
      clearInterval(check);
      clearTimeout(deadline);
      worker.removeEventListener("message", receive);
      reject(error);
    }
  });
}
