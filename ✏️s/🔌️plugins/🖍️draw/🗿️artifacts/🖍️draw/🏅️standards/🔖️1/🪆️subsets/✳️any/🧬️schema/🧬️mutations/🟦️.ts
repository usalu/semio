/** 🧩️ Draw direct-mutation discriminated union — mirrors the Rust `DrawMutation` dispatch enum
 * (sibling `🦀️.rs`, `#[serde(tag = "mutation", rename_all = "camelCase")]`), same
 * declaration order and camelCase discriminant per variant. Same shape as the jack reference
 * (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts`).
 *
 * Six verbs import their payload from an existing `<verb>/🦠️mutation/🟦️.ts` leaf. The
 * other eight (`renameLayer`, `updateLayerTransform`, `replaceLayerFill`, `replaceLayerStroke`,
 * `setLayerBooleanOperation`, `updateLayerTraceParams`, `createLayer`, `deleteLayer`) have no TS
 * mutation leaf on disk yet — `📓️ts-mirrors-draw.md` found the repo's own
 * `policyMutationTsMirrorBreaches` gate (`📜️script.ts`) treats an absent leaf as "low" priority,
 * non-blocking advisory, so none were scaffolded — their payload shapes are inlined here instead,
 * each annotated with its Rust source. */
import type { SetLayerVisible } from "./👁️set-layer-visible/🦠️mutation/🟦️.ts";
import type { SetLayerLocked } from "./🔒️set-layer-locked/🦠️mutation/🟦️.ts";
import type { SetLayerOpacity } from "./🌫️set-layer-opacity/🦠️mutation/🟦️.ts";
import type { SetLayerBlendMode } from "./🖌️set-layer-blend-mode/🦠️mutation/🟦️.ts";
import type { DuplicateLayer } from "./🧬️duplicate-layer/🦠️mutation/🟦️.ts";
import type { ReorderLayer } from "./🔃reorder-layer/🦠️mutation/🟦️.ts";
import type { DrawLayerNode } from "../🟦️.ts";

/** ✏️ Mirrors Rust `RenameLayer` (`✏️rename-layer/🦠️mutation/🦀️.rs`). */
export interface RenameLayer {
  layerId: string;
  newName: string;
}

/** 🔄️ Mirrors Rust `DrawTransform` (`🗿️artifacts/🖍️draw/🦀️.rs`). */
export interface DrawTransform {
  x: number;
  y: number;
  scaleX: number;
  scaleY: number;
  rotation: number;
}

/** 🔄️ Mirrors Rust `UpdateLayerTransform` (`🔄️update-layer-transform/🦠️mutation/🦀️.rs`). */
export interface UpdateLayerTransform {
  layerId: string;
  transform: DrawTransform;
}

/** 🎨️ Mirrors Rust `GradientStop` (`🗿️artifacts/🖍️draw/🦀️.rs`). */
export interface GradientStop {
  offset: number;
  color: [number, number, number, number];
}

/** 🎨️ Mirrors Rust `FillStyle` (`🗿️artifacts/🖍️draw/🦀️.rs`), tagged on `kind`. */
export type FillStyle =
  | { kind: "solid"; color: [number, number, number, number] }
  | { kind: "linearGradient"; x1: number; y1: number; x2: number; y2: number; stops: GradientStop[] }
  | { kind: "radialGradient"; cx: number; cy: number; r: number; stops: GradientStop[] };

/** 🔁 Mirrors Rust `ReplaceLayerFill` (`🔁replace-layer-fill/🦠️mutation/🦀️.rs`). */
export interface ReplaceLayerFill {
  layerId: string;
  fill?: FillStyle;
}

/** ✏️ Mirrors Rust `StrokeStyle` (`🗿️artifacts/🖍️draw/🦀️.rs`). */
export interface StrokeStyle {
  color: [number, number, number, number];
  width: number;
  cap: string;
  join: string;
  dash?: number[];
}

/** ♻️ Mirrors Rust `ReplaceLayerStroke` (`♻️replace-layer-stroke/🦠️mutation/🦀️.rs`). */
export interface ReplaceLayerStroke {
  layerId: string;
  stroke?: StrokeStyle;
}

/** 🔀 Mirrors Rust `SetLayerBooleanOperation` (`🔀set-layer-boolean-operation/🦠️mutation/🦀️.rs`). */
export interface SetLayerBooleanOperation {
  layerId: string;
  booleanOperation: string;
}

/** 🔧 Mirrors Rust `DrawTraceParams` (`🗿️artifacts/🖍️draw/🦀️.rs`). */
export interface DrawTraceParams {
  threshold: number;
  simplifyEpsilon: number;
}

/** 🔧 Mirrors Rust `UpdateLayerTraceParams` (`🔧update-layer-trace-params/🦠️mutation/🦀️.rs`). */
export interface UpdateLayerTraceParams {
  layerId: string;
  params: DrawTraceParams;
}

/** 🌱 Mirrors Rust `CreateLayer` (`🌱create-layer/🦠️mutation/🦀️.rs`). */
export interface CreateLayer {
  parentId?: string;
  index?: number;
  layer: DrawLayerNode;
}

/** 🗑️ Mirrors Rust `DeleteLayer` (`🗑️delete-layer/🦠️mutation/🦀️.rs`). */
export interface DeleteLayer {
  layerId: string;
}

/** 🧩️ One arm per `DrawMutation` variant, same declaration order as the Rust enum. */
export type DrawMutation =
  | ({ mutation: "setLayerVisible" } & SetLayerVisible)
  | ({ mutation: "setLayerLocked" } & SetLayerLocked)
  | ({ mutation: "setLayerOpacity" } & SetLayerOpacity)
  | ({ mutation: "setLayerBlendMode" } & SetLayerBlendMode)
  | ({ mutation: "renameLayer" } & RenameLayer)
  | ({ mutation: "updateLayerTransform" } & UpdateLayerTransform)
  | ({ mutation: "replaceLayerFill" } & ReplaceLayerFill)
  | ({ mutation: "replaceLayerStroke" } & ReplaceLayerStroke)
  | ({ mutation: "setLayerBooleanOperation" } & SetLayerBooleanOperation)
  | ({ mutation: "updateLayerTraceParams" } & UpdateLayerTraceParams)
  | ({ mutation: "createLayer" } & CreateLayer)
  | ({ mutation: "duplicateLayer" } & DuplicateLayer)
  | ({ mutation: "deleteLayer" } & DeleteLayer)
  | ({ mutation: "reorderLayer" } & ReorderLayer);
