/** 🧬️ The owned contract of the model viewer: GPU-ready scene buffers, the per-frame draw list and the render backend interface that WebGL2, a software rasteriser and any later backend (WebGPU) implement. Domain-neutral, engine-neutral, no runtime dependency.
 * @see ../../🧬️schema/🟦️.ts */
import type { Issue, ModelDocument, Operation, PrimitiveSolid, Slug } from "../../🧬️schema/🟦️.ts";
import type { PoseSelector } from "../../🦴️mechanism/🟦️.ts";
import type { V3 } from "../../🧮️math/🟦️.ts";

/** 📦️ Axis-aligned bounds. */
export type Bounds = { min: V3; max: V3 };
/** 🧊️ Floats per interleaved vertex: position(3), normal(3), colour rgba(4). */
export const VERTEX_STRIDE = 10;
/** 🦴️ Floats per skinning vertex: four joint indices, four weights. */
export const SKIN_STRIDE = 8;
/** 🎨️ Floats per material slot: base rgba(4), emissive rgb(3), unlit, metallic, roughness, doubleSided, blend. */
export const MATERIAL_STRIDE = 12;

/** 🔺️ One drawable range of the shared buffers, instanced once per node that references its mesh. */
export type ScenePrimitive = {
  readonly index: number;
  readonly node: Slug;
  readonly nodeIndex: number;
  readonly mesh: Slug;
  readonly primitive: number;
  readonly mode: "triangles" | "lines";
  readonly material: Slug;
  readonly materialIndex: number;
  readonly skin: number;
  readonly vertexStart: number;
  readonly vertexCount: number;
  readonly indexStart: number;
  readonly indexCount: number;
  readonly bounds: Bounds;
  readonly tags: readonly Slug[];
  readonly solids: readonly PrimitiveSolid[];
};
/** 🪢️ A skin: joint node indices and inverse bind matrices; `paletteOffset` is its first row in the joint palette. */
export type SceneSkin = { readonly id: Slug; readonly jointNodes: Int32Array; readonly inverseBind: Float32Array; readonly paletteOffset: number };
/** 🌈️ A material of the document with its source values. */
export type SceneMaterial = { readonly id: Slug; readonly baseColor: V4; readonly emissive: readonly [number, number, number]; readonly metallic: number; readonly roughness: number; readonly unlit: boolean; readonly doubleSided: boolean; readonly blend: boolean };
/** 📐️ Four numbers. */
export type V4 = readonly [number, number, number, number];

/** 🧱️ A resolved document prepared for drawing: shared interleaved vertices, global indices, primitives, skins, variants. Immutable after creation. */
export type Scene = {
  readonly document: ModelDocument;
  readonly nodeIds: readonly Slug[];
  readonly nodeIndex: ReadonlyMap<Slug, number>;
  readonly materials: readonly SceneMaterial[];
  readonly materialIndex: ReadonlyMap<Slug, number>;
  readonly variants: ReadonlyMap<Slug, Int32Array>;
  readonly vertices: Float32Array;
  readonly skinning: Float32Array | null;
  readonly indices: Uint32Array;
  readonly primitives: readonly ScenePrimitive[];
  readonly skins: readonly SceneSkin[];
  readonly jointCount: number;
};

/** 🎬️ One entry of the draw list. `pass`: 0 opaque triangles, 1 blended triangles back to front, 2 lines. `matrixOffset` indexes `Frame.matrices`; skinned draws point at the trailing identity. `paletteOffset` is the first palette row of the skin, `-1` when rigid. */
export type Draw = { primitive: number; matrixOffset: number; material: number; paletteOffset: number; pass: 0 | 1 | 2; depth: number };
/** 💡️ Light in world space: direction towards the light, colour, hemisphere ambient. */
export type Light = { direction: Float32Array; color: Float32Array; sky: Float32Array; ground: Float32Array };
/** 🎞️ Everything a backend needs to draw one picture; reused across frames. */
export type Frame = {
  view: Float32Array;
  projection: Float32Array;
  viewProjection: Float32Array;
  eye: Float32Array;
  matrices: Float32Array;
  palette: Float32Array;
  materials: Float32Array;
  materialMap: Int32Array;
  draws: Draw[];
  drawCount: number;
  light: Light;
  issues: Issue[];
};

/** 🖌️ Appearance of one render: straight (non-premultiplied) sRGB background rgba. */
export type RenderStyle = { readonly background: V4 };
/** 🔌️ Backend state changes the controller reports. */
export type BackendStatus = "ready" | "lost" | "restored";

/** 🖥️ A render backend: upload a scene once, render frames, read pixels back. `readPixels` returns RGBA bytes, top row first, in device pixels. */
export interface RenderBackend {
  readonly kind: string;
  load(scene: Scene): void;
  resize(cssWidth: number, cssHeight: number, pixelRatio: number): void;
  render(frame: Frame, style: RenderStyle, operation?: Operation): void;
  readPixels(): Uint8ClampedArray;
  dispose(): void;
}

/** 🎛️ The pose half of a frame request: a pose id, a clip at a time, or the rest pose. */
export type FramePose = PoseSelector;
