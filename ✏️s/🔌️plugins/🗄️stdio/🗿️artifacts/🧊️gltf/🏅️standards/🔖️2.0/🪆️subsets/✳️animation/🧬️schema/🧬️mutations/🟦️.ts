/** 🧬 Transparent TypeScript aggregate for the animation slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateAnimationPayload } from './🌱️🎞️create-animation/🟦️.ts';
import type { GltfDeleteAnimationPayload } from './🗑️🎞️delete-animation/🟦️.ts';
import type { GltfMoveAnimationPayload } from './🚚️🎞️move-animation/🟦️.ts';
import type { GltfReorderAnimationsPayload } from './🔀️🎞️reorder-animations/🟦️.ts';

export type GltfAnimationMutation =
  | { readonly mutation: 'createAnimation'; readonly payload: GltfCreateAnimationPayload }
  | { readonly mutation: 'reorderAnimations'; readonly payload: GltfReorderAnimationsPayload }
  | { readonly mutation: 'deleteAnimation'; readonly payload: GltfDeleteAnimationPayload }
  | { readonly mutation: 'moveAnimation'; readonly payload: GltfMoveAnimationPayload };
