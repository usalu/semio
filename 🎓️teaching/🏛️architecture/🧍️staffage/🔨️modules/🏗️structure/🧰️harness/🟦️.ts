/** 🧰️ Test harness of the structural solver: builds a hand-made {@link EvaluatedState} from a language-agnostic case document
 * (`🧫️fixtures/<case>/🔣️.json`), so the solver is verified independently of the evaluation module.
 *
 * Forward kinematics here is the rest transform only (parent frame rotation + translation), root placed by `root`.
 *
 * @see ../🟦️.ts — the solver under test
 * @see ../../🔗️contracts/🟦️.ts — {@link EvaluatedState}
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Anchor, Asset, Bone, BoneOffset, Section, SegmentMass, Slug, SupportType } from "../../../🧬️schema/🟦️.ts";
import type { EvaluatedBone, EvaluatedState, Vec3, WorldForce } from "../../🔗️contracts/🟦️.ts";
import { rotationRad, solveChain } from "../../🦴️rig/🟦️.ts";

/** 🧱️ A bone of a case document. */
export type CaseBone = { readonly id: Slug; readonly parent: Slug | null; readonly x: number; readonly y: number; readonly angleDeg: number; readonly lengthM: number; readonly z?: number; readonly abductionDeg?: number; readonly twistDeg?: number; readonly joint?: "fixed" | "revolute"; readonly mass?: Pick<SegmentMass, "kg" | "comX" | "comY" | "distribution">; readonly section?: Omit<Section, "title"> };

/** 📍️ An anchor of a case document. */
export type CaseAnchor = { readonly id: Slug; readonly bone: Slug; readonly x: number; readonly y: number; readonly z?: number; readonly kind: Anchor["kind"]; readonly support?: SupportType; readonly normalDeg?: number; readonly tiltDeg?: number; readonly footprintLengthM?: number; readonly footprintWidthM?: number; readonly frictionCoefficient?: number };

/** 🧲️ A world load of a case document. */
export type CaseLoad = { readonly id: Slug; readonly bone: Slug; readonly at: { readonly x: number; readonly y: number; readonly z?: number }; readonly fxN: number; readonly fyN: number; readonly fzN?: number; readonly momentXNm?: number; readonly momentYNm?: number; readonly momentNm?: number };

/** 🎬️ What one scenario overrides on the case model. */
export type CaseScenario = { readonly id: string; readonly loads?: readonly CaseLoad[]; readonly contacts?: readonly Slug[]; readonly dynamics?: "quasi-static" | "inertial"; readonly accelerations?: Readonly<Record<Slug, { readonly x: number; readonly y: number; readonly z?: number }>>; readonly bones?: Readonly<Record<Slug, Partial<Pick<CaseBone, "angleDeg" | "x" | "y" | "z" | "abductionDeg" | "twistDeg">>>> };

/** 🧪️ A case document: model, scenarios and the third-party oracle values. */
export type StructureCase = {
  readonly case: string;
  readonly oracle: { readonly tool: string; readonly version: string; readonly values: Readonly<Record<string, unknown>> };
  readonly gravityMS2: number;
  readonly dynamics: "quasi-static" | "inertial";
  readonly kind: "anatomical" | "structural";
  readonly materials: readonly { readonly id: Slug; readonly elasticModulusPa: number }[];
  readonly root: { readonly x: number; readonly y: number; readonly angleDeg: number };
  readonly bones: readonly CaseBone[];
  readonly anchors: readonly CaseAnchor[];
  readonly contacts: readonly Slug[];
  readonly supports: readonly Slug[];
  readonly loads: readonly CaseLoad[];
  readonly scenarios: readonly CaseScenario[];
};

/** 📂️ Reads `🧫️fixtures/<name>/🔣️.json`. */
export function loadCase(name: string): StructureCase {
  const here = dirname(fileURLToPath(import.meta.url));
  return JSON.parse(readFileSync(resolve(here, "../../../🧫️fixtures", name, "🔣️.json"), "utf8")) as StructureCase;
}

const label = (id: string) => ({ en: id, de: id });

/** 🧱️ Builds the evaluated state of a case, optionally under one scenario's overrides. Bone frames come from the rig chain solver (rest angle, `abductionDeg`, `twistDeg`, lateral `z`), so a case can slew, spread or offset a member laterally.  */
export function buildState(doc: StructureCase, scenario?: CaseScenario): EvaluatedState {
  const dynamics = scenario?.dynamics ?? doc.dynamics;
  const offsets: Record<Slug, BoneOffset> = {};
  const bones: Bone[] = doc.bones.map((b) => {
    const o = scenario?.bones?.[b.id] ?? {};
    const abductionDeg = o.abductionDeg ?? b.abductionDeg ?? 0;
    const twistDeg = o.twistDeg ?? b.twistDeg ?? 0;
    if (abductionDeg !== 0 || twistDeg !== 0) offsets[b.id] = { abductionDeg, twistDeg };
    return {
      id: b.id,
      parent: b.parent,
      title: label(b.id),
      x: o.x ?? b.x,
      y: o.y ?? b.y,
      ...((o.z ?? b.z) === undefined ? {} : { z: o.z ?? b.z }),
      angleDeg: b.parent === null ? 0 : (o.angleDeg ?? b.angleDeg),
      lengthM: b.lengthM,
      joint: abductionDeg !== 0 || twistDeg !== 0 ? { type: "revolute" } : { type: b.joint ?? "fixed" },
      ...(b.mass ? { mass: { ...b.mass, source: "illustrative" as const } } : {}),
      ...(b.section ? { section: b.section } : {}),
    } as Bone;
  });
  const rig = { kind: doc.kind, root: bones[0]!.id, bones } as Asset["rig"];
  const solved = solveChain(rig, offsets, doc.root);
  const evaluated: Record<Slug, EvaluatedBone> = {};
  const three = (v: readonly [number, number, number]): Vec3 => ({ x: v[0], y: v[1], z: v[2] });
  for (const b of bones) {
    const { frame, affine } = solved[b.id]!;
    const tip: Vec3 = { x: frame.o[0] + frame.u[0] * b.lengthM, y: frame.o[1] + frame.u[1] * b.lengthM, z: frame.o[2] + frame.u[2] * b.lengthM };
    evaluated[b.id] = { id: b.id, world: affine, origin: { x: frame.o[0], y: frame.o[1] }, tip, frame: { o: three(frame.o), u: three(frame.u), v: three(frame.v), w: three(frame.w) }, angleRad: rotationRad(affine), offset: { angleDeg: 0, abductionDeg: offsets[b.id]?.abductionDeg ?? 0, twistDeg: offsets[b.id]?.twistDeg ?? 0, dx: 0, dy: 0 } };
  }
  const anchors: Anchor[] = doc.anchors.map((a) => ({ ...a, title: label(a.id) }) as Anchor);
  const at = (a: Anchor): { readonly x: number; readonly y: number; readonly z: number } => {
    const f = evaluated[a.bone]!.frame;
    const z = a.z ?? 0;
    return { x: f.o.x + f.u.x * a.x + f.v.x * a.y + f.w.x * z, y: f.o.y + f.u.y * a.x + f.v.y * a.y + f.w.y * z, z: f.o.z + f.u.z * a.x + f.v.z * a.y + f.w.z * z };
  };
  const loads: WorldForce[] = (scenario?.loads ?? doc.loads).map((l) => ({ id: l.id, at: { x: l.at.x, y: l.at.y }, z: l.at.z ?? 0, fxN: l.fxN, fyN: l.fyN, fzN: l.fzN ?? 0, mxNm: l.momentXNm ?? 0, myNm: l.momentYNm ?? 0, mzNm: l.momentNm ?? 0, bone: l.bone, kind: "applied", title: label(l.id) }));
  const asset = {
    id: doc.case,
    rig,
    anchors,
    materials: doc.materials.map((m) => ({ id: m.id, title: label(m.id), emissivity: 0.9, elasticModulusPa: m.elasticModulusPa, source: "illustrative" })),
    structure: { gravityMS2: doc.gravityMS2, dynamics, supports: doc.supports, assumptions: [], clinical: false },
  } as unknown as Asset;
  const accelerations = Object.fromEntries(Object.entries(scenario?.accelerations ?? {}).map(([id, a]) => [id, { x: a.x, y: a.y, z: a.z ?? 0 }]));
  return {
    asset,
    query: { kind: "pose", pose: doc.case },
    timeS: 0,
    clipTimeS: 0,
    frame: 0,
    phase: null,
    root: { x: doc.root.x, y: doc.root.y, z: 0, angleDeg: doc.root.angleDeg },
    bones: evaluated,
    shapes: {} as EvaluatedState["shapes"],
    anchors: Object.fromEntries(anchors.map((a) => [a.id, { anchor: a, at: { x: at(a).x, y: at(a).y }, z: at(a).z }])),
    contacts: scenario?.contacts ?? doc.contacts,
    loads,
    thermal: { activityMet: 1, externalWorkW: 0, operating: {}, regions: {}, emitters: {}, breathing: null, passengers: {} },
    respiration: null,
    cycle: 0,
    representation: { kind: "none" },
    bounds: { minX: 0, minY: 0, maxX: 0, maxY: 0 },
    kinematics: { comVelocity: { x: 0, y: 0, z: 0 }, segmentAccelerations: accelerations },
  };
}

/** 📏️ `|a - b| <= max(abs, rel·max(|a|, |b|))`, the one closeness predicate of the structural cases. */
export function near(a: number, b: number, rel = 1e-9, abs = 1e-9): boolean {
  return Math.abs(a - b) <= Math.max(abs, rel * Math.max(Math.abs(a), Math.abs(b)));
}
