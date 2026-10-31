/** 🧍️ Typed twin of `🧬️schema/🔣️.json` (`semio.staffage/v2`): handcrafted architectural staffage with real dimensions, rigs, poses, clips,
 * individually authored drawing-scale representations, layered thermal models, respiration and structural models.
 *
 * Schema-first — `🔣️.json` is the single source of truth; this module restates every `$defs` entry under the same name with `readonly`
 * fields. No runtime dependency.
 *
 * Conventions (normative):
 * - Units are SI: metres, kilograms, seconds, newtons, pascals, watts, kelvin differences; temperatures are authored in °C.
 * - Model space is a right-handed elevation plane: x to the right, y up, origin on the ground at the asset's reference point.
 * - Angles are authored in degrees, counter-clockwise positive; a bone's local x axis points from its joint along the bone.
 * - Time is in seconds; frame n of a clip sits at t = n / fps; looping clips wrap the pose at `durationS`, root motion and air transport
 *   continue on the absolute timeline.
 * - Physical dimensions, drawing scale and viewport zoom are independent. The physical scenario (rig, masses, thermal, respiration, loads)
 *   is shared by every scale; only {@link Representation}s are scale-specific, and each is authored for exactly one preset denominator.
 * - Paper quantities (pens, hatch spacing, print profiles) are millimetres on paper; model quantities are metres.
 *
 * @see 🧬️schema/🔣️.json — the normative contract
 * @see ../README.md — the domain model
 */

//#region 🔖️Scalars
/** 🏷️ A kebab-case identifier, unique within its scope (`^[a-z0-9]+(?:-[a-z0-9]+)*$`). */
export type Slug = string;

/** 🌍️ A human-readable text in every supported language; there is no default language. */
export type Text = { readonly en: string; readonly de: string };

/** 🗣️ The languages every {@link Text} carries, English first. */
export const LANGUAGES = ["en", "de"] as const;

/** 🗣️ One of {@link LANGUAGES}. */
export type Language = (typeof LANGUAGES)[number];

/** 🔖️ The document version of every catalog file. */
export const SCHEMA_VERSION = "semio.staffage/v2" as const;

/** 🗂️ The eight archive categories. */
export const CATEGORIES = ["humans", "animals", "vegetation", "furniture", "objects", "vehicles", "machinery", "components"] as const;

/** 🗂️ One of {@link CATEGORIES}. */
export type CategoryId = (typeof CATEGORIES)[number];

/** 🧾️ How a physical parameter is known: measured, taken from literature, calibrated against literature data, estimated by the author, or purely illustrative. */
export const PARAMETER_SOURCES = ["measured", "literature", "calibrated", "authored-estimate", "illustrative"] as const;

/** 🧾️ One of {@link PARAMETER_SOURCES}. */
export type ParameterSource = (typeof PARAMETER_SOURCES)[number];

/** 👁️ The projection an authored geometry set is drawn in. */
export const VIEWPOINTS = ["elevation", "plan"] as const;

/** 👁️ One of {@link VIEWPOINTS}. */
export type Viewpoint = (typeof VIEWPOINTS)[number];

/** 📤️ The output kinds an asset can be exported to. */
export const EXPORT_FORMATS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"] as const;

/** 📤️ One of {@link EXPORT_FORMATS}. */
export type ExportFormat = (typeof EXPORT_FORMATS)[number];

/** ✅️ How far an asset has come: planned, authored data only, or every representation, view, pose, clip, model and export checked. */
export const COMPLETION = ["planned", "authored", "verified"] as const;

/** ✅️ One of {@link COMPLETION}. */
export type Completion = (typeof COMPLETION)[number];
//#endregion 🔖️Scalars

//#region 📚️Provenance
/** 📚️ A source a parameter or a shape relies on. */
export type Reference = {
  readonly id: Slug;
  readonly citation: string;
  readonly url?: string;
};

/** 🪪️ Who made an asset and how. */
export type Provenance = {
  readonly author: string;
  readonly method: "handcrafted";
  readonly created: string;
  readonly references: readonly Reference[];
  readonly notes?: Text;
};
//#endregion 📚️Provenance

//#region 🧱️Materials
/** 🧱️ A material with the properties the thermal and structural models read; absent properties are not used by any model. */
export type Material = {
  readonly id: Slug;
  readonly title: Text;
  readonly emissivity: number;
  readonly solarAbsorptivity?: number;
  readonly densityKgM3?: number;
  readonly conductivityWmK?: number;
  readonly specificHeatJKgK?: number;
  readonly elasticModulusPa?: number;
  readonly strengthPa?: number;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};
//#endregion 🧱️Materials

//#region 🦴️Rig
/** 🔩️ How a bone may move relative to its parent. */
export const JOINT_TYPES = ["fixed", "revolute", "prismatic"] as const;

/** 🔩️ One of {@link JOINT_TYPES}. */
export type JointType = (typeof JOINT_TYPES)[number];

/** 🔩️ A joint and its limits; revolute limits are angle offsets from rest, prismatic limits are offsets along `axisDeg` (bone-parent frame). */
export type Joint = {
  readonly type: JointType;
  readonly minDeg?: number;
  readonly maxDeg?: number;
  readonly axisDeg?: number;
  readonly minM?: number;
  readonly maxM?: number;
};

/** ⚖️ A segment mass: point mass at the local centre, or spread uniformly along the bone from 0 to `lengthM`. */
export type SegmentMass = {
  readonly kg: number;
  readonly comX: number;
  readonly comY: number;
  readonly distribution: "point" | "uniform";
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 📏️ A member cross-section for stress: area, second moment of area about the bending axis and extreme-fibre distance. */
export type Section = {
  readonly areaM2: number;
  readonly secondMomentM4: number;
  readonly extremeFibreM: number;
  readonly material: Slug;
  readonly title?: Text;
};

/** 🦴️ A bone: a rigid frame placed at (`x`, `y`) and rotated by `angleDeg` in its parent's frame at rest; its local x axis runs along it. */
export type Bone = {
  readonly id: Slug;
  readonly parent: Slug | null;
  readonly title: Text;
  readonly x: number;
  readonly y: number;
  readonly angleDeg: number;
  readonly lengthM: number;
  readonly joint: Joint;
  readonly mass?: SegmentMass;
  readonly section?: Section;
};

/** 🦴️ A transform hierarchy with exactly one root. `kind` names the articulation family. */
export type Rig = {
  readonly kind: "anatomical" | "botanical" | "mechanical" | "structural";
  readonly root: Slug;
  readonly bones: readonly Bone[];
};
//#endregion 🦴️Rig

//#region 📍️Anchors
/** 📍️ What an anchor is for. */
export const ANCHOR_KINDS = ["contact", "support", "attachment", "emitter", "load"] as const;

/** 📍️ One of {@link ANCHOR_KINDS}. */
export type AnchorKind = (typeof ANCHOR_KINDS)[number];

/** 🧷️ How a support restrains: unilateral ground contact (compression only, friction), pin, roller or fixed. */
export const SUPPORT_TYPES = ["contact", "pin", "roller", "fixed"] as const;

/** 🧷️ One of {@link SUPPORT_TYPES}. */
export type SupportType = (typeof SUPPORT_TYPES)[number];

/** 📍️ A point fixed on a bone (local coordinates); contacts and supports carry the footprint used for contact pressure. `normalDeg` is
 * the world direction a contact or roller restrains (default 90, vertical); for emitters `directionDeg` in the emitter gives the outflow. */
export type Anchor = {
  readonly id: Slug;
  readonly bone: Slug;
  readonly x: number;
  readonly y: number;
  readonly kind: AnchorKind;
  readonly title: Text;
  readonly support?: SupportType;
  readonly normalDeg?: number;
  readonly footprintLengthM?: number;
  readonly footprintWidthM?: number;
  readonly frictionCoefficient?: number;
};
//#endregion 📍️Anchors

//#region ✏️Geometry
/** 🖋️ The line-hierarchy role of a shape: silhouette, secondary detail, fine hatch/texture, transparent glazing or ground line. */
export const SHAPE_ROLES = ["body", "detail", "texture", "glazing", "ground"] as const;

/** 🖋️ One of {@link SHAPE_ROLES}. */
export type ShapeRole = (typeof SHAPE_ROLES)[number];

/** 💊️ A stadium between two local points with radii `r0` and `r1` (tangent hull of two circles). */
export type CapsuleShape = { readonly kind: "capsule"; readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number; readonly r0: number; readonly r1: number };

/** ⭕️ An ellipse in local coordinates. */
export type EllipseShape = { readonly kind: "ellipse"; readonly cx: number; readonly cy: number; readonly rx: number; readonly ry: number; readonly rotationDeg?: number };

/** ⬛️ A rectangle with optional corner radius, lower-left at (`x`, `y`). */
export type RectShape = { readonly kind: "rect"; readonly x: number; readonly y: number; readonly w: number; readonly h: number; readonly r?: number };

/** 🔷️ A polygon or polyline through local points. */
export type PolyShape = { readonly kind: "poly"; readonly points: readonly (readonly [number, number])[]; readonly closed: boolean };

/** 〰️ An SVG-style path (`M L H V C Q A Z`, absolute or relative) in local metres, y up. */
export type PathShape = { readonly kind: "path"; readonly d: string };

/** ✏️ One of the shape primitives. */
export type ShapeGeometry = CapsuleShape | EllipseShape | RectShape | PolyShape | PathShape;

/** ✏️ An authored shape of one representation, bound to a bone. `region` binds it to one thermal region; a shape that merges several
 * surfaces at a small scale lists them in `regions` instead and is painted with their area-weighted value (labelled as merged). `hatch`
 * names a hatch of the representation drawn inside the shape. `pen` overrides the representation's pen of the role (paper mm). */
export type Shape = ShapeGeometry & {
  readonly id: Slug;
  readonly bone: Slug;
  readonly role: ShapeRole;
  readonly z: number;
  readonly filled: boolean;
  readonly tone: number;
  readonly region?: Slug;
  readonly regions?: readonly Slug[];
  readonly material?: Slug;
  readonly hatch?: Slug;
  readonly penMm?: number;
};
//#endregion ✏️Geometry

//#region 🖨️Representations
/** 🖨️ The preset drawing-scale denominators, smallest scale first; every asset decides each of them explicitly. */
export const SCALE_DENOMINATORS = [500, 200, 100, 50] as const;

/** 🖨️ One of {@link SCALE_DENOMINATORS}. */
export type ScaleDenominator = (typeof SCALE_DENOMINATORS)[number];

/** 🔑️ The key of a representation in {@link Asset.representations}: the denominator as a string. */
export type RepresentationKey = `${ScaleDenominator}`;

/** ✒️ The pens of a representation per role, in paper millimetres; `null` means the role is not drawn at this scale. */
export type PenSet = { readonly body: number; readonly detail: number | null; readonly texture: number | null; readonly glazing: number | null; readonly ground: number };

/** ▤️ A hatch drawn inside shapes that name it: parallel lines, cross-hatch, dots or stipple, spacing and pen in paper millimetres. */
export type Hatch = { readonly id: Slug; readonly title: Text; readonly pattern: "lines" | "cross" | "dots" | "stipple"; readonly angleDeg: number; readonly spacingMm: number; readonly penMm: number };

/** 📝️ The authored design decisions of one representation, reviewed per asset and scale. */
export type RepresentationDecisions = {
  readonly silhouette: Text;
  readonly retained: Text;
  readonly merged: Text;
  readonly omitted: Text;
  readonly thinElements: Text;
  readonly hatching: Text;
  readonly styles: Text;
  readonly overlays: Text;
};

/** 🖨️ A representation individually authored for one preset scale. `validRange` is the range of custom denominators this
 * representation is validated for (it always contains its own denominator); custom scales inside it render it as a derived use. */
export type AuthoredRepresentation = {
  readonly status: "authored";
  readonly denominator: ScaleDenominator;
  readonly validRange: { readonly minDenominator: number; readonly maxDenominator: number };
  readonly pens: PenSet;
  readonly hatches: readonly Hatch[];
  readonly geometry: { readonly elevation: readonly Shape[]; readonly plan?: readonly Shape[] };
  readonly decisions: RepresentationDecisions;
};

/** ↪️ What to offer instead of an unavailable representation: another scale of the asset, a symbol of the asset, or another asset. */
export type Alternative = { readonly kind: "scale"; readonly denominator: ScaleDenominator } | { readonly kind: "symbol"; readonly symbol: Slug } | { readonly kind: "asset"; readonly asset: Slug };

/** 🚫️ An explicit decision that the asset cannot be meaningfully drawn at this scale: why, which feature limits it and at what real size,
 * and what to use instead (at least one alternative). */
export type UnavailableRepresentation = {
  readonly status: "unavailable";
  readonly denominator: ScaleDenominator;
  readonly reason: Text;
  readonly limitingFeature: Text;
  readonly limitingSizeM: number;
  readonly alternatives: readonly Alternative[];
};

/** 🖨️ The decision for one preset scale. */
export type Representation = AuthoredRepresentation | UnavailableRepresentation;

/** 🔣️ A conventional symbol, separately identified from the pictorial representations. Its dimensional semantics are explicit:
 * `true-extent` geometry is model metres at real size; `true-position` marks a real position (its insertion point at the model origin)
 * with geometry in paper millimetres; `nominal` is a fixed-size paper sign in paper millimetres whose size carries no dimension. */
export type SymbolRepresentation = {
  readonly id: Slug;
  readonly title: Text;
  readonly description: Text;
  readonly semantics: "true-extent" | "true-position" | "nominal";
  readonly units: "model-m" | "paper-mm";
  readonly denominators: readonly ScaleDenominator[];
  readonly pens: PenSet;
  readonly hatches: readonly Hatch[];
  readonly geometry: { readonly elevation: readonly Shape[]; readonly plan?: readonly Shape[] };
  readonly dimensionalNote: Text;
};

/** 🖨️ A print profile: the smallest stroke, gap between separate outlines, feature width and hatch spacing that reliably print. */
export type PrintProfile = {
  readonly id: Slug;
  readonly title: Text;
  readonly description: Text;
  readonly minStrokeMm: number;
  readonly minGapMm: number;
  readonly minFeatureMm: number;
  readonly minHatchSpacingMm: number;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🖨️ The print profile document `🗂️catalog/🖨️print-profiles/🔣️.json`. */
export type PrintProfileDocument = { readonly $schema: string; readonly version: typeof SCHEMA_VERSION; readonly defaultProfile: Slug; readonly profiles: readonly PrintProfile[]; readonly references: readonly Reference[] };
//#endregion 🖨️Representations

//#region 🧍️Poses
/** 🎚️ An offset from rest for one bone: rotation delta and translation delta along the parent frame (prismatic joints). */
export type BoneOffset = { readonly angleDeg?: number; readonly dx?: number; readonly dy?: number };

/** 🧲️ A load applied at an anchor or a bone point, in world newtons; `distributed` spreads `wyNPerM` along a bone segment instead. */
export type AppliedLoad = {
  readonly id: Slug;
  readonly kind: "carried" | "imposed" | "live" | "wind" | "snow" | "occupant" | "equipment";
  readonly title: Text;
  readonly anchor?: Slug;
  readonly bone?: Slug;
  readonly x?: number;
  readonly y?: number;
  readonly fxN: number;
  readonly fyN: number;
  readonly momentNm?: number;
  readonly distributed?: { readonly bone: Slug; readonly fromM: number; readonly toM: number; readonly wxNPerM: number; readonly wyNPerM: number };
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🌡️ The region state a pose binds: share of the area exposed to air, share of the area in sun, whether the surface is wetted, and the
 * conductive contact (seat, ground) it rests on. */
export type RegionBinding = { readonly exposedFraction: number; readonly sunlitFraction: number; readonly wettedness?: number; readonly contact?: { readonly fraction: number; readonly tempC: number } };

/** 🌡️ The thermal and respiratory state a pose or clip key binds explicitly: activity, external work, operating levels of machines,
 * the breathing pattern, every region and every emitter. Bodies bind `activityMet` and `breathing`; every pose binds every region and
 * every non-respiratory emitter (strength 0…1) of its asset. */
export type ThermalBinding = {
  readonly activityMet?: number;
  readonly externalWorkW?: number;
  readonly operating?: Readonly<Record<Slug, number>>;
  readonly breathing?: Slug;
  readonly passengers?: Readonly<Record<Slug, { readonly activityMet: number; readonly breathing?: Slug }>>;
  readonly regions: Readonly<Record<Slug, RegionBinding>>;
  readonly emitters: Readonly<Record<Slug, number>>;
};

/** 🧍️ A configuration: bone offsets, root placement, active contacts, thermal binding and loads. `key: true` marks an in-between key
 * pose of a clip, evaluable like any pose but not offered as a named pose; every asset has at least three named poses. */
export type Pose = {
  readonly id: Slug;
  readonly title: Text;
  readonly description: Text;
  readonly key?: boolean;
  readonly bones: Readonly<Record<Slug, BoneOffset>>;
  readonly root?: { readonly x?: number; readonly y?: number; readonly angleDeg?: number };
  readonly ground: "auto" | "authored";
  readonly contacts: readonly Slug[];
  readonly thermal: ThermalBinding;
  readonly loads: readonly AppliedLoad[];
};
//#endregion 🧍️Poses

//#region 🎞️Clips
/** 📈️ How a key eases into the next one: `smooth` is a C1 cubic Hermite (Catmull-Rom tangents from the neighbouring keys, wrapped for
 * looping clips, zero at the ends of non-looping ones); `linear` constant rate; `in`/`out` accelerate/decelerate; `step` holds. */
export const EASES = ["linear", "smooth", "in", "out", "step"] as const;

/** 📈️ One of {@link EASES}. */
export type Ease = (typeof EASES)[number];

/** 🔑️ A clip key: at `t` the clip shows `pose` with optional bone overrides; `ease` shapes the way to the next key. */
export type ClipKey = { readonly t: number; readonly pose: Slug; readonly ease: Ease; readonly bones?: Readonly<Record<Slug, BoneOffset>> };

/** 🦶️ A time span during which an anchor is in contact; planted contacts do not slide. */
export type ContactSpan = { readonly anchor: Slug; readonly fromS: number; readonly toS: number };

/** 🎬️ A named action phase; `ballistic` declares an airborne phase in which instants without any contact are legitimate. */
export type Phase = { readonly id: Slug; readonly fromS: number; readonly toS: number; readonly title: Text; readonly ballistic?: boolean };

/** ⚡️ What a clip event marks. Respiratory events are derived from the breathing pattern, not authored. */
export const EVENT_KINDS = ["contact-on", "contact-off", "ignition", "shutdown", "load-on", "load-off", "thermal", "motion", "operating"] as const;

/** 📌️ A point event on the clip timeline. */
export type ClipEvent = { readonly t: number; readonly kind: (typeof EVENT_KINDS)[number]; readonly title: Text; readonly target?: Slug };

/** 🎛️ A scalar track of a clip: `activityMet`, `externalWorkW`, `emitter:<id>`, `operating:<id>`, `load:<id>` (scale factor),
 * `region:<id>:<field>` (a {@link RegionBinding} number field) or `breathing` (index into `respiration.patterns`, eased `step`). */
export type ScalarTrack = { readonly target: string; readonly ease: Ease; readonly keys: readonly { readonly t: number; readonly v: number }[] };

/** 🎞️ A deterministic clip over named poses. `rootMotion: "plant"` solves the root so planted contacts stay fixed in world space; for a
 * looping clip each cycle advances the root by `advanceM` on the absolute timeline. */
export type Clip = {
  readonly id: Slug;
  readonly title: Text;
  readonly description: Text;
  readonly durationS: number;
  readonly fps: number;
  readonly loop: boolean;
  readonly rootMotion: "none" | "plant";
  readonly advanceM?: number;
  readonly keys: readonly ClipKey[];
  readonly contacts: readonly ContactSpan[];
  readonly phases: readonly Phase[];
  readonly events: readonly ClipEvent[];
  readonly tracks: readonly ScalarTrack[];
};
//#endregion 🎞️Clips

//#region 🌡️Thermal
/** 🌤️ Ambient conditions. `solarWM2` is the irradiance on a surface normal to the sun; sunlit areas receive it, shaded areas only the
 * diffuse share `diffuseFraction`. `skyTempC` and `groundTempC` refine the radiant environment of up- and down-facing areas; absent they
 * equal `meanRadiantTempC`. The asset states its reference case; the viewer may choose any preset or custom environment. */
export type Environment = {
  readonly airTempC: number;
  readonly meanRadiantTempC: number;
  readonly skyTempC?: number;
  readonly groundTempC?: number;
  readonly relativeHumidity: number;
  readonly windMS: number;
  readonly windDirectionDeg: number;
  readonly solarWM2: number;
  readonly diffuseFraction: number;
  readonly pressurePa: number;
};

/** 🌤️ A named environment preset of `🗂️catalog/🌤️environments/🔣️.json`, shared by every asset so results stay comparable. */
export type EnvironmentPreset = { readonly id: Slug; readonly title: Text; readonly description: Text; readonly environment: Environment; readonly source: ParameterSource; readonly reference?: Slug };

/** 🌤️ The environment preset document. */
export type EnvironmentDocument = { readonly $schema: string; readonly version: typeof SCHEMA_VERSION; readonly presets: readonly EnvironmentPreset[]; readonly references: readonly Reference[] };

/** 🧮️ How a region's surface temperature is obtained.
 * - `body`: a living body segment; heat flows from the regulated core through tissue (conductance driven by skin blood flow) to the skin,
 *   then through the region's `layers` (clothing, fur, feathers, trapped air) to the outer surface.
 * - `leaf`: leaf energy balance with stomatal conductance (latent heat by transpiration).
 * - `passive`: solar absorption, radiation, convection and contact conduction of an inert object, with heat capacity.
 * - `operating`: an inert surface heated from inside by `operating.heatW × level` (engines, motors, lamps, heaters, exhausts).
 * - `conditioned`: a building element separating the environment from an interior at `conditioned.interiorTempC` through its `layers`.
 * - `fixed`: a prescribed surface temperature (only for reference fixtures). */
export const THERMAL_MODELS = ["body", "leaf", "passive", "operating", "conditioned", "fixed"] as const;

/** 🧮️ One of {@link THERMAL_MODELS}. */
export type ThermalModelKind = (typeof THERMAL_MODELS)[number];

/** 🧶️ One layer of a region, innermost first: thermal resistance, vapour resistance and areal heat capacity, all referred to the
 * region's outer `areaM2` (an intrinsic clothing insulation I_cl per skin area enters as I_cl · f_cl). */
export type ThermalLayer = {
  readonly id: Slug;
  readonly title: Text;
  readonly kind: "tissue" | "clothing" | "fur" | "feather" | "hair" | "air-gap" | "shell" | "insulation" | "glass" | "masonry" | "timber" | "metal" | "polymer";
  readonly material: Slug;
  readonly thicknessM?: number;
  readonly resistanceM2KW: number;
  readonly vapourResistanceM2PaW?: number;
  readonly heatCapacityJM2K?: number;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🔥️ A thermal region: one surface patch with its area, shape for convection correlations, layers and model parameters. The outermost
 * layer's material (or `material` when there are no layers) gives emissivity and solar absorptivity. `hidden` explains why a region
 * facing away from the viewer is not drawn in elevation (e.g. a canopy underside); every other region is drawn at every authored scale. */
export type ThermalRegion = {
  readonly id: Slug;
  readonly title: Text;
  readonly kind: "skin" | "clothing" | "hair" | "fur" | "feather" | "foliage" | "bark" | "stem" | "surface" | "engine" | "exhaust" | "glazing" | "tyre" | "wall" | "heater" | "lamp" | "water";
  readonly model: ThermalModelKind;
  readonly material: Slug;
  readonly areaM2: number;
  readonly characteristicLengthM: number;
  readonly geometry: "cylinder" | "sphere" | "vertical-plate" | "horizontal-plate-up" | "horizontal-plate-down";
  readonly skyViewFactor: number;
  readonly layers: readonly ThermalLayer[];
  readonly heatCapacityJM2K: number;
  readonly body?: { readonly segment: "head" | "trunk" | "arm" | "hand" | "leg" | "foot" | "tail" | "wing"; readonly metabolicShare: number; readonly skinFraction: number; readonly subject?: Slug };
  readonly leaf?: { readonly stomatalConductanceMaxMS: number; readonly leafAreaIndex: number; readonly photosynthesis: "c3" | "c4" | "cam" };
  readonly operating?: { readonly key: Slug; readonly heatW: number };
  readonly conditioned?: { readonly interiorTempC: number; readonly interiorFilmM2KW: number };
  readonly fixed?: { readonly surfaceTempC: number };
  readonly hidden?: Text;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🫀️ The whole-body thermal physiology of a human or animal: size, basal metabolism, regulated set points and thermoregulatory limits. */
export type BodyPhysiology = {
  readonly species: "human" | "dog" | "cat" | "horse" | "cow" | "deer" | "bird" | "sheep" | "goat" | "pig" | "fox" | "squirrel" | "hedgehog" | "duck" | "gull";
  readonly massKg: number;
  readonly skinAreaM2: number;
  readonly basalMetabolismW: number;
  readonly metWM2: number;
  readonly coreSetPointC: number;
  readonly skinSetPointC: number;
  readonly sweating: boolean;
  readonly panting: boolean;
  readonly maxSkinBloodFlowLM2H: number;
  readonly minSkinBloodFlowLM2H: number;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🫁️ A breathing pattern: frequency, tidal volume and the split of one cycle into inhalation, pause, exhalation and pause (fractions
 * summing to 1, in that order). Flow during inhalation and exhalation is a half-sine whose integral equals the tidal volume. */
export type BreathingPattern = {
  readonly id: Slug;
  readonly title: Text;
  readonly rateBpm: number;
  readonly tidalVolumeL: number;
  readonly inhaleFraction: number;
  readonly pauseAfterInhaleFraction: number;
  readonly exhaleFraction: number;
  readonly pauseAfterExhaleFraction: number;
  readonly route: "nose" | "mouth" | "nose-and-mouth";
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🫁️ Respiration of a body: the nostril and mouth emitters (emitter anchors on the head), exhaled air state, the patterns poses bind,
 * and the chest/abdomen bones that move with lung volume (`angleDeg` and `dy` at one tidal volume above functional residual capacity). */
export type Respiration = {
  readonly noseEmitter?: Slug;
  readonly mouthEmitter?: Slug;
  readonly exhaledTempC: number;
  readonly exhaledRelativeHumidity: number;
  readonly patterns: readonly BreathingPattern[];
  readonly motion: readonly { readonly bone: Slug; readonly angleDeg: number; readonly dy: number }[];
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🧒️ A second living body carried by the asset (a toddler in a stroller or on a hip, a rider): its own physiology and respiration;
 * its regions name it in `body.subject` (regions without `subject` belong to the asset's primary `body`). Poses bind its activity and
 * breathing pattern in `ThermalBinding.passengers`. */
export type Passenger = { readonly id: Slug; readonly title: Text; readonly physiology: BodyPhysiology; readonly respiration?: Respiration };

/** 💨️ A rig-attached source of air or vapour. `breath` emitters take flow, temperature and humidity from {@link Respiration}; `exhaust`,
 * `vent`, `flue` and `fan` scale `volumeFlowM3S`, `temperatureRiseK` and `vapourKgPerKg` with the operating level of `operatingKey`;
 * `transpiration` releases the latent flux of its foliage `region` as vapour, spread over `anchors`; `plume` is the free convection
 * above a warm surface `region`. */
export type Emitter = {
  readonly id: Slug;
  readonly kind: "breath" | "exhaust" | "vent" | "flue" | "fan" | "transpiration" | "plume";
  readonly title: Text;
  readonly anchor: Slug;
  readonly anchors?: readonly Slug[];
  readonly directionDeg: number;
  readonly diameterM: number;
  readonly operatingKey?: Slug;
  readonly volumeFlowM3S?: number;
  readonly temperatureRiseK?: number;
  readonly vapourKgPerKg?: number;
  readonly region?: Slug;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** ⚙️ An operating state family of a machine: its key, title and the named levels (0 = off, 1 = rated). */
export type OperatingKey = { readonly id: Slug; readonly title: Text; readonly levels: readonly { readonly v: number; readonly title: Text }[] };

/** 🌡️ The thermal model of an asset. */
export type ThermalModel = {
  readonly environment: Environment;
  readonly baseMet?: number;
  readonly body?: BodyPhysiology;
  readonly respiration?: Respiration;
  readonly passengers: readonly Passenger[];
  readonly operating: readonly OperatingKey[];
  readonly regions: readonly ThermalRegion[];
  readonly emitters: readonly Emitter[];
  readonly assumptions: readonly Text[];
};
//#endregion 🌡️Thermal

//#region 🏗️Structure
/** 🏗️ The structural model: gravity, dynamic treatment, support policy and assumptions; masses and sections live on bones. */
export type StructuralModel = {
  readonly gravityMS2: number;
  readonly dynamics: "quasi-static" | "inertial";
  readonly supports: readonly Slug[];
  readonly assumptions: readonly Text[];
  readonly clinical: false;
};
//#endregion 🏗️Structure

//#region 🧍️Asset
/** 📐️ Nominal overall real dimensions (bounding box of the default pose) and total mass; every authored representation reproduces them. */
export type Dimensions = { readonly heightM: number; readonly widthM: number; readonly depthM: number; readonly massKg: number };

/** 🖼️ Presentation defaults. */
export type Presentation = { readonly defaultPose: Slug; readonly defaultClip: Slug; readonly defaultScale: ScaleDenominator; readonly viewpoints: readonly Viewpoint[] };

/** 🧍️ One handcrafted staffage asset. The physical scenario is scale-independent; `representations` decides every preset scale. */
export type Asset = {
  readonly id: Slug;
  readonly category: CategoryId;
  readonly title: Text;
  readonly summary: Text;
  readonly tags: readonly Slug[];
  readonly provenance: Provenance;
  readonly dimensions: Dimensions;
  readonly materials: readonly Material[];
  readonly rig: Rig;
  readonly anchors: readonly Anchor[];
  readonly representations: Readonly<Record<RepresentationKey, Representation>>;
  readonly symbols: readonly SymbolRepresentation[];
  readonly poses: readonly Pose[];
  readonly clips: readonly Clip[];
  readonly thermal: ThermalModel;
  readonly structure: StructuralModel;
  readonly presentation: Presentation;
  readonly exports: readonly ExportFormat[];
  readonly completion: Completion;
};

/** 🗂️ One asset file: `🗂️catalog/<category folder>/<asset id>/🔣️.json`. */
export type AssetDocument = { readonly $schema: string; readonly version: typeof SCHEMA_VERSION; readonly asset: Asset };
//#endregion 🧍️Asset

//#region 📋️Manifest
/** 📋️ A manifest row: the authoring plan and status of one asset, checked against the asset document. `representations` lists the
 * status per preset scale. */
export type ManifestEntry = {
  readonly id: Slug;
  readonly category: CategoryId;
  readonly title: Text;
  readonly heightM: number;
  readonly widthM: number;
  readonly poses: readonly Slug[];
  readonly clips: readonly Slug[];
  readonly representations: Readonly<Record<RepresentationKey, "authored" | "unavailable">>;
  readonly symbols: readonly Slug[];
  readonly analytics: readonly ("thermal" | "transport" | "structural")[];
  readonly viewpoints: readonly Viewpoint[];
  readonly exports: readonly ExportFormat[];
  readonly distinction: Text;
  readonly completion: Completion;
};

/** 🗂️ A category of the archive with its folder of asset folders. */
export type CategoryEntry = { readonly id: CategoryId; readonly emoji: string; readonly title: Text; readonly description: Text; readonly folder: string };

/** 📋️ The archive manifest `🗂️catalog/🔣️.json`. */
export type Manifest = {
  readonly $schema: string;
  readonly version: typeof SCHEMA_VERSION;
  readonly categories: readonly CategoryEntry[];
  readonly collections: readonly { readonly id: Slug; readonly title: Text; readonly description: Text; readonly assets: readonly Slug[] }[];
  readonly entries: readonly ManifestEntry[];
};
//#endregion 📋️Manifest
