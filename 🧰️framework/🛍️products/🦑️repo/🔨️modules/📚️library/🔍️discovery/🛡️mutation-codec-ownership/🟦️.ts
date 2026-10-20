import { type RustToken, rustFindTopLevel, rustTokenPairs, rustTokenSegments, rustTokenText, rustTokens } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { inspectRustStructure, rustPatternVariantPath } from "../🏛️rust-structure/🟦️.ts";
import { rustTypeAliasEdges, rustTypeNames } from "../🔎️rust-source-identities/🟦️.ts";

export interface RustMutationCodecOwnershipFact {
  readonly kind: "whole-aggregate-serialization" | "whole-aggregate-deserialization" | "aggregate-variant-match";
}

function rustMutationCodecAliasClosure(seeds: readonly string[], aliases: ReadonlyMap<string, readonly string[]>): Set<string> {
  const names = new Set(seeds);
  let changed = true;
  while (changed) {
    changed = false;
    for (const [alias, targets] of aliases) {
      if (names.has(alias) || !targets.some((target) => names.has(target))) continue;
      names.add(alias);
      changed = true;
    }
  }
  return names;
}

function rustMutationCodecCall(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, nameIndex: number): { readonly open: number; readonly close: number; readonly genericStart: number | null; readonly genericEnd: number | null } | null {
  let cursor = nameIndex + 1;
  let genericStart: number | null = null;
  let genericEnd: number | null = null;
  if (tokens[cursor]?.text === "::" && tokens[cursor + 1]?.text === "<") {
    genericStart = cursor + 2;
    let depth = 1;
    cursor += 2;
    for (; cursor < tokens.length; cursor += 1) {
      if (tokens[cursor]!.text === "<") depth += 1;
      else if (tokens[cursor]!.text === ">") depth -= 1;
      if (depth === 0) break;
    }
    if (depth !== 0) return null;
    genericEnd = cursor;
    cursor += 1;
  }
  if (tokens[cursor]?.text !== "(") return null;
  const close = pairs.get(cursor);
  return close === undefined ? null : { open: cursor, close, genericStart, genericEnd };
}

function rustMutationCodecExpressionIsAggregate(tokens: readonly RustToken[], start: number, end: number, aggregateValues: readonly ReadonlyMap<string, boolean>[]): boolean {
  const values = tokens.slice(start, end).filter((token) => !["&", "mut", "(", ")"].includes(token.text));
  if (values.length !== 1 || values[0]?.kind !== "identifier") return false;
  for (let index = aggregateValues.length - 1; index >= 0; index -= 1) if (aggregateValues[index]!.has(values[0].text)) return aggregateValues[index]!.get(values[0].text) === true;
  return false;
}

function rustMutationCodecImplRanges(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, aggregateTypes: ReadonlySet<string>): (readonly [number, number])[] {
  const ranges: (readonly [number, number])[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]?.text !== "impl") continue;
    let open = -1;
    for (let cursor = index + 1; cursor < tokens.length; cursor += 1) {
      if (tokens[cursor]!.text === "{") {
        open = cursor;
        break;
      }
      if (tokens[cursor]!.text === ";") break;
    }
    const close = open < 0 ? undefined : pairs.get(open);
    if (close === undefined) continue;
    const forIndex = rustFindTopLevel(tokens, pairs, index + 1, open, new Set(["for"]));
    let cursor = forIndex < 0 ? index + 1 : forIndex + 1;
    if (tokens[cursor]?.text === "<") {
      let depth = 1;
      cursor += 1;
      for (; cursor < open && depth > 0; cursor += 1) {
        if (tokens[cursor]?.text === "<") depth += 1;
        else if (tokens[cursor]?.text === ">") depth -= 1;
      }
    }
    const path: string[] = [];
    for (; cursor < open; cursor += 1) {
      const token = tokens[cursor]!;
      if (token.kind === "identifier") path.push(token.text);
      else if (token.text === "::") continue;
      else break;
    }
    if (aggregateTypes.has(path.at(-1) ?? "")) ranges.push([open + 1, close]);
    index = close;
  }
  return ranges;
}

function rustMutationCodecSerdeCall(tokens: readonly RustToken[], nameIndex: number, serdeAliases: ReadonlySet<string>): boolean {
  return tokens[nameIndex - 1]?.text === "::" && serdeAliases.has(tokens[nameIndex - 2]?.text ?? "");
}

interface RustMutationCodecFunctionRange {
  readonly start: number;
  readonly bodyStart: number;
  readonly bodyEnd: number;
  readonly aggregateImpl: boolean;
}

function rustMutationCodecFunctions(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, aggregateImplRanges: readonly (readonly [number, number])[]): RustMutationCodecFunctionRange[] {
  const functions: RustMutationCodecFunctionRange[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]?.text !== "fn") continue;
    let bodyStart = -1;
    for (let cursor = index + 1; cursor < tokens.length; cursor += 1) {
      if (tokens[cursor]!.text !== "{") continue;
      bodyStart = cursor;
      break;
    }
    const bodyEnd = bodyStart < 0 ? undefined : pairs.get(bodyStart);
    if (bodyEnd === undefined) continue;
    functions.push({ start: index, bodyStart, bodyEnd, aggregateImpl: aggregateImplRanges.some(([start, end]) => index >= start && index < end) });
    index = bodyEnd;
  }
  return functions;
}

function rustMutationCodecTypeIsAggregate(tokens: readonly RustToken[], start: number, end: number, aggregateTypes: ReadonlySet<string>, aggregateImpl: boolean): boolean {
  const names = rustTypeNames(rustTokenText(tokens, start, end));
  return names.some((name) => aggregateTypes.has(name)) || (aggregateImpl && names.includes("Self"));
}

function rustMutationCodecParameterValues(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, fnRange: RustMutationCodecFunctionRange, aggregateTypes: ReadonlySet<string>): Map<string, boolean> {
  const values = new Map<string, boolean>();
  for (let cursor = fnRange.start + 1; cursor < fnRange.bodyStart; cursor += 1) {
    if (tokens[cursor]?.text !== "(") continue;
    const close = pairs.get(cursor);
    if (close === undefined) break;
    for (const [start, end] of rustTokenSegments(tokens, pairs, cursor + 1, close, ",")) {
      const colon = rustFindTopLevel(tokens, pairs, start, end, new Set([":"]));
      const name = tokens[start]?.text === "mut" ? tokens[start + 1] : tokens[start];
      const receiver = tokens.slice(start, end).find((token) => token.text === "self");
      if (receiver) {
        values.set("self", fnRange.aggregateImpl);
        continue;
      }
      if (name?.kind !== "identifier") continue;
      if (colon >= 0) values.set(name.text, rustMutationCodecTypeIsAggregate(tokens, colon + 1, end, aggregateTypes, fnRange.aggregateImpl));
    }
    break;
  }
  return values;
}

function rustMutationCodecLetValue(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, index: number, end: number, values: readonly Map<string, boolean>[], aggregateTypes: ReadonlySet<string>, aggregateImpl: boolean): void {
  const semicolon = rustFindTopLevel(tokens, pairs, index + 1, end, new Set([";"]));
  const statementEnd = semicolon < 0 ? end : semicolon;
  const name = tokens[index + (tokens[index + 1]?.text === "mut" ? 2 : 1)];
  const equals = rustFindTopLevel(tokens, pairs, index + 1, statementEnd, new Set(["="]));
  const colon = equals < 0 ? -1 : rustFindTopLevel(tokens, pairs, index + 1, equals, new Set([":"]));
  if (name?.kind === "identifier" && equals >= 0) values.at(-1)!.set(name.text, (colon >= 0 && rustMutationCodecTypeIsAggregate(tokens, colon + 1, equals, aggregateTypes, aggregateImpl)) || rustMutationCodecExpressionIsAggregate(tokens, equals + 1, statementEnd, values));
}

function rustMutationCodecInferredAggregate(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, index: number, callClose: number, fnRange: RustMutationCodecFunctionRange, aggregateTypes: ReadonlySet<string>): boolean {
  let statementStart = fnRange.bodyStart + 1;
  for (let cursor = index - 1; cursor >= statementStart; cursor -= 1) {
    if (![";", "{", "}"].includes(tokens[cursor]?.text ?? "")) continue;
    statementStart = cursor + 1;
    break;
  }
  const letIndex = tokens.slice(statementStart, index).findIndex((token) => token.text === "let");
  if (letIndex >= 0) {
    const start = statementStart + letIndex;
    const equals = rustFindTopLevel(tokens, pairs, start + 1, index, new Set(["="]));
    const colon = equals < 0 ? -1 : rustFindTopLevel(tokens, pairs, start + 1, equals, new Set([":"]));
    if (colon >= 0) return rustMutationCodecTypeIsAggregate(tokens, colon + 1, equals, aggregateTypes, fnRange.aggregateImpl);
  }
  const arrow = rustFindTopLevel(tokens, pairs, fnRange.start + 1, fnRange.bodyStart, new Set(["->"]));
  const returnsCall = tokens.slice(statementStart, index).some((token) => token.text === "return");
  const tailCall = !tokens.slice(callClose + 1, fnRange.bodyEnd).some((token) => token.text === ";");
  return arrow >= 0 && (returnsCall || tailCall) && rustMutationCodecTypeIsAggregate(tokens, arrow + 1, fnRange.bodyStart, aggregateTypes, fnRange.aggregateImpl);
}

/** 🧪️ Finds executable aggregate codec bypasses from Rust token structure without type expansion. */
export function createRustMutationCodecOwnershipInspector(aggregateSource: string): (codecSource: string) => readonly RustMutationCodecOwnershipFact[] {
  const aggregateTokens = rustTokens(aggregateSource);
  const aggregatePairs = rustTokenPairs(aggregateTokens);
  const aggregateAliases = rustTypeAliasEdges(aggregateTokens, aggregatePairs);
  const aggregateEnums = inspectRustStructure(aggregateSource).enums.filter((item) => item.name.includes("Mutation"));
  const aggregateTypeSeeds = aggregateEnums.map((item) => item.name);
  const aggregateVariantNames = new Set(aggregateEnums.flatMap((item) => item.variants.map((variant) => variant.name)));
  return (codecSource) => {
    const tokens = rustTokens(codecSource);
    const pairs = rustTokenPairs(tokens);
    const aliases = new Map([...aggregateAliases, ...rustTypeAliasEdges(tokens, pairs)]);
    const aggregateTypes = rustMutationCodecAliasClosure(aggregateTypeSeeds, aliases);
    const serdeAliases = rustMutationCodecAliasClosure(["serde_json"], aliases);
    const aggregateImplRanges = rustMutationCodecImplRanges(tokens, pairs, aggregateTypes);
    const facts: RustMutationCodecOwnershipFact[] = [];
    for (const fnRange of rustMutationCodecFunctions(tokens, pairs, aggregateImplRanges)) {
      const values: Map<string, boolean>[] = [rustMutationCodecParameterValues(tokens, pairs, fnRange, aggregateTypes)];
      for (let index = fnRange.bodyStart + 1; index < fnRange.bodyEnd; index += 1) {
        if (tokens[index]?.text === "{") values.push(new Map());
        else if (tokens[index]?.text === "}") values.pop();
        if (tokens[index]?.text === "let") rustMutationCodecLetValue(tokens, pairs, index, fnRange.bodyEnd, values, aggregateTypes, fnRange.aggregateImpl);
        const name = tokens[index]?.text;
        if (!name || !rustMutationCodecSerdeCall(tokens, index, serdeAliases)) continue;
        const call = rustMutationCodecCall(tokens, pairs, index);
        if (!call) continue;
        if (["to_vec", "to_string", "to_value", "to_writer"].includes(name) && rustTokenSegments(tokens, pairs, call.open + 1, call.close, ",").some(([start, end]) => rustMutationCodecExpressionIsAggregate(tokens, start, end, values))) facts.push({ kind: "whole-aggregate-serialization" });
        if (!["from_slice", "from_str", "from_reader", "from_value"].includes(name)) continue;
        const genericTypes = call.genericStart === null || call.genericEnd === null ? [] : rustTypeNames(rustTokenText(tokens, call.genericStart, call.genericEnd));
        if (genericTypes.some((type) => aggregateTypes.has(type)) || (genericTypes.includes("Self") && fnRange.aggregateImpl) || (genericTypes.length === 0 && rustMutationCodecInferredAggregate(tokens, pairs, index, call.close, fnRange, aggregateTypes))) facts.push({ kind: "whole-aggregate-deserialization" });
      }
    }
    for (let index = 0; index < tokens.length; index += 1) {
      if (tokens[index]?.text !== "match") continue;
      let open = -1;
      for (let cursor = index + 1; cursor < tokens.length; cursor += 1) if (tokens[cursor]!.text === "{") { open = cursor; break; }
      const close = open < 0 ? undefined : pairs.get(open);
      if (close === undefined) continue;
      const aggregateImpl = aggregateImplRanges.some(([start, end]) => index >= start && index < end);
      for (const [start, end] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
        const arrow = rustFindTopLevel(tokens, pairs, start, end, new Set(["=>"]));
        const path = arrow < 0 ? null : rustPatternVariantPath(tokens, start, arrow);
        const parts = path?.split("::") ?? [];
        const owner = parts.at(-2) ?? "";
        if (parts.length > 1 && (aggregateTypes.has(owner) || (aggregateImpl && owner === "Self")) && aggregateVariantNames.has(parts.at(-1) ?? "")) facts.push({ kind: "aggregate-variant-match" });
      }
      index = close;
    }
    return facts;
  };
}
