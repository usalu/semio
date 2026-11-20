import { lifecycle, nextImageDecodeRequestId, pageImageDecode, setNextImageDecodeRequestId, setPageImageDecode } from "../🧵️state/🟦️.ts";
import type { ReferenceImageDimensions } from "../../🖼️reference-image-decode/🟦️.ts";
import { post } from "../⏱️steps-and-reporting/🟦️.ts";
import type { BrowserFrameUiMessage } from "../../🚚️browser-frame-transport/📋️contract/🟦️.ts";

const MAX_WAITING_DECODES = 16;
type DecodeOwner = "reference" | "icon";
interface DecodeCredit {
  readonly owner: DecodeOwner;
  readonly signal: AbortSignal;
  abort(): void;
}
interface WaitingCredit {
  readonly credit: DecodeCredit;
  readonly enter: () => void;
}
const waiting: WaitingCredit[] = [];
let active: DecodeCredit | undefined;

export function hasReferencePageImageDecode(): boolean {
  return active?.owner === "reference" || waiting.some(({ credit }) => credit.owner === "reference");
}

export function requestPageImageDecode(source: Uint8Array<ArrayBuffer>, dimensions: ReferenceImageDimensions): Promise<ImageBitmap> {
  return decodeOnPage(async () => source, dimensions, "reference");
}

export function requestPageIconSvgDecode(source: () => Promise<Uint8Array<ArrayBuffer>>, dimensions: ReferenceImageDimensions, signal: AbortSignal): Promise<ImageBitmap> {
  return decodeOnPage(source, dimensions, "icon", signal);
}

async function decodeOnPage(source: () => Promise<Uint8Array<ArrayBuffer>>, dimensions: ReferenceImageDimensions, owner: DecodeOwner, signal?: AbortSignal): Promise<ImageBitmap> {
  signal?.throwIfAborted();
  if (waiting.length >= MAX_WAITING_DECODES) throw new Error("reference-image-decode-credits");
  const controller = new AbortController();
  const credit: DecodeCredit = { owner, signal: controller.signal, abort: () => controller.abort(new DOMException("image decode retired", "AbortError")) };
  const cancel = () => credit.abort();
  signal?.addEventListener("abort", cancel, { once: true });
  try {
    await new Promise<void>((resolve, reject) => {
      if (!active) {
        active = credit;
        resolve();
        return;
      }
      const onCancel = () => {
        const index = waiting.indexOf(entry);
        if (index >= 0) waiting.splice(index, 1);
        reject(credit.signal.reason);
      };
      const entry = { credit, enter: () => { credit.signal.removeEventListener("abort", onCancel); resolve(); } };
      credit.signal.addEventListener("abort", onCancel, { once: true });
      waiting.push(entry);
    });
    credit.signal.throwIfAborted();
    const bytes = await source();
    credit.signal.throwIfAborted();
    return await dispatchPageImageDecode(bytes, dimensions, credit.signal);
  } finally {
    signal?.removeEventListener("abort", cancel);
    if (active === credit) {
      const next = waiting.shift();
      active = next?.credit;
      next?.enter();
    }
  }
}

function dispatchPageImageDecode(source: Uint8Array<ArrayBuffer>, dimensions: ReferenceImageDimensions, signal: AbortSignal): Promise<ImageBitmap> {
  const requestId = setNextImageDecodeRequestId(nextImageDecodeRequestId + 1) - 1;
  return new Promise<ImageBitmap>((resolve, reject) => {
    const cancel = () => {
      setPageImageDecode(undefined);
      try { post({ kind: "image-decode-cancel", lifecycle, requestId }); } catch {}
      reject(signal.reason);
    };
    const done = () => signal.removeEventListener("abort", cancel);
    signal.addEventListener("abort", cancel, { once: true });
    setPageImageDecode({
      requestId,
      resolve: (bitmap) => { done(); resolve(bitmap); },
      reject: (error) => { done(); reject(error); },
    });
    try {
      post({ kind: "image-decode", lifecycle, requestId, source, dimensions }, [source.buffer]);
    } catch (error) {
      setPageImageDecode(undefined);
      done();
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

export function cancelPageImageDecode(owner?: DecodeOwner): void {
  if (!owner || active?.owner === owner) active?.abort();
  for (const { credit } of [...waiting]) {
    if (!owner || credit.owner === owner) credit.abort();
  }
}
