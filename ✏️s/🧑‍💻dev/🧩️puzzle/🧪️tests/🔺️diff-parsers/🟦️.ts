/** 🔺️ Executes the three Puzzle diff schema modules against their own `🔣️.json`.
 *
 * Every `parsePuzzle<N>d…Modification` delegates to a `parsePuzzle<N>d…Patch`, and those four
 * functions were declared in the schema but never written, so the modules threw `ReferenceError`
 * on first call and had therefore never been executed. This pins both halves: the owned parser and
 * Ajv reading the sibling schema must admit and refuse exactly the same documents.
 * @see https://ajv.js.org/json-schema.html */
import { artifactReferenceSchema } from "@semio-tech/schema-ts/artifact-reference";
import { existsSync, readFileSync } from "node:fs";
import Ajv, { type ValidateFunction } from "ajv";
import { describe, expect, it } from "vitest";
import * as puzzle2dDiff from "../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
import * as puzzle3dDiff from "../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
import * as puzzle5dDiff from "../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";

/** 🚪️ The framework-owned `ArtifactRef` every composed-child handle `$ref`s. */
const frameworkIoSchema = "../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json";

type DiffModule = Readonly<Record<string, unknown>>;
type ModificationCase = {
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
const fastener5d = { source: "part-a", target: "part-b" };
const volume5d = { hidden: false, locked: false };

const cases: readonly ModificationCase[] = [
  {
    dimension: "puzzle2d",
    module: puzzle2dDiff,
    schemaUrl: new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle2dNodeModification", def: "Puzzle2dNodeModification", patch: node2d },
      { entry: "parsePuzzle2dEdgeModification", def: "Puzzle2dEdgeModification", patch: edge2d },
      { entry: "parsePuzzle2dTargetRegionModification", def: "Puzzle2dTargetRegionModification", patch: region2d },
    ],
  },
  {
    dimension: "puzzle3d",
    module: puzzle3dDiff,
    schemaUrl: new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle3dObjectModification", def: "Puzzle3dObjectModification", patch: object3d },
      { entry: "parsePuzzle3dAttractionModification", def: "Puzzle3dAttractionModification", patch: attraction3d },
      { entry: "parsePuzzle3dTargetVolumeModification", def: "Puzzle3dTargetVolumeModification", patch: volume3d },
      { entry: "parsePuzzle3dReferenceModification", def: "Puzzle3dReferenceModification", patch: reference3d },
    ],
  },
  {
    dimension: "puzzle5d",
    module: puzzle5dDiff,
    schemaUrl: new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json", import.meta.url),
    entries: [
      { entry: "parsePuzzle5dPartModification", def: "Puzzle5dPartModification", patch: part5d },
      { entry: "parsePuzzle5dFastenerModification", def: "Puzzle5dFastenerModification", patch: fastener5d },
      { entry: "parsePuzzle5dTargetVolumeModification", def: "Puzzle5dTargetVolumeModification", patch: volume5d },
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

describe("puzzle diff modification parsers", () => {
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
  const fixture = JSON.parse(readFileSync(new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json", import.meta.url), "utf8")) as {cases: {name:string;words:string[]}[]};
  const schemaUrl=cases.find((row)=>row.dimension==="puzzle3d")!.schemaUrl;
  for (const row of fixture.cases) {
    const origin=row.words.map((bits)=>({bits:BigInt(bits)}));
    const input={id:"object-a",patch:{origin}};
    const actual=parserOf(puzzle3dDiff,"parsePuzzle3dObjectModification")(input);
    const wire=JSON.parse(JSON.stringify(actual,(_,value)=>typeof value==="bigint"?value.toString(16).padStart(16,"0"):value));
    expect(oracleOf(schemaUrl,"Puzzle3dObjectModification")(wire)).toBe(true);
    expect(wire.patch.origin.map((word:{bits:string})=>BigInt("0x"+word.bits).toString())).toEqual(row.words);
    expect(()=>parserOf(puzzle3dDiff,"parsePuzzle3dObjectModification")({id:"object-a",patch:{origin:[1,2,3]}})).toThrow();
  }
});

/** 🔢️ The same exact scalar words cross the Puzzle5d JSON IO boundary and retain native identity. */
it("puzzle5d: typed sparse field words match the neutral corpus and Ajv transport oracle", () => {
  const fixture = JSON.parse(readFileSync(new URL("../../../../🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json", import.meta.url), "utf8")) as {cases: {name:string;words:string[]}[]};
  const schemaUrl=cases.find((row)=>row.dimension==="puzzle5d")!.schemaUrl;
  for (const row of fixture.cases) {
    const origin=row.words.map((bits)=>({bits:BigInt(bits)}));
    const input={id:"volume-a",patch:{origin}};
    const actual=parserOf(puzzle5dDiff,"parsePuzzle5dTargetVolumeModification")(input);
    const wire=JSON.parse(JSON.stringify(actual,(_,value)=>typeof value==="bigint"?value.toString(16).padStart(16,"0"):value));
    expect(oracleOf(schemaUrl,"Puzzle5dTargetVolumeModification")(wire)).toBe(true);
    expect(wire.patch.origin.map((word:{bits:string})=>BigInt("0x"+word.bits).toString())).toEqual(row.words);
    expect(()=>parserOf(puzzle5dDiff,"parsePuzzle5dTargetVolumeModification")({id:"volume-a",patch:{origin:[1,2,3]}})).toThrow();
    expect(()=>parserOf(puzzle5dDiff,"parsePuzzle5dFastenerModification")({id:"fastener-a",patch:{gap:0.25}})).toThrow();
  }
});

/** 🔢️ Insertion, removal and relocation indices are non-negative integers, exactly as the sibling schema says. */
it("puzzle3d and puzzle5d: collection indices refuse what the schema refuses", () => {
  const sites = [
    { dimension: "puzzle3d", module: puzzle3dDiff, entry: "parsePuzzle3dObjectRemoval", def: "Puzzle3dObjectRemoval" },
    { dimension: "puzzle5d", module: puzzle5dDiff, entry: "parsePuzzle5dPartRemoval", def: "Puzzle5dPartRemoval" },
  ];
  for (const { dimension, module, entry, def } of sites) {
    const schemaUrl = cases.find((row) => row.dimension === dimension)!.schemaUrl;
    const oracle = oracleOf(schemaUrl, def);
    for (const index of [0, 3]) {
      expect(oracle({ id: "a", index })).toBe(true);
      expect(parserOf(module, entry)({ id: "a", index })).toEqual({ id: "a", index });
    }
    for (const index of [-1, 1.5, "0"]) {
      expect(oracle({ id: "a", index })).toBe(false);
      expect(() => parserOf(module, entry)({ id: "a", index })).toThrow();
    }
  }
});
