/** 🧬️ DAG editor — the compiled-DAG window: typed twin of `🦀️component.rs`'s read-only wire-literal
 * render boundary. `surfaceKind` stays node-graph (verbatim from the pre-migration manifest) even
 * though `render()` builds a text-editor scene — see the Rust file's own doc comment. */

/** 🧱️ The compiled window's typed view-model — the TS mirror of the Rust `render()` boundary's
 * inputs (`DagSnapshot` + `DagCamera`) and output (a read-only text-editor scene carrying the
 * fixture's `wire` literal). */
export interface DagPlayCompiledViewModel {
  windowKindId: "dag-compiled-dag";
  bodyKey: "dag.play.compiled-dag";
  surfaceId: "dag.play.compiled-dag";
  language: "wire";
  buffer: string;
}

export const DAG_PLAY_WINDOW_COMPILED = "dag-compiled-dag" as const;
export const DAG_PLAY_BODY_COMPILED = "dag.play.compiled-dag" as const;
export const DAG_PLAY_SURFACE_COMPILED = "dag.play.compiled-dag" as const;
