import { expect, test } from "bun:test";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import TOML from "@iarna/toml";
import { rustTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

type Mapping = Readonly<{ role: string; source: string; destination: string }>;
type Corpus = Readonly<{
  owner: string;
  package: Readonly<{ name: string; library: string; manifest: string }>;
  mappings: readonly Mapping[];
  specificFamilies: readonly Readonly<{ source: string; sha256: string; owner: string }>[];
  consumers: readonly Readonly<{ owner: string; manifest: string; source: string; root: string; boundary: "complete-package" | "language-only" }>[];
}>;
const root = resolve(import.meta.dir, "../../../../../../.."), owner = resolve(root, "🧰️framework/🔨️modules/🗣️dsl");
const read = (path: string): string => readFileSync(resolve(root, path), "utf8");
const corpus = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🧱️ownership/🔣️.json"), "utf8")) as Corpus;
const grammar = corpus.mappings.find(row => row.role === "grammar")!;
const actualGrammar = (): string => read(grammar.destination);
const tokens = (source: string): readonly string[] => rustTokens(source).map(token => token.text);
const method = (source: string, ownerType: string, name: string): Readonly<{ parameters: string; result: string; body: string }> => {
  const stream = rustTokens(source), pairs = rustTokenPairs(stream);
  for (let index = 0; index < stream.length - 2; index++) {
    if (stream[index]?.text !== "impl" || stream[index + 1]?.text !== ownerType || stream[index + 2]?.text !== "{") continue;
    const end = pairs.get(index + 2)!;
    for (let cursor = index + 3; cursor < end; cursor++) {
      if (stream[cursor]?.text !== "fn" || stream[cursor + 1]?.text !== name || stream[cursor + 2]?.text !== "(") continue;
      const argsEnd = pairs.get(cursor + 2)!, bodyStart = stream.findIndex((token, offset) => offset > argsEnd && token.text === "{");
      if (bodyStart < 0 || bodyStart > end) throw new Error(`missing ${ownerType}::${name} body`);
      const bodyEnd = pairs.get(bodyStart)!;
      return { parameters: source.slice(stream[cursor + 2]!.start, stream[argsEnd]!.end), result: source.slice(stream[argsEnd]!.end, stream[bodyStart]!.start), body: source.slice(stream[bodyStart]!.start, stream[bodyEnd]!.end) };
    }
  }
  throw new Error(`missing ${ownerType}::${name}`);
};

test("all seven current family assets stay at their actual specific owners", () => {
  for (const row of corpus.specificFamilies) {
    expect(existsSync(resolve(root, row.source)), row.source).toBe(true);
    expect(row.source.startsWith(`${row.owner}/`)).toBe(true);
    expect(row.source.startsWith(`${corpus.owner}/`)).toBe(false);
  }
});

test.each(corpus.consumers)("the actual $owner language binding directly selects the general provider", (consumer) => {
    const text = read(consumer.manifest), native = Bun.TOML.parse(text) as { dependencies?: Record<string, string | { package?: string; path?: string }> }, thirdParty = TOML.parse(text) as typeof native;
    expect(native).toEqual(thirdParty);
    const dependencies = Object.entries(native.dependencies ?? {}), kernel = dependencies.filter(([name, row]) => name === "semio-framework-os-kernel" || typeof row === "object" && row.package === "semio-framework-os-kernel");
    if (consumer.boundary === "complete-package") expect(kernel, `${consumer.manifest}: current concrete product edge`).toEqual([]);
    expect(dependencies.some(([name, row]) => name === corpus.package.name || typeof row === "object" && row.package === corpus.package.name), consumer.manifest).toBe(true);
    expect(tokens(read(consumer.source)).includes("os_dsl"), consumer.source).toBe(false);
});

test("actual lower vocabulary owns the generic definitions without a product facade", () => {
  const deficits = corpus.mappings.filter(row => !existsSync(resolve(root, row.destination))).map(row => ({ role: row.role, currentOwner: row.source, expectedOwner: row.destination }));
  expect(deficits, "real generic definitions remain mounted by OS Kernel").toEqual([]);
  expect(existsSync(resolve(root, corpus.package.manifest)), "actual generic Cargo owner").toBe(true);
  for (const row of corpus.mappings) expect(tokens(read(row.destination)).includes("os_dsl"), row.destination).toBe(false);
});

test("the actual recognizer requires selected fragment and macro registrations and refuses missing fragments", () => {
  const source = actualGrammar(), compile = method(source, "Recognizer", "compile"), stream = tokens(source);
  expect(compile.parameters, "actual implicit one-argument constructor").toContain("FragmentRegistry");
  expect(compile.parameters).toContain("MacroMatcher");
  expect(compile.result, "missing selected fragment must produce an owned error").toContain("Result");
  expect(tokens(compile.body).includes("Err"), "actual missing fragment must refuse").toBe(true);
  expect(stream.includes("default_macros"), "specific matcher defaults cannot remain lower").toBe(false);
  expect(stream.includes("verify_protocol_bytes"), "specific shallow envelope classifier cannot remain lower").toBe(false);
  expect(stream.some((token, index) => token === "fn" && stream[index + 1] === "builtin"), "embedded family registry cannot remain lower").toBe(false);
  for (const row of corpus.specificFamilies) expect(source.includes(row.source.split("/👪️family/")[1]!), row.source).toBe(false);
});

test("the product has no lower lexical or idiom compatibility mounts and all actual constructor sites select arguments", () => {
  const kernel = tokens(read("🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs"));
  for (const name of ["token", "lexer", "trust"]) expect(kernel.some((token, index) => token === "mod" && kernel[index + 1] === name), name).toBe(false);
  const component = tokens(read("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs"));
  for (const name of ["DslIdiom", "CompletionItem", "IdiomHooks", "LanguageSpec", "LanguageRole"]) expect(component.some((token, index) => ["struct", "trait", "enum"].includes(token) && component[index + 1] === name), name).toBe(false);
  const sources = (path: string): string[] => readdirSync(resolve(root, path), { withFileTypes: true }).flatMap(row => row.isDirectory() ? sources(path + "/" + row.name) : row.isFile() && row.name.endsWith(".rs") ? [path + "/" + row.name] : []);
  for (const source of new Set(["🧰️framework/🔨️modules/🗣️dsl", "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts", ...corpus.consumers.map(row => row.owner)].flatMap(sources))) {
    const text = read(source);
    if (!text.includes("Recognizer") || !text.includes("compile")) continue;
    const stream = rustTokens(text), pairs = rustTokenPairs(stream);
    for (let index = 0; index < stream.length - 3; index++) {
      if (stream[index]?.text !== "Recognizer" || stream[index + 1]?.text !== "::" || stream[index + 2]?.text !== "compile" || stream[index + 3]?.text !== "(") continue;
      const close = pairs.get(index + 3)!; let commas = 0;
      for (let cursor = index + 4; cursor < close; cursor++) { if (["(", "[", "{"].includes(stream[cursor]!.text)) cursor = pairs.get(cursor)!; else if (stream[cursor]?.text === ",") commas++; }
      expect(commas, source + ":" + stream[index]!.start).toBe(2);
    }
  }
}, 30000);

test("diagnostic and span identities have an acyclic canonical owner beneath DSL and replication", () => {
  const manifest = "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/Cargo.toml";
  expect(existsSync(resolve(root, manifest))).toBe(true);
  const diagnostic = TOML.parse(read(manifest)) as any;
  expect(diagnostic.package.name).toBe("semio-framework-diagnostic");
  expect(diagnostic.dependencies["semio-framework-value"]).toBeDefined();
  expect(diagnostic.dependencies["semio-framework-dsl"]).toBeUndefined();
  expect(diagnostic.dependencies["semio-framework-replication"]).toBeUndefined();
  for (const module of ["🗣️dsl", "📡️replication"]) {
    const packageManifest = TOML.parse(read("🧰️framework/🔨️modules/" + module + "/📦️packages/🦀️rust/Cargo.toml")) as any;
    expect(packageManifest.dependencies["semio-framework-diagnostic"]).toBeDefined();
  }
  const dslManifest = TOML.parse(read(corpus.package.manifest)) as any;
  expect(dslManifest.dependencies["semio-framework-replication"]).toBeUndefined();
  expect(diagnostic.dependencies["wasm-bindgen"]).toBeUndefined();
});

