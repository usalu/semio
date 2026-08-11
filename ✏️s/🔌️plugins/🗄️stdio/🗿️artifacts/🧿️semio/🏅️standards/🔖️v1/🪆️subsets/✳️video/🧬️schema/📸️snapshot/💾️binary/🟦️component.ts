/** 💾️ Binary envelope mirror for `stdio.semio.video` snapshot: magic + version + length-prefixed
 * JSON body (see the sibling 📸️snapshot/🟦️component.ts for the JSON body's own shape). */
export interface SemioVideoSnapshotBinaryEnvelope {
  magic: "stdio.semio.video";
  version: number;
  bodyLen: number;
  body: Uint8Array;
}
