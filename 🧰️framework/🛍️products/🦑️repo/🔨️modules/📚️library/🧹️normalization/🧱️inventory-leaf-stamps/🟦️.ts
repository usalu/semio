import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull } from "../📁️input/🟦️.ts";
import { readFileSync } from "node:fs";
import type { Stats } from "node:fs";
import type { ReferenceEdit, TaxonomyInventory, TaxonomyInventoryEntry } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath } from "../🏷️file-kind-matching/🟦️.ts";

/** 🕰️ The stamp a leaf carried when the inventory read and hashed it, and whether those bytes were exact UTF-8. */
export interface LeafStamp { readonly dev: number; readonly ino: number; readonly size: number; readonly mtimeMs: number; readonly ctimeMs: number; readonly mode: number; readonly observedAt: number; readonly utf8: boolean }

export const inventoryLeafStamps = new WeakMap<TaxonomyInventory, ReadonlyMap<string, LeafStamp>>();

/** ⏱️ Stamps this close to their observation may still change within one timestamp tick, so they never prove content (racy-git rule). */
export const LEAF_STAMP_RACY_MS = 2000;

/** 🪪️ True when a leaf provably still holds the exact UTF-8 bytes the inventory hashed: it carries the identical stamp, observed long enough
 * after its last change. Anything else, including a path that is no longer admissible, must be re-read. */
export function leafStampCurrent(repoRoot: string, path: string, stamp: LeafStamp | undefined, admit: typeof assertLexicalInputOutsideOpaque): boolean {
  if (!stamp?.utf8 || stamp.mtimeMs > stamp.observedAt - LEAF_STAMP_RACY_MS || stamp.ctimeMs > stamp.observedAt - LEAF_STAMP_RACY_MS) return false;
  let current: Stats | null;
  try { current = lstatOrNull(admit(repoRoot, path, "Reference preimage", true)); } catch { return false; }
  return current !== null && current.isFile() && current.dev === stamp.dev && current.ino === stamp.ino && current.size === stamp.size && current.mtimeMs === stamp.mtimeMs && current.ctimeMs === stamp.ctimeMs && (current.mode & 0o7777) === stamp.mode;
}

/** 📖️ A memoized lazy read of one leaf as UTF-8 text, flagging whether its bytes still have the inventoried digest. */
export function leafContent(repoRoot: string, path: string, contentHash: string): () => Readonly<{ text: string; matches: boolean }> {
  let loaded: Readonly<{ text: string; matches: boolean }> | undefined;
  return () => {
    if (!loaded) { const bytes = readFileSync(absolutePath(repoRoot, path)); loaded = { text: bytes.toString("utf8"), matches: sha256(bytes) === contentHash }; }
    return loaded;
  };
}

/** 🧊️ Recognizes inert fixture payloads from existing language-neutral test collection policy. */
export function taxonomyFixturePayloadPath(path: string, policy: Pick<LoadedTaxonomy["discoverySchema"], "testsDirName" | "testFixturesDirName">): boolean {
  const segments = path.split("/");
  return segments.some((segment, index) => segment === policy.testFixturesDirName && segments.slice(0, index).includes(policy.testsDirName));
}

const inventoryEntryIndexes = new WeakMap<readonly TaxonomyInventoryEntry[], Readonly<{ bySource: ReadonlyMap<string, TaxonomyInventoryEntry>; byNormalized: ReadonlyMap<string, TaxonomyInventoryEntry> }>>();

/** 🗂️ The first inventory entry per source path and per normalized path, built once per entry list, so a lookup answers exactly what `entries.find` answered. */
function inventoryEntryIndex(inventory: TaxonomyInventory): Readonly<{ bySource: ReadonlyMap<string, TaxonomyInventoryEntry>; byNormalized: ReadonlyMap<string, TaxonomyInventoryEntry> }> {
  const cached = inventoryEntryIndexes.get(inventory.entries);
  if (cached) return cached;
  const bySource = new Map<string, TaxonomyInventoryEntry>(), byNormalized = new Map<string, TaxonomyInventoryEntry>();
  for (const entry of inventory.entries) {
    if (!bySource.has(entry.sourcePath)) bySource.set(entry.sourcePath, entry);
    if (!byNormalized.has(entry.normalizedPath)) byNormalized.set(entry.normalizedPath, entry);
  }
  const index = { bySource, byNormalized };
  inventoryEntryIndexes.set(inventory.entries, index);
  return index;
}

export function inventoryEntriesBySourcePath(inventory: TaxonomyInventory): ReadonlyMap<string, TaxonomyInventoryEntry> { return inventoryEntryIndex(inventory).bySource; }

export function inventoryEntriesByNormalizedPath(inventory: TaxonomyInventory): ReadonlyMap<string, TaxonomyInventoryEntry> { return inventoryEntryIndex(inventory).byNormalized; }

const referenceEditPathIndexes = new WeakMap<readonly ReferenceEdit[], ReadonlyMap<string, readonly ReferenceEdit[]>>();

/** 🗃️ A file's planned edits in their list order, grouped once per edit list, so a lookup answers exactly what `edits.filter(edit.path === path)` answered. */
export function referenceEditsByPath(edits: readonly ReferenceEdit[]): ReadonlyMap<string, readonly ReferenceEdit[]> {
  const cached = referenceEditPathIndexes.get(edits);
  if (cached) return cached;
  const index = new Map<string, ReferenceEdit[]>();
  for (const edit of edits) {
    const group = index.get(edit.path);
    if (group) group.push(edit);
    else index.set(edit.path, [edit]);
  }
  referenceEditPathIndexes.set(edits, index);
  return index;
}

export function referenceEditIdentity(edit: ReferenceEdit): string {
  return `${edit.path}\u0000${edit.structuredLocation}\u0000${edit.oldValue}\u0000${edit.newValue}`;
}
