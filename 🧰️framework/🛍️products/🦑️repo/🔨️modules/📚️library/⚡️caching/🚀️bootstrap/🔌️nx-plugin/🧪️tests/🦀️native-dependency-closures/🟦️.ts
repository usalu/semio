/** 🦀️ Native dependency closure law over `../🧫️fixtures/🦀️native-dependency-closures/🔣️.json`: the plugin's `nativeDependencyRoots`, which
 * shares every non-root crate closure across the roots of one graph construction, answers every root (with and without tests, in sequence,
 * through one shared memo) exactly what the original per-root visited-set traversal from Git `HEAD` answers, including thrown errors.
 * Oracle: the pre-change implementation itself, compiled from the fixture's pinned `oracleRevision`. */
import { expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import ts from "typescript";
import { memoizedObservedComputationV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";
import { readInferenceGraphSourceV1 } from "../../../../../🏗️build/🕸️graph/📥️inference/🧪️tests/📖️source/🟦️.ts";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🦀️native-dependency-closures/🔣️.json"), "utf8")) as { schemaVersion: number; oracleRevision: string; manifests: Record<string, string>; roots: string[] };
const oraclePath = resolve(import.meta.dir, "../../../../../🟨️.mjs"), pluginPath = resolve(import.meta.dir, "../../../../../🏗️build/🕸️graph/📥️inference/🟨️.mjs"), repoRoot = execFileSync("git", ["rev-parse", "--show-toplevel"], { cwd: import.meta.dir, encoding: "utf8" }).trim();
const toml = createRequire(pluginPath)("@iarna/toml") as { parse: (text: string) => unknown };
type Roots = (root: string, workspaceRoot: string, tests?: boolean, cache?: Map<string, unknown>, closures?: Map<string, unknown>) => readonly string[];

/** 🧬️ Compiles `nativeDependencyRoots` and its helpers from one plugin source with plain filesystem and TOML dependencies. */
function implementation(source: string): Roots {
  const syntax = ts.createSourceFile("plugin.mjs", source, ts.ScriptTarget.Latest, true), wanted = new Set(["nativeDependencyRoots", "nativeDependencies", "buildPath", "nxPath", "NATIVE_DIRECTORY_CLOSURES", "NATIVE_DIRECTORY_NAMES"]);
  const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? wanted.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => declaration.initializer !== undefined && wanted.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  return new Function("existsSync", "readToml", "join", "resolve", "dirname", "relative", "memoizedObservedComputationV1", `${code}\nreturn nativeDependencyRoots;`)(existsSync, (path: string) => toml.parse(readFileSync(path, "utf8")), join, resolve, dirname, relative, memoizedObservedComputationV1) as Roots;
}

/** 🎯️ One answer: the closure, or the error message and code a root's traversal throws. */
function answer(run: () => readonly string[]): unknown {
  try { return run(); } catch (error) { return { error: (error as Error).message, code: (error as { code?: string }).code ?? null }; }
}

test("shared crate closures answer every root exactly as the original traversal", () => {
  expect(fixture.schemaVersion).toBe(1);
  const workspace = mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir()), "native-closures-"));
  try {
    for (const [path, content] of Object.entries(fixture.manifests)) { mkdirSync(dirname(join(workspace, path)), { recursive: true }); writeFileSync(join(workspace, path), content); }
    const current = implementation(readInferenceGraphSourceV1()), original = implementation(execFileSync("git", ["show", `${fixture.oracleRevision}:${relative(repoRoot, oraclePath)}`], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 28 }));
    const cache = new Map<string, unknown>(), closures = new Map<string, unknown>();
    let compared = 0;
    for (const tests of [false, true]) for (const root of fixture.roots) {
      expect(answer(() => current(root, workspace, tests, cache, closures)), `${root} tests=${tests}`).toEqual(answer(() => original(root, workspace, tests)));
      compared++;
    }
    expect(compared).toBe(fixture.roots.length * 2);
  } finally { rmSync(workspace, { recursive: true, force: true }); }
});
