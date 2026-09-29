/** 🎥️ The video tier of the raster module — TypeScript twin of `🦀️.rs` beside this file: RGBA8 frames → H.264
 * (AVC) access units → ISO-BMFF (MP4) bytes, first-party and dependency-free. `AvcPcmEncoder` writes every new picture
 * as an exact all-`I_PCM` IDR and every repeat as an all-`P_Skip` P picture; `writeAvcMp4` muxes any encoder's AVCC
 * samples (this one's, or WebCodecs' `VideoEncoder` chunks on the browser host) into a progressive MP4. Both twins answer
 * `🧫️fixtures/🔣️.json` byte for byte, and `🧪️tests/🎞️ffmpeg-decode` decodes every fixture stream with FFmpeg.
 * <https://www.itu.int/rec/T-REC-H.264> · <https://www.iso.org/standard/83102.html> · <https://www.iso.org/standard/83529.html> */

//#region 🔖️Parameters
/** 🎛️ The stream every encoder and the muxer agree on. */
export interface VideoStreamParameters {
  readonly width: number;
  readonly height: number;
  readonly fps: number;
}

/** 📏️ The largest picture edge the first-party encoder admits. */
export const VIDEO_MAXIMUM_EDGE_PIXELS = 4096;

/** 🎞️ The fastest frame rate the tier admits. */
export const VIDEO_MAXIMUM_FPS = 120;

/** 🧾️ `profile_idc` Baseline. */
export const AVC_PROFILE_BASELINE = 66;

/** 🧾️ `constraint_set0_flag` + `constraint_set1_flag` (Constrained Baseline). */
export const AVC_CONSTRAINED_BASELINE_FLAGS = 0xc0;

/** 📶️ H.264 Table A-1 rows: `[level_idc, MaxFS, MaxMBPS]`. */
export const AVC_LEVEL_LIMITS: readonly (readonly [number, number, number])[] = [
  [10, 99, 1_485],
  [11, 396, 3_000],
  [12, 396, 6_000],
  [13, 396, 11_880],
  [20, 396, 11_880],
  [21, 792, 19_800],
  [22, 1_620, 20_250],
  [30, 1_620, 40_500],
  [31, 3_600, 108_000],
  [32, 5_120, 216_000],
  [40, 8_192, 245_760],
  [41, 8_192, 245_760],
  [42, 8_704, 522_240],
  [50, 22_080, 589_824],
  [51, 36_864, 983_040],
  [52, 36_864, 2_073_600],
];

/** 🧱️ Picture width and height in 16×16 macroblocks. */
export function videoMacroblocks(parameters: VideoStreamParameters): { readonly width: number; readonly height: number } {
  return { width: Math.ceil(parameters.width / 16), height: Math.ceil(parameters.height / 16) };
}

/** 📶️ The smallest Table A-1 level holding the stream's frame size and macroblock rate (5.2 when none does). */
export function avcLevelIdc(parameters: VideoStreamParameters): number {
  const mbs = videoMacroblocks(parameters);
  const frame = mbs.width * mbs.height;
  const rate = frame * parameters.fps;
  return AVC_LEVEL_LIMITS.find(([, maxFrame, maxRate]) => frame <= maxFrame && rate <= maxRate)?.[0] ?? 52;
}

/** 🚨️ Everything the video tier refuses, in the Rust twin's vocabulary. */
export class VideoEncodeError extends Error {
  constructor(
    readonly code: "dimensions" | "frameRate" | "frameBytes" | "containerTooLarge" | "empty" | "nothingToRepeat" | "cancelled",
    message: string,
  ) {
    super(message);
  }
}

/** 🚦️ Refuses what no H.264 4:2:0 stream of this tier can carry. */
export function validateVideoStreamParameters(parameters: VideoStreamParameters): void {
  const { width, height, fps } = parameters;
  if (!Number.isInteger(width) || !Number.isInteger(height) || width <= 0 || height <= 0 || width > VIDEO_MAXIMUM_EDGE_PIXELS || height > VIDEO_MAXIMUM_EDGE_PIXELS || width % 2 !== 0 || height % 2 !== 0) {
    throw new VideoEncodeError("dimensions", `video picture ${width}x${height} is not an even size within 2..=${VIDEO_MAXIMUM_EDGE_PIXELS}`);
  }
  if (!Number.isInteger(fps) || fps <= 0 || fps > VIDEO_MAXIMUM_FPS) throw new VideoEncodeError("frameRate", `video frame rate ${fps} is outside 1..=${VIDEO_MAXIMUM_FPS}`);
}
//#endregion 🔖️Parameters

//#region 🔖️Bitstream
class BitWriter {
  private bytes: Uint8Array;
  private length = 0;
  private current = 0;
  private used = 0;

  constructor(capacity: number) {
    this.bytes = new Uint8Array(Math.max(16, capacity));
  }

  private push(value: number): void {
    if (this.length === this.bytes.length) {
      const grown = new Uint8Array(this.bytes.length * 2);
      grown.set(this.bytes);
      this.bytes = grown;
    }
    this.bytes[this.length++] = value;
  }

  bit(bit: number): void {
    this.current = ((this.current << 1) | (bit & 1)) & 0xff;
    this.used += 1;
    if (this.used === 8) {
      this.push(this.current);
      this.current = 0;
      this.used = 0;
    }
  }

  bits(value: number, count: number): void {
    for (let shift = count - 1; shift >= 0; shift -= 1) this.bit(Math.floor(value / 2 ** shift));
  }

  ue(value: number): void {
    const coded = value + 1;
    const length = Math.floor(Math.log2(coded)) + 1;
    for (let index = 1; index < length; index += 1) this.bit(0);
    this.bits(coded, length);
  }

  se(value: number): void {
    this.ue(value > 0 ? 2 * value - 1 : -2 * value);
  }

  alignZero(): void {
    while (this.used !== 0) this.bit(0);
  }

  byte(value: number): void {
    if (this.used === 0) this.push(value);
    else this.bits(value, 8);
  }

  append(bytes: Uint8Array): void {
    if (this.used !== 0) {
      for (const value of bytes) this.bits(value, 8);
      return;
    }
    if (this.length + bytes.length > this.bytes.length) {
      const grown = new Uint8Array(Math.max(this.bytes.length * 2, this.length + bytes.length));
      grown.set(this.bytes.subarray(0, this.length));
      this.bytes = grown;
    }
    this.bytes.set(bytes, this.length);
    this.length += bytes.length;
  }

  trailing(): Uint8Array {
    this.bit(1);
    this.alignZero();
    return this.bytes.subarray(0, this.length);
  }
}

/** 🛡️ RBSP → NAL unit with `emulation_prevention_three_byte` (§7.4.1). */
function nalUnit(nalRefIdc: number, nalUnitType: number, rbsp: Uint8Array): Uint8Array {
  const out = new Uint8Array(rbsp.length + Math.ceil(rbsp.length / 2) + 1);
  let length = 0;
  out[length++] = (nalRefIdc << 5) | nalUnitType;
  let zeros = 0;
  for (let index = 0; index < rbsp.length; index += 1) {
    const byte = rbsp[index]!;
    if (zeros >= 2 && byte <= 3) {
      out[length++] = 3;
      zeros = 0;
    }
    out[length++] = byte;
    zeros = byte === 0 ? zeros + 1 : 0;
  }
  return out.slice(0, length);
}
//#endregion 🔖️Bitstream

//#region 🔖️Encoder
/** 🧩️ One SPS and one PPS NAL unit (header byte included), exactly what `avcC` carries. `record` is a platform encoder's
 * own `avcC` (WebCodecs `decoderConfig.description`), written verbatim so its NAL length size stays the one its samples use. */
export interface AvcDecoderConfiguration {
  readonly sps: Uint8Array;
  readonly pps: Uint8Array;
  readonly record?: Uint8Array;
}

/** 🎞️ One access unit in AVCC framing (4-byte big-endian NAL lengths). */
export interface EncodedVideoSample {
  readonly data: Uint8Array;
  readonly sync: boolean;
}

/** 🔌️ The interface every H.264 encoder of this tier answers to (the first-party one below; the browser host plugs WebCodecs in
 * behind its own port): one sample per new picture, one per repeat of the previous picture. */
export interface VideoEncoderPort {
  readonly parameters: VideoStreamParameters;
  configuration(): AvcDecoderConfiguration;
  encode(rgba: Uint8Array | Uint8ClampedArray): EncodedVideoSample;
  repeat(): EncodedVideoSample;
}

/** 📐️ `frame_num` is 4 bits in every SPS this tier writes (`log2_max_frame_num_minus4 = 0`). */
const AVC_FRAME_NUM_MODULUS = 16;

/** 🎨️ BT.601 limited-range luma (integer form, identical in the Rust twin). */
export function bt601Luma(r: number, g: number, b: number): number {
  return ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
}

/** 🎨️ BT.601 limited-range blue-difference chroma. */
export function bt601Cb(r: number, g: number, b: number): number {
  return ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
}

/** 🎨️ BT.601 limited-range red-difference chroma. */
export function bt601Cr(r: number, g: number, b: number): number {
  return ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;
}

function sequenceParameterSet(parameters: VideoStreamParameters): Uint8Array {
  const mbs = videoMacroblocks(parameters);
  const bits = new BitWriter(32);
  bits.byte(AVC_PROFILE_BASELINE);
  bits.byte(AVC_CONSTRAINED_BASELINE_FLAGS);
  bits.byte(avcLevelIdc(parameters));
  bits.ue(0);
  bits.ue(0);
  bits.ue(2);
  bits.ue(1);
  bits.bit(0);
  bits.ue(mbs.width - 1);
  bits.ue(mbs.height - 1);
  bits.bit(1);
  bits.bit(1);
  const cropRight = (mbs.width * 16 - parameters.width) / 2;
  const cropBottom = (mbs.height * 16 - parameters.height) / 2;
  if (cropRight !== 0 || cropBottom !== 0) {
    bits.bit(1);
    bits.ue(0);
    bits.ue(cropRight);
    bits.ue(0);
    bits.ue(cropBottom);
  } else {
    bits.bit(0);
  }
  bits.bit(0);
  return nalUnit(3, 7, bits.trailing());
}

function pictureParameterSet(): Uint8Array {
  const bits = new BitWriter(8);
  bits.ue(0);
  bits.ue(0);
  bits.bit(0);
  bits.bit(0);
  bits.ue(0);
  bits.ue(0);
  bits.ue(0);
  bits.bit(0);
  bits.bits(0, 2);
  bits.se(0);
  bits.se(0);
  bits.se(0);
  bits.bit(1);
  bits.bit(0);
  bits.bit(0);
  return nalUnit(3, 8, bits.trailing());
}

/** 🎨️ The BT.601 planes of one picture padded to whole macroblocks (edge pixels repeated): luma per pixel, each chroma
 * sample the rounded mean of its 2×2 block — the Rust twin's per-macroblock arithmetic, computed once per plane. */
function pcmPlanes(parameters: VideoStreamParameters, rgba: Uint8Array | Uint8ClampedArray): { readonly luma: Uint8Array; readonly cb: Uint8Array; readonly cr: Uint8Array; readonly stride: number } {
  const { width, height } = parameters;
  const mbs = videoMacroblocks(parameters);
  const stride = mbs.width * 16;
  const rows = mbs.height * 16;
  const luma = new Uint8Array(stride * rows);
  const cbFull = new Int16Array(stride * rows);
  const crFull = new Int16Array(stride * rows);
  for (let y = 0; y < rows; y += 1) {
    const source = Math.min(y, height - 1) * width;
    for (let x = 0; x < stride; x += 1) {
      const index = (source + Math.min(x, width - 1)) * 4;
      const r = rgba[index]!;
      const g = rgba[index + 1]!;
      const b = rgba[index + 2]!;
      const at = y * stride + x;
      luma[at] = ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
      cbFull[at] = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
      crFull[at] = ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;
    }
  }
  const chromaStride = stride / 2;
  const cb = new Uint8Array(chromaStride * (rows / 2));
  const cr = new Uint8Array(chromaStride * (rows / 2));
  for (let y = 0; y < rows / 2; y += 1) {
    for (let x = 0; x < chromaStride; x += 1) {
      const top = 2 * y * stride + 2 * x;
      const bottom = top + stride;
      cb[y * chromaStride + x] = (cbFull[top]! + cbFull[top + 1]! + cbFull[bottom]! + cbFull[bottom + 1]! + 2) >> 2;
      cr[y * chromaStride + x] = (crFull[top]! + crFull[top + 1]! + crFull[bottom]! + crFull[bottom + 1]! + 2) >> 2;
    }
  }
  return { luma, cb, cr, stride };
}

function idrPcmSlice(parameters: VideoStreamParameters, rgba: Uint8Array | Uint8ClampedArray, idrPicId: number): Uint8Array {
  const mbs = videoMacroblocks(parameters);
  const { luma, cb, cr, stride } = pcmPlanes(parameters, rgba);
  const chromaStride = stride / 2;
  const bits = new BitWriter(mbs.width * mbs.height * 386 + 16);
  bits.ue(0);
  bits.ue(7);
  bits.ue(0);
  bits.bits(0, 4);
  bits.ue(idrPicId);
  bits.bit(0);
  bits.bit(0);
  bits.se(0);
  bits.ue(1);
  for (let mbY = 0; mbY < mbs.height; mbY += 1) {
    for (let mbX = 0; mbX < mbs.width; mbX += 1) {
      bits.ue(25);
      bits.alignZero();
      for (let y = 0; y < 16; y += 1) bits.append(luma.subarray((mbY * 16 + y) * stride + mbX * 16, (mbY * 16 + y) * stride + mbX * 16 + 16));
      for (const plane of [cb, cr]) {
        for (let y = 0; y < 8; y += 1) bits.append(plane.subarray((mbY * 8 + y) * chromaStride + mbX * 8, (mbY * 8 + y) * chromaStride + mbX * 8 + 8));
      }
    }
  }
  return nalUnit(3, 5, bits.trailing());
}

/** ⏯️ A P slice whose one `mb_skip_run` covers every macroblock — it decodes to its reference exactly (Rust `p_skip_slice`). */
function pSkipSlice(parameters: VideoStreamParameters, frameNum: number): Uint8Array {
  const mbs = videoMacroblocks(parameters);
  const bits = new BitWriter(16);
  bits.ue(0);
  bits.ue(5);
  bits.ue(0);
  bits.bits(frameNum, 4);
  bits.bit(0);
  bits.bit(0);
  bits.bit(0);
  bits.se(0);
  bits.ue(1);
  bits.ue(mbs.width * mbs.height);
  return nalUnit(2, 1, bits.trailing());
}

function avccSample(nal: Uint8Array, sync: boolean): EncodedVideoSample {
  const data = new Uint8Array(nal.length + 4);
  new DataView(data.buffer).setUint32(0, nal.length);
  data.set(nal, 4);
  return { data, sync };
}

/** 🧱️ First-party H.264 encoder: every new picture one IDR slice of `I_PCM` macroblocks, every repeat one P slice of
 * `P_Skip` macroblocks referencing the picture before it. */
export class AvcPcmEncoder implements VideoEncoderPort {
  private readonly config: AvcDecoderConfiguration;
  private idrPicId = 0;
  private frameNum: number | null = null;

  constructor(readonly parameters: VideoStreamParameters) {
    validateVideoStreamParameters(parameters);
    this.config = { sps: sequenceParameterSet(parameters), pps: pictureParameterSet() };
  }

  configuration(): AvcDecoderConfiguration {
    return this.config;
  }

  encode(rgba: Uint8Array | Uint8ClampedArray): EncodedVideoSample {
    const expected = this.parameters.width * this.parameters.height * 4;
    if (rgba.length !== expected) throw new VideoEncodeError("frameBytes", `video frame carries ${rgba.length} RGBA bytes, the stream needs ${expected}`);
    const slice = idrPcmSlice(this.parameters, rgba, this.idrPicId);
    this.idrPicId = (this.idrPicId + 1) % 2;
    this.frameNum = 0;
    return avccSample(slice, true);
  }

  repeat(): EncodedVideoSample {
    if (this.frameNum === null) throw new VideoEncodeError("nothingToRepeat", "video frame repeats a picture before any picture was encoded");
    this.frameNum = (this.frameNum + 1) % AVC_FRAME_NUM_MODULUS;
    return avccSample(pSkipSlice(this.parameters, this.frameNum), false);
  }
}
//#endregion 🔖️Encoder

//#region 🔖️Mp4Writer
const ascii = (text: string): number[] => [...text].map((character) => character.charCodeAt(0));
const be32 = (...values: readonly number[]): number[] => values.flatMap((value) => [(value >>> 24) & 0xff, (value >>> 16) & 0xff, (value >>> 8) & 0xff, value & 0xff]);
const be16 = (value: number): number[] => [(value >>> 8) & 0xff, value & 0xff];
const boxed = (kind: string, ...body: readonly (readonly number[])[]): number[] => {
  const flat = body.flat();
  return [...be32(flat.length + 8), ...ascii(kind), ...flat];
};
const fullBox = (kind: string, versionFlags: number, ...body: readonly (readonly number[])[]): number[] => boxed(kind, be32(versionFlags), ...body);
const UNITY_MATRIX = [0x0001_0000, 0, 0, 0, 0x0001_0000, 0, 0, 0, 0x4000_0000];

/** 🧩️ `avcC` — AVCDecoderConfigurationRecord for one SPS and one PPS with 4-byte NAL lengths. */
export function avcDecoderConfigurationRecord(configuration: AvcDecoderConfiguration): Uint8Array {
  if (configuration.record) return configuration.record;
  const { sps, pps } = configuration;
  return Uint8Array.from([1, sps[1] ?? 0, sps[2] ?? 0, sps[3] ?? 0, 0xff, 0xe1, ...be16(sps.length), ...sps, 1, ...be16(pps.length), ...pps]);
}

/** 🧩️ The SPS and PPS of an `avcC` record (what WebCodecs hands over as `decoderConfig.description`). */
export function avcConfigurationFromRecord(record: Uint8Array): AvcDecoderConfiguration {
  const view = new DataView(record.buffer, record.byteOffset, record.byteLength);
  const spsLength = view.getUint16(6);
  const sps = record.slice(8, 8 + spsLength);
  const ppsLength = view.getUint16(9 + spsLength);
  return { sps, pps: record.slice(11 + spsLength, 11 + spsLength + ppsLength), record };
}

/** 🔤️ `avc1.PPCCLL` — the RFC 6381 codec string of a configuration. */
export function avcCodecString(configuration: AvcDecoderConfiguration): string {
  const hex = (value: number | undefined): string => (value ?? 0).toString(16).toUpperCase().padStart(2, "0");
  return `avc1.${hex(configuration.sps[1])}${hex(configuration.sps[2])}${hex(configuration.sps[3])}`;
}

/** ⏱️ Movie duration in milliseconds of `frames` frames at `fps` (rounded half up). */
export function videoDurationMilliseconds(frames: number, fps: number): number {
  return Math.floor((frames * 1000 + Math.floor(fps / 2)) / Math.max(1, fps));
}

function moov(parameters: VideoStreamParameters, configuration: AvcDecoderConfiguration, samples: readonly EncodedVideoSample[], chunkOffset: number): number[] {
  const frames = samples.length;
  const durationMs = videoDurationMilliseconds(frames, parameters.fps);
  const mvhd = fullBox("mvhd", 0, be32(0, 0, 1000, durationMs, 0x0001_0000), [0x01, 0x00, 0, 0], new Array(8).fill(0), be32(...UNITY_MATRIX), new Array(24).fill(0), be32(2));
  const tkhd = fullBox("tkhd", 3, be32(0, 0, 1, 0, durationMs, 0, 0), new Array(8).fill(0), be32(...UNITY_MATRIX), be32(parameters.width * 65536, parameters.height * 65536));
  const mdhd = fullBox("mdhd", 0, be32(0, 0, parameters.fps, frames), [0x55, 0xc4, 0, 0]);
  const hdlr = fullBox("hdlr", 0, be32(0), ascii("vide"), new Array(12).fill(0), ascii("VideoHandler\0"));
  const vmhd = fullBox("vmhd", 1, new Array(8).fill(0));
  const dinf = boxed("dinf", fullBox("dref", 0, be32(1), fullBox("url ", 1)));
  const avc1 = boxed(
    "avc1",
    [0, 0, 0, 0, 0, 0, 0, 1],
    new Array(16).fill(0),
    be16(parameters.width),
    be16(parameters.height),
    be32(0x0048_0000, 0x0048_0000, 0),
    be16(1),
    new Array(32).fill(0),
    [0x00, 0x18, 0xff, 0xff],
    boxed("avcC", [...avcDecoderConfigurationRecord(configuration)]),
  );
  const stsd = fullBox("stsd", 0, be32(1), avc1);
  const stts = fullBox("stts", 0, be32(1, frames, 1));
  const sync = samples.flatMap((sample, index) => (sample.sync ? [index + 1] : []));
  const stss = sync.length === samples.length ? [] : fullBox("stss", 0, be32(sync.length), sync.flatMap((number) => be32(number)));
  const stsc = fullBox("stsc", 0, be32(1, 1, frames, 1));
  const stsz = fullBox("stsz", 0, be32(0, frames), samples.flatMap((sample) => be32(sample.data.length)));
  const stco = fullBox("stco", 0, be32(1, chunkOffset));
  const stbl = boxed("stbl", stsd, stts, stss, stsc, stsz, stco);
  const minf = boxed("minf", vmhd, dinf, stbl);
  const mdia = boxed("mdia", mdhd, hdlr, minf);
  const trak = boxed("trak", tkhd, mdia);
  return boxed("moov", mvhd, trak);
}

/** 📦️ Muxes AVCC `samples` (one frame each, presentation order) into a progressive MP4: `ftyp`, `moov`, `mdat`. */
export function writeAvcMp4(parameters: VideoStreamParameters, configuration: AvcDecoderConfiguration, samples: readonly EncodedVideoSample[]): Uint8Array {
  validateVideoStreamParameters(parameters);
  if (samples.length === 0) throw new VideoEncodeError("empty", "video has no frames");
  const mediaBytes = samples.reduce((sum, sample) => sum + sample.data.length, 0);
  const ftyp = boxed("ftyp", ascii("isom"), be32(0x200), ascii("isom"), ascii("iso2"), ascii("avc1"), ascii("mp41"));
  const headerBytes = ftyp.length + moov(parameters, configuration, samples, 0).length + 8;
  if (headerBytes + mediaBytes > 0xffff_ffff) throw new VideoEncodeError("containerTooLarge", `video media data of ${mediaBytes} bytes exceeds the 32-bit MP4 chunk offset range`);
  const head = [...ftyp, ...moov(parameters, configuration, samples, headerBytes), ...be32(mediaBytes + 8), ...ascii("mdat")];
  const out = new Uint8Array(head.length + mediaBytes);
  out.set(head);
  let cursor = head.length;
  for (const sample of samples) {
    out.set(sample.data, cursor);
    cursor += sample.data.length;
  }
  return out;
}
//#endregion 🔖️Mp4Writer

//#region 🔖️Pipeline
/** 🎬️ One run of identical frames: an RGBA8 picture shown for `frames` consecutive frames. */
export interface VideoFrameRun {
  readonly rgba: Uint8Array | Uint8ClampedArray;
  readonly frames: number;
}

/** 📈️ Encodes `runs` with `encoder` (each run's picture once, then a repeat per further frame) and muxes the result,
 * reporting `(done, total)` after every frame and stopping with a `cancelled` error as soon as `cancelled()` answers true. */
export function encodeVideoRuns(encoder: VideoEncoderPort, runs: readonly VideoFrameRun[], progress: (done: number, total: number) => void, cancelled: () => boolean = () => false): Uint8Array {
  const total = runs.reduce((sum, run) => sum + run.frames, 0);
  const samples: EncodedVideoSample[] = [];
  for (const run of runs) {
    for (let frame = 0; frame < run.frames; frame += 1) {
      if (cancelled()) throw new VideoEncodeError("cancelled", "video encoding was cancelled");
      samples.push(frame === 0 ? encoder.encode(run.rgba) : encoder.repeat());
      progress(samples.length, total);
    }
  }
  return writeAvcMp4(encoder.parameters, encoder.configuration(), samples);
}
//#endregion 🔖️Pipeline
