/** 🧷️ A property whose schema states no structure while its declared type instantiates a generic of the codebase has no projection: the parser is refused instead of emitted with a type error or a cast. Every projected source is type-checked with the TypeScript compiler and agrees with the independent Ajv validator; the cases are the language-agnostic corpus next to this file.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🧪️tests/🧷️generic-types/🔣️.json
 */
import { afterAll, expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import { checkParsers, projectParsers, type ProjectionOptions } from "../../🟦️.ts";
import { instantiatesGeneric } from "../../📖️declared-types/🟦️.ts";
import corpus from "./🔣️.json";

type Row = { readonly name: string; readonly seed: string; readonly schema: { readonly $id: string }; readonly refused: readonly string[]; readonly valid: readonly unknown[]; readonly invalid: readonly unknown[] };
const rows = corpus.cases as unknown as readonly Row[];

const root = mkdtempSync(join(tmpdir(), "semio-generic-types-"));
afterAll(() => rmSync(root, { recursive: true, force: true }));

const options: ProjectionOptions = { root, mode: "apply", under: "✏️s", formats: new Set(["🟦️typescript"]), closing: false, replace: new Set(), restate: new Set() };
const moduleOf = (name: string): string => join(root, "✏️s", "oracle", name, name, "🧬️schema");
const scopeOf = (name: string): string => `✏️s/oracle/${name}`;

function write(file: string, text: string): void {
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, text);
}

const typeErrors = async (leaf: string): Promise<string[]> => {
  const ts = await import("typescript");
  const program = ts.createProgram([leaf], { strict: true, noEmit: true, skipLibCheck: true, types: [], target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, allowImportingTsExtensions: true });
  return ts.getPreEmitDiagnostics(program).map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"));
};

for (const row of rows) {
  test(`${row.name}: ${row.refused.length > 0 ? "the parser is refused and nothing is emitted" : "the parser is projected, type-checks and agrees with Ajv"}`, async () => {
    const leaf = join(moduleOf(row.name), "🟦️.ts");
    write(join(moduleOf(row.name), "🔣️.json"), `${JSON.stringify(row.schema, null, 2)}\n`);
    write(leaf, `/** 🌱️ Hand-written declarations of the ${row.name} case. */\n${row.seed}`);
    const report = projectParsers({ ...options, under: scopeOf(row.name) });
    expect(report.refused.length, report.refused.join("\n")).toBe(row.refused.length);
    for (const [position, text] of row.refused.entries()) expect(report.refused[position]).toContain(text);
    const emitted = readFileSync(leaf, "utf8");
    expect(emitted.includes("export function parseWrapper(")).toBe(row.refused.length === 0);
    expect(emitted).not.toContain(" as unknown as ");
    expect(await typeErrors(leaf)).toEqual([]);
    expect(checkParsers({ root, scopes: [scopeOf(row.name)], unowned: [] })).toMatchObject({ ok: true, pending: [], drift: [], dangling: [], helperless: [] });
    if (row.refused.length > 0) return;
    const parser = ((await import(pathToFileURL(leaf).href)) as { parseWrapper: (value: unknown) => unknown }).parseWrapper;
    const validate = new Ajv({ strict: false }).compile(row.schema);
    for (const value of [...row.valid, ...row.invalid]) {
      const accepted = ((): boolean => {
        try {
          parser(value);
          return true;
        } catch {
          return false;
        }
      })();
      expect(accepted, JSON.stringify(value)).toBe(validate(value) as boolean);
    }
    for (const value of row.valid) expect(validate(value), JSON.stringify(value)).toBe(true);
    for (const value of row.invalid) expect(validate(value), JSON.stringify(value)).toBe(false);
  }, 60_000);
}

test("instantiatesGeneric tells the generics of the codebase from the types of the TypeScript library", () => {
  const aliases = new Map([["PointBox", "Box<Point>"], ["Plain", "Record<string, unknown>"], ["Loop", "Loop"]]);
  const of = (text: string): boolean => instantiatesGeneric(text, (name) => aliases.get(name));
  for (const text of ["Box<Point>", "Box<Point> | null", "Box<Point>[]", "PointBox", "PointBox[]", "readonly Box<Point>[]", "Record<string, Box<Point>>", "[Point, Box<Point>]", "NamedTripleDiff<string, D, T>"]) expect(of(text), text).toBe(true);
  for (const text of ["string", "Point", "Point[]", "Record<string, unknown>", "ReadonlyArray<string>", "Array<Point>", "Plain", "Loop", "[Point, string]", "string | null"]) expect(of(text), text).toBe(false);
});
