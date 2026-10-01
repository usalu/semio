/** 🧫️ Canonical GLTF mutation/source snapshots retain every finite native wire field.
 * The strict independent Ajv oracle validates all committed and malformed JSON fixtures;
 * scalar assertions compare exact bigint, binary64 and ordered local JSON/pair values.
 * @see ./🦀️.rs */
import { describe, expect, test } from "bun:test";
import {binary64Value,type Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import { GltfWireRefusal, parseGltfSnapshot, type GltfWireReader } from "../../../📸️snapshot/🟦️.ts";
import { parseGltfDiff } from "../../../🔺️diff/🟦️.ts";
import { parseGltfMutation } from "../../🟦️.ts";

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

const mutations = join(import.meta.dir, "../..");
const corpus = join(import.meta.dir, "../../../../🧫️fixtures/🧬️mutations");
const document = (path: string): { $id: string; oneOf?: { properties: { mutation: { const: string } } }[] } => JSON.parse(readFileSync(path, "utf8"));
const assertWireFields=(owned:unknown,wire:Json):void=>{
 if(owned!==null&&typeof owned==="object"&&!Array.isArray(owned)&&"kind"in owned){const value=owned as {kind:string;value?:unknown;values?:unknown[];members?:[string,unknown][]};switch(value.kind){case"null":expect(wire).toBeNull();return;case"boolean":case"string":assertWireFields(value.value,wire);return;case"number":assertWireFields(value.value,wire);return;case"array":assertWireFields(value.values,wire);return;case"object":assertWireFields(value.members,wire);return}}
 if(typeof wire==="number"){if(typeof owned==="bigint"){expect(owned).toBe(BigInt(wire));return}if(owned!==null&&typeof owned==="object"&&"bits"in owned){expect(binary64Value(owned as Binary64)).toBe(wire);return}expect(owned).toBe(wire);return}
 if(wire===null||typeof wire==="string"||typeof wire==="boolean"){expect(owned).toBe(wire);return}
 if(Array.isArray(wire)){expect(Array.isArray(owned)).toBe(true);const values=owned as unknown[];expect(values).toHaveLength(wire.length);wire.forEach((value,index)=>assertWireFields(values[index],value));return}
 if(Array.isArray(owned)){expect(owned.map(value=>(value as [string,unknown])[0])).toEqual(Object.keys(wire));for(const[name,value]of owned as [string,unknown][])assertWireFields(value,wire[name]!);return}
 expect(owned!==null&&typeof owned==="object").toBe(true);for(const[key,value]of Object.entries(wire))assertWireFields((owned as Record<string,unknown>)[key],value);
};

const cases = (directory: string): string[] =>
  existsSync(join(directory, "🦠️mutation/🔣️.json"))
    ? [directory]
    : readdirSync(directory, { withFileTypes: true })
        .filter((entry) => entry.isDirectory())
        .flatMap((entry) => cases(join(directory, entry.name)))
        .sort();

const ajv = semioSchemaAjvV1({ strict: true, allErrors: true });
const snapshotSchema = document(join(mutations, "../📸️snapshot/🔣️.json"));
const diffSchema = document(join(mutations, "../🔺️diff/🔣️.json"));
const aggregateSchema = document(join(mutations, "🔣️.json"));
ajv.addSchema(snapshotSchema).addSchema(diffSchema);
for (const domain of readdirSync(mutations, { withFileTypes: true }).filter((entry) => entry.isDirectory()))
  for (const verb of readdirSync(join(mutations, domain.name), { withFileTypes: true }).filter((entry) => entry.isDirectory()))
    if (existsSync(join(mutations, domain.name, verb.name, "🧬️schema/🔣️.json"))) ajv.addSchema(document(join(mutations, domain.name, verb.name, "🧬️schema/🔣️.json")));
const oracle = (schema: { $id: string }) => (instance: unknown): boolean => ajv.getSchema(schema.$id)!(instance) === true;
ajv.addSchema(aggregateSchema);

/** 🧷️ A member no contract declares, spliced into the closed object at `path` of a committed wire. */
const breakAt = (wire: Json, path: readonly string[]): Json => {
  const copy = structuredClone(wire) as { [key: string]: Json };
  const target = path.reduce<{ [key: string]: Json }>((node, key) => node[key] as { [key: string]: Json }, copy);
  target["x-undeclared"] = true;
  return copy;
};

interface Wire {
  readonly file: string;
  readonly parse: GltfWireReader<unknown>;
  readonly admits: (instance: unknown) => boolean;
  readonly closed: (wire: Json) => readonly (readonly string[])[];
}

const wires: readonly Wire[] = [
  {
    file: "🦠️mutation/🔣️.json",
    parse: parseGltfMutation,
    admits: oracle(aggregateSchema),
    closed: (wire) => {
      const payload = (wire as { payload: { phase?: string } }).payload;
      return [[], ["payload"], ...(payload.phase === undefined ? [] : [["payload", "value"]])];
    },
  },
  { file: "📸️snapshot/⬅️before/🔣️.json", parse: parseGltfSnapshot, admits: oracle(snapshotSchema), closed: () => [[], ["document"], ["document", "asset"]] },
  { file: "📸️snapshot/➡️after/🔣️.json", parse: parseGltfSnapshot, admits: oracle(snapshotSchema), closed: () => [[], ["document"], ["document", "asset"]] },
  { file: "🔺️diff/🔣️.json", parse: parseGltfDiff, admits: oracle(diffSchema), closed: () => [[]] },
];

describe("glTF mutation corpus through the TypeScript twins", () => {
  const found = cases(corpus);

  test("every GltfMutation branch of the aggregate schema owns a committed case", () => {
    const committed = new Set(found.map((directory) => (JSON.parse(readFileSync(join(directory, "🦠️mutation/🔣️.json"), "utf8")) as { mutation: string }).mutation));
    expect([...committed].sort()).toEqual(aggregateSchema.oneOf!.map((branch) => branch.properties.mutation.const).sort());
  });

  for (const directory of found)
    for (const wire of wires) {
      const name = `${relative(corpus, directory)} ${wire.file}`;
      if (!existsSync(join(directory, wire.file))) {
        test(`${name} is absent because the case is rejected`, () => {
          expect(existsSync(join(directory, wire.file.replace("🔣️.json", "🚫️.absent")))).toBe(true);
          expect((JSON.parse(readFileSync(join(directory, "🎯️outcome/🔣️.json"), "utf8")) as { status: string }).status).toBe("rejected");
        });
        continue;
      }
      test(`${name} decodes every native wire field into canonical owned values and agrees with Ajv`, () => {
        const text = readFileSync(join(directory, wire.file), "utf8");
        const committed = JSON.parse(text) as Json;
        expect(wire.admits(committed)).toBe(true);
        assertWireFields(wire.parse(committed),committed);
        for (const path of wire.closed(committed)) {
          const broken = breakAt(committed, path);
          expect(wire.admits(broken)).toBe(false);
          expect(() => wire.parse(broken)).toThrow(GltfWireRefusal);
        }
      });
    }

  test("an unknown mutation tag is refused by the twin and by Ajv", () => {
    const wire = { mutation: "reshapeEverything", payload: { phase: "apply", value: {} } };
    expect(oracle(aggregateSchema)(wire)).toBe(false);
    expect(() => parseGltfMutation(wire)).toThrow(GltfWireRefusal);
  });
});
