/** 🔗️ Node-graph edit rows (design §13.3): the closed row vocabulary every renderer dispatches as `nodeGraphEdit`
 * arguments, validated by Ajv against the framework schema. Renderer journal reading and flow add-node descriptors
 * retain their separate renderer fixtures; Rust guest decoders consume the same neutral row corpus.
 * @see 🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { describe, expect, it } from "vitest";
import { flowAddWidgetArgs, nodeGraphEditRows } from "../../🧱️elements/🕸️NodeGraph/🟦️.tsx";

const engineRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoRoot = resolve(engineRoot, "../../../../../..");
const toolMachineRoot = resolve(repoRoot, "🧰️framework/🔨️modules/🛠️tool-machine");
const fixture = JSON.parse(readFileSync(resolve(toolMachineRoot, "🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json"), "utf8")) as {
  readonly accepted: readonly { readonly id: string; readonly row: Record<string, unknown> }[];
  readonly refused: readonly { readonly id: string; readonly row: Record<string, unknown> }[];
};
const journalFixture = JSON.parse(readFileSync(resolve(engineRoot, "🧫️fixtures/🧫️node-graph-journal/🔣️.json"), "utf8")) as {
  readonly answers: readonly { readonly id: string; readonly json: string; readonly rows: readonly Record<string, unknown>[] }[];
};
const flowFixture = JSON.parse(readFileSync(resolve(engineRoot, "🧫️fixtures/🧫️flow-add-widget/🔣️.json"), "utf8")) as {
  readonly addWidget: readonly { readonly id: string; readonly descriptor: string; readonly x: number; readonly y: number; readonly args: Record<string, unknown> }[];
};
const schema = JSON.parse(readFileSync(resolve(toolMachineRoot, "🧬️schema/🔣️node-graph-edit-rows/🔣️.json"), "utf8"));
const flowSchema = JSON.parse(readFileSync(resolve(engineRoot, "🧬️schema/🔣️flow-add-widget/🔣️.json"), "utf8"));
const ajv = new Ajv2020({ allErrors: true, strict: true });
ajv.addSchema(schema);
const validateRows = ajv.getSchema(schema.$id)!;
const validateAddWidget = ajv.compile(flowSchema);

describe("node-graph edit rows", () => {
  it("accepts every fixture row and refuses every refused one", () => {
    for (const { id, row } of fixture.accepted) expect(validateRows({ operations: [row] }), `${id}: ${JSON.stringify(validateRows.errors)}`).toBe(true);
    for (const { id, row } of fixture.refused) expect(validateRows({ operations: [row] }), id).toBe(false);
  });

  it("reads a host journal answer as its rows and nothing else", () => {
    for (const { id, json, rows } of journalFixture.answers) expect(nodeGraphEditRows(json), id).toEqual(rows);
    const accepted = fixture.accepted.map(({ row }) => row);
    expect(nodeGraphEditRows(JSON.stringify({ operations: accepted }))).toEqual(accepted);
  });

  it("turns a catalogue descriptor into the addWidget record the wgpu drop dispatches", () => {
    for (const { id, descriptor, x, y, args } of flowFixture.addWidget) {
      expect(flowAddWidgetArgs(descriptor, x, y), id).toEqual(args);
      expect(validateAddWidget(args), `${id}: ${JSON.stringify(validateAddWidget.errors)}`).toBe(true);
    }
  });

  it("never publishes the whole fixture from either React node-graph surface", () => {
    const reactGraph = readFileSync(resolve(engineRoot, "🧱️elements/🕸️NodeGraph/🟦️.tsx"), "utf8");
    expect(reactGraph).not.toContain("setHostSnapshot");
    expect(reactGraph).not.toContain("commitFixture");
    expect(reactGraph).not.toContain("session.addWidget(");
    const wgpu = readFileSync(resolve(engineRoot, "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
    expect(wgpu).not.toContain("setHostSnapshot");
    expect(wgpu).toContain("flow::dag::write_dag_graph_edit_rows(edits, &mut BoundedGraphEditRows(builder))?");
    expect(wgpu).not.toContain('builder.string(Some("operation")');
    const dag = readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs"), "utf8");
    for (const operation of ["connect", "disconnect", "move", "setSlider", "insertPort"]) expect(dag).toContain(`sink.text("operation", "${operation}")?`);
  });
});
