/** 🪟️ Which surfaces a document browser actor makes visible on each render turn (`🔣️.json`): every verified window, every
 * verified panel body and every reserved refresh section — engagements, measures and tool measures on every turn, the
 * app-static catalogue at mount — so an actor-bound document's window chrome follows the live document exactly as the local
 * refresh's does (ticket 26/09/23 S18 §14c, C13 P1: the shell read these sections from its LOCAL instance, which never sees
 * a hub document, so draw's "N layers" stayed at 1 after committed `addLayer`s). Pure: the worker encodes each surface's
 * view context. */

import contract from "./🔣️.json" with { type: "json" };

export const BROWSER_ACTOR_VISIBLE_SURFACES_V1 = contract;

/** 🎬️ `mount` = the first paint of an actor lifetime, `repaint` = a host view change or an own edit, `refresh` = a remote
 * edit or an action that changed no document. */
export type BrowserActorVisibleTurnV1 = keyof typeof contract.turns;

export type BrowserActorVisibleSurfaceV1 = Readonly<{ surface: string; bodyKey: string; context: "window" | "panel" | "section" }>;

type RenderSurfaceV1 = Readonly<{ key: string; bodyKey: string }>;

/** 🪟️ The surfaces one turn announces, in announcement order: the windows (when the turn repaints them), the panels, then
 * the sections (the static catalogue only when the turn paints it). */
export function browserActorVisibleSurfacesV1(surfaces: Readonly<{ windows: readonly RenderSurfaceV1[]; panels: readonly RenderSurfaceV1[] }>, turn: BrowserActorVisibleTurnV1): readonly BrowserActorVisibleSurfaceV1[] {
  const { windows, staticSections } = contract.turns[turn];
  return [
    ...(windows ? surfaces.windows.map(({ key, bodyKey }) => ({ surface: key, bodyKey, context: "window" as const })) : []),
    ...surfaces.panels.map(({ key, bodyKey }) => ({ surface: key, bodyKey, context: "panel" as const })),
    ...contract.sections.filter((section) => staticSections || !section.static).map(({ bodyKey }) => ({ surface: bodyKey, bodyKey, context: "section" as const })),
  ];
}
