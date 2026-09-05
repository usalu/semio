/** 🧬 Transparent TypeScript aggregate for the skin slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateSkinPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🦴️skin/🌱️create/🟦️.ts';
import type { GltfDeleteSkinPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🦴️skin/🗑️delete/🟦️.ts';
import type { GltfMoveSkinPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🦴️skin/🚚️move/🟦️.ts';
import type { GltfReorderSkinsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🦴️skin/🔀️reorder/🟦️.ts';

export type GltfSkinMutation =
  | { readonly mutation: 'createSkin'; readonly payload: GltfCreateSkinPayload }
  | { readonly mutation: 'reorderSkins'; readonly payload: GltfReorderSkinsPayload }
  | { readonly mutation: 'deleteSkin'; readonly payload: GltfDeleteSkinPayload }
  | { readonly mutation: 'moveSkin'; readonly payload: GltfMoveSkinPayload };
