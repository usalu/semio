/** 🧬️ Canonical persisted Remodeling model and its declared JSON boundary. */
import * as snapshot from "./📸️snapshot/🟦️.ts";
export * from "./📸️snapshot/🟦️.ts";
export interface RemodelingArtifact extends snapshot.RemodelingSnapshot {}
export const REMODELING_ARTIFACT_SPEC=snapshot.REMODELING_SNAPSHOT_SPEC;
export const REMODELING_SNAPSHOT_FIELDS=["schema","id","streams","assets","durableArtifacts","calibration","params","gcps","results"] as const;
/** 📸️ Persist the nine actual native artifact fields. */
export const remodelingArtifactToSnapshot=(v:RemodelingArtifact):snapshot.RemodelingSnapshot=>({schema:v.schema,id:v.id,streams:v.streams,assets:v.assets,durableArtifacts:v.durableArtifacts,calibration:v.calibration,params:v.params,gcps:v.gcps,results:v.results});
/** 🧩️ Restore the artifact's actual persisted state. */
export const remodelingArtifactFromSnapshot=(v:snapshot.RemodelingSnapshot):RemodelingArtifact=>remodelingArtifactToSnapshot(v);
/** 🔣️ Admit only the explicitly declared file transport scalars. */
export const decodeRemodelingArtifact=(v:unknown):RemodelingArtifact=>snapshot.decodeRemodelingSnapshot(v);
/** 🛂️ Validate the canonical artifact without transport coercion. */
export const parseRemodelingArtifact=(v:unknown,at="$"):RemodelingArtifact=>snapshot.parseRemodelingSnapshot(v,at);
//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingArtifactGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingArtifactGuardRefusal(at, why);
};

type remodelRemodelingArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingArtifactGuardReject(at, "value is not an object");
export const remodelRemodelingArtifactGuardArray = (value: unknown, at: string, bounds: remodelRemodelingArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingArtifactGuardString = (value: unknown, at: string, bounds: remodelRemodelingArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingArtifactGuardReject(at, "value is not a boolean"));
export const remodelRemodelingArtifactGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingArtifactGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingArtifactGuardNumber(value, at, bounds) : remodelRemodelingArtifactGuardReject(at, "value is not an integer");
export const remodelRemodelingArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

/** 🧾️ Validate the owned ArtifactDialect record. */
export const parseArtifactDialect=(value:unknown,at="$"):snapshot.ArtifactDialect=>snapshot.decodeRecord(value,snapshot.ARTIFACT_DIALECT_SPEC,at,false) as unknown as snapshot.ArtifactDialect;
/** 🧾️ Validate the owned ArtifactRef record. */
export const parseArtifactRef=(value:unknown,at="$"):snapshot.ArtifactRef=>snapshot.decodeRecord(value,snapshot.ARTIFACT_REF_SPEC,at,false) as unknown as snapshot.ArtifactRef;
/** 🧾️ Validate the owned CalibrationState record. */
export const parseCalibrationState=(value:unknown,at="$"):snapshot.CalibrationState=>snapshot.decodeRecord(value,snapshot.CALIBRATION_STATE_SPEC,at,false) as unknown as snapshot.CalibrationState;
/** 🧾️ Validate the owned CameraPosePreview record. */
export const parseCameraPosePreview=(value:unknown,at="$"):snapshot.CameraPosePreview=>snapshot.decodeRecord(value,snapshot.CAMERA_POSE_PREVIEW_SPEC,at,false) as unknown as snapshot.CameraPosePreview;
/** 🧾️ Validate the owned CameraTrajectory record. */
export const parseCameraTrajectory=(value:unknown,at="$"):snapshot.CameraTrajectory=>snapshot.decodeRecord(value,snapshot.CAMERA_TRAJECTORY_SPEC,at,false) as unknown as snapshot.CameraTrajectory;
/** 🧾️ Validate the owned DenseCloud record. */
export const parseDenseCloud=(value:unknown,at="$"):snapshot.DenseCloud=>snapshot.decodeRecord(value,snapshot.DENSE_CLOUD_SPEC,at,false) as unknown as snapshot.DenseCloud;
/** 🧾️ Validate the owned DenseParams record. */
export const parseDenseParams=(value:unknown,at="$"):snapshot.DenseParams=>snapshot.decodeRecord(value,snapshot.DENSE_PARAMS_SPEC,at,false) as unknown as snapshot.DenseParams;
/** 🧾️ Validate the owned FeatureParams record. */
export const parseFeatureParams=(value:unknown,at="$"):snapshot.FeatureParams=>snapshot.decodeRecord(value,snapshot.FEATURE_PARAMS_SPEC,at,false) as unknown as snapshot.FeatureParams;
/** 🧾️ Validate the owned FrameRef record. */
export const parseFrameRef=(value:unknown,at="$"):snapshot.FrameRef=>snapshot.decodeRecord(value,snapshot.FRAME_REF_SPEC,at,false) as unknown as snapshot.FrameRef;
/** 🧾️ Validate the owned GcpObservation record. */
export const parseGcpObservation=(value:unknown,at="$"):snapshot.GcpObservation=>snapshot.decodeRecord(value,snapshot.GCP_OBSERVATION_SPEC,at,false) as unknown as snapshot.GcpObservation;
/** 🧾️ Validate the owned GeoParams record. */
export const parseGeoParams=(value:unknown,at="$"):snapshot.GeoParams=>snapshot.decodeRecord(value,snapshot.GEO_PARAMS_SPEC,at,false) as unknown as snapshot.GeoParams;
/** 🧾️ Validate the owned GeoProducts record. */
export const parseGeoProducts=(value:unknown,at="$"):snapshot.GeoProducts=>snapshot.decodeRecord(value,snapshot.GEO_PRODUCTS_SPEC,at,false) as unknown as snapshot.GeoProducts;
/** 🧾️ Validate the owned GroundControlPoint record. */
export const parseGroundControlPoint=(value:unknown,at="$"):snapshot.GroundControlPoint=>snapshot.decodeRecord(value,snapshot.GROUND_CONTROL_POINT_SPEC,at,false) as unknown as snapshot.GroundControlPoint;
/** 🧾️ Validate the owned ImageAsset record. */
export const parseImageAsset=(value:unknown,at="$"):snapshot.ImageAsset=>snapshot.decodeRecord(value,snapshot.IMAGE_ASSET_SPEC,at,false) as unknown as snapshot.ImageAsset;
/** 🧾️ Validate the owned IngestParams record. */
export const parseIngestParams=(value:unknown,at="$"):snapshot.IngestParams=>snapshot.decodeRecord(value,snapshot.INGEST_PARAMS_SPEC,at,false) as unknown as snapshot.IngestParams;
/** 🧾️ Validate the owned MatchParams record. */
export const parseMatchParams=(value:unknown,at="$"):snapshot.MatchParams=>snapshot.decodeRecord(value,snapshot.MATCH_PARAMS_SPEC,at,false) as unknown as snapshot.MatchParams;
/** 🧾️ Validate the owned MediaStream record. */
export const parseMediaStream=(value:unknown,at="$"):snapshot.MediaStream=>snapshot.decodeRecord(value,snapshot.MEDIA_STREAM_SPEC,at,false) as unknown as snapshot.MediaStream;
/** 🧾️ Validate the owned MeshParams record. */
export const parseMeshParams=(value:unknown,at="$"):snapshot.MeshParams=>snapshot.decodeRecord(value,snapshot.MESH_PARAMS_SPEC,at,false) as unknown as snapshot.MeshParams;
/** 🧾️ Validate the owned MotionParams record. */
export const parseMotionParams=(value:unknown,at="$"):snapshot.MotionParams=>snapshot.decodeRecord(value,snapshot.MOTION_PARAMS_SPEC,at,false) as unknown as snapshot.MotionParams;
/** 🧾️ Validate the owned MotionTrackSummary record. */
export const parseMotionTrackSummary=(value:unknown,at="$"):snapshot.MotionTrackSummary=>snapshot.decodeRecord(value,snapshot.MOTION_TRACK_SUMMARY_SPEC,at,false) as unknown as snapshot.MotionTrackSummary;
/** 🧾️ Validate the owned QcReportSnapshot record. */
export const parseQcReportSnapshot=(value:unknown,at="$"):snapshot.QcReportSnapshot=>snapshot.decodeRecord(value,snapshot.QC_REPORT_SPEC,at,false) as unknown as snapshot.QcReportSnapshot;
/** 🧾️ Validate the owned ReconstructionParams record. */
export const parseReconstructionParams=(value:unknown,at="$"):snapshot.ReconstructionParams=>snapshot.decodeRecord(value,snapshot.RECONSTRUCTION_PARAMS_SPEC,at,false) as unknown as snapshot.ReconstructionParams;
/** 🧾️ Validate the owned ReconstructionResults record. */
export const parseReconstructionResults=(value:unknown,at="$"):snapshot.ReconstructionResults=>snapshot.decodeRecord(value,snapshot.RECONSTRUCTION_RESULTS_SPEC,at,false) as unknown as snapshot.ReconstructionResults;
/** 🧾️ Validate the owned RemodelingDurableArtifact record. */
export const parseRemodelingDurableArtifact=(value:unknown,at="$"):snapshot.RemodelingDurableArtifact=>snapshot.decodeRecord(value,snapshot.DURABLE_ARTIFACT_SPEC,at,false) as unknown as snapshot.RemodelingDurableArtifact;
/** 🧾️ Validate the owned RemodelingMesh record. */
export const parseRemodelingMesh=(value:unknown,at="$"):snapshot.RemodelingMesh=>snapshot.decodeRecord(value,snapshot.REMODELING_MESH_SPEC,at,false) as unknown as snapshot.RemodelingMesh;
/** 🧾️ Validate the owned RigExtrinsic record. */
export const parseRigExtrinsic=(value:unknown,at="$"):snapshot.RigExtrinsic=>snapshot.decodeRecord(value,snapshot.RIG_EXTRINSIC_SPEC,at,false) as unknown as snapshot.RigExtrinsic;
/** 🧾️ Validate the owned SfmParams record. */
export const parseSfmParams=(value:unknown,at="$"):snapshot.SfmParams=>snapshot.decodeRecord(value,snapshot.SFM_PARAMS_SPEC,at,false) as unknown as snapshot.SfmParams;
/** 🧾️ Validate the owned SparseCloud record. */
export const parseSparseCloud=(value:unknown,at="$"):snapshot.SparseCloud=>snapshot.decodeRecord(value,snapshot.SPARSE_CLOUD_SPEC,at,false) as unknown as snapshot.SparseCloud;
/** 🧾️ Validate the owned VideoSource record. */
export const parseVideoSource=(value:unknown,at="$"):snapshot.VideoSource=>snapshot.decodeRecord(value,snapshot.VIDEO_SOURCE_SPEC,at,false) as unknown as snapshot.VideoSource;
/** 🧾️ Validate the owned WatertightReportSnapshot record. */
export const parseWatertightReportSnapshot=(value:unknown,at="$"):snapshot.WatertightReportSnapshot=>snapshot.decodeRecord(value,snapshot.WATERTIGHT_REPORT_SPEC,at,false) as unknown as snapshot.WatertightReportSnapshot;
/** 🧾️ Validate the owned CameraCalibration record. */
export const parseCameraCalibration=(value:unknown,at="$"):snapshot.CameraCalibration=>snapshot.decodeRecord(value,snapshot.CAMERA_CALIBRATION_SPEC,at,false) as unknown as snapshot.CameraCalibration;
