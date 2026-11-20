import type { Asset, DepictionOptions, DrawingScene, EvaluationQuery } from "@teaching/staffage";
import { evaluateState, inlineSvg, sceneOf } from "../🟦️.ts";

/** 🎞️ Display data sent back from the drawing worker. */
export type Drawing = Pick<DrawingScene, "widthMm" | "heightMm"> & Pick<DrawingScene["metadata"], "origin" | "mmPerMetre" | "fidelity"> & { readonly markup: string; readonly rootM: number };

function drawingQueryOf(asset: Asset, query: EvaluationQuery, options: DepictionOptions): EvaluationQuery {
  if (query.kind !== "clip" || !(options.view.endsWith("-line") || options.view.endsWith("-solid"))) return query;
  const clip = asset.clips.find(candidate => candidate.id === query.clip);
  if (!clip?.loop || clip.durationS <= 0) return query;
  const nanosPerSecond = 1e9;
  const duration = Math.round(clip.durationS * nanosPerSecond);
  const time = Math.round((query.timeS % clip.durationS) * nanosPerSecond);
  return duration > 0 ? { ...query, timeS: ((time % duration) + duration) % duration / nanosPerSecond } : query;
}

/** 🖌️ Draws one instant using the shared scene and SVG pipeline. */
export function drawFigure(asset: Asset, options: DepictionOptions, query: EvaluationQuery, wrapLoops = false): Drawing {
  const state = evaluateState(asset, wrapLoops ? drawingQueryOf(asset, query, options) : query, options.representation);
  const scene = sceneOf(state, options);
  const { origin, mmPerMetre, fidelity } = scene.metadata;
  return { widthMm: scene.widthMm, heightMm: scene.heightMm, origin, mmPerMetre, fidelity, markup: inlineSvg(scene), rootM: state.bones[asset.rig.root]?.origin.x ?? 0 };
}

/** 📬️ The drawing request protocol, independent of the transport. */
export type DrawingRequest =
  | { readonly kind: "asset"; readonly id: number; readonly asset: Asset }
  | { readonly kind: "release"; readonly id: number }
  | { readonly kind: "draw"; readonly id: number; readonly lease: number; readonly options: DepictionOptions; readonly query: EvaluationQuery; readonly wrapLoops: boolean };

/** 📨️ Replies to draw requests, with the owning lease for cancellation. */
export type DrawingReply =
  | { readonly lease: number; readonly drawing: Drawing }
  | { readonly lease: number; readonly error: string };
