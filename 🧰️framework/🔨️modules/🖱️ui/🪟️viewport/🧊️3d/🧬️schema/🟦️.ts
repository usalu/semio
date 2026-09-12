import { viewportNumber, viewportRecord, viewportZoom } from "../../🟦️.ts";

/** 🌐️ Orbit navigation; projection and authored cameras have separate owners. */
export interface Viewport3dOrbit { position: [number, number, number]; target: [number, number, number]; zoom: number; up?: [number, number, number] }

function vector(value: unknown): [number, number, number] {
  if (!Array.isArray(value) || value.length !== 3) throw new TypeError("Expected three viewport coordinates");
  return [viewportNumber(value[0]), viewportNumber(value[1]), viewportNumber(value[2])];
}

/** 📥️ Admits the closed orbit navigation schema emitted by renderer gestures. */
export function parseViewport3dOrbit(value: unknown): Viewport3dOrbit {
  const record = viewportRecord(value, ["position", "target", "zoom"], ["up"]);
  return { position: vector(record.position), target: vector(record.target), zoom: viewportZoom(record.zoom), ...(Object.hasOwn(record, "up") ? { up: vector(record.up) } : {}) };
}

export type Viewport3dProjectionKind = "orthographic" | "axonometric" | "oblique" | "onePoint" | "twoPoint" | "threePoint" | "curvilinear";
export type Viewport3dOrthographicView = "plan" | "top" | "bottom" | "front" | "back" | "left" | "right";
export type Viewport3dAxonometricVariant = "isometric" | "dimetric" | "trimetric";
export type Viewport3dAxonometricQuadrant = "ne" | "nw" | "se" | "sw";
export type Viewport3dAxonometricHemisphere = "upper" | "lower";
export type Viewport3dObliqueVariant = "cabinet" | "cavalier" | "military";
export type Viewport3dOnePointAxis = "x" | "y" | "z";
export type Viewport3dCurvilinearMapping = "fisheye" | "panini";

/** 📐️ Complete editable projection preset bank retained by one viewport-owning window. */
export interface Viewport3dProjectionPreferences {
  readonly kind: Viewport3dProjectionKind;
  readonly orthographicView: Viewport3dOrthographicView;
  readonly axonometricVariant: Viewport3dAxonometricVariant;
  readonly axonometricAngleA: number;
  readonly axonometricAngleB: number;
  readonly axonometricQuadrant: Viewport3dAxonometricQuadrant;
  readonly obliqueVariant: Viewport3dObliqueVariant;
  readonly obliqueAngle: number;
  readonly obliqueDepth: number;
  readonly onePointAxis: Viewport3dOnePointAxis;
  readonly fov: number;
  readonly twoPointShift: number;
  readonly curvilinearFov: number;
  readonly curvilinearStrength: number;
  readonly curvilinearMapping: Viewport3dCurvilinearMapping;
}

export type Viewport3dProjectionMode =
  | { readonly kind: "orthographic" }
  | { readonly kind: "axonometric"; readonly variant: Viewport3dAxonometricVariant; readonly angleA: number; readonly angleB: number }
  | { readonly kind: "oblique"; readonly variant: Viewport3dObliqueVariant; readonly angle: number; readonly depthScale: number }
  | { readonly kind: "onePoint"; readonly fov: number }
  | { readonly kind: "twoPoint"; readonly fov: number; readonly verticalShift: number }
  | { readonly kind: "threePoint"; readonly fov: number }
  | { readonly kind: "curvilinear"; readonly fov: number; readonly strength: number; readonly mapping: Viewport3dCurvilinearMapping };

export type Viewport3dProjectionOrientation =
  | { readonly type: "cardinal"; readonly view: Viewport3dOrthographicView }
  | { readonly type: "corner"; readonly quadrant: Viewport3dAxonometricQuadrant; readonly hemisphere?: Viewport3dAxonometricHemisphere }
  | { readonly type: "free" };

/** 🎯️ Active mathematical projection snapshot transported to a renderer. */
export interface Viewport3dProjectionSpec {
  readonly mode: Viewport3dProjectionMode;
  readonly orientation: Viewport3dProjectionOrientation;
}

function projectionEnum<T extends string>(value: unknown, allowed: readonly T[]): T {
  if (typeof value !== "string" || !allowed.includes(value as T)) throw new TypeError("Expected declared viewport projection value");
  return value as T;
}

function projectionNumber(value: unknown, minimum: number, maximum: number): number {
  const number = viewportNumber(value);
  if (number < minimum || number > maximum) throw new TypeError("Viewport projection value is outside its declared range");
  return number;
}

function activeProjectionNumber(value: unknown): number {
  return viewportNumber(value);
}

const projectionKinds = ["orthographic", "axonometric", "oblique", "onePoint", "twoPoint", "threePoint", "curvilinear"] as const;
const orthographicViews = ["plan", "top", "bottom", "front", "back", "left", "right"] as const;
const axonometricVariants = ["isometric", "dimetric", "trimetric"] as const;
const axonometricQuadrants = ["ne", "nw", "se", "sw"] as const;
const axonometricHemispheres = ["upper", "lower"] as const;
const obliqueVariants = ["cabinet", "cavalier", "military"] as const;
const onePointAxes = ["x", "y", "z"] as const;
const curvilinearMappings = ["fisheye", "panini"] as const;

/** 📥️ Admits all fifteen required projection presets without coercion or foreign fields. */
export function parseViewport3dProjectionPreferences(value: unknown): Viewport3dProjectionPreferences {
  const record = viewportRecord(value, [
    "kind",
    "orthographicView",
    "axonometricVariant",
    "axonometricAngleA",
    "axonometricAngleB",
    "axonometricQuadrant",
    "obliqueVariant",
    "obliqueAngle",
    "obliqueDepth",
    "onePointAxis",
    "fov",
    "twoPointShift",
    "curvilinearFov",
    "curvilinearStrength",
    "curvilinearMapping",
  ]);
  return {
    kind: projectionEnum(record.kind, projectionKinds),
    orthographicView: projectionEnum(record.orthographicView, orthographicViews),
    axonometricVariant: projectionEnum(record.axonometricVariant, axonometricVariants),
    axonometricAngleA: projectionNumber(record.axonometricAngleA, 5, 75),
    axonometricAngleB: projectionNumber(record.axonometricAngleB, 5, 75),
    axonometricQuadrant: projectionEnum(record.axonometricQuadrant, axonometricQuadrants),
    obliqueVariant: projectionEnum(record.obliqueVariant, obliqueVariants),
    obliqueAngle: projectionNumber(record.obliqueAngle, 5, 90),
    obliqueDepth: projectionNumber(record.obliqueDepth, 0.05, 1),
    onePointAxis: projectionEnum(record.onePointAxis, onePointAxes),
    fov: projectionNumber(record.fov, 15, 120),
    twoPointShift: projectionNumber(record.twoPointShift, -1, 1),
    curvilinearFov: projectionNumber(record.curvilinearFov, 60, 160),
    curvilinearStrength: projectionNumber(record.curvilinearStrength, 0, 1),
    curvilinearMapping: projectionEnum(record.curvilinearMapping, curvilinearMappings),
  };
}

function parseViewport3dProjectionMode(value: unknown): Viewport3dProjectionMode {
  const candidate = viewportRecord(value, ["kind"], ["variant", "angleA", "angleB", "angle", "depthScale", "fov", "verticalShift", "strength", "mapping"]);
  switch (projectionEnum(candidate.kind, projectionKinds)) {
    case "orthographic": {
      viewportRecord(value, ["kind"]);
      return { kind: "orthographic" };
    }
    case "axonometric": {
      const record = viewportRecord(value, ["kind", "variant", "angleA", "angleB"]);
      return { kind: "axonometric", variant: projectionEnum(record.variant, axonometricVariants), angleA: activeProjectionNumber(record.angleA), angleB: activeProjectionNumber(record.angleB) };
    }
    case "oblique": {
      const record = viewportRecord(value, ["kind", "variant", "angle", "depthScale"]);
      return { kind: "oblique", variant: projectionEnum(record.variant, obliqueVariants), angle: activeProjectionNumber(record.angle), depthScale: activeProjectionNumber(record.depthScale) };
    }
    case "onePoint": {
      const record = viewportRecord(value, ["kind", "fov"]);
      return { kind: "onePoint", fov: activeProjectionNumber(record.fov) };
    }
    case "twoPoint": {
      const record = viewportRecord(value, ["kind", "fov", "verticalShift"]);
      return { kind: "twoPoint", fov: activeProjectionNumber(record.fov), verticalShift: activeProjectionNumber(record.verticalShift) };
    }
    case "threePoint": {
      const record = viewportRecord(value, ["kind", "fov"]);
      return { kind: "threePoint", fov: activeProjectionNumber(record.fov) };
    }
    case "curvilinear": {
      const record = viewportRecord(value, ["kind", "fov", "strength", "mapping"]);
      return { kind: "curvilinear", fov: activeProjectionNumber(record.fov), strength: activeProjectionNumber(record.strength), mapping: projectionEnum(record.mapping, curvilinearMappings) };
    }
  }
}

function parseViewport3dProjectionOrientation(value: unknown): Viewport3dProjectionOrientation {
  const candidate = viewportRecord(value, ["type"], ["view", "quadrant", "hemisphere"]);
  switch (projectionEnum(candidate.type, ["cardinal", "corner", "free"] as const)) {
    case "cardinal": {
      const record = viewportRecord(value, ["type", "view"]);
      return { type: "cardinal", view: projectionEnum(record.view, orthographicViews) };
    }
    case "corner": {
      const record = viewportRecord(value, ["type", "quadrant"], ["hemisphere"]);
      return {
        type: "corner",
        quadrant: projectionEnum(record.quadrant, axonometricQuadrants),
        ...(Object.hasOwn(record, "hemisphere") ? { hemisphere: projectionEnum(record.hemisphere, axonometricHemispheres) } : {}),
      };
    }
    case "free":
      viewportRecord(value, ["type"]);
      return { type: "free" };
  }
}

/** 📡️ Admits an exact active projection mode and any declared orientation alternative. */
export function parseViewport3dProjectionSpec(value: unknown): Viewport3dProjectionSpec {
  const record = viewportRecord(value, ["mode", "orientation"]);
  return { mode: parseViewport3dProjectionMode(record.mode), orientation: parseViewport3dProjectionOrientation(record.orientation) };
}

/** 🎯️ Creates the full editable bank without sharing mutable state between windows. */
export function defaultViewport3dProjectionPreferences(): Viewport3dProjectionPreferences {
  return {
    kind: "threePoint",
    orthographicView: "top",
    axonometricVariant: "isometric",
    axonometricAngleA: 15,
    axonometricAngleB: 12,
    axonometricQuadrant: "ne",
    obliqueVariant: "cavalier",
    obliqueAngle: 45,
    obliqueDepth: 1,
    onePointAxis: "y",
    fov: 50,
    twoPointShift: 0,
    curvilinearFov: 120,
    curvilinearStrength: 1,
    curvilinearMapping: "fisheye",
  };
}

/** 🧩️ Derives one active renderer-neutral projection while retaining the complete preference bank. */
export function deriveActiveProjection(preferences: Viewport3dProjectionPreferences): Viewport3dProjectionSpec {
  switch (preferences.kind) {
    case "orthographic":
      return { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: preferences.orthographicView } };
    case "axonometric": {
      const angleA = preferences.axonometricVariant === "isometric" ? 30 : preferences.axonometricAngleA;
      const angleB = preferences.axonometricVariant === "isometric" ? 30 : preferences.axonometricVariant === "dimetric" ? preferences.axonometricAngleA : preferences.axonometricAngleB;
      return { mode: { kind: "axonometric", variant: preferences.axonometricVariant, angleA, angleB }, orientation: { type: "corner", quadrant: preferences.axonometricQuadrant, hemisphere: "upper" } };
    }
    case "oblique":
      return {
        mode: { kind: "oblique", variant: preferences.obliqueVariant, angle: preferences.obliqueAngle, depthScale: preferences.obliqueDepth },
        orientation: { type: "cardinal", view: preferences.obliqueVariant === "military" ? "plan" : "front" },
      };
    case "onePoint":
      return { mode: { kind: "onePoint", fov: preferences.fov }, orientation: { type: "cardinal", view: preferences.onePointAxis === "x" ? "left" : preferences.onePointAxis === "z" ? "top" : "front" } };
    case "twoPoint":
      return { mode: { kind: "twoPoint", fov: preferences.fov, verticalShift: preferences.twoPointShift }, orientation: { type: "free" } };
    case "curvilinear":
      return { mode: { kind: "curvilinear", fov: preferences.curvilinearFov, strength: preferences.curvilinearStrength, mapping: preferences.curvilinearMapping }, orientation: { type: "free" } };
    case "threePoint":
      return { mode: { kind: "threePoint", fov: preferences.fov }, orientation: { type: "free" } };
  }
}
