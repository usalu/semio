/** 📋️ Structural load cases: the EN 1991-1-1 imposed (occupancy) loads and material densities that scenes and assets draw their loads from, and the unit arithmetic that turns them into newtons.
 * Self-weight is mass times gravity at the centroid, `W = m·g`; a floor load acts on a tributary width as a line load, `w = q_k·b`; both are forces, never momentum.
 * @see ./🔣️.json — the data
 * @see ./🔣️.schema.json — the normative schema `semio.staffage.load-cases/v1`
 * @see ../🟦️.ts — the solver that places the resulting forces
 * @see ../../../🧬️schema/🟦️.ts — `Text`, `Slug`
 */
import type { Slug, Text } from "../../../🧬️schema/🟦️.ts";

/** 🆔️ The schema id of the load cases. */
export const LOAD_CASES_SCHEMA_ID = "semio.staffage.load-cases/v1";

/** 👥️ An imposed load of an occupancy category: characteristic uniform load in kN/m² and concentrated load in kN. */
export type OccupancyLoad = { readonly id: Slug; readonly category: string; readonly title: Text; readonly areaKNPerM2: number; readonly pointKN: number; readonly table: string };

/** 🧱️ A material with its weight density in kN/m³. */
export type MaterialDensity = { readonly id: Slug; readonly title: Text; readonly densityKNPerM3: number; readonly table: string };

/** 📋️ The load-case document. */
export type LoadCases = { readonly version: 1; readonly gravityMS2: number; readonly standard: { readonly id: Slug; readonly title: Text }; readonly occupancy: readonly OccupancyLoad[]; readonly materials: readonly MaterialDensity[] };

const SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const TABLE = /^[0-9A]\.[0-9]+$/u;
const isObject = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);
const positive = (value: unknown): boolean => typeof value === "number" && Number.isFinite(value) && value > 0;
const isText = (value: unknown): boolean => isObject(value) && typeof value.en === "string" && value.en.length > 0 && typeof value.de === "string" && value.de.length > 0;

/** 🧐️ Every way a document departs from the schema; empty when it is valid. */
export function loadCaseIssues(document: unknown): readonly string[] {
  if (!isObject(document)) return ["document: must be an object"];
  const issues: string[] = [];
  if (document.version !== 1) issues.push("version: must be 1");
  if (!positive(document.gravityMS2)) issues.push("gravityMS2: must be a positive number");
  const standard = document.standard;
  if (!isObject(standard) || typeof standard.id !== "string" || !SLUG.test(standard.id) || !isText(standard.title)) issues.push("standard: needs a slug id and a title with en and de");
  const group = (key: "occupancy" | "materials", quantity: string, extra: string | null): void => {
    const rows = document[key];
    if (!Array.isArray(rows) || rows.length === 0) return void issues.push(`${key}: must be a non-empty array`);
    const seen = new Set<string>();
    rows.forEach((row: unknown, index: number) => {
      const at = `${key}/${index}`;
      if (!isObject(row)) return void issues.push(`${at}: must be an object`);
      if (typeof row.id !== "string" || !SLUG.test(row.id)) issues.push(`${at}/id: must be a slug`);
      else if (seen.has(row.id)) issues.push(`${at}/id: duplicate ${row.id}`);
      else seen.add(row.id);
      if (!isText(row.title)) issues.push(`${at}/title: needs en and de`);
      if (!positive(row[quantity])) issues.push(`${at}/${quantity}: must be a positive number`);
      if (extra !== null && !positive(row[extra])) issues.push(`${at}/${extra}: must be a positive number`);
      if (typeof row.table !== "string" || !TABLE.test(row.table)) issues.push(`${at}/table: must name a table such as 6.2`);
      if (key === "occupancy" && !(typeof row.category === "string" && /^[A-H][0-9]?$/u.test(row.category))) issues.push(`${at}/category: must be a category such as C3`);
    });
  };
  group("occupancy", "areaKNPerM2", "pointKN");
  group("materials", "densityKNPerM3", null);
  return issues;
}

/** 📥️ Validates and returns the document; throws one error listing every issue. */
export function parseLoadCases(document: unknown): LoadCases {
  const issues = loadCaseIssues(document);
  if (issues.length > 0) throw new Error(`invalid load cases:\n${issues.join("\n")}`);
  return document as LoadCases;
}

/** 👥️ The occupancy load `id`; throws naming the known ids. */
export function occupancyOf(cases: LoadCases, id: Slug): OccupancyLoad {
  const found = cases.occupancy.find((load) => load.id === id);
  if (!found) throw new Error(`unknown occupancy load ${id}; known: ${cases.occupancy.map((load) => load.id).join(", ")}`);
  return found;
}

/** 🧱️ The material `id`; throws naming the known ids. */
export function materialOf(cases: LoadCases, id: Slug): MaterialDensity {
  const found = cases.materials.find((material) => material.id === id);
  if (!found) throw new Error(`unknown material ${id}; known: ${cases.materials.map((material) => material.id).join(", ")}`);
  return found;
}

/** ⚖️ The weight in newtons of a mass: `W = m·g`. */
export const weightN = (massKg: number, gravityMS2: number): number => massKg * gravityMS2;

/** 📏️ A floor load on a tributary width as a line load in N/m: `w = q_k·b`. */
export const lineLoadNPerM = (areaKNPerM2: number, tributaryWidthM: number): number => areaKNPerM2 * 1000 * tributaryWidthM;

/** 🏋️ The self-weight line load in N/m of a prismatic member: `w = γ·A`. */
export const selfWeightNPerM = (densityKNPerM3: number, areaM2: number): number => densityKNPerM3 * 1000 * areaM2;

/** 🧭️ The reactions in newtons of a simply supported span under a uniform line load `w` (N/m) and a point load `p` (N) at `a` metres from the left support, and the maximum sagging moment in N·m; the closed form the solver and anastruct are checked against. */
export function simpleBeam(spanM: number, wNPerM: number, pN: number, aM: number): { readonly leftN: number; readonly rightN: number; readonly maxMomentNm: number } {
  const leftN = (wNPerM * spanM) / 2 + (pN * (spanM - aM)) / spanM;
  const rightN = (wNPerM * spanM) / 2 + (pN * aM) / spanM;
  const moment = (x: number): number => leftN * x - (wNPerM * x * x) / 2 - (x > aM ? pN * (x - aM) : 0);
  const zeroShear = wNPerM > 0 ? [leftN / wNPerM, (leftN - pN) / wNPerM].filter((x) => x > 0 && x < spanM) : [];
  return { leftN, rightN, maxMomentNm: Math.max(...[aM, ...zeroShear].map(moment)) };
}

/** 🧭️ The wall reaction in newtons and the clamping moment in N·m (hogging, positive) of a cantilever of `lengthM` under a uniform line load `w` (N/m) and a tip point load `p` (N), both downward. */
export const cantilever = (lengthM: number, wNPerM: number, pN: number): { readonly reactionN: number; readonly momentNm: number } => ({ reactionN: wNPerM * lengthM + pN, momentNm: (wNPerM * lengthM * lengthM) / 2 + pN * lengthM });
