/** 💾️ Binary representation for `stdio.bmp` (diff): the generic serde-derived encoding —
 * raw `serde_json::to_vec`/`from_slice` of `BmpDiff`, no envelope header (unlike the
 * snapshot facet, diffs are never wrapped in the `.semio` binary envelope). */
export type BmpDiffBinary = Uint8Array;
