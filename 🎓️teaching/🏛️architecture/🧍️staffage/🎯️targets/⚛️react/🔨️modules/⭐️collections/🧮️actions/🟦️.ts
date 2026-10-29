/** 🧮️ Pure rules of the collections: placing a collection on the playground, comparing its first assets, saving and editing a saved collection.
 *
 * @see ../../💾️persistence/🟦️.ts — the events these produce
 * @see ../../🗺️routing/🟦️.ts — the comparison address
 */
import type { Asset, Slug } from "@teaching/staffage";
import { compareToParams, type CompareState } from "../../🗺️routing/🟦️.ts";
import type { ArchiveEvent, Playground, SavedCollection } from "../../💾️persistence/🟦️.ts";

/** 📏️ The gap between neighbours on the playground, metres. */
export const PLACEMENT_GAP_M = 0.6;

/** 🆔️ A fresh identifier with a prefix. */
export function freshId(prefix: string): string {
  const random = typeof crypto !== "undefined" && "randomUUID" in crypto ? crypto.randomUUID().slice(0, 8) : Math.random().toString(36).slice(2, 10);
  return `${prefix}-${Date.now().toString(36)}-${random}`;
}

/** 📍️ The events that place assets side by side after the existing placements, each separated by its width plus the gap; the ground grows when they do not fit. */
export function placeAllEvents(assets: readonly Asset[], playground: Playground, newId: () => string = () => freshId("placed")): readonly ArchiveEvent[] {
  const start = playground.placements.reduce((end, placement) => Math.max(end, placement.xM + 0.5), 0) + PLACEMENT_GAP_M;
  let cursor = start;
  const placed = assets.map((asset) => {
    const xM = cursor + asset.dimensions.widthM / 2;
    cursor += asset.dimensions.widthM + PLACEMENT_GAP_M;
    return { id: newId(), asset: asset.id, xM, pick: null, offsetS: 0, mirrored: false } as const;
  });
  const events: ArchiveEvent[] = [];
  if (cursor > playground.lengthM) events.push({ type: "playground-configured", patch: { lengthM: Math.min(200, Math.ceil(cursor)) } });
  return [...events, ...placed.map((placement): ArchiveEvent => ({ type: "placement-added", placement }))];
}

/** ⚖️ The comparison address query of the first four assets in one view; null when fewer than two are given. */
export function compareFirstFour(assets: readonly Slug[]): URLSearchParams | null {
  if (assets.length < 2) return null;
  const state: CompareState = { panes: assets.slice(0, 4).map((asset) => ({ asset, view: "dark-line", selection: null })), scale: 100, timeS: null };
  return compareToParams(state);
}

/** 💾️ A new saved collection. */
export const savedCollection = (name: string, assets: readonly Slug[], id: string = freshId("saved")): SavedCollection => ({ id, name: name.trim(), assets: [...new Set(assets)] });

/** ✏️ A saved collection with an asset added (once) or removed. */
export const withAsset = (collection: SavedCollection, asset: Slug, present: boolean): SavedCollection => ({ ...collection, assets: present ? (collection.assets.includes(asset) ? collection.assets : [...collection.assets, asset]) : collection.assets.filter((candidate) => candidate !== asset) });
