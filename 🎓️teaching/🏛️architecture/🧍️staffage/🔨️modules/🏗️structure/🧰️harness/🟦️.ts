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
import type { Anchor, Asset, Bone, Section, SegmentMass, Slug, SupportType } from "../../../🧬️schema/🟦️.ts";
import type { Affine, EvaluatedBone, EvaluatedState, Point, WorldForce } from "../../🔗️contracts/🟦️.ts";

/** 🧱️ A bone of a case document. */
export type CaseBone = { readonly id: Slug; readonly parent: Slug | null; readonly x: number; readonly y: number; readonly angleDeg: number; readonly lengthM: number; readonly joint?: "fixed" | "revolute"; readonly mass?: Pick<SegmentMass, "kg" | "comX" | "comY" | "distribution">; readonly section?: Omit<Section, "title"> };

/** 📍️ An anchor of a case document. */
export type CaseAnchor = { readonly id: Slug; readonly bone: Slug; readonly x: number; readonly y: number; readonly kind: Anchor["kind"]; readonly support?: SupportType; readonly normalDeg?: number; readonly footprintLengthM?: number; readonly footprintWidthM?: number; readonly frictionCoefficient?: number };

/** 🧲️ A world load of a case document. */
export type CaseLoad = { readonly id: Slug; readonly bone: Slug; readonly at: Point; readonly fxN: number; readonly fyN: number; readonly momentNm?: number };

/** 🎬️ What one scenario overrides on the case model. */
export type CaseScenario = { readonly id: string; readonly loads?: readonly CaseLoad[]; readonly contacts?: readonly Slug[]; readonly dynamics?: "quasi-static" | "inertial"; readonly accelerations?: Readonly<Record<Slug, Point>>; readonly bones?: Readonly<Record<Slug, Partial<Pick<CaseBone, "angleDeg" | "x" | "y">>>> };

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

/** 🧱️ Builds the evaluated state of a case, optionally under one scenario's overrides. */
export function buildState(doc: StructureCase, scenario?: CaseScenario): EvaluatedState {
  const dynamics = scenario?.dynamics ?? doc.dynamics;
  const bones: Bone[] = doc.bones.map((b) => {
    const o = scenario?.bones?.[b.id] ?? {};
    return {
      id: b.id,
      parent: b.parent,
      title: label(b.id),
      x: o.x ?? b.x,
      y: o.y ?? b.y,
      angleDeg: o.angleDeg ?? b.angleDeg,
      lengthM: b.lengthM,
      joint: { type: b.joint ?? "fixed" },
      ...(b.mass ? { mass: { ...b.mass, source: "illustrative" as const } } : {}),
      ...(b.section ? { section: b.section } : {}),
    } as Bone;
  });
  const world = new Map<Slug, { a: number; origin: Point }>();
  const evaluated: Record<Slug, EvaluatedBone> = {};
  for (const b of bones) {
    const parent = b.parent === null ? null : world.get(b.parent)!;
    const base = parent ?? { a: (doc.root.angleDeg * Math.PI) / 180, origin: { x: doc.root.x, y: doc.root.y } };
    const pc = Math.cos(base.a);
    const ps = Math.sin(base.a);
    const origin = parent ? { x: base.origin.x + pc * b.x - ps * b.y, y: base.origin.y + ps * b.x + pc * b.y } : base.origin;
    const angle = base.a + (b.parent === null ? 0 : (b.angleDeg * Math.PI) / 180);
    world.set(b.id, { a: angle, origin });
    const c = Math.cos(angle);
    const s = Math.sin(angle);
    const affine: Affine = [c, s, -s, c, origin.x, origin.y];
    evaluated[b.id] = { id: b.id, world: affine, origin, tip: { x: origin.x + c * b.lengthM, y: origin.y + s * b.lengthM }, angleRad: angle, offset: { angleDeg: 0, dx: 0, dy: 0 } };
  }
  const anchors: Anchor[] = doc.anchors.map((a) => ({ ...a, title: label(a.id) }) as Anchor);
  const at = (a: Anchor): Point => {
    const e = evaluated[a.bone]!;
    return { x: e.origin.x + Math.cos(e.angleRad) * a.x - Math.sin(e.angleRad) * a.y, y: e.origin.y + Math.sin(e.angleRad) * a.x + Math.cos(e.angleRad) * a.y };
  };
  const loads: WorldForce[] = (scenario?.loads ?? doc.loads).map((l) => ({ id: l.id, at: l.at, fxN: l.fxN, fyN: l.fyN, momentNm: l.momentNm ?? 0, bone: l.bone, kind: "applied", title: label(l.id) }));
  const asset = {
    id: doc.case,
    rig: { kind: doc.kind, root: bones[0]!.id, bones },
    anchors,
    materials: doc.materials.map((m) => ({ id: m.id, title: label(m.id), emissivity: 0.9, elasticModulusPa: m.elasticModulusPa, source: "illustrative" })),
    structure: { gravityMS2: doc.gravityMS2, dynamics, supports: doc.supports, assumptions: [], clinical: false },
  } as unknown as Asset;
  return {
    asset,
    query: { kind: "pose", pose: doc.case },
    timeS: 0,
    frame: 0,
    phase: null,
    bones: evaluated,
    shapes: {} as EvaluatedState["shapes"],
    anchors: Object.fromEntries(anchors.map((a) => [a.id, { anchor: a, at: at(a) }])),
    contacts: scenario?.contacts ?? doc.contacts,
    loads,
    thermal: { activityMet: 1, externalWorkW: 0, operating: {}, regions: {}, emitters: {}, breathing: null, passengers: {} },
    respiration: null,
    cycle: 0,
    representation: { kind: "none" },
    bounds: { minX: 0, minY: 0, maxX: 0, maxY: 0 },
    kinematics: { comVelocity: { x: 0, y: 0 }, segmentAccelerations: scenario?.accelerations ?? {} },
  };
}

/** 📏️ `|a - b| <= max(abs, rel·max(|a|, |b|))`, the one closeness predicate of the structural cases. */
export function near(a: number, b: number, rel = 1e-9, abs = 1e-9): boolean {
  return Math.abs(a - b) <= Math.max(abs, rel * Math.max(Math.abs(a), Math.abs(b)));
}
