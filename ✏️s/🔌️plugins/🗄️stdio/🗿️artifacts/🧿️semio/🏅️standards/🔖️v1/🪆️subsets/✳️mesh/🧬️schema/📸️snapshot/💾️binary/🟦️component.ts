/** 🧬️ `s.stdio.semio.mesh` binary envelope — magic + token-length-prefixed token + the real
 * `SemioMeshSnapshot` JSON payload (`serde_json::to_vec`). Mirrors the sibling
 * `../🦀️component.rs` binary marker; see `../📸️snapshot/../🦀️component.rs` for the real field
 * shape the JSON payload decodes to. */
export interface Stdio_semio_mesh_snapshot_binary_envelope {
  magic: Uint8Array;      // 8 bytes: 0x89 "SEM" CR LF SUB LF
  tokenLen: number;       // u32 LE
  token: string;          // "stdio.semio.mesh.pack v1"
  payload: Uint8Array;    // serde_json::to_vec(SemioMeshSnapshot)
}
