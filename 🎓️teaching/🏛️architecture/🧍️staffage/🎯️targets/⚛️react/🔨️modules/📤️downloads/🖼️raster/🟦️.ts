/** 🖼️ Browser rasterisation and recording: SVG frames to PNG through a canvas, and canvas streams to WebM or MP4 through `MediaRecorder`; every probe reads the real browser.
 *
 * @see ../🟦️.ts — the export actions that call these
 */

/** 🎞️ A host capability that turns a painted SVG into PNG bytes (alpha kept when the SVG has none painted behind it). */
export type Rasteriser = (svg: string, widthPx: number, heightPx: number, signal?: AbortSignal) => Promise<Uint8Array>;

/** 🎥️ A host capability that records frames, supplied in order, into a video container. */
export type Recorder = (frames: AsyncIterable<Uint8Array>, size: { readonly widthPx: number; readonly heightPx: number }, fps: number, mime: string, signal?: AbortSignal) => Promise<{ readonly bytes: Uint8Array; readonly mime: string }>;

/** 🔎️ Whether this runtime can draw SVG into a canvas and read PNG back. */
export function canRasterise(): boolean {
  if (typeof document === "undefined" || typeof Image === "undefined" || typeof URL === "undefined" || typeof URL.createObjectURL !== "function") return false;
  try {
    return document.createElement("canvas").getContext("2d") !== null;
  } catch {
    return false;
  }
}

function aborted(signal?: AbortSignal): void {
  if (signal?.aborted) throw new DOMException("Export cancelled", "AbortError");
}

async function toCanvas(svg: string, widthPx: number, heightPx: number, signal?: AbortSignal): Promise<HTMLCanvasElement> {
  aborted(signal);
  const url = URL.createObjectURL(new Blob([svg], { type: "image/svg+xml;charset=utf-8" }));
  try {
    const image = new Image();
    image.decoding = "async";
    image.src = url;
    await image.decode();
    aborted(signal);
    const canvas = document.createElement("canvas");
    canvas.width = widthPx;
    canvas.height = heightPx;
    const context = canvas.getContext("2d")!;
    context.drawImage(image, 0, 0, widthPx, heightPx);
    return canvas;
  } finally {
    URL.revokeObjectURL(url);
  }
}

/** 🖼️ The browser rasteriser: image decode, canvas draw, `toBlob("image/png")`. */
export const browserRasteriser: Rasteriser = async (svg, widthPx, heightPx, signal) => {
  const canvas = await toCanvas(svg, widthPx, heightPx, signal);
  const blob = await new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, "image/png"));
  if (blob === null) throw new Error("canvas.toBlob returned no PNG");
  return new Uint8Array(await blob.arrayBuffer());
};

/** 🎥️ The container candidates, best first, that `MediaRecorder` can actually produce here. */
export function recorderMimes(container: "mp4" | "webm"): string[] {
  if (typeof MediaRecorder === "undefined" || typeof MediaRecorder.isTypeSupported !== "function") return [];
  const candidates = container === "mp4" ? ["video/mp4;codecs=avc1.640028", "video/mp4;codecs=avc1", "video/mp4;codecs=h264", "video/mp4"] : ["video/webm;codecs=vp9", "video/webm;codecs=vp8", "video/webm;codecs=av1", "video/webm"];
  return candidates.filter((mime) => MediaRecorder.isTypeSupported(mime));
}

const wait = (ms: number, signal?: AbortSignal): Promise<void> =>
  new Promise((resolve, reject) => {
    const timer = setTimeout(resolve, Math.max(0, ms));
    signal?.addEventListener("abort", () => {
      clearTimeout(timer);
      reject(new DOMException("Export cancelled", "AbortError"));
    }, { once: true });
  });

/** 🎥️ Records PNG frames into a video at the stated frame rate through `MediaRecorder` and `canvas.captureStream`. Frames are paced in real time; the codec is the one the browser chose. */
export const browserRecorder: Recorder = async (frames, size, fps, mime, signal) => {
  const canvas = document.createElement("canvas");
  canvas.width = size.widthPx;
  canvas.height = size.heightPx;
  const context = canvas.getContext("2d")!;
  const stream = canvas.captureStream(0);
  const track = stream.getVideoTracks()[0] as MediaStreamTrack & { requestFrame?: () => void };
  const recorder = new MediaRecorder(stream, { mimeType: mime, videoBitsPerSecond: 8_000_000 });
  const chunks: Blob[] = [];
  recorder.ondataavailable = (event) => event.data.size > 0 && chunks.push(event.data);
  const stopped = new Promise<void>((resolve) => (recorder.onstop = () => resolve()));
  recorder.start();
  const started = performance.now();
  let n = 0;
  try {
    for await (const png of frames) {
      aborted(signal);
      const bitmap = await createImageBitmap(new Blob([png as BlobPart], { type: "image/png" }));
      context.clearRect(0, 0, size.widthPx, size.heightPx);
      context.drawImage(bitmap, 0, 0);
      bitmap.close();
      track.requestFrame?.();
      n++;
      await wait(started + (n * 1000) / fps - performance.now(), signal);
    }
  } finally {
    if (recorder.state !== "inactive") recorder.stop();
    await stopped;
    for (const t of stream.getTracks()) t.stop();
  }
  aborted(signal);
  const blob = new Blob(chunks, { type: recorder.mimeType || mime });
  return { bytes: new Uint8Array(await blob.arrayBuffer()), mime: recorder.mimeType || mime };
};
