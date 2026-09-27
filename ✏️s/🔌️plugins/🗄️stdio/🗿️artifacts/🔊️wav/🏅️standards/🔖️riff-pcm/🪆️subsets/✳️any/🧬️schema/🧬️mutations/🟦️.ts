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

/** 📦️ Verbatim retained RIFF chunk outside `fmt ` and `data`. */
export type RiffChunk = Readonly<{ fourcc: string; data: readonly number[] }>;

/** 🧬️ Authoritative typed WAV snapshot shape used by aggregate mutations. */
export type WavSnapshot = Readonly<{
  schema: string;
  fmt: WavFmt;
  data: WavData;
  otherChunks: readonly RiffChunk[];
}>;

/** 🧬️ WAV mutation aggregate, matching the native internally tagged union exactly. */
export type WavMutation =
  | Readonly<{ mutation: "setSnapshot"; snapshot: WavSnapshot }>
  | Readonly<{ mutation: "patchSnapshot"; patch: SnapshotPatch }>
  | Readonly<{ mutation: "setFmt"; fmt: WavFmt }>
  | Readonly<{ mutation: "setData"; data: WavData }>
  | Readonly<{ mutation: "patchData"; index: number; removeCount: number; data: WavData; moveTo?: number }>
  | Readonly<{ mutation: "setOtherChunks"; chunks: readonly RiffChunk[] }>;
