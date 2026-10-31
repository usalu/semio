/** 🧍️ Public `@teaching/staffage` entry: explicit re-exports of the modules that own each member, nothing is defined here.
 * @see ../../README.md — the domain model and conventions
 * @see ../../🧬️schema/🔣️.json — the normative contract `semio.staffage/v2` */
export * from "../../🧬️schema/🟦️.ts";
export * from "../../🔨️modules/🔗️contracts/🟦️.ts";
export { ANNOTATION_PEN_MM, CSS_PX_PER_MM, SCALE_PRESETS, decimalsFor, formatNumber, formatQuantity, metresToMm, mmPerMetre, mmToMetres, paperMm, quantityParts, textSizeMm, viewportMm } from "../../🔨️modules/📐️units/🟦️.ts";
export type { QuantityParts, QuantityUnit } from "../../🔨️modules/📐️units/🟦️.ts";
export { solveRig } from "../../🔨️modules/🦴️rig/🟦️.ts";
export { clipOf, cycleOf, frameAt, poseOf, sampleClip, samplePose, transition, wrapTime } from "../../🔨️modules/🎞️animation/🟦️.ts";
export { evaluate, flattenGeometry, flattenPath, rigBounds } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
export { validateAsset, validateManifest } from "../../🔨️modules/🗂️catalog/🟦️.ts";
export type { AssetContext, Issue } from "../../🔨️modules/🗂️catalog/🟦️.ts";
export { QUANTITY_SPECS, areaWeighted as areaWeightedThermal, environmentAt, operativeTemperatureC, penmanMonteith, radiationSplit, simulateBody, solveThermal, solveThermalAt, uValue } from "../../🔨️modules/🌡️thermal/🟦️.ts";
export { freeBody, solveStructure, structuralLegend } from "../../🔨️modules/🏗️structure/🟦️.ts";
export type { FreeBodyForce, StructuralQuantity } from "../../🔨️modules/🏗️structure/🟦️.ts";
export { defaultDepictionOptions, depict, depictAll, modelScene, resolveDepiction } from "../../🔨️modules/🎨️depiction/🟦️.ts";
export { explanationBox } from "../../🔨️modules/🎨️depiction/🪧️explanation/🟦️.ts";
export type { ExplanationBox } from "../../🔨️modules/🎨️depiction/🪧️explanation/🟦️.ts";
export { areaWeighted, hatchOf, regionsOf } from "../../🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
export { DEFAULT_PRINT_PROFILE, PRINT_PROFILES, RepresentationError, compareRepresentations, measureSuitability, resolutionLabel, resolutionText, pickRepresentation, printProfile, requireDrawable, resolveRepresentation, resolveSymbol, supportedScales } from "../../🔨️modules/🖨️representation/🟦️.ts";
export type { Analysis } from "../../🔨️modules/🎨️depiction/🟦️.ts";
export { svgMarkup, writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
export { writePdf } from "../../🔨️modules/📤️export/📕️pdf/🟦️.ts";
export { writeDxf } from "../../🔨️modules/📤️export/📐️dxf/🟦️.ts";
export { DwgUnavailableError, configureDwg, dwgAvailability, writeDwg } from "../../🔨️modules/📤️export/🖊️dwg/🟦️.ts";
export { frameManifest, frameName, frameSchedule, writePngSequenceZip } from "../../🔨️modules/📤️export/🎞️frames/🟦️.ts";
export type { FrameManifestInput, FrameSchedule } from "../../🔨️modules/📤️export/🎞️frames/🟦️.ts";
export { encodeClip, videoCapabilities } from "../../🔨️modules/📤️export/🎥️video/🟦️.ts";
export type { CodecSupport, EncodeOptions, EncodedClip, VideoCapabilities, VideoFormat, VideoFrame } from "../../🔨️modules/📤️export/🎥️video/🟦️.ts";
