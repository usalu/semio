/** 🗿️ `semio.model/v1`: the domain-neutral, engine-neutral 3D model kernel — schema twin, validation, tessellation, pose and animation evaluation, feature edges and the GLB and FBX encoders. No runtime dependencies. The browser viewer lives behind its own entry (`🖥️viewer/🟦️.ts`, package subpath `@semio-tech/framework-3d-js/model/viewer`) so that clients without a DOM never load it.
 * @see 🧬️schema/🔣️.json */
export type * from "./🧬️schema/🟦️.ts";
export { CancelledError, Pace, silentOperation } from "./⏱️operation/🟦️.ts";
export { validateModel } from "./✅️validation/🟦️.ts";
export { tessellate, tessellateSolid, restWorldMatrices, ModelInvalidError } from "./🔺️tessellation/🟦️.ts";
export { poseModel, sampleMaterials, type PoseSelector, type PoseResult, type LocalTransform } from "./🦴️mechanism/🟦️.ts";
export { featureEdges } from "./📏️edges/🟦️.ts";
export { encodeGlb, type GlbOptions } from "./📤️export/🧊️glb/🟦️.ts";
export { encodeFbx, encodeFbxDetailed, ownedCompressor, FbxCancelledError, FbxEncodeError, type Compressor, type FbxOptions, type FbxResult, type Notice } from "./📤️export/🎬️fbx/🟦️.ts";
export { compose, multiply4, invert4, qMul, qSlerp, qRotate, transformPoint, type M4, type Q4, type V3 } from "./🧮️math/🟦️.ts";
