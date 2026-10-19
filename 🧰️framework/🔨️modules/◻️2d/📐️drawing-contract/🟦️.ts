/** 📐️ Drawing scene contracts: the vectors, path segments, nodes, fills, strokes and refs every 2D module (booleans, path, trace, text) and the canvas raster agree on; a leaf, so children never import the module index. */
export type Vec2 = readonly [number, number];

export type PathSegmentKind = "move" | "line" | "quad" | "cubic" | "arc" | "close";

export interface DrawingScene {
  readonly width: number;
  readonly height: number;
  readonly nodes: readonly SceneNode[];
}

export interface SceneNode {
  readonly transform: readonly [number, number, number, number, number, number];
  readonly node: DrawingNode;
  readonly fill?: FillStyle;
  readonly stroke?: StrokeStyle;
  readonly opacity?: number;
  readonly clip?: readonly PathSegment[];
}

export type DrawingNode =
  | { readonly kind: "rect"; readonly x: number; readonly y: number; readonly width: number; readonly height: number }
  | { readonly kind: "ellipse"; readonly cx: number; readonly cy: number; readonly rx: number; readonly ry: number }
  | { readonly kind: "circle"; readonly cx: number; readonly cy: number; readonly r: number }
  | { readonly kind: "line"; readonly x1: number; readonly y1: number; readonly x2: number; readonly y2: number }
  | { readonly kind: "polygon"; readonly points: readonly Vec2[] }
  | { readonly kind: "path"; readonly segments: readonly PathSegment[] }
  | { readonly kind: "text"; readonly x: number; readonly y: number; readonly content: string; readonly size: number; readonly font?: string; readonly anchor?: "start" | "middle" | "end"; readonly baseline?: "alphabetic" | "middle" | "top" | "bottom" }
  | { readonly kind: "group"; readonly children: readonly string[] };

export type PathSegment =
  | { readonly kind: "move"; readonly to: Vec2 }
  | { readonly kind: "line"; readonly to: Vec2 }
  | { readonly kind: "quad"; readonly ctrl: Vec2; readonly to: Vec2 }
  | { readonly kind: "cubic"; readonly ctrl1: Vec2; readonly ctrl2: Vec2; readonly to: Vec2 }
  | {
      readonly kind: "arc";
      readonly rx: number;
      readonly ry: number;
      readonly rotation: number;
      readonly largeArc: boolean;
      readonly sweep: boolean;
      readonly to: Vec2;
    }
  | { readonly kind: "close" };

export type FillStyle =
  | { readonly kind: "solid"; readonly color: readonly [number, number, number, number] }
  | {
      readonly kind: "linearGradient";
      readonly x1: number;
      readonly y1: number;
      readonly x2: number;
      readonly y2: number;
      readonly stops: readonly GradientStop[];
    }
  | { readonly kind: "radialGradient"; readonly cx: number; readonly cy: number; readonly r: number; readonly stops: readonly GradientStop[] };

export interface GradientStop {
  readonly offset: number;
  readonly color: readonly [number, number, number, number];
}

export interface StrokeStyle {
  readonly color: readonly [number, number, number, number];
  readonly width: number;
  readonly cap: "butt" | "round" | "square";
  readonly join: "miter" | "round" | "bevel";
  readonly dash?: readonly number[];
}

export type DrawingRef = string & { readonly __brand: "DrawingRef" };

export function drawingRef(id: string): DrawingRef {
  return id as DrawingRef;
}

export const DRAWING_REF_PATTERN = /^drawing-/;

export function isDrawingRef(value: unknown): value is DrawingRef {
  return typeof value === "string" && DRAWING_REF_PATTERN.test(value);
}

export const DRAW_BOOLEAN_OPERATIONS = ["union", "difference", "intersection", "xor"] as const;
export type DrawBooleanOperation = (typeof DRAW_BOOLEAN_OPERATIONS)[number];
