/** 📦️ Lift kids: module exporting `matchesLift`, `absolutizeBeyond`, `liftModuleKids`. */
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { rustTokens } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { identifierSet, parseRustSourceTree, type RustModuleNode } from "../🌳️module-tree/🟦️.ts";
import { declaredNames, hostPrivate, prependAfterInner, widenHostItems, type TestTransformResult } from "../🧪️test-closure/🟦️.ts";

/** 🪜️ A file module mounted from a host file, with the chain of module names that encloses its declaration. */
type LibMount = { readonly hostFile: string; readonly node: RustModuleNode; readonly file: string; readonly trail: readonly string[]; readonly hostIsRoot: boolean; readonly inFileScope: boolean };

/** 🪜️ The path a module has once the private `component` mounts are collapsed into their parents. */
function plainTrail(trail: readonly string[]): string[] {
  return trail.filter((segment) => segment !== "component");
}

/** 🎯️ Whether a parent module path is named by a lift pattern: a path suffix, or with a trailing `::*` that module and everything below it. */
export function matchesLift(parent: string, pattern: string): boolean {
  if (!pattern.endsWith("::*")) return parent === pattern || parent.endsWith(`::${pattern}`);
  const base = pattern.slice(0, -3).split("::");
  const segments = parent.split("::");
  for (let at = 0; at + base.length <= segments.length; at += 1) if (base.every((segment, index) => segments[at + index] === segment)) return true;
  return false;
}

/** 🔭️ Finds every file module mounted below the root, skipping `cfg(test)` mounts. */
function discoverLibMounts(rootFile: string, rootSource: string, readText: (path: string) => string | null): LibMount[] {
  const out: LibMount[] = [];
  const visit = (nodes: readonly RustModuleNode[], file: string, base: string, inFileScope: boolean, trail: readonly string[], isRoot: boolean): void => {
    for (const node of nodes) {
      const path = [...trail, node.name];
      if (node.inline) { visit(node.children, file, resolve(base, node.pathAttribute ?? node.name), false, path, isRoot); continue; }
      if (node.pathAttribute === null || node.conditionalTest) continue;
      const child = resolve(inFileScope ? dirname(file) : base, node.pathAttribute);
      const text = readText(child);
      if (text === null) continue;
      out.push({ hostFile: file, node, file: child, trail, hostIsRoot: isRoot, inFileScope });
      visit(parseRustSourceTree(text).modules, child, dirname(child), true, path, false);
    }
  };
  visit(parseRustSourceTree(rootSource).modules, rootFile, dirname(rootFile), true, [], true);
  return out;
}

/** 🧭️ Rewrites the `super::` chains of a file that reach more than `keep` modules above it into absolute `crate::` paths; chains that stay inside the file's own inline modules or inside the lifted subtree are left alone, `pub(super)` that leaves the subtree becomes `pub(crate)`, and `null` means a chain climbs above the crate root. */
export function absolutizeBeyond(text: string, moduleEff: readonly string[], keep: number, plain = true): string | null {
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
    if (above <= keep) continue;
    if (above > moduleEff.length) return null;
    const target = plain ? plainTrail(moduleEff.slice(0, moduleEff.length - above)) : moduleEff.slice(0, moduleEff.length - above);
    const restriction = tokens[k - 1]?.text === "(" && tokens[k - 2]?.text === "pub" && tokens[last + 1]?.text === ")";
    edits.push({ start: tokens[k]!.start, end: tokens[last]!.end, text: restriction ? "crate" : ["crate", ...target].join("::") });
  }
  let out = text;
  for (const edit of edits.sort((left, right) => right.start - left.start)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}

/** 🪜️ Lifts the child modules of the named parents out of their hosts: every `super::` chain of a child subtree that climbs above the child becomes an absolute `crate::` path, private host items and imports the child reaches through `super::*` are carried or widened, so the child no longer needs its host in the same crate and the slicer can place it alone. Returns the transform result extended by the lift. */
export function liftModuleKids(previous: TestTransformResult | null, rootFile: string, originalRootSource: string, patterns: readonly string[]): TestTransformResult {
  const result: TestTransformResult = previous ?? { rootSource: originalRootSource, edits: new Map(), creates: new Map(), deletes: new Set(), notes: [] };
  const overlay = new Map<string, string>();
  const readText = (path: string): string | null => overlay.get(path) ?? (result.deletes.has(path) ? null : result.creates.get(path) ?? result.edits.get(path) ?? (path === rootFile ? result.rootSource : existsSync(path) ? readFileSync(path, "utf8") : null));
  const setText = (path: string, text: string): void => {
    overlay.set(path, text);
    if (path === rootFile) result.rootSource = text;
    else if (result.creates.has(path)) result.creates.set(path, text);
    else result.edits.set(path, text);
  };
  const mounts = discoverLibMounts(rootFile, result.rootSource, readText);
  const shortName = (file: string): string => file.split("/").slice(-4).join("/");
  const lifted = new Set<string>();
  const liftSubtree = (file: string, path: readonly string[], depth: number, seen: Set<string>): boolean => {
    if (seen.has(file)) return true;
    seen.add(file);
    const text = readText(file);
    if (text === null) return true;
    const rewritten = absolutizeBeyond(text, path, depth);
    if (rewritten === null) return false;
    if (rewritten !== text) setText(file, rewritten);
    for (const node of parseRustSourceTree(text).modules) {
      if (node.pathAttribute === null || node.inline) continue;
      if (!liftSubtree(resolve(dirname(file), node.pathAttribute), [...path, node.name], depth + 1, seen)) return false;
    }
    return true;
  };
  const subtreeText = (file: string, seen = new Set<string>()): string => {
    if (seen.has(file)) return "";
    seen.add(file);
    const text = readText(file) ?? "";
    let out = text;
    for (const node of parseRustSourceTree(text).modules) if (!node.inline && node.pathAttribute !== null) out += `\n${subtreeText(resolve(dirname(file), node.pathAttribute), seen)}`;
    return out;
  };
  for (const mount of mounts) {
    const parent = plainTrail(mount.trail).join("::");
    if (!patterns.some((pattern) => matchesLift(parent, pattern))) continue;
    if (lifted.has(mount.file)) continue;
    const body = readText(mount.file);
    if (body === null) continue;
    const whole = subtreeText(mount.file);
    if (!/\bsuper\b/u.test(whole)) continue;
    const here = [...mount.trail, mount.node.name];
    const hostText = readText(mount.hostFile) ?? "";
    const needsHost = !mount.hostIsRoot;
    if (needsHost && !mount.inFileScope) { result.notes.push(`kept ${shortName(mount.file)} in its host (declared inside an inline module of a non-root file)`); continue; }
    const privateUse = needsHost ? hostPrivate(hostText, identifierSet(whole), declaredNames(body), whole) : { items: [] as string[], carry: [] as string[], relative: [] as string[] };
    if (privateUse.relative.length) { result.notes.push(`kept ${shortName(mount.file)} in its host (it uses relative private imports of the host: ${privateUse.relative.join(", ").slice(0, 120)})`); continue; }
    const before = new Map<string, string>();
    const snapshotFiles = (file: string): void => { if (before.has(file)) return; const text = readText(file); if (text === null) return; before.set(file, text); for (const node of parseRustSourceTree(text).modules) if (!node.inline && node.pathAttribute !== null) snapshotFiles(resolve(dirname(file), node.pathAttribute)); };
    snapshotFiles(mount.file);
    if (!liftSubtree(mount.file, here, 0, new Set<string>())) {
      for (const [file, text] of before) setText(file, text);
      result.notes.push(`kept ${shortName(mount.file)} in its host (a super chain climbs above the crate root)`);
      continue;
    }
    const widened = privateUse.items.length ? widenHostItems(hostText, privateUse.items) : [];
    if (widened === null) {
      for (const [file, text] of before) setText(file, text);
      result.notes.push(`kept ${shortName(mount.file)} in its host (it uses private items of its host that cannot be widened: ${privateUse.items.join(", ").slice(0, 120)})`);
      continue;
    }
    if (widened.length) {
      let hostNow = readText(mount.hostFile) ?? "";
      for (const edit of [...widened].sort((a, b) => b.start - a.start)) hostNow = hostNow.slice(0, edit.start) + edit.text + hostNow.slice(edit.end);
      setText(mount.hostFile, hostNow);
    }
    const hostPath = plainTrail(mount.trail);
    const importLines = privateUse.items.length ? [`use crate::${hostPath.join("::")}::{${privateUse.items.join(", ")}};`] : [];
    const added = [...privateUse.carry, ...importLines];
    if (added.length) setText(mount.file, prependAfterInner(readText(mount.file) ?? body, added));
    lifted.add(mount.file);
    result.notes.push(`lifted ${shortName(mount.file)} out of ${hostPath.join("::") || "the root"} (super paths above it made absolute${privateUse.carry.length ? ", private absolute imports carried" : ""}${privateUse.items.length ? `, widened ${privateUse.items.join(", ")} to pub` : ""})`);
  }
  return result;
}
