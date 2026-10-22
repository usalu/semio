import { rustFindTopLevel, rustTokenPairs, rustTokens, type RustToken } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { useLeafNames } from "../🌳️module-tree/🟦️.ts";
import { runtimeParseTomlV1 } from "../../../../../../../🔨️modules/⚙️runtime/📄️parse/🟦️.ts";

/** 🧰️ A feature of a carved crate with the entries it enables. */
export type TestkitFeature = Readonly<{ name: string; entries: readonly string[] }>;

/** 🔑️ The dependency key of one manifest row (`name = …` and the dotted `name.workspace = true`). */
export function dependencyRowKey(row: string): string {
  return /^\s*([A-Za-z0-9_-]+)(?:\.[A-Za-z0-9_-]+)?\s*=/u.exec(row)?.[1] ?? "";
}

/** 🪪️ The crate identifier a dependency key is written as in code. */
export function dependencyIdent(key: string): string {
  return key.replaceAll("-", "_");
}

type Predicate = { kind: "atom"; name: string } | { kind: "value"; name: string; value: string } | { kind: "call"; name: string; args: Predicate[] };

function parsePredicate(text: string, from: number): { node: Predicate; next: number } | null {
  let at = from;
  const skip = (): void => { while (/\s/u.test(text[at] ?? "")) at += 1; };
  skip();
  const name = /^[A-Za-z_][A-Za-z0-9_]*/u.exec(text.slice(at))?.[0];
  if (name === undefined) return null;
  at += name.length;
  skip();
  if (text[at] === "=") {
    at += 1;
    skip();
    const value = /^"((?:[^"\\]|\\.)*)"/u.exec(text.slice(at));
    if (!value) return null;
    return { node: { kind: "value", name, value: value[1]! }, next: at + value[0].length };
  }
  if (text[at] === "(") {
    at += 1;
    const args: Predicate[] = [];
    for (;;) {
      skip();
      if (at >= text.length) return null;
      if (text[at] === ")") { at += 1; break; }
      const arg = parsePredicate(text, at);
      if (!arg) return null;
      args.push(arg.node);
      at = arg.next;
      skip();
      if (text[at] === ",") at += 1;
    }
    return { node: { kind: "call", name, args }, next: at };
  }
  return { node: { kind: "atom", name }, next: at };
}

/** 🚩️ The features that a `cfg(...)` attribute text (without `#[` and `]`) uses as a test-support gate: the `feature = "x"` arguments of an `any(test, …)`, and a feature named `testkit` anywhere outside `not(...)`. Every other feature (a real build feature, a feature under `all(test, …)`) is not a test-support gate. */
export function testGateFeatures(attribute: string): string[] {
  const text = attribute.trim();
  if (!/^cfg\s*\(/u.test(text)) return [];
  const parsed = parsePredicate(text, 0);
  const found = new Set<string>();
  const visit = (node: Predicate, negated: boolean): void => {
    if (node.kind === "value") { if (!negated && node.name === "feature" && node.value === "testkit") found.add(node.value); return; }
    if (node.kind !== "call") return;
    if (node.name === "any" && !negated && node.args.some((arg) => arg.kind === "atom" && arg.name === "test")) for (const arg of node.args) if (arg.kind === "value" && arg.name === "feature") found.add(arg.value);
    for (const arg of node.args) visit(arg, negated || node.name === "not");
  };
  if (parsed) visit(parsed.node, false);
  return [...found];
}

/** 🚩️ Every feature that a `cfg(...)` attribute text requires: the `feature = "x"` arguments outside `not(...)`. */
export function cfgFeatures(attribute: string): string[] {
  const text = attribute.trim();
  if (!/^cfg\s*\(/u.test(text)) return [];
  const parsed = parsePredicate(text, 0);
  const found = new Set<string>();
  const visit = (node: Predicate, negated: boolean): void => {
    if (node.kind === "value") { if (!negated && node.name === "feature") found.add(node.value); return; }
    if (node.kind === "call") for (const arg of node.args) visit(arg, negated || node.name === "not");
  };
  if (parsed) visit(parsed.node, false);
  return [...found];
}

/** 🚪️ The `cfg(...)` attribute text of an item that exists only for tests (`cfg(test)` or a test-support gate such as `cfg(any(test, feature = "testkit"))`), or `null` for an item that is built in every configuration; a re-export of the item must carry the same attribute. */
export function testOnlyGate(attributeTexts: readonly string[]): string | null {
  return attributeTexts.find((attribute) => /^cfg\(\s*test\s*\)$/u.test(attribute.trim()) || testGateFeatures(attribute).length > 0) ?? null;
}

/** 🚧️ One item, statement or method behind a test-support gate. */
export type TestGate = Readonly<{ features: readonly string[]; start: number; end: number; names: readonly string[]; method: boolean; owner: string | null; use: boolean }>;

const ITEM_KEYWORDS: ReadonlySet<string> = new Set(["fn", "struct", "enum", "type", "trait", "union", "static", "const", "mod"]);
const QUALIFIERS: ReadonlySet<string> = new Set(["async", "unsafe", "extern", "default", "safe"]);

function gateNames(text: string, tokens: readonly RustToken[], from: number, to: number): { names: string[]; use: boolean } {
  let index = from;
  const exported = tokens[from]?.text === "pub" && tokens[from + 1]?.text !== "(";
  if (tokens[index]?.text === "pub") {
    index += 1;
    if (tokens[index]?.text === "(") {
      let depth = 0;
      for (; index < to; index += 1) {
        if (tokens[index]!.text === "(") depth += 1;
        else if (tokens[index]!.text === ")" && --depth === 0) { index += 1; break; }
      }
    }
  }
  while (index < to && (QUALIFIERS.has(tokens[index]!.text) || tokens[index]!.kind === "string" || (tokens[index]!.text === "const" && ["fn", "unsafe", "async", "extern"].includes(tokens[index + 1]?.text ?? "")))) index += 1;
  const word = tokens[index]?.text;
  if (word === "use") return { names: exported ? useLeafNames(text.slice(tokens[from]!.start, tokens[to - 1]!.end)) : [], use: true };
  if (word === "macro_rules" && tokens[index + 1]?.text === "!" && tokens[index + 2]) return { names: [tokens[index + 2]!.text], use: false };
  if (word !== undefined && ITEM_KEYWORDS.has(word) && exported) {
    let at = index + 1;
    if (tokens[at]?.text === "mut") at += 1;
    return { names: tokens[at]?.kind === "identifier" ? [tokens[at]!.text] : [], use: false };
  }
  return { names: [], use: false };
}

function implOwner(tokens: readonly RustToken[], keyword: number, open: number): string | null {
  let from = keyword + 1;
  if (tokens[keyword]!.text === "trait") return tokens[from]?.kind === "identifier" ? tokens[from]!.text : null;
  if (tokens[from]?.text === "<") {
    let depth = 0;
    for (; from < open; from += 1) { for (const char of tokens[from]!.text) { if (char === "<") depth += 1; else if (char === ">") depth -= 1; } if (depth <= 0) { from += 1; break; } }
  }
  let at = from;
  for (let index = from; index < open; index += 1) if (tokens[index]!.text === "for") at = index + 1;
  let name: string | null = null;
  for (let index = at; index < open; index += 1) {
    const token = tokens[index]!;
    if (token.kind === "identifier" && token.text !== "where" && token.text !== "dyn" && token.text !== "mut" && token.text !== "const") name = token.text;
    else if (token.text === "<" || token.text === "where") break;
  }
  return name;
}

function implBodies(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>): [number, number, string | null][] {
  const bodies: [number, number, string | null][] = [];
  tokens.forEach((token, index) => {
    if (token.text !== "impl" && token.text !== "trait") return;
    if (tokens[index - 1]?.text === "::" || tokens[index - 1]?.text === ".") return;
    const open = rustFindTopLevel(tokens, pairs, index + 1, tokens.length, new Set(["{", ";"]));
    if (open >= 0 && tokens[open]!.text === "{" && pairs.get(open) !== undefined) bodies.push([open, pairs.get(open)!, implOwner(tokens, index, open)]);
  });
  return bodies;
}

function methodNames(tokens: readonly RustToken[], pairs: ReadonlyMap<number, number>, first: number): string[] {
  let at = first;
  if (tokens[at]?.text !== "pub") return [];
  at += 1;
  if (tokens[at]?.text === "(") return [];
  while (QUALIFIERS.has(tokens[at]?.text ?? "") || tokens[at]?.kind === "string" || tokens[at]?.text === "const") at += 1;
  void pairs;
  return tokens[at]?.text === "fn" && tokens[at + 1]?.kind === "identifier" ? [tokens[at + 1]!.text] : [];
}

/** 🔭️ Finds every item behind a test-support gate in a Rust file: the attribute's features, the span of the gated item up to its closing brace or semicolon, the names it declares (the leaf names of a `use`) and whether it sits in an `impl` or `trait` body (a method). */
export function scanTestGates(text: string, featuresOf: (attribute: string) => string[] = testGateFeatures): TestGate[] {
  if (!text.includes("cfg(")) return [];
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const bodies = implBodies(tokens, pairs);
  const gates: TestGate[] = [];
  for (let index = 0; index + 3 < tokens.length; index += 1) {
    if (tokens[index]!.text !== "#" || tokens[index + 1]!.text !== "[" || tokens[index + 2]!.text !== "cfg" || tokens[index + 3]!.text !== "(") continue;
    const close = pairs.get(index + 1);
    if (close === undefined) continue;
    const features = featuresOf(text.slice(tokens[index + 2]!.start, tokens[close]!.start));
    if (!features.length) continue;
    let first = close + 1;
    while (tokens[first]?.text === "#" && tokens[first + 1]?.text === "[" && pairs.get(first + 1) !== undefined) first = pairs.get(first + 1)! + 1;
    if (first >= tokens.length) continue;
    const boundary = rustFindTopLevel(tokens, pairs, first, tokens.length, new Set([";", "{"]));
    let last = boundary < 0 ? tokens.length : tokens[boundary]!.text === ";" ? boundary + 1 : (pairs.get(boundary) ?? tokens.length - 1) + 1;
    if (boundary >= 0 && tokens[boundary]!.text === "{" && tokens[last]?.text === ";") last += 1;
    const enclosing = bodies.find(([open, end]) => index > open && index < end);
    const described = gateNames(text, tokens, first, last);
    gates.push({ features, start: tokens[index]!.start, end: tokens[last - 1]!.end, names: enclosing ? methodNames(tokens, pairs, first) : described.names, method: enclosing !== undefined, owner: enclosing?.[2] ?? null, use: described.use });
  }
  return gates;
}

/** 🧩️ One carved crate as the wiring analysis sees it: the text of every source file of its library and test closures, the test gates found per file, and, per gate feature, the files that its closure compiles only when that feature is on. */
export type TestkitCrate = Readonly<{
  name: string;
  ident: string;
  dependsOn: readonly string[];
  devDependsOn?: readonly string[];
  ownedFiles?: ReadonlySet<string>;
  files: ReadonlyMap<string, string>;
  gates: ReadonlyMap<string, readonly TestGate[]>;
  featureFiles: ReadonlyMap<string, ReadonlySet<string>>;
}>;

/** 🧩️ Scans the files of one crate for test gates and asks `featureClosure` for the paths that the crate's closure adds when a gate feature is on. */
export async function buildTestkitCrate(spec: Readonly<{ name: string; ident: string; dependsOn: readonly string[]; devDependsOn?: readonly string[]; ownedFiles?: ReadonlySet<string>; files: ReadonlyMap<string, string> }>, featureClosure: (feature: string) => Promise<ReadonlySet<string>>): Promise<TestkitCrate> {
  const gates = new Map<string, readonly TestGate[]>();
  const candidates = new Set<string>();
  for (const [path, text] of spec.files) {
    const found = scanTestGates(text);
    if (!found.length) continue;
    gates.set(path, found);
    for (const gate of found) for (const feature of gate.features) candidates.add(feature);
  }
  const featureFiles = new Map<string, ReadonlySet<string>>();
  for (const feature of [...candidates].sort()) featureFiles.set(feature, await featureClosure(feature));
  return { ...spec, gates, featureFiles };
}

/** 🏷️ The names of the items a file declares outside `impl` and `trait` bodies (`fn`, `struct`, `enum`, `trait`, `type`, `const`, `static`, `mod`, `union` and `macro_rules!`). */
export function declaredNames(text: string): Set<string> {
  const tokens = rustTokens(text);
  const bodies = implBodies(tokens, rustTokenPairs(tokens));
  const names = new Set<string>();
  tokens.forEach((token, index) => {
    if (token.kind !== "identifier" || bodies.some(([open, end]) => index > open && index < end)) return;
    if (ITEM_KEYWORDS.has(token.text) && tokens[index + 1]?.kind === "identifier" && tokens[index - 1]?.text !== "::" && tokens[index - 1]?.text !== ".") names.add(tokens[index + 1]!.text === "mut" ? (tokens[index + 2]?.text ?? "") : tokens[index + 1]!.text);
    else if (token.text === "macro_rules" && tokens[index + 1]?.text === "!" && tokens[index + 2]) names.add(tokens[index + 2]!.text);
  });
  for (const word of ["fn", "mut", "unsafe", "async", "extern", "const", "static", "type"]) names.delete(word);
  return names;
}

/** 📦️ One external dependency of the pre-carve package manifest. */
export type ExternalDependency = Readonly<{ key: string; ident: string; dependency: boolean; devOnly: boolean; devFeatures: readonly string[]; devRow: string | null; row: string | null }>;

/** 🚪️ Names behind a feature that a dev-dependency row adds to an external dependency, or `null` when the dependency source is unreadable and every gated name must count as referenced. */
export type ExternalGateReader = (external: ExternalDependency, feature: string) => ReadonlySet<string> | null;

/** 📦️ Reads the external dependencies of a package manifest: which are normal dependencies, which exist only as dev-dependencies, and the features a dev-dependency row adds. */
export function externalDependencies(manifestText: string, rows: Readonly<{ dependencies?: readonly string[]; devDependencies: readonly string[] }>): ExternalDependency[] {
  const parsed = runtimeParseTomlV1(manifestText) as { dependencies?: Record<string, unknown>; "dev-dependencies"?: Record<string, unknown> };
  const normal = parsed.dependencies ?? {};
  const dev = parsed["dev-dependencies"] ?? {};
  const featuresOf = (spec: unknown): string[] => (typeof spec === "object" && spec !== null && Array.isArray((spec as { features?: unknown }).features) ? (spec as { features: unknown[] }).features.map(String) : []);
  const devRows = new Map(rows.devDependencies.map((row) => [dependencyRowKey(row), row]));
  const allRows = new Map([...(rows.dependencies ?? []), ...rows.devDependencies].map((row) => [dependencyRowKey(row), row]));
  return [...new Set([...Object.keys(normal), ...Object.keys(dev)])].sort().map((key) => ({
    key,
    ident: dependencyIdent(key),
    dependency: key in normal,
    devOnly: !(key in normal) && key in dev,
    devFeatures: featuresOf(dev[key]).filter((feature) => !featuresOf(normal[key]).includes(feature)),
    devRow: devRows.get(key) ?? null,
    row: allRows.get(key) ?? null,
  }));
}

/** 🔌️ Rewrites a dependency row so that the dependency is optional. */
export function optionalDependencyRow(row: string): string {
  const key = dependencyRowKey(row);
  const escaped = key;
  const dotted = new RegExp(`^(\\s*)${escaped}\\.workspace\\s*=\\s*true\\s*$`, "u").exec(row);
  if (dotted) return `${dotted[1]}${key} = { workspace = true, optional = true }`;
  const plain = new RegExp(`^(\\s*)${escaped}\\s*=\\s*("[^"]*")\\s*$`, "u").exec(row);
  if (plain) return `${plain[1]}${key} = { version = ${plain[2]}, optional = true }`;
  if (/optional\s*=\s*true/u.test(row)) return row;
  return row.replace(/\s*\}\s*$/u, ", optional = true }");
}

/** 🧭️ The wiring one crate needs: the gate features it declares (enabling external features and optional dependencies, forwarding to the crates whose gated items its gated code names), the dependencies whose gate features its code consumes, the optional dependencies its gated code needs, and the names its own gates declare per feature. */
export type CrateWiring = Readonly<{
  features: readonly TestkitFeature[];
  optionalDependencies: readonly string[];
  consumed: ReadonlyMap<string, readonly string[]>;
  gatedNames: ReadonlyMap<string, ReadonlySet<string>>;
}>;

type GateSlot = { names: Set<string>; reexports: Set<string>; methods: Map<string, Set<string>>; code: string[]; definition: boolean };

function identifiers(text: string): Set<string> {
  const set = new Set<string>();
  for (const token of rustTokens(text)) if (token.kind === "identifier") set.add(token.text);
  return set;
}

function ownedNames(crate: TestkitCrate): Set<string> {
  const names = new Set<string>();
  for (const text of crate.files.values()) for (const name of declaredNames(text)) names.add(name);
  return names;
}

type Usage = { plain: Set<string>; byRoot: Map<string, Set<string>> };

function usageOf(crate: TestkitCrate): Usage {
  const usage: Usage = { plain: new Set(), byRoot: new Map() };
  for (const text of crate.files.values()) {
    const tokens = rustTokens(text);
    for (let index = 0; index < tokens.length; index += 1) {
      const token = tokens[index]!;
      if (token.text === "use" && !["::", ".", "="].includes(tokens[index - 1]?.text ?? "")) {
        let end = index + 1;
        while (end < tokens.length && tokens[end]!.text !== ";") end += 1;
        let first = index + 1;
        if (tokens[first]?.text === "::") first += 1;
        const root = tokens[first]?.text ?? "";
        const slot = usage.byRoot.get(root) ?? new Set<string>();
        usage.byRoot.set(root, slot);
        for (let at = first; at < end; at += 1) if (tokens[at]!.kind === "identifier") slot.add(tokens[at]!.text);
        index = end;
      } else if (token.kind === "identifier") {
        usage.plain.add(token.text);
        if (tokens[index + 1]?.text === "::") {
          const slot = usage.byRoot.get(token.text) ?? new Set<string>();
          usage.byRoot.set(token.text, slot);
          for (let at = index + 2; at < tokens.length && tokens[at]!.kind === "identifier"; at += 2) { slot.add(tokens[at]!.text); if (tokens[at + 1]?.text !== "::") break; }
        }
      }
    }
  }
  return usage;
}

function gateSlots(crate: TestkitCrate, ignore: ReadonlySet<string>, owns: (path: string) => boolean): { slots: Map<string, GateSlot>; aliases: Map<string, string> } {
  const slots = new Map<string, GateSlot>();
  const slot = (feature: string): GateSlot => { let value = slots.get(feature); if (!value) slots.set(feature, (value = { names: new Set(), reexports: new Set(), methods: new Map(), code: [], definition: false })); return value; };
  const aliases = new Map<string, string>();
  for (const [path, text] of crate.files) {
    for (const match of text.matchAll(/extern\s+crate\s+([A-Za-z0-9_]+)\s+as\s+([A-Za-z0-9_]+)\s*;/gu)) aliases.set(match[2]!, match[1]!);
    if (!owns(path)) continue;
    for (const [feature, paths] of crate.featureFiles) if (!ignore.has(feature) && paths.has(path)) slot(feature).code.push(text);
    for (const gate of crate.gates.get(path) ?? []) for (const feature of gate.features) {
      if (ignore.has(feature)) continue;
      const target = slot(feature);
      target.code.push(text.slice(gate.start, gate.end));
      if (!gate.method) for (const name of gate.names) { if (gate.use) target.reexports.add(name); else { target.names.add(name); target.definition = true; } }
      else if (gate.owner !== null) for (const name of gate.names) { target.methods.set(name, new Set([...(target.methods.get(name) ?? []), gate.owner])); target.definition = true; }
    }
  }
  return { slots, aliases };
}

/** 🧭️ Derives the test-support wiring of a carve from the sources. A gated file that several crates mount belongs to the crate whose plan slices contain it, else to the crates that do not depend on another one of them. A crate defines a gate feature when an item other than a re-export sits behind it, and declares it when it defines it or when another crate enables it; a declared feature enables the external features and optional dependencies its gated code needs and forwards to the crates whose gated items that code names. A crate whose gated code names something that a dev-dependency feature of an external crate adds enables the dev-dependency features of every external crate it names. A crate consumes a dependency's feature when its code names an exported gated item of the dependency (a gated method counts when the code names the method and its type) or when its own gated code uses the dependency's crate. Features in `declared` are forwarded to every crate by the plain feature path and are left out here. */
export function planTestkitWiring(crates: readonly TestkitCrate[], externals: readonly ExternalDependency[], declared: ReadonlySet<string>, externalGates: ExternalGateReader = () => null): Map<string, CrateWiring> {
  const byName = new Map(crates.map((crate) => [crate.name, crate]));
  const reach = new Map<string, Set<string>>();
  const reachable = (name: string): Set<string> => {
    let value = reach.get(name);
    if (!value) {
      reach.set(name, (value = new Set()));
      for (const dependency of byName.get(name)?.dependsOn ?? []) { value.add(dependency); for (const deeper of reachable(dependency)) value.add(deeper); }
    }
    return value;
  };
  const holders = new Map<string, string[]>();
  for (const crate of crates) for (const path of crate.gates.keys()) holders.set(path, [...(holders.get(path) ?? []), crate.name]);
  const ownsFile = (crate: TestkitCrate) => (path: string): boolean => {
    const all = holders.get(path);
    if (all === undefined || all.length < 2) return true;
    const planned = all.filter((name) => byName.get(name)!.ownedFiles?.has(path));
    if (planned.length) return planned.includes(crate.name);
    return !all.some((other) => other !== crate.name && reachable(crate.name).has(other));
  };
  const analysis = new Map(crates.map((crate) => [crate.name, gateSlots(crate, declared, ownsFile(crate))]));
  const usages = new Map<string, Usage>();
  const merged = new Map<string, Set<string>>();
  const usageFor = (crate: TestkitCrate, ident: string): Set<string> => {
    const key = `${crate.name}\0${ident}`;
    let value = merged.get(key);
    if (!value) {
      let usage = usages.get(crate.name);
      if (!usage) usages.set(crate.name, (usage = usageOf(crate)));
      merged.set(key, (value = new Set([...usage.plain, ...(usage.byRoot.get(ident) ?? [])])));
    }
    return value;
  };
  const owned = new Map<string, Set<string>>();
  const ownersOf = (crate: TestkitCrate): Set<string> => { let value = owned.get(crate.name); if (!value) owned.set(crate.name, (value = ownedNames(crate))); return value; };
  const consumed = new Map<string, Map<string, string[]>>();
  const enabled = new Set<string>();
  for (const crate of crates) {
    const own = analysis.get(crate.name)!;
    const mine = new Map<string, string[]>();
    for (const dependency of [...crate.dependsOn, ...(crate.devDependsOn ?? [])]) {
      const target = byName.get(dependency);
      if (!target) continue;
      for (const [feature, theirs] of analysis.get(dependency)!.slots) {
        const slot = own.slots.get(feature);
        if (theirs.names.size === 0 && theirs.reexports.size === 0 && theirs.methods.size === 0) continue;
        const used = usageFor(crate, target.ident);
        const rooted = usages.get(crate.name)?.byRoot.get(target.ident) ?? new Set<string>();
        const mine2 = ownersOf(crate);
        const named = [...theirs.names].some((name) => used.has(name) && !mine2.has(name)) || [...theirs.reexports].some((name) => rooted.has(name)) || [...theirs.methods].some(([name, owners]) => used.has(name) && [...owners].some((owner) => used.has(owner)));
        const gatedUse = (slot?.code ?? []).some((code) => identifiers(code).has(target.ident));
        if (named || gatedUse) { mine.set(dependency, [...new Set([...(mine.get(dependency) ?? []), feature])].sort()); enabled.add(`${dependency}\0${feature}`); }
      }
    }
    consumed.set(crate.name, mine);
  }
  const declares = (name: string, feature: string): boolean => analysis.get(name)?.slots.get(feature)?.definition === true || enabled.has(`${name}\0${feature}`);
  const wiring = new Map<string, CrateWiring>();
  for (const crate of crates) {
    const own = analysis.get(crate.name)!;
    const features: TestkitFeature[] = [];
    const optional = new Set<string>();
    for (const [feature, slot] of [...own.slots].sort((a, b) => (a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0))) {
      if (!declares(crate.name, feature)) continue;
      const referenced = new Set<string>();
      for (const code of slot.code) for (const identifier of identifiers(code)) referenced.add(own.aliases.get(identifier) ?? identifier);
      const dependencies: string[] = [];
      const forwardedExternal: string[] = [];
      const forwarded: string[] = [];
      const named = externals.filter((external) => referenced.has(external.ident));
      for (const external of named) if (external.devOnly && external.devRow !== null) { optional.add(external.key); dependencies.push(`dep:${external.key}`); }
      const toolkit = named.filter((external) => external.dependency && external.devFeatures.length > 0);
      const needed = toolkit.some((external) => external.devFeatures.some((added) => { const names = externalGates(external, added); return names === null || [...names].some((name) => referenced.has(name)); }));
      if (needed) for (const external of toolkit) for (const added of [...external.devFeatures].sort()) forwardedExternal.push(`${external.key}/${added}`);
      for (const dependency of crate.dependsOn) {
        if (!declares(dependency, feature)) continue;
        const theirs = analysis.get(dependency)!.slots.get(feature)!;
        const target = byName.get(dependency)!;
        if (referenced.has(target.ident) || [...theirs.names].some((name) => referenced.has(name) && !slot.names.has(name)) || [...theirs.methods].some(([name, owners]) => referenced.has(name) && [...owners].some((owner) => referenced.has(owner)) && !slot.methods.has(name))) forwarded.push(`${dependency}/${feature}`);
      }
      features.push({ name: feature, entries: [...new Set([...dependencies, ...forwardedExternal, ...forwarded])] });
    }
    wiring.set(crate.name, { features, optionalDependencies: [...optional].sort(), consumed: consumed.get(crate.name)!, gatedNames: new Map([...own.slots].map(([feature, slot]) => [feature, new Set([...slot.names, ...slot.reexports, ...slot.methods.keys()])])) });
  }
  return wiring;
}

/** 🎯️ The features a `[[test]]` target needs through `required-features`: the gate features of the crate whose library it links (an integration test sees the library without `cfg(test)`) whose gated names the target's own identifiers use. */
export function requiredTestFeatures(targetTokens: ReadonlySet<string>, gatedNames: ReadonlyMap<string, ReadonlySet<string>>): string[] {
  return [...gatedNames].filter(([, names]) => [...names].some((name) => targetTokens.has(name))).map(([feature]) => feature).sort();
}

/** 🎯️ Adds `required-features` to the named `[[test]]` sections of a manifest; a section that has the key keeps it, extended with the missing features. */
export function withRequiredFeatures(manifestText: string, byTarget: ReadonlyMap<string, readonly string[]>): string {
  if (!byTarget.size) return manifestText;
  const lines = manifestText.split("\n");
  for (let at = 0; at < lines.length; at += 1) {
    if (lines[at]!.trim() !== "[[test]]") continue;
    let end = at + 1;
    while (end < lines.length && !/^\s*\[/u.test(lines[end]!)) end += 1;
    const body = lines.slice(at + 1, end);
    const name = body.map((line) => /^\s*name\s*=\s*"([^"]+)"/u.exec(line)?.[1]).find(Boolean);
    const wanted = name === undefined ? undefined : byTarget.get(name);
    if (!wanted?.length) continue;
    const existing = body.findIndex((line) => /^\s*required-features\s*=/u.test(line));
    const current = existing >= 0 ? [...body[existing]!.matchAll(/"([^"]+)"/gu)].map((match) => match[1]!) : [];
    const row = `required-features = [${[...new Set([...current, ...wanted])].map((feature) => JSON.stringify(feature)).join(", ")}]`;
    if (existing >= 0) lines[at + 1 + existing] = row;
    else {
      let last = body.length - 1;
      while (last >= 0 && body[last]!.trim() === "") last -= 1;
      lines.splice(at + 1 + last + 1, 0, row);
    }
  }
  return lines.join("\n");
}

/** 🚩️ The manifest row of a gate feature. */
export function testkitFeatureRow(feature: TestkitFeature): string {
  return `${feature.name} = [${feature.entries.map((entry) => JSON.stringify(entry)).join(", ")}]`;
}

/** 🚩️ Adds feature rows to a package manifest: a row of an existing `[features]` section is replaced, a missing section is created in front of `[dependencies]`. */
export function withFeatureRows(manifestText: string, features: readonly TestkitFeature[]): string {
  let out = manifestText;
  for (const feature of features) {
    const row = testkitFeatureRow(feature);
    const present = new RegExp(`^${feature.name.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}\\s*=\\s*\\[[^\\]]*\\]`, "mu");
    if (present.test(out)) { out = out.replace(present, () => row); continue; }
    const section = /^\[features\]\s*$/mu.exec(out);
    if (section) { const lineEnd = out.indexOf("\n", section.index) + 1; out = out.slice(0, lineEnd) + row + "\n" + out.slice(lineEnd); continue; }
    const dependencies = /^\[dependencies\]\s*$/mu.exec(out);
    out = dependencies ? `${out.slice(0, dependencies.index)}[features]\n${row}\n\n${out.slice(dependencies.index)}` : `${out.trimEnd()}\n\n[features]\n${row}\n`;
  }
  return out;
}
