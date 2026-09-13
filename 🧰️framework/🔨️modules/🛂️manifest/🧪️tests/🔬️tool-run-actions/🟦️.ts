/** ⏯️ Tool run declaration and action injection over `🧫️fixtures/⏯️tool-run-actions.json`; oracle: ajv validates the fixture and hostile mutations against the manifest schema, which reaches the `⏯️tool-run` schema by `$ref`. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import toolRunSchema from "../../../⏯️tool-run/🧬️schema/🔣️.json";
import { TOOL_RUN_ACTIONS, TOOL_RUN_LABELS } from "../../../⏯️tool-run/🟦️.ts";
import fixture from "../../🧫️fixtures/⏯️tool-run-actions.json";
import schema from "../../🧬️schema/🔣️.json";
import { appDeclaresToolRun } from "../../🟦️.ts";

const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(toolRunSchema).addSchema(schema);
const validator = (name: string) => ajv.getSchema(`${schema.$id}#/$defs/${name}`)!;
const toolWithRun = fixture.cases[2]!.tools[1]!;

describe("⏯️ manifest tool run declaration", () => {
  test("the fixture validates against the manifest schema", () => {
    const validate = validator("ToolRunActionsFixture");
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("hostile run declarations are rejected through the tool-run $ref", () => {
    const tool = validator("ToolDefinition");
    const utility = validator("UtilityDefinition");
    expect(tool(toolWithRun)).toBe(true);
    expect(utility(fixture.cases[3]!.utilities[0])).toBe(true);
    expect(tool({ ...toolWithRun, run: { ...toolWithRun.run, stages: [] } })).toBe(false);
    expect(tool({ ...toolWithRun, run: { ...toolWithRun.run, trace: "voxel" } })).toBe(false);
    expect(tool({ ...toolWithRun, run: { ...toolWithRun.run, reasons: [{ ...toolWithRun.run!.reasons[0], code: 65280 }] } })).toBe(false);
    expect(tool({ ...toolWithRun, run: { ...toolWithRun.run, extra: true } })).toBe(false);
    expect(tool({ ...toolWithRun, runs: toolWithRun.run })).toBe(false);
    expect(validator("ToolRunActionsFixture")({ ...fixture, actions: fixture.actions.map((row, index) => (index === 0 ? { ...row, id: "toolRunCancel" } : row)) })).toBe(false);
    expect(validator("ToolRunActionsFixture")({ ...fixture, actions: fixture.actions.map((row, index) => (index === 6 ? { ...row, keys: "escape" } : row)) })).toBe(false);
  });

  test("actions are injected exactly when a tool or utility declares run", () => {
    for (const testCase of fixture.cases) expect(appDeclaresToolRun(testCase as never), testCase.name).toBe(testCase.injected);
  });

  test("fixture rows equal the tool-run action contract ids, args, chords and EN/DE labels", () => {
    expect(fixture.actions.map((row) => row.id)).toEqual(TOOL_RUN_ACTIONS.map((action) => action.id));
    for (const [index, action] of TOOL_RUN_ACTIONS.entries()) {
      const row = fixture.actions[index]!;
      expect(row.args.map((arg) => ({ name: arg.id, required: arg.required })), row.id).toEqual(action.args.map((arg) => ({ name: arg.name, required: arg.required })));
      expect(row.label, row.id).toEqual(TOOL_RUN_LABELS[action.label]);
      expect("keys" in row ? row.keys : row.panelChord, row.id).toBe(action.chord);
    }
    expect(fixture.reservedChords.filter((chord) => fixture.actions.some((row) => "keys" in row && row.keys === chord))).toEqual([]);
  });
});
