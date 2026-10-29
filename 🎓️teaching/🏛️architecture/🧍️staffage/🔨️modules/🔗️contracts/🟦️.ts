/** 🔗️ Runtime contracts between the staffage modules: one evaluated state feeds every view, analysis and export.
 *
 * Pipeline: `Asset` → `evaluate()` → {@link EvaluatedState} (world metres) → `solveThermal()` / `solveStructure()` →
 * `depict()` → {@link DrawingScene} (paper millimetres) → SVG / PDF / raster frames; `modelScene()` → {@link ModelScene} (model metres) → DXF / DWG.
 *
 * World space: metres, x right, y up, ground at y = 0. Paper space: millimetres, x right, y down, origin top-left.
 *
 * @see ../../🧬️schema/🟦️.ts — the authored data
 */
import type { Anchor, Asset, Environment, Language, Shape, Slug, Text, Viewpoint } from "../../🧬️schema/🟦️.ts";

//#region 🧮️Evaluation
/** 🎯️ What to evaluate: a static pose, or a clip at an arbitrary time (seconds, wrapped when the clip loops). */
export type EvaluationQuery = { readonly kind: "pose"; readonly pose: Slug } | { readonly kind: "clip"; readonly clip: Slug; readonly timeS: number } | { readonly kind: "transition"; readonly from: Slug; readonly to: Slug; readonly durationS: number; readonly timeS: number };

/** 📐️ A 2D affine transform `[a, b, c, d, e, f]`: x' = a·x + c·y + e, y' = b·x + d·y + f. */
export type Affine = readonly [number, number, number, number, number, number];

/** 🦴️ One evaluated bone in world space. */
export type EvaluatedBone = { readonly id: Slug; readonly world: Affine; readonly origin: Point; readonly tip: Point; readonly angleRad: number; readonly offset: { readonly angleDeg: number; readonly dx: number; readonly dy: number } };

/** 📍️ A world point. */
export type Point = { readonly x: number; readonly y: number };

/** ✏️ One evaluated shape: flattened world outline (closed rings or open polylines) plus its authored metadata. */
export type EvaluatedShape = { readonly shape: Shape; readonly rings: readonly (readonly Point[])[]; readonly closed: boolean };

/** 🧲️ A world-space force at a point. */
export type WorldForce = { readonly id: Slug; readonly at: Point; readonly fxN: number; readonly fyN: number; readonly momentNm: number; readonly bone: Slug; readonly kind: string; readonly title: Text };

/** 🌡️ The scalar thermal inputs at the evaluated instant (pose binding blended along the clip, then tracks applied). */
export type ThermalInputs = {
  readonly activityMet: number;
  readonly operating: Readonly<Record<Slug, number>>;
  readonly regions: Readonly<Record<Slug, { readonly exposedFraction: number; readonly surfaceTempC?: number; readonly coreTempC?: number; readonly wettedness?: number }>>;
  readonly emitters: Readonly<Record<Slug, number>>;
};

/** 🧍️ The single evaluated state every view, analysis and export derives from. Deterministic in (asset, query). */
export type EvaluatedState = {
  readonly asset: Asset;
  readonly query: EvaluationQuery;
  readonly timeS: number;
  readonly frame: number;
  readonly phase: Slug | null;
  readonly bones: Readonly<Record<Slug, EvaluatedBone>>;
  readonly shapes: Readonly<Record<Viewpoint, readonly EvaluatedShape[]>>;
  readonly anchors: Readonly<Record<Slug, { readonly anchor: Anchor; readonly at: Point }>>;
  readonly contacts: readonly Slug[];
  readonly loads: readonly WorldForce[];
  readonly thermal: ThermalInputs;
  readonly bounds: { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };
  readonly kinematics: { readonly comVelocity: Point; readonly segmentAccelerations: Readonly<Record<Slug, Point>> };
};
//#endregion 🧮️Evaluation

//#region 🌡️Thermal
/** 🔥️ The solved state of one thermal region at the evaluated instant (W/m² positive = leaving the surface). */
export type RegionField = {
  readonly region: Slug;
  readonly surfaceTempC: number;
  readonly radiantFluxWM2: number;
  readonly convectiveFluxWM2: number;
  readonly latentFluxWM2: number;
  readonly netFluxWM2: number;
  readonly radiantW: number;
  readonly convectiveW: number;
  readonly latentW: number;
  readonly convectionCoefficientWM2K: number;
  readonly radiationCoefficientWM2K: number;
  readonly exposedFraction: number;
};

/** 💨️ One deterministic tracer parcel of an emitter, advected from its emission time; `excessK` and `vapourKgPerKg` decay along the jet/plume. */
export type Parcel = { readonly emitter: Slug; readonly at: Point; readonly ageS: number; readonly velocity: Point; readonly excessK: number; readonly vapourKgPerKg: number; readonly carries: "airflow" | "vapour" };

/** 〰️ A jet/plume centreline sampled from the source under the current wind and buoyancy. */
export type Streamline = { readonly emitter: Slug; readonly points: readonly Point[]; readonly excessK: readonly number[]; readonly speedMS: readonly number[]; readonly carries: "airflow" | "vapour"; readonly active: number };

/** 🌡️ The thermal field of one evaluated state under one environment. */
export type ThermalField = {
  readonly environment: Environment;
  readonly regions: Readonly<Record<Slug, RegionField>>;
  readonly totals: { readonly radiantW: number; readonly convectiveW: number; readonly latentW: number; readonly emitterSensibleW: number; readonly emitterLatentW: number };
  readonly streamlines: readonly Streamline[];
  readonly parcels: readonly Parcel[];
  readonly notes: readonly Text[];
};
//#endregion 🌡️Thermal

//#region 🏗️Structure
/** 🧷️ A support reaction at an anchor in world newtons and newton-metres, plus contact pressure where a footprint is given; `cop` is
 * the centre of pressure inside the footprint (a footprint contact carries |M| ≤ N · footprintLengthM / 2). */
export type Reaction = { readonly anchor: Slug; readonly at: Point; readonly fxN: number; readonly fyN: number; readonly momentNm: number; readonly pressurePa: number | null; readonly active: boolean; readonly cop?: Point };

/** 🔩️ The internal load a parent transmits to a child bone at their joint (acting on the child subtree), in world axes and bone axes. */
export type JointLoad = { readonly bone: Slug; readonly at: Point; readonly fxN: number; readonly fyN: number; readonly axialN: number; readonly shearN: number; readonly momentNm: number };

/** 📈️ One station of a member diagram: position along the bone (m) and internal N (tension +), V, M (sagging +) at that cut. */
export type DiagramStation = { readonly s: number; readonly at: Point; readonly axialN: number; readonly shearN: number; readonly momentNm: number; readonly stressMinPa: number | null; readonly stressMaxPa: number | null };

/** 📈️ The axial, shear and moment diagram of one bone. */
export type MemberDiagram = { readonly bone: Slug; readonly stations: readonly DiagramStation[]; readonly deflection: readonly Point[] | null };

/** ⚖️ The structural solution of one evaluated state. */
export type StructuralResult = {
  readonly gravityMS2: number;
  readonly dynamics: "quasi-static" | "inertial";
  readonly centreOfMass: Point;
  readonly totalMassKg: number;
  readonly segmentWeights: readonly WorldForce[];
  readonly inertialForces: readonly WorldForce[];
  readonly applied: readonly WorldForce[];
  readonly reactions: readonly Reaction[];
  readonly joints: Readonly<Record<Slug, JointLoad>>;
  readonly members: Readonly<Record<Slug, MemberDiagram>>;
  readonly residual: { readonly fxN: number; readonly fyN: number; readonly momentNm: number };
  readonly notes: readonly Text[];
};
//#endregion 🏗️Structure

//#region 🎨️Depiction
/** 🖼️ The seven synchronized views. */
export const VIEWS = ["dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"] as const;

/** 🖼️ One of {@link VIEWS}. */
export type View = (typeof VIEWS)[number];

/** 🌡️ The quantity the thermal view paints. */
export const THERMAL_QUANTITIES = ["surface-temperature", "radiant-flux", "convective-flux", "latent-flux", "net-flux"] as const;

/** 🌡️ One of {@link THERMAL_QUANTITIES}. */
export type ThermalQuantity = (typeof THERMAL_QUANTITIES)[number];

/** 🏗️ The quantity the force view paints on members. */
export const FORCE_QUANTITIES = ["axial", "shear", "stress", "contact-pressure"] as const;

/** 🏗️ One of {@link FORCE_QUANTITIES}. */
export type ForceQuantity = (typeof FORCE_QUANTITIES)[number];

/** 🎚️ A display range for an analytic legend; `auto` derives it from the values present. */
export type DisplayRange = { readonly mode: "auto" } | { readonly mode: "fixed"; readonly min: number; readonly max: number };

/** 🖨️ A drawing scale `1:denominator`; presets 500, 200, 100, 50, any positive denominator allowed. */
export type DrawingScale = { readonly denominator: number };

/** 🎛️ Everything a depiction needs besides the state. `theme` sets the ink and paper of the analytic views (thermal, forces, moments);
 * the four graphic views carry their own theme in their name. Scenes of different views share `metadata.origin`, not their extent. */
export type DepictionOptions = {
  readonly view: View;
  readonly viewpoint: Viewpoint;
  readonly scale: DrawingScale;
  readonly language: Language;
  readonly transparent: boolean;
  readonly marginMm: number;
  readonly annotations: { readonly dimensions: boolean; readonly scaleBar: boolean; readonly legend: boolean; readonly labels: boolean; readonly ground: boolean };
  readonly thermalQuantity: ThermalQuantity;
  readonly forceQuantity: ForceQuantity;
  readonly diagrams: { readonly axial: boolean; readonly shear: boolean; readonly moment: boolean; readonly freeBody: boolean };
  readonly range: DisplayRange;
  readonly patterns: boolean;
  readonly theme?: "light" | "dark";
  readonly environment?: Environment;
};

/** ✒️ A paper-space path command list (mm, y down). */
export type PathCommand = { readonly op: "M" | "L"; readonly x: number; readonly y: number } | { readonly op: "C"; readonly x1: number; readonly y1: number; readonly x2: number; readonly y2: number; readonly x: number; readonly y: number } | { readonly op: "Z" };

/** 🖌️ A stroke: sRGB `#rrggbb`, width in paper millimetres, optional dash pattern in millimetres. */
export type Stroke = { readonly color: string; readonly widthMm: number; readonly dashMm?: readonly number[]; readonly opacity?: number; readonly cap?: "butt" | "round"; readonly join?: "miter" | "round" };

/** 🪣️ A fill: sRGB colour, opacity, or a hatch pattern id for colour-independent reading. */
export type Fill = { readonly color: string; readonly opacity?: number; readonly pattern?: "compression" | "tension" | "hot" | "cold" | "vapour" };

/** 🧾️ The physical value carried by a drawn item, for inspection and accessible tables. */
export type ItemValue = { readonly quantity: string; readonly value: number; readonly unit: string; readonly label: Text };

/** 🧩️ One drawn item. */
export type DrawingItem =
  | { readonly kind: "path"; readonly id: string; readonly commands: readonly PathCommand[]; readonly stroke?: Stroke; readonly fill?: Fill; readonly value?: ItemValue; readonly bone?: Slug; readonly region?: Slug }
  | { readonly kind: "text"; readonly id: string; readonly x: number; readonly y: number; readonly text: string; readonly sizeMm: number; readonly color: string; readonly anchor: "start" | "middle" | "end"; readonly weight?: "normal" | "bold" };

/** 🗂️ A named drawing layer (SVG group, PDF optional content order). */
export const LAYERS = ["background", "ground", "fill", "field", "body", "detail", "texture", "glazing", "diagrams", "forces", "annotations", "legend"] as const;

/** 🗂️ One of {@link LAYERS}. */
export type LayerId = (typeof LAYERS)[number];

/** 🗂️ A layer with its items. */
export type DrawingLayer = { readonly id: LayerId; readonly title: Text; readonly items: readonly DrawingItem[] };

/** 🏷️ A legend: title, unit, sign convention and colour stops, all explicit; `stops[].at` is normalised 0…1 over [`min`, `max`]. */
export type Legend = { readonly title: Text; readonly quantity: string; readonly unit: string; readonly convention: Text; readonly min: number; readonly max: number; readonly stops: readonly { readonly at: number; readonly color: string }[]; readonly interpretation: Text };

/** 🖨️ A paper-space scene at a given scale: what SVG and PDF writers serialise one to one. */
export type DrawingScene = {
  readonly widthMm: number;
  readonly heightMm: number;
  readonly background: string | null;
  readonly layers: readonly DrawingLayer[];
  readonly legends: readonly Legend[];
  readonly metadata: { readonly asset: Slug; readonly title: Text; readonly query: EvaluationQuery; readonly timeS: number; readonly view: View; readonly viewpoint: Viewpoint; readonly scale: DrawingScale; readonly paperUnits: "mm"; readonly modelUnits: "m"; readonly mmPerMetre: number; readonly origin: { readonly xMm: number; readonly yMm: number } };
};
//#endregion 🎨️Depiction

//#region 📐️CAD
/** 📐️ A CAD layer with a colour (ACI) and lineweight (1/100 mm) and its model-space polylines in metres. */
export type ModelLayer = { readonly name: string; readonly aci: number; readonly lineweight: number; readonly polylines: readonly { readonly points: readonly Point[]; readonly closed: boolean }[]; readonly texts: readonly { readonly at: Point; readonly heightM: number; readonly text: string }[] };

/** 📐️ A model-space scene in declared units for DXF and DWG. */
export type ModelScene = { readonly units: "m"; readonly asset: Slug; readonly timeS: number; readonly layers: readonly ModelLayer[] };
//#endregion 📐️CAD

//#region ⏳️Jobs
/** ⏳️ Progress and cancellation for every expensive operation (exports, frame renders, catalog validation). */
export type JobControl = { readonly signal?: AbortSignal; readonly progress?: (done: number, total: number, label: Text) => void };
//#endregion ⏳️Jobs
