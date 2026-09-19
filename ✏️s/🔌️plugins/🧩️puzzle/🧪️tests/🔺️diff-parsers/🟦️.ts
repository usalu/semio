/** 🔺️ Executes the three Puzzle diff schema modules against their own `🔣️.json`.
 *
 * Every `parsePuzzle<N>d…PatchEntry` delegates to a `parsePuzzle<N>d…Patch`, and those four
 * functions were declared in the schema but never written, so the modules threw `ReferenceError`
 * on first call and had therefore never been executed. This pins both halves: the owned parser and
 * Ajv reading the sibling schema must admit and refuse exactly the same documents.
 * @see https://ajv.js.org/json-schema.html */
import { existsSync, readFileSync } from "node:fs";
import Ajv, { type ValidateFunction } from "ajv";
import { describe, expect, it } from "vitest";
import * as puzzle2dDiff from "../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
import * as puzzle3dDiff from "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
import * as puzzle5dDiff from "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";

/** 🚪️ The framework-owned `ArtifactRef` every composed-child handle `$ref`s. */
const frameworkIoSchema = "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json";

type DiffModule = Readonly<Record<string, unknown>>;
type PatchEntryCase = {
  readonly dimension: string;
  readonly module: DiffModule;
  readonly schemaUrl: URL;
  readonly entries: readonly { readonly entry: string; readonly def: string; readonly replacement: Record<string, unknown> }[];
};

const node2d = { id: "node-a", x: 1, y: 2, anchor: "fixed", handles: [{ id: "handle-a", angle: 0.5 }] };
const edge2d = { id: "edge-a", source: "node-a", target: "node-b", gap: 0, shift: 0, rise: 0, rotation: 0, turn: 0, tilt: 0, x: 3, y: 4 };
const region2d = { id: "region-a", x: 0, y: 0, width: 10, height: 20, hidden: false, locked: false };
const object3d = { id: "object-a", origin: [1, 2, 3], vortices: [{ id: "vortex-a", position: [0, 0, 1] }] };
const attraction3d = { attracting: "object-a", attracted: "object-b", gap: 0.25 };
const volume3d = { id: "volume-a" };
const reference3d = { id: "reference-a" };
const part5d = { id: "part-a", partKind: "beam" };
const fastener5d = { id: "fastener-a", source: "part-a", target: "part-b" };
const volume5d = { id: "volume-a", origin: [0, 0, 0], hidden: false, locked: false };

const cases: readonly PatchEntryCase[] = [
  {
    dimension: "puzzle2d",
    module: puzzle2dDiff,
    schemaUrl: new URL("../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle2dNodePatchEntry", def: "Puzzle2dNodePatchEntry", replacement: node2d },
      { entry: "parsePuzzle2dEdgePatchEntry", def: "Puzzle2dEdgePatchEntry", replacement: edge2d },
      { entry: "parsePuzzle2dTargetRegionPatchEntry", def: "Puzzle2dTargetRegionPatchEntry", replacement: region2d },
    ],
  },
  {
    dimension: "puzzle3d",
    module: puzzle3dDiff,
    schemaUrl: new URL("../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle3dObjectPatchEntry", def: "Puzzle3dObjectPatchEntry", replacement: object3d },
      { entry: "parsePuzzle3dAttractionPatchEntry", def: "Puzzle3dAttractionPatchEntry", replacement: attraction3d },
      { entry: "parsePuzzle3dTargetVolumePatchEntry", def: "Puzzle3dTargetVolumePatchEntry", replacement: volume3d },
      { entry: "parsePuzzle3dReferencePatchEntry", def: "Puzzle3dReferencePatchEntry", replacement: reference3d },
    ],
  },
  {
    dimension: "puzzle5d",
    module: puzzle5dDiff,
    schemaUrl: new URL("../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle5dPartPatchEntry", def: "Puzzle5dPartPatchEntry", replacement: part5d },
      { entry: "parsePuzzle5dFastenerPatchEntry", def: "Puzzle5dFastenerPatchEntry", replacement: fastener5d },
      { entry: "parsePuzzle5dTargetVolumePatchEntry", def: "Puzzle5dTargetVolumePatchEntry", replacement: volume5d },
    ],
  },
];

/** 🚪️ Resolves one exported parser, refusing a name the module never emitted. */
function parserOf(module: DiffModule, name: string): (value: unknown, at?: string) => unknown {
  const parser = module[name];
  if (typeof parser !== "function") throw new Error(`${name} is not exported`);
  return parser as (value: unknown, at?: string) => unknown;
}

/** 🔬️ Compiles the sibling schema's `$defs` entry as the independent oracle. The diff document
 * `$ref`s its artifact sibling by absolute `$id`, and puzzle5d's artifact `$ref`s the snapshot in
 * turn, so every sibling that exists is registered before compilation. */
function oracleOf(schemaUrl: URL, def: string): ValidateFunction {
  const document = JSON.parse(readFileSync(schemaUrl, "utf8")) as { $id: string };
  const ajv = new Ajv({ strict: false, allErrors: true });
  for (const sibling of [frameworkIoSchema, "../📸️snapshot/🔣️.json", "../🔣️.json"]) {
    const path = new URL(sibling, schemaUrl);
    if (existsSync(path)) ajv.addSchema(JSON.parse(readFileSync(path, "utf8")) as object);
  }
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${def}`);
  if (validate === undefined) throw new Error(`${def} is not a definition of ${document.$id}`);
  return validate;
}

describe("puzzle diff patch-entry parsers", () => {
  for (const { dimension, module, schemaUrl, entries } of cases) {
    for (const { entry, def, replacement } of entries) {
      it(`${dimension}: ${entry} admits a replacement patch its schema admits`, () => {
        const document = { id: String(replacement["id"] ?? "entry-a"), patch: { replacement } };
        expect(oracleOf(schemaUrl, def)(document)).toBe(true);
        expect(JSON.parse(JSON.stringify(parserOf(module, entry)(document)))).toEqual(document);
      });

      it(`${dimension}: ${entry} refuses what its schema refuses`, () => {
        const oracle = oracleOf(schemaUrl, def);
        for (const hostile of [{ id: 1, patch: { replacement } }, { id: "entry-a", patch: { replacement: [] } }, { id: "entry-a" }]) {
          expect(oracle(hostile)).toBe(false);
          expect(() => parserOf(module, entry)(hostile)).toThrow();
        }
      });
    }

    it(`${dimension}: the empty patch carries no replacement`, () => {
      const first = entries[0]!;
      const document = { id: "entry-a", patch: {} };
      expect(oracleOf(schemaUrl, first.def)(document)).toBe(true);
      expect(JSON.parse(JSON.stringify(parserOf(module, first.entry)(document)))).toEqual(document);
    });
  }
});
