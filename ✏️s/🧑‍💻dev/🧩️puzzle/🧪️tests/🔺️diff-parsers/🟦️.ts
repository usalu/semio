/** 🔺️ Executes the three Puzzle diff schema modules against their own `🔣️.json`.
 *
 * Every `parsePuzzle<N>d…PatchEntry` delegates to a `parsePuzzle<N>d…Patch`, and those four
 * functions were declared in the schema but never written, so the modules threw `ReferenceError`
 * on first call and had therefore never been executed. This pins both halves: the owned parser and
 * Ajv reading the sibling schema must admit and refuse exactly the same documents.
 * @see https://ajv.js.org/json-schema.html */
import artifactReferenceSchema from "../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import { existsSync, readFileSync } from "node:fs";
import Ajv, { type ValidateFunction } from "ajv";
import { describe, expect, it } from "vitest";
import * as puzzle2dDiff from "../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
import * as puzzle3dDiff from "../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
import * as puzzle5dDiff from "../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";

/** 🚪️ The framework-owned `ArtifactRef` every composed-child handle `$ref`s. */
const frameworkIoSchema = "../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json";

type DiffModule = Readonly<Record<string, unknown>>;
type PatchEntryCase = {
  readonly dimension: string;
  readonly module: DiffModule;
  readonly schemaUrl: URL;
  readonly entries: readonly { readonly entry: string; readonly def: string; readonly patch: Record<string, unknown> }[];
};

const node2d = { x: 1, y: 2, anchor: "fixed" };
const edge2d = { source: "node-a", target: "node-b", gap: 0, x: 3, y: 4 };
const region2d = { x: 0, y: 0, width: 10, height: 20, hidden: false, locked: false };
const object3d = { label: "object-a", hidden: false };
const attraction3d = { attracting: "object-a", attracted: "object-b" };
const volume3d = { hidden: false, locked: true };
const reference3d = { source: {url:"memory://reference-a",mediaKind:null} };
const part5d = { partKind: "beam" };
const fastener5d = { gap: 0.25 };
const volume5d = { origin: [0, 0, 0], hidden: false, locked: false };

const cases: readonly PatchEntryCase[] = [
  {
    dimension: "puzzle2d",
    module: puzzle2dDiff,
    schemaUrl: new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle2dNodePatchEntry", def: "Puzzle2dNodePatchEntry", patch: node2d },
      { entry: "parsePuzzle2dEdgePatchEntry", def: "Puzzle2dEdgePatchEntry", patch: edge2d },
      { entry: "parsePuzzle2dTargetRegionPatchEntry", def: "Puzzle2dTargetRegionPatchEntry", patch: region2d },
    ],
  },
  {
    dimension: "puzzle3d",
    module: puzzle3dDiff,
    schemaUrl: new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle3dObjectPatchEntry", def: "Puzzle3dObjectPatchEntry", patch: object3d },
      { entry: "parsePuzzle3dAttractionPatchEntry", def: "Puzzle3dAttractionPatchEntry", patch: attraction3d },
      { entry: "parsePuzzle3dTargetVolumePatchEntry", def: "Puzzle3dTargetVolumePatchEntry", patch: volume3d },
      { entry: "parsePuzzle3dReferencePatchEntry", def: "Puzzle3dReferencePatchEntry", patch: reference3d },
    ],
  },
  {
    dimension: "puzzle5d",
    module: puzzle5dDiff,
    schemaUrl: new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle5dPartPatchEntry", def: "Puzzle5dPartPatchEntry", patch: part5d },
      { entry: "parsePuzzle5dFastenerPatchEntry", def: "Puzzle5dFastenerPatchEntry", patch: fastener5d },
      { entry: "parsePuzzle5dTargetVolumePatchEntry", def: "Puzzle5dTargetVolumePatchEntry", patch: volume5d },
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
  const ajv = new Ajv({ strict: false, allErrors: true }).addSchema(artifactReferenceSchema);
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
    for (const { entry, def, patch } of entries) {
      it(`${dimension}: ${entry} admits a typed field patch its schema admits`, () => {
        const document = { id: "entry-a", patch };
        expect(oracleOf(schemaUrl, def)(document)).toBe(true);
        expect(JSON.parse(JSON.stringify(parserOf(module, entry)(document)))).toEqual(document);
      });

      it(`${dimension}: ${entry} refuses what its schema refuses`, () => {
        const oracle = oracleOf(schemaUrl, def);
        for (const hostile of [{ id: 1, patch }, { id: "entry-a", patch: { replacement: [] } }, { id: "entry-a" }]) {
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

/** 🔢️ Shared exact scalar words cross the test's JSON IO boundary and retain native identity. */
it("puzzle3d: typed sparse field words match the neutral corpus and Ajv transport oracle", () => {
  const fixture = JSON.parse(readFileSync(new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧫️fixtures/🔢️native-fields/🔣️.json", import.meta.url), "utf8")) as {cases: {name:string;words:string[]}[]};
  const schemaUrl=cases.find((row)=>row.dimension==="puzzle3d")!.schemaUrl;
  for (const row of fixture.cases) {
    const origin=row.words.map((bits)=>({bits:BigInt(bits)}));
    const input={id:"object-a",patch:{origin}};
    const actual=parserOf(puzzle3dDiff,"parsePuzzle3dObjectPatchEntry")(input);
    const wire=JSON.parse(JSON.stringify(actual,(_,value)=>typeof value==="bigint"?value.toString(16).padStart(16,"0"):value));
    expect(oracleOf(schemaUrl,"Puzzle3dObjectPatchEntry")(wire)).toBe(true);
    expect(wire.patch.origin.map((word:{bits:string})=>BigInt("0x"+word.bits).toString())).toEqual(row.words);
    expect(()=>parserOf(puzzle3dDiff,"parsePuzzle3dObjectPatchEntry")({id:"object-a",patch:{origin:[1,2,3]}})).toThrow();
  }
});
