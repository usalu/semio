/** ⚛️ Public `@teaching/staffage-react` entry: explicit re-exports of the members that own each surface, nothing is defined here.
 * @see ./📚️app/🟦️.tsx — the archive app the site mounts
 * @see ../../🔨️modules/🧍️figure/🟦️.tsx — the figure component */
export { StaffageFigure, StaffageValueTable } from "../../🔨️modules/🧍️figure/🟦️.tsx";
export type { StaffageFigureProps } from "../../🔨️modules/🧍️figure/🟦️.tsx";
export { StaffageClock, useClock } from "../../🔨️modules/⏱️clock/🟦️.ts";
export type { ClockOptions, ClockScheduler, ClockState } from "../../🔨️modules/⏱️clock/🟦️.ts";
export { STAFFAGE_ATTRIBUTES, defineStaffageElement } from "../../🔨️modules/🧩️element/🟦️.ts";
export type { StaffageCatalog, StaffageElementState } from "../../🔨️modules/🧩️element/🟦️.ts";
export { exportCapabilities, exportFigure, exportFilename, saveExport } from "../../🔨️modules/📤️downloads/🟦️.ts";
export type { ExportCapabilities, ExportDetails, ExportFormat, ExportHost, ExportOptions, ExportResult, FormatCapability } from "../../🔨️modules/📤️downloads/🟦️.ts";
export { evaluateState, figureFrame, frameOf, sceneExtent } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
export type { DepictionRequest, SceneFrame, StaffageFrame } from "../../🔨️modules/🧮️pipeline/🟦️.ts";
export { VIEW_NAMES, describeScene } from "../../🔨️modules/🗣️labels/🟦️.ts";
export { LANGUAGES, THERMAL_QUANTITIES, FORCE_QUANTITIES, VIEWS, SCALE_PRESETS } from "@teaching/staffage";
export type {
  Asset,
  CategoryId,
  DepictionOptions,
  DisplayRange,
  DrawingScale,
  DrawingScene,
  Environment,
  EvaluatedState,
  EvaluationQuery,
  ForceQuantity,
  ItemValue,
  JobControl,
  Language,
  ModelScene,
  StructuralResult,
  Text,
  ThermalField,
  ThermalQuantity,
  View,
  Viewpoint,
} from "@teaching/staffage";
