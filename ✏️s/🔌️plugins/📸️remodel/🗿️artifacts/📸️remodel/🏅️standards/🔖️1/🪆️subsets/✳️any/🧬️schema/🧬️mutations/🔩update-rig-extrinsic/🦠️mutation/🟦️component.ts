/** 🔩 update-rig-extrinsic mutation payload — full-record replace of an existing rig pose. */
export interface UpdateRigExtrinsic {
  extrinsic: { cameraId: string; rotationWxyz: [number, number, number, number]; translationM: [number, number, number] };
}
