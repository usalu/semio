/** 🧬️ AviArtifact — full artifact state, mirrors AviSnapshot field for field. */
export interface AviArtifact {
  schema: string;
  mainHeader: import("./📸️snapshot/🟦️").AviMainHeader;
  streams: import("./📸️snapshot/🟦️").AviStream[];
  idx1Present: boolean;
  unknownChunks: import("./📸️snapshot/🟦️").RiffChunk[];
}
