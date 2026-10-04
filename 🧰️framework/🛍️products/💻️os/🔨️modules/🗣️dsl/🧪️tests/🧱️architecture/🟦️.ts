import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import JSON5 from "json5";
import { parse as parseJsonDocument, type ParseError } from "jsonc-parser";
import TOML from "@iarna/toml";
import ts from "typescript";
import { createToken, Lexer } from "chevrotain";
import { rustTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { undoJsonOwnershipCuts } from "../🧬️record-floor/📑️receipt/🟦️.ts";

type Specimen = Readonly<{ path: string; sha256: string; bytes: number; source: string }>;
type Mapping = Readonly<{ role: string; source: string; destination: string; sha256: string }>;
type Capture = Readonly<{ inputs: readonly Specimen[]; callerRows: readonly unknown[]; constructorReferences: readonly Readonly<{ path: string; ownerType: string; operation: string; line: number }>[]; compileReferenceRefusals: readonly Readonly<{ path: string; error: string }>[] }>;
type Corpus = Readonly<{
  owner: string;
  package: Readonly<{ name: string; library: string; manifest: string }>;
  capture: Readonly<{ path: string; sha256: string; inputs: number; callerCandidates: number; constructorReferences: number }>;
  mappings: readonly Mapping[];
  specificFamilies: readonly Readonly<{ source: string; sha256: string; owner: string }>[];
  originalLawCohorts: readonly Readonly<{ source: string; sha256: string; count: number }>[];
  consumers: readonly Readonly<{ owner: string; manifest: string; source: string; root: string; boundary: "complete-package" | "language-only" }>[];
  composition: Readonly<Record<string, unknown>>;
  independentCases: readonly Readonly<{ id: string; text: string; escaped: string; jsonWire: string }>[];
  refusedEscapes: readonly Readonly<{ id: string; escaped: string; error: string }>[];
  lexicalCases: readonly Readonly<{ id: string; text: string; tokens: readonly Readonly<{ kind: string; text: string; start: number; end: number }>[] }>[];
}>;
const root = resolve(import.meta.dir, "../../../../../../.."), owner = resolve(root, "🧰️framework/🔨️modules/🗣️dsl"), sha = (value: string): string => createHash("sha256").update(value).digest("hex");
const documentationCut = JSON.parse(readFileSync(resolve(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/🧬️diagnostic-documentation-cut.json"), "utf8")) as {rows: readonly {path: string; source: string; beforeSha256: string; afterSha256: string; operations: readonly {from: string; to: string}[]}[]};
const read = (path: string): string => {
  let source = undoJsonOwnershipCuts(path, readFileSync(resolve(root, path), "utf8"));
  const cut = documentationCut.rows.find(row => row.path === path);
  if (!cut) return source;
  expect(sha(source), path).toBe(cut.afterSha256);
  for (const operation of cut.operations.toReversed()) {
    expect(source.split(operation.to).length, path).toBe(2);
    source = source.replace(operation.to, operation.from);
  }
  expect(sha(source), path).toBe(cut.beforeSha256);
  expect(source, path).toBe(cut.source);
  return source;
};
const corpus = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🧱️ownership/🔣️.json"), "utf8")) as Corpus;
const captured = JSON.parse(read(corpus.capture.path)) as Capture;
const specimens = new Map(captured.inputs.map(row => [row.path, row]));
const grammar = corpus.mappings.find(row => row.role === "grammar")!;
const actualGrammar = (): string => read(existsSync(resolve(root, grammar.destination)) ? grammar.destination : grammar.source);
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

const nativeLaws = (source: string): readonly Readonly<{ name: string; body: string }>[] => {
  const stream = rustTokens(source), pairs = rustTokenPairs(stream), laws: { name: string; body: string }[] = [];
  for (let index = 0; index < stream.length; index++) {
    if (stream[index]?.text !== "#" || stream[index + 1]?.text !== "[") continue;
    const attributeEnd = pairs.get(index + 1)!;
    if (!stream.slice(index + 2, attributeEnd).some(token => token.text === "test" || token.text === "async_test")) continue;
    const fn = stream.findIndex((token, offset) => offset > attributeEnd && token.text === "fn"), open = stream.findIndex((token, offset) => offset > fn && token.text === "{");
    const close = pairs.get(open)!;
    laws.push({ name: stream[fn + 1]!.text, body: source.slice(stream[open]!.start, stream[close]!.end) });
    index = close;
  }
  return laws;
};
const normalizeLaw = (input: string): readonly string[] => {
  let source = input.replace('parse_grammar(&print_protocol(&spec)).expect("project protocol")', 'project_protocol(spec.clone())');
  let stream = rustTokens(source), pairs = rustTokenPairs(stream);
  const edits: { start: number; end: number; text: string }[] = [];
  for (let index = 0; index < stream.length - 3; index++) {
    if (stream[index]?.text !== "Recognizer" || stream[index + 1]?.text !== "::" || stream[index + 2]?.text !== "compile" || stream[index + 3]?.text !== "(") continue;
    const close = pairs.get(index + 3)!, commas: number[] = [];
    for (let cursor = index + 4; cursor < close; cursor++) {
      if (["(", "[", "{"].includes(stream[cursor]!.text)) cursor = pairs.get(cursor)!;
      else if (stream[cursor]?.text === ",") commas.push(cursor);
    }
    if (commas.length !== 2) continue;
    const end = stream[close + 1]?.text === "." && stream[close + 2]?.text === "expect" && stream[close + 3]?.text === "(" ? pairs.get(close + 3)! : close;
    const label = source.slice(stream[close + 4]?.start, stream[close + 4]?.end);
    if (![ '"selected grammar fragments"', '"selected fragments"' ].includes(label)) throw Error("Unknown law constructor transformation");
    edits.push({ start: stream[commas[0]!]!.start, end: stream[end]!.end, text: ")" });
  }
  for (const edit of edits.sort((left, right) => right.start - left.start)) source = source.slice(0, edit.start) + edit.text + source.slice(edit.end);
  stream = rustTokens(source);
  const result: string[] = [];
  for (let index = 0; index < stream.length; index++) {
    if (["semio_framework_diagnostic", "semio_framework_dsl", "semio_framework_os_kernel", "dsl_core", "dsl_grammar", "dsl", "crate"].includes(stream[index]!.text) && stream[index + 1]?.text === "::") {
      index += 2;
      if (stream[index]?.text === "os_dsl" && stream[index + 1]?.text === "::") index += 2;
      if (stream[index]?.text === "grammar" && stream[index + 1]?.text === "::") index += 2;
    }
    result.push(stream[index]!.text);
  }
  return result;
};

test("every original specimen and constructor reference retains its exact captured authority", () => {
  expect(sha(read(corpus.capture.path))).toBe(corpus.capture.sha256);
  expect(captured.inputs.length).toBe(corpus.capture.inputs);
  expect(captured.callerRows.length).toBe(corpus.capture.callerCandidates);
  expect(captured.constructorReferences.length).toBe(corpus.capture.constructorReferences);
  expect(specimens.size).toBe(captured.inputs.length);
  for (const row of captured.inputs) {
    expect(sha(row.source), row.path).toBe(row.sha256);
    expect(Buffer.byteLength(row.source), row.path).toBe(row.bytes);
  }
  for (const row of captured.constructorReferences) {
    const source = specimens.get(row.path)!.source, stream = rustTokens(source);
    expect(stream.some((token, index) => token.text === row.ownerType && stream[index + 1]?.text === "::" && stream[index + 2]?.text === row.operation && stream[index + 3]?.text === "("), `${row.path}:${row.line}`).toBe(true);
  }
  for (const row of captured.compileReferenceRefusals) expect(row.error).toContain("Unsupported Rust compile expression");
});

test("all original lexer compiler grammar and literal assertions remain at their actual owners", () => {
  for (const row of corpus.originalLawCohorts) {
    expect(specimens.get(row.source)?.sha256, row.source).toBe(row.sha256);
    const destinations = row.source.includes("/🔍️lexer/") ? [row.source.replace("/🛍️products/💻️os", "")] : row.source.includes("/📖️grammar/📡️literal/") ? [row.source.replace("/🛍️products/💻️os", "")] : row.source.includes("/📖️grammar/") ? [row.source, row.source.replace("/🛍️products/💻️os", "")] : [row.source];
    const original = nativeLaws(specimens.get(row.source)!.source), current = destinations.flatMap(path => nativeLaws(read(path)));
    expect(original.length, row.source).toBe(row.count);
    expect(current.map(law => law.name).sort(), row.source).toEqual(original.map(law => law.name).sort());
    for (const law of original) expect(normalizeLaw(current.find(item => item.name === law.name)!.body), law.name).toEqual(normalizeLaw(law.body));
  }
});

test("all seven current family assets stay at their actual specific owners", () => {
  const originalFamilies = captured.inputs.filter(row => row.path.includes("/👪️family/") && row.path.endsWith("/📖️.grammar.semio"));
  expect(new Set(corpus.specificFamilies.map(row => row.source))).toEqual(new Set(originalFamilies.map(row => row.path)));
  for (const row of corpus.specificFamilies) {
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

test("every generic production and moved law source has an exact original-byte inverse", () => {
  type Cut = Readonly<{ source: string; sha256: string; destination: string; regions?: readonly Readonly<{ start: number; end: number }>[]; prefix?: string; suffix?: string; separator?: string; operations: readonly Readonly<{ from: string; to: string; all?: boolean }>[] }>;
  const cuts = JSON.parse(read(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/🧬️source-cut/🔣️.json")) as readonly Cut[];
  const retained = JSON.parse(read(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/🧬️owned-inputs/🔣️.json")) as readonly Specimen[];
  const authority = new Map([...captured.inputs, ...retained].map(row => [row.path, row]));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(resolve(owner, "🧬️schema/🧱️ownership-cut/🔣️.json"), "utf8")));
  expect(validate(cuts), JSON.stringify(validate.errors)).toBe(true);
  expect(validate([{ ...cuts[0], implicitFamilyFallback: true }])).toBe(false);
  for (const cut of cuts) {
    const original = authority.get(cut.source)!;
    expect(sha(original.source), cut.source).toBe(cut.sha256);
    let expected = cut.regions ? (cut.prefix ?? "") + cut.regions.map(region => original.source.slice(region.start, region.end)).join(cut.separator ?? "") + (cut.suffix ?? "") : original.source;
    const inverse: Readonly<{ before: string; after: string }>[] = [];
    for (const operation of cut.operations) {
      expect(expected.includes(operation.from), cut.source).toBe(true);
      const next = operation.all ? expected.split(operation.from).join(operation.to) : expected.replace(operation.from, operation.to);
      inverse.push({ before: expected, after: next });
      expected = next;
    }
    expect(read(cut.destination), cut.destination).toBe(expected);
    for (const row of inverse.toReversed()) { expect(expected).toBe(row.after); expected = row.before; }
    expect(expected, cut.source).toBe(cut.regions ? (cut.prefix ?? "") + cut.regions.map(region => original.source.slice(region.start, region.end)).join(cut.separator ?? "") + (cut.suffix ?? "") : original.source);
  }
});

test("all 503 original and current shipped grammar inputs retain explicit separate authority", () => {
  const rows = (captured as Capture & { shippedGrammarLawInputs: readonly Readonly<{ path: string; sha256: string }>[] }).shippedGrammarLawInputs;
  const admission = JSON.parse(read(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/🧬️shipped-current-admission.json")) as {rows: readonly {path: string; originalSha256: string; currentSha256: string; current?: string}[]};
  const validate = new Ajv({strict: true, allErrors: true}).compile(JSON.parse(readFileSync(resolve(owner, "🧬️schema/🧬️current-grammars/🔣️.json"), "utf8")));
  expect(validate(admission), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({...admission, frozenOriginalReplacement: true})).toBe(false);
  expect(rows.length).toBe(503);
  expect(admission.rows.length).toBe(503);
  expect(admission.rows.map(row => row.path)).toEqual(rows.map(row => row.path));
  for (const row of rows) {
    expect(sha(specimens.get(row.path)!.source), row.path).toBe(row.sha256);
    const current = admission.rows.find(input => input.path === row.path)!;
    expect(current.originalSha256).toBe(row.sha256);
    expect(sha(read(row.path)), row.path).toBe(current.currentSha256);
    if (current.current !== undefined) expect(sha(current.current)).toBe(current.currentSha256);
    else expect(current.currentSha256).toBe(row.sha256);
    expect(row.path.startsWith(corpus.owner + "/"), row.path).toBe(false);
  }
  const native = read("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🧪️tests/🧬️originals/🦀️.rs");
  expect(native).toContain("Recognizer::compile(&grammar, &registry, product_macros())");
  expect(native).toContain("assert_eq!(rows.len(), 503)");
});

test("the product has no lower lexical or idiom compatibility mounts and all actual constructor sites select arguments", () => {
  const kernel = tokens(read("🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs"));
  for (const name of ["token", "lexer", "trust"]) expect(kernel.some((token, index) => token === "mod" && kernel[index + 1] === name), name).toBe(false);
  const component = tokens(read("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs"));
  for (const name of ["DslIdiom", "CompletionItem", "IdiomHooks", "LanguageSpec", "LanguageRole"]) expect(component.some((token, index) => ["struct", "trait", "enum"].includes(token) && component[index + 1] === name), name).toBe(false);
  for (const source of new Set(captured.constructorReferences.map(row => row.path))) {
    if (!existsSync(resolve(root, source))) continue;
    const stream = rustTokens(read(source)), pairs = rustTokenPairs(stream);
    for (let index = 0; index < stream.length - 3; index++) {
      if (stream[index]?.text !== "Recognizer" || stream[index + 1]?.text !== "::" || stream[index + 2]?.text !== "compile" || stream[index + 3]?.text !== "(") continue;
      const close = pairs.get(index + 3)!; let commas = 0;
      for (let cursor = index + 4; cursor < close; cursor++) { if (["(", "[", "{"].includes(stream[cursor]!.text)) cursor = pairs.get(cursor)!; else if (stream[cursor]?.text === ",") commas++; }
      expect(commas, source + ":" + stream[index]!.start).toBe(2);
    }
  }
});

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
  const floor = JSON.parse(read(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/🧬️diagnostic-floor-before.json")) as {inputs: Specimen[]};
  for (const row of floor.inputs) expect(sha(row.source)).toBe(row.sha256);
  const original = floor.inputs.find(row => row.path.endsWith("⚠️diagnostic/🦀️.rs"))!;
  const bridgeStart = original.source.indexOf('#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]');
  const bridgeEnd = original.source.indexOf('//#endregion 🔖️Fault', bridgeStart);
  const browserBridge = original.source.slice(bridgeStart, bridgeEnd);
  expect(read(original.path).replace("use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};", "use crate::value::{DslValue, FromValue, ToValue, ValueError};")).toBe(original.source.slice(0, bridgeStart) + original.source.slice(bridgeEnd));
  expect(read("✏️s/🔌️plugins/🧩️puzzle/🌉️wasm/⚠️diagnostic/🦀️.rs").endsWith(browserBridge)).toBe(true);
  expect(diagnostic.dependencies["wasm-bindgen"]).toBeUndefined();
  const span = floor.inputs.find(row => row.path.includes("📍️span"))!;
  expect(read(span.path).replaceAll("semio_framework_value::", "crate::value::")).toBe(span.source);
  const retirement = floor.inputs.find(row => row.path.includes("📡️replication/🧬️retirement"))!;
  const leaf = "semio_framework_value::artifact_retire_leaf!(crate::diagnostic::Severity);";
  const retireStart = retirement.source.indexOf("impl semio_framework_value::retirement::RetireOwned for crate::diagnostic::FaultCode");
  const retireEnd = retirement.source.indexOf("\nimpl ", retireStart + 5);
  const faultRetirement = retirement.source.slice(retireStart, retireEnd);
  expect(read("🧰️framework/🔨️modules/⚠️diagnostic/🧬️retirement/🦀️.rs")).toBe(leaf.replace("crate::diagnostic::", "crate::") + "\n" + faultRetirement.replaceAll("crate::diagnostic::", "crate::") + "\n");
  expect(read(retirement.path)).toBe(retirement.source.replace(leaf + "\n", "").replace(faultRetirement + "\n", ""));
  const laws = floor.inputs.find(row => row.path.includes("fault-describe"))!;
  expect(read(laws.path)).toBe(laws.source);
});

test("all original portable declarations retain exact bodies or the declared erased type-only inverse", () => {
  const cut = JSON.parse(read(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/🧬️portable-owner-cut/🔣️.json")) as {sourceSha256: string; sourceText: string; tests: readonly {source: string; sha256: string; destination: string}[]};
  expect(sha(cut.sourceText)).toBe(cut.sourceSha256);
  expect(cut.tests.length).toBe(15);
  const typeCut = JSON.parse(read(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/🧬️portable-type-cut.json")) as {destination: string; source: string; beforeSha256: string; afterSha256: string; operation: {from: string; to: string}};
  expect(typeCut.operation).toEqual({from: "expectedMissing: readonly string[]", to: "expectedMissing: string[]"});
  let transformed = 0;
  for (const row of cut.tests) {
    expect(sha(row.source)).toBe(row.sha256);
    if (row.source !== typeCut.source) { expect(read(row.destination)).toContain(row.source); continue; }
    transformed++;
    expect(row.destination).toBe(typeCut.destination);
    expect(row.sha256).toBe(typeCut.beforeSha256);
    expect(row.source.split(typeCut.operation.from).length).toBe(2);
    const current = row.source.replace(typeCut.operation.from, typeCut.operation.to);
    expect(sha(current)).toBe(typeCut.afterSha256);
    expect(current.replace(typeCut.operation.to, typeCut.operation.from)).toBe(row.source);
    expect(read(row.destination)).toContain(current);
    const emit = (source: string): string => ts.transpileModule(source, {compilerOptions: {target: ts.ScriptTarget.ESNext}}).outputText;
    expect(emit(current)).toBe(emit(row.source));
  }
  expect(transformed).toBe(1);
});

test("canonical diagnostic client bindings retain exact original source through declared owner replacements", () => {
  const cut = JSON.parse(read(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/🧬️diagnostic-client-correction.json")) as {rows: readonly {path: string; source: string; beforeSha256: string; afterSha256: string; references: number; operations: readonly {from: string; to: string; all: boolean}[]}[]};
  expect(cut.rows.length).toBe(14);
  let references = 0;
  for (const row of cut.rows) {
    expect(sha(row.source), row.path).toBe(row.beforeSha256);
    let current = row.source;
    let rowReferences = 0;
    for (const operation of row.operations) {
      const count = current.split(operation.from).length - 1;
      expect(operation.all, row.path).toBe(true);
      expect(count, row.path).toBeGreaterThan(0);
      rowReferences += count;
      references += count;
      current = current.split(operation.from).join(operation.to);
    }
    expect(rowReferences, row.path).toBe(row.references);
    expect(sha(current), row.path).toBe(row.afterSha256);
    expect(read(row.path), row.path).toBe(current);
  }
  expect(references).toBe(86);
});
