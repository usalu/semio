/** 🧬 Transparent TypeScript aggregate for the skin slice of the glTF 2.0 mutation vocabulary. */
import type { GltfCreateSkinPayload } from './🌱️🧥️create-skin/🟦️.ts';
import type { GltfDeleteSkinPayload } from './🗑️🧥️delete-skin/🟦️.ts';
import type { GltfMoveSkinPayload } from './🚚️🧥️move-skin/🟦️.ts';
import type { GltfReorderSkinsPayload } from './🔀️🧥️reorder-skins/🟦️.ts';

export type GltfSkinMutation =
  | { readonly mutation: 'createSkin'; readonly payload: GltfCreateSkinPayload }
  | { readonly mutation: 'reorderSkins'; readonly payload: GltfReorderSkinsPayload }
  | { readonly mutation: 'deleteSkin'; readonly payload: GltfDeleteSkinPayload }
  | { readonly mutation: 'moveSkin'; readonly payload: GltfMoveSkinPayload };
