/** ⏱️ Taxonomy scaling laws over `../🧫️fixtures/⏱️taxonomy-scaling-laws/🔣️.json`: every linear-time replacement agrees exactly with the
 * quadratic original it replaced. Oracles: Buffer byte order for `utf8Compare`; picomatch (third-party glob) for the fixed-contract literal-leaf
 * prefilter; the original `/^\s*\[[^\]]+\]:\s*(\S+)/gmu` regex for link reference definitions; naive rescans for edits and subtrees. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { basename, dirname, posix, resolve } from "node:path";
import picomatch from "picomatch";
import ts from "typescript";
import { normalizeRelative, utf8Compare } from "../../🧹️normalization/🛣️path/🟦️.ts";
import { createTaxonomyPathMatcher } from "../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts";
import { normalizationEngineDeclarations } from "../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";

type Fixture = { schemaVersion: number; utf8Order: { left: string; right: string; sign: number }[]; literalLeafPaths: string[]; markdownReferenceDefinitions: { content: string; values: string[] }[] };
const library = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(library, "🧪️tests/🧫️fixtures/⏱️taxonomy-scaling-laws/🔣️.json"), "utf8")) as Fixture;
const taxonomy = JSON.parse(readFileSync(resolve(library, "🔣️taxonomy.json"), "utf8")) as { fixedFilenameContracts: Record<string, { pathPattern: string }>; fixedDirectoryContracts: Record<string, { pathPattern: string }> };
const syntax = ts.createSourceFile("normalization.ts", normalizationEngineDeclarations(resolve(library, "🧹️normalization")), ts.ScriptTarget.Latest, true);

/** 🧬️ Compiles the named production declarations (functions and module constants) of the normalizer into one callable closure. */
function production<T>(names: readonly string[], entry: string): T {
  const wanted = new Set(names);
  const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? wanted.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => wanted.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  return new Function("basename", "dirname", "posix", "utf8Compare", "normalizeRelative", `${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn ${entry};`)(basename, dirname, posix, utf8Compare, normalizeRelative) as T;
}

/** 🎲️ A small deterministic generator so property laws are reproducible. */
function random(seed: number): () => number {
  let state = seed;
  return () => { state = (state * 1103515245 + 12345) & 0x7fffffff; return state / 0x80000000; };
}

test("fixture is closed and versioned", () => {
  expect(fixture.schemaVersion).toBe(1);
  expect(fixture.utf8Order.length).toBeGreaterThan(0);
});

test("utf8Compare orders exactly as UTF-8 bytes without allocating", () => {
  for (const row of fixture.utf8Order) {
    expect(utf8Compare(row.left, row.right), JSON.stringify(row)).toBe(row.sign);
    expect(Math.sign(Buffer.compare(Buffer.from(row.left), Buffer.from(row.right)))).toBe(row.sign);
  }
  const next = random(7), pool = ["a", "/", "-", "z", "é", "️", "", "￿", "😀", "🧬", "\u{10ffff}", "퟿", "0", "~"];
  for (let round = 0; round < 20_000; round++) {
    const word = (): string => Array.from({ length: 1 + Math.floor(next() * 6) }, () => pool[Math.floor(next() * pool.length)]!).join("");
    const left = word(), right = word();
    expect(utf8Compare(left, right)).toBe(Math.sign(Buffer.compare(Buffer.from(left), Buffer.from(right))));
  }
});

test("the fixed-contract literal leaf never excludes a contract the glob oracle matches", () => {
  const leaf = production<(pattern: string) => string | null>(["fixedContractLiteralLeaf"], "fixedContractLiteralLeaf");
  const patterns = [...Object.values(taxonomy.fixedFilenameContracts), ...Object.values(taxonomy.fixedDirectoryContracts)].map((contract) => contract.pathPattern);
  const paths = [...fixture.literalLeafPaths, ...patterns.filter((pattern) => !/[*?[\]{}]/u.test(pattern))];
  let checked = 0;
  for (const pattern of patterns) {
    const matches = picomatch(pattern.normalize("NFC"), { dot: true, nobrace: true, noextglob: true }), literal = leaf(pattern);
    for (const path of paths) if (matches(path.normalize("NFC"))) { checked++; expect(literal === null || literal === posix.basename(path.normalize("NFC")), `${pattern} ${path}`).toBe(true); }
  }
  expect(checked).toBeGreaterThan(0);
});

test("link reference definitions are found in one linear pass exactly as the original regex finds them", () => {
  const scan = production<(content: string) => { value: string; start: number; end: number; structuredLocation: string }[]>(["markdownReferenceDefinitionTokens", "MARKDOWN_WHITESPACE_RUN", "MARKDOWN_TEXT_RUN", "MARKDOWN_LINE_TERMINATOR", "requireTaxonomyValue", "lineLocation", "indexedLineContent", "indexedLineStarts", "regexTokens"], "{ scan: markdownReferenceDefinitionTokens, original: (content) => regexTokens(content, 'markdown', 'markdown-link', [/^\\s*\\[[^\\]]+\\]:\\s*(\\S+)/gmu]) }") as unknown as { scan: (content: string) => unknown[]; original: (content: string) => unknown[] };
  for (const row of fixture.markdownReferenceDefinitions) {
    expect((scan.scan(row.content) as { value: string }[]).map((token) => token.value), JSON.stringify(row.content)).toEqual(row.values);
    expect([...row.content.matchAll(/^\s*\[[^\]]+\]:\s*(\S+)/gmu)].map((match) => match[1])).toEqual(row.values);
  }
  const next = random(11), pieces = ["[", "]", ":", " ", "\n", "\r", "\r\n", " ", "a", "b/c.md", "\t", "x"];
  for (let round = 0; round < 5_000; round++) {
    const content = Array.from({ length: Math.floor(next() * 30) }, () => pieces[Math.floor(next() * pieces.length)]!).join("");
    expect(scan.scan(content), JSON.stringify(content)).toEqual(scan.original(content));
  }
});

test("edits are applied in one pass exactly as sequential splicing applies them, overlaps and mismatches included", () => {
  type Edit = { path: string; structuredLocation: string; oldValue: string; newValue: string };
  const apply = production<(content: string, edits: readonly Edit[]) => string>(["applyEditsToContent", "referenceEditOffset"], "applyEditsToContent");
  const sequential = (content: string, edits: readonly Edit[]): string => {
    let result = content;
    const offset = (edit: Edit): number => Number(edit.structuredLocation.match(/@(\d+)$/)![1]);
    for (const edit of [...edits].sort((a, b) => offset(b) - offset(a) || b.structuredLocation.localeCompare(a.structuredLocation))) {
      const start = offset(edit);
      if (result.slice(start, start + edit.oldValue.length) !== edit.oldValue) throw new Error(`Reference edit preimage mismatch at ${edit.path}:${edit.structuredLocation}`);
      result = result.slice(0, start) + edit.newValue + result.slice(start + edit.oldValue.length);
    }
    return result;
  };
  const outcome = (run: () => string): string => { try { return run(); } catch (error) { return `!${(error as Error).message}`; } };
  const next = random(13);
  for (let round = 0; round < 10_000; round++) {
    const content = Array.from({ length: 4 + Math.floor(next() * 30) }, () => "ab😀c/"[Math.floor(next() * 6)]).join("");
    const edits = Array.from({ length: Math.floor(next() * 5) }, (_, index): Edit => {
      const start = Math.floor(next() * content.length), end = Math.min(content.length, start + Math.floor(next() * 4));
      return { path: "p", structuredLocation: `l${index}@${start}`, oldValue: next() < 0.05 ? "zz" : content.slice(start, end), newValue: `N${index}` };
    });
    expect(outcome(() => apply(content, edits))).toBe(outcome(() => sequential(content, edits)));
  }
});

test("entry subtrees are answered from one sorted index in insertion order exactly as a full rescan", () => {
  const subtree = production<<T>(entries: ReadonlyMap<string, T>, root: string) => T[]>(["entrySubtree", "entrySubtreeIndexes"], "entrySubtree");
  const next = random(17), segments = ["a", "a-b", "b", "😀", "a.b", "z"];
  for (let round = 0; round < 500; round++) {
    const entries = new Map<string, number>();
    for (let index = 0; index < 40; index++) entries.set(Array.from({ length: 1 + Math.floor(next() * 3) }, () => segments[Math.floor(next() * segments.length)]!).join("/"), index);
    for (const root of ["a", "a-b", "😀", "a/b", "q"]) expect(subtree(entries, root)).toEqual([...entries.entries()].filter(([path]) => path === root || path.startsWith(`${root}/`)).map(([, value]) => value));
  }
});

test("generator mutation queries answer exactly what full scans of pathsOverlap and the path matcher answer", () => {
  type Index = { overlapping: (root: string, admit?: (path: string) => boolean) => boolean; matching: (pattern: string, matches: (path: string, pattern: string) => boolean) => boolean };
  const api = production<{ index: (paths: Iterable<string>) => Index; overlap: (left: string, right: string) => boolean; subtree: (values: readonly string[], prefix: string) => Iterable<string> }>(["mutationIndex", "sortedSubtree", "pathsOverlap"], "{ index: mutationIndex, overlap: pathsOverlap, subtree: sortedSubtree }");
  const next = random(41), segments = ["a", "a-b", "a.b", "b", "😀", "e\u0301", "é", "z", "a0"], patterns = ["a/**", "a/*.b", "**/b", "a-b/**/z", "😀/*", "é/**", "a/b", "*"];
  const matcher = createTaxonomyPathMatcher();
  for (let round = 0; round < 400; round++) {
    const paths = Array.from({ length: 1 + Math.floor(next() * 12) }, () => Array.from({ length: 1 + Math.floor(next() * 3) }, () => segments[Math.floor(next() * segments.length)]!).join("/"));
    const index = api.index(paths), sorted = [...new Set(paths)].sort();
    for (const root of ["", "a", "a/b", "😀", "é", "e\u0301", "a-b", "z/a"]) {
      expect(index.overlapping(root), JSON.stringify([paths, root])).toBe(paths.some((path) => api.overlap(path, root)));
      const admit = (path: string): boolean => !path.endsWith("b");
      expect(index.overlapping(root, admit), JSON.stringify([paths, root, "admit"])).toBe(paths.some((path) => api.overlap(path, root) && admit(path)));
      if (root) expect([...api.subtree(sorted, root)]).toEqual(sorted.filter((path) => path === root || path.startsWith(`${root}/`)));
    }
    for (const pattern of patterns) expect(index.matching(pattern, matcher.matches), JSON.stringify([paths, pattern])).toBe(paths.some((path) => matcher.matches(path, pattern)));
  }
});

test("unsupported path literals are found in one linear pass exactly as the original two regexes find them", () => {
  const scan = production<{ quoted: (content: string) => [number, string, string][]; paths: (content: string) => [number, string, string][] }>(["quotedLiteralMatches", "pathLiteralMatches", "PATH_RUN_EXCLUDED", "PATH_LOOKAHEAD", "PATH_PREFIXES", "REGEX_WHITESPACE", "LINE_TERMINATORS"], "{ quoted: quotedLiteralMatches, paths: pathLiteralMatches }");
  const quoted = /"((?:\\.|[^"\\\r\n])+)"|'((?:\\.|[^'\\\r\n])+)'|`((?:\\.|[^`\\\r\n])+)`/gu, paths = /(?:^|[\s(=,:])((?:\.\.?\/|\/)?[^\s"'`()\],;]+\/[^\s"'`()\],;]+|[A-Za-z0-9_.@-]+\.[A-Za-z0-9.]+)(?=$|[\s"'`),;\]])/gmu;
  const oracle = (pattern: RegExp, content: string): [number, string, string][] => [...content.matchAll(pattern)].map((match) => [match.index!, match[0], (match[1] ?? match[2] ?? match[3])!]);
  const next = random(43), pieces = ["\"", "'", "`", "\\", "/", ".", "a", "B", "9", "_", "@", "-", " ", "\t", "\n", "\r", " ", "(", ")", "=", ",", ":", ";", "]", "[", "😀", " ", "x/y", "./", "../", "a.b", "..", "é"];
  for (let round = 0; round < 20_000; round++) {
    const content = Array.from({ length: Math.floor(next() * 24) }, () => pieces[Math.floor(next() * pieces.length)]!).join("");
    expect(scan.quoted(content), JSON.stringify(content)).toEqual(oracle(quoted, content));
    expect(scan.paths(content), JSON.stringify(content)).toEqual(oracle(paths, content));
  }
  for (const content of [`x${":a".repeat(4000)}(`, `x${"=b.c".repeat(3000)}]`, `${" a:b/".repeat(3000)}(`, `k:${"a".repeat(8000)}.b(`]) expect(scan.paths(content)).toEqual(oracle(paths, content));
});
