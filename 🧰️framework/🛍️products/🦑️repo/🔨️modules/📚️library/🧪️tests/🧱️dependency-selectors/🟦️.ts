import { expect, test } from "bun:test";
import Ajv from "ajv/dist/2020.js";
import { cruise } from "dependency-cruiser";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { dependencyDirectionEdges, dependencyDirectionResolverRules, dependencyDirectionRule, dependencyDirectionSourceInventory } from "../../🕸️dependencies/🧭️direction/🟦️.ts";
import { authoredPackageExportAuthority } from "../../🕸️dependencies/🧭️direction/🏗️construction/🟦️.ts";
import fixture from "../🧫️fixtures/🧱️dependency-selectors/🔣️.json";
import schema from "../../🧬️schema/🧱️dependency-selectors/🔣️.json";

test("selector vectors satisfy the portable schema", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.graphCases.map((row) => row.id)).size).toBe(fixture.graphCases.length);
});

test("the independent verifier and the resolver agree on every selector vector", async () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  for (const row of fixture.graphCases) {
    const cwd = realpathSync(mkdtempSync(join(output, "dependency-selectors-")));
    try {
      for (const [path, content] of Object.entries(row.files)) {
        mkdirSync(dirname(join(cwd, path)), { recursive: true });
        writeFileSync(join(cwd, path), content);
      }
      const rule = dependencyDirectionRule(row.rule), expected = row.expected.map(([from, to]) => JSON.stringify([row.rule.name, from, to])).sort();
      const result = await cruise([...row.entries], { baseDir: cwd, validate: true, ruleSet: { forbidden: dependencyDirectionResolverRules([rule]) as never }, outputType: "json", tsPreCompilationDeps: true, combinedDependencies: true, doNotFollow: { path: "(^|/)node_modules(/|$)" } }, { bustTheCache: true });
      const graph = (typeof result.output === "string" ? JSON.parse(result.output) : result.output) as { modules: { source: string }[]; summary: { violations: { from: string; to: string; rule: { name: string } }[] } };
      expect(graph.summary.violations.map((edge) => JSON.stringify([edge.rule.name, edge.from, edge.to])).sort(), `${row.id} resolver`).toEqual(expected);
      const scope = { workspaceRoots: ["case", "r", "ok", "other"], workspacePackages: [], excludedPaths: [], nonFollowedPaths: ["(^|/)node_modules(/|$)"], expectedSources: graph.modules.map((module) => module.source).filter((source) => !source.includes("node_modules")).sort() };
      const edges = dependencyDirectionEdges(graph, [rule], scope).map((edge) => JSON.stringify([edge.rule, edge.from, edge.to])).sort();
      expect(edges, `${row.id} verifier`).toEqual(expected);
    } finally {
      rmSync(cwd, { recursive: true, force: true });
    }
  }
});

test("rules the verifier cannot enforce independently are refused", () => {
  for (const row of fixture.refusals) expect(() => dependencyDirectionRule(row.rule), row.id).toThrow();
});

test("the source inventory ignores a link whose target is gone and still refuses linked sources", () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const cwd = realpathSync(mkdtempSync(join(output, "dependency-inventory-")));
  const scope = { workspaceRoots: ["case"], workspacePackages: [], excludedPaths: [], nonFollowedPaths: ["(^|/)node_modules(/|$)"], expectedSources: [] };
  try {
    mkdirSync(join(cwd, "case"), { recursive: true });
    writeFileSync(join(cwd, "case/real.ts"), "export {};\n");
    symlinkSync(join(cwd, "gone"), join(cwd, "case/.stale-install-link"), "dir");
    expect(dependencyDirectionSourceInventory(cwd, ["case"], scope)).toEqual(["case/real.ts"]);
    symlinkSync(join(cwd, "gone"), join(cwd, "case/linked.ts"));
    expect(() => dependencyDirectionSourceInventory(cwd, ["case"], scope)).toThrow();
  } finally {
    rmSync(cwd, { recursive: true, force: true });
  }
});

test("resolver reports and the verifier agree on built outputs, bundler queries and resolved package specifiers", () => {
  for (const row of fixture.reportCases) {
    const rule = dependencyDirectionRule(row.rule), authority = authoredPackageExportAuthority("pkg/x", { exports: row.exports });
    const modules = [...row.modules, ...(row.extraSources ?? []).map((source) => ({ source, dependencies: [] }))];
    const dependencies = modules.reduce((count, module) => count + (module as { dependencies: unknown[] }).dependencies.length, 0);
    const report = { modules, summary: { totalCruised: modules.length, totalDependenciesCruised: dependencies, error: row.edges.length, violations: row.edges.map(([from, to]) => ({ from, to, rule: { name: row.rule.name } })), ruleSetUsed: { forbidden: [row.rule] } } };
    const scope = { workspaceRoots: ["pkg"], workspacePackages: [{ name: "@semio-tech/x", owner: "pkg/x", ...authority }], excludedPaths: ["(^|/)dist(/|$)"], nonFollowedPaths: ["(^|/)node_modules(/|$)"], expectedSources: modules.map((module) => module.source).sort() };
    if (row.accept) expect(dependencyDirectionEdges(report, [rule], scope).map((edge) => [edge.from, edge.to]), row.id).toEqual(row.edges);
    else expect(() => dependencyDirectionEdges(report, [rule], scope), row.id).toThrow();
  }
});
