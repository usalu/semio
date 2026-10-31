import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import draft07 from "ajv/dist/refs/json-schema-draft-07.json";
import { expect, it } from "vitest";

type Corpus = {
  referenceCases: { id: string; root: string; documents: string[]; compiles: boolean }[];
  repositoryRoots: string[];
  draft07Keywords: string[];
  patternCases: { id: string; pattern: string; compiles: boolean; matches?: { units: number[]; expected: boolean }[] }[];
  opacityCases: { id: string; path: string; opaque: boolean }[];
  authoring: { schema: string; documents: string[]; cases: { id: string; definition: string; value: unknown; valid: boolean }[] };
};

const here = dirname(fileURLToPath(import.meta.url));
const corpus = JSON.parse(readFileSync(join(here, "📥️inputs.json"), "utf8")) as Corpus;
let root = here;
while (!existsSync(join(root, ".🧬semio"))) {
  const parent = dirname(root);
  if (parent === root) throw Error("Repository root required");
  root = parent;
}
const read = (path: string) => JSON.parse(readFileSync(join(root, path), "utf8"));
const taxonomy = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
const ajvConstructor = (Ajv as unknown as { default?: typeof Ajv }).default ?? Ajv;

for (const row of corpus.referenceCases) it("agrees with Ajv on reference inputs: " + row.id, () => {
  const ajv = new ajvConstructor({ strict: false });
  let compiled = true;
  try {
    for (const body of row.documents) ajv.addSchema(JSON.parse(body));
    ajv.compile(JSON.parse(row.root));
  } catch {
    compiled = false;
  }
  expect(compiled).toBe(row.compiles);
});

it("covers every actual declared repository source area", () => {
  expect(Object.keys(taxonomy.areaLayers).sort()).toEqual([...corpus.repositoryRoots].sort());
});

it("classifies keywords using the published Draft07 vocabulary", () => {
  expect(Object.keys(draft07.properties).sort()).toEqual([...corpus.draft07Keywords].sort());
});

for (const row of corpus.patternCases) it("agrees with ECMAScript pattern behavior: " + row.id, () => {
  const ajv = new ajvConstructor({ strict: false });
  let validator: ReturnType<typeof ajv.compile> | undefined;
  try {
    validator = ajv.compile({ pattern: row.pattern });
  } catch {
    validator = undefined;
  }
  expect(validator !== undefined).toBe(row.compiles);
  if (validator) for (const entry of row.matches ?? []) expect(validator(String.fromCharCode(...entry.units))).toBe(entry.expected);
});

const preservation = read("🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json").definitions.RelativePreservationProtocol.const;
const retirement = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json").definitions.Policy.const;
const escapePattern = (value: string) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const opaquePaths = (Object.values(taxonomy.pathExclusions) as { mode: string; path: string }[]).filter(row => row.mode === "opaque").map(row => escapePattern(row.path.replace(/\/$/, "")));
const prefixes = [preservation.capsulePrefix, retirement.markerPrefix].map(escapePattern);
const opaque = new ajvConstructor({ strict: false }).compile({
  type: "string",
  anyOf: [
    { pattern: "^(?:" + opaquePaths.join("|") + ")(?:/|$)" },
    { pattern: "(?:^|/)(?:" + prefixes.join("|") + ")[a-f0-9]{" + preservation.capsuleDigestHex + "}(?:/|$)" },
  ],
});
for (const row of corpus.opacityCases) it("agrees on Source opacity: " + row.id, () => {
  expect(opaque(row.path)).toBe(row.opaque);
});

for (const row of corpus.authoring.cases) it("agrees on actual authored BIM structure: " + row.id, () => {
  const facet = read(corpus.authoring.schema);
  expect(facet.$defs?.[row.definition]).toBeDefined();
  const ajv = new ajvConstructor({ strict: false });
  for (const path of corpus.authoring.documents) ajv.addSchema(read(path));
  const validate = ajv.compile({ $defs: facet.$defs, $ref: "#/$defs/" + row.definition });
  expect(validate(row.value)).toBe(row.valid);
});
