import { existsSync, readdirSync, readFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { rustTokenPairs, rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { identifierSet, parseRustSourceTree, prunedUse, useLeafNames, type RustModuleNode, type RustSourceTree } from "../🌳️module-tree/🟦️.ts";
import { relativePosix } from "../📝️render/🟦️.ts";

/** 🧪️ Marker the fixture rewrite leaves in a path until the owning crate directory is known. */
export const FIXTURE_PLACEHOLDER = "@fixtures/";

/** 🔁️ Which test-closure rewrites a carve may apply. */
export type TestTransformOptions = Readonly<{ fixtures: boolean; aggregate: boolean; hoist: boolean; detach?: boolean; minimumAggregateGroup?: number }>;

/** 🧾️ Files a test-closure rewrite changes, keyed by absolute path. */
export type TestTransformResult = {
  rootSource: string;
  readonly edits: Map<string, string>;
  readonly creates: Map<string, string>;
  readonly deletes: Set<string>;
  readonly notes: string[];
};

type TestMount = { readonly hostFile: string; readonly node: RustModuleNode; readonly testFile: string; readonly ownerPath: readonly string[]; readonly hostIsRoot: boolean };

const FIXTURE_HELPER = [
  "fn runtime_fixture_text(relative: &str) -> &'static str {",
  "    let path = std::path::Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(relative);",
  "    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!(\"fixture {}: {error}\", path.display())).into_boxed_str())",
  "}",
].join("\n");

/** 🧪️ Rewrites `include_str!` of test fixtures into runtime reads, so fixtures leave the compiled source closure. */
export function fixturesToRuntime(text: string, fileAbs: string, repoRoot: string): string | null {
  const tokens = rustTokens(text);
  const pairs = rustTokenPairs(tokens);
  const edits: { start: number; end: number; text: string }[] = [];
  const names = new Set<string>();
  const definitionRanges: [number, number][] = [];
  const literal = (index: number): string | null => (tokens[index]?.kind === "string" ? tokens[index]!.text.slice(1, -1) : null);
  const target = (relative: string): string => `${FIXTURE_PLACEHOLDER}${relativePosix(repoRoot, resolve(dirname(fileAbs), relative))}`;
  const isFixture = (relative: string): boolean => relativePosix(repoRoot, resolve(dirname(fileAbs), relative)).includes("/🧪️tests/");
  const literalMacros: [number, number][] = [];
  for (const [open, close] of pairs) if (tokens[open]!.text === "(" && tokens[open - 1]?.text === "!" && ["concat", "stringify"].includes(tokens[open - 2]?.text ?? "")) literalMacros.push([open, close]);
  const inLiteralMacro = (index: number): boolean => literalMacros.some(([open, close]) => open < index && index < close);
  const depth = new Array<number>(tokens.length).fill(0);
  let current = 0;
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text === "}") current -= 1;
    depth[i] = current;
    if (tokens[i]!.text === "{") current += 1;
  }
  for (let i = 0; i + 6 < tokens.length; i += 1) {
    if (tokens[i]!.text !== "include_str" || tokens[i + 1]!.text !== "!" || tokens[i + 2]!.text !== "(") continue;
    const relative = literal(i + 3);
    if (relative === null || tokens[i + 4]!.text !== ")" || !isFixture(relative) || inLiteralMacro(i)) continue;
    let head = i - 1;
    const level = depth[i];
    const isConst = tokens[head]?.text === "=" && tokens[head - 1]?.text === "str" && tokens[head - 2]?.text === "&" && tokens[head - 3]?.text === ":" && tokens[head - 4]?.kind === "identifier" && (tokens[head - 5]?.text === "const" || tokens[head - 5]?.text === "static") && depth[i] === 0;
    if (isConst) {
      const name = tokens[head - 4]!.text;
      let start = head - 5;
      if (tokens[start - 1]?.text === ")" && pairs.get(start - 1) !== undefined) start = pairs.get(start - 1)! - 1;
      else if (tokens[start - 1]?.text === "pub") start -= 1;
      const end = tokens[i + 5]?.text === ";" ? i + 5 : i + 4;
      edits.push({ start: tokens[start]!.start, end: tokens[end]!.end, text: `static ${name}: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("${target(relative)}"));` });
      names.add(name);
      definitionRanges.push([tokens[start]!.start, tokens[end]!.end]);
    } else if (level !== undefined && level >= 1) {
      edits.push({ start: tokens[i]!.start, end: tokens[i + 4]!.end, text: `runtime_fixture_text("${target(relative)}")` });
    }
  }
  if (!edits.length) return null;
  for (let i = 0; i < tokens.length; i += 1) {
    const token = tokens[i]!;
    if (token.kind !== "identifier" || !names.has(token.text) || definitionRanges.some(([a, b]) => token.start >= a && token.end <= b)) continue;
    if (tokens[i - 1]?.text === "." || tokens[i - 1]?.text === "::") continue;
    if (inLiteralMacro(i)) return null;
    edits.push({ start: token.start, end: token.end, text: `(*${token.text})` });
  }
  let out = text;
  for (const edit of edits.sort((a, b) => b.start - a.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return `${out.trimEnd()}\n\n${FIXTURE_HELPER}\n`;
}

/** 🔁️ Replaces fixture placeholders by paths relative to the owning crate directory. */
export function resolveFixturePlaceholders(text: string, repoRoot: string, manifestDir: string): string {
  return text.replace(/@fixtures\/([^"]+)"/gu, (_, repoRelative: string) => `${relativePosix(manifestDir, resolve(repoRoot, repoRelative))}"`);
}
/** 🧭️ Rewrites the file-level `super::` paths of a test module into the absolute path of the module they name, so the test no longer needs its parent module to be in the same crate; a `pub(super)` restriction widens to `pub(crate)` because a `pub(in path)` restriction cannot be resolved through the glob re-export a mirror root makes. */
export function absolutizeSuperPaths(text: string, moduleEff: readonly string[]): string | null {
  const tree = parseRustSourceTree(text);
  const spans: { start: number; end: number; names: readonly string[] }[] = [];
  const collect = (nodes: readonly RustModuleNode[], trail: readonly string[]): void => {
    for (const node of nodes) if (node.inline) { const names = [...trail, node.name]; spans.push({ start: node.bodyStart, end: node.bodyEnd, names }); collect(node.children, names); }
  };
  collect(tree.modules, []);
  const tokens = rustTokens(text);
  const edits: { start: number; end: number; text: string }[] = [];
  for (let k = 0; k < tokens.length; k += 1) {
    if (tokens[k]!.text !== "super" || tokens[k - 1]?.text === "::") continue;
    const inside = spans.filter((span) => tokens[k]!.start >= span.start && tokens[k]!.start < span.end).sort((a, b) => b.names.length - a.names.length)[0];
    const nested = inside?.names ?? [];
    let count = 1;
    let last = k;
    while (tokens[last + 1]?.text === "::" && tokens[last + 2]?.text === "super") { count += 1; last += 2; }
    if (count <= nested.length) continue;
    const above = count - nested.length;
    if (above > moduleEff.length) return null;
    const target = moduleEff.slice(0, moduleEff.length - above);
    const restriction = tokens[k - 1]?.text === "(" && tokens[k - 2]?.text === "pub" && tokens[last + 1]?.text === ")";
    edits.push({ start: tokens[k]!.start, end: tokens[last]!.end, text: restriction ? "crate" : ["crate", ...target].join("::") });
  }
  let out = text;
  for (const edit of edits.sort((left, right) => right.start - left.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}


function usedAsItem(tokens: readonly { kind: string; text: string }[], name: string): boolean {
  const upper = /^[A-Z]/u.test(name);
  for (let i = 0; i < tokens.length; i += 1) {
    if (tokens[i]!.text !== name) continue;
    const previous = tokens[i - 1]?.text;
    const next = tokens[i + 1]?.text;
    if (previous === "." || previous === "let" || previous === "mut" || previous === "fn" || previous === "struct" || previous === "enum" || previous === "const" || previous === "static" || previous === "type" || previous === "trait" || previous === "mod") continue;
    if (upper || next === "(" || next === "::" || next === "!" || previous === "::" || previous === "&" || previous === "(" || previous === "," || previous === "=" || previous === "->" || previous === ":") return true;
  }
  return false;
}

export function hostPrivate(hostText: string, bodyIdentifiers: ReadonlySet<string>, declaredInBody: ReadonlySet<string>, bodyText: string | null = null): { items: string[]; carry: string[]; relative: string[] } {
  const tree = parseRustSourceTree(hostText);
  const bodyTokens = bodyText === null ? null : rustTokens(bodyText);
  const items: string[] = [];
  const carry: string[] = [];
  const relative: string[] = [];
  for (const item of tree.items) {
    const restricted = item.visibility === "private" || item.visibility === "pub(super)" || item.visibility === "pub(self)";
    if (!restricted) continue;
    if (item.kind === "use") {
      const statement = hostText.slice(item.declStart, item.end);
      const leaves = useLeafNames(statement).filter((leaf) => bodyIdentifiers.has(leaf) && !declaredInBody.has(leaf));
      if (!leaves.length) continue;
      const pruned = prunedUse(statement, leaves);
      if (!pruned) continue;
      if (/^use\s+(?:self|super)::/u.test(pruned) || /\*\s*;\s*$/u.test(pruned)) relative.push(pruned);
      else carry.push(pruned);
    } else if (item.kind !== "extern-crate" && item.kind !== "macro" && item.kind !== "impl" && item.name && bodyIdentifiers.has(item.name) && !declaredInBody.has(item.name) && (!bodyTokens || usedAsItem(bodyTokens, item.name))) items.push(item.name);
  }
  for (const node of tree.modules) {
    const restricted = node.visibility === "private" || node.visibility === "pub(super)" || node.visibility === "pub(self)";
    if (restricted && !node.conditionalTest && node.name !== "component" && bodyIdentifiers.has(node.name) && !declaredInBody.has(node.name) && (!bodyTokens || usedAsItem(bodyTokens, node.name))) items.push(node.name);
  }
  return { items, carry, relative };
}

/** ⬆️ Puts lines in front of a source body but behind its inner doc comments and inner attributes, which must stay first. */
export function prependAfterInner(body: string, lines: readonly string[]): string {
  if (!lines.length) return body;
  const rows = body.split("\n");
  let cut = 0;
  while (cut < rows.length && (/^\s*\/\/!/u.test(rows[cut]!) || /^\s*#!\[/u.test(rows[cut]!) || (rows[cut]!.trim() === "" && cut > 0 && /^\s*(?:\/\/!|#!\[)/u.test(rows[cut - 1]!)))) cut += 1;
  return [...rows.slice(0, cut), ...lines, ...rows.slice(cut)].join("\n");
}

/** 🔓️ Edits that make the named restricted items and modules of a host file `pub`, or null when one of them cannot be found. */
export function widenHostItems(hostText: string, names: readonly string[]): { start: number; end: number; text: string }[] | null {
  const tree = parseRustSourceTree(hostText);
  const edits: { start: number; end: number; text: string }[] = [];
  for (const name of names) {
    const target = tree.items.find((item) => item.name === name && item.kind !== "use" && item.kind !== "impl" && item.kind !== "macro") ?? tree.modules.find((node) => node.name === name);
    if (!target) return null;
    const visibility = /^pub(?:\([^)]*\))?\s*/u.exec(hostText.slice(target.declStart, target.declStart + 40));
    edits.push({ start: target.declStart, end: target.declStart + (visibility ? visibility[0].length : 0), text: "pub " });
  }
  return edits;
}

export function declaredNames(body: string): Set<string> {
  const names = new Set<string>();
  for (const item of parseRustSourceTree(body).items) {
    if (item.name) names.add(item.name);
    if (item.kind === "use") for (const leaf of useLeafNames(body.slice(item.declStart, item.end))) names.add(leaf);
  }
  return names;
}

function snake(name: string): string {
  return name.replace(/^[^\p{L}\p{N}]+/u, "").replace(/[^A-Za-z0-9]+/gu, "_").replace(/^_+|_+$/gu, "").toLowerCase();
}

function discoverMounts(rootFile: string, rootSource: string, readText: (path: string) => string | null): { mounts: TestMount[]; inline: Map<string, { node: RustModuleNode; base: string }> } {
  const mounts: TestMount[] = [];
  const inline = new Map<string, { node: RustModuleNode; base: string }>();
  const visit = (nodes: readonly RustModuleNode[], file: string, base: string, inFileScope: boolean, trail: readonly string[], isRoot: boolean): void => {
    for (const node of nodes) {
      const path = [...trail, node.name];
      if (node.inline) {
        inline.set(path.join("::"), { node, base: resolve(base, node.pathAttribute ?? node.name) });
        visit(node.children, file, resolve(base, node.pathAttribute ?? node.name), false, path, isRoot);
        continue;
      }
      if (node.pathAttribute === null) continue;
      const child = resolve(inFileScope ? dirname(file) : base, node.pathAttribute);
      if (node.conditionalTest) { mounts.push({ hostFile: file, node, testFile: child, ownerPath: trail, hostIsRoot: isRoot }); continue; }
      const text = readText(child);
      if (text === null) continue;
      visit(parseRustSourceTree(text).modules, child, dirname(child), true, path, false);
    }
  };
  visit(parseRustSourceTree(rootSource).modules, rootFile, dirname(rootFile), true, [], true);
  return { mounts, inline };
}

/** 🧪️ Rewrites the test closure of an artifact root: hoists root-bound tests, aggregates sibling leaf tests, moves fixtures to runtime reads. */
export function transformTestClosure(repoRoot: string, rootFile: string, rootSource: string, rootNames: ReadonlySet<string>, options: TestTransformOptions): TestTransformResult {
  const result: TestTransformResult = { rootSource, edits: new Map(), creates: new Map(), deletes: new Set(), notes: [] };
  const overlay = new Map<string, string>();
  const readText = (path: string): string | null => overlay.get(path) ?? (result.deletes.has(path) ? null : result.creates.get(path) ?? (existsSync(path) ? readFileSync(path, "utf8") : null));
  const setText = (path: string, text: string): void => {
    overlay.set(path, text);
    if (path === rootFile) result.rootSource = text;
    else if (result.creates.has(path)) result.creates.set(path, text);
    else result.edits.set(path, text);
  };
  const rootEdits: { start: number; end: number; text: string }[] = [];
  const hostEdits = new Map<string, { start: number; end: number; text: string }[]>();
  const queueEdit = (host: string, edit: { start: number; end: number; text: string }): void => {
    if (host === rootFile) rootEdits.push(edit);
    else hostEdits.set(host, [...(hostEdits.get(host) ?? []), edit]);
  };
  const flushEdits = (): void => {
    const apply = (host: string, edits: { start: number; end: number; text: string }[]): void => {
      let text = readText(host) ?? "";
      for (const edit of edits.sort((a, b) => b.start - a.start)) text = text.slice(0, edit.start) + edit.text + text.slice(edit.end);
      setText(host, text);
    };
    if (rootEdits.length) apply(rootFile, rootEdits.splice(0));
    for (const [host, edits] of hostEdits) apply(host, edits);
    hostEdits.clear();
  };
  const removalOf = (host: string, node: RustModuleNode): { start: number; end: number; text: string } => {
    const text = readText(host) ?? "";
    let end = node.end;
    while (text[end] === " " || text[end] === "\t") end += 1;
    if (text[end] === "\n") end += 1;
    let start = node.start;
    const lineStart = text.lastIndexOf("\n", start - 1) + 1;
    if (/^[ \t]*$/u.test(text.slice(lineStart, start))) start = lineStart;
    return { start, end, text: "" };
  };
  const orphanedDocRemoval = (host: string, node: RustModuleNode): { start: number; end: number; text: string }[] => {
    const text = readText(host) ?? "";
    const lines: { start: number; end: number; text: string }[] = [];
    let cursor = text.lastIndexOf("\n", node.start - 1);
    const doc: { start: number; end: number }[] = [];
    while (cursor >= 0) {
      const lineStart = text.lastIndexOf("\n", cursor - 1) + 1;
      const line = text.slice(lineStart, cursor).trim();
      if (line.startsWith("///")) doc.push({ start: lineStart, end: cursor + 1 });
      else if (doc.length || !(line === "" || line.startsWith("//"))) break;
      else if (line.startsWith("//!")) break;
      cursor = lineStart - 1;
    }
    for (const range of doc) lines.push({ start: range.start, end: range.end, text: "" });
    return lines;
  };
  let sourceCache: [string, string][] | null = null;
  const artifactSources = (): [string, string][] => {
    if (sourceCache) return sourceCache;
    const out: [string, string][] = [];
    const walk = (directory: string): void => {
      for (const entry of readdirSync(directory, { withFileTypes: true })) {
        const path = join(directory, entry.name);
        if (entry.isDirectory()) { if (!/^(node_modules|target|dist)$/u.test(entry.name)) walk(path); }
        else if (entry.name.endsWith(".rs")) out.push([path, readText(path) ?? ""]);
      }
    };
    walk(dirname(rootFile));
    return (sourceCache = out);
  };
  const subtreeText = (file: string, seen = new Set<string>()): string => {
    if (seen.has(file)) return "";
    seen.add(file);
    const text = readText(file) ?? "";
    let out = text;
    for (const node of parseRustSourceTree(text).modules) if (!node.inline && node.pathAttribute !== null) out += `\n${subtreeText(resolve(dirname(file), node.pathAttribute), seen)}`;
    return out;
  };
  const hostGlobCarry = (mount: TestMount, body: string): string[] | null => {
    const hostPath = mount.ownerPath.filter((segment) => segment !== "component").join("::");
    if (!hostPath || !new RegExp(`\\buse\\s+crate::${hostPath}::\\*\\s*;`, "u").test(body)) return [];
    const hostText = readText(mount.hostFile) ?? "";
    const carry: string[] = [];
    for (const item of parseRustSourceTree(hostText).items) {
      if (item.kind !== "use" || item.visibility !== "private") continue;
      const statement = hostText.slice(item.declStart, item.end).trim();
      if (/^use\s+(?:self|super)::/u.test(statement) || statement.includes("{self") || /\bsuper::/u.test(statement)) return null;
      carry.push(statement);
    }
    return carry;
  };
  const widenFor = (mount: TestMount, items: readonly string[], body: string): string[] | null => {
    if (!items.length) return [];
    const hostText = readText(mount.hostFile) ?? "";
    const edits = widenHostItems(hostText, items);
    if (!edits) return null;
    for (const edit of edits) queueEdit(mount.hostFile, edit);
    const bodyNames = identifierSet(body);
    for (const node of parseRustSourceTree(hostText).modules) {
      if (!items.includes(node.name) || node.inline || node.pathAttribute === null) continue;
      const childFile = resolve(dirname(mount.hostFile), node.pathAttribute);
      const childText = readText(childFile);
      if (childText === null) continue;
      const restrictedNames = parseRustSourceTree(childText).items.filter((item) => item.name && item.kind !== "use" && item.kind !== "impl" && item.kind !== "macro" && item.visibility !== "pub" && bodyNames.has(item.name)).map((item) => item.name!);
      for (const edit of widenHostItems(childText, restrictedNames) ?? []) queueEdit(childFile, edit);
    }
    return [`use crate::${mount.ownerPath.filter((segment) => segment !== "component").join("::")}::{${items.join(", ")}};`];
  };
  if (options.hoist || options.detach) {
    const { mounts } = discoverMounts(rootFile, result.rootSource, readText);
    const seen = new Set<string>();
    const visit = (file: string): void => {
      if (seen.has(file)) return;
      seen.add(file);
      const text = readText(file);
      if (text === null) return;
      const plain = text.replace(/\bcrate((?:::[A-Za-z_][A-Za-z0-9_]*)*?)::component(?=::)/gu, "crate$1");
      if (plain !== text) { setText(file, plain); result.notes.push(`dropped the private component segment from the absolute paths of ${relativePosix(repoRoot, file)}`); }
      for (const node of parseRustSourceTree(plain).modules) if (!node.inline && node.pathAttribute !== null) visit(resolve(dirname(file), node.pathAttribute));
    };
    for (const mount of mounts) visit(mount.testFile);
  }
  if (options.hoist) {
    const { mounts } = discoverMounts(rootFile, result.rootSource, readText);
    const hoisted: string[] = [];
    for (const mount of mounts) {
      const body = readText(mount.testFile);
      if (body === null) continue;
      const bound = [...subtreeText(mount.testFile).matchAll(/\bcrate::([A-Za-z_][A-Za-z0-9_]*)/gu)].map((match) => match[1]!).filter((name) => rootNames.has(name));
      if (!bound.length || mount.hostIsRoot || /\bsuper::/u.test(body)) continue;
      const whole = subtreeText(mount.testFile);
      const privateUse = hostPrivate(readText(mount.hostFile) ?? "", identifierSet(whole), declaredNames(body), whole);
      if (privateUse.relative.length) { result.notes.push(`kept ${relativePosix(repoRoot, mount.testFile)} in place (it uses private items of its host module: ${privateUse.relative.join(", ").slice(0, 120)})`); continue; }
      const name = mount.node.name;
      const hostLast = mount.ownerPath.filter((segment) => segment !== "component").at(-1);
      const named = hostLast === undefined ? null : new RegExp(`\\b${hostLast}::${name}\\b`, "u");
      const referenced = mounts.some((other) => other !== mount && new RegExp(`\\bsuper::(?:super::)*${name}\\b`, "u").test(readText(other.testFile) ?? "")) || (named !== null && artifactSources().some(([file, text]) => file !== mount.testFile && named.test(text)));
      if (referenced) { result.notes.push(`kept ${relativePosix(repoRoot, mount.testFile)} in place (another test module names it through super::${name})`); continue; }
      const rawGlobCarry = hostGlobCarry(mount, body);
      const known = new Set([...privateUse.carry.flatMap((statement) => useLeafNames(statement)), ...declaredNames(body)]);
      const globCarry = rawGlobCarry === null ? null : rawGlobCarry.flatMap((statement) => {
        const leaves = useLeafNames(statement);
        if (!leaves.length) return [`#[allow(unused_imports)]\n${statement}`];
        const fresh = leaves.filter((leaf) => !known.has(leaf));
        return fresh.length ? [`#[allow(unused_imports)]\n${prunedUse(statement, fresh)}`] : [];
      });
      if (globCarry === null) { result.notes.push(`kept ${relativePosix(repoRoot, mount.testFile)} in place (it glob-imports its host, whose private imports are relative)`); continue; }
      const widenedUse = widenFor(mount, privateUse.items, body);
      if (widenedUse === null) { result.notes.push(`kept ${relativePosix(repoRoot, mount.testFile)} in place (it uses private items of its host module that cannot be widened: ${privateUse.items.join(", ").slice(0, 120)})`); continue; }
      queueEdit(mount.hostFile, removalOf(mount.hostFile, mount.node));
      for (const edit of orphanedDocRemoval(mount.hostFile, mount.node)) queueEdit(mount.hostFile, edit);
      if (privateUse.carry.length || widenedUse.length || globCarry.length) setText(mount.testFile, prependAfterInner(body, [...privateUse.carry, ...globCarry, ...widenedUse]));
      hoisted.push(`#[cfg(test)]\n#[path = "${relativePosix(dirname(rootFile), mount.testFile)}"]\nmod hoisted_${snake(basename(dirname(dirname(mount.testFile))))}_${hoisted.length};`);
      result.notes.push(`hoisted ${relativePosix(repoRoot, mount.testFile)} to the artifact root (references ${[...new Set(bound)].join(", ")}${privateUse.items.length ? `; widened ${privateUse.items.join(", ")} of the host to pub` : ""})`);
    }
    flushEdits();
    if (hoisted.length) setText(rootFile, `${result.rootSource.trimEnd()}\n\n${hoisted.join("\n")}\n`);
  }
  if (options.fixtures) {
    const { mounts } = discoverMounts(rootFile, result.rootSource, readText);
    for (const mount of mounts) {
      const text = readText(mount.testFile);
      if (text === null) continue;
      const rewritten = fixturesToRuntime(text, mount.testFile, repoRoot);
      if (rewritten !== null) { setText(mount.testFile, rewritten); result.notes.push(`runtime fixtures in ${relativePosix(repoRoot, mount.testFile)}`); }
    }
    const rootRewrite = fixturesToRuntime(result.rootSource, rootFile, repoRoot);
    if (rootRewrite !== null) setText(rootFile, rootRewrite);
  }
  if (options.aggregate) {
    const { mounts, inline } = discoverMounts(rootFile, result.rootSource, readText);
    const groups = new Map<string, TestMount[]>();
    for (const mount of mounts) {
      if (mount.ownerPath.length < 2) continue;
      const key = mount.ownerPath.slice(0, -1).join("::");
      groups.set(key, [...(groups.get(key) ?? []), mount]);
    }
    for (const [key, members] of groups) {
      const parent = inline.get(key);
      const eligible = members.filter((mount) => {
        const body = readText(mount.testFile);
        if (body === null) return false;
        const uses = body.match(/\buse super::\*;/gu)?.length ?? 0;
        const others = (body.match(/\bsuper::/gu)?.length ?? 0) - uses;
        if (!(uses <= 1 && others === 0 && !/^\s*(?:pub\s+)?mod\s/mu.test(body) && !/^#!\[/mu.test(body))) return false;
        if (mount.hostIsRoot) return true;
        const privateUse = hostPrivate(readText(mount.hostFile) ?? "", identifierSet(body), declaredNames(body), body);
        return privateUse.items.length === 0 && privateUse.relative.length === 0;
      });
      if (!parent || parent.node.children.some((child) => child.name === "component") || eligible.length < (options.minimumAggregateGroup ?? 3)) continue;
      const lca = (() => {
        const parts = eligible.map((mount) => dirname(mount.testFile).split("/"));
        let length = parts[0]!.length;
        for (const other of parts) { let i = 0; while (i < length && other[i] === parts[0]![i]) i += 1; length = i; }
        return parts[0]!.slice(0, length).join("/");
      })();
      let aggregate = resolve(lca, "🧪️tests/🔬️unit/🦀️.rs");
      if (readText(aggregate) !== null && !eligible.some((mount) => mount.testFile === aggregate)) aggregate = resolve(lca, "🧪️tests/🧩️leaves/🦀️.rs");
      const moduleNames = new Map<string, number>();
      const modules = eligible.map((mount) => {
        const owner = mount.ownerPath[mount.ownerPath.length - 1]!;
        const body = readText(mount.testFile)!.replace(/\buse super::\*;/u, `use super::super::${owner}::*;`).trim();
        const used = identifierSet(body);
        const hostText = readText(mount.hostFile) ?? "";
        const carried = mount.hostIsRoot ? [] : parseRustSourceTree(hostText).items.filter((item) => item.kind === "use" && !item.visibility.startsWith("pub")).map((item) => ({ statement: hostText.slice(item.declStart, item.end), leaves: useLeafNames(hostText.slice(item.declStart, item.end)).filter((leaf) => used.has(leaf) && !new RegExp(`\\buse\\s+[^;]*\\b${leaf}\\b`, "u").test(body)) })).filter((entry) => entry.leaves.length && !/\*\s*;\s*$/u.test(entry.statement)).map((entry) => prunedUse(entry.statement, entry.leaves));
        const seen = (moduleNames.get(owner) ?? 0) + 1;
        moduleNames.set(owner, seen);
        return `mod ${seen === 1 ? owner : `${owner}_${seen}`} {\n${carried.length ? `${carried.join("\n")}\n` : ""}${body}\n}`;
      });
      for (const mount of eligible) {
        queueEdit(mount.hostFile, removalOf(mount.hostFile, mount.node));
        for (const edit of orphanedDocRemoval(mount.hostFile, mount.node)) queueEdit(mount.hostFile, edit);
        result.deletes.add(mount.testFile);
        result.edits.delete(mount.testFile);
      }
      const aggregateText = `//! 🧪️ Aggregated unit tests of the \`${key.split("::").pop()}\` leaves, one module per leaf.\n\n${modules.join("\n\n")}\n`;
      result.deletes.delete(aggregate);
      if (existsSync(aggregate)) result.edits.set(aggregate, aggregateText);
      else result.creates.set(aggregate, aggregateText);
      overlay.set(aggregate, aggregateText);
      queueEdit(rootFile, { start: parent.node.bodyEnd, end: parent.node.bodyEnd, text: `\n#[cfg(test)]\n#[path = "${relativePosix(parent.base, aggregate)}"]\nmod leaf_tests;\n` });
      result.notes.push(`aggregated ${eligible.length} leaf test files under ${key} into ${relativePosix(repoRoot, aggregate)}`);
    }
    flushEdits();
  }
  if (options.detach) {
    const { mounts } = discoverMounts(rootFile, result.rootSource, readText);
    for (const mount of mounts) {
      if (mount.hostIsRoot) continue;
      const body = readText(mount.testFile);
      if (body === null) continue;
      const whole = subtreeText(mount.testFile);
      const privateUse = hostPrivate(readText(mount.hostFile) ?? "", identifierSet(whole), declaredNames(body), whole);
      const hasSuper = /\bsuper::/u.test(body);
      if (privateUse.relative.length || (!hasSuper && !privateUse.items.length)) continue;
      const absolute = hasSuper ? absolutizeSuperPaths(body, [...mount.ownerPath.filter((segment) => segment !== "component"), mount.node.name]) : body;
      if (absolute === null || (absolute === body && !privateUse.items.length)) continue;
      const widenedUse = widenFor(mount, privateUse.items, body);
      if (widenedUse === null) continue;
      setText(mount.testFile, prependAfterInner(absolute, [...privateUse.carry, ...widenedUse]));
      result.notes.push(`made ${relativePosix(repoRoot, mount.testFile)} independent of its host module (super paths absolute${privateUse.carry.length ? ", private absolute imports carried" : ""}${privateUse.items.length ? `, widened ${privateUse.items.join(", ")} to pub` : ""})`);
    }
    flushEdits();
  }
  return result;
}

export type { RustSourceTree };
