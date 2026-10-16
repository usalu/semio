// #region 🧊️MeshWindowKit
/// <reference types="vitest/importMeta" />
/** 🧊️ `@semio-tech/plugin-window-kits` — TS twin of Rust `MeshWindowKit` (`framework.window.mesh`). */
import type { UiComponentSceneNode, World3dScene } from "@semio-tech/framework";

/** 🆔️ Frozen kind id — twin of Rust `MeshWindowKit::KIND_ID`. */
export const MESH_WINDOW_KIND_ID = "framework.window.mesh";

/** 🧊️ Reuses the existing `World3dScene` wire shape (camera/meshes/instances/selection JSON blobs) — twin of Rust `MeshView`. */
export type MeshView = {
  readonly cameraJson: string;
  readonly meshesJson: string;
  readonly instancesJson: string;
  readonly selectionJson: string;
};

/** 🧊️ Twin of Rust `MeshWindowKit::render` — builds a `world-3d` component scene from `view`. */
export function renderMesh(view: MeshView): UiComponentSceneNode {
  const scene: World3dScene = { cameraJson: view.cameraJson, meshesJson: view.meshesJson, instancesJson: view.instancesJson, selectionJson: view.selectionJson };
  const node: UiComponentSceneNode = { type: "componentScene", surfaceId: MESH_WINDOW_KIND_ID, controllerId: MESH_WINDOW_KIND_ID, componentKind: "world-3d", world3d: scene };
  return node;
}

// #endregion 🧊️MeshWindowKit
