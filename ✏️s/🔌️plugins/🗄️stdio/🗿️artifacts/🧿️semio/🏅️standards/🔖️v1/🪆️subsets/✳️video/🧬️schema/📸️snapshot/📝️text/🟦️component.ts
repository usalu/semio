/** 📝️ Text representation mirror for `stdio.semio.video` (snapshot): envelope header line +
 * hex(JSON) body. The JSON body's own structure is `../🟦️component.ts`'s `SemioVideoSnapshot`. */
export interface SemioVideoSnapshotTextEnvelope {
  header: "schema stdio.semio.video";
  /** hex-encoded UTF-8 JSON, decodes as `SemioVideoSnapshot` (see ../🟦️component.ts) */
  bodyHex: string;
}
