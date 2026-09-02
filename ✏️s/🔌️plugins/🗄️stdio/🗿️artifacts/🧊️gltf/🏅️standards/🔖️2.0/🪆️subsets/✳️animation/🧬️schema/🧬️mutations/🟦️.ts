/** 🧬 Transparent TypeScript aggregate for the animation slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateAnimationPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🎞️create-animation/🟦️.ts';
import type { GltfDeleteAnimationPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🎞️delete-animation/🟦️.ts';
import type { GltfMoveAnimationPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🎞️move-animation/🟦️.ts';
import type { GltfReorderAnimationsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🎞️reorder-animations/🟦️.ts';

export type GltfAnimationMutation =
  | { readonly mutation: 'createAnimation'; readonly payload: GltfCreateAnimationPayload }
  | { readonly mutation: 'reorderAnimations'; readonly payload: GltfReorderAnimationsPayload }
  | { readonly mutation: 'deleteAnimation'; readonly payload: GltfDeleteAnimationPayload }
  | { readonly mutation: 'moveAnimation'; readonly payload: GltfMoveAnimationPayload };
