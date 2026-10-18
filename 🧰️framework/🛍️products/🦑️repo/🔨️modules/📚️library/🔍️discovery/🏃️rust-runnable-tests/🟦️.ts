import { type RustAttributes, type RustToken, rustAttributes, rustFindTopLevel, rustPathAttributes, rustStringValue, rustTokenPairs, rustTokenSegments, rustTokens, rustVisibility } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import type { RustRunnableTestFact, RustTestFacts, RustTestModuleFact } from "../🦀️rust-fact-contracts/🟦️.ts";

/** 📍️ Reads a single decoded module mount from actual Rust path metadata. */
export function rustPathAttribute(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: RustAttributes): string | null {
  return rustPathAttributes(tokens, pairs, attributes)[0]?.path ?? null;
}

/** 🧪️ Identifies a test-only module from tokenized `cfg(...test...)` attributes. */
export function rustCfgTest(tokens: readonly RustToken[], attributes: RustAttributes): boolean {
  return attributes.ranges.some(([start, end]) => tokens[start]?.text === "cfg" && tokens.slice(start + 1, end).some((token) => token.kind === "identifier" && token.text === "test"));
}

type RustTestConfiguration = "enabled" | "disabled" | "ambiguous";

interface RustTestAttributes {
  readonly configuration: RustTestConfiguration;
  readonly test: boolean;
  readonly ignored: boolean;
  readonly pathTarget: string | null;
}

/** 🧪️ Combines Rust test configurations without treating unknown cfg values as enabled. */
function rustTestConfigurationAnd(left: RustTestConfiguration, right: RustTestConfiguration): RustTestConfiguration {
  if (left === "disabled" || right === "disabled") return "disabled";
  if (left === "ambiguous" || right === "ambiguous") return "ambiguous";
  return "enabled";
}

/** 🧪️ Negates the test-mode truth value of one parsed cfg predicate. */
function rustTestConfigurationNot(value: RustTestConfiguration): RustTestConfiguration {
  return value === "enabled" ? "disabled" : value === "disabled" ? "enabled" : "ambiguous";
}

/** 🧪️ Evaluates only cfg predicates whose truth is proven by rustc --test. */
function rustTestConfigurationExpression(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): RustTestConfiguration {
  const head = tokens[start];
  if (!head || head.kind !== "identifier") return "ambiguous";
  if (head.text === "test" && start + 1 === end) return "enabled";
  const open = tokens[start + 1]?.text === "(" ? start + 1 : -1;
  const close = open < 0 ? undefined : pairs.get(open);
  if (close === undefined || close + 1 !== end) return "ambiguous";
  const parts = rustTokenSegments(tokens, pairs, open + 1, close, ",");
  if (head.text === "not" && parts.length === 1) return rustTestConfigurationNot(rustTestConfigurationExpression(tokens, pairs, ...parts[0]!));
  if (head.text === "all") return parts.reduce<RustTestConfiguration>((value, part) => rustTestConfigurationAnd(value, rustTestConfigurationExpression(tokens, pairs, ...part)), "enabled");
  if (head.text === "any") {
    const values = parts.map((part) => rustTestConfigurationExpression(tokens, pairs, ...part));
    if (values.includes("enabled")) return "enabled";
    return values.every((value) => value === "disabled") ? "disabled" : "ambiguous";
  }
  return "ambiguous";
}

/** 🧪️ Reads consecutive inner attributes that configure their enclosing module or crate scope. */
function rustInnerAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): RustAttributes {
  const ranges: (readonly [number, number])[] = [];
  let index = start;
  while (index + 2 < end && tokens[index]!.text === "#" && tokens[index + 1]!.text === "!" && tokens[index + 2]!.text === "[") {
    const close = pairs.get(index + 2);
    if (close === undefined || close >= end) break;
    ranges.push([index + 3, close]);
    index = close + 1;
  }
  return { ranges, next: index };
}

/** 🧪️ Recognizes whether an unresolved cfg_attr could change a test's executability or source identity. */
function rustTestAttributeCanAffectExecution(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, start: number, end: number): boolean {
  const name = tokens[start]?.text;
  if (name === "cfg" || name === "ignore" || name === "path") return true;
  if (name !== "cfg_attr" || tokens[start + 1]?.text !== "(") return true;
  const close = pairs.get(start + 1);
  if (close === undefined || close + 1 !== end) return true;
  const parts = rustTokenSegments(tokens, pairs, start + 2, close, ",");
  return parts.slice(1).some((part) => rustTestAttributeCanAffectExecution(tokens, pairs, ...part));
}

/** 🧪️ Resolves direct and test-enabled cfg_attr attributes without assuming unknown cfg values. */
function rustTestAttributes(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, attributes: RustAttributes): RustTestAttributes {
  let configuration: RustTestConfiguration = "enabled";
  let test = false;
  let ignored = false;
  let pathTarget: string | null = null;
  const apply = (start: number, end: number, fromCfgAttr = false): void => {
    const name = tokens[start]?.text;
    if (name === "cfg") {
      if (tokens[start + 1]?.text !== "(") { configuration = rustTestConfigurationAnd(configuration, "ambiguous"); return; }
      const close = pairs.get(start + 1);
      configuration = rustTestConfigurationAnd(configuration, close === undefined || close + 1 !== end ? "ambiguous" : rustTestConfigurationExpression(tokens, pairs, start + 2, close));
      return;
    }
    if (name === "test") { test = true; return; }
    if (name === "ignore") { ignored = true; return; }
    if (name === "path") {
      const equals = tokens[start + 1]?.text === "=" ? start + 1 : -1;
      const target = equals < 0 ? null : rustStringValue(tokens[equals + 1]);
      if (target === null || pathTarget !== null) configuration = rustTestConfigurationAnd(configuration, "ambiguous");
      else pathTarget = target;
      return;
    }
    if (name !== "cfg_attr" || tokens[start + 1]?.text !== "(") {
      if (fromCfgAttr) configuration = rustTestConfigurationAnd(configuration, "ambiguous");
      return;
    }
    const close = pairs.get(start + 1);
    if (close === undefined || close + 1 !== end) { configuration = rustTestConfigurationAnd(configuration, "ambiguous"); return; }
    const parts = rustTokenSegments(tokens, pairs, start + 2, close, ",");
    if (parts.length < 2) { configuration = rustTestConfigurationAnd(configuration, "ambiguous"); return; }
    const condition = rustTestConfigurationExpression(tokens, pairs, ...parts[0]!);
    if (condition === "enabled") for (const part of parts.slice(1)) apply(...part, true);
    else if (condition === "ambiguous" && parts.slice(1).some((part) => rustTestAttributeCanAffectExecution(tokens, pairs, ...part))) configuration = rustTestConfigurationAnd(configuration, "ambiguous");
  };
  for (const range of attributes.ranges) apply(...range);
  return { configuration, test, ignored, pathTarget };
}

/** 🧪️ Preserves only an exact, leaf-local path spelling for an inline module's child mount base. */
function rustTestInlineMountBase(target: string | null, name: string): readonly string[] | null {
  if (target === null) return [name];
  if (!target || target.includes("\0") || target.includes("\\") || target.startsWith("/") || /^[A-Za-z]:/u.test(target)) return null;
  const segments = target.split("/");
  return segments.some((segment) => !segment || segment === "." || segment === "..") ? null : segments;
}

/** 🧪️ Extracts only top-level, enabled, non-ignored test functions and their explicit mounted modules. */
export function inspectRustRunnableTests(source: string): RustTestFacts {
  const tokens = rustTokens(source);
  const pairs = rustTokenPairs(tokens);
  const runnableTests: RustRunnableTestFact[] = [];
  const mountedModules: RustTestModuleFact[] = [];
  const skipItem = (start: number, end: number): number => {
    const boundary = rustFindTopLevel(tokens, pairs, start, end, new Set([";", "{"]));
    if (boundary < 0) return end;
    if (tokens[boundary]!.text === ";") return boundary + 1;
    return (pairs.get(boundary) ?? end - 1) + 1;
  };
  const parseScope = (start: number, end: number, modulePath: readonly string[], mountBase: readonly string[], inherited: RustTestConfiguration): void => {
    const inner = rustInnerAttributes(tokens, pairs, start, end);
    const scopeConfiguration = rustTestConfigurationAnd(inherited, rustTestAttributes(tokens, pairs, inner).configuration);
    for (let index = inner.next; index < end;) {
      const attributes = rustAttributes(tokens, pairs, index, end);
      const visibility = rustVisibility(tokens, pairs, attributes.next);
      const keyword = tokens[visibility.next]?.text;
      const itemAttributes = rustTestAttributes(tokens, pairs, attributes);
      const configuration = rustTestConfigurationAnd(scopeConfiguration, itemAttributes.configuration);
      if (keyword === "mod") {
        const name = tokens[visibility.next + 1];
        const boundary = rustFindTopLevel(tokens, pairs, visibility.next + 2, end, new Set([";", "{"]));
        if (!name || name.kind !== "identifier" || boundary < 0) return;
        const childPath = [...modulePath, name.text];
        const inline = tokens[boundary]!.text === "{";
        if (!inline) mountedModules.push({ name: name.text, modulePath: childPath, mountBase, pathTarget: itemAttributes.pathTarget, configuration });
        if (!inline) { index = boundary + 1; continue; }
        const close = pairs.get(boundary);
        if (close === undefined) return;
        const inlineBase = rustTestInlineMountBase(itemAttributes.pathTarget, name.text);
        parseScope(boundary + 1, close, childPath, inlineBase === null ? mountBase : [...mountBase, ...inlineBase], inlineBase === null ? rustTestConfigurationAnd(configuration, "ambiguous") : configuration);
        index = close + 1;
        continue;
      }
      if (keyword === "fn") {
        const name = tokens[visibility.next + 1];
        if (name?.kind === "identifier" && configuration === "enabled" && itemAttributes.test && !itemAttributes.ignored) runnableTests.push({ name: name.text, modulePath });
      }
      index = skipItem(attributes.next, end);
    }
  };
  parseScope(0, tokens.length, [], [], "enabled");
  return { schemaVersion: 1, runnableTests, mountedModules };
}
