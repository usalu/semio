/** 🎥️ Browser video export: capability probing and clip encoding.
 *
 * - MP4 (H.264, 4:2:0) uses the first-party `AvcPcmEncoder`/`writeAvcMp4` of the raster video tier: valid, constant-frame-rate, but
 *   uncompressed (`I_PCM` pictures, `P_Skip` repeats), and without alpha (transparent frames are composited over the background colour).
 * - WebM uses `MediaRecorder` on a canvas stream where the browser supports it. Timestamps come from the wall clock, so the frame rate is
 *   only approximately constant, and alpha survives only where the browser's VP8/VP9 encoder writes it (reported as `unverified`).
 * <https://www.w3.org/TR/mediastream-recording/> · <https://www.itu.int/rec/T-REC-H.264> */
import type { JobControl } from "../../🔗️contracts/🟦️.ts";
import { AvcPcmEncoder, writeAvcMp4, type EncodedVideoSample } from "../../../../../../🧰️framework/🔨️modules/🖌️raster/🎥️video/🟦️.ts";

/** 🎞️ A frame source item: pixel data or a 2D canvas. */
export type VideoFrame = { readonly data: Uint8ClampedArray; readonly width: number; readonly height: number } | { readonly getContext: (kind: "2d") => { getImageData: (x: number, y: number, w: number, h: number) => { data: Uint8ClampedArray; width: number; height: number } } | null; readonly width: number; readonly height: number };

/** 🎬️ The output formats. */
export type VideoFormat = "mp4" | "webm";

/** 🔎️ One probed codec. */
export type CodecSupport = { readonly format: VideoFormat; readonly codec: string; readonly mime: string; readonly available: boolean; readonly alpha: "none" | "unverified"; readonly engine: "first-party" | "MediaRecorder" };

/** 🔎️ What this runtime can encode. */
export type VideoCapabilities = { readonly codecs: readonly CodecSupport[]; readonly pngSequence: true };

const WEBM_CANDIDATES = [["vp9", "video/webm;codecs=vp9"], ["vp8", "video/webm;codecs=vp8"], ["av1", "video/webm;codecs=av01"]] as const;
const MP4_CANDIDATES = [["avc1", "video/mp4;codecs=avc1.42E01E"]] as const;

/** 🔎️ Probes `MediaRecorder.isTypeSupported` for WebM VP9/VP8/AV1 and MP4 AVC, and lists the first-party H.264 MP4 writer (always available). */
export function videoCapabilities(): VideoCapabilities {
  const recorder = (globalThis as { MediaRecorder?: { isTypeSupported(mime: string): boolean } }).MediaRecorder;
  const probe = (mime: string): boolean => recorder?.isTypeSupported(mime) === true;
  return {
    pngSequence: true,
    codecs: [
      { format: "mp4", codec: "h264-pcm", mime: "video/mp4", available: true, alpha: "none", engine: "first-party" },
      ...MP4_CANDIDATES.map(([codec, mime]) => ({ format: "mp4" as const, codec, mime, available: probe(mime), alpha: "none" as const, engine: "MediaRecorder" as const })),
      ...WEBM_CANDIDATES.map(([codec, mime]) => ({ format: "webm" as const, codec, mime, available: probe(mime), alpha: "unverified" as const, engine: "MediaRecorder" as const })),
    ],
  };
}

/** 🎬️ The encoded clip and what is honestly true about it. */
export type EncodedClip = { readonly bytes: Uint8Array; readonly mime: string; readonly codec: string; readonly alpha: "none" | "unverified"; readonly frames: number; readonly note: { readonly en: string; readonly de: string } };

/** ⚙️ Encoding options. */
export type EncodeOptions = { readonly fps: number; readonly frameCount: number; readonly background?: string; readonly codec?: string; readonly control?: JobControl };

function pixels(frame: VideoFrame): { readonly data: Uint8ClampedArray; readonly width: number; readonly height: number } {
  if ("data" in frame) return frame;
  const context = frame.getContext("2d");
  if (context === null) throw new Error("canvas has no 2d context");
  return context.getImageData(0, 0, frame.width, frame.height);
}

function flatten(data: Uint8ClampedArray, background: readonly [number, number, number]): Uint8ClampedArray {
  const out = new Uint8ClampedArray(data.length);
  for (let at = 0; at < data.length; at += 4) {
    const alpha = data[at + 3]! / 255;
    for (let channel = 0; channel < 3; channel += 1) out[at + channel] = Math.round(data[at + channel]! * alpha + background[channel]! * (1 - alpha));
    out[at + 3] = 255;
  }
  return out;
}

const parseBackground = (color: string | undefined): readonly [number, number, number] => {
  const match = /^#([0-9a-f]{6})$/iu.exec(color ?? "#ffffff");
  if (!match) throw new Error(`background must be #rrggbb, got ${color}`);
  const value = Number.parseInt(match[1]!, 16);
  return [(value >> 16) & 255, (value >> 8) & 255, value & 255];
};

async function encodeMp4(frames: AsyncIterable<VideoFrame>, options: EncodeOptions): Promise<EncodedClip> {
  const background = parseBackground(options.background);
  let encoder: AvcPcmEncoder | undefined;
  let previous: Uint8ClampedArray | undefined;
  const samples: EncodedVideoSample[] = [];
  for await (const frame of frames) {
    options.control?.signal?.throwIfAborted();
    const { data, width, height } = pixels(frame);
    encoder ??= new AvcPcmEncoder({ width, height, fps: options.fps });
    const rgba = flatten(data, background);
    const same = previous !== undefined && previous.length === rgba.length && previous.every((value, index) => value === rgba[index]);
    samples.push(same ? encoder.repeat() : encoder.encode(rgba));
    previous = rgba;
    options.control?.progress?.(samples.length, options.frameCount, { en: "Encoding MP4", de: "MP4 wird kodiert" });
  }
  if (!encoder) throw new Error("clip has no frames");
  return { bytes: writeAvcMp4(encoder.parameters, encoder.configuration(), samples), mime: "video/mp4", codec: "h264-pcm", alpha: "none", frames: samples.length, note: { en: "H.264 I_PCM, constant frame rate, no alpha channel; transparent pixels are composited over the background colour.", de: "H.264 I_PCM, konstante Bildrate, kein Alphakanal; transparente Pixel werden über die Hintergrundfarbe gerechnet." } };
}

async function encodeWebm(frames: AsyncIterable<VideoFrame>, options: EncodeOptions): Promise<EncodedClip> {
  const caps = videoCapabilities().codecs.filter((entry) => entry.format === "webm" && entry.available && (options.codec === undefined || entry.codec === options.codec));
  const chosen = caps[0];
  if (!chosen) throw new Error("no WebM codec is supported by MediaRecorder in this browser");
  const host = globalThis as unknown as { document?: { createElement(tag: "canvas"): HTMLCanvasElement }; MediaRecorder: new (stream: MediaStream, options: { mimeType: string }) => MediaRecorder };
  if (!host.document) throw new Error("WebM recording needs a DOM canvas");
  const canvas = host.document.createElement("canvas");
  const context = canvas.getContext("2d")!;
  const chunks: Blob[] = [];
  let recorder: MediaRecorder | undefined;
  let track: (MediaStreamTrack & { requestFrame?: () => void }) | undefined;
  let count = 0;
  for await (const frame of frames) {
    options.control?.signal?.throwIfAborted();
    const { data, width, height } = pixels(frame);
    if (!recorder) {
      canvas.width = width;
      canvas.height = height;
      const stream = (canvas as HTMLCanvasElement & { captureStream(rate: number): MediaStream }).captureStream(0);
      track = stream.getVideoTracks()[0];
      recorder = new host.MediaRecorder(stream, { mimeType: chosen.mime });
      recorder.ondataavailable = (event) => { if (event.data.size > 0) chunks.push(event.data); };
      recorder.start();
    }
    context.clearRect(0, 0, width, height);
    context.putImageData(new ImageData(new Uint8ClampedArray(data), width, height), 0, 0);
    track?.requestFrame?.();
    await new Promise((done) => setTimeout(done, 1000 / options.fps));
    count += 1;
    options.control?.progress?.(count, options.frameCount, { en: "Recording WebM", de: "WebM wird aufgenommen" });
  }
  if (!recorder) throw new Error("clip has no frames");
  const stopped = new Promise<void>((done) => { recorder!.onstop = () => done(); });
  recorder.stop();
  await stopped;
  const bytes = new Uint8Array(await new Blob(chunks, { type: chosen.mime }).arrayBuffer());
  return { bytes, mime: "video/webm", codec: chosen.codec, alpha: "unverified", frames: count, note: { en: "Recorded in real time through MediaRecorder: the frame rate is approximately constant; alpha is kept only if this browser's encoder writes it.", de: "In Echtzeit über MediaRecorder aufgenommen: Die Bildrate ist nur näherungsweise konstant; Alpha bleibt nur erhalten, wenn der Encoder dieses Browsers es schreibt." } };
}

/** 🎬️ Encodes `frames` (ImageData-like or canvases, in schedule order) as MP4 or WebM, honest about alpha and frame-rate guarantees. */
export function encodeClip(frames: AsyncIterable<VideoFrame>, format: VideoFormat, options: EncodeOptions): Promise<EncodedClip> {
  return format === "mp4" ? encodeMp4(frames, options) : encodeWebm(frames, options);
}
