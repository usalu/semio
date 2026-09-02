/** 🧬 Transparent TypeScript aggregate for the material slice of the glTF 2.0 mutation vocabulary. */
import type { GltfChangeMaterialAlphaModePayload } from './✏️💎️change-material-alpha-mode/🟦️.ts';
import type { GltfChangeMaterialDoubleSidedPayload } from './✏️💎️change-material-double-sided/🟦️.ts';
import type { GltfCreateImagePayload } from './🌱️🖼️create-image/🟦️.ts';
import type { GltfCreateMaterialPayload } from './🌱️💎️create-material/🟦️.ts';
import type { GltfCreateSamplerPayload } from './🌱️🎛️create-sampler/🟦️.ts';
import type { GltfCreateTexturePayload } from './🌱️🎨️create-texture/🟦️.ts';
import type { GltfDeleteImagePayload } from './🗑️🖼️delete-image/🟦️.ts';
import type { GltfDeleteMaterialPayload } from './🗑️💎️delete-material/🟦️.ts';
import type { GltfDeleteSamplerPayload } from './🗑️🎛️delete-sampler/🟦️.ts';
import type { GltfDeleteTexturePayload } from './🗑️🎨️delete-texture/🟦️.ts';
import type { GltfMoveImagePayload } from './🚚️🖼️move-image/🟦️.ts';
import type { GltfMoveMaterialPayload } from './🚚️💎️move-material/🟦️.ts';
import type { GltfMoveSamplerPayload } from './🚚️🎛️move-sampler/🟦️.ts';
import type { GltfMoveTexturePayload } from './🚚️🎨️move-texture/🟦️.ts';
import type { GltfReorderImagesPayload } from './🔀️🖼️reorder-images/🟦️.ts';
import type { GltfReorderMaterialsPayload } from './🔀️💎️reorder-materials/🟦️.ts';
import type { GltfReorderSamplersPayload } from './🔀️🎛️reorder-samplers/🟦️.ts';
import type { GltfReorderTexturesPayload } from './🔀️🎨️reorder-textures/🟦️.ts';

export type GltfMaterialMutation =
  | { readonly mutation: 'changeMaterialAlphaMode'; readonly payload: GltfChangeMaterialAlphaModePayload }
  | { readonly mutation: 'changeMaterialDoubleSided'; readonly payload: GltfChangeMaterialDoubleSidedPayload }
  | { readonly mutation: 'createSampler'; readonly payload: GltfCreateSamplerPayload }
  | { readonly mutation: 'createTexture'; readonly payload: GltfCreateTexturePayload }
  | { readonly mutation: 'createMaterial'; readonly payload: GltfCreateMaterialPayload }
  | { readonly mutation: 'createImage'; readonly payload: GltfCreateImagePayload }
  | { readonly mutation: 'reorderSamplers'; readonly payload: GltfReorderSamplersPayload }
  | { readonly mutation: 'reorderTextures'; readonly payload: GltfReorderTexturesPayload }
  | { readonly mutation: 'reorderMaterials'; readonly payload: GltfReorderMaterialsPayload }
  | { readonly mutation: 'reorderImages'; readonly payload: GltfReorderImagesPayload }
  | { readonly mutation: 'deleteSampler'; readonly payload: GltfDeleteSamplerPayload }
  | { readonly mutation: 'deleteTexture'; readonly payload: GltfDeleteTexturePayload }
  | { readonly mutation: 'deleteMaterial'; readonly payload: GltfDeleteMaterialPayload }
  | { readonly mutation: 'deleteImage'; readonly payload: GltfDeleteImagePayload }
  | { readonly mutation: 'moveSampler'; readonly payload: GltfMoveSamplerPayload }
  | { readonly mutation: 'moveTexture'; readonly payload: GltfMoveTexturePayload }
  | { readonly mutation: 'moveMaterial'; readonly payload: GltfMoveMaterialPayload }
  | { readonly mutation: 'moveImage'; readonly payload: GltfMoveImagePayload };
