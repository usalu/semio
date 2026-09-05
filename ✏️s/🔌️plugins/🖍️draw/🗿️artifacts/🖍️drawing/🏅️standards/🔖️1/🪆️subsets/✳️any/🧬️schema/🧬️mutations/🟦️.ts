/** 🧩️ Drawing direct-mutation discriminated union — mirrors the Rust `DrawingMutation` dispatch enum
 * (sibling `🦀️.rs`, `#[serde(tag = "mutation", rename_all = "camelCase")]`), same
 * declaration order and camelCase discriminant per variant. Same shape as the jack reference
 * (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts`).
 *
 * Six verbs import their payload from an existing `<verb>/🦠️mutation/🟦️.ts` leaf. The
 * other eight (`renameLayer`, `updateLayerTransform`, `replaceLayerFill`, `replaceLayerStroke`,
 * `setLayerBooleanOperation`, `updateLayerTraceParams`, `createLayer`, `deleteLayer`) have no TS
 * mutation leaf on disk yet — `📓️ts-mirrors-drawing.md` found the repo's own
 * `policyMutationTsMirrorBreaches` gate (`📜️script.ts`) treats an absent leaf as "low" priority,
 * non-blocking advisory, so none were scaffolded — their payload shapes are inlined here instead,
 * each annotated with its Rust source. */
import type { SetLayerVisible } from "../../../🏷️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🦠️mutation/🟦️.ts";
import type { SetLayerLocked } from "../../../🏷️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🦠️mutation/🟦️.ts";
import type { SetLayerOpacity } from "../../../🎨️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🦠️mutation/🟦️.ts";
import type { SetLayerBlendMode } from "../../../🎨️style/🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🦠️mutation/🟦️.ts";
import type { DuplicateLayer } from "../../../🧱️structure/🧬️schema/🧬️mutations/📋️duplicate-layer/🦠️mutation/🟦️.ts";
import type { ReorderLayer } from "../../../🧱️structure/🧬️schema/🧬️mutations/🔃reorder-layer/🦠️mutation/🟦️.ts";
import type { DrawingLayerNode } from "../🟦️.ts";

/** ✏️ Mirrors Rust `RenameLayer` (`✏️rename-layer/🦠️mutation/🦀️.rs`). */
export interface RenameLayer {
  layerId: string;
  newName: string;
}

/** 🔄️ Mirrors Rust `DrawingTransform` (`🗿️artifacts/🖍️drawing/🦀️.rs`). */
export interface DrawingTransform {
  x: number;
  y: number;
  scaleX: number;
  scaleY: number;
  rotation: number;
}

/** 🔄️ Mirrors Rust `UpdateLayerTransform` (`🔄️update-layer-transform/🦠️mutation/🦀️.rs`). */
export interface UpdateLayerTransform {
  layerId: string;
  transform: DrawingTransform;
}

/** 🎨️ Mirrors Rust `GradientStop` (`🗿️artifacts/🖍️drawing/🦀️.rs`). */
export interface GradientStop {
  offset: number;
  color: [number, number, number, number];
}

/** 🎨️ Mirrors Rust `FillStyle` (`🗿️artifacts/🖍️drawing/🦀️.rs`), tagged on `kind`. */
export type FillStyle =
  | { kind: "solid"; color: [number, number, number, number] }
  | { kind: "linearGradient"; x1: number; y1: number; x2: number; y2: number; stops: GradientStop[] }
  | { kind: "radialGradient"; cx: number; cy: number; r: number; stops: GradientStop[] };

/** 🔁 Mirrors Rust `ReplaceLayerFill` (`🎨️replace-layer-fill/🦠️mutation/🦀️.rs`). */
export interface ReplaceLayerFill {
  layerId: string;
  fill?: FillStyle;
}

/** ✏️ Mirrors Rust `StrokeStyle` (`🗿️artifacts/🖍️drawing/🦀️.rs`). */
export interface StrokeStyle {
  color: [number, number, number, number];
  width: number;
  cap: string;
  join: string;
  dash?: number[];
}

/** ♻️ Mirrors Rust `ReplaceLayerStroke` (`🖊️replace-layer-stroke/🦠️mutation/🦀️.rs`). */
export interface ReplaceLayerStroke {
  layerId: string;
  stroke?: StrokeStyle;
}

/** 🔀 Mirrors Rust `SetLayerBooleanOperation` (`🔀set-layer-boolean-operation/🦠️mutation/🦀️.rs`). */
export interface SetLayerBooleanOperation {
  layerId: string;
  booleanOperation: string;
}

/** 🔧 Mirrors Rust `DrawingTraceParams` (`🗿️artifacts/🖍️drawing/🦀️.rs`). */
export interface DrawingTraceParams {
  threshold: number;
  simplifyEpsilon: number;
}

/** 🔧 Mirrors Rust `UpdateLayerTraceParams` (`🔍️update-layer-trace-params/🦠️mutation/🦀️.rs`). */
export interface UpdateLayerTraceParams {
  layerId: string;
  params: DrawingTraceParams;
}

/** 🌱 Mirrors Rust `CreateLayer` (`➕️create-layer/🦠️mutation/🦀️.rs`). */
export interface CreateLayer {
  parentId?: string;
  index?: number;
  layer: DrawingLayerNode;
}

/** 🗑️ Mirrors Rust `DeleteLayer` (`🗑️delete-layer/🦠️mutation/🦀️.rs`). */
export interface DeleteLayer {
  layerId: string;
}

/** 🧩️ One arm per `DrawingMutation` variant, same declaration order as the Rust enum. */
export type DrawingMutation =
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
