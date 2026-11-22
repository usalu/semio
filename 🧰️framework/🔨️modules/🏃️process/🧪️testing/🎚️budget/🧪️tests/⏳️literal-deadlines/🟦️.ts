/** ⏳️ Law: a test file declares no literal deadline above the threshold of `🔣️.json`; its deadlines derive from a declared budget stretched by the host contention (`stretchedBudgetMillisecondsV1`, a declared plan or a suite row). Each file has a ceiling of such literals, a new file or a higher count is refused, a lower count must lower the ceiling, so the list only shrinks; the independent Python scan must count the same. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { validateJsonSchemaSubset } from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { testPythonV1 } from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import { stretchedBudgetMillisecondsV1 } from "../../../../📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import config from "./🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const root = fileURLToPath(new URL("../../../../../../", import.meta.url)).replace(/\/$/u, "");

/** 🔭️ The literal deadlines at or above the threshold in every test file below the framework root, counted per file and form. */
function literalDeadlinesV1(): Record<string, number> {
  const forms = config.forms.map(form => new RegExp(form.pattern, "gmu")), testFile = new RegExp(config.testFile, "u"), skipped = new Set(config.skippedDirectories), found: Record<string, number> = {};
  const walk = (folder: string, relative: string): void => {
    for (const entry of readdirSync(folder, { withFileTypes: true })) {
      const next = relative === "" ? entry.name : relative + "/" + entry.name;
      if (entry.isSymbolicLink()) continue;
      if (entry.isDirectory()) { if (!skipped.has(entry.name)) walk(folder + "/" + entry.name, next); continue; }
      if (!/\.(ts|mjs|js)$/u.test(entry.name) || !testFile.test(next)) continue;
      const text = readFileSync(folder + "/" + entry.name, "utf8");
      const count = forms.reduce((sum, form) => sum + [...text.matchAll(form)].filter(match => Number(match[1]!.replaceAll("_", "")) >= config.thresholdMilliseconds).length, 0);
      if (count > 0) found[next] = count;
    }
  };
  walk(root, "");
  return found;
}

test("the literal deadline ceiling file matches its schema under Ajv and the repository validator", () => {
  expect(new Ajv({ strict: true }).validate(schema, config)).toBe(true);
  expect(validateJsonSchemaSubset(schema, config)).toEqual([]);
});

test("no test adds a literal deadline above the threshold and every file that lost one lowers its ceiling", () => {
  const actual = literalDeadlinesV1(), allowed = config.ceilings as Record<string, number>;
  const added = Object.keys(actual).filter(path => !(path in allowed) || actual[path]! > allowed[path]!);
  const slack = Object.keys(allowed).filter(path => (actual[path] ?? 0) < allowed[path]!).map(path => path + " -> " + (actual[path] ?? 0));
  expect({ added, slack }).toEqual({ added: [], slack: [] });
}, stretchedBudgetMillisecondsV1(120000));

test("the independent Python scan counts the same literal deadlines per file", () => {
  const python = testPythonV1();
  if (!python) throw Error("Independent Python oracle required");
  const result = Bun.spawnSync([python, "-I", fileURLToPath(new URL("./🐍️.py", import.meta.url)), fileURLToPath(new URL("./🔣️.json", import.meta.url)), root]);
  expect(result.stderr.toString()).toBe("");
  expect(JSON.parse(result.stdout.toString())).toEqual(literalDeadlinesV1());
}, stretchedBudgetMillisecondsV1(120000));
