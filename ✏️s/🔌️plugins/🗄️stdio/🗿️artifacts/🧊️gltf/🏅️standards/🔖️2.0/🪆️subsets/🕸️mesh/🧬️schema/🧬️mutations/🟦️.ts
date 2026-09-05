/** 🧬 Transparent TypeScript aggregate for the mesh slice of the glTF 2.0 mutation vocabulary. */
import type { GltfBindMorphTargetAttributePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/🔗️bind/🟦️.ts';
import type { GltfBindPrimitiveAttributePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔤️primitive-attribute/🔗️bind/🟦️.ts';
import type { GltfBindPrimitiveIndicesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔢️primitive-indices/🔗️bind/🟦️.ts';
import type { GltfBindPrimitiveMaterialPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🧱️primitive-material/🔗️bind/🟦️.ts';
import type { GltfChangeMeshExtensionDataPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🕸️mesh/🧩️change-extensions/🟦️.ts';
import type { GltfChangeMeshExtraDataPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🕸️mesh/📝️change-extras/🟦️.ts';
import type { GltfChangeMeshMorphWeightsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🕸️mesh/⚖️change-weights/🟦️.ts';
import type { GltfChangeMeshNamePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🕸️mesh/🏷️rename/🟦️.ts';
import type { GltfChangePrimitiveExtensionDataPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔺️primitive/🧩️change-extensions/🟦️.ts';
import type { GltfChangePrimitiveExtraDataPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔺️primitive/📝️change-extras/🟦️.ts';
import type { GltfChangePrimitiveTopologyModePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔺️primitive/📐️change-topology/🟦️.ts';
import type { GltfCreateAccessorPayload } from '../../../♾️any/🧬️schema/🧬️mutations/📐️accessor/🌱️create/🟦️.ts';
import type { GltfCreateMeshPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🕸️mesh/🌱️create/🟦️.ts';
import type { GltfCreateMorphTargetPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🧬️morph-target/🌱️create/🟦️.ts';
import type { GltfCreatePrimitivePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔺️primitive/🌱️create/🟦️.ts';
import type { GltfDeleteAccessorPayload } from '../../../♾️any/🧬️schema/🧬️mutations/📐️accessor/🗑️delete/🟦️.ts';
import type { GltfDeleteMeshPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🕸️mesh/🗑️delete/🟦️.ts';
import type { GltfDeleteMorphTargetPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🧬️morph-target/🗑️delete/🟦️.ts';
import type { GltfDeletePrimitivePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔺️primitive/🗑️delete/🟦️.ts';
import type { GltfMoveAccessorPayload } from '../../../♾️any/🧬️schema/🧬️mutations/📐️accessor/🚚️move/🟦️.ts';
import type { GltfMoveMeshPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🕸️mesh/🚚️move/🟦️.ts';
import type { GltfMoveMorphTargetAttributePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/🚚️move/🟦️.ts';
import type { GltfMoveMorphTargetPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🧬️morph-target/🚚️move/🟦️.ts';
import type { GltfMovePrimitiveAttributePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔤️primitive-attribute/🚚️move/🟦️.ts';
import type { GltfMovePrimitivePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔺️primitive/🚚️move/🟦️.ts';
import type { GltfReorderAccessorsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/📐️accessor/🔀️reorder/🟦️.ts';
import type { GltfReorderMeshsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🕸️mesh/🔀️reorder/🟦️.ts';
import type { GltfReorderMorphTargetAttributesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/🔀️reorder/🟦️.ts';
import type { GltfReorderMorphTargetsPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🧬️morph-target/🔀️reorder/🟦️.ts';
import type { GltfReorderPrimitiveAttributesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔤️primitive-attribute/🔀️reorder/🟦️.ts';
import type { GltfReorderPrimitivesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔺️primitive/🔀️reorder/🟦️.ts';
import type { GltfUnbindMorphTargetAttributePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🎚️morph-attribute/✂️unbind/🟦️.ts';
import type { GltfUnbindPrimitiveAttributePayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔤️primitive-attribute/✂️unbind/🟦️.ts';
import type { GltfUnbindPrimitiveIndicesPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🔢️primitive-indices/✂️unbind/🟦️.ts';
import type { GltfUnbindPrimitiveMaterialPayload } from '../../../♾️any/🧬️schema/🧬️mutations/🧱️primitive-material/✂️unbind/🟦️.ts';

export type GltfMeshMutation =
  | { readonly mutation: 'unbindPrimitiveAttribute'; readonly payload: GltfUnbindPrimitiveAttributePayload }
  | { readonly mutation: 'unbindPrimitiveIndices'; readonly payload: GltfUnbindPrimitiveIndicesPayload }
  | { readonly mutation: 'unbindPrimitiveMaterial'; readonly payload: GltfUnbindPrimitiveMaterialPayload }
  | { readonly mutation: 'unbindMorphTargetAttribute'; readonly payload: GltfUnbindMorphTargetAttributePayload }
  | { readonly mutation: 'changePrimitiveExtensionData'; readonly payload: GltfChangePrimitiveExtensionDataPayload }
  | { readonly mutation: 'changePrimitiveExtraData'; readonly payload: GltfChangePrimitiveExtraDataPayload }
  | { readonly mutation: 'changePrimitiveTopologyMode'; readonly payload: GltfChangePrimitiveTopologyModePayload }
  | { readonly mutation: 'changeMeshExtensionData'; readonly payload: GltfChangeMeshExtensionDataPayload }
  | { readonly mutation: 'changeMeshExtraData'; readonly payload: GltfChangeMeshExtraDataPayload }
  | { readonly mutation: 'changeMeshMorphWeights'; readonly payload: GltfChangeMeshMorphWeightsPayload }
  | { readonly mutation: 'changeMeshName'; readonly payload: GltfChangeMeshNamePayload }
  | { readonly mutation: 'createAccessor'; readonly payload: GltfCreateAccessorPayload }
  | { readonly mutation: 'createPrimitive'; readonly payload: GltfCreatePrimitivePayload }
  | { readonly mutation: 'createMesh'; readonly payload: GltfCreateMeshPayload }
  | { readonly mutation: 'createMorphTarget'; readonly payload: GltfCreateMorphTargetPayload }
  | { readonly mutation: 'reorderAccessors'; readonly payload: GltfReorderAccessorsPayload }
  | { readonly mutation: 'reorderPrimitiveAttributes'; readonly payload: GltfReorderPrimitiveAttributesPayload }
  | { readonly mutation: 'reorderPrimitives'; readonly payload: GltfReorderPrimitivesPayload }
  | { readonly mutation: 'reorderMeshs'; readonly payload: GltfReorderMeshsPayload }
  | { readonly mutation: 'reorderMorphTargetAttributes'; readonly payload: GltfReorderMorphTargetAttributesPayload }
  | { readonly mutation: 'reorderMorphTargets'; readonly payload: GltfReorderMorphTargetsPayload }
  | { readonly mutation: 'bindPrimitiveAttribute'; readonly payload: GltfBindPrimitiveAttributePayload }
  | { readonly mutation: 'bindPrimitiveIndices'; readonly payload: GltfBindPrimitiveIndicesPayload }
  | { readonly mutation: 'bindPrimitiveMaterial'; readonly payload: GltfBindPrimitiveMaterialPayload }
  | { readonly mutation: 'bindMorphTargetAttribute'; readonly payload: GltfBindMorphTargetAttributePayload }
  | { readonly mutation: 'deleteAccessor'; readonly payload: GltfDeleteAccessorPayload }
  | { readonly mutation: 'deletePrimitive'; readonly payload: GltfDeletePrimitivePayload }
  | { readonly mutation: 'deleteMesh'; readonly payload: GltfDeleteMeshPayload }
  | { readonly mutation: 'deleteMorphTarget'; readonly payload: GltfDeleteMorphTargetPayload }
  | { readonly mutation: 'moveAccessor'; readonly payload: GltfMoveAccessorPayload }
  | { readonly mutation: 'movePrimitive'; readonly payload: GltfMovePrimitivePayload }
  | { readonly mutation: 'movePrimitiveAttribute'; readonly payload: GltfMovePrimitiveAttributePayload }
  | { readonly mutation: 'moveMesh'; readonly payload: GltfMoveMeshPayload }
  | { readonly mutation: 'moveMorphTarget'; readonly payload: GltfMoveMorphTargetPayload }
  | { readonly mutation: 'moveMorphTargetAttribute'; readonly payload: GltfMoveMorphTargetAttributePayload };
