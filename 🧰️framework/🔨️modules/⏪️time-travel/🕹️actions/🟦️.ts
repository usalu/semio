/** ⏪️ TypeScript twin of `semio_framework::history_edit` (`🦀️.rs` beside this file): the reserved history-edit verbs of the
 * time-travel session, their arguments and the finalize dialog id. Exported through `@semio-tech/framework`. */

/** ✏️ Mirrors Rust `HISTORY_EDIT_ACTION_IDS` — the thirteen reserved history-edit verbs, host-driven on the instance's
 * one time-travel session. Every verb but `historyEditBegin`/`historyEditExit` may carry `generation`
 * ({@link HISTORY_EDIT_ARG_GENERATION}); without it the verb addresses the live session. */
export const HISTORY_EDIT_ACTION_IDS = [
  "historyEditBegin",
  "historyEditInput",
  "historyEditUseSelection",
  "historyEditWithdraw",
  "historyEditAccept",
  "historyEditDiscard",
  "historyEditRestore",
  "historyEditFinalize",
  "historyEditCommit",
  "historyEditBack",
  "historyEditExit",
  "historyEditCancelReplay",
  "historyEditRerun",
] as const;
export type HistoryEditActionId = (typeof HISTORY_EDIT_ACTION_IDS)[number];
/** 🪪️ The mutation id argument of `historyEditBegin`, of a row's `historyEditWithdraw` and of `historyEditRestore` — mirrors Rust `HISTORY_EDIT_ARG_MUTATION_ID`. */
export const HISTORY_EDIT_ARG_MUTATION_ID = "mutationId";
/** 🧩️ The member store argument (`<slot>/<childId>`) beside every `mutationId`, absent for the document's own store — mirrors Rust `HISTORY_EDIT_ARG_STORE`. */
export const HISTORY_EDIT_ARG_STORE = "store";
/** 🧭️ The input pointer argument — mirrors Rust `HISTORY_EDIT_ARG_PATH`. */
export const HISTORY_EDIT_ARG_PATH = "path";
/** 🎚️ `historyEditInput`'s value argument (the new value, or the inserted item) — mirrors Rust `HISTORY_EDIT_ARG_VALUE`. */
export const HISTORY_EDIT_ARG_VALUE = "value";
/** ✂️ `historyEditInput`'s list edit argument (absent sets; `insert`/`remove` one item at `path`) — mirrors Rust `HISTORY_EDIT_ARG_EDIT`. */
export const HISTORY_EDIT_ARG_EDIT = "edit";
/** ➕️ The list edit that inserts an item — mirrors Rust `HISTORY_EDIT_INPUT_INSERT`. */
export const HISTORY_EDIT_INPUT_INSERT = "insert";
/** ➖️ The list edit that removes an item — mirrors Rust `HISTORY_EDIT_INPUT_REMOVE`. */
export const HISTORY_EDIT_INPUT_REMOVE = "remove";
/** 🧿️ The session generation argument — mirrors Rust `HISTORY_EDIT_ARG_GENERATION`. */
export const HISTORY_EDIT_ARG_GENERATION = "generation";
/** 🏷️ `historyEditCommit`'s alternative name argument — mirrors Rust `HISTORY_EDIT_ARG_NAME`. */
export const HISTORY_EDIT_ARG_NAME = "name";
/** ✍️ The overwrite choice of `historyEditCommit` — mirrors Rust `HISTORY_EDIT_CHOICE_OVERWRITE`. */
export const HISTORY_EDIT_CHOICE_OVERWRITE = "overwrite";
/** 🗳️ The finalize dialog id the history-edit contribution reserves on every editor — mirrors Rust `HISTORY_EDIT_FINALIZE_DIALOG_ID`. */
export const HISTORY_EDIT_FINALIZE_DIALOG_ID = "finalizeHistoryEdit";
