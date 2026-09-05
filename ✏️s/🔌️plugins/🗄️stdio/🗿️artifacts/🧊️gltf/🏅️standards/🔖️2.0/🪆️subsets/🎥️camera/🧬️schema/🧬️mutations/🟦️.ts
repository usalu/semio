/** 🧬 Transparent TypeScript aggregate for the camera slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateCameraPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎥️camera/🌱️create/🟦️.ts';
import type { GltfDeleteCameraPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎥️camera/🗑️delete/🟦️.ts';
import type { GltfMoveCameraPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎥️camera/🚚️move/🟦️.ts';
import type { GltfReorderCamerasPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎥️camera/🔀️reorder/🟦️.ts';

export type GltfCameraMutation =
  | { readonly mutation: 'createCamera'; readonly payload: GltfCreateCameraPayload }
  | { readonly mutation: 'reorderCameras'; readonly payload: GltfReorderCamerasPayload }
  | { readonly mutation: 'deleteCamera'; readonly payload: GltfDeleteCameraPayload }
  | { readonly mutation: 'moveCamera'; readonly payload: GltfMoveCameraPayload };
