import { lifecycle, nextImageDecodeRequestId, pageImageDecode, setNextImageDecodeRequestId, setPageImageDecode } from "../🧵️state/🟦️.ts";
import type { ReferenceImageDimensions } from "../../🖼️reference-image-decode/🟦️.ts";
import { post } from "../⏱️steps-and-reporting/🟦️.ts";
import type { BrowserFrameUiMessage } from "../../🚚️browser-frame-transport/📋️contract/🟦️.ts";

export function requestPageImageDecode(source: Uint8Array<ArrayBuffer>, dimensions: ReferenceImageDimensions): Promise<ImageBitmap> {
  if (pageImageDecode) return Promise.reject(new Error("reference-image-decode-credits"));
  const requestId = (setNextImageDecodeRequestId(nextImageDecodeRequestId + 1) - 1);
  return new Promise<ImageBitmap>((resolve, reject) => {
    setPageImageDecode({ requestId, resolve, reject });
    try {
      post({ kind: "image-decode", lifecycle, requestId, source, dimensions }, [source.buffer]);
    } catch (error) {
      setPageImageDecode(undefined);
      reject(error instanceof Error ? error : new Error(String(error)));
    }
  });
}

export function settlePageImageDecode(message: Extract<BrowserFrameUiMessage, { readonly kind: "image-decode-result" }>): void {
  const pending = pageImageDecode;
  if (!pending || pending.requestId !== message.requestId) {
    message.bitmap?.close();
    return;
  }
  setPageImageDecode(undefined);
  if (message.bitmap) pending.resolve(message.bitmap);
  else pending.reject(new Error(message.detail ?? "reference-image-decode-refused"));
}

export function cancelPageImageDecode(): void {
  const pending = pageImageDecode;
  if (!pending) return;
  setPageImageDecode(undefined);
  try {
    post({ kind: "image-decode-cancel", lifecycle, requestId: pending.requestId });
  } catch {}
  pending.reject(new DOMException("reference image generation retired", "AbortError"));
}
