/** 🔁 replace-tracks mutation payload — whole-value swap of `ReconstructionResults.tracks`. */
export interface ReplaceTracks {
  tracks: { id: string; length: number; class: "static" | "moving"; meanSpeedMS: number }[];
}
