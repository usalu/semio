/** 🧍️ Typed twin of `🧬️schema/🔣️.json` (`semio.staffage/v2`): handcrafted architectural staffage with real dimensions, rigs, poses, clips,
 * individually authored drawing-scale representations, layered thermal models, respiration and structural models.
 *
 * Schema-first — `🔣️.json` is the single source of truth; this module restates every `$defs` entry under the same name with `readonly`
 * fields. No runtime dependency.
 *
 * Conventions (normative):
 * - Units are SI: metres, kilograms, seconds, newtons, pascals, watts, kelvin differences; temperatures are authored in °C.
 * - World frame (asset-local and scene frame are one): x ahead, y up, z lateral with the left of the asset at +z, a left-handed triple; origin on the ground at the asset's reference point. The side elevation shows the x-y plane seen from the asset's right with ahead on the right of the paper.
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

/** 🌐️ One of {@link LANGUAGES}. */
export type Language = (typeof LANGUAGES)[number];

/** 🔖️ The document version of every catalog file. */
export const SCHEMA_VERSION = "semio.staffage/v2" as const;

/** 🗂️ The nine archive categories. */
export const CATEGORIES = ["humans", "animals", "vegetation", "furniture", "appliances", "objects", "vehicles", "machinery", "components"] as const;

/** 📁️ One of {@link CATEGORIES}. */
export type CategoryId = (typeof CATEGORIES)[number];

/** 🧾️ How a physical parameter is known: measured, taken from literature, calibrated against literature data, estimated by the author, or purely illustrative. */
export const PARAMETER_SOURCES = ["measured", "literature", "calibrated", "authored-estimate", "illustrative"] as const;

/** 🧪️ One of {@link PARAMETER_SOURCES}. */
export type ParameterSource = (typeof PARAMETER_SOURCES)[number];

/** 👁️ The projection an authored geometry set is drawn in. `elevation` is the side elevation, the rig plane (paper x = world x ahead, paper up = y), seen from the asset's right (-z). `front` is the front elevation, seen from ahead (+x): paper x = world z (the asset's left is paper right), paper up = y. `plan` is the plan (top view), seen from above (+y): paper x = world x, paper up = world z (the asset's left is up the page). Front and plan shapes are authored in the rest pose and follow their bone through its full 3D frame. `axonometric` is the derived 3D view of the `volume`; only the first three have authored shapes ({@link PLANAR_VIEWPOINTS}). */
export const VIEWPOINTS = ["elevation", "front", "plan", "axonometric"] as const;

/** 🔭️ One of {@link VIEWPOINTS}. */
export type Viewpoint = (typeof VIEWPOINTS)[number];

/** 📐️ The viewpoints with authored planar shapes, the keys of `geometry` and of every evaluated shape set. */
export const PLANAR_VIEWPOINTS = ["elevation", "front", "plan"] as const;

/** 🧭️ One of {@link PLANAR_VIEWPOINTS}. */
export type PlanarViewpoint = (typeof PLANAR_VIEWPOINTS)[number];

/** 📤️ The output kinds an asset can be exported to: `glb` a glTF 2.0 binary with every pose and clip, `fbx` an FBX 7.4 binary of one clip, `model` the engine-neutral JSON model document, `model-zip` the ZIP of every pose and clip of an asset as glb, fbx and model files. */
export const EXPORT_FORMATS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"] as const;

/** 📦️ One of {@link EXPORT_FORMATS}. */
export type ExportFormat = (typeof EXPORT_FORMATS)[number];

/** ✅️ How far an asset has come: planned, authored data only, or every representation, view, pose, clip, model and export checked. */
export const COMPLETION = ["planned", "authored", "verified"] as const;

/** 🏁️ One of {@link COMPLETION}. */
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
/** 🧊️ The transitions a {@link PhaseChange} describes: `boiling` (liquid to vapour) and `melting` (solid to liquid). */
export const PHASE_TRANSITIONS = ["boiling", "melting"] as const;

/** 🔄️ One of {@link PHASE_TRANSITIONS}. */
export type PhaseTransition = (typeof PHASE_TRANSITIONS)[number];

/** 💧️ A phase change at constant temperature: while a region is heated, its temperature is capped at `pointC` and the surplus power is converted into mass of the new phase at `latentHeatJKg`
 * (water: boiling 100 degC, 2.257 MJ/kg at 1 atm; ice: melting 0 degC, 0.334 MJ/kg). A region's own declaration overrides the one of its material. The inventory is assumed ample (no depletion).
 * @see https://doi.org/10.1002/9781119175957 — Incropera & DeWitt, latent heat */
export type PhaseChange = { readonly transition: PhaseTransition; readonly pointC: number; readonly latentHeatJKg: number };

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
  readonly phaseChange?: PhaseChange;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};
//#endregion 🧱️Materials

//#region 🦴️Rig
/** 🔩️ How a bone may move relative to its parent. */
export const JOINT_TYPES = ["fixed", "revolute", "prismatic"] as const;

/** 🔗️ One of {@link JOINT_TYPES}. */
export type JointType = (typeof JOINT_TYPES)[number];

/** 🦾️ A joint and its limits; revolute limits are angle offsets from rest, prismatic limits are offsets along `axisDeg` (bone-parent frame). */
export type Joint = {
  readonly type: JointType;
  readonly minDeg?: number;
  readonly maxDeg?: number;
  readonly twistMinDeg?: number;
  readonly twistMaxDeg?: number;
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

/** 🦴️ A bone: a rigid frame placed at (`x`, `y`) and rotated by `angleDeg` in its parent's frame at rest; its local x axis runs along it. `z` is the lateral rest offset of the origin in the parent frame (left is +, default 0): the pivot of a spread or twist of a bone beside the symmetry plane (a wing root, a shoulder). */
export type Bone = {
  readonly id: Slug;
  readonly parent: Slug | null;
  readonly title: Text;
  readonly x: number;
  readonly y: number;
  readonly z?: number;
  readonly angleDeg: number;
  readonly lengthM: number;
  readonly joint: Joint;
  readonly mass?: SegmentMass;
  readonly section?: Section;
};

/** 🌳️ A transform hierarchy with exactly one root. `kind` names the articulation family. */
export type Rig = {
  readonly kind: "anatomical" | "botanical" | "mechanical" | "structural";
  readonly root: Slug;
  readonly bones: readonly Bone[];
};
//#endregion 🦴️Rig

//#region 📍️Anchors
/** 📍️ What an anchor is for. */
export const ANCHOR_KINDS = ["contact", "support", "attachment", "emitter", "load"] as const;

/** 🎯️ One of {@link ANCHOR_KINDS}. */
export type AnchorKind = (typeof ANCHOR_KINDS)[number];

/** 🧷️ How a support restrains: unilateral ground contact (compression only, friction), pin, roller or fixed. */
export const SUPPORT_TYPES = ["contact", "pin", "roller", "fixed"] as const;

/** 🪨️ One of {@link SUPPORT_TYPES}. */
export type SupportType = (typeof SUPPORT_TYPES)[number];

/** 🪝️ A point fixed on a bone (local coordinates); contacts and supports carry the footprint used for contact pressure. `normalDeg` is
 * the world direction a contact or roller restrains (default 90, vertical); `tiltDeg` (-90 to 90, default 0) lifts that normal out of the sagittal plane towards the left (+z) and is a physical value, not a placeholder: 0 keeps the restraint in the plane, 90 restrains the lateral direction only, so it is stated only on supports that restrain out of plane; for emitters `directionDeg` in the emitter gives the outflow. */
export type Anchor = {
  readonly id: Slug;
  readonly bone: Slug;
  readonly x: number;
  readonly y: number;
  readonly z?: number;
  readonly kind: AnchorKind;
  readonly title: Text;
  readonly support?: SupportType;
  readonly normalDeg?: number;
  readonly tiltDeg?: number;
  readonly footprintLengthM?: number;
  readonly footprintWidthM?: number;
  readonly frictionCoefficient?: number;
};
//#endregion 📍️Anchors

//#region ✏️Geometry
/** 🖋️ The line-hierarchy role of a shape: silhouette, secondary detail, fine hatch/texture, transparent glazing or ground line. */
export const SHAPE_ROLES = ["body", "detail", "texture", "glazing", "ground"] as const;

/** 🎨️ One of {@link SHAPE_ROLES}. */
export type ShapeRole = (typeof SHAPE_ROLES)[number];

/** 💊️ A stadium between two local points with radii `r0` and `r1` (tangent hull of two circles). In plan geometry `followTip` makes the `(x1, y1)` end follow the x travel of the tip of the shape's bone (the `(x0, y0)` end follows its origin), so the stadium stretches along a limb seen from above. */
export type CapsuleShape = { readonly kind: "capsule"; readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number; readonly r0: number; readonly r1: number; readonly followTip?: boolean };

/** ⭕️ An ellipse in local coordinates. */
export type EllipseShape = { readonly kind: "ellipse"; readonly cx: number; readonly cy: number; readonly rx: number; readonly ry: number; readonly rotationDeg?: number };

/** ⬛️ A rectangle with optional corner radius, lower-left at (`x`, `y`). In front geometry `followTip` stretches it vertically between the origin and the tip of its bone (a vertex at the bone origin height follows the y travel of the origin, one at the tip height the y travel of the tip, linearly between). */
export type RectShape = { readonly kind: "rect"; readonly x: number; readonly y: number; readonly w: number; readonly h: number; readonly r?: number; readonly followTip?: boolean };

/** 🔷️ A polygon or polyline through local points. In front geometry `followTip` stretches it vertically between the origin and the tip of its bone, as for {@link RectShape}. */
export type PolyShape = { readonly kind: "poly"; readonly points: readonly (readonly [number, number])[]; readonly closed: boolean; readonly followTip?: boolean };

/** 〰️ An SVG-style path (`M L H V C Q A Z`, absolute or relative) in local metres, y up. */
export type PathShape = { readonly kind: "path"; readonly d: string };

/** 🦿️ An end point of a {@link MemberShape}: metres from the origin of `bone` (default the shape's own bone; the shape's own bone or one of its ancestors) at rest, `x` along the bone (u), `y` along its normal (v), `z` lateral (w, left is +); `z` is required and never defaulted. */
export type MemberEnd = { readonly bone?: Slug; readonly x: number; readonly y: number; readonly z: number };

/** 🎡️ The physical section of a member: a rectangle of width `w` (along bar x bone-w) and height `h` (completing the orthonormal frame), or a circle of diameter `d`, in metres. */
export type MemberSection = { readonly kind: "rect"; readonly w: number; readonly h: number } | { readonly kind: "circle"; readonly d: number };

/** 📊️ A prismatic bar between two bone-local end points with a physical section, carried by the full 3D frames of its bones and drawn in its true foreshortened outline in every viewpoint. It has no planar geometry and no `solid`: a diagonal is one member, never a bounding rectangle. */
export type MemberShape = { readonly kind: "member"; readonly from: MemberEnd; readonly to: MemberEnd; readonly section: MemberSection };

/** 🔺️ One of the shape primitives. */
export type ShapeGeometry = CapsuleShape | EllipseShape | RectShape | PolyShape | PathShape | MemberShape;

/** 🌀️ How a shape of any viewpoint is a solid of its rig-plane profile, authored in the bone's local frame like a side elevation shape and placed with the full 3D frame of its bone (spread, twist and sagittal rotation) and projected to the viewpoint (side elevation, front elevation or plan), so one 3D definition reads right in all three views at every pose and clip instant.
 * `revolved`: an axisymmetric part (umbrella canopy, rib, lid, drum) about an axis through the rest origin of `axisBone` along its bone-local direction `axis` (`u` along the bone, `v` its normal in the sagittal plane, `w` the lateral axis), through the point translated from that origin by `axisX` along world x and `axisZ` along world z (default 0) at rest, the axis then carried with that bone; `axisBone` is the shape's own bone or one of its ancestors, so the solid moves rigidly with its bone; the optional `lateralScale` (> 0, default 1, a physical circle) scales the lateral radius to an elliptic cross-section; the front paper x is the lateral distance from the axis (`mirror` also draws the reflection, so a half profile reads as the whole body), the plan is the circle of the widest lateral reach about the axis.
 * `extruded`: a prismatic part swept along the bone-local axis `sweep` (`w` lateral, with the profile in the (u, v) plane; `v` along the bone normal, with the profile in the (u, w) plane; required, never defaulted), here described for the lateral sweep: a part (blind slat, damper flap, door panel, boom, board) spanning `z0`..`z1` (left is +); the front is `z0`..`z1` by the height range of the profile, the plan the forward range of the profile by `z0`..`z1`. The optional `zTip` is the span at the bone tip in the same order as `z0`..`z1` (`zTip[0] < zTip[1]`); the span interpolates linearly along the bone from `z0`..`z1` at its origin, so a tapered limb, blade or branch is one extruded solid. */
export type Solid = { readonly kind: "revolved"; readonly axisBone: Slug; readonly axis: "u" | "v" | "w"; readonly axisX: number; readonly axisZ?: number; readonly lateralScale?: number; readonly mirror?: boolean } | { readonly kind: "extruded"; readonly sweep: "v" | "w"; readonly z0: number; readonly z1: number; readonly zTip?: readonly [number, number] };

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
  readonly solid?: Solid;
};
//#endregion ✏️Geometry

//#region 🧊️Volume
/** 🔢️ A pair [a, b]: a point of a profile plane, in metres. */
export type Vector2 = readonly [number, number];

/** 🤸️ A triple [u, v, w]: a point or direction in the bone-local frame (u along the bone, v its normal in the sagittal plane, w lateral with the left of the asset at +), in metres for points. */
export type Vector3 = readonly [number, number, number];

/** 💾️ A rotation quaternion [x, y, z, w] of unit length; the identity is [0, 0, 0, 1]. */
export type Quaternion = readonly [number, number, number, number];

/** 🌟️ The six kinds of a {@link VolumeSolid}. */
export const VOLUME_SOLID_KINDS = ["box", "ellipsoid", "capsule", "revolve", "extrude", "loft"] as const;

/** 🧬️ One of {@link VOLUME_SOLID_KINDS}. */
export type VolumeSolidKind = (typeof VOLUME_SOLID_KINDS)[number];

/** 🗃️ The coordinate planes an `extrude` solid lies on: `xy` has the profile coordinates (u, v) and extrudes along w, `yz` (v, w) along u, `zx` (w, u) along v. */
export const VOLUME_PLANES = ["xy", "yz", "zx"] as const;

/** 🔁️ The largest `count` of a {@link VolumeRepeat}. */
export const VOLUME_REPEAT_MAX = 256;

/** 🧮️ The most bones a solid is blended over. */
export const VOLUME_WEIGHTS_MAX = 4;

/** 🪜️ A placement of one solid inside the bone-local frame, applied to its own coordinates in the order scale (positive per-axis factors), rotation (unit quaternion), translation (metres); absent parts mean identity. */
export type VolumeTransform = { readonly translation?: Vector3; readonly rotation?: Quaternion; readonly scale?: Vector3 };

/** 🕰️ `count` copies of a solid: copy k = 0 .. count-1 is the solid after `transform`, moved by the step applied k times (a rotation of `rotationRad` about `rotationAxis` through `origin`, default the origin of the frame, then `translation`). */
export type VolumeRepeat = { readonly count: number; readonly translation?: Vector3; readonly rotationAxis?: Vector3; readonly rotationRad?: number; readonly origin?: Vector3 };

/** 🕸️ The fields every {@link VolumeSolid} carries: `bone` frames its coordinates and carries it (or, with `weights`, blends it over up to four bones that sum to 1), `material` an entry of the asset's materials, `role` the line-hierarchy role, `tone` the grey 0..1 of the solid views, `region`/`regions` its thermal regions, `maxDenominator` the largest preset denominator that still shows it. */
export type VolumeSolidBase = {
  readonly id: Slug;
  readonly bone: Slug;
  readonly material: Slug;
  readonly role: ShapeRole;
  readonly tone: number;
  readonly region?: Slug;
  readonly regions?: readonly Slug[];
  readonly maxDenominator?: ScaleDenominator;
  readonly weights?: readonly { readonly bone: Slug; readonly weight: number }[];
  readonly transform?: VolumeTransform;
  readonly repeat?: VolumeRepeat;
  readonly tags?: readonly Slug[];
};

/** 💡️ One declarative solid of a {@link Volume} in the bone-local rest frame of its bone, in metres: `box` (centred, edges along u, v, w, optionally rounded), `ellipsoid` (centred, semi-axes along u, v, w), `capsule` (hull of two spheres, optionally elliptic through `lateralScale`), `revolve` (a [axial, radial] profile about the axis through `origin`), `extrude` (a polygon with holes on `plane` pushed from `from` to `to`, optionally tapered) and `loft` (superelliptic sections joined in order). */
export type VolumeSolid = VolumeSolidBase &
  (
    | { readonly kind: "box"; readonly size: Vector3; readonly cornerRadius?: number }
    | { readonly kind: "ellipsoid"; readonly radii: Vector3 }
    | { readonly kind: "capsule"; readonly p0: Vector3; readonly p1: Vector3; readonly r0: number; readonly r1: number; readonly lateralScale?: number }
    | { readonly kind: "revolve"; readonly profile: readonly Vector2[]; readonly axis: Vector3; readonly origin: Vector3; readonly sweepRad?: number; readonly lateralScale?: number }
    | { readonly kind: "extrude"; readonly profile: readonly Vector2[]; readonly holes?: readonly (readonly Vector2[])[]; readonly plane: (typeof VOLUME_PLANES)[number]; readonly from: number; readonly to: number; readonly taperTo?: readonly [number, number] }
    | { readonly kind: "loft"; readonly sections: readonly { readonly at: Vector3; readonly rx: number; readonly ry: number; readonly exponent?: number; readonly rotation?: Quaternion }[]; readonly capStart: boolean; readonly capEnd: boolean }
  );

/** 🔧️ The declarative 3D volume of an asset: its solids bound to the bones of the rig in the bone-local rest frame, the source of the engine-neutral model, the GLB and FBX files and the axonometric view. `decisions` states what the solids stand for and which simplifications were made. */
export type Volume = { readonly status: "authored"; readonly solids: readonly VolumeSolid[]; readonly decisions: readonly Text[] };
//#endregion 🧊️Volume

//#region 🖨️Representations
/** 🖨️ The preset drawing-scale denominators, smallest scale first; every asset decides each of them explicitly. */
export const SCALE_DENOMINATORS = [500, 200, 100, 50] as const;

/** 🖱️ One of {@link SCALE_DENOMINATORS}. */
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

/** 🗺️ A representation individually authored for one preset scale. `validRange` is the range of custom denominators this
 * representation is validated for (it always contains its own denominator); custom scales inside it render it as a derived use. */
export type AuthoredRepresentation = {
  readonly status: "authored";
  readonly denominator: ScaleDenominator;
  readonly validRange: { readonly minDenominator: number; readonly maxDenominator: number };
  readonly pens: PenSet;
  readonly hatches: readonly Hatch[];
  readonly geometry: { readonly elevation: readonly Shape[]; readonly front: readonly Shape[]; readonly plan: readonly Shape[] };
  readonly sections?: { readonly elevation?: ViewSection; readonly front?: ViewSection; readonly plan?: ViewSection };
  readonly decisions: RepresentationDecisions;
};

/** ✂️ A viewpoint drawn as a section: the cut plane lies at `plane` metres along the view axis (elevation: the lateral position z of a vertical plane along the forward axis, seen from the right, the part at smaller z removed; front: the forward position x of a plane across the forward axis, seen from ahead, the part at larger x removed; plan: the height y of a horizontal plane, seen from above, the part above removed) and is named by `label` (such as A-A). The side elevation shows the cut-line marker of a front or plan section, the plan the cut-line marker of an elevation section; sheets and the viewer caption the cut "Section A-A" / "Schnitt A-A" (elevation and front) and "Plan section B-B" / "Grundriss-Schnitt B-B" (plan). */
export type ViewSection = { readonly plane: number; readonly label: Text };

/** ↪️ What to offer instead of an unavailable representation: another scale of the asset, a symbol of the asset, or another asset. */
export type Alternative = { readonly kind: "scale"; readonly denominator: ScaleDenominator } | { readonly kind: "symbol"; readonly symbol: Slug } | { readonly kind: "asset"; readonly asset: Slug };

/** 🚫️ Why a scale has no physically scaled drawing, in all three viewpoints: a real feature is smaller than the print profile can print, or the asset has no meaningful physical depiction at this scale. */
export const UNAVAILABLE_CAUSES = ["below-printable-feature", "no-physical-depiction"] as const;

/** 🔎️ One of {@link UNAVAILABLE_CAUSES}. */
export type UnavailableCause = (typeof UNAVAILABLE_CAUSES)[number];

/** 🪛️ An explicit decision that the asset cannot be meaningfully drawn at this scale: why,which feature limits it and at what real size,
 * and what to use instead (at least one alternative). */
export type UnavailableRepresentation = {
  readonly status: "unavailable";
  readonly denominator: ScaleDenominator;
  readonly cause: UnavailableCause;
  readonly reason: Text;
  readonly limitingFeature: Text;
  readonly limitingSizeM: number;
  readonly alternatives: readonly Alternative[];
};

/** 🔀️ The decision for one preset scale. */
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
  readonly geometry: { readonly elevation: readonly Shape[]; readonly front: readonly Shape[]; readonly plan: readonly Shape[] };
  readonly dimensionalNote: Text;
};

/** 📃️ A print profile: the smallest stroke, gap between separate outlines, feature width and hatch spacing that reliably print. */
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

/** 📑️ The print profile document `🗂️catalog/🖨️print-profiles/🔣️.json`. */
export type PrintProfileDocument = { readonly $schema: string; readonly version: typeof SCHEMA_VERSION; readonly defaultProfile: Slug; readonly profiles: readonly PrintProfile[]; readonly references: readonly Reference[] };
//#endregion 🖨️Representations

//#region 🧍️Poses
/** 🎚️ An offset from rest for one bone: rotation delta and translation delta along the parent frame (prismatic joints). `abductionDeg` is the lateral spread of a revolute bone: the bone turns out of the sagittal plane about its own normal (positive towards the left of the asset, +z), added down the chain like `angleDeg`. `twistDeg` turns the bone and its subtree about its own longitudinal axis (a yaw for a vertical bone: slew, swivel, steering; positive counter-clockwise seen from above for a bone pointing up). Both are solved in a three-dimensional chain whose orthographic projections are the side elevation, the front elevation and the plan (top view). */
export type BoneOffset = { readonly angleDeg?: number; readonly abductionDeg?: number; readonly twistDeg?: number; readonly dx?: number; readonly dy?: number };

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
  readonly z?: number;
  readonly fzN?: number;
  readonly momentXNm?: number;
  readonly momentYNm?: number;
  readonly momentNm?: number;
  readonly distributed?: { readonly bone: Slug; readonly fromM: number; readonly toM: number; readonly wxNPerM: number; readonly wyNPerM: number; readonly wzNPerM?: number };
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🌬️ The region state a pose binds: share of the area exposed to air, share of the area in sun, whether the surface is wetted, and the
 * conductive contact (seat, ground) it rests on. */
export type RegionBinding = { readonly exposedFraction: number; readonly sunlitFraction: number; readonly wettedness?: number; readonly contact?: { readonly fraction: number; readonly tempC: number } };

/** 🏃️ The thermal and respiratory state a pose or clip key binds explicitly: activity, external work, operating levels of machines,
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

/** 📥️ A configuration: bone offsets, root placement, active contacts, thermal binding and loads. `key: true` marks an in-between key
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

/** 🎢️ One of {@link EASES}. */
export type Ease = (typeof EASES)[number];

/** ⏱️ A clip key: at `t` the clip shows `pose` with optional bone overrides; `ease` shapes the way to the next key. */
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
  /** 🛰️ Horizontal bearing the air moves towards, degrees from the forward axis +x towards the left axis +z (0 forward, 90 towards the left of the asset, 180 backward; the frame is left-handed: x ahead, y up, z left); the wind has no vertical component. */
  readonly windDirectionDeg: number;
  readonly solarWM2: number;
  readonly diffuseFraction: number;
  readonly pressurePa: number;
};

/** 🏞️ A named environment preset of `🗂️catalog/🌤️environments/🔣️.json`, shared by every asset so results stay comparable. */
export type EnvironmentPreset = { readonly id: Slug; readonly title: Text; readonly description: Text; readonly environment: Environment; readonly source: ParameterSource; readonly reference?: Slug };

/** 🏔️ The environment preset document. */
export type EnvironmentDocument = { readonly $schema: string; readonly version: typeof SCHEMA_VERSION; readonly presets: readonly EnvironmentPreset[]; readonly references: readonly Reference[] };

/** ♨️ How a region's surface temperature is obtained.
 * - `body`: a living body segment; heat flows from the regulated core through tissue (conductance driven by skin blood flow) to the skin,
 *   then through the region's `layers` (clothing, fur, feathers, trapped air) to the outer surface.
 * - `leaf`: leaf energy balance with stomatal conductance (latent heat by transpiration).
 * - `passive`: solar absorption, radiation, convection and contact conduction of an inert object, with heat capacity.
 * - `operating`: an inert surface heated from inside by `operating.heatW × level` (engines, motors, lamps, heaters, exhausts); a negative `heatW` removes heat (evaporators, cooling coils).
 * - `conditioned`: a building element separating the environment from an interior at `conditioned.interiorTempC` through its `layers`.
 * - `hydronic`: a surface heated by the water of a heating circuit: the inside temperature is the circuit water at the region's position along the flow, from the supply temperature falling to the return temperature, at the circuit's operating level; heat flows from it through the layers and the water-side film.
 * - `coupled`: a surface whose inside is the water of another region of the asset (a jug or pot wall): below the fill level the wall follows that region's temperature through the layers and the water-side film, and the heat it draws is taken from that region.
 * - `fixed`: a prescribed surface temperature (only for reference fixtures). */
export const THERMAL_MODELS = ["body", "leaf", "passive", "operating", "conditioned", "hydronic", "coupled", "fixed"] as const;

/** 🔬️ One of {@link THERMAL_MODELS}. */
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

/** 📖️ One row of a heating-circuit table: the supply and return water temperatures at one operating level `v`. */
export type CircuitRow = { readonly v: number; readonly supplyTempC: number; readonly returnTempC: number };

/** 🚰️ The inside of a `hydronic` region: the water of the circuit driven by the operating level `key`. `rows` (ascending in `v`, at least one) give the supply and return temperatures at named levels and are interpolated linearly in between; below the first row the water falls linearly to the air temperature at level 0, above the last row it holds. `flowPosition` (0 at the inlet, 1 at the outlet) places the region along the flow: its water is `supply − (supply − return) · flowPosition`. `filmM2KW` is the resistance between that water and the layers (water-side film, fins), referred to the region's outer area. */
export type HydronicCircuit = { readonly key: Slug; readonly rows: readonly CircuitRow[]; readonly flowPosition: number; readonly filmM2KW: number };

/** 🫗️ The inside of a `coupled` region: the water of the region `region` of the same asset. `wetFraction` is the share of the outer area behind which the water stands (fill level over wall height); only that share conducts, `filmM2KW` is the water-side resistance referred to the wetted area. */
export type WaterCoupling = { readonly region: Slug; readonly wetFraction: number; readonly filmM2KW: number };

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
  readonly hydronic?: HydronicCircuit;
  readonly coupled?: WaterCoupling;
  readonly fixed?: { readonly surfaceTempC: number };
  readonly phaseChange?: PhaseChange;
  readonly hidden?: Text;
  readonly source: ParameterSource;
  readonly reference?: Slug;
};

/** 🧯️ The operating key that drives a region: the key of its `operating` heat or of its `hydronic` circuit, else `null`. */
export const operatingKeyOf = (region: ThermalRegion): Slug | null => region.operating?.key ?? region.hydronic?.key ?? null;

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

/** 👃️ Respiration of a body: the nostril and mouth emitters (emitter anchors on the head), exhaled air state, the patterns poses bind,
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
 * a `vent`, `flue`, `fan` or `exhaust` with a `region` instead releases the vapour mass rate of that region's evaporation (surface evaporation plus phase change) as air carrying `vapourKgPerKg`
 * (its `volumeFlowM3S` is not used), heated by `temperatureRiseK` and weighted by the operating level;
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
  readonly referenceEnvironment?: Slug | Environment;
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
/** 🧰️ Nominal overall real dimensions and total mass: the physical envelope of everything that belongs to the asset in its rest pose (held items, attachments, plates, flanges, lids, footings), without stroke width, ground shading, plumes, transport or annotations; every authored representation reproduces them in every viewpoint within max(2 % of the declared value, 15 mm).
 * `heightM` is the extent along world y (up), `widthM` along world x (ahead: the real depth of an object drawn in side elevation; paper x of the side elevation and of the plan), `depthM` along world z (lateral: paper x of the front elevation, paper up of the plan; the left-right width of an object drawn in side elevation). */
export type Dimensions = { readonly heightM: number; readonly widthM: number; readonly depthM: number; readonly massKg: number };

/** 🖼️ Presentation defaults. */
export type Presentation = { readonly defaultPose: Slug; readonly defaultClip: Slug; readonly defaultScale: ScaleDenominator };

/** 🏺️ One handcrafted staffage asset. The physical scenario is scale-independent; `representations` decides every preset scale. */
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
  readonly volume: Volume;
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

/** 📄️ One asset file: `🗂️catalog/<category folder>/<asset id>/🔣️.json`. */
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
  readonly exports: readonly ExportFormat[];
  readonly distinction: Text;
  readonly completion: Completion;
};

/** 📂️ A category of the archive with its folder of asset folders. */
export type CategoryEntry = { readonly id: CategoryId; readonly emoji: string; readonly title: Text; readonly description: Text; readonly folder: string };

/** 📜️ The archive manifest `🗂️catalog/🔣️.json`. */
export type Manifest = {
  readonly $schema: string;
  readonly version: typeof SCHEMA_VERSION;
  readonly categories: readonly CategoryEntry[];
  readonly collections: readonly { readonly id: Slug; readonly title: Text; readonly description: Text; readonly assets: readonly Slug[] }[];
  readonly entries: readonly ManifestEntry[];
};
//#endregion 📋️Manifest
