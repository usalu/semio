/** 🧬️ Remodeling mutation vocabulary — tagged union of 34 of `RemodelingMutation`'s 35 handcrafted
 *  semantic mutation kinds (`CommitReconstruction` has no TS triad leaf yet, so it is not listed
 *  here). Each member's fields mirror its Rust payload struct (see the per-triad
 *  `🦠️mutation/component.ts` mirrors under this same directory). `RemodelingMutation` carries
 *  `#[serde(tag = "mutation", rename_all = "camelCase")]`, so the tag values are the camelCase form
 *  of the Rust variant names (confirmed by the `update-feature-params` fixture:
 *  `{"mutation":"updateFeatureParams", ...}`), NOT the kebab-case `kind` slugs used for this
 *  directory's per-leaf folder names. */
export type RemodelingMutationTag =
  | "createStream"
  | "deleteStream"
  | "changeStreamSync"
  | "addStreamFrame"
  | "removeStreamFrame"
  | "replaceStreamSource"
  | "createAsset"
  | "deleteAsset"
  | "createCameraCalibration"
  | "updateCameraCalibration"
  | "deleteCameraCalibration"
  | "createRigExtrinsic"
  | "deleteRigExtrinsic"
  | "updateRigExtrinsic"
  | "createGcp"
  | "deleteGcp"
  | "addGcpObservation"
  | "removeGcpObservation"
  | "updateIngestParams"
  | "updateFeatureParams"
  | "updateMatchParams"
  | "updateSfmParams"
  | "updateDenseParams"
  | "updateMeshParams"
  | "updateMotionParams"
  | "updateGeoParams"
  | "replaceJob"
  | "replaceSparse"
  | "replaceDense"
  | "replaceMeshResult"
  | "replaceTrajectory"
  | "replaceTracks"
  | "replaceGeoProducts"
  | "replaceQc";

export interface RemodelingMutationEnvelope {
  mutation: RemodelingMutationTag;
}
