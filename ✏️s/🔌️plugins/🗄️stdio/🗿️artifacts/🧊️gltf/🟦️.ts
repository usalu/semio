/** 🗄️ stdio.gltf TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export {parseGltfSnapshot,parseGltfJson} from "./🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts";
export type {GltfSnapshot,GltfDocument,GltfSourceForm,GltfJson,GltfAsset,GltfScene,GltfNode,GltfMesh,GltfPrimitive,GltfMorphTarget,GltfAccessor,GltfAccessorType,GltfComponentType,GltfBuffer,GltfBufferView,GltfSparseAccessor,GltfSparseIndices,GltfSparseValues,GltfMaterial,GltfPbrMetallicRoughness,GltfTextureInfo,GltfNormalTextureInfo,GltfOcclusionTextureInfo,GltfTexture,GltfImage,GltfSampler,GltfSkin,GltfAnimation,GltfAnimationSampler,GltfAnimationChannel,GltfAnimationChannelTarget,GltfCamera,GltfCameraProjection,GltfOrthographic,GltfPerspective} from "./🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts";
export {GLTF_SQLITE_SCHEMA,projectGltfSnapshotSqlite,reconstructGltfSnapshotSqlite,validateGltfSnapshotSqliteDialect} from "./🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
export {parseGltfDiff} from "./🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/🟦️.ts";
export type {GltfDiff} from "./🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/🟦️.ts";
export {parseGltfMutation} from "./🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🟦️.ts";
export type {GltfMutation} from "./🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🟦️.ts";
