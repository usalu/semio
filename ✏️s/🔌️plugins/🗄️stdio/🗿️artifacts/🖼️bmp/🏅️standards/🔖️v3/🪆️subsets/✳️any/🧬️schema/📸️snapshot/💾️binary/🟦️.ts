/** 💾️ Binary representation for `stdio.bmp` (snapshot): the shared `.semio` binary
 * envelope — 8-byte magic, u32 LE token length, UTF-8 token `"stdio.bmp.pack v1"` — wrapping
 * a payload that IS the real on-disk BMP bytes (BITMAPFILEHEADER + BITMAPINFOHEADER +
 * optional BI_BITFIELDS masks + optional palette + pixel data), the SAME bytes
 * `engine::decode_bmp`/`encode_bmp` read and write
 * (`store::semio_format::wrap_binary`/`unwrap_binary`). */
export type BmpSnapshotBinary = Uint8Array;
