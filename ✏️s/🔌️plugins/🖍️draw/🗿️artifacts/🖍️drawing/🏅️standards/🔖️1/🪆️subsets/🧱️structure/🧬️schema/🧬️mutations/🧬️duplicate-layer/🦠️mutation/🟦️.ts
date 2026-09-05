/** 🧬️ Drawing mutation — `DuplicateLayer` payload mirror: copies an existing layer to a new,
 * content-addressed id right after its source. Source address only — the duplicate's id is
 * deterministic (content-addressed from the source, see the Rust `clone_drawing_layer_node`), so
 * `diff`/`inverse` take it as an already-resolved parameter rather than recomputing the hash. */
export interface DuplicateLayer {
  layerId: string;
}
