import { type RustAttributes, type RustMetadataAttributeFact, type RustStructuralVisibility, type RustToken, type RustVisibility, rustAttributes, rustFindTopLevel, rustMetadataAttributeHead, rustMetadataAttributes, rustMetadataDerives, rustMetadataPath, rustStringValue, rustTokenPairs, rustTokenSegments, rustTokenText, rustTokens, rustVisibility } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import type { RustConstIdentityFact, RustEnumFact, RustEnumVariantFact, RustImplFact, RustIncludeFact, RustInlinePayloadFact, RustMatchArmFact, RustModuleFact, RustPayloadFieldFact, RustStructuralFacts } from "../🦀️rust-fact-contracts/🟦️.ts";
import { rustCfgTest, rustPathAttribute } from "../🏃️rust-runnable-tests/🟦️.ts";

/** 🔤️ Returns the first path-shaped constructor in one tokenized match pattern. */
export function rustPatternVariantPath(tokens: readonly RustToken[], start: number, end: number): string | null {
  for (let index = start; index < end; index += 1) {
    if (tokens[index]?.kind !== "identifier") continue;
    const parts = [tokens[index]!.text];
    let cursor = index + 1;
    while (cursor + 1 < end && tokens[cursor]!.text === "::" && tokens[cursor + 1]!.kind === "identifier") {
      parts.push(tokens[cursor + 1]!.text);
      cursor += 2;
    }
    if (parts.length > 1) return parts.join("::");
  }
  return null;
}

/** 🏷️ Derives stable string identity fields from one struct-literal const expression. */
function rustConstIdentityFields(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): Readonly<Record<string, string>> {
  const open = rustFindTopLevel(tokens, pairs, start, end, new Set(["{"]));
  const close = open < 0 ? undefined : pairs.get(open);
  if (open < 0 || close === undefined || close > end) return {};
  const fields: Record<string, string> = {};
  for (const [fieldStart, fieldEnd] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
    const colon = rustFindTopLevel(tokens, pairs, fieldStart, fieldEnd, new Set([":"]));
    const name = tokens[fieldStart];
    const value = colon < 0 ? null : rustStringValue(tokens[colon + 1]);
    if (name?.kind === "identifier" && value !== null) fields[name.text] = value;
  }
  return Object.fromEntries(Object.entries(fields).sort(([left], [right]) => left.localeCompare(right)));
}

/** 🌳️ Parses Rust items from tokens and exposes only stable structural facts. */
class RustStructureParser {
  readonly modules: RustModuleFact[] = [];
  readonly enums: RustEnumFact[] = [];
  readonly impls: RustImplFact[] = [];
  readonly inlinePayloads: RustInlinePayloadFact[] = [];
  readonly constants: RustConstIdentityFact[] = [];

  readonly tokens: readonly RustToken[];
  readonly pairs: ReadonlyMap<number, number>;
  constructor(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>) { this.tokens = tokens; this.pairs = pairs; }

  parse(): void {
    this.parseScope(0, this.tokens.length);
  }

  private parseScope(start: number, end: number, inheritedConditional = false): void {
    let index = start, scopeConditional = inheritedConditional;
    while (index + 2 < end && this.tokens[index]?.text === "#" && this.tokens[index + 1]?.text === "!" && this.tokens[index + 2]?.text === "[") {
      const close = this.pairs.get(index + 2);
      if (close === undefined || close >= end) return;
      scopeConditional ||= rustMetadataAttributes(this.tokens, this.pairs, { ranges: [[index + 3, close]], next: close + 1 }).conditional;
      index = close + 1;
    }
    while (index < end) {
      const attributes = rustAttributes(this.tokens, this.pairs, index, end);
      const visibility = rustVisibility(this.tokens, this.pairs, attributes.next);
      const keyword = this.tokens[visibility.next]?.text;
      const conditional = scopeConditional || rustMetadataAttributes(this.tokens, this.pairs, attributes).conditional;
      if (keyword === "mod") index = this.parseModule(attributes, visibility, end, conditional);
      else if (keyword === "enum") index = this.parseEnum(attributes, visibility, end, conditional);
      else if (keyword === "struct") index = this.parseStruct(visibility, end);
      else if (keyword === "impl") index = this.parseImpl(visibility.next, end);
      else if (keyword === "const" && this.tokens[visibility.next + 1]?.text !== "fn") index = this.parseConst(visibility.next, end, null);
      else index = this.skipItem(attributes.next, end);
    }
  }

  private skipItem(start: number, end: number): number {
    const boundary = rustFindTopLevel(this.tokens, this.pairs, start, end, new Set([";", "{"]));
    if (boundary < 0) return end;
    if (this.tokens[boundary]!.text === ";") return boundary + 1;
    const close = this.pairs.get(boundary);
    return close === undefined ? end : close + 1;
  }

  private parseModule(attributes: RustAttributes, visibility: RustVisibility, end: number, conditional: boolean): number {
    const keyword = visibility.next;
    const name = this.tokens[keyword + 1]?.text ?? "";
    const boundary = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, end, new Set([";", "{"]));
    const inline = boundary >= 0 && this.tokens[boundary]!.text === "{";
    const fact: RustModuleFact = { name, visibility: visibility.value, inline, pathTarget: rustPathAttribute(this.tokens, this.pairs, attributes), cfgTest: rustCfgTest(this.tokens, attributes) };
    this.modules.push(fact);
    if (!inline) return boundary < 0 ? end : boundary + 1;
    const close = this.pairs.get(boundary);
    if (close === undefined) return end;
    this.parseScope(boundary + 1, close, conditional);
    return close + 1;
  }

  private parseEnum(attributes: RustAttributes, visibility: RustVisibility, end: number, conditional: boolean): number {
    const keyword = visibility.next;
    const name = this.tokens[keyword + 1]?.text ?? "";
    const open = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, end, new Set(["{"]));
    const close = open < 0 ? undefined : this.pairs.get(open);
    if (open < 0 || close === undefined) return end;
    const variants: RustEnumVariantFact[] = [];
    for (const [rawStart, rawEnd] of rustTokenSegments(this.tokens, this.pairs, open + 1, close, ",")) {
      const attributes = rustAttributes(this.tokens, this.pairs, rawStart, rawEnd);
      const variantStart = attributes.next;
      const variant = this.tokens[variantStart];
      if (variant?.kind !== "identifier") continue;
      const shape = this.tokens[variantStart + 1]?.text;
      const variantConditional = conditional || rustMetadataAttributes(this.tokens, this.pairs, attributes).conditional;
      if (shape === "(") {
        const fieldClose = this.pairs.get(variantStart + 1) ?? variantStart + 1;
        const fieldTypes = rustTokenSegments(this.tokens, this.pairs, variantStart + 2, fieldClose, ",").map(([fieldStart, fieldEnd]) => rustTokenText(this.tokens, fieldStart, fieldEnd));
        variants.push({ name: variant.text, fieldStyle: "tuple", fieldTypes, wrappedTupleLeafType: fieldTypes.length === 1 ? fieldTypes[0]! : null, ...(variantConditional ? { conditional: true as const } : {}) });
      } else if (shape === "{") {
        const fieldClose = this.pairs.get(variantStart + 1) ?? variantStart + 1;
        const fieldTypes = rustTokenSegments(this.tokens, this.pairs, variantStart + 2, fieldClose, ",").map(([fieldStart, fieldEnd]) => {
          const fieldAttributes = rustAttributes(this.tokens, this.pairs, fieldStart, fieldEnd);
          const fieldVisibility = rustVisibility(this.tokens, this.pairs, fieldAttributes.next);
          const colon = rustFindTopLevel(this.tokens, this.pairs, fieldVisibility.next, fieldEnd, new Set([":"]));
          return colon < 0 ? "" : rustTokenText(this.tokens, colon + 1, fieldEnd);
        }).filter(Boolean);
        variants.push({ name: variant.text, fieldStyle: "struct", fieldTypes, wrappedTupleLeafType: null, ...(variantConditional ? { conditional: true as const } : {}) });
      } else variants.push({ name: variant.text, fieldStyle: "unit", fieldTypes: [], wrappedTupleLeafType: null, ...(variantConditional ? { conditional: true as const } : {}) });
    }
    this.enums.push({ name, visibility: visibility.value, variants, ...(conditional ? { conditional: true as const } : {}) });
    return close + 1;
  }

  private parseStruct(visibility: RustVisibility, end: number): number {
    const keyword = visibility.next;
    const name = this.tokens[keyword + 1]?.text ?? "";
    const boundary = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, end, new Set([";", "{", "("]));
    if (boundary < 0) return end;
    const shape = this.tokens[boundary]!.text;
    if (shape === ";") {
      this.inlinePayloads.push({ name, visibility: visibility.value, fieldStyle: "unit", fields: [] });
      return boundary + 1;
    }
    const close = this.pairs.get(boundary);
    if (close === undefined) return end;
    const fields: RustPayloadFieldFact[] = [];
    for (const [rawStart, rawEnd] of rustTokenSegments(this.tokens, this.pairs, boundary + 1, close, ",")) {
      const attributes = rustAttributes(this.tokens, this.pairs, rawStart, rawEnd);
      const fieldVisibility = rustVisibility(this.tokens, this.pairs, attributes.next);
      const colon = rustFindTopLevel(this.tokens, this.pairs, fieldVisibility.next, rawEnd, new Set([":"]));
      if (shape === "{") {
        const fieldName = this.tokens[fieldVisibility.next];
        if (fieldName?.kind === "identifier" && colon >= 0) fields.push({ name: fieldName.text, type: rustTokenText(this.tokens, colon + 1, rawEnd) });
      } else fields.push({ name: null, type: rustTokenText(this.tokens, fieldVisibility.next, rawEnd) });
    }
    this.inlinePayloads.push({ name, visibility: visibility.value, fieldStyle: shape === "{" ? "struct" : "tuple", fields });
    return close + 1;
  }

  private parseImpl(keyword: number, end: number): number {
    const open = rustFindTopLevel(this.tokens, this.pairs, keyword + 1, end, new Set(["{"]));
    const close = open < 0 ? undefined : this.pairs.get(open);
    if (open < 0 || close === undefined) return end;
    let angleDepth = 0;
    let forIndex = -1;
    for (let index = keyword + 1; index < open; index += 1) {
      const text = this.tokens[index]!.text;
      if (text === "<") angleDepth += 1;
      else if (text === ">") angleDepth = Math.max(0, angleDepth - 1);
      else if (text === ">>") angleDepth = Math.max(0, angleDepth - 2);
      else if (text === "for" && angleDepth === 0) forIndex = index;
    }
    const headerStart = keyword + 1;
    const traitPath = forIndex < 0 ? null : rustTokenText(this.tokens, headerStart, forIndex).replace(/^!/, "");
    const selfType = rustTokenText(this.tokens, forIndex < 0 ? headerStart : forIndex + 1, open);
    const methods: string[] = [];
    const associatedConstants: string[] = [];
    let index = open + 1;
    while (index < close) {
      const attributes = rustAttributes(this.tokens, this.pairs, index, close);
      const visibility = rustVisibility(this.tokens, this.pairs, attributes.next);
      const token = this.tokens[visibility.next]?.text;
      if (token === "fn" || (token === "async" && this.tokens[visibility.next + 1]?.text === "fn")) {
        const nameIndex = visibility.next + (token === "async" ? 2 : 1);
        if (this.tokens[nameIndex]?.kind === "identifier") methods.push(this.tokens[nameIndex]!.text);
        index = this.skipItem(attributes.next, close);
      } else if (token === "const") {
        const name = this.tokens[visibility.next + 1]?.text;
        if (name) associatedConstants.push(name);
        index = this.parseConst(visibility.next, close, selfType);
      } else index = this.skipItem(attributes.next, close);
    }
    this.impls.push({ traitPath, selfType, methods, associatedConstants });
    return close + 1;
  }

  private parseConst(keyword: number, end: number, owner: string | null): number {
    const name = this.tokens[keyword + 1]?.text ?? "";
    const semicolon = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, end, new Set([";"]));
    if (semicolon < 0) return end;
    const colon = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, semicolon, new Set([":"]));
    const equals = rustFindTopLevel(this.tokens, this.pairs, keyword + 2, semicolon, new Set(["="]));
    const valueStart = equals < 0 ? semicolon : equals + 1;
    const value = rustTokenText(this.tokens, valueStart, semicolon);
    this.constants.push({
      owner,
      name,
      type: colon < 0 || equals < 0 ? null : rustTokenText(this.tokens, colon + 1, equals),
      value,
      stringValue: valueStart + 1 === semicolon ? rustStringValue(this.tokens[valueStart]) : null,
      identityFields: rustConstIdentityFields(this.tokens, this.pairs, valueStart, semicolon),
    });
    return semicolon + 1;
  }
}

/** 🎯️ Finds every token-aware match arm, including nested function bodies, without scanning literals. */
function rustMatchArms(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): RustMatchArmFact[] {
  const facts: RustMatchArmFact[] = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index]!.kind !== "identifier" || tokens[index]!.text !== "match") continue;
    const open = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set(["{"]));
    const close = open < 0 ? undefined : pairs.get(open);
    if (open < 0 || close === undefined) continue;
    for (const [armStart, armEnd] of rustTokenSegments(tokens, pairs, open + 1, close, ",")) {
      const arrow = rustFindTopLevel(tokens, pairs, armStart, armEnd, new Set(["=>"]));
      if (arrow < 0) continue;
      facts.push({ pattern: rustTokenText(tokens, armStart, arrow), variantPath: rustPatternVariantPath(tokens, armStart, arrow), expression: rustTokenText(tokens, arrow + 1, armEnd) });
    }
  }
  return facts;
}

/** 📦️ Finds include macros and explicitly records whether their token tree reaches `OUT_DIR`. */
function rustIncludes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): RustIncludeFact[] {
  const facts: RustIncludeFact[] = [];
  const names = new Set(["include", "include_str", "include_bytes"] as const);
  for (let index = 0; index + 2 < tokens.length; index += 1) {
    const name = tokens[index]!.text as RustIncludeFact["macro"];
    if (!names.has(name) || tokens[index + 1]!.text !== "!" || tokens[index + 2]!.text !== "(") continue;
    const close = pairs.get(index + 2);
    if (close === undefined) continue;
    const expressionTokens = tokens.slice(index + 3, close);
    facts.push({
      macro: name,
      expression: rustTokenText(tokens, index + 3, close),
      usesOutDir: expressionTokens.some((token) => (token.kind === "identifier" && token.text === "OUT_DIR") || rustStringValue(token) === "OUT_DIR"),
    });
  }
  return facts;
}

/** 🧩️One literal child-kind attribute with its source line. */
export interface RustChildKindMetadataFact {
  readonly kind: string;
  readonly line: number;
}

/** 🔗️Reads child-kind attributes from Rust tokens without treating comments or literal contents as declarations. */
export function inspectRustChildKindMetadata(source: string): readonly RustChildKindMetadataFact[] {
  const tokens = rustTokens(source);
  const facts: RustChildKindMetadataFact[] = [];
  const expected = ["#", "[", "child", "(", "kind", "="];
  for (let index = 0; index + 8 < tokens.length; index += 1) {
    if (!expected.every((text, offset) => tokens[index + offset]?.text === text)) continue;
    if (tokens[index + 7]?.text !== ")" || tokens[index + 8]?.text !== "]") continue;
    const kind = rustStringValue(tokens[index + 6]);
    if (kind === null) continue;
    facts.push({ kind, line: source.slice(0, tokens[index]!.start).split("\n").length });
    index += 8;
  }
  return facts;
}

/** 🧠️ Extracts a stable schema-versioned structural report from one Rust source string. */
export function inspectRustStructure(source: string): RustStructuralFacts {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const parser = new RustStructureParser(tokens, pairs);
  parser.parse();
  const modules = parser.modules;
  return {
    schemaVersion: 1,
    modules,
    enums: parser.enums,
    impls: parser.impls,
    inlinePayloads: parser.inlinePayloads,
    matchArms: rustMatchArms(tokens, pairs),
    constants: parser.constants,
    includes: rustIncludes(tokens, pairs),
    testModules: modules.filter((module) => module.cfgTest),
  };
}

export interface RustMutationMetadataDeclarationFact { readonly name: string; readonly kind: "struct" | "enum" | "union"; readonly visibility: RustStructuralVisibility; readonly modulePath: readonly string[]; readonly derives: readonly string[]; readonly conditional?: true; readonly mutationLeaf: { readonly state: "absent" | "valid" | "malformed" | "ambiguous" | "conditional"; readonly contracts: readonly string[]; readonly detail: string | null }; }

export interface RustCrateAliasFact { readonly kind: "extern" | "self" | "use" | "reexport"; readonly source: string; readonly alias: string; readonly modulePath: readonly string[]; readonly conditional: boolean; readonly restricted?: true; }

export interface RustMutationMetadataFacts { readonly schemaVersion: 1; readonly declarations: readonly RustMutationMetadataDeclarationFact[]; readonly crateAliases: readonly RustCrateAliasFact[]; readonly manualMutationLeafImpls: readonly RustImplFact[]; }

/** 🧭️ Produces fail-closed `mutation_leaf` evidence from exact attribute tokens. */
function rustMutationLeafEvidence(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: readonly RustMetadataAttributeFact[], inheritedConditional: boolean): RustMutationMetadataDeclarationFact["mutationLeaf"] {
  const metadata = attributes.filter((item) => item.name === "mutation_leaf");
  if (metadata.length === 0) return { state: "absent", contracts: [], detail: null };
  const contracts: string[] = [];
  let malformed = false;
  for (const attribute of metadata) {
    const head = rustMetadataAttributeHead(tokens, pairs, attribute.start, attribute.end);
    if (!head?.arguments) { malformed = true; continue; }
    const entries = rustTokenSegments(tokens, pairs, head.arguments[0], head.arguments[1], ",");
    let contractCount = 0;
    for (const [start, end] of entries) {
      const equals = rustFindTopLevel(tokens, pairs, start, end, new Set(["="]));
      if (tokens[start]?.text !== "contract" || equals !== start + 1) { malformed = true; continue; }
      contractCount += 1;
      const contract = rustMetadataPath(tokens, equals + 1, end, true);
      if (contract === null || contract.split("::").some((segment) => ["crate", "self", "super"].includes(segment))) malformed = true;
      else contracts.push(contract);
    }
    if (contractCount !== 1 || entries.length !== 1) malformed = true;
  }
  const conditional = inheritedConditional || metadata.some((item) => item.conditional);
  if (metadata.length > 1 || contracts.length > 1) return { state: "ambiguous", contracts, detail: "duplicate or conflicting mutation_leaf attributes" };
  if (conditional) return { state: "conditional", contracts, detail: "conditional mutation_leaf metadata" };
  if (malformed || contracts.length !== 1) return { state: "malformed", contracts, detail: "invalid mutation_leaf syntax" };
  return { state: "valid", contracts, detail: null };
}

/** 🔀️ Expands one tokenized Rust use tree into exact imported aliases. */
function rustUseAliases(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): readonly { readonly source: string; readonly alias: string }[] {
  const aliases: { source: string; alias: string }[] = [];
  const parse = (segmentStart: number, segmentEnd: number, prefix: readonly string[], absolute: boolean): void => {
    let index = segmentStart;
    if (tokens[index]?.text === "{") {
      const close = pairs.get(index);
      if (close === undefined || close !== segmentEnd - 1) return;
      for (const [childStart, childEnd] of rustTokenSegments(tokens, pairs, index + 1, close, ",")) parse(childStart, childEnd, prefix, absolute);
      return;
    }
    if (tokens[index]?.text === "::") { if (prefix.length > 0) return; absolute = true; index += 1; }
    const parts: string[] = [];
    while (tokens[index]?.kind === "identifier") {
      parts.push(tokens[index]!.text);
      index += 1;
      if (tokens[index]?.text !== "::") break;
      index += 1;
      if (tokens[index]?.text === "{") {
        const close = pairs.get(index);
        if (close === undefined || close !== segmentEnd - 1) return;
        for (const [childStart, childEnd] of rustTokenSegments(tokens, pairs, index + 1, close, ",")) parse(childStart, childEnd, [...prefix, ...parts], absolute);
        return;
      }
      if (tokens[index]?.text === "*" && index + 1 === segmentEnd && parts.length > 0) {
        const source = `${absolute ? "::" : ""}${[...prefix, ...parts].join("::")}`;
        aliases.push({ source, alias: "*" });
        return;
      }
      if (tokens[index]?.kind !== "identifier") return;
    }
    if (parts.length === 0) return;
    let alias: string;
    if (tokens[index]?.text === "as" && tokens[index + 1]?.kind === "identifier" && index + 2 === segmentEnd) alias = tokens[index + 1]!.text;
    else if (index === segmentEnd) alias = parts.at(-1) === "self" ? prefix.at(-1) ?? "self" : parts.at(-1)!;
    else return;
    const sourceParts = parts.at(-1) === "self" ? prefix : [...prefix, ...parts];
    const source = `${absolute ? "::" : ""}${sourceParts.join("::")}`;
    if (sourceParts.length > 0 && alias !== "_" && !["::crate", "::self", "::super"].some((root) => source === root || source.startsWith(`${root}::`))) aliases.push({ source, alias });
  };
  parse(start, end, [], false);
  return aliases;
}

/** 🧬️ Returns token-derived declaration and alias evidence; it deliberately does not resolve crates. */
export function inspectRustMutationMetadataFacts(source: string): RustMutationMetadataFacts {
  const tokens = rustTokens(source), pairs = rustTokenPairs(tokens), declarations: RustMutationMetadataDeclarationFact[] = [], crateAliases: RustCrateAliasFact[] = [];
  const scope = (start: number, end: number, modulePath: readonly string[], inheritedConditional: boolean): void => {
    let index = start, scopeConditional = inheritedConditional;
    while (index + 2 < end && tokens[index]?.text === "#" && tokens[index + 1]?.text === "!" && tokens[index + 2]?.text === "[") {
      const close = pairs.get(index + 2);
      if (close === undefined || close >= end) return;
      scopeConditional ||= rustMetadataAttributes(tokens, pairs, { ranges: [[index + 3, close]], next: close + 1 }).conditional;
      index = close + 1;
    }
    for (; index < end;) {
      const rawAttributes = rustAttributes(tokens, pairs, index, end), metadataAttributes = rustMetadataAttributes(tokens, pairs, rawAttributes), visibility = rustVisibility(tokens, pairs, rawAttributes.next), keyword = tokens[visibility.next]?.text, conditional = scopeConditional || metadataAttributes.conditional;
      if (keyword === "mod" && tokens[visibility.next + 1]?.kind === "identifier") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";", "{"]));
        if (boundary < 0) return;
        if (tokens[boundary]?.text === "{") {
          const close = pairs.get(boundary);
          if (close !== undefined) scope(boundary + 1, close, [...modulePath, tokens[visibility.next + 1]!.text], conditional);
          index = (close ?? boundary) + 1;
          continue;
        }
        index = boundary + 1;
        continue;
      }
      if (keyword === "extern" && tokens[visibility.next + 1]?.text === "crate") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";"]));
        const sourceToken = tokens[visibility.next + 2], aliasToken = tokens[visibility.next + 3]?.text === "as" ? tokens[visibility.next + 4] : sourceToken;
        if (boundary >= 0 && sourceToken?.kind === "identifier" && aliasToken?.kind === "identifier" && (aliasToken === sourceToken ? visibility.next + 3 === boundary : visibility.next + 5 === boundary)) crateAliases.push({ kind: sourceToken.text === "self" ? "self" : "extern", source: sourceToken.text, alias: aliasToken.text, modulePath, conditional, ...(visibility.value === "pub" ? {} : { restricted: true as const }) });
        index = boundary < 0 ? end : boundary + 1;
        continue;
      }
      if (keyword === "use") {
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 1, end, new Set([";"]));
        if (boundary < 0) return;
        for (const { source: aliasSource, alias } of rustUseAliases(tokens, pairs, visibility.next + 1, boundary)) crateAliases.push({ kind: visibility.value === "private" ? "use" : "reexport", source: aliasSource, alias, modulePath, conditional, ...(visibility.value === "pub" ? {} : { restricted: true as const }) });
        index = boundary + 1;
        continue;
      }
      if (["struct", "enum", "union"].includes(keyword ?? "") && tokens[visibility.next + 1]?.kind === "identifier") declarations.push({ name: tokens[visibility.next + 1]!.text, kind: keyword as "struct" | "enum" | "union", visibility: visibility.value, modulePath, derives: rustMetadataDerives(tokens, pairs, metadataAttributes.items), ...(conditional ? { conditional: true as const } : {}), mutationLeaf: rustMutationLeafEvidence(tokens, pairs, metadataAttributes.items, conditional) });
      const boundary = rustFindTopLevel(tokens, pairs, rawAttributes.next, end, new Set([";", "{"]));
      index = boundary < 0 ? end : tokens[boundary]?.text === "{" ? (pairs.get(boundary) ?? boundary) + 1 : boundary + 1;
    }
  };
  scope(0, tokens.length, [], false);
  const structural = inspectRustStructure(source);
  return { schemaVersion: 1, declarations, crateAliases, manualMutationLeafImpls: structural.impls.filter((item) => item.traitPath?.split("::").at(-1) === "MutationLeaf") };
}
