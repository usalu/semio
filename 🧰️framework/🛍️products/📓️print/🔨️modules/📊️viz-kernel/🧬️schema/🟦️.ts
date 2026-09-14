/** 🧬️ Typed twin of `🧬️schema/🔣️.json` for the visualization kernel: the §79 grammar vocabulary
 * every kernel module speaks, plus the catalogue and probe record shapes the schema declares.
 * @see ../../../🧬️schema/🔣️.json
 */

//#region 🔖️Localization
/** 🌍️ A user-visible string in every language the print product ships; there is no default one. */
export type VizLocalizedText = { readonly en: string; readonly de: string };

/** 🌍️ The languages a kernel result may be labelled in. */
export const VIZ_LANGUAGES = ["en", "de"] as const;

export type VizLanguage = (typeof VIZ_LANGUAGES)[number];
//#endregion 🔖️Localization

//#region 🔖️Primitives
/** 📍️ A point in figure millimetres; the kernel never carries units. */
export type VizPoint = readonly [number, number];

/** 📦️ An axis-aligned extent in figure millimetres. */
export type VizExtent = { readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number };

/** 🔢️ Any value a grammar option may carry. */
export type VizOptionValue = string | number | boolean;

/** 🗄️ One row of a data table; columns are addressed by name. */
export type VizRow = Readonly<Record<string, VizOptionValue | null>>;

/** 🗄️ A named, column-oriented data table as `\SemioVizTable` declares it. */
export type VizTable = { readonly name: string; readonly columns: readonly string[]; readonly rows: readonly VizRow[] };
//#endregion 🔖️Primitives

//#region 🔖️Scale
/** 📐️ Every scale kind the grammar exposes, mirroring `semio-viz-scale`. */
export const VIZ_SCALE_KINDS = ["linear", "log", "pow", "sqrt", "symlog", "identity", "ordinal", "band", "point", "quantile", "quantize", "threshold", "sequential", "diverging", "temporal"] as const;

export type VizScaleKind = (typeof VIZ_SCALE_KINDS)[number];

/** 📐️ The option vocabulary of `\SemioVizScale`; every key is optional and documented by its default. */
export type VizScaleOptions = {
  readonly nice?: boolean | number;
  readonly clamp?: boolean;
  readonly padding?: number;
  readonly paddingInner?: number;
  readonly paddingOuter?: number;
  readonly align?: number;
  readonly round?: boolean;
  readonly base?: number;
  readonly exponent?: number;
  readonly constant?: number;
  readonly ticks?: number;
  readonly tickValues?: readonly number[];
  readonly tickFormat?: string;
  readonly interpolator?: string;
  readonly scheme?: string;
  readonly reverse?: boolean;
  readonly unknown?: VizOptionValue;
};

/** 📐️ A scale declaration as the grammar records it, independent of the implementation that runs it. */
export type VizScaleSpec = {
  readonly name: string;
  readonly kind: VizScaleKind;
  readonly domain: readonly (number | string)[];
  readonly range: readonly (number | string)[];
  readonly options?: VizScaleOptions;
};
//#endregion 🔖️Scale

//#region 🔖️Marks
/** ✒️ The curve interpolators `\SemioVizPath` accepts; every one is a true interpolator. */
export const VIZ_CURVE_KINDS = ["linear", "linear-closed", "step", "step-before", "step-after", "basis", "basis-closed", "basis-open", "bundle", "cardinal", "cardinal-closed", "cardinal-open", "catmull-rom", "catmull-rom-closed", "catmull-rom-open", "monotone-x", "monotone-y", "natural", "bezier"] as const;

export type VizCurveKind = (typeof VIZ_CURVE_KINDS)[number];

/** ✒️ The symbol glyphs `\SemioVizSymbol` draws, sized by area like d3. */
export const VIZ_SYMBOL_KINDS = ["circle", "cross", "diamond", "square", "star", "triangle", "wye", "asterisk", "diamond2", "plus", "square2", "triangle2", "times"] as const;

export type VizSymbolKind = (typeof VIZ_SYMBOL_KINDS)[number];
//#endregion 🔖️Marks

//#region 🔖️Coordinate
/** 🧭️ The coordinate systems `\SemioVizCoordinate` installs inside a plot. */
export const VIZ_COORDINATE_KINDS = ["cartesian", "polar", "ternary", "barycentric", "parallel", "logpolar", "geographic"] as const;

export type VizCoordinateKind = (typeof VIZ_COORDINATE_KINDS)[number];

/** 🧭️ Option vocabulary shared by the coordinate systems; unused keys are ignored by a system. */
export type VizCoordinateOptions = {
  readonly startAngle?: number;
  readonly endAngle?: number;
  readonly innerRadius?: number;
  readonly outerRadius?: number;
  readonly base?: number;
  readonly axes?: readonly string[];
  readonly projection?: string;
  readonly transpose?: boolean;
};
//#endregion 🔖️Coordinate

//#region 🔖️Layout
/** 🧮️ Every layout algorithm `\SemioVizLayout` dispatches on (taxonomy §75 and §78). */
export const VIZ_LAYOUT_ALGORITHMS = ["stack", "bin", "hexbin", "beeswarm", "jitter", "pie", "arc", "chord", "sankey", "alluvial", "treemap", "partition", "pack", "force", "tree", "cluster", "dag", "bundling", "voronoi", "delaunay", "hull", "contour", "density", "projection"] as const;

export type VizLayoutAlgorithm = (typeof VIZ_LAYOUT_ALGORITHMS)[number];

/** 🧮️ The treemap tilings, in the order `semio-viz-hierarchy` registers them. */
export const VIZ_TREEMAP_TILINGS = ["squarify", "slice", "dice", "slice-dice", "binary", "resquarify"] as const;

export type VizTreemapTiling = (typeof VIZ_TREEMAP_TILINGS)[number];

/** 🧮️ Stack orders and offsets, mirroring `\SemioVizLayout{stack}`. */
export const VIZ_STACK_ORDERS = ["none", "ascending", "descending", "inside-out", "reverse", "appearance"] as const;

export const VIZ_STACK_OFFSETS = ["none", "expand", "diverging", "silhouette", "wiggle"] as const;

export type VizStackOrder = (typeof VIZ_STACK_ORDERS)[number];

export type VizStackOffset = (typeof VIZ_STACK_OFFSETS)[number];
//#endregion 🔖️Layout

//#region 🔖️Geo
/** 🌍️ The projections `semio-viz-geo` registers; each one is an exact d3-geo twin. */
export const VIZ_PROJECTIONS = ["equirectangular", "mercator", "transverse-mercator", "azimuthal-equal-area", "azimuthal-equidistant", "gnomonic", "orthographic", "stereographic", "conic-conformal", "conic-equal-area", "conic-equidistant", "albers", "equal-earth", "natural-earth"] as const;

export type VizProjectionKind = (typeof VIZ_PROJECTIONS)[number];
//#endregion 🔖️Geo

//#region 🔖️Specification
/** 🎨️ A resolved encoding channel: the column it reads and the scale that maps it. */
export type VizEncoding = { readonly column?: string; readonly scale?: string; readonly value?: VizOptionValue };

/** 🎨️ Every encoding channel `\SemioVizPlot` accepts. */
export const VIZ_CHANNELS = ["x", "y", "x2", "y2", "angle", "radius", "size", "shape", "fill", "stroke", "opacity", "text", "dash", "width", "order", "detail"] as const;

export type VizChannel = (typeof VIZ_CHANNELS)[number];

/** 🧮️ One transform step of the §79 pipeline. */
export type VizTransformSpec = { readonly kind: string; readonly options?: Readonly<Record<string, VizOptionValue | readonly VizOptionValue[]>> };

/** 🧱️ One layer of a chart: a mark, its encodings and the transform and layout feeding it. */
export type VizLayerSpec = {
  readonly mark: string;
  readonly data?: string;
  readonly transform?: readonly VizTransformSpec[];
  readonly layout?: { readonly algorithm: VizLayoutAlgorithm; readonly options?: Readonly<Record<string, VizOptionValue>> };
  readonly encodings?: Partial<Readonly<Record<VizChannel, VizEncoding>>>;
  readonly options?: Readonly<Record<string, VizOptionValue>>;
};

/** 📏️ A guide drawn beside the data rectangle. */
export type VizGuideSpec = {
  readonly kind: "axis" | "legend" | "grid";
  readonly scale?: string;
  readonly orient?: "top" | "right" | "bottom" | "left";
  readonly title?: VizLocalizedText;
  readonly ticks?: number;
  readonly tickValues?: readonly number[];
  readonly tickFormat?: string;
  readonly tickSize?: number;
  readonly minor?: boolean;
  readonly grid?: boolean;
  readonly offset?: number;
};

/** 📊️ A whole chart as taxonomy §79 defines it — the unit the scene renderer and TikZ emitter consume. */
export type VizChartSpecification = {
  readonly width: number;
  readonly height: number;
  readonly margin?: { readonly top: number; readonly right: number; readonly bottom: number; readonly left: number };
  readonly theme?: { readonly name?: string; readonly appearance?: "light" | "dark"; readonly palette?: readonly string[] };
  readonly language?: VizLanguage;
  readonly tables?: readonly VizTable[];
  readonly scales?: readonly VizScaleSpec[];
  readonly coordinate?: { readonly kind: VizCoordinateKind; readonly options?: VizCoordinateOptions };
  readonly layers: readonly VizLayerSpec[];
  readonly guides?: readonly VizGuideSpec[];
  readonly title?: VizLocalizedText;
};
//#endregion 🔖️Specification

//#region 🔖️Catalogue
/** 📚️ One catalogue entry as `🖼️assets/🔣️viz-catalog.json` stores it. */
export type VizCatalogEntry = {
  readonly id: string;
  readonly slug: string;
  readonly title: VizLocalizedText;
  readonly kind: "mark" | "chart" | "layout" | "axis" | "scale";
  readonly namespace: string;
  readonly family: string;
  readonly options?: Readonly<Record<string, VizOptionValue>>;
  readonly data?: string;
  readonly covers?: readonly string[];
};

/** 🧪️ One numeric record of the probe protocol, shared by the LaTeX and the TypeScript subject. */
export type VizProbeRecord = { readonly case: string; readonly scenario: string; readonly key: string; readonly values: readonly (number | string)[] };

/** 🧪️ A probe stream grouped by key — the projection a differential comparison reads. */
export type VizProbeProjection = Readonly<Record<string, readonly (number | string)[]>>;
//#endregion 🔖️Catalogue
