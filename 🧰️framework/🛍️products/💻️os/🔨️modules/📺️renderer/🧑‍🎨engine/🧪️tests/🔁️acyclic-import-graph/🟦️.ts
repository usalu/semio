/**
 * 🔁️ The renderer import graph stays acyclic, value, type-only and dynamic edges included. Our own strongly-connected-component scan agrees with dependency-cruiser on a portable corpus and finds no cycle in the real tree. A module's in-source `🧪️tests` are leaves of that module, exactly as the `no-circular` policy rule treats them: an edge between two production modules counts when both lie in one component, whether or not a test module closes the loop.
 * @vitest-environment node
 */
import { expect, test } from "vitest";
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve, sep } from "node:path";
import ts from "typescript";
import { cruise } from "dependency-cruiser";
import corpus from "./🧫️fixtures/🔣️.json";

type Case = { id: string; files: Record<string, string>; modules: string[] };
const rendererRoot = resolve(import.meta.dirname, "../../..");
const IN_SOURCE_TEST = /(^|\/)🧪️tests\//u;
const SPECIFIER_SUFFIXES = ["", ".ts", ".tsx", "/🟦️.ts", "/🟦️.tsx"];

const sourceFiles = (dir: string): string[] => readdirSync(dir).flatMap((name) => {
  if (name === "node_modules") return [];
  const path = join(dir, name);
  return statSync(path).isDirectory() ? sourceFiles(path) : /\.tsx?$/u.test(name) && !name.endsWith(".d.ts") ? [path] : [];
});

const portable = (root: string, file: string): string => relative(root, file).split(sep).join("/");

const cyclicModules = (root: string, files: readonly string[]): string[] => {
  const known = new Set(files);
  const graph = new Map<string, string[]>(files.map((file) => [file, ts.preProcessFile(readFileSync(file, "utf8"), true, true).importedFiles.map((entry) => entry.fileName).filter((name) => name.startsWith(".")).flatMap((name) => SPECIFIER_SUFFIXES.map((suffix) => resolve(dirname(file), name) + suffix).filter((target) => known.has(target)).slice(0, 1))]));
  const index = new Map<string, number>(), low = new Map<string, number>(), onStack = new Set<string>(), stack: string[] = [], component = new Map<string, number>();
  let counter = 0, components = 0;
  const visit = (node: string): void => {
    index.set(node, counter); low.set(node, counter); counter += 1; stack.push(node); onStack.add(node);
    for (const next of graph.get(node)!) {
      if (!index.has(next)) { visit(next); low.set(node, Math.min(low.get(node)!, low.get(next)!)); }
      else if (onStack.has(next)) low.set(node, Math.min(low.get(node)!, index.get(next)!));
    }
    if (low.get(node) !== index.get(node)) return;
    let member: string;
    do { member = stack.pop()!; onStack.delete(member); component.set(member, components); } while (member !== node);
    components += 1;
  };
  for (const file of files) if (!index.has(file)) visit(file);
  const modules = new Set<string>();
  for (const [from, targets] of graph) for (const to of targets) {
    if (component.get(from) !== component.get(to) || IN_SOURCE_TEST.test(portable(root, from)) || IN_SOURCE_TEST.test(portable(root, to))) continue;
    for (const member of component.keys()) if (component.get(member) === component.get(from) && !IN_SOURCE_TEST.test(portable(root, member))) modules.add(portable(root, member));
  }
  return [...modules].sort();
};

const oracleModules = async (root: string): Promise<string[]> => {
  const result = await cruise(["."], {
    baseDir: root,
    tsPreCompilationDeps: true,
    exclude: { path: "node_modules" },
    includeOnly: { path: "^(?!\\.\\./)" },
    ruleSet: { forbidden: [{ name: "no-circular", severity: "error", from: { pathNot: "(^|/)🧪️tests/" }, to: { circular: true, pathNot: "(^|/)🧪️tests/" } }] },
    validate: true,
  });
  const output = result.output as { summary: { violations: { type: string; from: string; cycle?: { name: string }[] }[] } };
  return [...new Set(output.summary.violations.filter((violation) => violation.type === "cycle").flatMap((violation) => [violation.from, ...(violation.cycle ?? []).map((step) => step.name)]).filter((name) => !IN_SOURCE_TEST.test(name)))].sort();
};

test("portable corpus: own scan, dependency-cruiser and expectation agree on every case", async () => {
  for (const row of corpus.cases as unknown as Case[]) {
    const dir = realpathSync(mkdtempSync(join(tmpdir(), "semio-renderer-acyclic-")));
    try {
      for (const [name, text] of Object.entries(row.files)) { mkdirSync(dirname(join(dir, name)), { recursive: true }); writeFileSync(join(dir, name), text); }
      const files = Object.keys(row.files).map((name) => resolve(dir, name));
      const expected = [...row.modules].sort();
      expect(cyclicModules(dir, files), row.id + " own").toEqual(expected);
      expect(await oracleModules(dir), row.id + " oracle").toEqual(expected);
    } finally { rmSync(dir, { recursive: true, force: true }); }
  }
}, 120000);

test("renderer has no import cycle of any edge kind outside in-source tests", async () => {
  const files = sourceFiles(rendererRoot).map((file) => resolve(file));
  expect(files.length).toBeGreaterThan(500);
  expect(cyclicModules(rendererRoot, files)).toEqual([]);
  expect(await oracleModules(rendererRoot)).toEqual([]);
}, 600000);
