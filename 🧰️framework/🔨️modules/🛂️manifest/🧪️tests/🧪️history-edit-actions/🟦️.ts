/** ✏️ The reserved history-edit verbs over `🧫️fixtures/🧫️history-edit-actions/🔣️.json` — TypeScript twin of the Rust law
 * beside this file; oracle: Ajv (2020-12) validates the fixture and rejects hostile rows. */
import { describe, expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import fixture from "../../🧫️fixtures/🧫️history-edit-actions/🔣️.json";
import schema from "../../🧫️fixtures/🧫️history-edit-actions/🧬️schema/🔣️.json";
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

const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);

describe("✏️ manifest history-edit verbs", () => {
  test("the fixture validates against its schema", () => {
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("hostile fixtures are rejected", () => {
    expect(validate({ ...fixture, actions: fixture.actions.slice(1) })).toBe(false);
    expect(validate({ ...fixture, actions: fixture.actions.map((row, index) => (index === 0 ? { ...row, id: "editHistory" } : row)) })).toBe(false);
    expect(validate({ ...fixture, actions: fixture.actions.map((row, index) => (index === 4 ? { ...row, keys: "mod+enter" } : row)) })).toBe(false);
    expect(validate({ ...fixture, constants: { ...fixture.constants, overwrite: "replace" } })).toBe(false);
  });

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
