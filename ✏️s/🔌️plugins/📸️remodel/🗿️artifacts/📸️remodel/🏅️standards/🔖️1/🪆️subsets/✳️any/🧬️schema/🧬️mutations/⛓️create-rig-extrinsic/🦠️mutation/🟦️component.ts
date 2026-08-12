/** ⛓ create-rig-extrinsic mutation payload — brings a new rig pose into existence. */
export interface CreateRigExtrinsic {
  extrinsic: RigExtrinsic;
}

export interface RigExtrinsic {
  cameraId: string;
  rotationWxyz: [number, number, number, number];
  translationM: [number, number, number];
}
