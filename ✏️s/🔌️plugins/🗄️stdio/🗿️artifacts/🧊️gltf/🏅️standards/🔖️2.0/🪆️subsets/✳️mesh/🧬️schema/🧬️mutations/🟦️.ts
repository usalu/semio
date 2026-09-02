/** 🧬 Transparent TypeScript aggregate for the mesh slice of the glTF 2.0 mutation vocabulary. */
import type { GltfBindMorphTargetAttributePayload } from './🔗️🧬️bind-morph-target-attribute/🟦️.ts';
import type { GltfBindPrimitiveAttributePayload } from './🔗️🔺️bind-primitive-attribute/🟦️.ts';
import type { GltfBindPrimitiveIndicesPayload } from './🔗️🔺️bind-primitive-indices/🟦️.ts';
import type { GltfBindPrimitiveMaterialPayload } from './🔗️🔺️bind-primitive-material/🟦️.ts';
import type { GltfChangeMeshExtensionDataPayload } from './✏️🕸️change-mesh-extension-data/🟦️.ts';
import type { GltfChangeMeshExtraDataPayload } from './✏️🕸️change-mesh-extra-data/🟦️.ts';
import type { GltfChangeMeshMorphWeightsPayload } from './✏️🕸️change-mesh-morph-weights/🟦️.ts';
import type { GltfChangeMeshNamePayload } from './✏️🕸️change-mesh-name/🟦️.ts';
import type { GltfChangePrimitiveExtensionDataPayload } from './✏️🔺️change-primitive-extension-data/🟦️.ts';
import type { GltfChangePrimitiveExtraDataPayload } from './✏️🔺️change-primitive-extra-data/🟦️.ts';
import type { GltfChangePrimitiveTopologyModePayload } from './✏️🔺️change-primitive-topology-mode/🟦️.ts';
import type { GltfCreateAccessorPayload } from './🌱️📐️create-accessor/🟦️.ts';
import type { GltfCreateMeshPayload } from './🌱️🕸️create-mesh/🟦️.ts';
import type { GltfCreateMorphTargetPayload } from './🌱️🧬️create-morph-target/🟦️.ts';
import type { GltfCreatePrimitivePayload } from './🌱️🔺️create-primitive/🟦️.ts';
import type { GltfDeleteAccessorPayload } from './🗑️📐️delete-accessor/🟦️.ts';
import type { GltfDeleteMeshPayload } from './🗑️🕸️delete-mesh/🟦️.ts';
import type { GltfDeleteMorphTargetPayload } from './🗑️🧬️delete-morph-target/🟦️.ts';
import type { GltfDeletePrimitivePayload } from './🗑️🔺️delete-primitive/🟦️.ts';
import type { GltfMoveAccessorPayload } from './🚚️📐️move-accessor/🟦️.ts';
import type { GltfMoveMeshPayload } from './🚚️🕸️move-mesh/🟦️.ts';
import type { GltfMoveMorphTargetAttributePayload } from './🚚️🧬️move-morph-target-attribute/🟦️.ts';
import type { GltfMoveMorphTargetPayload } from './🚚️🧬️move-morph-target/🟦️.ts';
import type { GltfMovePrimitiveAttributePayload } from './🚚️🔺️move-primitive-attribute/🟦️.ts';
import type { GltfMovePrimitivePayload } from './🚚️🔺️move-primitive/🟦️.ts';
import type { GltfReorderAccessorsPayload } from './🔀️📐️reorder-accessors/🟦️.ts';
import type { GltfReorderMeshsPayload } from './🔀️🕸️reorder-meshs/🟦️.ts';
import type { GltfReorderMorphTargetAttributesPayload } from './🔀️🧬️reorder-morph-target-attributes/🟦️.ts';
import type { GltfReorderMorphTargetsPayload } from './🔀️🧬️reorder-morph-targets/🟦️.ts';
import type { GltfReorderPrimitiveAttributesPayload } from './🔀️🔺️reorder-primitive-attributes/🟦️.ts';
import type { GltfReorderPrimitivesPayload } from './🔀️🔺️reorder-primitives/🟦️.ts';
import type { GltfUnbindMorphTargetAttributePayload } from './✂️🧬️unbind-morph-target-attribute/🟦️.ts';
import type { GltfUnbindPrimitiveAttributePayload } from './✂️🔺️unbind-primitive-attribute/🟦️.ts';
import type { GltfUnbindPrimitiveIndicesPayload } from './✂️🔺️unbind-primitive-indices/🟦️.ts';
import type { GltfUnbindPrimitiveMaterialPayload } from './✂️🔺️unbind-primitive-material/🟦️.ts';

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
