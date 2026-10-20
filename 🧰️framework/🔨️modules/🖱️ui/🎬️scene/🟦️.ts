// #region 🎬️Scene
/// <reference types="vitest/importMeta" />

export * from "./✂️text-splice/🟦️.ts";
export * from "./📏️world3d-modelling/🟦️.ts";

//#region ComponentSceneProtocol
/** 🖼️ A 2D canvas surface scene payload — mirrors the wasm `componentScene` node's `canvas2d` field. */
export type Canvas2dScene = {
  readonly cameraX: number;
  readonly cameraY: number;
  readonly zoom: number;
  readonly layersJson: string;
  readonly framing?: import("./📷️framing/🟦️.ts").Canvas2dFraming | null;
  /** ⏯️ The base64url `ToolRunTraceDelta` paged to this window — see {@link World3dScene.toolRunTrace}. */
  readonly toolRunTrace?: string | null;
  /** 🚚️ The spine's lane manifest — see {@link CANVAS2D_SCENE_LANES}. */
  readonly lanes?: readonly SceneLaneRef[];
};

export * from "./🧬️schema/🧩️block-list/🟦️.ts";
import type { SceneLaneRef } from "./🚚️lane-contract/🟦️.ts";
//#endregion ComponentSceneProtocol
// #endregion 🎬️Scene

export { fitCanvasFrame, type Canvas2dFraming, type Canvas2dFrameCamera } from "./📷️framing/🟦️.ts";
export { PAINT2D_SCENE_LANES, paint2dSceneFromLanes } from "./🛤️lanes/🟦️.ts";
export type { Paint2dScene, TableScene } from "./📄️documents/🟦️.ts";
export type { World3dScene } from "./🌐️world3d/🟦️.ts";
