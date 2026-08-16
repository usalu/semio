/** 📋️ Imperative editor — the main window: typed twin of `🦀️component.rs`'s `render()` boundary. A
 * table of the document's top-level steps plus, once `run` has been dispatched, the resulting scope
 * rows appended below them. */

/** ✏️ One table row — either a step (`id` = the step id) or a `run`-output entry
 * (`id` = `"run-output.<key>"` / `"run-output"`), see `run_output_rows`' doc comment. */
export interface ImperativePlayMainRow {
  index: number;
  id: string;
  kind: string;
}

/** ✏️ The main window's typed view-model — mirrors `render(document, run_output_json, labels)`'s
 * three inputs plus the localized column headers `imperative_labels` resolves. */
export interface ImperativePlayMainViewModel {
  windowKindId: "imperative-main";
  bodyKey: "imperative.play.main";
  surfaceId: "imperative.play.main";
  columns: { id: "index" | "id" | "kind"; label: string }[];
  rows: ImperativePlayMainRow[];
}

export const IMPERATIVE_PLAY_WINDOW_MAIN = "imperative-main" as const;
export const IMPERATIVE_PLAY_BODY_MAIN = "imperative.play.main" as const;
export const IMPERATIVE_PLAY_SURFACE_MAIN = "imperative.play.main" as const;
