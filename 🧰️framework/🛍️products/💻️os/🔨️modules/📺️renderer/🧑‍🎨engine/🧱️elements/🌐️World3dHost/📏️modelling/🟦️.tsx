// #region 🧲️Header
/** 📏️ The React World3d modelling layer: annotations (dimension, angle, point marker, leader label) drawn at
 * screen-constant size with an accessible list, scalar-field heatmaps with an accessible legend, the pick
 * granularity filter, the section plane with its stencil cap, and sub-element highlight tokens.
 *
 * The contract is the typed lanes `annotations`, `scalarField` and `modellingOptions` of the world scene
 * (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📏️world3d-modelling`); every function that decides something is pure
 * and is pinned against `🧫️fixtures/📏️world3d-modelling/🔣️.json`.
 * @see ../../../../../../../../🔨️modules/🖱️ui/🎬️scene/📏️world3d-modelling/🟦️.ts */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useMemo, useRef, useSyncExternalStore, type CSSProperties, type ReactElement } from "react";
import {
  AlwaysStencilFunc,
  BackSide,
  DecrementWrapStencilOp,
  DoubleSide,
  FrontSide,
  IncrementWrapStencilOp,
  Mesh,
  MeshBasicMaterial,
  NotEqualStencilFunc,
  Object3D,
  Quaternion,
  ReplaceStencilOp,
  sceneHostPort,
  ThreePlane,
  Vector3,
  type ThreeCamera,
} from "@semio-tech/ui-react";
import { resolveColorHex, themeColorVar, tokenVar } from "@semio-tech/ui-styling";
import {
  resolveWorld3dText,
  world3dPickTargets,
  world3dScalarFieldColorBytes,
  world3dScalarLegend,
  world3dSectionClipPlane,
  WORLD3D_COLOR_RAMPS,
  WORLD3D_SCALAR_NO_DATA_HEX,
  type World3dAnnotation,
  type World3dAnnotationLayer,
  type World3dHighlight,
  type World3dMarkerShape,
  type World3dPickGranularity,
  type World3dScalarField,
  type World3dSection,
  type World3dTone,
} from "@semio-tech/framework";

const { useFrame, useThree } = sceneHostPort.fiber;
// #endregion 🔌️Adapters

//#region 🎨️Tone
/** 🎨️ The CSS colour expression of a tone: `neutral` is the foreground, every other tone the palette token of the same name. */
export function world3dToneColorExpression(tone: World3dTone): string {
  return tone === "neutral" ? themeColorVar("foreground") : tokenVar(tone);
}

/** 🎨️ The resolved `#rrggbb` of a tone under the live theme. */
export function resolveWorld3dToneHex(tone: World3dTone): string {
  return resolveColorHex(world3dToneColorExpression(tone));
}
//#endregion 🎨️Tone

//#region 🔒️StableLane
/** 🔒️ Keeps a typed lane value's identity while its content hash is unchanged, so an unrelated refresh
 * that re-parses every JSON lane never recomputes what depends on this one. Without a hash it falls back to identity. */
export function useStableWorld3dLane<T>(value: T | undefined, hash: string | undefined): T | undefined {
  const held = useRef<{ readonly hash: string | undefined; readonly value: T | undefined } | null>(null);
  if (held.current === null || hash === undefined || held.current.hash !== hash) held.current = { hash, value };
  return hash === undefined ? value : held.current.value;
}
//#endregion 🔒️StableLane

//#region 🎯️PickFilter
export type World3dPickFilterSelection = {
  readonly selectionMode?: string;
  readonly granularity?: string;
  readonly componentIds?: readonly number[];
  readonly hoveredComponent?: { readonly mode?: string };
  readonly targets?: { readonly mesh?: boolean; readonly vertex?: boolean; readonly edge?: boolean; readonly face?: boolean; readonly exclusive?: boolean };
};

const PICK_SELECTION_MODE: Readonly<Record<World3dPickGranularity, string>> = { shape: "mesh", face: "face", edge: "edge", vertex: "vertex" };

/** 🎯️ Constrains a selection record to the pick filter: hover and selection reach only the chosen granularity,
 * component ids and a hovered component of another granularity are dropped, and the targets turn exclusive so
 * no sub-element miss falls back to a whole-shape hover. No filter returns the selection untouched. */
export function world3dSelectionWithPickFilter<S extends World3dPickFilterSelection>(selection: S, filter: World3dPickGranularity | undefined): S {
  if (filter === undefined) return selection;
  const mode = PICK_SELECTION_MODE[filter];
  const current = selection.selectionMode ?? selection.granularity ?? "mesh";
  const sameMode = filter !== "shape" && current === mode;
  const hovered = selection.hoveredComponent;
  return {
    ...selection,
    selectionMode: mode,
    targets: { ...world3dPickTargets(filter), exclusive: true },
    componentIds: sameMode ? selection.componentIds : [],
    hoveredComponent: filter !== "shape" && hovered?.mode === mode ? hovered : undefined,
  };
}
//#endregion 🎯️PickFilter

//#region 🖍️Highlight
export type World3dHighlightPaintEntry = { readonly hover?: string; readonly selected?: string; readonly widthPx?: number };
export type World3dHighlightPaint = { readonly face?: World3dHighlightPaintEntry; readonly edge?: World3dHighlightPaintEntry; readonly vertex?: World3dHighlightPaintEntry };

/** 🖍️ Resolves the highlight tokens to colours through the live theme. */
export function resolveWorld3dHighlightPaint(highlight: World3dHighlight | undefined, toneHex: (tone: World3dTone) => string = resolveWorld3dToneHex): World3dHighlightPaint | undefined {
  if (!highlight) return undefined;
  const entry = (style: World3dHighlight["face"]): World3dHighlightPaintEntry | undefined =>
    style ? { ...(style.hover ? { hover: toneHex(style.hover) } : {}), ...(style.selected ? { selected: toneHex(style.selected) } : {}), ...(style.widthPx === undefined ? {} : { widthPx: style.widthPx }) } : undefined;
  return { face: entry(highlight.face), edge: entry(highlight.edge), vertex: entry(highlight.vertex) };
}

export type World3dSubElementPaint = {
  readonly faceSelect: string;
  readonly faceHover: string;
  readonly edgeSelect: string;
  readonly edgeHover: string;
  readonly vertexSelect: string;
  readonly vertexHover: string;
  readonly edgeWidth: number;
  readonly vertexMarkPx: number;
};

/** 🖍️ The one paint table of sub-element hover and selection: theme defaults (secondary hover, primary selection)
 * unless a token overrides that granularity. */
export function world3dSubElementPaint(colors: { readonly select: string; readonly hover: string; readonly edgeHover: string }, paint: World3dHighlightPaint | undefined, defaults: { readonly edgeWidth: number; readonly vertexMarkPx: number }): World3dSubElementPaint {
  return {
    faceSelect: paint?.face?.selected ?? colors.select,
    faceHover: paint?.face?.hover ?? colors.hover,
    edgeSelect: paint?.edge?.selected ?? colors.select,
    edgeHover: paint?.edge?.hover ?? colors.edgeHover,
    vertexSelect: paint?.vertex?.selected ?? colors.select,
    vertexHover: paint?.vertex?.hover ?? colors.hover,
    edgeWidth: paint?.edge?.widthPx ?? defaults.edgeWidth,
    vertexMarkPx: paint?.vertex?.widthPx ?? defaults.vertexMarkPx,
  };
}
//#endregion 🖍️Highlight

//#region 🌡️Heatmap
type HeatmapMeshData = {
  readonly positions: readonly number[];
  readonly indices: readonly number[];
  readonly colors?: readonly number[];
  readonly attributes?: Record<string, { readonly semantic: string }>;
};

function srgbByteToLinear(byte: number): number {
  const value = byte / 255;
  return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
}

/** 🌡️ The mesh data painted with the scalar field, or `null` when the field does not fit the mesh (its value count differs from the vertex or triangle count).
 * A vertex field becomes the canonical per-vertex RGBA colours; a face field becomes a constant per-triangle colour attribute. Picking ids are untouched. */
export function world3dMeshDataWithScalarField<M extends HeatmapMeshData>(data: M, field: World3dScalarField): (M & { readonly heatmap: true }) | null {
  const bytes = world3dScalarFieldColorBytes(field);
  const attributes = Object.fromEntries(Object.entries(data.attributes ?? {}).filter(([, attribute]) => attribute.semantic !== "color"));
  if (field.domain === "vertex") {
    if (field.values.length !== data.positions.length / 3) return null;
    const colors = new Array<number>(field.values.length * 4);
    for (let index = 0; index < field.values.length; index += 1) {
      colors[index * 4] = srgbByteToLinear(bytes[index * 3]!);
      colors[index * 4 + 1] = srgbByteToLinear(bytes[index * 3 + 1]!);
      colors[index * 4 + 2] = srgbByteToLinear(bytes[index * 3 + 2]!);
      colors[index * 4 + 3] = 1;
    }
    return { ...data, colors, attributes, heatmap: true } as unknown as M & { readonly heatmap: true };
  }
  if (field.values.length !== data.indices.length / 3) return null;
  const values = Array.from({ length: field.values.length }, (_, index) => [srgbByteToLinear(bytes[index * 3]!), srgbByteToLinear(bytes[index * 3 + 1]!), srgbByteToLinear(bytes[index * 3 + 2]!), 1]);
  return { ...data, colors: undefined, attributes: { ...attributes, scalarField: { domain: "face", semantic: "color", interpolation: "constant", values } }, heatmap: true } as unknown as M & { readonly heatmap: true };
}

/** 🌡️ Replaces the record of the field's mesh by its painted twin; every other record keeps its identity so retained visuals are reused. */
export function world3dMeshRecordsWithScalarField<R extends { readonly id: string; readonly data?: HeatmapMeshData }>(records: readonly R[], field: World3dScalarField | undefined): { readonly records: readonly R[]; readonly status: "none" | "applied" | "mismatch" | "missing" } {
  if (!field) return { records, status: "none" };
  const index = records.findIndex((record) => record.id === field.meshId);
  const record = records[index];
  if (!record?.data) return { records, status: "missing" };
  const painted = world3dMeshDataWithScalarField(record.data, field);
  if (!painted) return { records, status: "mismatch" };
  const next = records.slice();
  next[index] = { ...record, data: painted };
  return { records: next, status: "applied" };
}

/** 🌡️ Memoised {@link world3dMeshRecordsWithScalarField}; the painted record identity is stable until the field or the mesh changes. */
export function useWorld3dScalarFieldMeshes<R extends { readonly id: string; readonly data?: HeatmapMeshData }>(records: readonly R[], field: World3dScalarField | undefined) {
  return useMemo(() => world3dMeshRecordsWithScalarField(records, field), [records, field]);
}
//#endregion 🌡️Heatmap

//#region 🏷️Strings
type Strings = {
  readonly annotations: string;
  readonly dimension: string;
  readonly angle: string;
  readonly marker: string;
  readonly leader: string;
  readonly legend: string;
  readonly scale: string;
  readonly noData: string;
  readonly mismatch: string;
};

const STRINGS: Readonly<Record<"en" | "de", Strings>> = {
  en: { annotations: "Annotations", dimension: "Dimension", angle: "Angle", marker: "Marker", leader: "Label", legend: "Legend", scale: "Scale values", noData: "No data", mismatch: "The scalar field does not match the mesh" },
  de: { annotations: "Anmerkungen", dimension: "Bemaßung", angle: "Winkel", marker: "Markierung", leader: "Beschriftung", legend: "Legende", scale: "Skalenwerte", noData: "Keine Daten", mismatch: "Das Skalarfeld passt nicht zum Netz" },
};

/** 🏷️ The fixed strings of the modelling layer for a locale (German for `de*`, English otherwise). */
export function world3dModellingStrings(locale: string | undefined): Strings {
  return (locale ?? "en").toLowerCase().split(/[-_]/)[0] === "de" ? STRINGS.de : STRINGS.en;
}

/** 🏷️ A legend value in the locale's number format with at most four significant digits; the wgpu twin `format_world3d_legend_value` is pinned against the same fixture rows. */
export function formatWorld3dLegendValue(value: number, locale: string | undefined): string {
  try {
    return new Intl.NumberFormat(locale, { maximumSignificantDigits: 4 }).format(value);
  } catch {
    return String(value);
  }
}
//#endregion 🏷️Strings

//#region 📏️Annotations
export type World3dScreenSize = { readonly width: number; readonly height: number };

export type World3dProjectedAnnotation = {
  readonly id: string;
  readonly kind: World3dAnnotation["kind"];
  readonly tone: World3dTone;
  readonly visible: boolean;
  readonly lines: readonly (readonly [number, number, number, number])[];
  readonly arrows: readonly { readonly x: number; readonly y: number; readonly angle: number }[];
  readonly arc?: { readonly cx: number; readonly cy: number; readonly radius: number; readonly start: number; readonly sweep: number };
  readonly marker?: { readonly x: number; readonly y: number; readonly shape: World3dMarkerShape };
  readonly label?: { readonly x: number; readonly y: number; readonly align: "middle" | "start" | "end" };
};

/** 📏️ Screen-constant sizes in CSS pixels: they never scale with zoom or distance. */
export const WORLD3D_ANNOTATION_METRICS = { arrowPx: 9, markerPx: 10, labelGapPx: 14, strokePx: 1.5 } as const;

/** 📏️ Projects the annotation layer through a camera into screen pixels (`x` right, `y` down, origin top-left).
 * Pure: the same camera, size and layer always give the same geometry. An annotation with any anchor behind the
 * camera or outside the depth range is invisible. Occlusion by the model is deliberately not tested, as in CAD dimensioning. */
export function projectWorld3dAnnotations(layer: World3dAnnotationLayer, camera: ThreeCamera, size: World3dScreenSize): readonly World3dProjectedAnnotation[] {
  const scratch = new Vector3();
  const point = (position: readonly [number, number, number]) => {
    scratch.set(position[0], position[1], position[2]).project(camera);
    return { x: ((scratch.x + 1) / 2) * size.width, y: ((1 - scratch.y) / 2) * size.height, ok: Number.isFinite(scratch.x) && Number.isFinite(scratch.y) && scratch.z >= -1 && scratch.z <= 1 };
  };
  const hidden = (item: World3dAnnotation): World3dProjectedAnnotation => ({ id: item.id, kind: item.kind, tone: item.tone, visible: false, lines: [], arrows: [] });
  return layer.items.map((item): World3dProjectedAnnotation => {
    if (item.kind === "dimension") {
      const [from, to] = [point(item.from), point(item.to)];
      const [a, b] = [point([item.from[0] + item.offset[0], item.from[1] + item.offset[1], item.from[2] + item.offset[2]]), point([item.to[0] + item.offset[0], item.to[1] + item.offset[1], item.to[2] + item.offset[2]])];
      if (![from, to, a, b].every((entry) => entry.ok)) return hidden(item);
      const [dx, dy] = [b.x - a.x, b.y - a.y];
      const length = Math.hypot(dx, dy);
      const normal = length > 1e-9 ? ([-dy / length, dx / length] as const) : ([0, -1] as const);
      const up = normal[1] <= 0 ? 1 : -1;
      const heading = Math.atan2(dy, dx);
      return {
        id: item.id,
        kind: item.kind,
        tone: item.tone,
        visible: true,
        lines: [[from.x, from.y, a.x, a.y], [to.x, to.y, b.x, b.y], [a.x, a.y, b.x, b.y]],
        arrows: [{ x: a.x, y: a.y, angle: heading + Math.PI }, { x: b.x, y: b.y, angle: heading }],
        label: { x: (a.x + b.x) / 2 + up * normal[0] * WORLD3D_ANNOTATION_METRICS.labelGapPx, y: (a.y + b.y) / 2 + up * normal[1] * WORLD3D_ANNOTATION_METRICS.labelGapPx, align: "middle" },
      };
    }
    if (item.kind === "angle") {
      const vertex = point(item.vertex);
      const [ta, tb] = [point([item.vertex[0] + item.directionA[0], item.vertex[1] + item.directionA[1], item.vertex[2] + item.directionA[2]]), point([item.vertex[0] + item.directionB[0], item.vertex[1] + item.directionB[1], item.vertex[2] + item.directionB[2]])];
      if (![vertex, ta, tb].every((entry) => entry.ok)) return hidden(item);
      const [ax, ay, bx, by] = [ta.x - vertex.x, ta.y - vertex.y, tb.x - vertex.x, tb.y - vertex.y];
      if (Math.hypot(ax, ay) < 1e-6 || Math.hypot(bx, by) < 1e-6) return hidden(item);
      const start = Math.atan2(ay, ax);
      let sweep = Math.atan2(by, bx) - start;
      while (sweep > Math.PI) sweep -= 2 * Math.PI;
      while (sweep <= -Math.PI) sweep += 2 * Math.PI;
      const middle = start + sweep / 2;
      const labelRadius = item.radiusPx + WORLD3D_ANNOTATION_METRICS.labelGapPx;
      return {
        id: item.id,
        kind: item.kind,
        tone: item.tone,
        visible: true,
        lines: [[vertex.x, vertex.y, vertex.x + Math.cos(start) * item.radiusPx, vertex.y + Math.sin(start) * item.radiusPx], [vertex.x, vertex.y, vertex.x + Math.cos(start + sweep) * item.radiusPx, vertex.y + Math.sin(start + sweep) * item.radiusPx]],
        arrows: [],
        arc: { cx: vertex.x, cy: vertex.y, radius: item.radiusPx, start, sweep },
        label: { x: vertex.x + Math.cos(middle) * labelRadius, y: vertex.y + Math.sin(middle) * labelRadius, align: "middle" },
      };
    }
    if (item.kind === "marker") {
      const position = point(item.position);
      return position.ok ? { id: item.id, kind: item.kind, tone: item.tone, visible: true, lines: [], arrows: [], marker: { x: position.x, y: position.y, shape: item.shape } } : hidden(item);
    }
    const anchor = point(item.anchor);
    if (!anchor.ok) return hidden(item);
    const label = { x: anchor.x + item.labelOffsetPx[0], y: anchor.y + item.labelOffsetPx[1] };
    return { id: item.id, kind: item.kind, tone: item.tone, visible: true, lines: [[anchor.x, anchor.y, label.x, label.y]], arrows: [], marker: { x: anchor.x, y: anchor.y, shape: "dot" }, label: { ...label, align: item.labelOffsetPx[0] >= 0 ? "start" : "end" } };
  });
}

/** 📡️ The projected annotations a camera frame produced: written by {@link World3dAnnotationProjector} inside the canvas, read by {@link World3dAnnotationOverlay} outside it. */
export type World3dAnnotationStore = {
  readonly get: () => readonly World3dProjectedAnnotation[];
  readonly set: (next: readonly World3dProjectedAnnotation[]) => void;
  readonly subscribe: (listener: () => void) => () => void;
};

export function createWorld3dAnnotationStore(): World3dAnnotationStore {
  let current: readonly World3dProjectedAnnotation[] = [];
  const listeners = new Set<() => void>();
  return {
    get: () => current,
    set: (next) => {
      current = next;
      for (const listener of listeners) listener();
    },
    subscribe: (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  };
}

/** 📷️ Projects the layer on every frame the camera or the viewport size changed. Mounted inside the canvas. */
export function World3dAnnotationProjector({ layer, store }: { readonly layer: World3dAnnotationLayer; readonly store: World3dAnnotationStore }): null {
  const invalidate = useThree((state) => state.invalidate);
  const seen = useRef<{ readonly layer: World3dAnnotationLayer | null; readonly view: Float64Array; readonly width: number; readonly height: number }>({ layer: null, view: new Float64Array(32), width: -1, height: -1 });
  useEffect(() => {
    seen.current = { layer: null, view: seen.current.view, width: -1, height: -1 };
    invalidate();
    return () => store.set([]);
  }, [layer, invalidate, store]);
  useFrame(({ camera, size }) => {
    camera.updateMatrixWorld();
    const view = seen.current.view;
    const matrices = [camera.matrixWorldInverse.elements, camera.projectionMatrix.elements];
    let changed = seen.current.layer !== layer || seen.current.width !== size.width || seen.current.height !== size.height;
    for (let matrix = 0; matrix < 2; matrix += 1) {
      for (let element = 0; element < 16; element += 1) {
        const value = matrices[matrix]![element]!;
        if (view[matrix * 16 + element] !== value) {
          view[matrix * 16 + element] = value;
          changed = true;
        }
      }
    }
    if (!changed) return;
    seen.current = { layer, view, width: size.width, height: size.height };
    store.set(projectWorld3dAnnotations(layer, camera, size));
  });
  return null;
}

const VISUALLY_HIDDEN: CSSProperties = { position: "absolute", width: 1, height: 1, margin: -1, padding: 0, overflow: "hidden", clip: "rect(0 0 0 0)", whiteSpace: "nowrap", border: 0 };

function arrowPoints(x: number, y: number, angle: number): string {
  const length = WORLD3D_ANNOTATION_METRICS.arrowPx;
  const [cos, sin] = [Math.cos(angle), Math.sin(angle)];
  const [bx, by] = [x - cos * length, y - sin * length];
  const half = length * 0.35;
  return `${x},${y} ${bx - sin * half},${by + cos * half} ${bx + sin * half},${by - cos * half}`;
}

function arcPath(arc: NonNullable<World3dProjectedAnnotation["arc"]>): string {
  const [x0, y0] = [arc.cx + Math.cos(arc.start) * arc.radius, arc.cy + Math.sin(arc.start) * arc.radius];
  const [x1, y1] = [arc.cx + Math.cos(arc.start + arc.sweep) * arc.radius, arc.cy + Math.sin(arc.start + arc.sweep) * arc.radius];
  return `M ${x0} ${y0} A ${arc.radius} ${arc.radius} 0 0 ${arc.sweep > 0 ? 1 : 0} ${x1} ${y1}`;
}

function markerShape(marker: NonNullable<World3dProjectedAnnotation["marker"]>): ReactElement {
  const r = WORLD3D_ANNOTATION_METRICS.markerPx / 2;
  if (marker.shape === "cross") return <path d={`M ${marker.x - r} ${marker.y - r} L ${marker.x + r} ${marker.y + r} M ${marker.x - r} ${marker.y + r} L ${marker.x + r} ${marker.y - r}`} fill="none" />;
  if (marker.shape === "ring") return <circle cx={marker.x} cy={marker.y} r={r} fill="none" />;
  return <circle cx={marker.x} cy={marker.y} r={r / 2} fill="currentColor" stroke="none" />;
}

/** 📏️ The annotation layer as DOM: an SVG for the lines, arrowheads, arcs and markers, absolutely positioned labels, and a visually hidden
 * list that names every annotation for assistive technology (the visual layer is `aria-hidden`). Screen-constant by construction: every size is a CSS pixel. */
export function World3dAnnotationOverlay({ layer, store, locale, toneHex = resolveWorld3dToneHex, className }: { readonly layer: World3dAnnotationLayer; readonly store: World3dAnnotationStore; readonly locale?: string; readonly toneHex?: (tone: World3dTone) => string; readonly className?: string }): ReactElement {
  const projected = useSyncExternalStore(store.subscribe, store.get, store.get);
  const strings = world3dModellingStrings(locale);
  const language = locale ?? "en";
  const byId = new Map(projected.map((entry) => [entry.id, entry]));
  const heading = layer.title ? resolveWorld3dText(layer.title, language) : strings.annotations;
  return (
    <div role="region" aria-label={heading} data-slot="world-annotation-layer" data-annotation-count={layer.items.length} className="pointer-events-none absolute inset-0 overflow-hidden">
      <ul data-slot="world-annotation-list" style={VISUALLY_HIDDEN}>
        {layer.items.map((item) => (
          <li key={item.id} data-annotation-id={item.id} data-annotation-kind={item.kind}>
            {strings[item.kind === "dimension" ? "dimension" : item.kind === "angle" ? "angle" : item.kind === "marker" ? "marker" : "leader"]}: {resolveWorld3dText(item.text, language)}
          </li>
        ))}
      </ul>
      <svg aria-hidden="true" focusable="false" className={className ?? "absolute inset-0 h-full w-full"} data-slot="world-annotation-geometry" style={{ overflow: "visible" }}>
        {layer.items.map((item) => {
          const entry = byId.get(item.id);
          if (!entry?.visible) return null;
          return (
            <g key={item.id} data-annotation-id={item.id} data-annotation-kind={item.kind} data-tone={item.tone} style={{ color: toneHex(item.tone) }} stroke="currentColor" strokeWidth={WORLD3D_ANNOTATION_METRICS.strokePx} fill="none">
              {entry.lines.map((line, index) => (
                <line key={index} x1={line[0]} y1={line[1]} x2={line[2]} y2={line[3]} />
              ))}
              {entry.arc ? <path d={arcPath(entry.arc)} /> : null}
              {entry.arrows.map((arrow, index) => (
                <polygon key={index} points={arrowPoints(arrow.x, arrow.y, arrow.angle)} fill="currentColor" stroke="none" />
              ))}
              {entry.marker ? markerShape(entry.marker) : null}
            </g>
          );
        })}
      </svg>
      {layer.items.map((item) => {
        const label = byId.get(item.id)?.label;
        if (!label || !byId.get(item.id)?.visible) return null;
        const shift = label.align === "middle" ? "-50%" : label.align === "start" ? "0%" : "-100%";
        return (
          <span
            key={item.id}
            aria-hidden="true"
            data-slot="world-annotation-label"
            data-annotation-id={item.id}
            className="absolute whitespace-nowrap rounded-sm bg-background/80 px-1 text-xs"
            style={{ left: 0, top: 0, transform: `translate(${label.x}px, ${label.y}px) translate(${shift}, -50%)`, color: toneHex(item.tone) }}
          >
            {resolveWorld3dText(item.text, language)}
          </span>
        );
      })}
    </div>
  );
}
//#endregion 📏️Annotations

//#region 🏷️Legend
/** 🏷️ The CSS gradient of a ramp, bottom (range minimum) to top (range maximum). */
export function world3dRampGradient(ramp: World3dScalarField["ramp"]): string {
  const stops = WORLD3D_COLOR_RAMPS[ramp];
  return `linear-gradient(to top, ${stops.map((hex, index) => `${hex} ${Math.round((index / (stops.length - 1)) * 100)}%`).join(", ")})`;
}

export type World3dLegendLines = {
  readonly title: string;
  readonly ticks: readonly { readonly text: string; readonly hex: string }[];
  readonly noData: string | null;
};

/** 🏷️ The text of a scalar-field legend for a locale: the title with its unit, one line per tick (value and unit, bottom to top) and the no-data caption when any value is missing.
 * Pure: the DOM legend below and the wgpu legend paint exactly these strings. */
export function world3dLegendLines(field: World3dScalarField, locale: string | undefined): World3dLegendLines {
  const language = locale ?? "en";
  const title = resolveWorld3dText(field.legend.title, language);
  const unit = field.legend.unit;
  return {
    title: unit ? `${title} (${unit})` : title,
    ticks: world3dScalarLegend(field).map((tick) => ({ text: `${formatWorld3dLegendValue(tick.value, language)}${unit ? ` ${unit}` : ""}`, hex: tick.hex })),
    noData: field.values.some((value) => value === null) ? world3dModellingStrings(locale).noData : null,
  };
}

/** 🏷️ The on-screen legend of a scalar field: title, unit, a ramp gradient and the evenly spaced tick values as an accessible list
 * (the gradient is decorative; the list carries the scale). */
export function World3dScalarLegendView({ field, locale, className }: { readonly field: World3dScalarField; readonly locale?: string; readonly className?: string }): ReactElement {
  const strings = world3dModellingStrings(locale);
  const language = locale ?? "en";
  const ticks = useMemo(() => world3dScalarLegend(field), [field]);
  const title = resolveWorld3dText(field.legend.title, language);
  const unit = field.legend.unit;
  const hasNoData = field.values.some((value) => value === null);
  return (
    <div role="group" aria-label={`${strings.legend}: ${title}`} data-slot="world-scalar-legend" data-ramp={field.ramp} data-mesh-id={field.meshId} className={className ?? "pointer-events-none absolute bottom-2 left-2 z-40 rounded px-single py-half text-xs shadow-sm"}>
      <div data-slot="world-scalar-legend-title" className="font-medium">
        {unit ? `${title} (${unit})` : title}
      </div>
      <div className="flex gap-single">
        <div aria-hidden="true" data-slot="world-scalar-legend-ramp" style={{ width: 12, minHeight: 96, background: world3dRampGradient(field.ramp), borderRadius: 2 }} />
        <ol aria-label={strings.scale} data-slot="world-scalar-legend-ticks" className="flex flex-col-reverse justify-between">
          {ticks.map((tick, index) => (
            <li key={index} data-slot="world-scalar-legend-tick" data-hex={tick.hex}>
              {formatWorld3dLegendValue(tick.value, language)}
              {unit ? ` ${unit}` : ""}
            </li>
          ))}
        </ol>
      </div>
      {hasNoData ? (
        <div data-slot="world-scalar-legend-no-data" className="flex items-center gap-half">
          <span aria-hidden="true" style={{ display: "inline-block", width: 10, height: 10, borderRadius: 2, background: WORLD3D_SCALAR_NO_DATA_HEX }} />
          <span>{strings.noData}</span>
        </div>
      ) : null}
    </div>
  );
}
//#endregion 🏷️Legend

//#region ✂️Section
/** ✂️ The three.js clipping plane of a section. */
export function world3dSectionPlane(section: World3dSection): ThreePlane {
  const [x, y, z, constant] = world3dSectionClipPlane(section);
  return new ThreePlane(new Vector3(x, y, z), constant);
}

type ClippableMaterial = { clippingPlanes: ThreePlane[] | null; clipShadows: boolean; needsUpdate: boolean };

/** ✂️ Assigns (or clears) the clipping planes of every material inside an instance root (`userData.world3dInstanceRoot`) under `root`;
 * the gumball, grid and other scene furniture stay unclipped, and subtrees flagged `userData.world3dSectionExempt` are skipped.
 * Returns how many materials changed. A material whose plane set toggles between none and some is flagged for recompilation. */
export function applyWorld3dSectionClipping(root: Object3D, planes: readonly ThreePlane[] | null): number {
  let changed = 0;
  const target = planes === null || planes.length === 0 ? null : (planes as ThreePlane[]);
  const visit = (object: Object3D, inside: boolean) => {
    if (object.userData.world3dSectionExempt === true) return;
    const within = inside || object.userData.world3dInstanceRoot === true;
    const material = within ? ((object as Mesh).material as ClippableMaterial | ClippableMaterial[] | undefined) : undefined;
    for (const entry of material === undefined ? [] : Array.isArray(material) ? material : [material]) {
      const current = entry.clippingPlanes;
      if (current === target || (target !== null && current?.length === target.length && current.every((plane, index) => plane === target[index]))) continue;
      if ((current === null) !== (target === null)) entry.needsUpdate = true;
      entry.clippingPlanes = target;
      entry.clipShadows = target !== null;
      changed += 1;
    }
    for (const child of object.children) visit(child, within);
  };
  visit(root, false);
  return changed;
}

/** ✂️ The solid shaded meshes the cap closes: those flagged `userData.world3dSolid`. */
export function collectWorld3dSolidMeshes(root: Object3D): Mesh[] {
  const solids: Mesh[] = [];
  const visit = (object: Object3D) => {
    if (object.userData.world3dSolid === true && (object as Mesh).isMesh) solids.push(object as Mesh);
    for (const child of object.children) visit(child);
  };
  visit(root);
  return solids;
}

/** ✂️ The stencil half of a section cap: for every solid mesh, a back-face pass that increments and a front-face pass that decrements the
 * stencil inside the clipped volume, so the stencil is non-zero exactly where the cut exposes the inside. Both passes write no colour or depth. */
export class World3dSectionCapStencils {
  private readonly attached = new Map<Mesh, readonly [Mesh, Mesh]>();
  private readonly back: MeshBasicMaterial;
  private readonly front: MeshBasicMaterial;

  constructor(plane: ThreePlane) {
    const stencil = (side: typeof BackSide | typeof FrontSide, operation: typeof IncrementWrapStencilOp | typeof DecrementWrapStencilOp) =>
      new MeshBasicMaterial({ side, clippingPlanes: [plane], colorWrite: false, depthWrite: false, depthTest: false, stencilWrite: true, stencilFunc: AlwaysStencilFunc, stencilFail: operation, stencilZFail: operation, stencilZPass: operation });
    this.back = stencil(BackSide, IncrementWrapStencilOp);
    this.front = stencil(FrontSide, DecrementWrapStencilOp);
  }

  get count(): number {
    return this.attached.size;
  }

  sync(root: Object3D): void {
    const solids = new Set(collectWorld3dSolidMeshes(root));
    for (const [solid, passes] of this.attached) {
      if (solids.has(solid) && passes[0].geometry === solid.geometry) continue;
      for (const pass of passes) solid.remove(pass);
      this.attached.delete(solid);
    }
    for (const solid of solids) {
      if (this.attached.has(solid)) continue;
      const [back, front] = [new Mesh(solid.geometry, this.back), new Mesh(solid.geometry, this.front)];
      for (const pass of [back, front]) {
        pass.userData.world3dSectionExempt = true;
        pass.renderOrder = 1;
        pass.raycast = () => undefined;
        solid.add(pass);
      }
      this.attached.set(solid, [back, front]);
    }
  }

  dispose(): void {
    for (const [solid, passes] of this.attached) for (const pass of passes) solid.remove(pass);
    this.attached.clear();
    this.back.dispose();
    this.front.dispose();
  }
}

/** ✂️ Clips every material of the instances group by the section plane and, with a cap, closes the cut with a stencilled plane of `capHex`.
 * Mounted inside the canvas; `groupRef` is the instances root. */
export function World3dSectionClip({ section, groupRef, capHex, extent }: { readonly section: World3dSection; readonly groupRef: { readonly current: Object3D | null }; readonly capHex?: string; readonly extent: number }): ReactElement | null {
  const gl = useThree((state) => state.gl);
  const invalidate = useThree((state) => state.invalidate);
  const plane = useMemo(() => world3dSectionPlane(section), [section]);
  const stencils = useMemo(() => (section.cap ? new World3dSectionCapStencils(plane) : null), [plane, section.cap]);
  const capRef = useRef<Mesh | null>(null);
  useEffect(() => {
    const previous = gl.localClippingEnabled;
    gl.localClippingEnabled = true;
    invalidate();
    return () => {
      gl.localClippingEnabled = previous;
      if (groupRef.current) applyWorld3dSectionClipping(groupRef.current, null);
      stencils?.dispose();
      invalidate();
    };
  }, [gl, groupRef, invalidate, stencils]);
  useFrame(() => {
    const root = groupRef.current;
    if (!root) return;
    applyWorld3dSectionClipping(root, [plane]);
    stencils?.sync(root);
  });
  const origin = useMemo(() => new Vector3(section.origin[0], section.origin[1], section.origin[2]), [section.origin]);
  const orientation = useMemo(() => new Quaternion().setFromUnitVectors(new Vector3(0, 0, 1), new Vector3(section.normal[0], section.normal[1], section.normal[2]).normalize()), [section.normal]);
  if (!section.cap) return null;
  return (
    <mesh ref={capRef} position={origin} quaternion={orientation} renderOrder={2} raycast={() => null} userData={{ world3dSectionExempt: true, world3dSectionCap: true }} onAfterRender={(renderer: { clearStencil: () => void }) => renderer.clearStencil()}>
      <planeGeometry args={[extent * 2, extent * 2]} />
      <meshBasicMaterial color={capHex ?? "#808080"} side={DoubleSide} stencilWrite stencilRef={0} stencilFunc={NotEqualStencilFunc} stencilFail={ReplaceStencilOp} stencilZFail={ReplaceStencilOp} stencilZPass={ReplaceStencilOp} />
    </mesh>
  );
}
//#endregion ✂️Section

//#region 🪪️GlbSubElementIds
/** 🪪️ The custom per-vertex glTF attributes a B-Rep export carries so a mesh viewer can resolve a triangle or a vertex to its topology id.
 * The wgpu renderer reads the same two attributes (`_FACE_ID` at the triangle's first corner, `_VERTEX_ID` per vertex), so both renderers pick from one source. */
export const WORLD3D_GLB_FACE_ID_ATTRIBUTE = "_FACE_ID";
export const WORLD3D_GLB_VERTEX_ID_ATTRIBUTE = "_VERTEX_ID";

type GlbAttributeLike = { readonly count: number; getX(index: number): number };
type GlbGeometryLike = { readonly index: GlbAttributeLike | null; getAttribute(name: string): GlbAttributeLike | undefined };

export type World3dGlbSubElementIds = {
  /** One topology id per triangle, read at the triangle's first corner; `null` when the geometry carries no `_FACE_ID`. */
  readonly faceIds: readonly number[] | null;
  /** One topology id per vertex; `null` when the geometry carries no `_VERTEX_ID`. */
  readonly vertexIds: readonly number[] | null;
};

function glbAttribute(geometry: GlbGeometryLike, name: string): GlbAttributeLike | undefined {
  return geometry.getAttribute(name) ?? geometry.getAttribute(name.toLowerCase());
}

/** 🪪️ Reads the sub-element ids of one loaded GLB primitive. three's `GLTFLoader` lower-cases custom attribute names, so `_face_id` is accepted beside `_FACE_ID`. */
export function world3dGlbSubElementIds(geometry: GlbGeometryLike): World3dGlbSubElementIds {
  const face = glbAttribute(geometry, WORLD3D_GLB_FACE_ID_ATTRIBUTE);
  const vertex = glbAttribute(geometry, WORLD3D_GLB_VERTEX_ID_ATTRIBUTE);
  const corners = geometry.index?.count ?? geometry.getAttribute("position")?.count ?? 0;
  const faceIds = face
    ? Array.from({ length: Math.floor(corners / 3) }, (_, triangle) => face.getX(geometry.index ? geometry.index.getX(triangle * 3) : triangle * 3))
    : null;
  const vertexIds = vertex ? Array.from({ length: vertex.count }, (_, index) => vertex.getX(index)) : null;
  return { faceIds, vertexIds };
}

const GLB_HIT_IDS = new WeakMap<object, World3dGlbSubElementIds>();

/** 🪪️ The topology face id of a pointer hit on a loaded GLB mesh, or `undefined` when the hit primitive carries no `_FACE_ID` (the hit then selects the whole shape). */
export function world3dGlbHitFaceId(object: Object3D | undefined, faceIndex: number | null | undefined): number | undefined {
  const geometry = (object as Mesh | undefined)?.geometry as unknown as GlbGeometryLike | undefined;
  if (!geometry || faceIndex === null || faceIndex === undefined) return undefined;
  let ids = GLB_HIT_IDS.get(geometry);
  if (!ids) {
    ids = world3dGlbSubElementIds(geometry);
    GLB_HIT_IDS.set(geometry, ids);
  }
  return ids.faceIds?.[faceIndex];
}

type GlbPositionLike = GlbAttributeLike & { getY(index: number): number; getZ(index: number): number };

/** 🪪️ The merged picking mesh of a loaded GLB scene: positions in the scene's own frame, indices into them and the sub-element ids,
 * each id table present only when EVERY primitive carries it (a mixed bank would leave anonymous gaps). `null` when the scene has no triangles. */
export function world3dGlbSubElementMeshData(root: Object3D): { readonly positions: readonly number[]; readonly indices: readonly number[]; readonly faceIds?: readonly number[]; readonly vertexIds?: readonly number[] } | null {
  root.updateMatrixWorld(true);
  const inverse = root.matrixWorld.clone().invert();
  const positions: number[] = [];
  const indices: number[] = [];
  const faceIds: number[] = [];
  const vertexIds: number[] = [];
  let everyFace = true;
  let everyVertex = true;
  const scratch = new Vector3();
  root.traverse((object) => {
    const geometry = (object as Mesh).isMesh ? ((object as Mesh).geometry as unknown as GlbGeometryLike) : null;
    const position = geometry?.getAttribute("position") as GlbPositionLike | undefined;
    if (!geometry || !position) return;
    const to = inverse.clone().multiply(object.matrixWorld);
    const base = positions.length / 3;
    for (let index = 0; index < position.count; index += 1) {
      scratch.set(position.getX(index), position.getY(index), position.getZ(index)).applyMatrix4(to);
      positions.push(scratch.x, scratch.y, scratch.z);
    }
    const count = geometry.index?.count ?? position.count;
    for (let corner = 0; corner < count; corner += 1) indices.push(base + (geometry.index ? geometry.index.getX(corner) : corner));
    const ids = world3dGlbSubElementIds(geometry);
    if (ids.faceIds) faceIds.push(...ids.faceIds);
    else everyFace = false;
    if (ids.vertexIds && ids.vertexIds.length === position.count) vertexIds.push(...ids.vertexIds);
    else everyVertex = false;
  });
  if (indices.length === 0) return null;
  return { positions, indices, ...(everyFace ? { faceIds } : {}), ...(everyVertex ? { vertexIds } : {}) };
}
//#endregion 🪪️GlbSubElementIds

//#region 🔎️Data
/** 🔎️ DOM data attributes for probes and tests: the pick filter, the section state and the heatmap status. */
export function world3dModellingDataAttributes(state: { readonly pickFilter?: World3dPickGranularity; readonly section?: World3dSection; readonly scalarField?: World3dScalarField; readonly scalarStatus?: string; readonly annotationCount?: number }): Record<string, string | undefined> {
  return {
    "data-world-pick-filter": state.pickFilter,
    "data-world-section": state.section ? (state.section.cap ? "capped" : "open") : undefined,
    "data-world-scalar-field": state.scalarField ? `${state.scalarField.meshId}:${state.scalarStatus ?? "none"}` : undefined,
    "data-world-annotation-count": state.annotationCount === undefined ? undefined : String(state.annotationCount),
  };
}
//#endregion 🔎️Data
