/** 🖥️ The owned, dependency-free model viewer for `semio.model/v1`: scene preparation, frames, orbit camera, picking, a WebGL2 backend, a software reference rasteriser and a framework-free controller / custom element. Domain-neutral; browser platform APIs only.
 * @see ./🧬️contract/🟦️.ts */
export { VERTEX_STRIDE, SKIN_STRIDE, MATERIAL_STRIDE } from "./🧬️contract/🟦️.ts";
export type { Bounds, Draw, Frame, Light, RenderBackend, RenderStyle, BackendStatus, Scene, SceneMaterial, ScenePrimitive, SceneSkin, V4, FramePose } from "./🧬️contract/🟦️.ts";
export { sceneOf, sceneOfAsync, buildScene, SceneError } from "./🧱️scene/🟦️.ts";
export { createFrame, frameOf, posedPositions, boundsOf, type FrameInput, type Lighting } from "./🎞️frame/🟦️.ts";
export { PRESETS, cameraOf, withPreset, basisOf, fitCamera, orbit, pan, zoom, visibleHeight, clipPlanes, viewMatrix, projectionMatrix, rayOf, type CameraState, type CameraPreset, type Projection, type Ray } from "./📷️camera/🟦️.ts";
export { pick, pickAt, solidOf, type Hit } from "./🎯️picking/🟦️.ts";
export { SoftwareBackend } from "./🖼️software/🟦️.ts";
export { WebGl2Backend, createWebGl2Backend } from "./🔶️webgl2/🟦️.ts";
export { ModelViewer, type ModelViewerOptions, type ViewerStrings, type ViewerStatus, type ViewerControls, type ViewerEvents, type BackendFactory } from "./🕹️controller/🟦️.ts";
export { parseCssColor } from "./🕹️controller/🎨️color/🟦️.ts";
export { createFollowState, followPoint, smoothFollow, rootIndexOf, removeRootTravel, type FollowInput, type FollowState, type RootMotion, type ViewerFollow } from "./🎥️follow/🟦️.ts";
export { defineModelViewerElement } from "./🧩️element/🟦️.ts";
