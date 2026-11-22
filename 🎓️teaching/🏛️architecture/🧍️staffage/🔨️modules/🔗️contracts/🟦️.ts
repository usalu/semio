/** 🔗️ Runtime contracts between the staffage modules: one evaluated state feeds every view, analysis and export.
 *
 * Pipeline: `Asset` → `evaluate()` → {@link EvaluatedState} (world metres) → `solveThermal()` / `solveStructure()` →
 * `depict()` → {@link DrawingScene} (paper millimetres) → SVG / PDF / raster frames; `modelScene()` → {@link ModelScene} (model metres) → DXF / DWG.
 *
 * World space: metres, x ahead, y up, z left of the asset (a left-handed triple), ground at y = 0; the side elevation is seen from the asset's right with ahead on the paper's right (see `../🎨️depiction/🔭️views/🟦️.ts`). Paper space: millimetres, x right, y down, origin top-left.
 *
 * @see ../../🧬️schema/🟦️.ts — the authored data
 */
import type { Alternative, Anchor, Asset, AuthoredRepresentation, Environment, Language, PlanarViewpoint, PrintProfile, ScaleDenominator, Shape, ShapeRole, Slug, SymbolRepresentation, Text, UnavailableCause, Viewpoint } from "../../🧬️schema/🟦️.ts";

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

/** 🗺️ The rigid frame of a bone in world space: origin `o` and the orthonormal axes `u` (along the bone, ahead-up-left), `v` (its normal, in the sagittal plane for a sagittal rest placement at zero spread) and `w` (its joint axis, lateral for a sagittal rest placement), as solved by `solveChain`. */
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
  readonly shapes: Readonly<Record<PlanarViewpoint, readonly EvaluatedShape[]>>;
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

/** 🏭️ The state of one emitter at the evaluated instant. */
export type EmitterState = { readonly emitter: Slug; readonly kind: string; readonly at: Vec3; readonly direction: Vec3; readonly volumeFlowM3S: number; readonly exitVelocityMS: number; readonly temperatureC: number; readonly vapourKgPerKg: number; readonly active: boolean };

/** 🌬️ The air transport at the evaluated instant: emitter states, the parcels of every jet, plume, breath and transpiration (a parcel with
 * `condensateKgPerKg` > 0 is visible condensation), their pathlines and, when asked for, the flow field every drawing of the air reads
 * (`flow`). Deterministic in (asset, query, environment timeline); identical whether reached by playing or scrubbing. */
export type TransportField = {
  readonly timeS: number;
  readonly emitters: readonly EmitterState[];
  readonly parcels: readonly Parcel[];
  readonly pathlines: readonly Pathline[];
  readonly flow: FlowField | null;
  readonly respiration: RespirationState | null;
  readonly model: { readonly stepS: number; readonly maxAgeS: number; readonly seed: number };
  readonly notes: readonly Text[];
};

/** 🌊️ One stream of the flow field: an instantaneous streamline of the reconstructed velocity field with, per point, the travel time from its first point (s), the speed (m/s) and the local temperature excess (K), vapour excess and condensate (kg/kg moist air).
 * `role` says what seeded it: the outflow of an emitter (`source`), ambient air drawn into a jet or plume (`entrained`), the moving ambient air (`ambient`) or air drawn to an inhaling nose or mouth (`inhaled`). */
export type FlowStream = { readonly seed: Vec3; readonly role: "source" | "entrained" | "ambient" | "inhaled"; readonly emitter: Slug | null; readonly points: readonly Vec3[]; readonly travelS: readonly number[]; readonly speedMS: readonly number[]; readonly excessK: readonly number[]; readonly vapourExcessKgPerKg: readonly number[]; readonly condensateKgPerKg: readonly number[] };

/** ☁️ One puff of the flow field: a parcel with its entrained radius and what it carries. */
export type FlowPuff = { readonly at: Vec3; readonly radiusM: number; readonly excessK: number; readonly vapourExcessKgPerKg: number; readonly condensateKgPerKg: number; readonly carries: "airflow" | "vapour" };

/** 🐾️ One path of the flow field: the way one parcel actually travelled from its emission to now (`points`, oldest first, with the parcel's age in seconds at each point, so the last age is its age now) and what the parcel carries now. In unsteady flow (breath, a moving emitter, gusts) paths differ from streams. */
export type FlowPath = { readonly parcel: string; readonly emitter: Slug; readonly points: readonly Vec3[]; readonly ageS: readonly number[]; readonly excessK: number; readonly vapourExcessKgPerKg: number; readonly condensateKgPerKg: number };

/** 🪨️ One obstacle of the air: a part of the evaluated body as the superellipsoid `(|q₁|ᵖ + |q₂|ᵖ + |q₃|ᵖ)^(1/p) = 1`, `q_i = axes_i·(x − centre)/radiiM_i` (world metres; exponent 2 an ellipsoid, 4 a rounded cylinder, 8 a rounded box). `blockage` is the share of the undisturbed normal flow it stops: 1 for a `solid` (a wall: no air crosses it, it hides what lies behind it),
 * less for a `lattice` (sparse solids such as the limbs of a crown) and for `foliage` (a porous canopy), which slow the air and veil what lies behind them. */
export type FlowObstacle = { readonly id: Slug; readonly kind: "solid" | "lattice" | "foliage"; readonly centre: Vec3; readonly axes: readonly [Vec3, Vec3, Vec3]; readonly radiiM: readonly [number, number, number]; readonly exponent: 2 | 4 | 8; readonly blockage: number };

/** 👁️ Where a flow is seen from: `toward` is the world direction from the scene to a far viewer (an orthographic drawing), `at` the world place of the eye (a perspective camera). */
export type FlowEye = { readonly toward: Vec3 } | { readonly at: Vec3 };

/** 🙈️ The visibility (0…1, see `FlowRibbon.visible`) up to which a mark counts as hidden behind the body and is not drawn by any renderer. */
export const FLOW_HIDDEN = 0.02;

/** 🌊️ The thermodynamic flow field at the evaluated instant (world metres): streams, puffs, the paths the parcels travelled (`paths`; empty when the solver was asked for no pathlines), the obstacles the air flows around (`obstacles`: the body of the asset at that instant; they also hide the marks behind them), the ambient air they are read against (`dewPointC` is the temperature below which that air condenses) and the tracer model (`stepM` arc-length step of the streams, a tracer every `periodS` seconds whose streak covers `exposureS` seconds of travel). */
export type FlowField = {
  readonly timeS: number;
  readonly streams: readonly FlowStream[];
  readonly puffs: readonly FlowPuff[];
  readonly paths: readonly FlowPath[];
  readonly obstacles: readonly FlowObstacle[];
  /** 📦️ The box the flow is shown in: streams end at its faces, puffs and paths outside it are left out. A drawing reserves its paper, so the frames of a sequence keep one extent. */
  readonly domain: { readonly min: Vec3; readonly max: Vec3 };
  readonly ambient: { readonly airTempC: number; readonly relativeHumidity: number; readonly dewPointC: number; readonly windMS: number; readonly windDirectionDeg: number; readonly pressurePa: number };
  readonly model: { readonly stepM: number; readonly periodS: number; readonly exposureS: number };
};

/** 🎗️ A drawn stream: its points with their state, how much of each point the eye of the marks sees past the obstacles (`visible`: 0 hidden behind the body, 1 in the open, between behind or inside foliage; all 1 without an eye) and the places and directions of its arrow heads. */
export type FlowRibbon = { readonly stream: number; readonly role: FlowStream["role"]; readonly emitter: Slug | null; readonly points: readonly Vec3[]; readonly speedMS: readonly number[]; readonly excessK: readonly number[]; readonly vapourExcessKgPerKg: readonly number[]; readonly condensateKgPerKg: readonly number[]; readonly visible: readonly number[]; readonly arrows: readonly { readonly at: Vec3; readonly direction: Vec3 }[] };

/** ☄️ A tracer of the instant: the path it covered during the exposure (tail first, head last), the state at its head, its fade (0…1) and how much of each point the eye sees (`visible`, as for a ribbon). */
export type FlowStreak = { readonly stream: number; readonly role: FlowStream["role"]; readonly points: readonly Vec3[]; readonly speedMS: number; readonly excessK: number; readonly vapourExcessKgPerKg: number; readonly condensateKgPerKg: number; readonly opacity: number; readonly visible: readonly number[] };

/** 👣️ A drawn path: the kept part of one {@link FlowPath} (`path` is its index in the field), oldest point first, with per point the seconds since the parcel was there (`agoS`, 0 at its head) and the fade 0…1 of a trail (1 everywhere without a trail; with a trail of `trailS` seconds `1 − agoS / trailS`, the first point interpolated to where the fade is 0), how much of each point the eye of the marks sees past the obstacles (`visible`, as for a ribbon) and the temperature excess the parcel carries now. */
export type FlowTrail = { readonly path: number; readonly parcel: string; readonly emitter: Slug; readonly points: readonly Vec3[]; readonly agoS: readonly number[]; readonly fade: readonly number[]; readonly visible: readonly number[]; readonly excessK: number };

/** ✨️ The generalised, view-neutral marks of a flow field at one time: what every drawing of the flow (paper, screen, 3D) projects; `puffVisible` is how much of each puff the eye of the marks sees past the obstacles (1 without an eye). */
export type FlowMarks = { readonly timeS: number; readonly ribbons: readonly FlowRibbon[]; readonly streaks: readonly FlowStreak[]; readonly puffs: readonly FlowPuff[]; readonly puffVisible: readonly number[]; readonly trails: readonly FlowTrail[] };

/** 🌪️ The conditions of the air a flow is solved in besides its environment: ambient turbulence (`intensity` = σ_u over the wind speed, `floorMS` the smallest σ_u, `lagrangianTimeS` the integral time scale), the wind profile (absent: uniform; else a logarithmic boundary layer of roughness length `z0M` that reaches the environment's wind speed at `referenceHeightM`), the life of a parcel `maxAgeS` (s) and the seed of the turbulent random walk. */
export type FlowConditions = { readonly turbulence?: { readonly intensity: number; readonly floorMS: number; readonly lagrangianTimeS: number }; readonly boundaryLayer?: { readonly z0M: number; readonly referenceHeightM: number }; readonly maxAgeS?: number; readonly seed?: number };

/** 🔎️ The generalisation and timing of flow marks on paper: the smallest spacing of streams and puffs and the shortest streak in paper millimetres, the seconds between two tracers of a stream, the seconds of travel a streak covers and `loopS`, the seconds after which the tracers of every stream repeat (the release interval of a stream is stretched until a whole number of them fits, so the frames of a loop of that length close; absent: no such period).
 * Paths: `spacingMm` thins them as it thins streams; `trailS` keeps only the last seconds of every path as a fading trail (absent: the whole path from the emission at full strength). Weight: `strokeMm` is the paper width of the widest stream in millimetres, every line of the flow scales with it (absent: the hairlines of a print, `FLOW_STYLE.streamMaxMm`). */
export type FlowMarkDetail = { readonly spacingMm?: number; readonly minStreakMm?: number; readonly periodS?: number; readonly exposureS?: number; readonly loopS?: number; readonly trailS?: number; readonly strokeMm?: number };

/** 🌊️ The layers of the thermodynamic flow view, the one vocabulary of drawn air: `streams` (stream ribbons with arrow heads), `tracers` (advected particles as streaks), `moisture` (vapour clouds), `fog` (condensed droplets where the dew point is crossed) and `pathlines` (the path each parcel actually travelled, the honest picture of unsteady flow such as breath). */
export const FLOW_LAYERS = ["streams", "tracers", "moisture", "fog", "pathlines"] as const;

/** 🌱️ The layers a thermal view shows unless told otherwise: everything but `pathlines`, whose turbulent paths are the analytical reading a reader asks for. */
export const DEFAULT_FLOW_LAYERS = ["streams", "tracers", "moisture", "fog"] as const satisfies readonly (typeof FLOW_LAYERS)[number][];

/** 📤️ One of {@link FLOW_LAYERS}. */
export type FlowLayer = (typeof FLOW_LAYERS)[number];
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

/** 🎛️ The three modes of a scale of the 3D model, each drawn from its own list of solids (`body`, `thermal`, `structure`): the unit of a scene. */
export type Mode = "body" | "thermal" | "structure";

/** 🧭️ The modes in scene order. */
export const MODES: readonly Mode[] = ["body", "thermal", "structure"];

/** 🗺️ The views each mode carries as variants of its geometry: the tone views of `body`, `thermal` of `thermal`, `forces` and `moments` of `structure`. */
export const VIEWS_OF: Readonly<Record<Mode, readonly View[]>> = { body: ["dark-line", "dark-solid", "light-line", "light-solid"], thermal: ["thermal"], structure: ["forces", "moments"] };

/** 🔎️ The mode (list of solids) a view is drawn from. */
export const modeOf = (view: View): Mode => (view === "thermal" ? "thermal" : view === "forces" || view === "moments" ? "structure" : "body");

/** 🏷️ The id of the kernel scene of a scale and mode: `s<scale>-<mode>`. */
export const sceneIdOf = (scale: number, mode: Mode): Slug => `s${scale}-${mode}`;

/** 📚️ The authored scales of `asset.volumes`, finest first, as denominators; no preset scale is authored when `volumes` is absent. */
export const authoredScales = (volumes: Readonly<Record<string, { readonly status: string; readonly denominator: number }>> | undefined): readonly number[] => (["50", "100", "200", "500"] as const).filter((key) => volumes?.[key]?.status === "authored").map(Number);

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

/** 🔭️ The three kinds of axonometric projection: `isometric` (the three world axes foreshortened equally, azimuth 45°, elevation 35.264°), `dimetric` (ISO 5456-3 1 : ½ : 1, elevation 19.471°, azimuth 69.295°: the ahead axis and the vertical axis true, the lateral axis half) and `free` (any azimuth and elevation). */
export const AXONOMETRIC_KINDS = ["isometric", "dimetric", "free"] as const;

/** 📤️ One of {@link AXONOMETRIC_KINDS}. */
export type AxonometricKind = (typeof AXONOMETRIC_KINDS)[number];

/** 👁️ Which orthographic axonometric camera the `axonometric` viewpoint uses; absent means `{ kind: "isometric" }`. `azimuthDeg` is the heading of the viewer from ahead (0°) towards the asset's right (90° is the side elevation, 180° from behind, negative from the left), `elevationDeg` the angle above the horizon (0° to 75°); the vertical axis is always drawn true to scale. */
export type AxonometricSpec = { readonly kind: "isometric" } | { readonly kind: "dimetric" } | { readonly kind: "free"; readonly azimuthDeg: number; readonly elevationDeg: number };

/** 🧱️ The depiction options that do not depend on whether text is rendered. `axonometric` chooses the camera of the `axonometric` viewpoint (default isometric); `shading` how the solid views of that viewpoint paint faces: `banded` (default) darkens a face of the tone colour in four bands by its angle to a fixed light, `flat` paints every face with the tone colour of its solid, as the planar views do. The analytic and line views are never shaded. */
export type DepictionCommon = {
  readonly view: View;
  readonly viewpoint: Viewpoint;
  readonly axonometric?: AxonometricSpec;
  readonly shading?: "banded" | "flat";
  readonly scale: DrawingScale;
  readonly transparent: boolean;
  readonly marginMm: number;
  readonly thermalQuantity: ThermalQuantity;
  /** 🌊️ The layers of the thermodynamic flow view drawn over the figure; absent or empty draws none. */
  readonly flow?: readonly FlowLayer[];
  /** ⏱️ Seconds the flow has developed before the query time when the depiction solves the transport itself (a pose at time 0 has emitted nothing yet); absent is 0. */
  readonly flowDevelopS?: number;
  /** 🌪️ The conditions of the air the flow is solved in besides the environment; absent uses the solver's defaults. */
  readonly flowConditions?: FlowConditions;
  /** 🔎️ How the flow marks are generalised and timed instead of the rule of the drawing scale; absent members keep that rule. */
  readonly flowDetail?: FlowMarkDetail;
  readonly profile: PrintProfile;
  readonly representation: RepresentationSelector;
  readonly forceQuantity: ForceQuantity;
  readonly range: DisplayRange;
  readonly patterns: boolean;
  /** 🧊️ The shape roles drawn (a level of detail): absent draws every role; the paper extent still reaches every shape of the representation. */
  readonly roles?: readonly ShapeRole[];
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

/** 🌟️ How a drawing relates to the real object: `physical` is metres scaled to paper millimetres (an authored scale representation or a `true-extent` symbol); `conventional-symbol` is a paper-millimetre sign (`true-position` or `nominal` symbol) whose size carries no dimension. Carried by `DrawingScene.metadata.fidelity` so every consumer can show it to the reader. */
export type Fidelity = "physical" | "conventional-symbol";

/** 🖨️ A paper-space scene at a given scale: what SVG and PDF writers serialise one to one. */
export type DrawingScene = {
  readonly widthMm: number;
  readonly heightMm: number;
  readonly background: string | null;
  readonly layers: readonly DrawingLayer[];
  readonly legends: readonly Legend[];
  readonly metadata: { readonly asset: Slug; readonly title: Text; readonly query: EvaluationQuery; readonly timeS: number; readonly view: View; readonly viewpoint: Viewpoint; readonly scale: DrawingScale; readonly resolution: RepresentationResolution["kind"]; readonly fidelity: Fidelity; readonly omittedValues: number; readonly representation: EvaluatedState["representation"]; readonly profile: Slug; readonly paperUnits: "mm"; readonly modelUnits: "m"; readonly mmPerMetre: number; readonly origin: { readonly xMm: number; readonly yMm: number }; readonly projection?: AxonometricProjection };
};

/** 🔄️ What the axonometric view of a scene is to scale: the kind and angles of the camera and, per world axis (ahead x, up y, left z), the paper length of one metre along that axis divided by `mmPerMetre` (1 = true to scale, ½ = foreshortened to half). Lengths along a world axis are read with that factor; no other direction is to scale. */
export type AxonometricProjection = { readonly kind: AxonometricKind; readonly azimuthDeg: number; readonly elevationDeg: number; readonly axisScale: { readonly x: number; readonly y: number; readonly z: number } };
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
