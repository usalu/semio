/** 🧬 Transparent TypeScript aggregate for the animation slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateAnimationPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎞️animation/🌱️create/🟦️.ts';
import type { GltfDeleteAnimationPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎞️animation/🗑️delete/🟦️.ts';
import type { GltfMoveAnimationPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎞️animation/🚚️move/🟦️.ts';
import type { GltfReorderAnimationsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎞️animation/🔀️reorder/🟦️.ts';

export type GltfAnimationMutation =
  | { readonly mutation: 'createAnimation'; readonly payload: GltfCreateAnimationPayload }
  | { readonly mutation: 'reorderAnimations'; readonly payload: GltfReorderAnimationsPayload }
  | { readonly mutation: 'deleteAnimation'; readonly payload: GltfDeleteAnimationPayload }
  | { readonly mutation: 'moveAnimation'; readonly payload: GltfMoveAnimationPayload };
