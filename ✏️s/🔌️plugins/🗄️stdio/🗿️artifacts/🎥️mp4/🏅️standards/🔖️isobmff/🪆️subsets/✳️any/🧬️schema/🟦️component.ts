/** 🧬️ Mp4Artifact — full artifact state, mirrors Mp4Snapshot field for field. */
export interface Mp4Artifact {
  schema: string;
  ftyp: import("./📸️snapshot/🟦️component").Mp4Ftyp;
  movie: import("./📸️snapshot/🟦️component").Mp4Movie;
  tracks: import("./📸️snapshot/🟦️component").Mp4Track[];
  unknownBoxes: import("./📸️snapshot/🟦️component").Mp4Box[];
}
