/** 🧲️ Reference reach laws over `../🧫️fixtures/🧲️reference-reach/🔣️.json`: the planner's skip predicate `referenceTokenMayResolveInto`
 * is never false for a token that the production resolver resolves into the target set (oracle: the resolver itself, run exhaustively over
 * every fixture and random token, path and target combination), and a token's digest query (`referenceTokenQuery` → `referenceQueryToken`)
 * resolves exactly like the token itself, so the planner's exact digest test `referenceDigestResolvesInto` sees what token resolution sees. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { basename, dirname, posix, resolve } from "node:path";
import ts from "typescript";
import { normalizeRelative } from "../../🧹️normalization/🛣️path/🟦️.ts";
import { normalizationEngineDeclarations } from "../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";

type Token = { adapter: string; structuredLocation: string; start: number; end: number; value: string; targetValues?: string[]; physicalTargets?: string[] };
type Index = { exact: Set<string> };
const library = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(library, "🧪️tests/🧫️fixtures/🧲️reference-reach/🔣️.json"), "utf8")) as { schemaVersion: number; repositoryPaths: string[]; tokenValues: string[]; referencingPaths: string[]; movedTargetSets: string[][] };
const syntax = ts.createSourceFile("normalization.ts", normalizationEngineDeclarations(resolve(library, "🧹️normalization")), ts.ScriptTarget.Latest, true);

/** 🧬️ Compiles the named production declarations of the normalizer into one closure over the injected dependencies. */
function production<T>(names: readonly string[], entry: string, dependencies: Record<string, unknown>): T {
  const wanted = new Set(names);
  const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? wanted.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => wanted.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  return new Function(...Object.keys(dependencies), `${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn ${entry};`)(...Object.values(dependencies)) as T;
}

const api = production<{ index: (paths: Iterable<string>, repoRoot?: string, coordinateRoots?: readonly string[]) => Index; resolve: (referencePath: string, token: Token, index: Index) => string | null; reach: (targets: Iterable<string>) => unknown; may: (token: Token, reach: unknown, index: Index) => boolean; query: (token: Token) => string[]; queryToken: (query: readonly string[]) => Token }>(
  ["referenceTokenQuery", "referenceQueryToken", "referencePathIndex", "addUniqueIndex", "referenceLeafNameSets", "referenceLeafNames", "referenceLeaf", "splitTokenSuffix", "resolveReferencePath", "resolveReferenceTokenPath", "ancestorReferenceCoordinateRoot", "referenceResolutionReach", "referenceTokenMayResolveInto", "addReferenceTokenReach", "visitReferenceTokenReach", "referenceSignatureMayResolveInto"],
  "{ index: referencePathIndex, resolve: resolveReferenceTokenPath, reach: referenceResolutionReach, may: referenceTokenMayResolveInto, query: referenceTokenQuery, queryToken: referenceQueryToken }",
  { normalizeRelative, posix, dirname, basename },
);

/** 🎲️ A small deterministic generator so property laws are reproducible. */
function random(seed: number): () => number {
  let state = seed;
  return () => { state = (state * 1103515245 + 12345) & 0x7fffffff; return state / 0x80000000; };
}

/** ⚖️ Checks the law for one token against every target set: a resolution into the set implies the predicate admits the token. */
function assertReach(index: Index, referencePath: string, token: Token, targetSets: readonly (readonly string[])[]): number {
  const resolved = api.resolve(referencePath, token, index);
  expect(api.resolve(referencePath, api.queryToken(api.query(token)), index), JSON.stringify({ referencePath, token })).toBe(resolved);
  let reached = 0;
  for (const targets of targetSets) {
    if (resolved === null || !targets.includes(resolved)) continue;
    reached++;
    expect(api.may(token, api.reach(targets), index), JSON.stringify({ referencePath, token, targets, resolved })).toBe(true);
  }
  return reached;
}

test("the reach predicate admits every fixture token the resolver resolves into a moved target set", () => {
  expect(fixture.schemaVersion).toBe(1);
  const roots = [undefined, "/repo"], targetSets = fixture.movedTargetSets;
  let reached = 0;
  for (const repoRoot of roots) for (const coordinateRoots of [[], ["a"], ["docs"]]) {
    const index = api.index(fixture.repositoryPaths, repoRoot, coordinateRoots);
    for (const referencePath of fixture.referencingPaths) for (const value of [...fixture.tokenValues, ...fixture.tokenValues.map((token) => `/repo/${token}`)]) {
      reached += assertReach(index, referencePath, { adapter: "json", structuredLocation: "x@0", start: 0, end: value.length, value }, targetSets);
      reached += assertReach(index, referencePath, { adapter: "rust", structuredLocation: "x@0", start: 0, end: value.length, value, targetValues: [value, `${value}.rs`, `./${value}/mod.rs`] }, targetSets);
      reached += assertReach(index, referencePath, { adapter: "rust", structuredLocation: "x@0", start: 0, end: value.length, value, physicalTargets: [value, value.normalize("NFC")] }, targetSets);
    }
  }
  expect(reached).toBeGreaterThan(50);
});

test("the reach predicate admits every random token the resolver resolves into a random target set", () => {
  const next = random(37), segments = ["a", "b", "c.ts", "d", "e.json", "é", "é", "__init__.py", "util.py", "x.v2.md", "🟦️.ts", ".", "..", ""];
  const word = (length: number): string => Array.from({ length }, () => segments[Math.floor(next() * segments.length)]!).join("/");
  let reached = 0;
  for (let round = 0; round < 300; round++) {
    const paths = [...new Set(Array.from({ length: 12 }, () => word(1 + Math.floor(next() * 3))).map((path) => { try { return normalizeRelative(path); } catch { return ""; } }).filter(Boolean))];
    if (paths.length === 0) continue;
    const index = api.index(paths, "/repo", next() < 0.5 ? [] : [paths[0]!.split("/")[0]!]);
    const targetSets = Array.from({ length: 4 }, () => paths.filter(() => next() < 0.3));
    for (let draw = 0; draw < 40; draw++) {
      const value = `${next() < 0.2 ? "./" : next() < 0.2 ? "../" : next() < 0.1 ? "/repo/" : ""}${word(1 + Math.floor(next() * 3))}${next() < 0.1 ? "#x" : ""}`;
      const referencePath = paths[Math.floor(next() * paths.length)]!;
      reached += assertReach(index, referencePath, { adapter: "json", structuredLocation: "x@0", start: 0, end: value.length, value }, targetSets);
      reached += assertReach(index, referencePath, { adapter: "python", structuredLocation: "x@0", start: 0, end: value.length, value: value.replaceAll("/", ".").replace(/\.py$/u, "") }, targetSets);
    }
  }
  expect(reached).toBeGreaterThan(200);
});
