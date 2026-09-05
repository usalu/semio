import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { transformSync } from "esbuild";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";
import { isDiscoverySkipDirectory } from "../../🔍️discovery/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as {
  schemaVersion: number;
  skipped: string[];
  admitted: string[];
  taxonomyDirectoryKind: string;
  execution: { target: string; command: string; launchName: string; launchCommand: string; launchGroup: string; launchOrder: number };
};
const taxonomy = JSON.parse(readFileSync(join(repoRoot, library, "🔣️taxonomy.json"), "utf8"));
const discoverySource = readFileSync(join(repoRoot, library, "🔍️discovery/🟦️.ts"), "utf8");
const discovery = ts.createSourceFile("discovery.ts", discoverySource, ts.ScriptTarget.Latest, true);
const declaration = (name: string): string => {
  const rows = discovery.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name);
  if (rows.length !== 1) throw new Error(`Expected exactly one ${name} implementation`);
  return rows[0]!.getText(discovery);
};
const constant = (name: string): string => {
  const rows = discovery.statements.filter(ts.isVariableStatement).filter((node) => node.declarationList.declarations.some((row) => row.name.getText(discovery) === name));
  if (rows.length !== 1) throw new Error(`Expected exactly one ${name} constant`);
  return rows[0]!.getText(discovery);
};
const code = `${constant("DISCOVERY_SKIP_DIRS")}\n${constant("CARGO_TARGET_DIR_PATTERN")}\n${declaration("isDiscoverySkipDirectory").replace(/^export\s+/u, "")}\nreturn isDiscoverySkipDirectory;`;
const compilers = (): string[] => [new Bun.Transpiler({ loader: "ts" }).transformSync(code), transformSync(code, { loader: "ts", target: "es2022" }).code];

test("discovery skips every Cargo target root and admits every look-alike, agreeing with the taxonomy slug oracle", () => {
  expect(vector.schemaVersion).toBe(1);
  const slug = new RegExp(taxonomy.semanticDirectoryKinds[vector.taxonomyDirectoryKind].slugPattern, "u");
  for (const name of vector.skipped) expect(isDiscoverySkipDirectory(name)).toBe(true);
  for (const name of vector.admitted) expect(isDiscoverySkipDirectory(name)).toBe(false);
  for (const name of [...vector.skipped, ...vector.admitted].filter((row) => row.startsWith("target-"))) expect(isDiscoverySkipDirectory(name)).toBe(slug.test(name));
  for (const compiled of compilers()) {
    const implementation = new Function(compiled)() as (name: string) => boolean;
    for (const name of vector.skipped) expect(implementation(name)).toBe(true);
    for (const name of vector.admitted) expect(implementation(name)).toBe(false);
  }
  const walkSites = discoverySource.split("\n").filter((line) => /DISCOVERY_SKIP_DIRS\.has\(|SEMANTIC_SKIP_DIRS\.has\(/u.test(line) && !/function isDiscoverySkipDirectory/u.test(line) && !/isDiscoverySkipDirectory\(|CARGO_TARGET_DIR_PATTERN/u.test(line));
  expect(walkSites).toEqual([]);
  console.log("[DEBUG] Cargo target discovery skip proof", JSON.stringify({ skipped: vector.skipped.length, admitted: vector.admitted.length }));
});

test("registers the Cargo target discovery skip gate through Nx and both launch catalogs", () => {
  const expected = vector.execution;
  const project = JSON.parse(readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(repoRoot, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
  }
});
