/** 🧍️ Public `@teaching/staffage` entry: explicit re-exports of the modules that own each member, nothing is defined here.
 * @see ../../README.md — the domain model and conventions
 * @see ../../🧬️schema/🔣️.json — the normative contract `semio.staffage/v1` */
export * from "../../🧬️schema/🟦️.ts";
export * from "../../🔨️modules/🔗️contracts/🟦️.ts";
export { CSS_PX_PER_MM, SCALE_PRESETS, decimalsFor, formatNumber, formatQuantity, lineWeightMm, lodFor, metresToMm, mmPerMetre, mmToMetres, presetIndex, quantityParts, textSizeMm } from "../../🔨️modules/📐️units/🟦️.ts";
export type { QuantityParts, QuantityUnit } from "../../🔨️modules/📐️units/🟦️.ts";
export { solveRig } from "../../🔨️modules/🦴️rig/🟦️.ts";
export { clipOf, frameAt, poseOf, sampleClip, samplePose, transition, wrapTime } from "../../🔨️modules/🎞️animation/🟦️.ts";
export { evaluate, flattenGeometry, flattenPath } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
export { validateAsset, validateManifest } from "../../🔨️modules/🗂️catalog/🟦️.ts";
export type { Issue } from "../../🔨️modules/🗂️catalog/🟦️.ts";
export { solveThermal, thermalColour, thermalLegend } from "../../🔨️modules/🌡️thermal/🟦️.ts";
export { freeBody, solveStructure, structuralLegend } from "../../🔨️modules/🏗️structure/🟦️.ts";
export type { FreeBodyForce, StructuralQuantity } from "../../🔨️modules/🏗️structure/🟦️.ts";
export { defaultDepictionOptions, depict, depictAll, modelScene } from "../../🔨️modules/🎨️depiction/🟦️.ts";
export type { Analysis } from "../../🔨️modules/🎨️depiction/🟦️.ts";
export { svgMarkup, writeSvg } from "../../🔨️modules/📤️export/🖼️svg/🟦️.ts";
export { writePdf } from "../../🔨️modules/📤️export/📕️pdf/🟦️.ts";
export { writeDxf } from "../../🔨️modules/📤️export/📐️dxf/🟦️.ts";
