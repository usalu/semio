import {
  parseRiffChunk,
  parseWavChunkRef,
  parseWavData,
  parseWavFmt,
  type RiffChunk,
  type WavChunkRef,
  type WavData,
  type WavFmt,
} from "../📸️snapshot/🟦️.ts";

/** ✂️ One sequential edit of the sample lane: `remove` elements from `index` replaced by `insert`. */
export type WavSplice = Readonly<{ index: number; remove: number; insert: WavData }>;

/** 🔺️ Sparse editable RIFF/WAVE state delta. */
export type WavDiff = Readonly<{
  fmt?: WavFmt;
  data?: WavData;
  dataSplices?: readonly WavSplice[];
  fmtPadByte?: number;
  dataPadByte?: number;
  otherChunks?: readonly RiffChunk[];
  chunkOrder?: readonly WavChunkRef[];
}>;

export class stdioWavRiffpcmAnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reject = (at: string, why: string): never => { throw new stdioWavRiffpcmAnyDiffGuardRefusal(at, why); };
const object = (value: unknown, at: string): Readonly<Record<string, unknown>> => value !== null && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : reject(at, "value is not an object");
const array = (value: unknown, at: string): readonly unknown[] => Array.isArray(value) ? value : reject(at, "value is not an array");
const nonNegative = (value: unknown, at: string): number => Number.isSafeInteger(value) && (value as number) >= 0 ? value as number : reject(at, "value is not a non-negative integer");
const byte = (value: unknown, at: string): number => Number.isSafeInteger(value) && (value as number) >= 0 && (value as number) <= 255 ? value as number : reject(at, "value is not a byte");

export function parseWavDiff(value: unknown, at = "$"): WavDiff {
  const row = object(value, at);
  return {
    ...(row.fmt === undefined ? {} : { fmt: parseWavFmt(row.fmt, `${at}.fmt`) }),
    ...(row.data === undefined ? {} : { data: parseWavData(row.data, `${at}.data`) }),
    ...(row.dataSplices === undefined ? {} : { dataSplices: array(row.dataSplices, `${at}.dataSplices`).map((item, index) => { const splice = object(item, `${at}.dataSplices[${index}]`); return { index: nonNegative(splice.index, `${at}.dataSplices[${index}].index`), remove: nonNegative(splice.remove, `${at}.dataSplices[${index}].remove`), insert: parseWavData(splice.insert, `${at}.dataSplices[${index}].insert`) }; }) }),
    ...(row.fmtPadByte === undefined ? {} : { fmtPadByte: byte(row.fmtPadByte, `${at}.fmtPadByte`) }),
    ...(row.dataPadByte === undefined ? {} : { dataPadByte: byte(row.dataPadByte, `${at}.dataPadByte`) }),
    ...(row.otherChunks === undefined ? {} : { otherChunks: array(row.otherChunks, `${at}.otherChunks`).map((item, index) => parseRiffChunk(item, `${at}.otherChunks[${index}]`)) }),
    ...(row.chunkOrder === undefined ? {} : { chunkOrder: array(row.chunkOrder, `${at}.chunkOrder`).map((item, index) => parseWavChunkRef(item, `${at}.chunkOrder[${index}]`)) }),
  };
}
