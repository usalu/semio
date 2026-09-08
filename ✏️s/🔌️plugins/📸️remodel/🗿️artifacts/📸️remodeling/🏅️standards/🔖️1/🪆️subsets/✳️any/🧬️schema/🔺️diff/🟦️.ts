/** 🔺️ Remodeling diff schema — TypeScript twin of `🔺️diff/🦀️.rs` plus its hand-written
 *  `MutationDiff<RemodelingSnapshot>` apply/absorb (`🔺️diff/📝️text/🦀️.rs:79`).
 *
 *  `RemodelingDiff` carries `#[serde(rename_all = "camelCase", default)]` and none of its eighteen
 *  `Option` fields has `skip_serializing_if`, so an untouched field is written as an explicit
 *  `null` — the encoder here does the same. `streams`/`gcps` travel wrapped in the two list
 *  wrappers so an optional list stays a scalar across every format.
 */

import {
  CALIBRATION_STATE_SPEC,
  DURABLE_ARTIFACT_SPEC,
  ARTIFACT_CHILD_SPEC,
  GROUND_CONTROL_POINT_SPEC,
  MEDIA_STREAM_SPEC,
  RECONSTRUCTION_JOB_SPEC,
  RECONSTRUCTION_PARAMS_SPEC,
  RECONSTRUCTION_RESULTS_SPEC,
  decodeRecord,
  defaultsOf,
  writeRecordJson,
  type CalibrationState,
  type GroundControlPoint,
  type MediaStream,
  type ReconstructionJob,
  type ReconstructionParams,
  type ReconstructionResults,
  type RecordSpec,
  type RemodelingAssetChild,
  type RemodelingDurableArtifactStore,
  type RemodelingSnapshot,
  type ValueSpec,
} from "../📸️snapshot/🟦️.ts";
import {
  REMODELING_ARTIFACT_SPEC,
  REMODELING_UI_CAMERA_SPEC,
  REMODELING_UI_FRAME_CURSOR_SPEC,
  REMODELING_UI_LAYERS_SPEC,
  REMODELING_UI_SELECTION_SPEC,
  remodelingArtifactToSnapshot,
  type RemodelingArtifact,
  type RemodelingUiCamera,
  type RemodelingUiFrameCursor,
  type RemodelingUiLayers,
  type RemodelingUiSelection,
} from "../🟦️.ts";

//#region 🔖️DeltaHelpers
/** 📋 Media-stream list wrapper so optional list diffs stay scalar across formats. */
export interface RemodelingMediaStreamList {
  values: MediaStream[];
}

/** 📋 GCP list wrapper so optional list diffs stay scalar across formats. */
export interface RemodelingGcpList {
  values: GroundControlPoint[];
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Diff
export interface RemodelingDiff {
  artifact: RemodelingArtifact | null;
  schema: string | null;
  id: string | null;
  streams: RemodelingMediaStreamList | null;
  assets: Record<string, RemodelingAssetChild> | null;
  durableArtifacts: RemodelingDurableArtifactStore | null;
  calibration: CalibrationState | null;
  params: ReconstructionParams | null;
  gcps: RemodelingGcpList | null;
  job: ReconstructionJob | null;
  results: ReconstructionResults | null;
  selection: RemodelingUiSelection | null;
  activeUtilityId: string | null;
  reportTable: string | null;
  frameCursor: RemodelingUiFrameCursor | null;
  camera: RemodelingUiCamera | null;
  layers: RemodelingUiLayers | null;
}
//#endregion 🔖️Diff

//#region 🔖️Spec
const f = (name: string, spec: ValueSpec, dflt: () => unknown): { name: string; spec: ValueSpec; dflt: () => unknown } => ({ name, spec, dflt });
const text = { k: "text" } as const;
const opt = (of: ValueSpec): ValueSpec => ({ k: "opt", of });
const list = (of: ValueSpec): ValueSpec => ({ k: "list", of });
const map = (of: ValueSpec): ValueSpec => ({ k: "map", of });
const rec = (of: () => RecordSpec): ValueSpec => ({ k: "rec", of });
const nothing = () => null;

export const REMODELING_MEDIA_STREAM_LIST_SPEC: RecordSpec = {
  title: "RemodelingMediaStreamList",
  serdeDefault: true,
  fields: [f("values", list(rec(() => MEDIA_STREAM_SPEC)), () => [])],
};

export const REMODELING_GCP_LIST_SPEC: RecordSpec = {
  title: "RemodelingGcpList",
  serdeDefault: true,
  fields: [f("values", list(rec(() => GROUND_CONTROL_POINT_SPEC)), () => [])],
};

export const REMODELING_DIFF_SPEC: RecordSpec = {
  title: "RemodelingDiff",
  serdeDefault: true,
  fields: [
    f("artifact", opt(rec(() => REMODELING_ARTIFACT_SPEC)), nothing),
    f("schema", opt(text), nothing),
    f("id", opt(text), nothing),
    f("streams", opt(rec(() => REMODELING_MEDIA_STREAM_LIST_SPEC)), nothing),
    f("assets", opt(map(rec(() => ARTIFACT_CHILD_SPEC))), nothing),
    f("durable_artifacts", opt(map(rec(() => DURABLE_ARTIFACT_SPEC))), nothing),
    f("calibration", opt(rec(() => CALIBRATION_STATE_SPEC)), nothing),
    f("params", opt(rec(() => RECONSTRUCTION_PARAMS_SPEC)), nothing),
    f("gcps", opt(rec(() => REMODELING_GCP_LIST_SPEC)), nothing),
    f("job", opt(rec(() => RECONSTRUCTION_JOB_SPEC)), nothing),
    f("results", opt(rec(() => RECONSTRUCTION_RESULTS_SPEC)), nothing),
    f("selection", opt(rec(() => REMODELING_UI_SELECTION_SPEC)), nothing),
    f("active_utility_id", opt(text), nothing),
    f("report_table", opt(text), nothing),
    f("frame_cursor", opt(rec(() => REMODELING_UI_FRAME_CURSOR_SPEC)), nothing),
    f("camera", opt(rec(() => REMODELING_UI_CAMERA_SPEC)), nothing),
    f("layers", opt(rec(() => REMODELING_UI_LAYERS_SPEC)), nothing),
    f("locale", opt(text), nothing),
  ],
};
//#endregion 🔖️Spec

//#region 🔖️Codec
/** 🫙 `RemodelingDiff::default()` — every lane untouched. */
export const emptyRemodelingDiff = (): RemodelingDiff => defaultsOf(REMODELING_DIFF_SPEC) as unknown as RemodelingDiff;

/** 🔺️ Decodes a parsed RFC 8259 value into a validated `RemodelingDiff`. */
export const decodeRemodelingDiff = (json: unknown): RemodelingDiff => decodeRecord(json, REMODELING_DIFF_SPEC, "") as unknown as RemodelingDiff;

/** 📄️ Encodes a diff as `serde_json::to_string_pretty` would render it. */
export const remodelingDiffToJsonText = (diff: RemodelingDiff): string => writeRecordJson(diff as unknown as Record<string, unknown>, REMODELING_DIFF_SPEC, 0);

/** 📄️ Encodes a diff into a plain JSON value. */
export const encodeRemodelingDiff = (diff: RemodelingDiff): unknown => JSON.parse(remodelingDiffToJsonText(diff));

/** 🔑 Persistent lanes the apply walks, paired with the snapshot field each writes. */
const PERSISTENT_LANES: readonly [keyof RemodelingDiff, keyof RemodelingSnapshot, boolean][] = [
  ["schema", "schema", false],
  ["id", "id", false],
  ["streams", "streams", true],
  ["assets", "assets", false],
  ["durableArtifacts", "durableArtifacts", false],
  ["calibration", "calibration", false],
  ["params", "params", false],
  ["gcps", "gcps", true],
  ["job", "job", false],
  ["results", "results", false],
];

/** 🩹 `MutationDiff::apply` — a whole-artifact replacement short-circuits, else lane by lane. */
export function applyRemodelingDiff(diff: RemodelingDiff, snapshot: RemodelingSnapshot): RemodelingSnapshot {
  if (diff.artifact !== null) return remodelingArtifactToSnapshot(diff.artifact);
  const next = structuredClone(snapshot) as unknown as Record<string, unknown>;
  for (const [lane, field, wrapped] of PERSISTENT_LANES) {
    const value = diff[lane];
    if (value === null || value === undefined) continue;
    next[field] = wrapped ? structuredClone((value as { values: unknown[] }).values) : structuredClone(value);
  }
  return next as unknown as RemodelingSnapshot;
}

/** 🧲 `MutationDiff::absorb` — later lanes win; a whole-artifact replacement wins outright. */
export function absorbRemodelingDiff(into: RemodelingDiff, other: RemodelingDiff): RemodelingDiff {
  if (other.artifact !== null) return structuredClone(other);
  const merged = structuredClone(into) as unknown as Record<string, unknown>;
  for (const [lane] of PERSISTENT_LANES) if (other[lane] !== null && other[lane] !== undefined) merged[lane] = structuredClone(other[lane]);
  return merged as unknown as RemodelingDiff;
}

/** 🖊️ The lanes a diff actually writes — the load-bearing shape assertion for a fixture. */
export const remodelingDiffLanes = (diff: RemodelingDiff): string[] =>
  REMODELING_DIFF_SPEC.fields.map((field) => field.name).filter((name) => {
    const key = name.split("_").map((part, index) => (index === 0 ? part : part.charAt(0).toUpperCase() + part.slice(1))).join("");
    return (diff as unknown as Record<string, unknown>)[key] !== null;
  });
//#endregion 🔖️Codec

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingDiffGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingDiffGuardRefusal(at, why);
};

type remodelRemodelingDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingDiffGuardReject(at, "value is not an object");
export const remodelRemodelingDiffGuardArray = (value: unknown, at: string, bounds: remodelRemodelingDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingDiffGuardString = (value: unknown, at: string, bounds: remodelRemodelingDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingDiffGuardReject(at, "value is not a boolean"));
export const remodelRemodelingDiffGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingDiffGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingDiffGuardNumber(value, at, bounds) : remodelRemodelingDiffGuardReject(at, "value is not an integer");
export const remodelRemodelingDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface RemodelingArtifact {
  readonly schema: string;
  readonly id: string;
  readonly streams: readonly MediaStream[];
  readonly assets: Readonly<Record<string, unknown>>;
  readonly durableArtifacts: Readonly<Record<string, unknown>>;
  readonly calibration: CalibrationState;
  readonly params: ReconstructionParams;
  readonly gcps: readonly GroundControlPoint[];
  readonly job: ReconstructionJob;
  readonly results: ReconstructionResults;
  readonly selection: RemodelingUiSelection;
  readonly activeUtilityId: string;
  readonly reportTable: string;
  readonly frameCursor: RemodelingUiFrameCursor;
  readonly camera: RemodelingUiCamera;
  readonly layers: RemodelingUiLayers;
  readonly
}
