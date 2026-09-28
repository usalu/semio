import type { SnapshotPatch } from "../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts";

/** 🎚️ Typed RIFF `fmt ` chunk carried by WAV snapshots and mutations. */
export type WavFmt = Readonly<{
  audioFormat: number;
  channels: number;
  sampleRate: number;
  byteRate: number;
  blockAlign: number;
  bitsPerSample: number;
  ext?: readonly number[];
}>;

/** 🔊️ Adjacently tagged WAV sample payload. */
export type WavData =
  | Readonly<{ kind: "pcm16"; value: readonly number[] }>
  | Readonly<{ kind: "pcm8"; value: readonly number[] }>
  | Readonly<{ kind: "float32"; value: readonly number[] }>
  | Readonly<{ kind: "raw"; value: readonly number[] }>;

/** 📦️ Verbatim retained auxiliary or duplicate canonical RIFF chunk. */
export type RiffChunk = Readonly<{ fourcc: string; data: readonly number[]; padByte?: number }>;

/** 🧭️ One position in the complete top-level RIFF/WAVE chunk sequence. */
export type WavChunkRef =
  | Readonly<{ kind: "format" }>
  | Readonly<{ kind: "samples" }>
  | Readonly<{ kind: "other"; value: number }>;

/** 🧬️ Authoritative typed WAV snapshot shape used by aggregate mutations. */
export type WavSnapshot = Readonly<{
  schema: "stdio.wav";
  fmt: WavFmt;
  data: WavData;
  fmtPadByte?: number;
  dataPadByte?: number;
  otherChunks: readonly RiffChunk[];
  chunkOrder: readonly WavChunkRef[];
}>;

/** 🧬️ WAV mutation aggregate, matching the native internally tagged union exactly. */
export type WavMutation =
  | Readonly<{ mutation: "setSnapshot"; snapshot: WavSnapshot }>
  | Readonly<{ mutation: "patchSnapshot"; patch: SnapshotPatch }>
  | Readonly<{ mutation: "setFmt"; fmt: WavFmt }>
  | Readonly<{ mutation: "setData"; data: WavData }>
  | Readonly<{ mutation: "patchData"; index: number; removeCount: number; data: WavData; moveTo?: number }>
  | Readonly<{ mutation: "setOtherChunks"; chunks: readonly RiffChunk[] }>;
