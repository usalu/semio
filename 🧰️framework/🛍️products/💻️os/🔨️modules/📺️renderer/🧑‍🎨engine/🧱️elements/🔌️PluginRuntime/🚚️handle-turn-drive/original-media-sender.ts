/** 📤 Sends one selected file to the issued instance loan with a bounded read per accepted page. */
export type OriginalMediaLoan = Readonly<{
  operationId: string;
  generation: string;
  instanceId: number;
  documentId: string;
  baseRevision: string;
}>;

export type OriginalMediaFrame =
  | Readonly<{kind: "begin"; loan: OriginalMediaLoan; name: string; mediaType: string; byteLength: number}>
  | Readonly<{kind: "page"; loan: OriginalMediaLoan; index: number; byteLength: number}>
  | Readonly<{kind: "complete" | "cancel"; loan: OriginalMediaLoan}>;

export type OriginalMediaSource = Readonly<{
  name: string;
  type: string;
  size: number;
  slice(start: number, end: number): Readonly<{arrayBuffer(): Promise<ArrayBuffer>}>;
}>;

type OriginalMediaSink = (frame: OriginalMediaFrame, bytes?: Uint8Array) => Promise<void>;

export const ORIGINAL_MEDIA_PAGE_BYTES = 4096;

function hasBoundedText(value: string, maximum: number): boolean {
  let length = 0;
  for (const _ of value) {
    if (++length > maximum) return false;
  }
  return length > 0;
}

function readMediaBody(
  slice: Readonly<{arrayBuffer(): Promise<ArrayBuffer>}>,
  signal: AbortSignal,
): Promise<ArrayBuffer> {
  signal.throwIfAborted();
  return new Promise((resolve, reject) => {
    const cancelled = () => reject(signal.reason);
    signal.addEventListener("abort", cancelled, {once: true});
    const settle = () => signal.removeEventListener("abort", cancelled);
    Promise.resolve().then(() => {
      signal.throwIfAborted();
      return slice.arrayBuffer();
    }).then(
      buffer => { settle(); resolve(buffer); },
      error => { settle(); reject(error); },
    );
  });
}

export async function sendOriginalMedia(
  loan: OriginalMediaLoan,
  source: OriginalMediaSource,
  maximumFileBytes: number,
  sink: OriginalMediaSink,
  control: {signal: AbortSignal; current: () => boolean; progress: (completed: number, total: number) => void},
): Promise<void> {
  const check = () => {
    control.signal.throwIfAborted();
    if (!control.current()) throw Error("The original media selection no longer belongs to this document");
  };
  check();
  const {size, name} = source;
  if (!Number.isSafeInteger(size) || !Number.isSafeInteger(maximumFileBytes) || size <= 0 || size > Math.min(maximumFileBytes, 512 * 1024 * 1024)) {
    throw Error("The selected media exceeds its admitted source extent");
  }
  const mediaType = source.type || "application/octet-stream";
  const {operationId, generation, instanceId, documentId, baseRevision} = loan;
  if (!hasBoundedText(name, 255) || !hasBoundedText(documentId, 256) || mediaType.length > 255 || !/^[a-z0-9][a-z0-9.+-]*\/[a-z0-9][a-z0-9.+-]*$/.test(mediaType) ||
      !/^[0-9a-f]{16}$/.test(operationId) || !/^[0-9a-f]{16}$/.test(generation) || !/^[0-9a-f]{64}$/.test(baseRevision) ||
      !Number.isInteger(instanceId) || instanceId < 0 || instanceId > 0xffffffff) {
    throw Error("The selected media metadata or source loan is invalid");
  }
  const captured = Object.freeze({operationId, generation, instanceId, documentId, baseRevision});
  try {
    await sink({kind: "begin", loan: captured, name, mediaType, byteLength: size});
    for (let offset = 0, index = 0; offset < size; index++) {
      check();
      const end = Math.min(size, offset + ORIGINAL_MEDIA_PAGE_BYTES);
      const bytes = new Uint8Array(await readMediaBody(source.slice(offset, end), control.signal));
      check();
      if (bytes.length !== end - offset) throw Error("The selected media changed during the read");
      await sink({kind: "page", loan: captured, index, byteLength: bytes.length}, bytes);
      offset = end;
      control.progress(offset, size);
    }
    check();
    await sink({kind: "complete", loan: captured});
  } catch (failure) {
    try {
      await sink({kind: "cancel", loan: captured});
    } catch (error) {
      throw new AggregateError([failure, error], "Original media transfer and retirement failed");
    }
    throw failure;
  }
}
