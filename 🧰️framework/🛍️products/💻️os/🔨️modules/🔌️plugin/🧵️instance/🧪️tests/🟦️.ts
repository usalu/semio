/** 🧵️ The acyclic partition of the instance use-case crates: the closed corpus `🧫️fixtures/🔣️.json` is read against the Rust staging sources, and the rank order is validated with the independent third-party `graphlib` (`alg.isAcyclic`, `alg.topsort`) instead of our own walk. */
import { expect, test } from "bun:test";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { alg, Graph } from "graphlib";
import corpus from "./🧫️fixtures/🔣️.json";

const plugin = resolve(import.meta.dir, "../..");
const COMPOSER = new Set(corpus.composerFiles);

/** 🧹️ Blanks comments, strings and char literals (newlines kept) so braces and calls are read from code only. */
function mask(text: string): string {
  const blank = (s: string): string => s.replace(/[^\n]/gu, " ");
  return text
    .replace(/\/\*[\s\S]*?\*\//gu, blank)
    .replace(/\/\/[^\n]*/gu, blank)
    .replace(/r(#*)"[\s\S]*?"\1/gu, blank)
    .replace(/"(?:[^"\\\n]|\\[\s\S])*"/gu, blank)
    .replace(/'(?:\\.[^']*|[^\\'])'/gu, blank);
}

function rustFiles(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (name === "📦️packages" || name === "🧪️tests" || name === "node_modules") continue;
    if (statSync(path).isDirectory()) out.push(...rustFiles(path));
    else if (name.endsWith(".rs")) out.push(path);
  }
  return out;
}

/** 🔎️ Every `impl ... <Type><'x, A, M, X> { ... }` block of `text` whose self type is one of `types`, as [type, body]. */
function implBlocks(text: string, types: readonly string[]): [string, string][] {
  const code = mask(text);
  const head = new RegExp(`\\bimpl\\s*<[^{;]*?>\\s*(${types.join("|")})\\s*<`, "gu");
  const out: [string, string][] = [];
  for (let m = head.exec(code); m; m = head.exec(code)) {
    const open = code.indexOf("{", m.index);
    let depth = 0;
    let end = open;
    for (; end < code.length; end++) {
      if (code[end] === "{") depth++;
      else if (code[end] === "}" && --depth === 0) break;
    }
    out.push([m[1], code.slice(open, end + 1)]);
  }
  return out;
}

function derivedBridges(): Set<string> {
  const byType = new Map<string, string>();
  const bySnake = new Map<string, string>();
  for (const c of corpus.crates) {
    byType.set(c.ctx, c.key);
    if (c.ref) byType.set(c.ref, c.key);
    bySnake.set(c.snake, c.key);
  }
  const found = new Set<string>();
  for (const crate of corpus.crates) {
    for (const folder of crate.files) {
      const root = join(plugin, folder);
      const files = statSync(root).isDirectory() ? rustFiles(root) : [];
      const single = join(root, "🦀️.rs");
      for (const file of files.length ? files : [single]) {
        for (const [type, body] of implBlocks(readFileSync(file, "utf8"), [...byType.keys()])) {
          const owner = byType.get(type)!;
          for (const m of body.matchAll(/\bself\s*\.\s*(?:as_([a-z_]+)|([a-z_]+)_ref)\s*\(\s*\)/gu)) {
            const callee = bySnake.get(m[1] ?? m[2]);
            if (callee && callee !== owner) found.add(`${owner}>${callee}`);
          }
        }
      }
    }
  }
  return found;
}

test("the declared bridges form an acyclic graph that the rank order topologically sorts, checked with graphlib", () => {
  const graph = new Graph({ directed: true });
  for (const key of corpus.rank) graph.setNode(key);
  for (const [from, to] of corpus.bridges) graph.setEdge(from, to);
  expect(alg.isAcyclic(graph)).toBe(true);
  const order = alg.topsort(graph);
  const rank = new Map(corpus.rank.map((key, index) => [key, index] as const));
  for (const [from, to] of corpus.bridges) expect(rank.get(from)!, `${from} calls ${to}`).toBeGreaterThan(rank.get(to)!);
  expect(order.length).toBe(corpus.rank.length);
});

test("the bridge calls of the Rust bundles are exactly the declared edges and never point up", () => {
  const found = derivedBridges();
  const declared = new Set(corpus.bridges.map(([from, to]) => `${from}>${to}`));
  expect([...found].sort()).toEqual([...declared].sort());
  const rank = new Map(corpus.rank.map((key, index) => [key, index] as const));
  for (const edge of found) {
    const [from, to] = edge.split(">");
    expect(rank.get(from)!, edge).toBeGreaterThan(rank.get(to)!);
  }
});

test("no inherent impl of VcsArtifactApp exists outside the composer files", () => {
  const offenders: string[] = [];
  for (const file of rustFiles(plugin)) {
    const rel = relative(plugin, file);
    if (COMPOSER.has(rel) || rel.startsWith("🧪️")) continue;
    if (/\bimpl\s*<[^{;]*?>\s*VcsArtifactApp\s*</u.test(mask(readFileSync(file, "utf8")))) offenders.push(rel);
  }
  expect(offenders).toEqual([]);
});

test("every bundle field names a field of the composer struct", () => {
  const composer = mask(readFileSync(join(plugin, "🧵️instance/🦀️.rs"), "utf8"));
  const start = composer.indexOf("pub struct VcsArtifactApp");
  const body = composer.slice(composer.indexOf("{", start), composer.indexOf("\n}\n", start));
  const fields = new Set([...body.matchAll(/\bpub\(crate\)\s+([a-z_]+)\s*:/gu)].map(m => m[1]));
  expect(fields.size).toBeGreaterThan(15);
  const bundles = rustFiles(plugin).filter(file => file.includes("🧺️bundle") || file.endsWith("🧩️extensions/🦀️.rs"));
  expect(bundles.length).toBeGreaterThanOrEqual(8);
  for (const file of bundles) {
    const code = mask(readFileSync(file, "utf8"));
    for (const m of code.matchAll(/\bpub struct (?:[A-Za-z]+)(?:Ctx|Ref)<[^{]*\{([^}]*)\}/gu)) {
      for (const f of m[1].matchAll(/\bpub\s+([a-z_]+)\s*:/gu)) {
        if (f[1] === "extension") continue;
        expect(fields.has(f[1]), `${relative(plugin, file)} borrows ${f[1]}`).toBe(true);
      }
    }
  }
});
