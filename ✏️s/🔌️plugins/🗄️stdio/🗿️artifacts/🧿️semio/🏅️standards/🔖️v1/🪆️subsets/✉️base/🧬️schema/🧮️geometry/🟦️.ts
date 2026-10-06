import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { type Binary64, type Binary32 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { parseBinary64, parseBinary32 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

export interface SemioPoint3 { x: Binary64; y: Binary64; z: Binary64; }
export interface SemioPoint2 { x: Binary64; y: Binary64; }
export interface SemioUv { u: Binary64; v: Binary64; }
export interface SemioRgba { r: Binary32; g: Binary32; b: Binary32; a: Binary32; }
export interface SemioQuaternion { x: Binary64; y: Binary64; z: Binary64; w: Binary64; }
export interface SemioTransform { translation: SemioPoint3; rotation: SemioQuaternion; scale: SemioPoint3; }

/** 📍️ Parses a three dimensional point. */
export function parseSemioPoint3(value: unknown, at = "$"): SemioPoint3 {const row=parseSchemaRecord(value,["x","y","z"],at);return{x:parseBinary64(row.x),y:parseBinary64(row.y),z:parseBinary64(row.z)};}
/** 📌️ Parses a two dimensional point. */
export function parseSemioPoint2(value: unknown, at = "$"): SemioPoint2 {const row=parseSchemaRecord(value,["x","y"],at);return{x:parseBinary64(row.x),y:parseBinary64(row.y)};}
/** 🧵️ Parses texture coordinates. */
export function parseSemioUv(value: unknown, at = "$"): SemioUv {const row=parseSchemaRecord(value,["u","v"],at);return{u:parseBinary64(row.u),v:parseBinary64(row.v)};}
/** 🎨️ Parses color components. */
export function parseSemioRgba(value: unknown, at = "$"): SemioRgba {const row=parseSchemaRecord(value,["r","g","b","a"],at);return{r:parseBinary32(row.r),g:parseBinary32(row.g),b:parseBinary32(row.b),a:parseBinary32(row.a)};}
/** 🧭️ Parses rotation coordinates. */
export function parseSemioQuaternion(value: unknown, at = "$"): SemioQuaternion {const row=parseSchemaRecord(value,["x","y","z","w"],at);return{x:parseBinary64(row.x),y:parseBinary64(row.y),z:parseBinary64(row.z),w:parseBinary64(row.w)};}
/** 📐️ Parses a placement from the shared coordinate contracts. */
export function parseSemioTransform(value: unknown, at = "$"): SemioTransform {
  const row = parseSchemaRecord(value, ["translation", "rotation", "scale"], at);
  return { translation: parseSemioPoint3(row.translation, at + ".translation"), rotation: parseSemioQuaternion(row.rotation, at + ".rotation"), scale: parseSemioPoint3(row.scale, at + ".scale") };
}
