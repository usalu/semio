/** 🧬 Transparent TypeScript aggregate for the material slice of the glTF 2.0 mutation vocabulary. */
import type { GltfChangeMaterialAlphaModePayload } from '../../../♾️any/🧬️schema/🧬️mutations/💎️material/🌫️change-alpha/🟦️.ts';
import type { GltfChangeMaterialDoubleSidedPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💎️material/🪞️change-sides/🟦️.ts';
import type { GltfCreateImagePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🖼️image/🌱️create/🟦️.ts';
import type { GltfCreateMaterialPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💎️material/🌱️create/🟦️.ts';
import type { GltfCreateSamplerPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎛️sampler/🌱️create/🟦️.ts';
import type { GltfCreateTexturePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎨️texture/🌱️create/🟦️.ts';
import type { GltfDeleteImagePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🖼️image/🗑️delete/🟦️.ts';
import type { GltfDeleteMaterialPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💎️material/🗑️delete/🟦️.ts';
import type { GltfDeleteSamplerPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎛️sampler/🗑️delete/🟦️.ts';
import type { GltfDeleteTexturePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎨️texture/🗑️delete/🟦️.ts';
import type { GltfMoveImagePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🖼️image/🚚️move/🟦️.ts';
import type { GltfMoveMaterialPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💎️material/🚚️move/🟦️.ts';
import type { GltfMoveSamplerPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎛️sampler/🚚️move/🟦️.ts';
import type { GltfMoveTexturePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎨️texture/🚚️move/🟦️.ts';
import type { GltfReorderImagesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🖼️image/🔀️reorder/🟦️.ts';
import type { GltfReorderMaterialsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/💎️material/🔀️reorder/🟦️.ts';
import type { GltfReorderSamplersPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎛️sampler/🔀️reorder/🟦️.ts';
import type { GltfReorderTexturesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎨️texture/🔀️reorder/🟦️.ts';

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
