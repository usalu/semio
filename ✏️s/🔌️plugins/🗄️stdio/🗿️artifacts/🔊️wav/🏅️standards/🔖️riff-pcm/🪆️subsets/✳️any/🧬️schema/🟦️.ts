import { parseWavSnapshot, type WavSnapshot } from "./📸️snapshot/🟦️.ts";

/** 🧬️ WAV artifact state is the complete WAV snapshot schema. */
export type WavArtifact = WavSnapshot;

export const parseWavArtifact = parseWavSnapshot;
