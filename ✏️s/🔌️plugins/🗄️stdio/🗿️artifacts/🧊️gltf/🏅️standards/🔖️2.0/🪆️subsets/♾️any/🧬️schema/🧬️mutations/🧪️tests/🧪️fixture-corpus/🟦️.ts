/** 🧫️ The committed glTF mutation corpus (`♾️any/🧫️fixtures/🧬️mutations/<entity>/<verb>/<case>/`) against the TypeScript
 * twins: every wire a case commits (`🦠️mutation`, both `📸️snapshot` sides, `🔺️diff`) decodes through its `parse<Export>()`
 * and re-encodes to the committed bytes; the strict third-party Ajv oracle (`semioSchemaAjvV1`) reaches the same verdict on
 * every committed wire and on every broken one, which both must refuse. The Rust half of this corpus law is `./🦀️.rs`.
 * @see ./🦀️.rs
 * @see https://ajv.js.org/ */
import { describe, expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import { GltfWireRefusal, parseGltfSnapshot, type GltfWireReader } from "../../../📸️snapshot/🟦️.ts";
import { parseGltfDiff } from "../../../🔺️diff/🟦️.ts";
import { parseGltfMutation } from "../../🟦️.ts";

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };

const mutations = join(import.meta.dir, "../..");
const corpus = join(import.meta.dir, "../../../../🧫️fixtures/🧬️mutations");
const document = (path: string): { $id: string; oneOf?: { properties: { mutation: { const: string } } }[] } => JSON.parse(readFileSync(path, "utf8"));
const encode = (value: unknown): string => `${JSON.stringify(value, null, 2)}\n`;

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
      test(`${name} decodes, re-encodes byte-equal, and agrees with Ajv`, () => {
        const text = readFileSync(join(directory, wire.file), "utf8");
        const committed = JSON.parse(text) as Json;
        expect(wire.admits(committed)).toBe(true);
        expect(encode(wire.parse(committed))).toBe(text);
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
