/** 🧍️ Typed twin of `🧬️schema/🔣️.json` (`semio.staffage/v1`): handcrafted architectural staffage with real dimensions, rigs, poses, clips,
 * thermal and structural models.
 *
 * Schema-first — `🔣️.json` is the single source of truth; this module restates every `$defs` entry under the same name with `readonly`
 * fields. No runtime dependency.
 *
 * Conventions (normative):
 * - Units are SI: metres, kilograms, seconds, newtons, pascals, watts, kelvin differences; temperatures are authored in °C.
 * - Model space is a right-handed elevation plane: x to the right, y up, origin on the ground at the asset's reference point.
 * - Angles are authored in degrees, counter-clockwise positive; a bone's local x axis points from its joint along the bone.
 * - Time is in seconds; frame n of a clip sits at t = n / fps; looping clips wrap t modulo `durationS`.
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
export const SCHEMA_VERSION = "semio.staffage/v1" as const;

/** 🗂️ The six archive categories. */
export const CATEGORIES = ["humans", "vegetation", "animals", "objects", "vehicles", "components"] as const;

/** 🗂️ One of {@link CATEGORIES}. */
export type CategoryId = (typeof CATEGORIES)[number];

/** 🧾️ How a physical parameter is known: measured, taken from literature, estimated by the author, or purely illustrative. */
export const PARAMETER_SOURCES = ["measured", "literature", "authored-estimate", "illustrative"] as const;

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

/** ✅️ How far an asset has come: authored data only, or every view, pose, clip, model and export checked. */
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
 * the world direction a contact or roller restrains (default 90, vertical); a roller along a tie rod restrains along the rod. */
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

/** 🔭️ The level of detail a shape needs: 0 always; 1 at 1:200 and larger; 2 at 1:100 and larger; 3 at 1:50 and larger. */
export type Lod = 0 | 1 | 2 | 3;

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

/** ✏️ An authored shape bound to a bone; its thermal region and material give it physical meaning. */
export type Shape = ShapeGeometry & {
  readonly id: Slug;
  readonly bone: Slug;
  readonly role: ShapeRole;
  readonly lod: Lod;
  readonly z: number;
  readonly filled: boolean;
  readonly tone: number;
  readonly region?: Slug;
  readonly material?: Slug;
};
//#endregion ✏️Geometry

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

/** 🌡️ The thermal state a pose or clip key binds: metabolic or operating level, region overrides and emitter strengths. */
export type ThermalBinding = {
  readonly activityMet?: number;
  readonly operating?: Readonly<Record<Slug, number>>;
  readonly regions?: Readonly<Record<Slug, { readonly exposedFraction?: number; readonly surfaceTempC?: number; readonly coreTempC?: number; readonly wettedness?: number }>>;
  readonly emitters?: Readonly<Record<Slug, number>>;
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

/** ⚡️ What a clip event marks. */
export const EVENT_KINDS = ["contact-on", "contact-off", "inhale", "exhale", "ignition", "shutdown", "load-on", "load-off", "thermal", "motion"] as const;

/** 📌️ A point event on the clip timeline. */
export type ClipEvent = { readonly t: number; readonly kind: (typeof EVENT_KINDS)[number]; readonly title: Text; readonly target?: Slug };

/** 🎛️ A scalar track of a clip: `activityMet`, `emitter:<id>`, `operating:<id>`, `load:<id>` (scale factor) or `region:<id>:<field>`. */
export type ScalarTrack = { readonly target: string; readonly ease: Ease; readonly keys: readonly { readonly t: number; readonly v: number }[] };

/** 🎞️ A deterministic clip over named poses. `rootMotion: "plant"` solves the root so planted contacts stay fixed in world space. */
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
/** 🌤️ Ambient conditions; the asset states its reference case, the viewer may change them. */
export type Environment = {
  readonly airTempC: number;
  readonly meanRadiantTempC: number;
  readonly relativeHumidity: number;
  readonly windMS: number;
  readonly windDirectionDeg: number;
  readonly solarWM2: number;
  readonly pressurePa: number;
};

/** 🧮️ How a region's surface temperature is obtained.
 * - `fixed`: `surfaceTempC` is given.
 * - `core-resistance`: steady state between `coreTempC` and the environment through `insulationM2KW` (skin, clothing, fur, walls).
 * - `leaf`: leaf energy balance with stomatal conductance (latent heat by transpiration, Penman–Monteith form).
 * - `passive`: solar absorption balanced by convection and radiation (inert objects).
 * - `operating`: ambient plus `riseK` times the operating level of `operatingKey` (engines, motors, lamps). */
export const THERMAL_MODELS = ["fixed", "core-resistance", "leaf", "passive", "operating"] as const;

/** 🧮️ One of {@link THERMAL_MODELS}. */
export type ThermalModelKind = (typeof THERMAL_MODELS)[number];

/** 🔥️ A thermal region: one surface patch with its material, area and model parameters. */
export type ThermalRegion = {
  readonly id: Slug;
  readonly title: Text;
  readonly kind: "skin" | "clothing" | "hair" | "fur" | "feather" | "foliage" | "bark" | "surface" | "engine" | "exhaust" | "glazing" | "tyre" | "wall";
  readonly material: Slug;
  readonly model: ThermalModelKind;
  readonly areaM2: number;
  readonly characteristicLengthM: number;
  readonly surfaceTempC?: number;
  readonly coreTempC?: number;
  readonly insulationM2KW?: number;
  readonly wettedness?: number;
  readonly stomatalConductanceMS?: number;
  readonly operatingKey?: Slug;
  readonly riseK?: number;
  readonly metabolicShare?: number;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 💨️ A rig-attached source of air or vapour: breath, exhaust, transpiration, vent or buoyant plume. */
export type Emitter = {
  readonly id: Slug;
  readonly kind: "breath" | "exhaust" | "transpiration" | "vent" | "plume";
  readonly title: Text;
  readonly anchor: Slug;
  readonly directionDeg: number;
  readonly carries: "airflow" | "vapour";
  readonly temperatureC: number;
  readonly velocityMS: number;
  readonly diameterM: number;
  readonly vapourKgPerKg?: number;
  readonly periodS?: number;
  readonly dutyFrom?: number;
  readonly dutyTo?: number;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🌡️ The thermal model of an asset. */
export type ThermalModel = {
  readonly environment: Environment;
  readonly baseMet?: number;
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
/** 📐️ Nominal overall real dimensions (bounding box of the default pose) and total mass. */
export type Dimensions = { readonly heightM: number; readonly widthM: number; readonly depthM: number; readonly massKg: number };

/** 🖼️ Presentation defaults. */
export type Presentation = { readonly defaultPose: Slug; readonly defaultClip: Slug; readonly viewpoints: readonly Viewpoint[] };

/** 🧍️ One handcrafted staffage asset. */
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
  readonly geometry: { readonly elevation: readonly Shape[]; readonly plan?: readonly Shape[] };
  readonly poses: readonly Pose[];
  readonly clips: readonly Clip[];
  readonly thermal: ThermalModel;
  readonly structure: StructuralModel;
  readonly presentation: Presentation;
  readonly exports: readonly ExportFormat[];
  readonly completion: Completion;
};

/** 🗂️ One category file: `🗂️catalog/<category>/🔣️.json`. */
export type CategoryDocument = { readonly $schema: string; readonly version: typeof SCHEMA_VERSION; readonly category: CategoryId; readonly assets: readonly Asset[] };
//#endregion 🧍️Asset

//#region 📋️Manifest
/** 📋️ A manifest row: the authoring plan and status of one asset, checked against the asset document. */
export type ManifestEntry = {
  readonly id: Slug;
  readonly category: CategoryId;
  readonly title: Text;
  readonly heightM: number;
  readonly widthM: number;
  readonly poses: readonly Slug[];
  readonly clips: readonly Slug[];
  readonly analytics: readonly ("thermal" | "structural")[];
  readonly viewpoints: readonly Viewpoint[];
  readonly exports: readonly ExportFormat[];
  readonly distinction: Text;
  readonly completion: Completion;
};

/** 🗂️ A category of the archive with its file. */
export type CategoryEntry = { readonly id: CategoryId; readonly emoji: string; readonly title: Text; readonly description: Text; readonly file: string };

/** 📋️ The archive manifest `🗂️catalog/🔣️.json`. */
export type Manifest = {
  readonly $schema: string;
  readonly version: typeof SCHEMA_VERSION;
  readonly categories: readonly CategoryEntry[];
  readonly collections: readonly { readonly id: Slug; readonly title: Text; readonly description: Text; readonly assets: readonly Slug[] }[];
  readonly entries: readonly ManifestEntry[];
};
//#endregion 📋️Manifest
