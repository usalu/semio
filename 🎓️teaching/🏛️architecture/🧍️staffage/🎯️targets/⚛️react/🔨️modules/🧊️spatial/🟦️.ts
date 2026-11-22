/** 🧊️ The pure rules of the interactive 3D view: the camera presets, the mapping of the seven representations onto the variants of a model, the level of detail of a scale, the key that says when a model must be built again, and the spoken description of the model state. No 3D code is loaded here; the viewer, the model pipeline and the exporters sit behind {@link ./🚪️gate/🟦️.ts}, so a page that only draws pays nothing for them.
 *
 * A model is built for an asset at one pose or clip and one analytic setting and holds every authored scale and mode as a scene (`modelOfAsset`): the scale and the representation only select the scene (`sceneOf`: the scale and the mode of the view) and the variant of the view, so changing them never rebuilds the model; a preset only turns the camera.
 *
 * @see ./🚪️gate/🟦️.ts — the lazy entries
 * @see ../🧍️figure/🟦️.tsx — the figure that mounts it for the `model` viewpoint
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🟦️.ts — the owned WebGL2 viewer
 */
import { VIEWS, authoredScales, modeOf, sceneIdOf } from "@teaching/staffage";
import type { Asset, DisplayRange, Environment, EvaluationQuery, FlowLayer, ForceQuantity, Language, ThermalQuantity, View } from "@teaching/staffage";
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

/** 🧾️ What a model is built from. The `query` is a named pose or one clip (its time is the clock's, not part of the model); `scales` (default every authored scale) limits the model to those scales, as the 3D playground needs one scale for many assets. */
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
  readonly scales?: readonly number[];
  /** 🌊️ The layers of the air flow shown over the thermal view; with any layer the model is built without emitter markers, since the flow itself shows where air leaves. */
  readonly flow?: readonly FlowLayer[];
};

/** 🏛️ The scene of the model that a request shows: the scale and the mode of the view (`s<scale>-<mode>`); the tessellation of every scene is the one `qualityOf` of the library, the very rule the downloads use. */
export const sceneOf = (request: Pick<SpatialRequest, "scale" | "view">): string => sceneIdOf(request.scale, modeOf(request.view));

/** 🔢️ The scales an asset has a 3D model at: its authored scales, finest first. A scale that is not among them has no 3D model (the viewer refuses it by name, never answers with another scale). */
export const modelScalesOf = (asset: Asset): readonly number[] => authoredScales(asset.volumes);

/** 🪜️ The preset scales a model can be chosen at, finest first (the order of the scale buttons). */
export const MODEL_SCALES: readonly number[] = [50, 100, 200, 500];

/** 🔢️ One preset scale of the scale control: it has a model, or it has none and the reason is in the reader's language (the reason of its representation, else the scales that do have a model). */
export type ScaleChoice = { readonly scale: number; readonly available: true } | { readonly scale: number; readonly available: false; readonly reason: string };

/** 🧮️ The scale choices of an asset: available exactly where it has an authored volume; no other scale stands in. */
export function scaleChoices(asset: Asset, language: Language): readonly ScaleChoice[] {
  const have = modelScalesOf(asset);
  return MODEL_SCALES.map((scale): ScaleChoice => {
    if (have.includes(scale)) return { scale, available: true };
    const representation = asset.representations[`${scale}` as "50"];
    const stated = representation?.status === "unavailable" ? representation.reason[language].trim() : "";
    return { scale, available: false, reason: stated === "" ? translator(language)("model.noScaleList", { scales: have.map((s) => `1:${s}`).join(", ") }) : stated };
  });
}

/** 🔑️ What, when it changes, makes the model a new one: the pose or clip, and the analytic settings of the thermal and structural looks. The scale, the view and the camera are not part of it: they select a scene and a variant of the one model. */
export function modelKey(asset: Asset, request: SpatialRequest): string {
  const { query } = request;
  const what = query.kind === "pose" ? ["pose", query.pose] : query.kind === "clip" ? ["clip", query.clip] : [query.kind];
  return JSON.stringify([asset.id, what, request.environment ?? null, request.thermalQuantity ?? null, request.forceQuantity ?? null, request.range?.mode === "span" ? request.range.spanK : null, request.theme ?? null, request.scales ?? null, (request.flow?.length ?? 0) > 0]);
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
