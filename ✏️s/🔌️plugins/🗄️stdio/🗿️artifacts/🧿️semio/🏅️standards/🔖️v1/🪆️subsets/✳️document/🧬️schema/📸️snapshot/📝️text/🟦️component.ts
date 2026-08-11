/** 📄️ DSL text envelope marker for `s.stdio.semio.document.snapshot`: `preamble\nhexBody`, where
 * `hexBody` is the snapshot's JSON encoding, 2 hex digits per byte (see the sibling `🦀️component.rs`
 * marker and `SemioDocumentSnapshot`'s real `ArtifactDsl` impl for the source of truth). */
export const TEXT_MARKER = "s.stdio.semio.document";
export interface SemioDocumentSnapshotDslText {
  preamble: string;
  hexBody: string;
}
