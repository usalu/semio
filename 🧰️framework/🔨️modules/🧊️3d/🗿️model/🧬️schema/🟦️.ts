/** 🗿️ Typed twin of `🧬️schema/🔣️.json` (`semio.model/v1`): the domain-neutral, engine-neutral 3D model.
 *
 * Schema-first — `🔣️.json` is the single source of truth; this module restates every `$defs` entry under the same name with `readonly` fields. No runtime dependency.
 *
 * Frame (normative): right-handed, +Y up, +Z ahead (front of the asset), +X the asset's left; metres, radians, quaternions `[x, y, z, w]`, column-major 4x4 matrices.
 * Solids live in the local frame of their node; a procedural document carries solids and no meshes, a resolved one carries meshes and no solids.
 *
 * @see 🧬️schema/🔣️.json — the normative contract
 */

/** 🏷️ A kebab-case identifier, unique within its scope. */
export type Slug = string;
/** 🌍️ A human-readable text in every supported language; there is no default language. */
export type Text = { readonly en: string; readonly de: string };
/** 📐️ Two numbers. */
export type Vec2 = readonly [number, number];
/** 📍️ Three numbers. */
export type Vec3 = readonly [number, number, number];
/** 🎲️ Four numbers; a quaternion is `[x, y, z, w]`. */
export type Vec4 = readonly [number, number, number, number];
/** 🧮️ Column-major 4x4 matrix. */
export type Mat4 = readonly [number, number, number, number, number, number, number, number, number, number, number, number, number, number, number, number];
/** 🔁️ Optional translation, rotation quaternion and scale; absent = identity. */
export type TransformPatch = { readonly translation?: Vec3; readonly rotation?: Vec4; readonly scale?: Vec3 };

/** 🦴️ The mechanism of a node: limits in rad or m about a node-local axis; `spherical` limits the swing cone half angle with `max` and the twist about `axis` with `twistMin`/`twistMax`. */
export type Joint = { readonly type: "fixed" | "revolute" | "prismatic" | "spherical" | "free"; readonly axis: Vec3; readonly min?: number; readonly max?: number; readonly twistMin?: number; readonly twistMax?: number };
/** 🧱️ A transform node; parents precede children. */
export type Node = { readonly id: Slug; readonly parent: Slug | null; readonly translation: Vec3; readonly rotation: Vec4; readonly scale: Vec3; readonly joint?: Joint; readonly mesh?: Slug; readonly skin?: Slug; readonly tags?: readonly Slug[] };
/** ♻️ The k-th copy applies the step k times. */
export type Repeat = { readonly count: number; readonly translation?: Vec3; readonly rotationAxis?: Vec3; readonly rotationRad?: number; readonly origin?: Vec3 };
/** 🧩️ The properties every solid shares. */
type SolidBase = { readonly id: Slug; readonly node: Slug; readonly material: Slug; readonly tags?: readonly Slug[]; readonly transform?: TransformPatch; readonly repeat?: Repeat; readonly weights?: readonly { readonly node: Slug; readonly weight: number }[] };
/** 📦️ A centred box with optionally rounded edges. */
export type BoxSolid = SolidBase & { readonly kind: "box"; readonly size: Vec3; readonly cornerRadius?: number };
/** 🥚️ A centred ellipsoid. */
export type EllipsoidSolid = SolidBase & { readonly kind: "ellipsoid"; readonly radii: Vec3 };
/** 💊️ The hull of two spheres. */
export type CapsuleSolid = SolidBase & { readonly kind: "capsule"; readonly p0: Vec3; readonly p1: Vec3; readonly r0: number; readonly r1: number; readonly lateralScale?: number };
/** 🌀️ A profile [axial, radial] revolved about an axis through origin. */
export type RevolveSolid = SolidBase & { readonly kind: "revolve"; readonly profile: readonly Vec2[]; readonly axis: Vec3; readonly origin: Vec3; readonly sweepRad?: number; readonly lateralScale?: number };
/** 🏗️ A polygon with holes extruded between two plane coordinates, optionally tapered. */
export type ExtrudeSolid = SolidBase & { readonly kind: "extrude"; readonly profile: readonly Vec2[]; readonly holes?: readonly (readonly Vec2[])[]; readonly plane: "xy" | "yz" | "zx"; readonly from: number; readonly to: number; readonly taperTo?: Vec2 };
/** 🧅️ One superelliptic loft section: with `rotation` it lies in the x, y plane of that rotation (absolute), without it perpendicular to the loft path tangent (x = node X projected perpendicular, fallback Z; y = tangent x x). */
export type LoftSection = { readonly at: Vec3; readonly rx: number; readonly ry: number; readonly exponent?: number; readonly rotation?: Vec4 };
/** 🧵️ Superelliptic sections lofted in order. */
export type LoftSolid = SolidBase & { readonly kind: "loft"; readonly sections: readonly LoftSection[]; readonly capStart: boolean; readonly capEnd: boolean };
/** 🧊️ Procedural declarative geometry in the node-local frame. */
export type Solid = BoxSolid | EllipsoidSolid | CapsuleSolid | RevolveSolid | ExtrudeSolid | LoftSolid;
/** 🧬️ Provenance range: the solid `id` and the index range `[first, first + count)` of the primitive it produced. */
export type PrimitiveSolid = { readonly id: Slug; readonly first: number; readonly count: number };
/** 🔺️ Triangle or line geometry; `joints`/`weights` carry four influences per vertex (skin joint index), `colors` are rgba; `solids` (optional) says which solid produced which contiguous index range, in solid order, covering all indices. */
export type Primitive = { readonly mode: "triangles" | "lines"; readonly material: Slug; readonly positions: readonly number[]; readonly normals?: readonly number[]; readonly colors?: readonly number[]; readonly joints?: readonly number[]; readonly weights?: readonly number[]; readonly indices: readonly number[]; readonly solids?: readonly PrimitiveSolid[] };
/** 🕸️ A set of primitives. */
export type Mesh = { readonly id: Slug; readonly primitives: readonly Primitive[] };
/** 🩻️ Joint nodes and their inverse bind matrices. */
export type Skin = { readonly id: Slug; readonly joints: readonly Slug[]; readonly inverseBind: readonly Mat4[] };
/** 🎨️ A physically based or unlit surface. */
export type Material = { readonly id: Slug; readonly title: Text; readonly baseColor: Vec4; readonly metallic: number; readonly roughness: number; readonly emissive: Vec3; readonly alphaMode: "opaque" | "blend"; readonly doubleSided: boolean; readonly unlit: boolean };
/** 🎭️ A named look of one geometry: material id to material id. */
export type Variant = { readonly id: Slug; readonly title: Text; readonly bindings: { readonly [materialId: Slug]: Slug } };
/** 🧍️ Node transform overrides. */
export type Pose = { readonly id: Slug; readonly title: Text; readonly nodes: { readonly [nodeId: Slug]: TransformPatch } };
/** 🎞️ One animated property. */
export type Channel = {
  readonly target: { readonly node: Slug; readonly path: "translation" | "rotation" | "scale" } | { readonly material: Slug; readonly path: "baseColor" | "emissive" };
  readonly interpolation: "step" | "linear";
  readonly times: readonly number[];
  readonly values: readonly number[];
};
/** 🎬️ A clip of channels and events. */
export type Animation = { readonly id: Slug; readonly title: Text; readonly durationS: number; readonly loop: boolean; readonly channels: readonly Channel[]; readonly events: readonly { readonly t: number; readonly id: Slug }[] };
/** 📚️ A neutral 3D model. */
export type ModelDocument = {
  readonly $schema?: string;
  readonly version: "semio.model/v1";
  readonly id: Slug;
  readonly title: Text;
  readonly provenance?: { readonly generator?: string; readonly source?: string; readonly notes?: readonly Text[] };
  readonly nodes: readonly Node[];
  readonly solids: readonly Solid[];
  readonly meshes: readonly Mesh[];
  readonly skins: readonly Skin[];
  readonly materials: readonly Material[];
  readonly variants: readonly Variant[];
  readonly poses: readonly Pose[];
  readonly animations: readonly Animation[];
};
/** 🎚️ Tessellation quality: maximum chord deviation in metres and the segment bounds of one full circle. */
export type Quality = { readonly chordM: number; readonly minSegments: number; readonly maxSegments: number };
/** 🩺️ One validation or pose finding; `path` is a JSON pointer into the document. */
export type Issue = { readonly severity: "error" | "warning"; readonly code: string; readonly path: string; readonly message: Text };

/** ⏱️ Progress and cancellation of one expensive operation: a behavioural interface, not data. `🔣️.json` names it in `$defs.Operation` for documentation only (functions are not JSON); no document ever contains one. `cancelled()` is polled at a bounded interval inside hot loops and a true answer ends the operation with a `CancelledError` and no partial result. */
export interface Operation {
  report(fraction: number): void;
  cancelled(): boolean;
}
