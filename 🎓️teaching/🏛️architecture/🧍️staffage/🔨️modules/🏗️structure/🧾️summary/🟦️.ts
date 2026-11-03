/** 🧾️ Structural summary of one solved state: the numbers a scene needs to label what the forces and moments views draw, in newtons and newton-metres (display prefixes such as kN and kN·m are chosen by the caller with `formatQuantity`), at the world positions the cast frames use (x ahead, y up, z left; the elevation view draws x and y, the plan view x and z).
 * Compression is negative axial force and tension positive, as in the solver; a moment is a moment of force about an axis (N·m), never a momentum. `mirror` flips the figure about the vertical plane x = 0 the way the cast exporter does: x and the x components of forces change sign, and the y and z components of moments do too (a moment is a pseudovector).
 * @see ../🟦️.ts — the solver
 * @see ../../📤️export/🎭️cast/🟦️.ts — writes it next to the frames of a forces or moments cast
 * @see ../../📤️export/🎭️cast/🧬️schema/🔣️.schema.json — `$defs/Structure`
 */
import type { Slug } from "../../../🧬️schema/🟦️.ts";
import type { StructuralResult } from "../../🔗️contracts/🟦️.ts";
import { STRUCTURAL_COLORS } from "../🟦️.ts";

/** 📍️ A world position in metres. */
export type SummaryPoint = { readonly x: number; readonly y: number; readonly z: number };

/** 🧲️ A force in newtons and a couple in newton-metres at a world position. */
export type SummaryForce = { readonly at: SummaryPoint; readonly fxN: number; readonly fyN: number; readonly fzN: number; readonly mxNm: number; readonly myNm: number; readonly mzNm: number };

/** 🧷️ One support reaction. */
export type SummaryReaction = SummaryForce & { readonly anchor: Slug };

/** 🏹️ One applied load (an authored imposed, live or point load). */
export type SummaryLoad = SummaryForce & { readonly id: Slug; readonly kind: string };

/** 🪵️ The extreme internal actions of one member over its diagram stations. `atMaxMomentM` is where the largest |M| acts. */
export type SummaryMember = { readonly bone: Slug; readonly maxCompressionN: number; readonly maxTensionN: number; readonly maxShearN: number; readonly maxSaggingNm: number; readonly maxHoggingNm: number; readonly atMaxMomentM: SummaryPoint };

/** 🧾️ The summary of one frame. */
export type StructuralSummary = {
  readonly weightN: number;
  readonly centreOfMassM: SummaryPoint;
  readonly loads: readonly SummaryLoad[];
  readonly reactions: readonly SummaryReaction[];
  readonly members: readonly SummaryMember[];
  readonly residual: Omit<SummaryForce, "at">;
};

/** 🧾️ The summary of a cast: its units, the compression and tension colour ramp, and one summary per frame in frame order. */
export type StructureIndex = { readonly units: { readonly force: "N"; readonly moment: "N·m" }; readonly colors: typeof STRUCTURAL_COLORS; readonly frames: readonly StructuralSummary[] };

const round = (value: number, digits = 6): number => {
  const rounded = Math.round(value * 10 ** digits) / 10 ** digits;
  return rounded === 0 ? 0 : rounded;
};

/** 🧮️ Summarises a structural result; the weight is the sum of the segment weights, positive downward. */
export function structuralSummary(result: StructuralResult, mirror: boolean): StructuralSummary {
  const sx = mirror ? -1 : 1;
  const point = (p: { readonly x: number; readonly y: number }, z: number): SummaryPoint => ({ x: round(sx * p.x), y: round(p.y), z: round(z) });
  const force = (f: { readonly fxN: number; readonly fyN: number; readonly fzN: number; readonly mxNm: number; readonly myNm: number; readonly mzNm: number }, digits = 6): Omit<SummaryForce, "at"> => ({ fxN: round(sx * f.fxN, digits), fyN: round(f.fyN, digits), fzN: round(f.fzN, digits), mxNm: round(f.mxNm, digits), myNm: round(sx * f.myNm, digits), mzNm: round(sx * f.mzNm, digits) });
  const members = Object.values(result.members).map((member): SummaryMember => {
    const stations = member.stations;
    const peak = stations.reduce((best, station) => (Math.abs(station.momentNm) > Math.abs(best.momentNm) ? station : best), stations[0]!);
    return {
      bone: member.bone,
      maxCompressionN: round(-Math.min(0, ...stations.map((station) => station.axialN))),
      maxTensionN: round(Math.max(0, ...stations.map((station) => station.axialN))),
      maxShearN: round(Math.max(...stations.map((station) => Math.abs(station.shearN)))),
      maxSaggingNm: round(Math.max(0, ...stations.map((station) => station.momentNm))),
      maxHoggingNm: round(-Math.min(0, ...stations.map((station) => station.momentNm))),
      atMaxMomentM: point(peak.at, peak.z),
    };
  });
  return {
    weightN: round(-result.segmentWeights.reduce((sum, weight) => sum + weight.fyN, 0)),
    centreOfMassM: point(result.centreOfMass, result.centreOfMass.z),
    loads: result.applied.map((load) => ({ id: load.id, kind: load.kind, at: point(load.at, load.z), ...force(load) })),
    reactions: result.reactions.filter((reaction) => reaction.active).map((reaction) => ({ anchor: reaction.anchor, at: point(reaction.at, reaction.z), ...force(reaction) })),
    members,
    residual: force(result.residual, 9),
  };
}

/** 🗂️ The index block of a cast from the per-frame summaries. */
export const structureIndex = (frames: readonly StructuralSummary[]): StructureIndex => ({ units: { force: "N", moment: "N·m" }, colors: STRUCTURAL_COLORS, frames });
