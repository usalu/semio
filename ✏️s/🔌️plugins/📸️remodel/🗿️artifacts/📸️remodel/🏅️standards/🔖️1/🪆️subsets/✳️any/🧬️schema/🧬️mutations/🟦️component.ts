/** 🧬️ Remodel mutation vocabulary — tagged union of all 34 handcrafted semantic mutation kinds.
 *  Each member's fields mirror its Rust payload struct (see the per-triad `🦠️mutation/component.ts`
 *  mirrors under this same directory). */
export type RemodelMutationTag =
  | "create-stream"
  | "delete-stream"
  | "change-stream-sync"
  | "add-stream-frame"
  | "remove-stream-frame"
  | "replace-stream-source"
  | "create-asset"
  | "delete-asset"
  | "create-camera-calibration"
  | "update-camera-calibration"
  | "delete-camera-calibration"
  | "create-rig-extrinsic"
  | "delete-rig-extrinsic"
  | "update-rig-extrinsic"
  | "create-gcp"
  | "delete-gcp"
  | "add-gcp-observation"
  | "remove-gcp-observation"
  | "update-ingest-params"
  | "update-feature-params"
  | "update-match-params"
  | "update-sfm-params"
  | "update-dense-params"
  | "update-mesh-params"
  | "update-motion-params"
  | "update-geo-params"
  | "replace-job"
  | "replace-sparse"
  | "replace-dense"
  | "replace-mesh-result"
  | "replace-trajectory"
  | "replace-tracks"
  | "replace-geo-products"
  | "replace-qc";

export interface RemodelMutationEnvelope {
  mutation: RemodelMutationTag;
}
