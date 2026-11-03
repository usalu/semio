/** 🔗️ Runtime contracts between the staffage modules: one evaluated state feeds every view, analysis and export.
 *
 * Pipeline: `Asset` → `evaluate()` → {@link EvaluatedState} (world metres) → `solveThermal()` / `solveStructure()` →
 * `depict()` → {@link DrawingScene} (paper millimetres) → SVG / PDF / raster frames; `modelScene()` → {@link ModelScene} (model metres) → DXF / DWG.
 *
 * World space: metres, x ahead, y up, z left of the asset (a left-handed triple), ground at y = 0; the side elevation is seen from the asset's right with ahead on the paper's right (see `../🎨️depiction/🔭️views/🟦️.ts`). Paper space: millimetres, x right, y down, origin top-left.
 *
 * @see ../../🧬️schema/🟦️.ts — the authored data
 */
import type { Alternative, Anchor, Asset, AuthoredRepresentation, Environment, Language, PrintProfile, ScaleDenominator, Shape, Slug, SymbolRepresentation, Text, UnavailableCause, Viewpoint } from "../../🧬️schema/🟦️.ts";

//#region 🧮️Evaluation
/** 🎯️ What to evaluate: a static pose (at `timeS` on its own timeline, default 0, so respiration and transport still evolve), a clip
 * at an absolute timeline time (seconds ≥ 0; a looping clip wraps the pose, root advance and transport continue), or a transition. */
export type EvaluationQuery = { readonly kind: "pose"; readonly pose: Slug; readonly timeS?: number } | { readonly kind: "clip"; readonly clip: Slug; readonly timeS: number } | { readonly kind: "transition"; readonly from: Slug; readonly to: Slug; readonly durationS: number; readonly timeS: number };

/** 🧭️ Which drawing to evaluate the geometry of: the authored representation chosen for a scale, or a named symbol. */
export type RepresentationSelector = { readonly kind: "scale"; readonly scale: DrawingScale } | { readonly kind: "symbol"; readonly symbol: Slug };

/** 🧮️ How a scale request resolves for an asset under a print profile.
 * - `authored`: the representation authored for `denominator`; `exact` when the requested scale equals it, otherwise a derived use
 *   inside its `validRange` (`requested` keeps the asked-for denominator; paper size always follows the requested scale).
 * - `unavailable`: the asset decided this scale is not drawable (authored decision) or the requested custom scale lies outside every
 *   `validRange`; carries the reason, the supported preset scales and alternatives.
 * - `unsuitable`: an authored representation exists but violates the print profile; carries the measured violations and alternatives.
 * - `symbol`: a symbol was selected explicitly. */
export type RepresentationResolution =
  | { readonly kind: "authored"; readonly requested: number; readonly denominator: ScaleDenominator; readonly exact: boolean; readonly representation: AuthoredRepresentation; readonly suitability: Suitability }
  | { readonly kind: "unavailable"; readonly requested: number; readonly cause: UnavailableCause | "outside-valid-range"; readonly reason: Text; readonly limitingFeature: Text | null; readonly limitingSizeM: number | null; readonly supported: readonly ScaleDenominator[]; readonly alternatives: readonly Alternative[] }
  | { readonly kind: "unsuitable"; readonly requested: number; readonly denominator: ScaleDenominator; readonly profile: Slug; readonly suitability: Suitability; readonly supported: readonly ScaleDenominator[]; readonly alternatives: readonly Alternative[] }
  | { readonly kind: "symbol"; readonly requested: number; readonly symbol: SymbolRepresentation };

/** 📏️ One print-profile violation measured on paper: which check, which shapes, the measured and required paper size. */
export type Violation = { readonly check: "stroke" | "gap" | "feature" | "hatch"; readonly shapes: readonly Slug[]; readonly measuredMm: number; readonly requiredMm: number; readonly at?: { readonly xMm: number; readonly yMm: number } };

/** ✅️ The suitability of one representation at one scale under one profile, measured over the named poses (and clip keys). */
export type Suitability = { readonly profile: PrintProfile; readonly suitable: boolean; readonly violations: readonly Violation[]; readonly minStrokeMm: number; readonly minGapMm: number; readonly minFeatureMm: number };

/** 📐️ A 2D affine transform `[a, b, c, d, e, f]`: x' = a·x + c·y + e, y' = b·x + d·y + f. */
export type Affine = readonly [number, number, number, number, number, number];

/** 🦴️ One evaluated bone in world space. */
export type EvaluatedBone = { readonly id: Slug; readonly world: Affine; readonly origin: Point; readonly tip: Vec3; readonly frame: BoneFrame; readonly angleRad: number; readonly offset: { readonly angleDeg: number; readonly abductionDeg: number; readonly twistDeg: number; readonly dx: number; readonly dy: number } };

/** 📍️ A world point. */
export type Point = { readonly x: number; readonly y: number };

/** 🧊️ A world point or vector in all three axes `(ahead, up, left)`: x forward, y up, z the lateral position (left of the asset is +z). */
export type Vec3 = { readonly x: number; readonly y: number; readonly z: number };

/** 🧭️ The rigid frame of a bone in world space: origin `o` and the orthonormal axes `u` (along the bone, ahead-up-left), `v` (its normal in the sagittal plane at zero spread) and `w` (the lateral axis), as solved by `solveChain`. */
export type BoneFrame = { readonly o: Vec3; readonly u: Vec3; readonly v: Vec3; readonly w: Vec3 };

/** ✏️ One evaluated shape: flattened world outline (closed rings or open polylines) plus its authored metadata. */
export type EvaluatedShape = { readonly shape: Shape; readonly rings: readonly (readonly Point[])[]; readonly closed: boolean };

/** 🧲️ A world-space force at the point `(at.x, at.y, z)`: components `fxN` ahead, `fyN` up, `fzN` left (all in world axes), and a couple `(mxNm, myNm, mzNm)` that acts in addition to the force and is independent of any reference point (a free moment applied at that point). The sign of the couple is positive about axis k when it turns i towards j for the cyclic order (i, j, k): `mxNm` turns y towards z, `myNm` turns z towards x, `mzNm` turns x towards y. `mzNm` is the sagittal moment about the lateral axis: it turns x (ahead) towards y (up), counter-clockwise in the side elevation. Because the world frame is left-handed, a moment arrow follows the left-hand rule; converted to a right-handed frame `(x, y, -z)` the couple is `(-mx, -my, mz)`. The moment of the force itself about a point `p` is `r × F` with `r = (at.x - p.x, at.y - p.y, z - p.z)`. */
export type WorldForce = { readonly id: Slug; readonly at: Point; readonly z: number; readonly fxN: number; readonly fyN: number; readonly fzN: number; readonly mxNm: number; readonly myNm: number; readonly mzNm: number; readonly bone: Slug; readonly kind: string; readonly title: Text };

/** 🌡️ The scalar thermal inputs at the evaluated instant (pose binding blended along the clip, then tracks applied); `passengers` per passenger body (ruling 4). */
export type ThermalInputs = {
  readonly activityMet: number;
  readonly externalWorkW: number;
  readonly operating: Readonly<Record<Slug, number>>;
  readonly regions: Readonly<Record<Slug, { readonly exposedFraction: number; readonly sunlitFraction: number; readonly wettedness?: number; readonly contact?: { readonly fraction: number; readonly tempC: number } }>>;
  readonly emitters: Readonly<Record<Slug, number>>;
  readonly breathing: Slug | null;
  readonly passengers: Readonly<Record<Slug, { readonly activityMet: number; readonly breathing: Slug | null }>>;
};

/** 🫁️ The respiratory state at the evaluated instant: continuous breathing phase (cycles since t = 0, integrated over the pattern
 * history), phase name, volumetric flow (m³/s, positive = exhaling), lung volume above functional residual capacity (L) and the pattern. */
export type RespirationState = { readonly pattern: Slug; readonly cycles: number; readonly phase: "inhale" | "pause-after-inhale" | "exhale" | "pause-after-exhale"; readonly flowM3S: number; readonly volumeL: number; readonly rateBpm: number; readonly tidalVolumeL: number };

/** 🧍️ The single evaluated state every view, analysis and export derives from. Deterministic in (asset, query). */
export type EvaluatedState = {
  readonly asset: Asset;
  readonly query: EvaluationQuery;
  readonly timeS: number;
  readonly clipTimeS: number;
  readonly frame: number;
  readonly phase: Slug | null;
  readonly bones: Readonly<Record<Slug, EvaluatedBone>>;
  readonly shapes: Readonly<Record<Viewpoint, readonly EvaluatedShape[]>>;
  readonly root: { readonly x: number; readonly y: number; readonly z: number; readonly angleDeg: number };
  readonly anchors: Readonly<Record<Slug, { readonly anchor: Anchor; readonly at: Point; readonly z: number }>>;
  readonly contacts: readonly Slug[];
  readonly loads: readonly WorldForce[];
  readonly thermal: ThermalInputs;
  readonly respiration: RespirationState | null;
  readonly cycle: number;
  readonly representation: { readonly kind: "scale"; readonly denominator: ScaleDenominator; readonly requested: number; readonly exact: boolean } | { readonly kind: "symbol"; readonly symbol: Slug } | { readonly kind: "none" };
  readonly bounds: { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };
  readonly kinematics: { readonly comVelocity: Vec3; readonly segmentAccelerations: Readonly<Record<Slug, Vec3>> };
};
//#endregion 🧮️Evaluation

//#region 🌡️Thermal
/** 🔬️ The physical quantities a thermal view can show, each with one unit:
 * `surface-temperature` °C of the outer surface; `skin-temperature` °C of the skin beneath clothing, fur or feathers (bodies only);
 * `emitted-radiant-flux` εσT⁴ W/m²; `net-radiative-flux` W/m² exchanged with sky, ground and surroundings (+ = loss);
 * `sensible-heat-flux` W/m² convected to the air (+ = to the air); `latent-heat-flux` W/m² carried by evaporation (+ = to the air);
 * `moisture-flux` g/(m²·h) water vapour released. */
export const THERMAL_QUANTITIES = ["surface-temperature", "skin-temperature", "emitted-radiant-flux", "net-radiative-flux", "sensible-heat-flux", "latent-heat-flux", "moisture-flux"] as const;

/** 🔢️ One of {@link THERMAL_QUANTITIES}. */
export type ThermalQuantity = (typeof THERMAL_QUANTITIES)[number];

/** 🔥️ The solved state of one thermal region at the evaluated instant. Fluxes in W/m² of region area, positive = leaving the surface
 * (`netRadiativeWM2` = sky + ground + surroundings parts; `contactWM2` conducted into seats/ground/handles);
 * powers in W include `exposedFraction`. `steadyTempC` is the steady-state surface temperature the transient response tends to.
 * A region with a `phaseChange` never exceeds its `pointC`: the surplus heating `phaseChangeW` (included in `latentW`, `latentWM2` and `moistureGM2H`) converts mass at `phaseChangeKgS = phaseChangeW / latentHeatJKg`;
 * `phaseChangedKg` is that mass accumulated over the solved input history (0 in a steady-state solve), `evaporationKgS` the whole vapour mass rate of the region (surface evaporation plus phase change) that vapour emitters follow. */
export type RegionField = {
  readonly region: Slug;
  readonly surfaceTempC: number;
  readonly steadyTempC: number;
  readonly skinTempC: number | null;
  readonly emittedRadiantWM2: number;
  readonly netRadiativeWM2: number;
  readonly netRadiativeSkyWM2: number;
  readonly netRadiativeGroundWM2: number;
  readonly netRadiativeSurroundingsWM2: number;
  readonly contactWM2: number;
  readonly sensibleWM2: number;
  readonly latentWM2: number;
  readonly moistureGM2H: number;
  readonly absorbedSolarWM2: number;
  readonly conductedWM2: number;
  readonly netRadiativeW: number;
  readonly sensibleW: number;
  readonly latentW: number;
  readonly phaseChangeW: number;
  readonly phaseChangeKgS: number;
  readonly phaseChangedKg: number;
  readonly evaporationKgS: number;
  readonly convectionCoefficientWM2K: number;
  readonly radiationCoefficientWM2K: number;
  readonly timeConstantS: number;
  readonly exposedFraction: number;
  readonly sunlitFraction: number;
};

/** 🫀️ The whole-body state of a living asset: core and mean skin temperature, metabolic and respiratory heat, skin blood flow, wettedness. */
export type BodyField = { readonly coreTempC: number; readonly meanSkinTempC: number; readonly metabolicW: number; readonly respiratorySensibleW: number; readonly respiratoryLatentW: number; readonly skinBloodFlowLM2H: number; readonly skinWettedness: number; readonly storageW: number };

/** 🌅️ The thermal surface field of one evaluated state under one environment (air transport lives in {@link TransportField}). */
export type ThermalField = {
  readonly environment: Environment;
  readonly regions: Readonly<Record<Slug, RegionField>>;
  readonly body: BodyField | null;
  readonly passengers: Readonly<Record<Slug, BodyField>>;
  readonly totals: { readonly netRadiativeW: number; readonly sensibleW: number; readonly latentW: number; readonly moistureGH: number; readonly emitterSensibleW: number; readonly emitterLatentW: number };
  readonly assumptions: readonly Text[];
  readonly notes: readonly Text[];
};

/** 🌤️ The environment along the absolute timeline: keys hold the environment from `t` on (`step`) or blend to the next (`linear`). */
export type EnvironmentTimeline = { readonly keys: readonly { readonly t: number; readonly environment: Environment; readonly ease: "step" | "linear" }[] };

/** 💨️ One Lagrangian air parcel in world space: emitted at `bornS` from `emitter` with the emitter's world position and direction at that
 * instant, then advected by ambient wind, its own decaying jet momentum and buoyancy while it entrains ambient air. `radiusM` grows with
 * entrainment; `excessK` and `vapourExcessKgPerKg` fall with dilution; `condensateKgPerKg` > 0 where the mixture is supersaturated. */
export type Parcel = { readonly id: string; readonly emitter: Slug; readonly bornS: number; readonly ageS: number; readonly at: Vec3; readonly velocity: Vec3; readonly radiusM: number; readonly excessK: number; readonly vapourExcessKgPerKg: number; readonly condensateKgPerKg: number; readonly carries: "airflow" | "vapour" };

/** 〰️ The path one parcel actually travelled from its emission to now (world space): a pathline. */
export type Pathline = { readonly parcel: string; readonly emitter: Slug; readonly points: readonly Vec3[]; readonly ages: readonly number[] };

/** 🌊️ An instantaneous streamline of the velocity field at the evaluated time, seeded at `seed`: tangent to the velocity everywhere at
 * this instant, not a particle path. */
export type Streamline = { readonly seed: Vec3; readonly points: readonly Vec3[]; readonly speedMS: readonly number[] };

/** 🏭️ The state of one emitter at the evaluated instant. */
export type EmitterState = { readonly emitter: Slug; readonly kind: string; readonly at: Vec3; readonly direction: Vec3; readonly volumeFlowM3S: number; readonly exitVelocityMS: number; readonly temperatureC: number; readonly vapourKgPerKg: number; readonly active: boolean };

/** 🌬️ The air transport at the evaluated instant: emitter states, parcels, their pathlines, instantaneous streamlines, the buoyant
 * body/surface plume and visible condensation. Deterministic in (asset, query, environment timeline); identical whether reached by
 * playing or scrubbing. */
export type TransportField = {
  readonly timeS: number;
  readonly emitters: readonly EmitterState[];
  readonly parcels: readonly Parcel[];
  readonly pathlines: readonly Pathline[];
  readonly streamlines: readonly Streamline[];
  readonly condensation: readonly { readonly at: Vec3; readonly radiusM: number; readonly condensateKgPerKg: number }[];
  readonly respiration: RespirationState | null;
  readonly model: { readonly stepS: number; readonly maxAgeS: number; readonly seed: number };
  readonly notes: readonly Text[];
};

/** 🖍️ What the transport overlay draws. */
export const TRANSPORT_LAYERS = ["parcels", "pathlines", "streamlines", "condensation"] as const;

/** 🪧️ One of {@link TRANSPORT_LAYERS}. */
export type TransportLayer = (typeof TRANSPORT_LAYERS)[number];
//#endregion 🌡️Thermal

//#region 🏗️Structure
/** 🧷️ A support reaction at an anchor in world newtons and newton-metres (all three force and moment components; `mxNm`, `myNm`, `mzNm` are the couple the support applies, so it does not depend on a reference point; positive about axis k turns i towards j for the cyclic order (i, j, k), so positive `mzNm` is counter-clockwise in the side elevation; x ahead, y up, z left is a left-handed frame), plus contact pressure where a footprint is given; `cop` is
 * the centre of pressure inside the footprint (a footprint contact carries |M| ≤ N · footprintLengthM / 2). */
export type Reaction = { readonly anchor: Slug; readonly at: Point; readonly z: number; readonly fxN: number; readonly fyN: number; readonly fzN: number; readonly mxNm: number; readonly myNm: number; readonly mzNm: number; readonly pressurePa: number | null; readonly active: boolean; readonly cop?: Vec3 };

/** 🔩️ The internal load a parent transmits to a child bone at their joint (acting on the child subtree), in world axes and bone axes: `fxN`, `fyN`, `fzN` the force on the child, `mxNm`, `myNm`, `mzNm` the moment of the child subtree's external loads about the joint origin (same signs as {@link Reaction}), `torsionNm`, `momentNm`, `momentLateralNm` its components about the bone axis `u`, lateral axis `w` and normal `v`, `shearN` and `shearZN` the shear along `v` and `w`. */
export type JointLoad = { readonly bone: Slug; readonly at: Point; readonly z: number; readonly fxN: number; readonly fyN: number; readonly fzN: number; readonly axialN: number; readonly shearN: number; readonly shearZN: number; readonly mxNm: number; readonly myNm: number; readonly mzNm: number; readonly torsionNm: number; readonly momentNm: number; readonly momentLateralNm: number };

/** 🪜️ One station of a member diagram: position along the bone (m) and internal N (tension +), V along `v` (`shearN`) and along `w` (`shearZN`), torsion about `u`, bending M about `w` (`momentNm`, sagging +) and about `v` (`momentLateralNm`) at that cut; `shearVector` is -Σ distal transverse force and `momentVector` the full moment of the distal loads about the cut point, both in world axes. */
export type DiagramStation = {
  readonly s: number;
  readonly at: Point;
  readonly z: number;
  readonly axialN: number;
  readonly shearN: number;
  readonly shearZN: number;
  readonly torsionNm: number;
  readonly momentNm: number;
  readonly momentLateralNm: number;
  readonly shearVector: Vec3;
  readonly momentVector: Vec3;
  readonly stressMinPa: number | null;
  readonly stressMaxPa: number | null;
};

/** 📈️ The axial, shear and moment diagram of one bone. */
export type MemberDiagram = { readonly bone: Slug; readonly stations: readonly DiagramStation[]; readonly deflection: readonly Point[] | null };

/** ⚖️ The structural solution of one evaluated state. */
export type StructuralResult = {
  readonly gravityMS2: number;
  readonly dynamics: "quasi-static" | "inertial";
  readonly centreOfMass: Vec3;
  readonly totalMassKg: number;
  readonly segmentWeights: readonly WorldForce[];
  readonly inertialForces: readonly WorldForce[];
  readonly applied: readonly WorldForce[];
  readonly reactions: readonly Reaction[];
  readonly joints: Readonly<Record<Slug, JointLoad>>;
  readonly members: Readonly<Record<Slug, MemberDiagram>>;
  readonly residual: { readonly fxN: number; readonly fyN: number; readonly fzN: number; readonly mxNm: number; readonly myNm: number; readonly mzNm: number };
  readonly notes: readonly Text[];
};
//#endregion 🏗️Structure

//#region 🎨️Depiction
/** 🖼️ The seven synchronized views. */
export const VIEWS = ["dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"] as const;

/** 🔎️ One of {@link VIEWS}. */
export type View = (typeof VIEWS)[number];

/** 🏗️ The quantity the force view paints on members. */
export const FORCE_QUANTITIES = ["axial", "shear", "stress", "contact-pressure"] as const;

/** 🔣️ One of {@link FORCE_QUANTITIES}. */
export type ForceQuantity = (typeof FORCE_QUANTITIES)[number];

/** 🎚️ A display range for an analytic legend: `shared` is the documented fixed comparison range of the quantity (identical for every
 * asset, scale and time), `fixed` a user range, `auto` the min…max of the values present (the legend states that it is normalised), `span` a symmetric range of
 * ±`spanK` kelvin around the centre (the air temperature) of the surface-temperature thermal view, authored once per scene so its casts stay mutually comparable; other quantities and views treat it as `shared`. */
export type DisplayRange = { readonly mode: "shared" } | { readonly mode: "auto" } | { readonly mode: "fixed"; readonly min: number; readonly max: number } | { readonly mode: "span"; readonly spanK: number };

/** 🔍️ A drawing scale `1:denominator`; presets 500, 200, 100, 50, any positive denominator allowed. */
export type DrawingScale = { readonly denominator: number };

/** 🎛️ Everything a depiction needs besides the state. `theme` sets the ink and paper of the analytic views (thermal, forces, moments);
 * the four graphic views carry their own theme in their name. Scenes of different views share `metadata.origin`, not their extent. */
export type DepictionOptions = DepictionCommon & (TextualDepiction | TextlessDepiction);

/** 🔤️ A depiction that may render text (labels, legend, dimensions, scale bar, free-body inset): the language is required and never defaulted. */
export type TextualDepiction = {
  readonly language: Language;
  readonly annotations: { readonly dimensions: boolean; readonly scaleBar: boolean; readonly legend: boolean; readonly labels: boolean; readonly ground: boolean };
  readonly diagrams: { readonly axial: boolean; readonly shear: boolean; readonly moment: boolean; readonly freeBody: boolean };
};

/** 🤐️ A depiction that renders no text: no language exists, and the type forbids every text annotation and the free-body inset. */
export type TextlessDepiction = {
  readonly language?: undefined;
  readonly annotations: { readonly dimensions: false; readonly scaleBar: false; readonly legend: false; readonly labels: false; readonly ground: boolean };
  readonly diagrams: { readonly axial: boolean; readonly shear: boolean; readonly moment: boolean; readonly freeBody: false };
};

/** 🧱️ The depiction options that do not depend on whether text is rendered. */
export type DepictionCommon = {
  readonly view: View;
  readonly viewpoint: Viewpoint;
  readonly scale: DrawingScale;
  readonly transparent: boolean;
  readonly marginMm: number;
  readonly thermalQuantity: ThermalQuantity;
  readonly transport: readonly TransportLayer[];
  readonly transportStrokeMm?: number;
  readonly transportTrailS?: number;
  readonly profile: PrintProfile;
  readonly representation: RepresentationSelector;
  readonly forceQuantity: ForceQuantity;
  readonly range: DisplayRange;
  readonly patterns: boolean;
  readonly theme?: "light" | "dark";
  readonly environment?: Environment;
  readonly environmentTimeline?: EnvironmentTimeline;
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

/** 🔖️ One of {@link LAYERS}. */
export type LayerId = (typeof LAYERS)[number];

/** 📚️ A layer with its items. */
export type DrawingLayer = { readonly id: LayerId; readonly title: Text; readonly items: readonly DrawingItem[] };

/** 🏷️ A legend: title, unit, sign convention and colour stops, all explicit; `stops[].at` is normalised 0…1 over [`min`, `max`]. */
export type Legend = { readonly title: Text; readonly quantity: string; readonly unit: string; readonly convention: Text; readonly min: number; readonly max: number; readonly stops: readonly { readonly at: number; readonly color: string }[]; readonly interpretation: Text; readonly range: DisplayRange["mode"]; readonly conditions?: Text; readonly assumptions?: readonly Text[] };

/** 🏷️ How a drawing relates to the real object: `physical` is metres scaled to paper millimetres (an authored scale representation or a `true-extent` symbol); `conventional-symbol` is a paper-millimetre sign (`true-position` or `nominal` symbol) whose size carries no dimension. Carried by `DrawingScene.metadata.fidelity` so every consumer can show it to the reader. */
export type Fidelity = "physical" | "conventional-symbol";

/** 🖨️ A paper-space scene at a given scale: what SVG and PDF writers serialise one to one. */
export type DrawingScene = {
  readonly widthMm: number;
  readonly heightMm: number;
  readonly background: string | null;
  readonly layers: readonly DrawingLayer[];
  readonly legends: readonly Legend[];
  readonly metadata: { readonly asset: Slug; readonly title: Text; readonly query: EvaluationQuery; readonly timeS: number; readonly view: View; readonly viewpoint: Viewpoint; readonly scale: DrawingScale; readonly resolution: RepresentationResolution["kind"]; readonly fidelity: Fidelity; readonly omittedValues: number; readonly representation: EvaluatedState["representation"]; readonly profile: Slug; readonly paperUnits: "mm"; readonly modelUnits: "m"; readonly mmPerMetre: number; readonly origin: { readonly xMm: number; readonly yMm: number } };
};
//#endregion 🎨️Depiction

//#region 📐️CAD
/** 🎨️ A CAD layer with a colour (ACI) and lineweight (1/100 mm) and its model-space polylines in metres. */
export type ModelLayer = { readonly name: string; readonly aci: number; readonly lineweight: number; readonly polylines: readonly { readonly points: readonly Point[]; readonly closed: boolean }[]; readonly texts: readonly { readonly at: Point; readonly heightM: number; readonly text: string }[] };

/** 🌐️ A model-space scene in declared units for DXF and DWG. */
export type ModelScene = { readonly units: "m"; readonly asset: Slug; readonly timeS: number; readonly representation: EvaluatedState["representation"]; readonly layers: readonly ModelLayer[] };
//#endregion 📐️CAD

//#region ⏳️Jobs
/** ⏳️ Progress and cancellation for every expensive operation (exports, frame renders, catalog validation). */
export type JobControl = { readonly signal?: AbortSignal; readonly progress?: (done: number, total: number, label: Text) => void };
//#endregion ⏳️Jobs
