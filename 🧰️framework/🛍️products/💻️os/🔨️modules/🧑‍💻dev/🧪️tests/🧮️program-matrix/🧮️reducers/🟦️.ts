/** 🧮️ Laws of the program matrix's pins over the language-agnostic fixture `../../../🧫️fixtures/🧮️program-matrix.json`:
 * the schema `🧬️schema/🔣️.json#/$defs/ProgramMatrixPinsV1` admits them (Ajv is the oracle); every rendered edit belongs
 * to a verb the pins drive and replaces that verb's staged rail arguments; resolution folds plugin → kind → subset; and
 * each rendered edit's control selects exactly the inline input the editor renders for it (jsdom is the oracle; the table
 * and JSON tree controls as measured live on serve 6540 with row-scoped DOM ids, the XML text node's input as its
 * `render_editor` builds it).
 * Revision-bound verbs (`set-cell`, `set-node`) are only drivable through their rendered editors (ticket 26/09/23 S18 §14c).
 * @see ../🟦️.ts */
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { JSDOM } from "jsdom";
import { describe, expect, it } from "vitest";
import schema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import { readMatrixPins, resolvePins, type MatrixProgram } from "../🟦️.ts";

const REPO = fileURLToPath(new URL("../../../../../../../..", import.meta.url));
const pins = readMatrixPins();

function contract(name: string): (value: unknown) => boolean {
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/${name}`);
  if (!validate) throw new Error(`no $defs/${name}`);
  return validate as (value: unknown) => boolean;
}

const program = (pluginId: string, appId: string): MatrixProgram => ({ pluginId, appId });
const STDIO_PROGRAMS = [
  program("stdio", "s.stdio.csv@rfc4180/*#editor"),
  program("stdio", "s.stdio.tsv@iana/*#editor"),
  program("stdio", "s.stdio.json@rfc8259/*#editor"),
  program("stdio", "s.stdio.json@rfc8259/i-json#editor"),
  program("stdio", "s.stdio.xml@1.0/*#editor"),
  program("stdio", "s.stdio.xml@1.0/valid#editor"),
];

describe("program matrix pins", () => {
  it("are admitted by their schema, which refuses a rendered edit without a control and an unknown section", () => {
    const valid = contract("ProgramMatrixPinsV1");
    expect(valid(pins)).toBe(true);
    expect(valid({ ...pins, kindEdits: { ...pins.kindEdits, "stdio/csv.set-cell": { value: "x" } } })).toBe(false);
    expect(valid({ ...pins, kindEdits: { ...pins.kindEdits, "stdio/csv.set-cell": { control: "input", value: "x", commit: "enter" } } })).toBe(false);
    expect(valid({ ...pins, verbArgs: {} })).toBe(false);
  });

  it("pin every rendered edit to a verb they drive, never beside staged rail arguments of that verb", () => {
    for (const editKey of Object.keys(pins.pluginEdits)) {
      const [plugin, verb] = [editKey.slice(0, editKey.indexOf(".")), editKey.slice(editKey.indexOf(".") + 1)];
      expect(pins.pluginVerbs[plugin] === verb || Object.entries(pins.kindVerbs).some(([kind, kindVerb]) => kind.startsWith(`${plugin}/`) && kindVerb === verb), editKey).toBe(true);
      expect(pins.pluginArgs[editKey], editKey).toBeUndefined();
    }
    for (const editKey of Object.keys(pins.kindEdits)) {
      const kind = editKey.slice(0, editKey.lastIndexOf("."));
      const verb = editKey.slice(editKey.lastIndexOf(".") + 1);
      const base = kind.split("/").slice(0, 2).join("/");
      expect(pins.kindVerbs[kind] ?? pins.kindVerbs[base] ?? pins.pluginVerbs[kind.split("/")[0]!], editKey).toBe(verb);
      expect(pins.kindArgs[editKey], editKey).toBeUndefined();
    }
  });

  it("resolve a subset's rendered edit from its own pin, else its kind's, else its plugin's", () => {
    const { verbs, args, edits } = resolvePins(REPO, pins, STDIO_PROGRAMS);
    expect([verbs["stdio/csv"], verbs["stdio/tsv"], verbs["stdio/json/i-json"], verbs["stdio/xml/valid"]]).toEqual(["set-cell", "set-cell", "set-node", "set-node"]);
    expect(edits["stdio/csv.set-cell"]).toEqual(pins.pluginEdits["stdio.set-cell"]);
    expect(edits["stdio/tsv.set-cell"]).toEqual(pins.pluginEdits["stdio.set-cell"]);
    expect(edits["stdio/json.set-node"]).toEqual(pins.kindEdits["stdio/json.set-node"]);
    expect(edits["stdio/json/i-json.set-node"]).toEqual(pins.kindEdits["stdio/json/i-json.set-node"]);
    expect(edits["stdio/xml/valid.set-node"]).toEqual(pins.kindEdits["stdio/xml.set-node"]);
    for (const key of ["stdio/csv.set-cell", "stdio/json.set-node", "stdio/xml/valid.set-node"]) expect(args[key], key).toBeUndefined();
  });

  it("select exactly the inline input each served editor renders for the pinned edit", () => {
    const body = new JSDOM(`<div data-slot="window-body">
      <table><tr><td><input id="spawned:stdio-2::framework.window.table/header-0/cell-0" aria-label="Header" value="name"></td></tr><tr><td><input id="spawned:stdio-2::framework.window.table/row-0/cell-0" aria-label="name" value="alpha"></td><td><input id="spawned:stdio-2::framework.window.table/row-0/cell-1" aria-label="note" value="plain"></td></tr><tr><td><input id="spawned:stdio-2::framework.window.table/row-1/cell-0" aria-label="name" value="Doe, John"></td></tr></table>
      <ul role="tree"><li role="treeitem"><input data-ui-node-key="edit" aria-label="{7}"><input data-ui-node-key="edit" aria-label='name: "semio"'><input data-ui-node-key="edit" aria-label='id: "semio.stdio.i-json.demo"'><input data-ui-node-key="edit" aria-label='members: [2]'><input data-ui-node-key="edit" aria-label='name: "alpha"'></li>
      <li role="treeitem"><input data-ui-node-key="edit" aria-label='text: "Tom &amp; Jerry"'></li></ul>
      <div data-slot="window-action-pane"><input id="action.set-cell.arg.value"></div></div>`).window.document;
    const hit = (control: string): string[] => [...body.querySelectorAll(`[data-slot="window-body"] ${control}`)].map((element) => element.id || element.getAttribute("aria-label") || "");
    expect(hit(pins.pluginEdits["stdio.set-cell"]!.control)).toEqual(["spawned:stdio-2::framework.window.table/row-0/cell-0"]);
    expect(hit(pins.kindEdits["stdio/json.set-node"]!.control)[0]).toBe('name: "semio"');
    expect(hit(pins.kindEdits["stdio/json/i-json.set-node"]!.control)).toEqual(['id: "semio.stdio.i-json.demo"']);
    expect(hit(pins.kindEdits["stdio/xml.set-node"]!.control)).toEqual(['text: "Tom & Jerry"']);
  });
});
