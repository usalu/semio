/** 💾️ Binary representation for `stdio.epw` (snapshot): the shared `.semio` binary
 * envelope — 8-byte magic, u32 LE token length, UTF-8 token `"stdio.epw.pack v1"` — wrapping
 * a payload that is the UTF-8 bytes of the SAME EPW document the text facet parses
 * (`store::semio_format::wrap_binary`/`unwrap_binary`). */
export type EpwSnapshotBinary = Uint8Array;
