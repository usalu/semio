/** 🔊️ WAV natural sample table window. */

export const WAV_EDIT_WINDOW_KIND_ID = "framework.window.table" as const;
export const WAV_EDIT_BODY_KEY = "framework.window.table" as const;

export const WAV_SAMPLE_TABLE_ACTIONS = {
  setSample: "set-cell",
  appendFrame: "add-row",
  insertFrame: "insert-frame",
  removeFrame: "remove-row",
  appendChannel: "add-column",
  insertChannel: "insert-channel",
  removeChannel: "remove-column",
  setSampleRate: "set-sample-rate",
} as const;
