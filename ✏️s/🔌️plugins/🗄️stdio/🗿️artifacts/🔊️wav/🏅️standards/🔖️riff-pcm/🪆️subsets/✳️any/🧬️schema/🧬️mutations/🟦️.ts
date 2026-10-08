
import type {WavFmt,WavData,RiffChunk,WavChunkRef,WavSnapshot} from "../📸️snapshot/🟦️.ts";
export type {WavFmt,WavData,RiffChunk,WavChunkRef,WavSnapshot} from "../📸️snapshot/🟦️.ts";

/** 🧬️ WAV mutation aggregate, matching the native internally tagged union exactly. */
export type WavMutation =
  | Readonly<{ mutation: "setFmt"; fmt: WavFmt }>
  | Readonly<{ mutation: "setData"; data: WavData }>
  | Readonly<{ mutation: "patchData"; index: number; removeCount: number; data: WavData; moveTo?: number }>
  | Readonly<{ mutation: "setOtherChunks"; chunks: readonly RiffChunk[]; chunkOrder?: readonly WavChunkRef[] }>;
