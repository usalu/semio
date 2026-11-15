/** 🏭️ The model pipeline of the 3D view: an asset at a request becomes one procedural neutral model with a scene per authored scale and mode (`modelOfAsset`), every scene is tessellated at the quality of its own scale (`qualityOf`, the rule of the downloads) and gets its feature edges for the line views, in cooperative slices that report progress and stop on cancellation. The prepared document is kept for the next time the same asset, pose or clip and analytic settings are asked for; the clip keeps its root motion exactly as the downloads do, the viewer follows the figure instead of cutting its travel.
 *
 * Progress is one fraction over four weighted phases (the viewer's own scene preparation is the fourth and is reported by the stage). Cancellation rejects with an `AbortError`, never with a half-built document.
 *
 * @see ../🟦️.ts — the request, the level of detail and the key
 * @see ../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts — the kernel
 */
import { ModelRequestError, EDGE_ANGLE_RAD, authoredScales, edgeMaterialOf, modelOfAsset, qualityOf, resolveScenes } from "@teaching/staffage";
import type { Asset, AssetModelOptions } from "@teaching/staffage";
import { CancelledError, featureEdges, type ModelDocument, type Operation } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { modelKey, type SpatialRequest } from "../🟦️.ts";

/** 🏷️ The phases of preparing a model, in order. */
export const PREPARE_PHASES = ["model", "tessellate", "edges", "scene"] as const;

/** 📤️ One of {@link PREPARE_PHASES}. */
export type PreparePhase = (typeof PREPARE_PHASES)[number];

/** 📐️ How much of the whole bar each phase is. */
export const PHASE_WEIGHTS: Readonly<Record<PreparePhase, number>> = { model: 0.45, tessellate: 0.3, edges: 0.1, scene: 0.15 };

/** ⏱️ Where a phase starts on the whole bar. */
export const phaseStart = (phase: PreparePhase): number => PREPARE_PHASES.slice(0, PREPARE_PHASES.indexOf(phase)).reduce((sum, next) => sum + PHASE_WEIGHTS[next], 0);

/** 🎛️ Progress and cancellation of a preparation. */
export type PrepareControl = { readonly signal?: AbortSignal; readonly progress?: (fraction: number, phase: PreparePhase) => void };

const CAPACITY = 3;
const prepared = new WeakMap<Asset, Map<string, ModelDocument>>();

/** 🧾️ The options of a model that do not name a pose or clip: the analytic settings (environment, thermal span, quantities, theme) of a request. */
export function analyticOptionsOf(request: Pick<SpatialRequest, "environment" | "thermalQuantity" | "forceQuantity" | "range" | "theme">): Pick<AssetModelOptions, "environment" | "thermalSpanK" | "thermalQuantity" | "forceQuantity" | "theme"> {
  return {
    ...(request.environment === undefined ? {} : { environment: request.environment }),
    ...(request.thermalQuantity === undefined ? {} : { thermalQuantity: request.thermalQuantity }),
    ...(request.forceQuantity === undefined ? {} : { forceQuantity: request.forceQuantity }),
    ...(request.range?.mode === "span" ? { thermalSpanK: request.range.spanK } : {}),
    ...(request.theme === undefined ? {} : { theme: request.theme }),
  };
}

/** 🌟️ The options `modelOfAsset` makes the model of a request from: {@link analyticOptionsOf}, the pose or the clip (its first instant is the static state) and every authored scale. A transition has no model. */
export function modelOptionsOf(_asset: Asset, request: SpatialRequest): AssetModelOptions {
  const { query } = request;
  if (query.kind === "transition") throw new ModelRequestError("model-bad-option", "transition");
  const base = analyticOptionsOf(request);
  return query.kind === "pose" ? { ...base, at: { kind: "pose", pose: query.pose }, poses: [query.pose], clips: [] } : { ...base, at: { kind: "clip", clip: query.clip, timeS: 0 }, poses: [], clips: [query.clip] };
}

/** ✂️ The document without the feature edges of its glyphs: a glyph (arrow, arc, emitter cone) is drawn only by the analytic variants that give it a visible material, so the outline of a glyph that is invisible in the other variants would hang in the picture as stray lines. */
export function withoutGlyphEdges(document: ModelDocument): ModelDocument {
  const glyphMeshes = new Set(document.nodes.filter((node) => node.mesh !== undefined && node.tags?.includes("glyph") === true).map((node) => node.mesh));
  return { ...document, meshes: document.meshes.map((mesh) => (glyphMeshes.has(mesh.id) ? { ...mesh, primitives: mesh.primitives.filter((primitive) => primitive.mode !== "lines") } : mesh)) };
}

const operationOf = (control: PrepareControl, phase: PreparePhase): Operation => ({
  report: (fraction) => control.progress?.(phaseStart(phase) + PHASE_WEIGHTS[phase] * Math.min(1, Math.max(0, fraction)), phase),
  cancelled: () => control.signal?.aborted === true,
});

/** 🗃️ The prepared document of a request if it is still kept. */
export const preparedOf = (asset: Asset, request: SpatialRequest): ModelDocument | undefined => prepared.get(asset)?.get(modelKey(asset, request));

/** 🚨️ Prepares the resolved model of a request: model, tessellation of every scene at its scale's quality, feature edges. Rejects with the signal's reason (an `AbortError`) when cancelled and with a {@link ModelRequestError} for an asset without an authored scale or a request that makes no model. */
export async function prepareModel(asset: Asset, request: SpatialRequest, control: PrepareControl = {}): Promise<ModelDocument> {
  const key = modelKey(asset, request);
  const kept = prepared.get(asset)?.get(key);
  if (kept !== undefined) {
    control.progress?.(phaseStart("scene"), "scene");
    return kept;
  }
  try {
    if (authoredScales(asset.volumes).length === 0) throw new ModelRequestError("model-no-authored-scale", asset.id);
    const procedural = await modelOfAsset(asset, modelOptionsOf(asset, request), operationOf(control, "model"));
    const resolved = await resolveScenes(procedural, (scale) => qualityOf(asset, scale), operationOf(control, "tessellate"));
    const edged = withoutGlyphEdges(await featureEdges(resolved, EDGE_ANGLE_RAD, { material: edgeMaterialOf(procedural) }, operationOf(control, "edges")));
    let cache = prepared.get(asset);
    if (cache === undefined) prepared.set(asset, (cache = new Map()));
    cache.set(key, edged);
    if (cache.size > CAPACITY) cache.delete(cache.keys().next().value as string);
    return edged;
  } catch (error) {
    if (error instanceof CancelledError || control.signal?.aborted === true) throw control.signal?.reason ?? new DOMException("This operation was aborted", "AbortError");
    throw error;
  }
}
