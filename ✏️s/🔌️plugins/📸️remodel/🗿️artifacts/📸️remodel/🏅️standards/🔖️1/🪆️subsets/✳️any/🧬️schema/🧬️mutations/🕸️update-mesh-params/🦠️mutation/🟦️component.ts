/** ⚙️ update-mesh-params mutation payload — full-record replace of one `ReconstructionParams` sub-struct. */
export interface UpdateMeshParams {
  params: { tsdfVoxelSizeMm: number; tsdfTruncationMm: number; decimateTargetTriangles: number; smoothingIterations: number; textureEnabled: boolean; textureSize: number; guaranteeWatertight: boolean; holeFillMaxBoundaryVerts: number; selfIntersectionCheck: boolean; };
}
