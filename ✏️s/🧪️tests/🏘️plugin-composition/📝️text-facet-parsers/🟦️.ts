/** 📝️ Every generated `📝️text` facet whose type is `string` parses a string and refuses everything else, agreeing with Ajv reading `{ "type": "string" }`.
 * @see https://ajv.js.org/json-schema.html#type */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";

const root = fileURLToPath(new URL("../../../../", import.meta.url));
const listed = Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "--", "✏️s/🔌️plugins/*/📝️text/*/🟦️.ts"], { cwd: root });
const stringParsers = (source: string): string[] =>
  [...source.matchAll(/export function (parse\w+Text)\(value: unknown, at = "\$"\): (\w+) \{\n  return \w+Guard(?:String|Object)\(value, `\$\{at\}`\);\n\}/g)]
    .filter(([, , type]) => new RegExp(`export type ${type} = string;`).test(source))
    .map(([, name]) => name!);
const cases = listed.stdout.toString().split("\n").filter(Boolean)
  .flatMap((path) => stringParsers(readFileSync(root + path, "utf8")).map((name) => ({ path, name })));
const oracle = new Ajv({ strict: true }).compile({ type: "string" });
const hostile: readonly unknown[] = [{}, [], null, undefined, 0, true];

test("the corpus of string text facets is not empty", () => {
  expect(cases.length).toBeGreaterThan(100);
});

for (const { path, name } of cases) test(`text facet ${name} in ${path.split("/🗿️artifacts/")[1]}`, async () => {
  const module = (await import(root + path)) as Record<string, unknown>;
  const parse = module[name] as (value: unknown, at?: string) => unknown;
  expect(typeof parse).toBe("function");
  for (const accepted of ["", "text", "a\nb"]) {
    expect(oracle(accepted)).toBe(true);
    expect(parse(accepted)).toBe(accepted);
  }
  for (const refused of hostile) {
    expect(oracle(refused)).toBe(false);
    expect(() => parse(refused)).toThrow();
  }
});
