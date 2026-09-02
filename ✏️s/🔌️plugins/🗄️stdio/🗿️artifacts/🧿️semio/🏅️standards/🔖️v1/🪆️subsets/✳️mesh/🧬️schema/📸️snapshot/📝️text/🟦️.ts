/** 🧬️ `s.stdio.semio.mesh` DSL text representation — a preamble line followed by a
 * whitespace-tolerant ASCII hex dump of `SemioMeshSnapshot`'s own JSON bytes (see the sibling
 * `../🦀️.rs` snapshot type for the real field shape this hex payload decodes to). */
export interface Stdio_semio_mesh_snapshot_dsl_text {
  /** `semio stdio.semio.mesh.dsl v1` */
  preamble: string;
  /** whitespace-tolerant ASCII hex dump of SemioMeshSnapshot's JSON bytes */
  hexBody: string;
}
