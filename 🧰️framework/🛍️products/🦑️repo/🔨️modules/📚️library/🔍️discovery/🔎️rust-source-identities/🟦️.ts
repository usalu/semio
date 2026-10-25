import { type RustToken, rustAttributes, rustFindTopLevel, rustStringValue, rustTokenPairs, rustTokenSegments, rustTokenText, rustTokens, rustVisibility } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import type { RustStructuralFacts, RustVirtualSourceFact } from "../🦀️rust-fact-contracts/🟦️.ts";
import { inspectRustStructure } from "../🏛️rust-structure/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { loadTaxonomy } from "../🔐️taxonomy-authority/🟦️.ts";

/** 🔎️ Lists top-level public Rust type declarations without comment or string decoys. */
export function inspectRustPublicTypeNames(source: string): readonly string[] {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), names: string[] = [];
  for (let index = 0; index < tokens.length;) {
    const attributes = rustAttributes(tokens, pairs, index, tokens.length), visibility = rustVisibility(tokens, pairs, attributes.next), keyword = tokens[visibility.next]?.text;
    if (visibility.value === "pub" && (keyword === "struct" || keyword === "enum") && tokens[visibility.next + 1]?.kind === "identifier") names.push(tokens[visibility.next + 1]!.text);
    const boundary = rustFindTopLevel(tokens, pairs, attributes.next, tokens.length, new Set([";", "{"]));
    if (boundary < 0) break;
    if (tokens[boundary]!.text === ";") { index = boundary + 1; continue; }
    const close = pairs.get(boundary); if (close === undefined) break; index = close + 1;
  }
  return [...new Set(names)].sort();
}

/** 🧷️ Locates one unambiguous public mutation aggregate declaration for structured insertion. */
export function inspectRustMutationAggregateSpan(source: string): { declarationStart: number; bodyOpen: number; bodyClose: number; enumName: string } | null {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const matches: { declarationStart: number; bodyOpen: number; bodyClose: number; enumName: string }[] = [];
  let malformed = false;
  for (let index = 0; index < tokens.length;) {
    const declarationIndex = index;
    const attributes = rustAttributes(tokens, pairs, declarationIndex, tokens.length);
    const visibility = rustVisibility(tokens, pairs, attributes.next);
    if (visibility.value !== "pub" || tokens[visibility.next]?.text !== "enum") {
      const boundary = rustFindTopLevel(tokens, pairs, attributes.next, tokens.length, new Set([";", "{"]));
      if (boundary < 0) break;
      if (tokens[boundary]!.text === ";") { index = boundary + 1; continue; }
      const close = pairs.get(boundary);
      if (close === undefined) { malformed = true; break; }
      index = close + 1;
      continue;
    }
    const name = tokens[visibility.next + 1];
    if (!name || name.kind !== "identifier" || !name.text.endsWith("Mutation")) { index = visibility.next + 1; continue; }
    const body = rustFindTopLevel(tokens, pairs, visibility.next + 2, tokens.length, new Set(["{"]));
    const close = body < 0 ? undefined : pairs.get(body);
    if (close === undefined) { malformed = true; break; }
    const attributeStart = tokens[declarationIndex]!.start;
    const docs = /((?:(?:[ \t]*\/\/\/[^\n]*(?:\n|$))|(?:[ \t]*\/\*\*[\s\S]*?\*\/[ \t]*(?:\n|$)))+)$/u.exec(source.slice(0, attributeStart));
    matches.push({ declarationStart: docs?.index ?? attributeStart, bodyOpen: tokens[body]!.start, bodyClose: tokens[close]!.start, enumName: name.text });
    index = close + 1;
  }
  return !malformed && matches.length === 1 ? matches[0]! : null;
}

/** 📜️ Renders the stable Rust structural report as deterministic newline-terminated JSON. */
export function renderRustStructuralFactsJson(facts: RustStructuralFacts): string {
  return `${JSON.stringify(facts, null, 2)}\n`;
}

/** 🪪️ Lists exact Rust identifiers and string identities without accepting comment text or prefixes. */
export function inspectRustSourceIdentities(source: string): readonly string[] {
  return [...new Set(rustTokens(source).flatMap((token) => token.kind === "identifier" ? [token.text] : token.kind === "string" ? [rustStringValue(token) ?? ""] : []))].sort();
}

/** 🏷️ Extracts type identities while excluding path prefixes and string/comment decoys. */
export function rustTypeNames(source: string): string[] {
  const tokens = rustTokens(source);
  return [...new Set(tokens.filter((token, index) => token.kind === "identifier" && tokens[index + 1]?.text !== "::" && !["as", "dyn", "impl", "mut", "const"].includes(token.text)).map((token) => token.text))];
}

/** 🔗️ Records local type and renamed-import edges without evaluating or expanding source. */
export function rustTypeAliasEdges(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): Map<string, string[]> {
  const aliases = new Map<string, string[]>();
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text === "type" && tokens[index + 1]?.kind === "identifier") {
      const end = rustFindTopLevel(tokens, pairs, index + 2, tokens.length, new Set([";"]));
      const equals = end < 0 ? -1 : rustFindTopLevel(tokens, pairs, index + 2, end, new Set(["="]));
      if (equals >= 0) aliases.set(tokens[index + 1]!.text, rustTypeNames(rustTokenText(tokens, equals + 1, end)));
    }
    if (tokens[index]?.text !== "use") continue;
    const end = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set([";"]));
    for (let cursor = index + 1; cursor < end; cursor++) {
      if (tokens[cursor]?.text === "as" && tokens[cursor - 1]?.kind === "identifier" && tokens[cursor + 1]?.kind === "identifier") aliases.set(tokens[cursor + 1]!.text, [tokens[cursor - 1]!.text]);
    }
  }
  return aliases;
}

/** 🚧️ Indexes an aggregate once and inspects each leaf for reachable aggregate-state inputs. */
export function createRustMutationInputInspector(aggregateSource: string): (leafSource: string) => readonly string[] {
  const aggregateTokens = rustTokens(aggregateSource);
  const aggregatePairs = rustTokenPairs(aggregateTokens);
  const aggregateAliases = rustTypeAliasEdges(aggregateTokens, aggregatePairs);
  const aggregateTypes = new Set<string>();
  const typeQueue: string[] = [];
  for (let index = 0; index + 3 < aggregateTokens.length; index++) {
    if (aggregateTokens[index]?.text !== "#" || aggregateTokens[index + 1]?.text !== "[" || aggregateTokens[index + 2]?.text !== "mutations" || aggregateTokens[index + 3]?.text !== "(") continue;
    const close = aggregatePairs.get(index + 3);
    if (close === undefined) continue;
    for (const [start, end] of rustTokenSegments(aggregateTokens, aggregatePairs, index + 4, close, ",")) {
      if (["snapshot", "diff"].includes(aggregateTokens[start]?.text ?? "") && aggregateTokens[start + 1]?.text === "=") typeQueue.push(...rustTypeNames(rustTokenText(aggregateTokens, start + 2, end)));
    }
  }
  for (let index = 0; index < typeQueue.length; index++) {
    const name = typeQueue[index]!;
    if (aggregateTypes.has(name)) continue;
    aggregateTypes.add(name);
    typeQueue.push(...aggregateAliases.get(name) ?? []);
  }
  if (aggregateTypes.size === 0) return () => [];
  const owners = new Set(inspectRustStructure(aggregateSource).enums.flatMap((item) => item.variants.flatMap((variant) => variant.fieldTypes.flatMap(rustTypeNames))));
  return (leafSource) => {
    const leafTokens = rustTokens(leafSource);
    const graph = rustTypeAliasEdges(leafTokens, rustTokenPairs(leafTokens));
    const leafFacts = inspectRustStructure(leafSource);
    for (const payload of leafFacts.inlinePayloads) graph.set(payload.name, payload.fields.flatMap((field) => rustTypeNames(field.type)));
    for (const payload of leafFacts.enums) graph.set(payload.name, payload.variants.flatMap((variant) => variant.fieldTypes.flatMap(rustTypeNames)));
    const carriers: string[] = [];
    for (const owner of owners) {
      if (!graph.has(owner)) continue;
      const paths = [[owner]];
      const visited = new Set<string>();
      for (let index = 0; index < paths.length; index++) {
        const path = paths[index]!;
        const name = path.at(-1)!;
        if (visited.has(name)) continue;
        visited.add(name);
        if (aggregateTypes.has(name)) carriers.push(path.join(" -> "));
        else for (const child of graph.get(name) ?? []) paths.push([...path, child]);
      }
    }
    return carriers.sort();
  };
}

/** 🛡️ Normalizes one virtual repository path without permitting absolute or parent traversal. */
function rustVirtualRelativePath(path: string): string {
  const normalized = path.normalize("NFC").replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/{2,}/gu, "/");
  if (!normalized || normalized.startsWith("/") || /^[A-Za-z]:\//u.test(normalized)) throw new Error(`Rust virtual source path must be repository-relative: ${JSON.stringify(path)}.`);
  const segments = normalized.split("/");
  if (segments.some((segment) => !segment || segment === "." || segment === "..")) throw new Error(`Rust virtual source path contains traversal: ${JSON.stringify(path)}.`);
  return segments.join("/");
}

/** 🚫️ Applies configured opaque prefixes lexically to a virtual path before its reader is called. */
export function taxonomyRelativePathIsExcluded(path: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const normalized = rustVirtualRelativePath(path);
  return Object.values(taxonomy.pathExclusions).some((exclusion) => {
    const prefix = exclusion.path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "");
    return normalized === prefix || normalized.startsWith(`${prefix}/`);
  });
}

/** 🧪️ Inspects an in-memory/virtual Rust tree, filtering opaque paths before any source read. */
export function inspectRustVirtualSources(paths: readonly string[], readSource: (path: string) => string, taxonomy: Taxonomy = loadTaxonomy()): RustVirtualSourceFact[] {
  const candidates = paths.map((path) => ({ original: path, normalized: rustVirtualRelativePath(path) }))
    .filter(({ normalized }) => !taxonomyRelativePathIsExcluded(normalized, taxonomy) && normalized.endsWith(".rs"))
    .sort((left, right) => Buffer.from(left.normalized).compare(Buffer.from(right.normalized)));
  const seen = new Set<string>();
  return candidates.map(({ original, normalized }) => {
    if (seen.has(normalized)) throw new Error(`Rust virtual source path is duplicated after normalization: ${JSON.stringify(normalized)}.`);
    seen.add(normalized);
    return { path: normalized, facts: inspectRustStructure(readSource(original)) };
  });
}
