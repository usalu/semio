/** 🧬 Transparent TypeScript aggregate for the mesh slice of the glTF 2.0 mutation vocabulary. */
import type { GltfBindMorphTargetAttributePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🧬️bind-morph-target-attribute/🟦️.ts';
import type { GltfBindPrimitiveAttributePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-attribute/🟦️.ts';
import type { GltfBindPrimitiveIndicesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-indices/🟦️.ts';
import type { GltfBindPrimitiveMaterialPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-material/🟦️.ts';
import type { GltfChangeMeshExtensionDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-extension-data/🟦️.ts';
import type { GltfChangeMeshExtraDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-extra-data/🟦️.ts';
import type { GltfChangeMeshMorphWeightsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-morph-weights/🟦️.ts';
import type { GltfChangeMeshNamePayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🕸️change-mesh-name/🟦️.ts';
import type { GltfChangePrimitiveExtensionDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-extension-data/🟦️.ts';
import type { GltfChangePrimitiveExtraDataPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-extra-data/🟦️.ts';
import type { GltfChangePrimitiveTopologyModePayload } from '../../../✳️any/🧬️schema/🧬️mutations/✏️🔺️change-primitive-topology-mode/🟦️.ts';
import type { GltfCreateAccessorPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️📐️create-accessor/🟦️.ts';
import type { GltfCreateMeshPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🕸️create-mesh/🟦️.ts';
import type { GltfCreateMorphTargetPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🧬️create-morph-target/🟦️.ts';
import type { GltfCreatePrimitivePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🌱️🔺️create-primitive/🟦️.ts';
import type { GltfDeleteAccessorPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️📐️delete-accessor/🟦️.ts';
import type { GltfDeleteMeshPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🕸️delete-mesh/🟦️.ts';
import type { GltfDeleteMorphTargetPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🧬️delete-morph-target/🟦️.ts';
import type { GltfDeletePrimitivePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🗑️🔺️delete-primitive/🟦️.ts';
import type { GltfMoveAccessorPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️📐️move-accessor/🟦️.ts';
import type { GltfMoveMeshPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🕸️move-mesh/🟦️.ts';
import type { GltfMoveMorphTargetAttributePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🧬️move-morph-target-attribute/🟦️.ts';
import type { GltfMoveMorphTargetPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🧬️move-morph-target/🟦️.ts';
import type { GltfMovePrimitiveAttributePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🔺️move-primitive-attribute/🟦️.ts';
import type { GltfMovePrimitivePayload } from '../../../✳️any/🧬️schema/🧬️mutations/🚚️🔺️move-primitive/🟦️.ts';
import type { GltfReorderAccessorsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️📐️reorder-accessors/🟦️.ts';
import type { GltfReorderMeshsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🕸️reorder-meshs/🟦️.ts';
import type { GltfReorderMorphTargetAttributesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🧬️reorder-morph-target-attributes/🟦️.ts';
import type { GltfReorderMorphTargetsPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🧬️reorder-morph-targets/🟦️.ts';
import type { GltfReorderPrimitiveAttributesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🔺️reorder-primitive-attributes/🟦️.ts';
import type { GltfReorderPrimitivesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/🔀️🔺️reorder-primitives/🟦️.ts';
import type { GltfUnbindMorphTargetAttributePayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🧬️unbind-morph-target-attribute/🟦️.ts';
import type { GltfUnbindPrimitiveAttributePayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-attribute/🟦️.ts';
import type { GltfUnbindPrimitiveIndicesPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-indices/🟦️.ts';
import type { GltfUnbindPrimitiveMaterialPayload } from '../../../✳️any/🧬️schema/🧬️mutations/✂️🔺️unbind-primitive-material/🟦️.ts';

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
