/** 🧬️ AviArtifact — full artifact state, mirrors AviSnapshot field for field. */
export interface AviArtifact {
  schema: string;
  mainHeader: import("./📸️snapshot/🟦️component").AviMainHeader;
  streams: import("./📸️snapshot/🟦️component").AviStream[];
  idx1Present: boolean;
  unknownChunks: import("./📸️snapshot/🟦️component").RiffChunk[];
}
