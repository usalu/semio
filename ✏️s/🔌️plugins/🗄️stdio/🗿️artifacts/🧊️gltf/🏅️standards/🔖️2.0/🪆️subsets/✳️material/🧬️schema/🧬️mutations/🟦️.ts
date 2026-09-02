/** 🧬 Transparent TypeScript aggregate for the material slice of the glTF 2.0 mutation vocabulary. */
import type { GltfChangeMaterialAlphaModePayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️💎️change-material-alpha-mode/🟦️.ts';
import type { GltfChangeMaterialDoubleSidedPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️💎️change-material-double-sided/🟦️.ts';
import type { GltfCreateImagePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🖼️create-image/🟦️.ts';
import type { GltfCreateMaterialPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️💎️create-material/🟦️.ts';
import type { GltfCreateSamplerPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🎛️create-sampler/🟦️.ts';
import type { GltfCreateTexturePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🎨️create-texture/🟦️.ts';
import type { GltfDeleteImagePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🖼️delete-image/🟦️.ts';
import type { GltfDeleteMaterialPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️💎️delete-material/🟦️.ts';
import type { GltfDeleteSamplerPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🎛️delete-sampler/🟦️.ts';
import type { GltfDeleteTexturePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🎨️delete-texture/🟦️.ts';
import type { GltfMoveImagePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🖼️move-image/🟦️.ts';
import type { GltfMoveMaterialPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️💎️move-material/🟦️.ts';
import type { GltfMoveSamplerPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🎛️move-sampler/🟦️.ts';
import type { GltfMoveTexturePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🎨️move-texture/🟦️.ts';
import type { GltfReorderImagesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🖼️reorder-images/🟦️.ts';
import type { GltfReorderMaterialsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️💎️reorder-materials/🟦️.ts';
import type { GltfReorderSamplersPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🎛️reorder-samplers/🟦️.ts';
import type { GltfReorderTexturesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🎨️reorder-textures/🟦️.ts';

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
