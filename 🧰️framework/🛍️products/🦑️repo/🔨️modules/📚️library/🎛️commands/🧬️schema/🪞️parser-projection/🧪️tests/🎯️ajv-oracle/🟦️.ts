/** 🎯️ Ajv oracle for the schema parser projection: the generated `parse<Export>` functions are projected from a language-agnostic corpus of schemas, compiled, type-checked, and compared instance by instance with the independent Ajv validator. A parser refuses an unknown property exactly where its schema says `additionalProperties: false`, so it returns the verdict of Ajv on every instance of the corpus; the one disagreement the corpus pins is the integer and tuple rule that is stricter than the schema.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🧪️tests/🧫️fixtures/🎯️ajv-oracle/🔣️.json
 */
import { afterAll, expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { auditLeaves, checkParsers, projectParsers, type ProjectionOptions } from "../../🟦️.ts";
import { closedHelper, helpersBlock } from "../../🚪️guard-helpers/🟦️.ts";

type Parser = (value: unknown) => unknown;
type Probe = { readonly document: string; readonly export: string; readonly valid?: unknown[]; readonly invalid?: unknown[]; readonly stricter?: unknown[]; readonly tolerated?: unknown[]; readonly defaulted?: { readonly input: unknown; readonly output: unknown }[] };
type RawProbe = { readonly document: string; readonly export: string; readonly text: string; readonly valid: boolean; readonly dropsKeys?: boolean };
type Corpus = { readonly cases: readonly { readonly name: string; readonly documents: readonly { readonly name: string; readonly seed: string; readonly schema: { readonly $id: string } }[]; readonly probes: readonly Probe[]; readonly rawProbes?: readonly RawProbe[] }[] };

const corpus = JSON.parse(readFileSync(new URL("../🧫️fixtures/🎯️ajv-oracle/🔣️.json", import.meta.url), "utf8"), (_key, value: unknown) => value) as Corpus;
const root = mkdtempSync(join(tmpdir(), "semio-parser-projection-"));
afterAll(() => rmSync(root, { recursive: true, force: true }));

const leafOf = (caseName: string, documentName: string): string => join(root, "✏️s", "oracle", caseName, documentName, "🧬️schema", "🟦️.ts");
const ajv = new Ajv({ strict: false, validateFormats: false });

for (const entry of corpus.cases) {
  for (const document of entry.documents) {
    mkdirSync(join(leafOf(entry.name, document.name), ".."), { recursive: true });
    writeFileSync(join(leafOf(entry.name, document.name), "..", "🔣️.json"), `${JSON.stringify(document.schema, null, 2)}\n`);
    writeFileSync(leafOf(entry.name, document.name), `/** 🌱️ Hand-written declarations of the ${entry.name} case. */\n${document.seed}`);
    ajv.addSchema(document.schema);
  }
}

const options: ProjectionOptions = { root, mode: "apply", under: "✏️s", formats: new Set(["🟦️typescript"]), closing: false, replace: new Set(), restate: new Set() };
const report = projectParsers(options);
const modules = new Map<string, Record<string, Parser>>();
for (const entry of corpus.cases) {
  for (const document of entry.documents) modules.set(`${entry.name}/${document.name}`, (await import(pathToFileURL(leafOf(entry.name, document.name)).href)) as Record<string, Parser>);
}

test("every export of the corpus is projected and nothing is refused", () => {
  expect(report.refused).toEqual([]);
  expect(report.touched.length).toBe(corpus.cases.flatMap((entry) => entry.documents).length);
});

const typeErrors = async (base: string, leaves: readonly string[]): Promise<string[]> => {
  const ts = await import("typescript");
  const program = ts.createProgram([...leaves], { strict: true, noEmit: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true });
  return ts.getPreEmitDiagnostics(program).map((diagnostic) => `${diagnostic.file?.fileName.slice(base.length)}: ${ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")}`);
};

test("the projected sources type-check under strict TypeScript", async () => {
  expect(await typeErrors(root, corpus.cases.flatMap((entry) => entry.documents.map((document) => leafOf(entry.name, document.name))))).toEqual([]);
});

test("the audit finds no drift and no missing helper in the projected sources", () => {
  const audit = auditLeaves({ root, under: "✏️s" });
  expect(audit.drift.map((row) => `${row.reason} ${row.name}`)).toEqual([]);
  expect(audit.helperless).toEqual([]);
});

test("a second projection with every restate reason rewrites nothing", () => {
  const again = projectParsers({ ...options, mode: "report", restate: new Set(["open", "stale", "untyped-map", "self-call"]) });
  expect(again.touched).toEqual([]);
});

const accepts = (parser: Parser, value: unknown): boolean => {
  try {
    parser(value);
    return true;
  } catch {
    return false;
  }
};

for (const entry of corpus.cases) {
  const documentIds = new Map(entry.documents.map((document) => [document.name, document.schema.$id]));
  const validatorOf = (probe: { readonly document: string; readonly export: string }) => ajv.getSchema(`${documentIds.get(probe.document)}#/$defs/${probe.export}`)!;
  const parserOf = (probe: { readonly document: string; readonly export: string }): Parser => modules.get(`${entry.name}/${probe.document}`)![`parse${probe.export}`]!;

  for (const probe of entry.probes) {
    test(`${entry.name}: parse${probe.export} agrees with Ajv`, () => {
      const validate = validatorOf(probe);
      const parser = parserOf(probe);
      expect(typeof validate, `no schema for ${probe.export}`).toBe("function");
      expect(typeof parser, `no parser for ${probe.export}`).toBe("function");
      for (const value of probe.valid ?? []) {
        expect(validate(value), `oracle should admit ${JSON.stringify(value)}`).toBe(true);
        expect(parser(value), `parser should return ${JSON.stringify(value)}`).toEqual(value);
      }
      for (const value of probe.invalid ?? []) {
        expect(validate(value), `oracle should refuse ${JSON.stringify(value)}`).toBe(false);
        expect(accepts(parser, value), `parser should refuse ${JSON.stringify(value)}`).toBe(false);
      }
    });

    test(`${entry.name}: parse${probe.export} is stricter than its schema exactly where recorded`, () => {
      const validate = validatorOf(probe);
      const parser = parserOf(probe);
      for (const value of probe.stricter ?? []) {
        expect(validate(value), `the schema admits ${JSON.stringify(value)}`).toBe(true);
        expect(accepts(parser, value), `the parser refuses ${JSON.stringify(value)}`).toBe(false);
      }
    });

    test(`${entry.name}: parse${probe.export} fills the schema default of an absent property the declared type requires`, () => {
      const validate = validatorOf(probe);
      const parser = parserOf(probe);
      for (const { input, output } of probe.defaulted ?? []) {
        expect(validate(input), `the schema admits ${JSON.stringify(input)}`).toBe(true);
        expect(parser(input), `the parser fills ${JSON.stringify(input)}`).toEqual(output);
      }
    });

    test(`${entry.name}: parse${probe.export} drops an undeclared property only where the schema leaves the object open`, () => {
      const validate = validatorOf(probe);
      const parser = parserOf(probe);
      for (const value of probe.tolerated ?? []) {
        expect(validate(value), `the schema admits ${JSON.stringify(value)}`).toBe(true);
        expect(accepts(parser, value), `the parser admits ${JSON.stringify(value)}`).toBe(true);
        expect(JSON.stringify(parser(value)), `the parser keeps only the declared properties of ${JSON.stringify(value)}`).not.toContain("extra");
      }
    });
  }

  for (const probe of entry.rawProbes ?? []) {
    test(`${entry.name}: parse${probe.export} agrees with Ajv on the raw JSON ${probe.text}`, () => {
      const value: unknown = JSON.parse(probe.text);
      expect(validatorOf(probe)(value)).toBe(probe.valid);
      expect(accepts(parserOf(probe), value)).toBe(probe.valid);
      if (probe.valid && !probe.dropsKeys) for (const key of Object.keys(value as object)) expect(Object.hasOwn(parserOf(probe)(value) as object, key)).toBe(true);
    });
  }
}

const OLDER = "olderGuard";
const portableOlder = "✏️s/oracle/older/older/🧬️schema/🟦️.ts";
const olderLeaf = join(root, "✏️s", "oracle", "older", "older", "🧬️schema", "🟦️.ts");
const olderSchema = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: "https://json.schemas.assets.semio-tech.com/s/oracle/older.json",
  title: "older",
  $defs: {
    Point: { title: "Point", type: "object", additionalProperties: false, required: ["x", "y"], properties: { x: { type: "number" }, y: { type: "number" } } },
    Marker: { title: "Marker", type: "object", required: ["id"], properties: { id: { type: "string" } } },
  },
};
const olderSource = `/** 🌱️ A file projected before the refusal of unknown properties existed: its helper block has no \`Closed\`. */
export interface Point {
  x: number;
  y: number;
}

export interface Marker {
  id: string;
}

${helpersBlock(OLDER).replace(closedHelper(OLDER), "")}
export function parsePoint(value: unknown, at = "$"): Point {
  const row = ${OLDER}Object(value, at);
  return {
    x: ${OLDER}Number(row["x"], \`\${at}.x\`),
    y: ${OLDER}Number(row["y"], \`\${at}.y\`),
  };
}

export function parseMarker(value: unknown, at = "$"): Marker {
  const row = ${OLDER}Object(value, at);
  return {
    id: ${OLDER}String(row["id"], \`\${at}.id\`),
  };
}
`;

test("a file projected before the refusal existed is restated in place, receives the helper once, and then agrees with Ajv", async () => {
  mkdirSync(join(olderLeaf, ".."), { recursive: true });
  writeFileSync(join(olderLeaf, "..", "🔣️.json"), `${JSON.stringify(olderSchema, null, 2)}\n`);
  writeFileSync(olderLeaf, olderSource);
  const scope = "✏️s/oracle/older";
  const before = auditLeaves({ root, under: scope });
  expect(before.drift.map((row) => `${row.reason} ${row.name}`)).toEqual(["open Point"]);
  expect(before.helperless).toEqual([]);
  const planned = projectParsers({ ...options, mode: "report", under: scope, restate: new Set(["open"]) });
  expect(planned.touched.length).toBe(1);
  expect(readFileSync(olderLeaf, "utf8")).toBe(olderSource);
  projectParsers({ ...options, under: scope, restate: new Set(["open"]) });
  const after = readFileSync(olderLeaf, "utf8");
  expect(after.split(`export const ${OLDER}Closed`).length - 1).toBe(1);
  expect(after.indexOf(`export const ${OLDER}Closed`)).toBeLessThan(after.indexOf(`export const ${OLDER}Array`));
  expect(after.indexOf("export function parsePoint")).toBeLessThan(after.indexOf("export function parseMarker"));
  expect(after).toContain(`${OLDER}Closed(${OLDER}Object(value, at), at, ["x", "y"])`);
  expect(after.split("export function parseMarker").length - 1).toBe(1);
  expect(auditLeaves({ root, under: scope })).toEqual({ drift: [], helperless: [] });
  expect(projectParsers({ ...options, mode: "report", under: scope, restate: new Set(["open", "stale"]) }).touched).toEqual([]);
  expect(await typeErrors(root, [olderLeaf])).toEqual([]);
  const module = (await import(`${pathToFileURL(olderLeaf).href}?restated`)) as Record<string, Parser>;
  const validate = new Ajv({ strict: false }).compile(olderSchema.$defs.Point);
  for (const value of [{ x: 0, y: 1 }, { x: 0, y: 1, z: 2 }, { x: 0 }, { x: "0", y: 1 }, null]) expect(accepts(module.parsePoint!, value), JSON.stringify(value)).toBe(validate(value));
  expect(module.parseMarker!({ id: "m", extra: 1 })).toEqual({ id: "m" });
  writeFileSync(olderLeaf, after.replace(closedHelper(OLDER), ""));
  expect(auditLeaves({ root, under: scope }).helperless).toEqual([portableOlder]);
});

const gateLeaf = join(root, "✏️s", "oracle", "gate", "gate", "🧬️schema", "🟦️.ts");
const gateSchema = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: "https://json.schemas.assets.semio-tech.com/s/oracle/gate.json",
  title: "gate",
  $defs: { Point: { title: "Point", type: "object", additionalProperties: false, required: ["x", "y"], properties: { x: { type: "number" }, y: { type: "number" } } } },
};
const gatePath = "✏️s/oracle/gate/gate/🧬️schema/🟦️.ts";

test("the gate fails on every drifted parser except the listed unowned ones, and on a listed parser that no longer drifts", () => {
  mkdirSync(join(gateLeaf, ".."), { recursive: true });
  writeFileSync(join(gateLeaf, "..", "🔣️.json"), `${JSON.stringify(gateSchema, null, 2)}\n`);
  writeFileSync(gateLeaf, `export interface Point {\n  x: number;\n  y: number;\n}\n\n${helpersBlock("gateGuard")}\nexport function parsePoint(value: unknown, at = "$"): Point {\n  const row = gateGuardClosed(gateGuardObject(value, at), at, ["x", "y"]);\n  return { x: gateGuardNumber(row["x"], \`\${at}.x\`), y: gateGuardNumber(row["y"], \`\${at}.y\`) };\n}\n`);
  const scopes = ["✏️s/oracle/gate"];
  const open = checkParsers({ root, scopes, unowned: [] });
  expect(open.drift.map((row) => `${row.reason} ${row.name}`)).toEqual(["stale Point"]);
  expect(open.ok).toBe(false);
  const listed = checkParsers({ root, scopes, unowned: [{ path: gatePath, name: "Point", reason: "hand" }] });
  expect(listed).toMatchObject({ ok: true, unowned: 1, drift: [], orphans: [] });
  const orphan = checkParsers({ root, scopes, unowned: [{ path: gatePath, name: "Point", reason: "hand" }, { path: gatePath, name: "Missing", reason: "hand" }] });
  expect(orphan.ok).toBe(false);
  expect(orphan.orphans.map((row) => row.name)).toEqual(["Missing"]);
  projectParsers({ ...options, under: scopes[0]!, restate: new Set(["stale"]), unowned: [] });
  const restated = checkParsers({ root, scopes, unowned: [] });
  expect(restated).toMatchObject({ ok: true, unowned: 0, drift: [], orphans: [] });
  const stale = checkParsers({ root, scopes, unowned: [{ path: gatePath, name: "Point", reason: "hand" }] });
  expect(stale.ok).toBe(false);
  expect(stale.orphans.map((row) => row.name)).toEqual(["Point"]);
});
