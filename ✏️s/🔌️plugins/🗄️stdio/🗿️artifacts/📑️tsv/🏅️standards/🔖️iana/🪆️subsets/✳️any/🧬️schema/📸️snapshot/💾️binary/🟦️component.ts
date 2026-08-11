/** 💾️ Binary representation for `stdio.tsv` (snapshot): the shared `.semio` binary
 * envelope — 8-byte magic, u32 LE token length, UTF-8 token `"stdio.tsv.pack v1"` — wrapping
 * a payload that is the UTF-8 bytes of the SAME TSV document the text facet parses
 * (`store::semio_format::wrap_binary`/`unwrap_binary`). */
export type TsvSnapshotBinary = Uint8Array;
