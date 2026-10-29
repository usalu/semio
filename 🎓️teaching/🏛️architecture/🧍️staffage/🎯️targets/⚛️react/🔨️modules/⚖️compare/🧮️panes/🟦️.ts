/** 🧮️ Pure rules of the comparison: default panes, the quick actions, selection keys and the clock span.
 *
 * @see ../../🗺️routing/🟦️.ts — `ComparePane`, `CompareState` and their address parameters
 */
import { VIEWS } from "@teaching/staffage";
import type { Asset, EvaluationQuery, Slug, View } from "@teaching/staffage";
import { COMPARE_LIMITS, type ComparePane } from "../../🗺️routing/🟦️.ts";
import type { Selection } from "../../💾️persistence/🟦️.ts";

/** 🎞️ The views a same-asset comparison walks through, in order. */
export const ACROSS_VIEWS: readonly View[] = ["dark-line", "thermal", "forces", "moments"];

/** 🔑️ The string a select holds for a selection: `default`, `pose:<id>` or `clip:<id>`. */
export function selectionKey(selection: Selection | null): string {
  return selection === null ? "default" : selection.kind === "pose" ? `pose:${selection.pose}` : `clip:${selection.clip}`;
}

/** 🔑️ The selection a {@link selectionKey} names. */
export function selectionFromKey(key: string): Selection | null {
  const [kind, slug] = key.split(":");
  return slug === undefined ? null : kind === "pose" ? { kind: "pose", pose: slug } : kind === "clip" ? { kind: "clip", clip: slug } : null;
}

/** 🎬️ The two default panes: the preferred asset in a dark line drawing and in the thermal view, else the first two ready assets. */
export function defaultPanes(ready: readonly Slug[], preferred: Slug | undefined = undefined): readonly ComparePane[] {
  if (ready.length === 0) return [];
  const wanted = preferred !== undefined && ready.includes(preferred);
  const first = wanted ? preferred : ready[0]!;
  if (ready.length === 1 || wanted) return [{ asset: first, view: "dark-line", selection: null }, { asset: first, view: "thermal", selection: null }];
  return [{ asset: ready[0]!, view: "dark-line", selection: null }, { asset: ready[1]!, view: "dark-line", selection: null }];
}

/** 🪞️ One asset in `count` different views. */
export function sameAssetAcrossViews(asset: Slug, count: number, selection: Selection | null = null): readonly ComparePane[] {
  const views = [...ACROSS_VIEWS, ...VIEWS.filter((view) => !ACROSS_VIEWS.includes(view))];
  return views.slice(0, Math.min(Math.max(count, COMPARE_LIMITS.min), COMPARE_LIMITS.max)).map((view) => ({ asset, view, selection }));
}

/** 👥️ `count` different assets in one view, starting with `first`. */
export function differentAssetsSameView(ready: readonly Slug[], first: Slug, view: View, count: number): readonly ComparePane[] {
  const order = [first, ...ready.filter((id) => id !== first)];
  return order.slice(0, Math.min(Math.max(count, COMPARE_LIMITS.min), COMPARE_LIMITS.max)).map((asset) => ({ asset, view, selection: null }));
}

/** ➕️ A pane to append: the last pane's asset in the next view not yet shown. */
export function nextPane(panes: readonly ComparePane[]): ComparePane {
  const last = panes[panes.length - 1]!;
  const used = new Set(panes.filter((pane) => pane.asset === last.asset).map((pane) => pane.view));
  return { asset: last.asset, view: VIEWS.find((view) => !used.has(view)) ?? last.view, selection: null };
}

/** 🔁️ The first two panes exchanged. */
export function swapFirstTwo(panes: readonly ComparePane[]): readonly ComparePane[] {
  return panes.length < 2 ? panes : [panes[1]!, panes[0]!, ...panes.slice(2)];
}

/** 🎯️ The query of a pane that a selection fixes; `undefined` leaves the asset's default clip to the clock. */
export function paneQuery(asset: Asset, selection: Selection | null): EvaluationQuery | undefined {
  if (selection === null) return undefined;
  if (selection.kind === "pose") return asset.poses.some((pose) => pose.id === selection.pose) ? { kind: "pose", pose: selection.pose } : undefined;
  return asset.clips.some((clip) => clip.id === selection.clip) ? { kind: "clip", clip: selection.clip, timeS: 0 } : undefined;
}

/** ⏱️ The shared clock's span: the longest clip any pane plays (0 when all panes show poses). */
export function clockSpan(panes: readonly ComparePane[], find: (id: Slug) => Asset | undefined): number {
  return panes.reduce((longest, pane) => {
    const asset = find(pane.asset);
    if (asset === undefined || pane.selection?.kind === "pose") return longest;
    const id = pane.selection?.kind === "clip" ? pane.selection.clip : asset.presentation.defaultClip;
    return Math.max(longest, asset.clips.find((clip) => clip.id === id)?.durationS ?? 0);
  }, 0);
}
