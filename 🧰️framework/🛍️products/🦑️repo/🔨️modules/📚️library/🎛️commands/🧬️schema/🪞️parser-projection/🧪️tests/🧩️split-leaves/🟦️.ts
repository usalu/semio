/** 🧩️ A schema leaf split into component files keeps one parser file per component: the projection puts each parser in the `🚪️parsers` folder of the file that declares its type, the leaf re-exports nothing and importers name the folder, a parser the leaf needs stays in the leaf, no import cycle appears (checked with the TypeScript compiler), the freshness audit walks every file, and the parsers agree with the independent Ajv validator.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */
import { afterAll, expect, test } from "bun:test";
import Ajv from "ajv";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import { auditLeaves, checkParsers, projectParsers, type ProjectionOptions } from "../../🟦️.ts";
import { helperPrefix } from "../../🗂️documents/🟦️.ts";

type Parser = (value: unknown) => unknown;

const root = mkdtempSync(join(tmpdir(), "semio-parser-split-"));
afterAll(() => rmSync(root, { recursive: true, force: true }));

const options: ProjectionOptions = { root, mode: "apply", under: "✏️s", formats: new Set(["🟦️typescript"]), closing: false, replace: new Set(), restate: new Set() };
const PARSERS = "🚪️parsers";

const schemaOf = (name: string): { readonly $schema: string; readonly $id: string; readonly title: string; readonly $defs: Record<string, unknown> } => ({
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: `https://json.schemas.assets.semio-tech.com/s/oracle/${name}.json`,
  title: name,
  $defs: {
    Point: { title: "Point", type: "object", additionalProperties: false, required: ["x", "y"], properties: { x: { type: "number" }, y: { type: "number" } } },
    Segment: { title: "Segment", type: "object", additionalProperties: false, required: ["from", "to"], properties: { from: { $ref: "#/$defs/Point" }, to: { $ref: "#/$defs/Point" } } },
    Track: { title: "Track", type: "object", additionalProperties: false, required: ["segments"], properties: { segments: { type: "array", items: { $ref: "#/$defs/Segment" } } } },
  },
});

const POINT = "export interface Point {\n  x: number;\n  y: number;\n}\n";
const SEGMENT = 'import type { Point } from "../📐️geometry/🟦️.ts";\n\nexport interface Segment {\n  from: Point;\n  to: Point;\n}\n\nexport interface Track {\n  segments: Segment[];\n}\n';

const moduleOf = (name: string): string => join(root, "✏️s", "oracle", name, name, "🧬️schema");
const scopeOf = (name: string): string => `✏️s/oracle/${name}`;

function write(file: string, text: string): void {
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, text);
}

function seed(name: string, files: Readonly<Record<string, string>>): void {
  write(join(moduleOf(name), "🔣️.json"), `${JSON.stringify(schemaOf(name), null, 2)}\n`);
  for (const [path, text] of Object.entries(files)) write(join(moduleOf(name), path), text);
}

const typeErrors = async (leaves: readonly string[]): Promise<string[]> => {
  const ts = await import("typescript");
  const program = ts.createProgram([...leaves], { strict: true, noEmit: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true });
  return ts.getPreEmitDiagnostics(program).map((diagnostic) => `${diagnostic.file?.fileName.slice(root.length)}: ${ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")}`);
};

const accepts = (parser: Parser, value: unknown): boolean => {
  try {
    parser(value);
    return true;
  } catch {
    return false;
  }
};

const POINTS = [{ x: 0, y: 1 }, { x: 0 }, { x: "0", y: 1 }, { x: 0, y: 1, z: 2 }, null];
const SEGMENTS = [{ from: { x: 0, y: 1 }, to: { x: 2, y: 3 } }, { from: { x: 0, y: 1 } }, { from: { x: 0, y: 1 }, to: { x: "2", y: 3 } }];
const TRACKS = [{ segments: [] }, { segments: [SEGMENTS[0]] }, { segments: [SEGMENTS[1]] }, { segments: 1 }];

async function agreesWithAjv(name: string, tag: string, homes: Readonly<Record<string, string>>): Promise<void> {
  const schema = schemaOf(name);
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(schema);
  const modules = new Map<string, Record<string, Parser>>();
  for (const file of new Set(Object.values(homes))) modules.set(file, (await import(`${pathToFileURL(file).href}?${tag}`)) as Record<string, Parser>);
  for (const [type, values] of [["Point", POINTS], ["Segment", SEGMENTS], ["Track", TRACKS]] as const) {
    const validate = ajv.getSchema(`${schema.$id}#/$defs/${type}`)!;
    const parser = modules.get(homes[type]!)![`parse${type}`]!;
    for (const value of values) expect(accepts(parser, value), `${type} ${JSON.stringify(value)}`).toBe(validate(value) as boolean);
  }
}

/** 🔬️ The cycles among the runtime (not type-only) relative imports and re-exports of `files`, found with the TypeScript compiler API as the independent oracle. */
async function importCycles(files: readonly string[]): Promise<string[][]> {
  const ts = await import("typescript");
  const edges = new Map<string, string[]>();
  for (const file of files) {
    const source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.ESNext);
    const targets: string[] = [];
    for (const statement of source.statements) {
      const specifier = ts.isImportDeclaration(statement) ? statement.moduleSpecifier : ts.isExportDeclaration(statement) ? statement.moduleSpecifier : undefined;
      const typeOnly = ts.isImportDeclaration(statement) ? statement.importClause?.isTypeOnly === true : ts.isExportDeclaration(statement) && statement.isTypeOnly;
      if (specifier !== undefined && ts.isStringLiteral(specifier) && specifier.text.startsWith(".") && !typeOnly) targets.push(join(dirname(file), specifier.text));
    }
    edges.set(file, targets);
  }
  const cycles: string[][] = [];
  const visit = (file: string, path: readonly string[]): void => {
    if (path.includes(file)) return void cycles.push([...path.slice(path.indexOf(file)), file].map((entry) => entry.slice(root.length)));
    for (const next of edges.get(file) ?? []) visit(next, [...path, file]);
  };
  for (const file of files) visit(file, []);
  return cycles;
}

const geometryParsers = (name: string): string => join(moduleOf(name), "📐️geometry", PARSERS, "🟦️.ts");
const pathParsers = (name: string): string => join(moduleOf(name), "🛤️paths", PARSERS, "🟦️.ts");

test("parsers of a split leaf are written next to the component that declares their type, the leaf re-exports nothing, nothing imports a cycle, and the audit walks every file", async () => {
  seed("fresh", { "🟦️.ts": '/** 🌱️ A leaf whose types live in component files. */\nimport type { Point } from "./📐️geometry/🟦️.ts";\nimport type { Segment, Track } from "./🛤️paths/🟦️.ts";\n\nexport type { Point, Segment, Track };\n', "📐️geometry/🟦️.ts": POINT, "🛤️paths/🟦️.ts": SEGMENT });
  const report = projectParsers({ ...options, under: scopeOf("fresh") });
  expect(report.refused).toEqual([]);
  const leaf = join(moduleOf("fresh"), "🟦️.ts");
  expect([...report.touched].sort()).toEqual(["📐️geometry/🚪️parsers/🟦️.ts", "🛤️paths/🚪️parsers/🟦️.ts", "🟦️.ts"].map((file) => `${scopeOf("fresh")}/fresh/🧬️schema/${file}`).sort());
  const leafText = readFileSync(leaf, "utf8");
  expect(leafText).toContain("//#region 🚪️Parsers");
  expect(leafText).not.toContain("export function parse");
  expect(leafText).not.toContain(PARSERS);
  expect(leafText).not.toMatch(/\bparse(?:Point|Segment|Track)\b/u);
  expect(readFileSync(geometryParsers("fresh"), "utf8")).toContain("export function parsePoint(");
  const paths = readFileSync(pathParsers("fresh"), "utf8");
  expect(paths).toContain("export function parseSegment(");
  expect(paths).toContain("export function parseTrack(");
  expect(paths).toContain('import { parsePoint } from "../../📐️geometry/🚪️parsers/🟦️.ts";');
  expect(paths).toContain('import type { Segment, Track } from "../🟦️.ts";');
  expect(paths).toMatch(/import \{ \w+Array, \w+Closed, \w+Object \} from "\.\.\/\.\.\/🟦️\.ts";/u);
  expect(auditLeaves({ root, under: scopeOf("fresh") })).toEqual({ drift: [], helperless: [] });
  expect(projectParsers({ ...options, mode: "report", under: scopeOf("fresh"), closing: true, restate: new Set(["open", "stale", "untyped-map", "self-call"]) }).touched).toEqual([]);
  expect(checkParsers({ root, scopes: [scopeOf("fresh")], unowned: [] })).toMatchObject({ ok: true, pending: [], drift: [], dangling: [], helperless: [] });
  expect(await typeErrors([leaf, geometryParsers("fresh"), pathParsers("fresh")])).toEqual([]);
  expect(await importCycles([leaf, geometryParsers("fresh"), pathParsers("fresh")])).toEqual([]);
  await agreesWithAjv("fresh", "fresh", { Point: geometryParsers("fresh"), Segment: pathParsers("fresh"), Track: pathParsers("fresh") });
}, 60_000);

test("a drifted parser in a parser folder is found there and restated in place, and a second definition of a parser is a duplicate", () => {
  const name = "fresh";
  const file = pathParsers(name);
  const clean = readFileSync(file, "utf8");
  const helper = helperPrefix(schemaOf(name).$id);
  write(file, clean.replace(`${helper}Closed(${helper}Object(value, at), at, ["from", "to"])`, `${helper}Object(value, at)`));
  const open = auditLeaves({ root, under: scopeOf(name) });
  expect(open.drift.map((row) => `${row.reason} ${row.name} ${row.path.slice(row.path.indexOf("🛤️"))}`)).toEqual([`open Segment 🛤️paths/🚪️parsers/🟦️.ts`]);
  expect(open.drift[0]!.leaf).toBe(`${scopeOf(name)}/${name}/🧬️schema/🟦️.ts`);
  expect(checkParsers({ root, scopes: [scopeOf(name)], unowned: [] }).ok).toBe(false);
  projectParsers({ ...options, under: scopeOf(name), restate: new Set(["open"]) });
  expect(readFileSync(file, "utf8")).toBe(clean);
  expect(auditLeaves({ root, under: scopeOf(name) })).toEqual({ drift: [], helperless: [] });
  const leaf = join(moduleOf(name), "🟦️.ts");
  const original = readFileSync(leaf, "utf8");
  write(leaf, `${original}\nexport function parsePoint(value: unknown, at = "$"): Point {\n  return ${helper}Object(value, at) as unknown as Point;\n}\n`);
  expect(auditLeaves({ root, under: scopeOf(name) }).drift.map((row) => `${row.reason} ${row.name}`)).toContain("duplicate Point");
  write(leaf, original);
});

test("distribute moves the fresh parsers of a leaf into the component folders and drops the imports it no longer needs", async () => {
  const name = "moved";
  seed(name, { "🟦️.ts": `/** 🌱️ A leaf that held its types and its parsers before it was split. */\n${POINT}\n${SEGMENT.replace('import type { Point } from "../📐️geometry/🟦️.ts";\n\n', "")}` });
  expect(projectParsers({ ...options, under: scopeOf(name) }).refused).toEqual([]);
  const leaf = join(moduleOf(name), "🟦️.ts");
  const whole = readFileSync(leaf, "utf8");
  expect(whole.match(/^export function parse/gmu)?.length).toBe(3);
  write(join(moduleOf(name), "📐️geometry/🟦️.ts"), POINT);
  write(join(moduleOf(name), "🛤️paths/🟦️.ts"), SEGMENT);
  write(leaf, `${whole.slice(0, whole.indexOf("export interface Point"))}import type { Point } from "./📐️geometry/🟦️.ts";\nimport type { Segment, Track } from "./🛤️paths/🟦️.ts";\n\nexport type { Point, Segment, Track };\n${whole.slice(whole.indexOf("//#region 🚪️Parsers"))}`);
  const folder = `${scopeOf(name)}/${name}/🧬️schema`;
  const planned = projectParsers({ ...options, mode: "report", under: scopeOf(name), distribute: [folder] });
  expect(planned.touched.length).toBe(3);
  expect(readFileSync(leaf, "utf8").match(/^export function parse/gmu)?.length).toBe(3);
  projectParsers({ ...options, under: scopeOf(name), distribute: [folder] });
  const after = readFileSync(leaf, "utf8");
  expect(after).not.toContain("export function parse");
  expect(after).not.toContain(PARSERS);
  expect(after).not.toMatch(/\bparse(?:Point|Segment|Track)\b/u);
  expect(auditLeaves({ root, under: scopeOf(name) })).toEqual({ drift: [], helperless: [] });
  expect(projectParsers({ ...options, mode: "report", under: scopeOf(name), distribute: [folder], closing: true }).touched).toEqual([]);
  expect(await typeErrors([leaf, geometryParsers(name), pathParsers(name)])).toEqual([]);
  expect(await importCycles([leaf, geometryParsers(name), pathParsers(name)])).toEqual([]);
  await agreesWithAjv(name, "moved", { Point: geometryParsers(name), Segment: pathParsers(name), Track: pathParsers(name) });
}, 60_000);

const documentOf = (name: string, defs: Record<string, unknown>): { readonly $schema: string; readonly $id: string; readonly title: string; readonly $defs: Record<string, unknown> } => ({
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: `https://json.schemas.assets.semio-tech.com/s/oracle/${name}.json`,
  title: name,
  $defs: defs,
});

const MARKER = { title: "Marker", type: "object", additionalProperties: false, required: ["label"], properties: { label: { type: "string" } } };

test("a parser the leaf's own code calls stays in the leaf with the parsers it needs, the other components still get folders, and no import closes a cycle", async () => {
  const name = "kept";
  const defs = { ...schemaOf(name).$defs, Marker: MARKER };
  const folder = (component: string): string => join(moduleOf(name), component, PARSERS, "🟦️.ts");
  write(join(moduleOf(name), "🔣️.json"), `${JSON.stringify(documentOf(name, defs), null, 2)}\n`);
  write(join(moduleOf(name), "🟦️.ts"), '/** 🌱️ A leaf whose own code calls a parser. */\nimport type { Point } from "./📐️geometry/🟦️.ts";\nimport type { Segment, Track } from "./🛤️paths/🟦️.ts";\nimport type { Marker } from "./📍️markers/🟦️.ts";\n\nexport type { Marker, Point, Segment, Track };\n\nexport const firstSegment = (value: unknown): Segment => parseSegment(value);\n');
  write(join(moduleOf(name), "📐️geometry/🟦️.ts"), POINT);
  write(join(moduleOf(name), "🛤️paths/🟦️.ts"), SEGMENT);
  write(join(moduleOf(name), "📍️markers/🟦️.ts"), "export interface Marker {\n  label: string;\n}\n");
  expect(projectParsers({ ...options, under: scopeOf(name) }).refused).toEqual([]);
  const leaf = join(moduleOf(name), "🟦️.ts");
  const leafText = readFileSync(leaf, "utf8");
  expect(leafText.match(/^export function parse(\w+)/gmu)).toEqual(["export function parsePoint", "export function parseSegment"]);
  expect(leafText).not.toContain(PARSERS);
  expect(existsSync(folder("📐️geometry"))).toBe(false);
  expect(readFileSync(folder("🛤️paths"), "utf8")).toMatch(/import \{ [^}]*\bparseSegment\b[^}]* \} from "\.\.\/\.\.\/🟦️\.ts";/u);
  expect(readFileSync(folder("🛤️paths"), "utf8")).toContain("export function parseTrack(");
  expect(readFileSync(folder("📍️markers"), "utf8")).toContain("export function parseMarker(");
  const files = [leaf, folder("🛤️paths"), folder("📍️markers")];
  expect(auditLeaves({ root, under: scopeOf(name) })).toEqual({ drift: [], helperless: [] });
  expect(projectParsers({ ...options, mode: "report", under: scopeOf(name), closing: true }).touched).toEqual([]);
  expect(await typeErrors(files)).toEqual([]);
  expect(await importCycles(files)).toEqual([]);
  const ajv = new Ajv({ strict: false });
  ajv.addSchema(documentOf(name, defs));
  const parsers = { ...((await import(`${pathToFileURL(leaf).href}?kept`)) as Record<string, Parser>), ...((await import(`${pathToFileURL(folder("🛤️paths")).href}?kept`)) as Record<string, Parser>) };
  for (const [type, values] of [["Point", POINTS], ["Segment", SEGMENTS], ["Track", TRACKS]] as const) {
    const validate = ajv.getSchema(`${documentOf(name, defs).$id}#/$defs/${type}`)!;
    for (const value of values) expect(accepts(parsers[`parse${type}`]!, value), `${type} ${JSON.stringify(value)}`).toBe(validate(value) as boolean);
  }
}, 60_000);

test("types that refer to each other across components keep their parsers in the leaf, so the folders never import each other in a cycle", async () => {
  const name = "loop";
  const defs = {
    Alpha: { title: "Alpha", type: "object", additionalProperties: false, properties: { beta: { $ref: "#/$defs/Beta" } } },
    Beta: { title: "Beta", type: "object", additionalProperties: false, properties: { alpha: { $ref: "#/$defs/Alpha" } } },
    Gamma: { title: "Gamma", type: "object", additionalProperties: false, required: ["name"], properties: { name: { type: "string" } } },
  };
  write(join(moduleOf(name), "🔣️.json"), `${JSON.stringify(documentOf(name, defs), null, 2)}\n`);
  write(join(moduleOf(name), "🟦️.ts"), '/** 🌱️ A leaf of components that refer to each other. */\nimport type { Alpha } from "./🅰️alpha/🟦️.ts";\nimport type { Beta } from "./🅱️beta/🟦️.ts";\nimport type { Gamma } from "./🌐️gamma/🟦️.ts";\n\nexport type { Alpha, Beta, Gamma };\n');
  write(join(moduleOf(name), "🅰️alpha/🟦️.ts"), 'import type { Beta } from "../🅱️beta/🟦️.ts";\n\nexport interface Alpha {\n  beta?: Beta;\n}\n');
  write(join(moduleOf(name), "🅱️beta/🟦️.ts"), 'import type { Alpha } from "../🅰️alpha/🟦️.ts";\n\nexport interface Beta {\n  alpha?: Alpha;\n}\n');
  write(join(moduleOf(name), "🌐️gamma/🟦️.ts"), "export interface Gamma {\n  name: string;\n}\n");
  expect(projectParsers({ ...options, under: scopeOf(name) }).refused).toEqual([]);
  const leaf = join(moduleOf(name), "🟦️.ts");
  expect(readFileSync(leaf, "utf8").match(/^export function parse(\w+)/gmu)).toEqual(["export function parseAlpha", "export function parseBeta"]);
  const gamma = join(moduleOf(name), "🌐️gamma", PARSERS, "🟦️.ts");
  expect(readFileSync(gamma, "utf8")).toContain("export function parseGamma(");
  expect(existsSync(join(moduleOf(name), "🅰️alpha", PARSERS))).toBe(false);
  expect(existsSync(join(moduleOf(name), "🅱️beta", PARSERS))).toBe(false);
  expect(auditLeaves({ root, under: scopeOf(name) })).toEqual({ drift: [], helperless: [] });
  expect(await typeErrors([leaf, gamma])).toEqual([]);
  expect(await importCycles([leaf, gamma])).toEqual([]);
});

test("a sibling document imports the parser of another document from the folder that holds it, never from the leaf", async () => {
  const head = (n: string): { readonly $schema: string; readonly $id: string; readonly title: string } => {
    const { $schema, $id, title } = documentOf(n, {});
    return { $schema, $id, title };
  };
  const owner = moduleOf("owner");
  const user = moduleOf("user");
  write(join(owner, "🔣️.json"), `${JSON.stringify({ ...head("owner"), $defs: { Point: { title: "Point", type: "object", additionalProperties: false, required: ["x"], properties: { x: { type: "number" } } } } }, null, 2)}\n`);
  write(join(owner, "🟦️.ts"), '/** 🌱️ The owning leaf. */\nimport type { Point } from "./📐️geometry/🟦️.ts";\n\nexport type { Point };\n');
  write(join(owner, "📐️geometry/🟦️.ts"), "export interface Point {\n  x: number;\n}\n");
  write(join(user, "🔣️.json"), `${JSON.stringify({ ...head("user"), $defs: { Shape: { title: "Shape", type: "object", additionalProperties: false, required: ["at"], properties: { at: { $ref: `${documentOf("owner", {}).$id}#/$defs/Point` } } } } }, null, 2)}\n`);
  write(join(user, "🟦️.ts"), '/** 🌱️ The importing leaf. */\nimport type { Point } from "../../../owner/owner/🧬️schema/🟦️.ts";\n\nexport interface Shape {\n  at: Point;\n}\n');
  expect(projectParsers({ ...options, under: "✏️s/oracle" }).refused).toEqual([]);
  const userLeaf = readFileSync(join(user, "🟦️.ts"), "utf8");
  expect(userLeaf).toContain('import { parsePoint } from "../../../owner/owner/🧬️schema/📐️geometry/🚪️parsers/🟦️.ts";');
  expect(readFileSync(join(owner, "🟦️.ts"), "utf8")).not.toMatch(/\bparsePoint\b/u);
  expect(checkParsers({ root, scopes: ["✏️s/oracle"], unowned: [] })).toMatchObject({ ok: true, pending: [], drift: [], dangling: [], helperless: [] });
  const files = [join(user, "🟦️.ts"), join(owner, "🟦️.ts"), join(owner, "📐️geometry", PARSERS, "🟦️.ts")];
  expect(await typeErrors(files)).toEqual([]);
  expect(await importCycles(files)).toEqual([]);
  const shape = (await import(`${pathToFileURL(join(user, "🟦️.ts")).href}?shape`)) as Record<string, Parser>;
  expect(accepts(shape.parseShape!, { at: { x: 1 } })).toBe(true);
  expect(accepts(shape.parseShape!, { at: { x: "1" } })).toBe(false);
  expect(accepts(shape.parseShape!, { at: { x: 1, y: 2 } })).toBe(false);
});
