/** 🏭️ The model pipeline of the 3D view: an asset at a request becomes a procedural neutral model (`modelOf`), is tessellated at the level of detail of its scale and gets its feature edges for the line views, in cooperative slices that report progress and stop on cancellation. The prepared document is kept for the next time the same asset, scale, pose or clip and analytic settings are asked for.
 *
 * Progress is one fraction over four weighted phases (the viewer's own scene preparation is the fourth and is reported by the stage). Cancellation rejects with an `AbortError`, never with a half-built document.
 *
 * @see ../🟦️.ts — the request, the level of detail and the key
 * @see ../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts — the kernel
 */
import { ModelRequestError, modelOf, EDGE_ANGLE_RAD } from "@teaching/staffage";
import type { Asset, ModelOptions } from "@teaching/staffage";
import { CancelledError, featureEdges, tessellate, type ModelDocument, type Operation } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { modelKey, qualityFor, type SpatialRequest } from "../🟦️.ts";

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

/** 🧾️ The options of a model that do not name a pose or clip: the scale and the analytic settings (environment, thermal span, quantities, theme) of a request. */
export function analyticOptionsOf(request: Pick<SpatialRequest, "scale" | "environment" | "thermalQuantity" | "forceQuantity" | "range" | "theme">): Pick<ModelOptions, "scale" | "environment" | "thermalSpanK" | "thermalQuantity" | "forceQuantity" | "theme"> {
  return {
    scale: request.scale,
    ...(request.environment === undefined ? {} : { environment: request.environment }),
    ...(request.thermalQuantity === undefined ? {} : { thermalQuantity: request.thermalQuantity }),
    ...(request.forceQuantity === undefined ? {} : { forceQuantity: request.forceQuantity }),
    ...(request.range?.mode === "span" ? { thermalSpanK: request.range.spanK } : {}),
    ...(request.theme === undefined ? {} : { theme: request.theme }),
  };
}

/** 🌟️ The options `modelOf` makes the model of a request from: {@link analyticOptionsOf}, the pose or the clip (its first instant is the static state) and all seven views. A transition has no model. */
export function modelOptionsOf(asset: Asset, request: SpatialRequest): ModelOptions {
  const { query } = request;
  if (query.kind === "transition") throw new ModelRequestError("model-bad-option", "transition");
  const base = analyticOptionsOf(request);
  return query.kind === "pose" ? { ...base, at: { kind: "pose", pose: query.pose }, poses: [query.pose], clips: [] } : { ...base, at: { kind: "clip", clip: query.clip, timeS: 0 }, poses: [], clips: [query.clip] };
}

/** 🧭️ The clips of a document played in place: the horizontal travel of the root nodes (neutral X and Z) is replaced by the rest position, the height (Y) and every joint motion stay. A camera that keeps its place cannot follow a car that drives away; the drawing too shows a figure at its origin. */
export function inPlace(document: ModelDocument): ModelDocument {
  const roots = new Map(document.nodes.filter((node) => node.parent === null).map((node) => [node.id, node]));
  return {
    ...document,
    animations: document.animations.map((animation) => ({
      ...animation,
      channels: animation.channels.map((channel) => {
        const root = "node" in channel.target && channel.target.path === "translation" ? roots.get(channel.target.node) : undefined;
        if (root === undefined) return channel;
        const values = channel.values.slice();
        for (let i = 0; i + 2 < values.length; i += 3) {
          values[i] = root.translation[0];
          values[i + 2] = root.translation[2];
        }
        return { ...channel, values };
      }),
    })),
  };
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

/** 🚨️ Prepares the resolved model of a request: model, tessellation, feature edges. Rejects with the signal's reason (an `AbortError`) when cancelled and with a {@link ModelRequestError} for an asset without volume or a request that makes no model. */
export async function prepareModel(asset: Asset, request: SpatialRequest, control: PrepareControl = {}): Promise<ModelDocument> {
  const key = modelKey(asset, request);
  const kept = prepared.get(asset)?.get(key);
  if (kept !== undefined) {
    control.progress?.(phaseStart("scene"), "scene");
    return kept;
  }
  try {
    const procedural = inPlace(await modelOf(asset, modelOptionsOf(asset, request), operationOf(control, "model")));
    const resolved = await tessellate(procedural, qualityFor(asset, request.scale), operationOf(control, "tessellate"));
    const edged = withoutGlyphEdges(await featureEdges(resolved, EDGE_ANGLE_RAD, { material: "edge" }, operationOf(control, "edges")));
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
