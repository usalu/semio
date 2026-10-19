import { ASSET_RESPONSE_BYTE_CAPACITY, ASSET_RESPONSE_PAGE_BYTES, ASSET_SEAL_ATTEMPTS, assetAbort, assetCancellation, assetPumping, closed, closing, failed, frameTurns, hostIoSeq, lifecycle, pendingHostIo, quarantined, runtime, scope, setAssetAbort, setAssetPumping, setHostIoSeq, suspensionLedger } from "../🧵️state/🟦️.ts";
import { macrotask, monitoredSuspension, ownedStep, post } from "../⏱️steps-and-reporting/🟦️.ts";
import { assertBrowserAssetResponseContinuation } from "../🧩️asset-cancellation/🟦️.ts";
import { publishedPageUrl } from "@semio-tech/assets/published-page";
import { meshAssetTransportUrl } from "@semio-tech/assets/mesh";
import { MESH_DELIVERY_CATALOG } from "@semio-tech/assets/mesh/catalog";
import { concatenateReferenceImageSource, decodeReferenceImage, referenceImageBitmapForStage, referenceImageSourceDigest, referenceImageSourceDimensions, referenceImageTargetSize, streamReferenceImageBitmapRows } from "../../🖼️reference-image-decode/🟦️.ts";
import { requestPageImageDecode } from "../🖼️page-image-decode/🟦️.ts";
import { browserAssetFailureDisposition } from "../🧩️asset-failure/🟦️.ts";
import { fault } from "../🧯️close-and-fault/🟦️.ts";
import type { BrowserFrameUiMessage } from "../../🚚️browser-frame-transport/📋️contract/🟦️.ts";

type AssetRequest = {
  readonly available: boolean;
  readonly url?: string;
  readonly referenceImage?: boolean;
  readonly responseByteCapacity?: number;
  readonly pageByteCapacity?: number;
};

export function scheduleAssetPump(): void {
  if (!assetCancellation.pollAdmitted() || assetPumping || !runtime || closed || closing || failed || quarantined) return;
  setAssetPumping(true);
  setTimeout(() => void pumpAsset(), 0);
}

async function pumpAsset(): Promise<void> {
  let activeRequest: AssetRequest | undefined;
  let responseController: AbortController | undefined;
  try {
    if (!runtime || closed || closing || failed || quarantined) return;
    const request = ownedStep("asset-request", () => JSON.parse(runtime!.pollAssetRequest()) as AssetRequest);
    activeRequest = request;
    if (!request.available) return;
    if (!request.url || request.responseByteCapacity !== ASSET_RESPONSE_BYTE_CAPACITY || request.pageByteCapacity !== ASSET_RESPONSE_PAGE_BYTES) {
      throw new Error("asset-request-protocol: request descriptor did not match fixed Worker credits");
    }
    responseController = new AbortController();
    setAssetAbort(responseController);
    const responseCurrent = (): void => assertBrowserAssetResponseContinuation(responseController!, () => ownedStep("asset-response-current", () => runtime!.assetResponseCurrent()));
    const response = await monitoredSuspension("asset-fetch", () => fetch(publishedPageUrl(meshAssetTransportUrl(request.url!, MESH_DELIVERY_CATALOG)), { signal: responseController!.signal }));
    responseCurrent();
    if (!response.ok || !response.body) throw new Error(`asset-fetch-status: ${response.status}`);
    const declaredHeader = ownedStep("asset-response-headers", () => response.headers.get("content-length"));
    const declared = declaredHeader === null ? undefined : Number(declaredHeader);
    if (declared !== undefined && (!Number.isSafeInteger(declared) || declared < 0 || declared > ASSET_RESPONSE_BYTE_CAPACITY)) throw new Error("asset-response-length: Content-Length exceeded fixed aggregate credits");
    let reserved = false;
    for (let attempt = 0; attempt < ASSET_SEAL_ATTEMPTS && !reserved; attempt++) {
      responseCurrent();
      reserved = ownedStep("asset-response-reserve", () => runtime!.reserveAssetResponse(declared ?? ASSET_RESPONSE_BYTE_CAPACITY));
      if (!reserved) {
        await macrotask();
        responseCurrent();
      }
    }
    if (!reserved) throw new Error("asset-reserve-busy: the renderer never freed its interaction state for the response credits");
    const reader = ownedStep("asset-stream-reader", () => response.body!.getReader({ mode: "byob" }) as ReadableStreamBYOBReader);
    let received = 0;
    const referenceChunks: Uint8Array<ArrayBuffer>[] = [];
    for (;;) {
      responseCurrent();
      const pageOwner = ownedStep("asset-page-owner", () => new Uint8Array(ASSET_RESPONSE_PAGE_BYTES));
      const chunk = await monitoredSuspension("asset-stream-read", () => reader.read(pageOwner));
      responseCurrent();
      if (chunk.done) break;
      const bytes = chunk.value;
      if (bytes.byteLength === 0 || bytes.byteLength > ASSET_RESPONSE_PAGE_BYTES) throw new Error("asset-response-page: stream violated fixed BYOB page credits");
      received += bytes.byteLength;
      if (received > (declared ?? ASSET_RESPONSE_BYTE_CAPACITY)) throw new Error("asset-response-overflow: stream exceeded admitted bytes");
      responseCurrent();
      ownedStep("asset-page", () => runtime!.pushAssetResponsePage(bytes));
      if (request.referenceImage) referenceChunks.push(Uint8Array.from(bytes));
      await macrotask();
      responseCurrent();
    }
    ownedStep("asset-stream-release", () => reader.releaseLock());
    if (declared !== undefined && received !== declared) throw new Error("asset-response-short-read: stream ended before declared bytes");
    if (request.referenceImage) {
      responseCurrent();
      const source = ownedStep("reference-image-source", () => concatenateReferenceImageSource(referenceChunks));
      const dimensions = ownedStep("reference-image-dimensions", () => referenceImageSourceDimensions(source));
      const digest = await monitoredSuspension("reference-image-digest", () => referenceImageSourceDigest(source), suspensionLedger);
      responseCurrent();
      const [width, height] = referenceImageTargetSize(dimensions.width, dimensions.height);
      let stageMode = 0;
      for (let attempt = 0; attempt < ASSET_SEAL_ATTEMPTS && stageMode === 0; attempt++) {
        responseCurrent();
        stageMode = ownedStep("reference-image-begin", () => runtime!.beginReferenceImage(width, height, digest));
        if (stageMode === 0) {
          post({ kind: "wake", lifecycle });
          await macrotask();
          responseCurrent();
        }
      }
      if (stageMode === 0) throw new Error("reference-image-begin-busy");
      if (stageMode !== 1 && stageMode !== 2) throw new Error("reference-image-begin-mode");
      const bitmap = await referenceImageBitmapForStage(
        stageMode,
        () => monitoredSuspension("reference-image-decode", () => decodeReferenceImage(new Blob([source], { type: dimensions.mediaType }), dimensions, () => runtime?.assetResponseCurrent() === true), suspensionLedger),
        () => monitoredSuspension("reference-image-page-decode", () => requestPageImageDecode(source, dimensions), suspensionLedger),
      );
      responseCurrent();
      if (bitmap) {
        try {
          await streamReferenceImageBitmapRows(
            bitmap,
            width,
            height,
            (offset, pixels) => {
              responseCurrent();
              return runtime!.pushReferenceImageRows(offset, pixels);
            },
            () => {
              responseCurrent();
              return true;
            },
            async () => {
              post({ kind: "wake", lifecycle });
              await macrotask();
              responseCurrent();
            },
            (operation) => ownedStep("reference-image-row-strip", operation),
          );
        } finally {
          ownedStep("reference-image-bitmap-close", () => bitmap.close());
        }
      }
      responseCurrent();
      if (!ownedStep("reference-image-seal", () => runtime!.sealReferenceImage())) throw new Error("reference-image-seal-busy");
    }
    let sealed = false;
    for (let attempt = 0; attempt < ASSET_SEAL_ATTEMPTS && !sealed; attempt++) {
      responseCurrent();
      sealed = ownedStep("asset-seal", () => runtime!.sealAssetResponse());
      if (!sealed) {
        await macrotask();
        responseCurrent();
      }
    }
    if (!sealed) throw new Error("asset-seal-busy: the renderer never freed its interaction state for the sealed response");
    frameTurns?.request("assetDecode");
  } catch (error) {
    const disposition = browserAssetFailureDisposition(error, activeRequest?.referenceImage === true);
    if (runtime) {
      try {
        if (activeRequest && disposition === "missing") {
          let rejected = false;
          for (let attempt = 0; attempt < ASSET_SEAL_ATTEMPTS && !rejected; attempt++) {
            rejected = ownedStep("asset-reject", () => runtime!.rejectAssetResponse());
            if (!rejected) await macrotask();
          }
          if (!rejected) ownedStep("asset-abort", () => runtime!.abortAssetResponse());
        } else ownedStep("asset-abort", () => runtime!.abortAssetResponse());
      } catch {}
    }
    if (activeRequest && disposition === "missing" && !closing && !closed && !failed) post({ kind: "wake", lifecycle });
    if (disposition === "fault" && !closing && !closed) fault("asset-stream-fault", error instanceof Error ? error.message : String(error));
  } finally {
    if (assetAbort === responseController) setAssetAbort(undefined);
    setAssetPumping(false);
    if (assetCancellation.releaseReturned()) scheduleAssetPump();
  }
}

export function requestHostIo(requestJson: string, bytes: Uint8Array | null): Promise<string> {
  if (closed || closing || failed) return Promise.reject(new Error("wgpu-host-io: the frame Worker is closing"));
  setHostIoSeq(hostIoSeq + (1));
  const requestId = hostIoSeq;
  return new Promise<string>((resolve, reject) => {
    pendingHostIo.set(requestId, { resolve, reject });
    try {
      const carried = bytes ? new Uint8Array(bytes) : null;
      scope.postMessage({ kind: "host-io", lifecycle, requestId, request: requestJson, bytes: carried }, carried ? [carried.buffer] : []);
    } catch (error) {
      pendingHostIo.delete(requestId);
      reject(error instanceof Error ? error : new Error(String(error)));
    }
  });
}

export function settleHostIo(message: Extract<BrowserFrameUiMessage, { readonly kind: "host-io-result" }>): void {
  const pending = pendingHostIo.get(message.requestId);
  if (!pending) return;
  pendingHostIo.delete(message.requestId);
  if (message.json === null) pending.reject(new Error(message.detail ?? "wgpu-host-io refused"));
  else pending.resolve(message.json);
}
