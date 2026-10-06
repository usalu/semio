/** 💾️ Binary representation for `stdio.csv` (diff): `protocol::OpBinary::encode_op` /
 * `decode_op` — raw `serde_json::to_vec`/`from_slice` of `CsvDiff`, no envelope header
 * (unlike the snapshot facet, ops are never wrapped in the `.semio` binary envelope). */
export type CsvDiffBinary = Uint8Array;
