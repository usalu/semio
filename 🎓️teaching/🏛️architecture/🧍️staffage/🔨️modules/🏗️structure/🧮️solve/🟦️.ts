/** 🧮️ Solve one structural case document and write the result as JSON: the command a scene uses to get reactions, forces and moment diagrams for a model that is not a catalog asset (a slab strip, a beam of a lesson).
 * `repo tools exec bun -- 🟦️.ts --case file --out file`. The case is the language-agnostic case document of the solver tests (bones with masses and sections, support anchors, point loads); a uniform load is a uniform bone mass `kg = w·L/g`, a force is never a momentum.
 * The output holds the {@link structuralSummary} (N, N·m) and, per member, every diagram station (position, axial force, shear, bending moment in N·m, deflection).
 * @see ../🟦️.ts — the solver
 * @see ../🧾️summary/🟦️.ts — the summary
 * @see ../🧰️harness/🟦️.ts — `StructureCase`, `buildState`
 */
import { readFileSync, realpathSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { solveStructure } from "../🟦️.ts";
import { type StructureCase, buildState } from "../🧰️harness/🟦️.ts";
import { type StructuralSummary, structuralSummary } from "../🧾️summary/🟦️.ts";

/** 📈️ One station of a member diagram in N and N·m. */
export type SolvedStation = { readonly s: number; readonly x: number; readonly y: number; readonly axialN: number; readonly shearN: number; readonly momentNm: number };

/** 📦️ The result of {@link solveCase}. */
export type SolvedCase = { readonly summary: StructuralSummary; readonly members: Readonly<Record<string, { readonly stations: readonly SolvedStation[]; readonly deflection: readonly { readonly x: number; readonly y: number }[] | null }>> };

/** 🧮️ Solves a case document (its `oracle` is not needed). */
export function solveCase(document: Omit<StructureCase, "oracle">): SolvedCase {
  const result = solveStructure(buildState({ ...document, oracle: { tool: "none", version: "0", values: {} } } as StructureCase));
  return {
    summary: structuralSummary(result, false),
    members: Object.fromEntries(Object.entries(result.members).map(([bone, member]) => [bone, { stations: member.stations.map((station) => ({ s: station.s, x: station.at.x, y: station.at.y, axialN: station.axialN, shearN: station.shearN, momentNm: station.momentNm })), deflection: member.deflection?.map((point) => ({ x: point.x, y: point.y })) ?? null }])),
  };
}

const real = (path: string): string => {
  try {
    return realpathSync.native(path);
  } catch {
    return resolve(path);
  }
};

if (process.argv[1] !== undefined && real(fileURLToPath(import.meta.url)) === real(resolve(process.argv[1]))) {
  const args = process.argv.slice(2);
  const value = (key: string): string => {
    const at = args.indexOf(`--${key}`);
    if (at < 0 || args[at + 1] === undefined) throw new Error(`missing --${key}\nusage: --case <file> --out <file>`);
    return args[at + 1]!;
  };
  try {
    writeFileSync(resolve(value("out")), `${JSON.stringify(solveCase(JSON.parse(readFileSync(resolve(value("case")), "utf8"))))}\n`);
  } catch (error) {
    process.stderr.write(`${(error as Error).message}\n`);
    process.exitCode = 1;
  }
}
