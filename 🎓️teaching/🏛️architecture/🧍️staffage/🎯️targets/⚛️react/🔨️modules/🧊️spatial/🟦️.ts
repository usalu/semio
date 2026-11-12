/** 🧊️ The pure rules of the interactive 3D view: the camera presets, the mapping of the seven representations onto the variants of a model, the level of detail of a scale, the key that says when a model must be built again, and the spoken description of the model state. No 3D code is loaded here; the viewer, the model pipeline and the exporters sit behind {@link ./🚪️gate/🟦️.ts}, so a page that only draws pays nothing for them.
 *
 * A model is built for an asset at one scale (the level of detail), one pose or clip and one analytic setting; the representation (view) and the camera never rebuild it: the seven views are the variants of the one geometry, a preset only turns the camera.
 *
 * @see ./🚪️gate/🟦️.ts — the lazy entries
 * @see ../🧍️figure/🟦️.tsx — the figure that mounts it for the `model` viewpoint
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🟦️.ts — the owned WebGL2 viewer
 */
import { VIEWS } from "@teaching/staffage";
import type { Asset, DisplayRange, Environment, EvaluationQuery, ForceQuantity, Language, ThermalQuantity, View } from "@teaching/staffage";
import { formatNumber } from "@teaching/staffage";
import { translator, type Key } from "../🌐️i18n/🟦️.ts";
import { VIEW_NAMES } from "../🗣️labels/🟦️.ts";

/** 📷️ The camera presets of the 3D view, in the order of the keys 1 to 5: the three planar viewpoints, the axonometric drawing's direction and a perspective view. */
export const CAMERA_PRESETS = ["elevation", "front", "plan", "axonometric", "perspective"] as const;

/** 🔭️ One of {@link CAMERA_PRESETS}. */
export type CameraPresetName = (typeof CAMERA_PRESETS)[number];

/** 👁️ The preset a reader first sees. */
export const DEFAULT_CAMERA: CameraPresetName = "perspective";

/** ❓️ Whether a string names a camera preset. */
export const isCameraPreset = (value: string | null | undefined): value is CameraPresetName => (CAMERA_PRESETS as readonly string[]).includes(value ?? "");

/** 🎭️ The variant of a model that shows a representation: the seven views are the variants of the model, under the id of the view. */
export const variantOf = (view: View): string => view;

/** 🎥️ The views of a model in variant order. */
export const MODEL_VIEWS: readonly View[] = VIEWS;

/** 🧾️ What a model is built from. The `query` is a named pose or one clip (its time is the clock's, not part of the model). */
export type SpatialRequest = {
  readonly view: View;
  readonly scale: number;
  readonly query: EvaluationQuery;
  readonly language: Language;
  readonly environment?: Environment;
  readonly thermalQuantity?: ThermalQuantity;
  readonly forceQuantity?: ForceQuantity;
  readonly range?: DisplayRange;
  readonly theme?: "light" | "dark";
};

/** 🎚️ The tessellation of a model at a scale (the level of detail): the chord deviation is 0.1 mm of paper at that scale, never finer than 1/800 of the largest dimension (the export quality) and never coarser than 1/60 of it; coarser scales also use fewer segments per circle. */
export function qualityFor(asset: Asset, scale: number): { readonly chordM: number; readonly minSegments: number; readonly maxSegments: number } {
  const largest = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
  const finest = Math.min(0.02, Math.max(0.0005, largest / 800));
  const chordM = Math.min(Math.max(scale * 0.0001, finest), Math.max(finest, largest / 60));
  return { chordM, minSegments: scale <= 20 ? 12 : 8, maxSegments: scale <= 20 ? 64 : scale <= 100 ? 48 : 32 };
}

/** 🔑️ What, when it changes, makes the model a new one: the scale, the pose or clip, and the analytic settings of the thermal and structural looks. The view and the camera are not part of it. */
export function modelKey(asset: Asset, request: SpatialRequest): string {
  const { query } = request;
  const what = query.kind === "pose" ? ["pose", query.pose] : query.kind === "clip" ? ["clip", query.clip] : [query.kind];
  return JSON.stringify([asset.id, request.scale, what, request.environment ?? null, request.thermalQuantity ?? null, request.forceQuantity ?? null, request.range?.mode === "span" ? request.range.spanK : null, request.theme ?? null]);
}

/** 🗣️ The spoken description of the model state in a language: the asset, pose or clip with its time, the representation, the scale and the camera. */
export function describeModel(asset: Asset, request: SpatialRequest & { readonly timeS?: number }, camera: CameraPresetName | null, language: Language): string {
  const t = translator(language);
  const { query } = request;
  const state =
    query.kind === "pose"
      ? t("model.state.pose", { pose: asset.poses.find((pose) => pose.id === query.pose)?.title[language] ?? query.pose })
      : query.kind === "clip"
        ? t("model.state.clip", { clip: asset.clips.find((clip) => clip.id === query.clip)?.title[language] ?? query.clip, time: formatNumber(request.timeS ?? query.timeS, 2, language) })
        : "";
  return t("model.describe", { title: asset.title[language], state, representation: VIEW_NAMES[request.view][language], scale: formatNumber(request.scale, Number.isInteger(request.scale) ? 0 : 1, language), camera: camera === null ? t("model.camera.free") : t(`model.camera.${camera}` as Key) });
}
