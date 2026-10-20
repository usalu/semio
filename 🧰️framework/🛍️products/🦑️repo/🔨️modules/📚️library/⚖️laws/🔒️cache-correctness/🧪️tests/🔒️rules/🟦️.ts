/** 🔒️ The cache-correctness rules agree with their language-agnostic corpus and an independent glob oracle, and the repository itself raises none of them. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { minimatch } from "minimatch";
import { cacheCorrectnessRepositoryFindingsV1, globMatchesV1, policyCacheCorrectnessBreaches, repositoryCacheRecordsV1, staleRustKeyExemptionsV1, type CacheFindingV1 } from "../../🟦️.ts";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const workspace = (() => { for (let at = import.meta.dir; ; at = dirname(at)) { if (existsSync(join(at, "bunfig.toml")) && existsSync(join(at, "nx.json"))) return at; if (dirname(at) === at) throw Error("workspace root not found"); } })();
const pairs = (findings: readonly CacheFindingV1[]) => findings.map((finding) => [finding.id, finding.rule] as [string, string]).sort((a, b) => a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : a[1] < b[1] ? -1 : a[1] > b[1] ? 1 : 0);

test("the corpus satisfies the strict independent validator", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
});

test("every corpus case raises exactly the rules it names", () => {
  for (const row of corpus.cases) expect(pairs(cacheCorrectnessRepositoryFindingsV1(row.records as never)), row.id).toEqual(pairs(row.expect.map(([id, rule]) => ({ id, rule, detail: "" })) as never));
});

test("the glob matcher agrees with minimatch on every corpus pair", () => {
  for (const row of corpus.globs) {
    expect(globMatchesV1(row.pattern, row.path), `${row.pattern} ~ ${row.path}`).toBe(row.match);
    expect(minimatch(row.path, row.pattern, { dot: true }), `minimatch ${row.pattern} ~ ${row.path}`).toBe(row.match);
  }
});

test("the policy classes name every exact target once, only with admitted decisions", async () => {
  const plugin = await import("../../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs");
  await plugin.libraryBootstrap;
  const api = plugin.cacheInternals as any, policy = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8")), seen = new Map<string, number>();
  policy.targetClasses.forEach((row: any, index: number) => {
    api.cacheDecisionV1(row.decision.class, row.decision.reason);
    for (const name of row.exact ?? []) { expect(api.normalizedTargetName(name), `${name} is written without a symbol prefix`).toBe(name); expect(seen.has(name), `${name} is classed by row ${seen.get(name)} and row ${index}`).toBe(false); seen.set(name, index); }
  });
  for (const row of policy.targetClasses) for (const name of row.exact ?? []) expect(api.targetClassV1(name)?.class, name).toBe(row.decision.class);
});

test("no replayed target of the repository breaks a cache-correctness rule", async () => {
  const plugin = await import("../../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs");
  await plugin.libraryBootstrap;
  const records = repositoryCacheRecordsV1(workspace, plugin.cacheInternals as never), findings = cacheCorrectnessRepositoryFindingsV1(records);
  expect(records.length).toBeGreaterThan(3000);
  expect(policyCacheCorrectnessBreaches(findings).map((breach) => breach.summary).slice(0, 40), `${findings.length} cache-correctness finding(s)`).toEqual([]);
}, 120000);


test("every exempt whole-workspace Rust key still exists: the exemption list only shrinks", async () => {
  const plugin = await import("../../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs");
  await plugin.libraryBootstrap;
  expect(staleRustKeyExemptionsV1(repositoryCacheRecordsV1(workspace, plugin.cacheInternals as never))).toEqual([]);
}, 120000);
