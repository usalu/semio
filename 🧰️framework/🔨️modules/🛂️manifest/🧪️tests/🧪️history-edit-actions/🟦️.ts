/** ✏️ The reserved history-edit verbs over `🧫️fixtures/🧫️history-edit-actions/🔣️.json` — TypeScript twin of the Rust law
 * beside this file; oracle: Ajv (2020-12) validates the fixture and rejects hostile rows. */
import { describe, expect, test } from "bun:test";

import fixture from "../../🧫️fixtures/🧫️history-edit-actions/🔣️.json";

import {
  DIALOG_CHOICE_ARG,
  HISTORY_EDIT_ACTION_IDS,
  HISTORY_EDIT_ARG_GENERATION,
  HISTORY_EDIT_ARG_MUTATION_ID,
  HISTORY_EDIT_ARG_NAME,
  HISTORY_EDIT_ARG_EDIT,
  HISTORY_EDIT_ARG_PATH,
  HISTORY_EDIT_ARG_STORE,
  HISTORY_EDIT_ARG_VALUE,
  HISTORY_EDIT_CHOICE_OVERWRITE,
  HISTORY_EDIT_FINALIZE_DIALOG_ID,
  HISTORY_EDIT_INPUT_INSERT,
  HISTORY_EDIT_INPUT_REMOVE,
} from "../../🟦️.ts";



describe("✏️ manifest history-edit verbs", () => {
  

  

  test("the TS ids and vocabulary equal the fixture", () => {
    expect(fixture.actions.map((row) => row.id)).toEqual([...HISTORY_EDIT_ACTION_IDS]);
    expect(fixture.constants).toEqual({
      mutationId: HISTORY_EDIT_ARG_MUTATION_ID,
      store: HISTORY_EDIT_ARG_STORE,
      path: HISTORY_EDIT_ARG_PATH,
      value: HISTORY_EDIT_ARG_VALUE,
      edit: HISTORY_EDIT_ARG_EDIT,
      insert: HISTORY_EDIT_INPUT_INSERT,
      remove: HISTORY_EDIT_INPUT_REMOVE,
      generation: HISTORY_EDIT_ARG_GENERATION,
      name: HISTORY_EDIT_ARG_NAME,
      choice: DIALOG_CHOICE_ARG,
      overwrite: HISTORY_EDIT_CHOICE_OVERWRITE,
      dialogId: HISTORY_EDIT_FINALIZE_DIALOG_ID,
    });
    expect(fixture.finalizeDialog.id).toBe(HISTORY_EDIT_FINALIZE_DIALOG_ID);
  });
});
