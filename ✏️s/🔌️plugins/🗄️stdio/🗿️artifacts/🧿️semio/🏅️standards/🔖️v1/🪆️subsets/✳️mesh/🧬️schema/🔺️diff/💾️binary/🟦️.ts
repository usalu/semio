/** 🔺️ `s.stdio.semio.mesh` DiffCodec BINARY representation — identical to the TEXT grammar
 * (`../📝️text/🟦️.ts`), UTF-8 encoded, no additional framing. */
export interface Stdio_semio_mesh_diff_binary_envelope {
  textUtf8: Uint8Array; // UTF-8 bytes of the print_diff() text grammar
}
