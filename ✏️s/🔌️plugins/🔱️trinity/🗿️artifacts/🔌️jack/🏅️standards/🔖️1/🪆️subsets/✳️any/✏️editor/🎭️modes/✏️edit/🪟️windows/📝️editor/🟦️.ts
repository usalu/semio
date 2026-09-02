/** 📝️ Trinity Jack editor — Query editor window: typed twin of `🦀️.rs`'s
 * `build_text_editor_scene` boundary (Jack query text + live tokens/diagnostics/completions/
 * occurrences, all pre-serialized JSON blobs on the Rust side — kept as opaque strings here rather
 * than re-declaring `core::semantic_tokens`'s internal shape in TS). */

/** ✂️ A `{start, end}` text selection, mirrors `cfg.editor_selection`. */
export interface TrinityJackEditEditorSelection {
  start: number;
  end: number;
}

/** 🧱️ The Query editor window's typed view-model — the TS mirror of the Rust `render()` boundary. */
export interface TrinityJackEditEditorViewModel {
  windowKindId: "trinity-jack-edit-editor";
  bodyKey: "trinity.jack.edit.editor";
  surfaceId: "trinity.jack.edit.editor";
  query: string;
  selection?: TrinityJackEditEditorSelection;
  tokensJson?: string;
  diagnosticsJson?: string;
  completionsJson?: string;
  occurrencesJson?: string;
  editable: true;
}

export const TRINITY_JACK_EDIT_EDITOR_WINDOW_KIND_ID = "trinity-jack-edit-editor" as const;
export const TRINITY_JACK_EDIT_EDITOR_BODY_KEY = "trinity.jack.edit.editor" as const;
export const TRINITY_JACK_EDIT_EDITOR_SURFACE_ID = "trinity.jack.edit.editor" as const;
