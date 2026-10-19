/** 🧬️ `s.stdio.semio.mesh` binary envelope — magic + token-length-prefixed token + the real
 * `SemioMeshSnapshot` JSON payload (`serde_json::to_vec`). Mirrors the sibling
 * `../../../🧬️schema/📸️snapshot/🦀️.rs` binary marker; see `../../../🧬️schema/📸️snapshot/🦀️.rs` for the real field
 * shape the JSON payload decodes to. */
export interface Stdio_semio_mesh_snapshot_binary_envelope {
  magic: Uint8Array;
  tokenLen: number;
  token: string;
  payload: Uint8Array;
}
