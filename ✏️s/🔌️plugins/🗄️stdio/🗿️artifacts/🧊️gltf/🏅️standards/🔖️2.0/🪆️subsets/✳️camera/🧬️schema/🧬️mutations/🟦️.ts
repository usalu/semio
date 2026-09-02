/** 🧬 Transparent TypeScript aggregate for the camera slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateCameraPayload } from './🌱️🎥️create-camera/🟦️.ts';
import type { GltfDeleteCameraPayload } from './🗑️🎥️delete-camera/🟦️.ts';
import type { GltfMoveCameraPayload } from './🚚️🎥️move-camera/🟦️.ts';
import type { GltfReorderCamerasPayload } from './🔀️🎥️reorder-cameras/🟦️.ts';

export type GltfCameraMutation =
  | { readonly mutation: 'createCamera'; readonly payload: GltfCreateCameraPayload }
  | { readonly mutation: 'reorderCameras'; readonly payload: GltfReorderCamerasPayload }
  | { readonly mutation: 'deleteCamera'; readonly payload: GltfDeleteCameraPayload }
  | { readonly mutation: 'moveCamera'; readonly payload: GltfMoveCameraPayload };
