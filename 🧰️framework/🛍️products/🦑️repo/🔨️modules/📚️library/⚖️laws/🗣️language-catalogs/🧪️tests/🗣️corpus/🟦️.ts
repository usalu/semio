/** 🗣️ The language law agrees with its language-agnostic corpus and an independent Python checker, and the repository itself raises none of its rules. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { LANGUAGE_LAW_RULES_V1, isProseV1, languageLawAllowlistV1, languageLawFindingsV1, proseSitesV1, readLanguageLawSourcesV1, type LanguageLawAllowlistV1 } from "../../🟦️.ts";
import allowlist from "../../🔒️allowlist/🔣️.json" with { type: "json" };
import allowlistSchema from "../../🔒️allowlist/🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const workspace = (() => { for (let at = import.meta.dir; ; at = dirname(at)) { if (existsSync(join(at, "bunfig.toml")) && existsSync(join(at, "nx.json"))) return at; if (dirname(at) === at) throw Error("workspace root not found"); } })();
const checker = fileURLToPath(new URL("./🐍️checker.py", import.meta.url));
const python = (args: readonly string[], input?: string): unknown => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", checker, ...args], { encoding: "utf8", input, maxBuffer: 1 << 28 });
  if (run.status !== 0) throw new Error(`checker failed: ${run.stderr}`);
  return JSON.parse(run.stdout);
};

test("the corpus and the allowlist satisfy the strict independent validator", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(ajv.validate(allowlistSchema, allowlist), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.name)).size).toBe(corpus.cases.length);
  const named = new Set(corpus.cases.flatMap((row) => row.expected.map((finding) => finding.rule)));
  for (const rule of LANGUAGE_LAW_RULES_V1) expect(named.has(rule), `a corpus case raises ${rule}`).toBe(true);
});

test("every corpus case raises exactly the findings it names", () => {
  for (const row of corpus.cases) expect(languageLawFindingsV1(row.sources, row.allowlist as LanguageLawAllowlistV1), row.name).toEqual(row.expected as never);
});

test("the prose census counts the literals the corpus counts", () => {
  for (const row of corpus.prose) expect(proseSitesV1(row.text), row.text).toBe(row.count);
  expect(isProseV1("Hello world")).toBe(true);
  expect(isProseV1("E_CODE")).toBe(false);
});

test("the independent checker raises exactly the same findings and counts", () => {
  const other = python(["cases"], JSON.stringify(corpus)) as { cases: { name: string; findings: unknown[] }[]; prose: number[] };
  other.cases.forEach((row, index) => { expect(row.name).toBe(corpus.cases[index]!.name); expect(row.findings, row.name).toEqual(corpus.cases[index]!.expected as never); });
  expect(other.prose).toEqual(corpus.prose.map((row) => row.count));
});

test("the repository holds no language law finding and its allowlist is exact", () => {
  const sources = readLanguageLawSourcesV1(workspace);
  expect(sources.length).toBeGreaterThan(1000);
  const findings = languageLawFindingsV1(sources, allowlist as LanguageLawAllowlistV1);
  expect(findings.slice(0, 40), `${findings.length} language law finding(s)`).toEqual([]);
  expect(languageLawAllowlistV1(sources, { ceiling: allowlist.ceiling, defaultsCeiling: allowlist.defaultsCeiling })).toEqual(allowlist as never);
}, 120000);

test("the independent checker agrees on the repository", () => {
  expect(python(["scan", workspace, join(dirname(fileURLToPath(import.meta.url)), "../../🔒️allowlist/🔣️.json")])).toEqual([]);
  expect(python(["cut", workspace])).toEqual(allowlist as never);
}, 120000);
