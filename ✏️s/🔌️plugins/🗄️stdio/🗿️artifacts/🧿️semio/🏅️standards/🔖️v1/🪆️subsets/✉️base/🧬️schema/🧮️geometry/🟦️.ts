import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";

export interface SemioPoint3 { x: number; y: number; z: number; }
export interface SemioPoint2 { x: number; y: number; }
export interface SemioUv { u: number; v: number; }
export interface SemioRgba { r: number; g: number; b: number; a: number; }
export interface SemioQuaternion { x: number; y: number; z: number; w: number; }
export interface SemioTransform { translation: SemioPoint3; rotation: SemioQuaternion; scale: SemioPoint3; }

function coordinates<K extends string>(value: unknown, keys: readonly K[], at: string): Record<K, number> {
  const row = parseSchemaRecord(value, keys, at);
  const result = {} as Record<K, number>;
  for (const key of keys) {
    const coordinate = row[key];
    if (typeof coordinate !== "number" || !Number.isFinite(coordinate)) throw new Error(at + "." + key + ": finite number required");
    result[key] = coordinate;
  }
  return result;
}

/** 📍️ Parses a three dimensional point. */
export function parseSemioPoint3(value: unknown, at = "$"): SemioPoint3 { return coordinates(value, ["x", "y", "z"], at); }
/** 📌️ Parses a two dimensional point. */
export function parseSemioPoint2(value: unknown, at = "$"): SemioPoint2 { return coordinates(value, ["x", "y"], at); }
/** 🧵️ Parses texture coordinates. */
export function parseSemioUv(value: unknown, at = "$"): SemioUv { return coordinates(value, ["u", "v"], at); }
/** 🎨️ Parses color components. */
export function parseSemioRgba(value: unknown, at = "$"): SemioRgba { return coordinates(value, ["r", "g", "b", "a"], at); }
/** 🧭️ Parses rotation coordinates. */
export function parseSemioQuaternion(value: unknown, at = "$"): SemioQuaternion { return coordinates(value, ["x", "y", "z", "w"], at); }
/** 📐️ Parses a placement from the shared coordinate contracts. */
export function parseSemioTransform(value: unknown, at = "$"): SemioTransform {
  const row = parseSchemaRecord(value, ["translation", "rotation", "scale"], at);
  return { translation: parseSemioPoint3(row.translation, at + ".translation"), rotation: parseSemioQuaternion(row.rotation, at + ".rotation"), scale: parseSemioPoint3(row.scale, at + ".scale") };
}
