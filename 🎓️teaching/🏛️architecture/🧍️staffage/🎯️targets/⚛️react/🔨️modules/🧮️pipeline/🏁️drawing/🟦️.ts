import type { Asset, DepictionOptions, DrawingScene, EvaluationQuery } from "@teaching/staffage";
import { unpackAsset, type PackedAsset } from "../../📚️catalog/🗜️payload/🟦️.ts";
import { evaluateState, inlineSvg, sceneOf, statePoints } from "../🟦️.ts";

/** 🎞️ Display data sent back from the drawing worker; `points` is what the drawn instant weighs ({@link statePoints}). */
export type Drawing = Pick<DrawingScene, "widthMm" | "heightMm"> & Pick<DrawingScene["metadata"], "origin" | "mmPerMetre" | "fidelity"> & { readonly markup: string; readonly rootM: number; readonly points: number };

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
  return { widthMm: scene.widthMm, heightMm: scene.heightMm, origin, mmPerMetre, fidelity, markup: inlineSvg(scene), rootM: state.bones[asset.rig.root]?.origin.x ?? 0, points: statePoints(state) };
}

/** 📬️ The drawing request protocol, independent of the transport. A catalog asset travels as the compressed payload it was shipped as (`packed`), any other asset as it is.
 * @see ../../📚️catalog/🗜️payload/🟦️.ts */
export type DrawingRequest =
  | { readonly kind: "asset"; readonly id: number; readonly asset: Asset }
  | { readonly kind: "packed"; readonly id: number; readonly packed: PackedAsset }
  | { readonly kind: "release"; readonly id: number }
  | { readonly kind: "draw"; readonly id: number; readonly lease: number; readonly options: DepictionOptions; readonly query: EvaluationQuery; readonly wrapLoops: boolean };

/** 📨️ Replies to draw requests, with the owning lease for cancellation. */
export type DrawingReply =
  | { readonly lease: number; readonly drawing: Drawing }
  | { readonly lease: number; readonly error: string };

/** 🧰️ The worker's side of the protocol: keeps the assets it was handed and answers each draw request; every other request has no answer. */
export function drawingBench(): (request: DrawingRequest) => DrawingReply | null {
  const assets = new Map<number, Asset>();
  return (request) => {
    if (request.kind === "asset") return assets.set(request.id, request.asset), null;
    if (request.kind === "packed") return assets.set(request.id, unpackAsset(request.packed)), null;
    if (request.kind === "release") return assets.delete(request.id), null;
    try {
      const asset = assets.get(request.id);
      if (!asset) throw new Error("Drawing worker does not hold the requested asset");
      return { lease: request.lease, drawing: drawFigure(asset, request.options, request.query, request.wrapLoops) };
    } catch (error) {
      return { lease: request.lease, error: error instanceof Error ? error.message : String(error) };
    }
  };
}
