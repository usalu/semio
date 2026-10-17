import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { Database } from "bun:sqlite";
import glob from "fast-glob";
import Parser from "web-tree-sitter";
import { runInNewContext } from "node:vm";
import ts from "typescript";
import fixture from "./🧫️fixtures/🔣️.json";

const plugins = resolve(import.meta.dir, "../..");
const parts = (file: string): string[] => file.split(/[\\/]/u);
const opaque = new Set(["node_modules", "target", "dist", ".git", ".nx", "🗑️generated", "🤖️generated"]);
const walk = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap(entry => entry.isDirectory() && !opaque.has(entry.name) ? walk(join(directory, entry.name)) : entry.isFile() ? [join(directory, entry.name)] : []);
const files = fixture.owners.flatMap(owner => walk(join(plugins, owner)));
const accepted = (file: string): boolean => {
  const path = parts(file), tests = path.indexOf("🧪️tests"), examples = path.indexOf("🧫️fixtures");
  return tests >= 0 && examples > tests && !path.slice(0, examples).some(part => ["🧬️schema", "🖼️assets"].includes(part));
};

const cfgWithoutTests = (expression: string): boolean | undefined => {
  if (expression === "test") return false;
  const group = /^(all|any|not)\((.*)\)$/u.exec(expression);
  if (!group) return undefined;
  const args: string[] = []; let depth = 0, quoted = false, start = 0;
  for (let i = 0; i < group[2]!.length; i++) {
    const character = group[2]![i];
    if (character === '"') quoted = !quoted;
    if (!quoted) {
      if (character === "(") depth++;
      if (character === ")") depth--;
      if (character === "," && depth === 0) { args.push(group[2]!.slice(start, i)); start = i + 1; }
    }
  }
  if (group[2]!.slice(start)) args.push(group[2]!.slice(start));
  const values = args.map(cfgWithoutTests);
  if (group[1] === "not") return values.length === 1 && values[0] !== undefined ? !values[0] : undefined;
  if (group[1] === "all") return values.includes(false) ? false : values.includes(undefined) ? undefined : true;
  return values.includes(true) ? true : values.includes(undefined) ? undefined : false;
};

test("plugin fixture ownership agrees with an independent SQLite path oracle", () => {
  const db = new Database(":memory:");
  try {
    db.run("CREATE TABLE component(position INTEGER, name TEXT)");
    for (const row of fixture.cases) {
      db.run("DELETE FROM component");
      parts(row.path).forEach((name, position) => db.run("INSERT INTO component VALUES (?, ?)", [position, name]));
      const oracle = db.query("SELECT EXISTS(SELECT 1 FROM component t, component f WHERE t.name='🧪️tests' AND f.name='🧫️fixtures' AND t.position<f.position AND NOT EXISTS(SELECT 1 FROM component p WHERE p.position<f.position AND p.name IN ('🧬️schema','🖼️assets'))) AS accepted").get() as { accepted: number };
      expect(accepted(row.path)).toBe(row.accepted);
      expect(Boolean(oracle.accepted)).toBe(row.accepted);
    }
  } finally { db.close(); }
});

test("current plugin example folders belong beneath actual neutral tests owners", () => {
  const actual = files.filter(file => parts(file).includes("🧫️fixtures")).sort();
  const oracle = fixture.owners.flatMap(owner => glob.sync("**/🧫️fixtures/**/*", { cwd: join(plugins, owner), absolute: true, onlyFiles: true, ignore: [...opaque].map(name => `**/${name}/**`) })).map(file => resolve(file)).sort();
  expect(actual).toEqual(oracle);
  expect(actual.length).toBeGreaterThanOrEqual(fixture.minimumExamples);
  for (const file of actual) {
    expect(accepted(realpathSync(file))).toBe(true);
    expect([...file].length).toBeLessThanOrEqual(256);
    expect(file.length).toBeLessThanOrEqual(256);
  }
}, 120_000);

test("native plugin example includes retain physical test-only source ownership", async () => {
  let count = 0;
  let parser: Parser | undefined;
  try {
    for (const file of files.filter(file => file.endsWith(".rs"))) {
      const source = readFileSync(file, "utf8");
      for (const match of source.matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu)) {
        const target = resolve(dirname(file), match[1]!);
        expect(existsSync(target), `${file} => ${target}`).toBe(true);
        const coordinate = relative(plugins, realpathSync(target));
        if (!coordinate.startsWith("..") && !isAbsolute(coordinate) && fixture.owners.includes(parts(coordinate)[0]!)) expect(accepted(realpathSync(target)), `${file} => ${target}`).toBe(true);
        if (!parts(file).includes("🧪️tests")) {
          if (!parser) {
            await Parser.init(); parser = new Parser();
            parser.setLanguage(await Parser.Language.load(resolve(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", import.meta.dir)), "out/tree-sitter-rust.wasm")));
          }
          const tree = parser.parse(source)!;
          try {
            let node: Parser.SyntaxNode | null = tree.rootNode.descendantForIndex(match.index!);
            let testing = false;
            while (node) {
              let previous = node.previousNamedSibling;
              while (previous?.type === "attribute_item") {
                const cfg = /^#\[cfg\((.*)\)\]$/u.exec(previous.text.replace(/\s/gu, ""));
                testing ||= Boolean(cfg && cfgWithoutTests(cfg[1]!) === false);
                previous = previous.previousNamedSibling;
              }
              node = node.parent;
            }
            expect(testing, file).toBe(true);
          } finally { tree.delete(); }
        }
        count++;
      }
    }
    expect(count).toBeGreaterThan(50);
  } finally { parser?.delete(); }
}, 120_000);

test("direct plugin TypeScript example imports retain physical coordinates", () => {
  let count = 0;
  for (const file of files.filter(file => file.endsWith(".ts") || file.endsWith(".tsx"))) {
    for (const match of readFileSync(file, "utf8").matchAll(/(?:from\s*|import\(\s*)["']([^"']*🧫️fixtures[^"']*)["']/gu)) {
      const target = resolve(dirname(file), match[1]!);
      const coordinate = relative(plugins, target);
      if (!coordinate.startsWith("..") && !isAbsolute(coordinate) && fixture.owners.includes(parts(coordinate)[0]!)) {
        expect(parts(file).includes("🧪️tests"), file).toBe(true);
        expect(match[1]!.startsWith(".")).toBe(true);
        expect(existsSync(target), `${file} => ${target}`).toBe(true);
        expect(accepted(realpathSync(target)), `${file} => ${target}`).toBe(true);
        count++;
      }
    }
  }
  expect(count).toBeGreaterThan(15);
}, 120_000);

test("Rust test-only predicates agree with independent JavaScript truth tables", () => {
  for (const row of fixture.cfgCases) {
    const expression = row.expression.replaceAll('feature="probe"', "probe");
    const withoutTests = [false, true].map(probe => runInNewContext(expression, { test: false, probe, all: (...values: boolean[]) => values.every(Boolean), any: (...values: boolean[]) => values.some(Boolean), not: (value: boolean) => !value }));
    expect(cfgWithoutTests(row.expression) === false).toBe(row.testOnly);
    expect(withoutTests.every(value => !value)).toBe(row.testOnly);
  }
});

for (const path of fixture.retiredTrialSchemas) test(`plugin examples retire whole-trial schema ${path}`, () => {
  expect(existsSync(join(plugins, path)), path).toBe(false);
});

test("original example generators default to physical test owners", () => {
  let coordinates = 0;
  for (const owner of fixture.computedOwners) {
    const file = join(plugins, owner), source = ts.createSourceFile(file, readFileSync(file, "utf8"), ts.ScriptTarget.Latest, true);
    const declarations = new Map<string, ts.Expression>();
    const gather = (node: ts.Node): void => { if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer) declarations.set(node.name.text, node.initializer); ts.forEachChild(node, gather); };
    gather(source);
    const evaluate = (node: ts.Expression, seen = new Set<string>()): string | undefined => {
      if (ts.isStringLiteralLike(node)) return node.text;
      if (node.getText(source) === "import.meta.dir") return dirname(file);
      if (ts.isIdentifier(node)) { const declaration = declarations.get(node.text); if (!declaration || seen.has(node.text)) return; return evaluate(declaration, new Set([...seen, node.text])); }
      if (!ts.isCallExpression(node)) return;
      const name = node.expression.getText(source);
      if (name === "dirname" && node.arguments[0]?.getText(source) === "fileURLToPath(import.meta.url)") return dirname(file);
      if (!["join", "resolve", "dirname"].includes(name)) return;
      const values = node.arguments.map(argument => evaluate(argument, seen));
      if (values.some(value => value === undefined)) return;
      return name === "dirname" ? dirname(values[0]!) : name === "join" ? join(...values as string[]) : resolve(...values as string[]);
    };
    const inspect = (node: ts.Node): void => {
      if (ts.isCallExpression(node) && node.arguments.some(argument => ts.isStringLiteralLike(argument) && argument.text.includes("🧫️fixtures"))) {
        const target = evaluate(node);
        if (target) { expect(accepted(target), `${owner}: ${node.getText(source)}`).toBe(true); expect(existsSync(target), target).toBe(true); coordinates++; }
      }
      ts.forEachChild(node, inspect);
    };
    inspect(source);
  }
  expect(coordinates).toBeGreaterThan(20);
}, 120_000);

test("original Cargo-manifest example coordinates remain test owned", () => {
  const repository = resolve(plugins, "../..");
  let readers = 0;
  for (const row of fixture.manifestReaders) {
    const file = join(repository, row.path), manifest = join(repository, row.manifest);
    expect(existsSync(manifest), row.manifest).toBe(true);
    for (const match of readFileSync(file, "utf8").matchAll(/concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu)) {
      const target = resolve(dirname(manifest) + match[1]!);
      expect(existsSync(target), `${row.path} => ${target}`).toBe(true);
      expect(accepted(realpathSync(target)), target).toBe(true);
      readers++;
    }
  }
  expect(readers).toBeGreaterThan(30);
});
