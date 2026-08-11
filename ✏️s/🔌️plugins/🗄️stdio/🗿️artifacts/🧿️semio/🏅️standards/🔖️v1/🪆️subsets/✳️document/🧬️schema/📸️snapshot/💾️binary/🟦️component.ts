/** 💾️ Pack binary envelope marker for `s.stdio.semio.document.snapshot`: a `store::semio_format`
 * header (magic "SEMI" + component tag + version + u32 length) followed by that many bytes of
 * UTF-8 JSON (see the sibling `🦀️component.rs` marker and `SemioDocumentSnapshot`'s real
 * `ArtifactPack` impl for the source of truth). */
export const BINARY_MAGIC = "s.stdio.semio.document";
export interface SemioDocumentSnapshotPackEnvelope {
  magic: "SEMI";
  componentTag: number;
  envelopeVersion: number;
  bodyLen: number;
  jsonBody: Uint8Array;
}
