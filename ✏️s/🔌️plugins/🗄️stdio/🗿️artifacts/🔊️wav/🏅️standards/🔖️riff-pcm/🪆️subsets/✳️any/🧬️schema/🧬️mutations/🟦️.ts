import type { SnapshotPatch } from "../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts";

import type {WavFmt,WavData,RiffChunk,WavChunkRef,WavSnapshot} from "../📸️snapshot/🟦️.ts";
export type {WavFmt,WavData,RiffChunk,WavChunkRef,WavSnapshot} from "../📸️snapshot/🟦️.ts";

/** 🧬️ WAV mutation aggregate, matching the native internally tagged union exactly. */
export type WavMutation =
  | Readonly<{ mutation: "setSnapshot"; snapshot: WavSnapshot }>
  | Readonly<{ mutation: "patchSnapshot"; patch: SnapshotPatch }>
  | Readonly<{ mutation: "setFmt"; fmt: WavFmt }>
  | Readonly<{ mutation: "setData"; data: WavData }>
  | Readonly<{ mutation: "patchData"; index: number; removeCount: number; data: WavData; moveTo?: number }>
  | Readonly<{ mutation: "setOtherChunks"; chunks: readonly RiffChunk[] }>;
