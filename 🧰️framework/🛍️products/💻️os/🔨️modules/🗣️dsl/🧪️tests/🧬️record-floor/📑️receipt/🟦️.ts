/** 📑️ Reconstructs original source bytes through explicit owned JSON import and argument cuts. */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

export type JsonSourceEdit = Readonly<{start: number; end: number; from: string; to: string}>;
export type JsonSourceCut = Readonly<{path: string; beforeSource?: string; beforeSha256: string; afterSha256: string; edits: readonly JsonSourceEdit[]}>;
const root = resolve(import.meta.dir, "../../../../../../../.."), fixture = resolve(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️record-floor-contract/🧫️fixtures");
export const jsonSourceHash = (source: string): string => createHash("sha256").update(source).digest("hex");
const load = (name: string): {rows: readonly JsonSourceCut[]} => JSON.parse(readFileSync(resolve(fixture, name), "utf8"));
export const jsonPathCuts = load("🧬️json-client-path-cut.json").rows;
export const jsonPolicyCuts = load("🧬️json-client-member-policy-cut.json").rows;
export const jsonMountCuts = load("🧬️json-mount-retirement-cut.json").rows;
export const jsonConcurrentCuts = load("🧬️json-concurrent-source-admission.json").rows;
export const jsonNativeRepairCuts = load("🧬️json-native-compiler-repair-cut.json").rows;
export type JsonGenericSourceCut = Readonly<{path: string; beforeSource: string; beforeSha256: string; namespaceAfterSha256: string; afterSha256: string; namespaceEdits: readonly JsonSourceEdit[]; policyEdits: readonly JsonSourceEdit[]}>;
export const jsonGenericCuts = (JSON.parse(readFileSync(resolve(fixture, "🧬️json-generic-reader-and-protocol-cut.json"), "utf8")) as {rows: readonly JsonGenericSourceCut[]}).rows;
const valueCut = JSON.parse(readFileSync(resolve(fixture, "🧬️json-value-law-import-cut.json"), "utf8")) as {path: string; source: string; beforeSha256: string; afterSha256: string};
const indexed = [jsonConcurrentCuts, jsonPolicyCuts, jsonMountCuts, jsonPathCuts].map(rows => new Map(rows.map(row => [row.path, row])));
const genericIndex = new Map(jsonGenericCuts.map(row => [row.path, row]));

/** 🧬️ Applies the authored source-coordinate edits without interpreting fixture or assertion data. */
export function applyJsonSourceEdits(source: string, edits: readonly JsonSourceEdit[]): string {
  for (const edit of edits.toReversed()) {
    if (source.slice(edit.start, edit.end) !== edit.from) throw Error("JSON source edit lost its authored input");
    source = source.slice(0, edit.start) + edit.to + source.slice(edit.end);
  }
  return source;
}

/** 🪞️ Inverts only declared source edits and verifies both whole-source byte authorities. */
export function undoJsonSourceCut(source: string, cut: JsonSourceCut): string {
  if (jsonSourceHash(source) !== cut.afterSha256) throw Error("JSON source cut current hash changed: " + cut.path);
  let delta = 0;
  const edits = cut.edits.map(edit => { const start = edit.start + delta; delta += edit.to.length - (edit.end - edit.start); return {...edit, start, end: start + edit.to.length}; });
  for (const edit of edits.toReversed()) {
    if (source.slice(edit.start, edit.end) !== edit.to) throw Error("JSON source cut inverse lost its authored output: " + cut.path);
    source = source.slice(0, edit.start) + edit.from + source.slice(edit.end);
  }
  if (jsonSourceHash(source) !== cut.beforeSha256 || cut.beforeSource !== undefined && source !== cut.beforeSource) throw Error("JSON source cut original bytes changed: " + cut.path);
  return source;
}

/** 🧾️ Restores the complete original law source before existing higher composition checks run. */
export function undoJsonOwnershipCuts(path: string, source: string): string {
  const repair = jsonNativeRepairCuts.find(row => row.path === path);
  if (repair) source = undoJsonSourceCut(source, repair);
  const generic = genericIndex.get(path);
  if (generic) {
    source = undoJsonSourceCut(source, {path, beforeSha256: generic.namespaceAfterSha256, afterSha256: generic.afterSha256, edits: generic.policyEdits});
    source = undoJsonSourceCut(source, {path, beforeSource: generic.beforeSource, beforeSha256: generic.beforeSha256, afterSha256: generic.namespaceAfterSha256, edits: generic.namespaceEdits});
  }
  const concurrent = indexed[0]!.get(path);
  if (concurrent) source = undoJsonSourceCut(source, concurrent);
  if (path === valueCut.path) {
    if (jsonSourceHash(source) !== valueCut.afterSha256) throw Error("JSON law Value import output changed");
    source = source.replaceAll("semio_framework_value::", "protocol::value::");
    if (source !== valueCut.source || jsonSourceHash(source) !== valueCut.beforeSha256) throw Error("JSON law Value import original changed");
  }
  for (const cuts of indexed.slice(1)) { const cut = cuts.get(path); if (cut) source = undoJsonSourceCut(source, cut); }
  return source;
}
