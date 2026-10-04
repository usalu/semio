import {type Binary32,parseBinary32Transport} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🎚️ Typed primary WAV format chunk. */
export type WavFmt = Readonly<{
  audioFormat: number;
  channels: number;
  sampleRate: number;
  byteRate: number;
  blockAlign: number;
  bitsPerSample: number;
  ext?: readonly number[];
}>;

/** 🔊️ Typed primary WAV sample chunk. */
export type WavData =
  | Readonly<{ kind: "pcm16"; value: readonly number[] }>
  | Readonly<{ kind: "pcm8"; value: readonly number[] }>
  | Readonly<{ kind: "float32"; value: readonly Binary32[] }>
  | Readonly<{ kind: "raw"; value: readonly number[] }>;

/** 📦️ Verbatim auxiliary or duplicate canonical RIFF chunk. */
export type RiffChunk = Readonly<{ fourcc: string; data: readonly number[]; padByte: number }>;

/** 🧭️ One position in the complete top-level RIFF/WAVE chunk sequence. */
export type WavChunkRef =
  | Readonly<{ kind: "format" }>
  | Readonly<{ kind: "samples" }>
  | Readonly<{ kind: "other"; value: bigint }>;

/** 🧬️ Complete editable WAV state. */
export type WavSnapshot = Readonly<{
  schema: string;
  fmt: WavFmt;
  data: WavData;
  fmtPadByte: number;
  dataPadByte: number;
  otherChunks: readonly RiffChunk[];
  chunkOrder: readonly WavChunkRef[];
}>;

export const MAXIMUM_FMT_EXTENSION_BYTES = 65_535;
export * from "./🪶️sqlite/🟦️.ts";

export class stdioWavRiffpcmAnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reject = (at: string, why: string): never => {
  throw new stdioWavRiffpcmAnySnapshotGuardRefusal(at, why);
};
const object = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : reject(at, "value is not an object");
const array = (value: unknown, at: string): readonly unknown[] => Array.isArray(value) ? value : reject(at, "value is not an array");
const string = (value: unknown, at: string): string => typeof value === "string" ? value : reject(at, "value is not a string");
const integer = (value: unknown, at: string, minimum: number, maximum: number): number =>
  Number.isSafeInteger(value) && (value as number) >= minimum && (value as number) <= maximum ? value as number : reject(at, `value is not an integer in ${minimum}..${maximum}`);
const bytes = (value: unknown, at: string): readonly number[] => array(value, at).map((item, index) => integer(item, `${at}[${index}]`, 0, 255));

export const parseWavFmt = (value: unknown, at: string): WavFmt => {
  const row = object(value, at);
  const ext = row.ext === undefined ? undefined : bytes(row.ext, `${at}.ext`);
  return {
    audioFormat: integer(row.audioFormat, `${at}.audioFormat`, 0, 65_535),
    channels: integer(row.channels, `${at}.channels`, 0, 65_535),
    sampleRate: integer(row.sampleRate, `${at}.sampleRate`, 0, 4_294_967_295),
    byteRate: integer(row.byteRate, `${at}.byteRate`, 0, 4_294_967_295),
    blockAlign: integer(row.blockAlign, `${at}.blockAlign`, 0, 65_535),
    bitsPerSample: integer(row.bitsPerSample, `${at}.bitsPerSample`, 0, 65_535),
    ...(ext === undefined ? {} : { ext }),
  };
};

export const parseWavData = (value: unknown, at: string): WavData => {
  const row = object(value, at);
  const kind = string(row.kind, `${at}.kind`);
  const values = array(row.value, `${at}.value`);
  if (kind === "pcm16") return { kind, value: values.map((item, index) => integer(item, `${at}.value[${index}]`, -32_768, 32_767)) };
  if (kind === "pcm8" || kind === "raw") return { kind, value: bytes(values, `${at}.value`) };
  if (kind === "float32") return { kind, value: values.map((item, index) => parseBinary32Transport(item)) };
  return reject(`${at}.kind`, "unknown WAV data kind");
};

export const parseRiffChunk = (value: unknown, at: string): RiffChunk => {
  const row = object(value, at);
  const fourcc = string(row.fourcc, `${at}.fourcc`);
  return { fourcc, data: bytes(row.data, `${at}.data`), padByte: integer(row.padByte === undefined ? 0 : row.padByte, `${at}.padByte`, 0, 255) };
};

export const parseWavChunkRef = (value: unknown, at: string): WavChunkRef => {
  const row = object(value, at);
  const kind = string(row.kind, `${at}.kind`);
  if (kind === "format" || kind === "samples") return { kind };
  if (kind === "other") { if (typeof row.value !== "bigint" || row.value < 0n || row.value > 18446744073709551615n) reject(`${at}.value`, "value must be an unsigned64 word"); return { kind, value: row.value as bigint }; }
  return reject(`${at}.kind`, "unknown WAV chunk reference");
};

/** 🧭️ Refuses snapshot states that cannot survive one exact RIFF/WAVE save and reopen. */
export function validateWavSerialization(snapshot: WavSnapshot, at = "$"): WavSnapshot {
  if (snapshot.schema !== "stdio.wav") reject(`${at}.schema`, "native WAV wire requires stdio.wav identity");
  if (snapshot.fmt.ext !== undefined && snapshot.fmt.ext.length > MAXIMUM_FMT_EXTENSION_BYTES) reject(`${at}.fmt.ext`, `native fmt extension exceeds ${MAXIMUM_FMT_EXTENSION_BYTES} bytes`);
  const fmtPayloadIsOdd = snapshot.fmt.ext !== undefined && snapshot.fmt.ext.length % 2 === 1;
  if (snapshot.fmtPadByte !== 0 && !fmtPayloadIsOdd) reject(`${at}.fmtPadByte`, "nonzero pad byte requires an odd serialized fmt payload length");
  const dataPayloadIsOdd = (snapshot.data.kind === "pcm8" || snapshot.data.kind === "raw") && snapshot.data.value.length % 2 === 1;
  if (snapshot.dataPadByte !== 0 && !dataPayloadIsOdd) reject(`${at}.dataPadByte`, "nonzero pad byte requires an odd serialized data payload length");
  snapshot.otherChunks.forEach((chunk, index) => {
    if (!/^[ -~]{4}$/u.test(chunk.fourcc)) reject(`${at}.otherChunks[${index}].fourcc`, "native fourcc requires four printable ASCII bytes");
    if (chunk.padByte !== 0 && chunk.data.length % 2 === 0) reject(`${at}.otherChunks[${index}].padByte`, "nonzero pad byte requires an odd chunk payload length");
  });
  return snapshot;
}

export function parseWavSnapshot(value: unknown, at = "$"): WavSnapshot {
  const row = object(value, at);
  return {
    schema: string(row.schema, `${at}.schema`),
    fmt: parseWavFmt(row.fmt, `${at}.fmt`),
    data: parseWavData(row.data, `${at}.data`),
    fmtPadByte: integer(row.fmtPadByte === undefined ? 0 : row.fmtPadByte, `${at}.fmtPadByte`, 0, 255),
    dataPadByte: integer(row.dataPadByte === undefined ? 0 : row.dataPadByte, `${at}.dataPadByte`, 0, 255),
    otherChunks: array(row.otherChunks, `${at}.otherChunks`).map((item, index) => parseRiffChunk(item, `${at}.otherChunks[${index}]`)),
    chunkOrder: array(row.chunkOrder, `${at}.chunkOrder`).map((item, index) => parseWavChunkRef(item, `${at}.chunkOrder[${index}]`)),
  };
}
