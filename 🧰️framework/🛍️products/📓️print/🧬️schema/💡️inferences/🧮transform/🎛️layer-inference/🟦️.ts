/** 🎛️ Layer inference: the orchestrator that turns a layer's declared transform and layout into its inferred table, over the statistics of `../🟦️.ts` and every layout family.
 * @see ../🟦️.ts — the data transforms it composes
 */
import { VIZ_LAYOUT_ALGORITHMS, VIZ_STACK_OFFSETS, VIZ_STACK_ORDERS, VIZ_PROJECTIONS, VIZ_TREEMAP_TILINGS, type VizChartSpecification, type VizLayerSpec, type VizOptionValue, type VizPoint, type VizRow, type VizTable } from "@semio-tech/print/schema/snapshot/chart";
import * as hierarchy from "../../🌳hierarchy/🟦️.ts";
import { vizCluster, vizTree } from "../../🌳hierarchy/🌲️tidy-tree-and-cluster/🟦️.ts";
import { vizPartition, vizTreemap } from "../../🌳hierarchy/🧱️treemap-tilings-and-partition/🟦️.ts";
import { vizPack } from "../../🌳hierarchy/🫧️circle-packing/🟦️.ts";
import * as network from "../../🕸️network/🟦️.ts";
import * as flow from "../../🌊flow/🟦️.ts";
import * as spatial from "../../📍spatial/🟦️.ts";
import { vizPie } from "../../🥧shape/🟦️.ts";
import { vizGeoProjection } from "../../🌍geo/🟦️.ts";
import { VIZ_KERNELS, type VizThresholdRule, bin, cumulativeSum, deviation, extent, fold, group, kernelDensity1d, linearRegression, mean, median, normalize, pivot, polynomialRegression, quantile, rollup, silvermanBandwidth, stack, sum, variance, window } from "../🟦️.ts";

//#region 🔖️LayerInference
type LayerOptions = Readonly<Record<string, VizOptionValue | null | readonly (VizOptionValue | null)[]>>;
/** ⏳️ Nonpersisted progress and cancellation controls for expensive inferred layouts. */
export type VizLayerInferenceControl = { readonly signal?: AbortSignal; readonly onProgress?: (completed: number, total: number) => void };

const INFERRED_TRANSFORM_OPTIONS: Readonly<Record<string, string>> = { filter: "column operator value", sort: "column columns descending order", group: "column columns as", aggregate: "column groupby group operation as", rollup: "column groupby group operation as", summary: "column groupby group operation as", fold: "columns key value", pivot: "index key value operation", join: "table left right prefix", window: "column as groupby size operation centred", normalize: "column as groupby mode", cumulative: "column as groupby", bin: "column thresholds min max", stack: "keys order offset", quantile: "column probabilities", kde: "column min max samples bandwidth kernel", regression: "x y method order" };
const INFERRED_LAYOUT_OPTIONS: Readonly<Record<string, string>> = { pie: "value sort startAngle endAngle padAngle innerRadius outerRadius cx cy", tree: "id parent value sort leaves separation cousinSeparation", cluster: "id parent value sort leaves separation cousinSeparation", treemap: "id parent value sort leaves tile round paddingInner paddingOuter paddingTop paddingRight paddingBottom paddingLeft", partition: "id parent value sort leaves padding round", pack: "id parent value sort leaves padding", bundling: "id parent value sort leaves separation cousinSeparation", arc: "id source target origin vertical output", dag: "id source target layerGap nodeGap sweeps output", force: "id source target seed alpha velocityDecay charge distance radius iterations output", sankey: "source target value nodeWidth nodePadding iterations align output", alluvial: "stages value nodeWidth nodePadding iterations align output", chord: "source target value padAngle transpose output radius", projection: "x y projection scale translateX translateY rotateLongitude rotateLatitude rotateGamma centerLongitude centerLatitude reflectX reflectY", hexbin: "x y radius", jitter: "x y seed amount", beeswarm: "x y radius axis", voronoi: "x y x0 y0 x1 y1", delaunay: "x y", hull: "x y", contour: "value columns rows thresholds", density: "x y x0 y0 x1 y1 cellSize bandwidth kernel thresholds" };

function validateOptions(options: LayerOptions, keys: string | undefined, name: string): void {
  if (keys === undefined) throw new Error(`Unknown computation ${name}`);
  const allowed = new Set(keys.split(" "));
  for (const key of Object.keys(options)) if (!allowed.has(key)) throw new Error(`Unknown ${name} option ${key}`);
}

function inferredTable(name: string, rows: readonly VizRow[], columns: readonly string[] = []): VizTable {
  return { name, columns: [...new Set([...columns, ...rows.flatMap(Object.keys)])], rows };
}

function optionNumber(options: LayerOptions, key: string, fallback: number): number {
  const value = options[key] ?? fallback;
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`Invalid numeric option ${key}`);
  return value;
}

function optionList(value: LayerOptions[string] | undefined): string[] {
  return value === undefined ? [] : Array.isArray(value) ? value.map(String) : String(value).split(/[,;]/).map((part) => part.trim()).filter(Boolean);
}

function optionEnum<T extends string>(options: LayerOptions, key: string, values: readonly T[], fallback: T): T {
  const value = String(options[key] ?? fallback);
  if (!values.includes(value as T)) throw new Error(`Invalid ${key}: ${value}`);
  return value as T;
}

function columnName(table: VizTable, options: LayerOptions, key = "column", fallback = "value"): string {
  const column = String(options[key] ?? fallback);
  if (!table.columns.includes(column)) throw new Error(`Unknown column ${column} in table ${table.name}`);
  return column;
}

function compareValues(a: VizOptionValue | null | undefined, b: VizOptionValue | null | undefined): number {
  if (a === b) return 0;
  if (a === null || a === undefined) return -1;
  if (b === null || b === undefined) return 1;
  if (typeof a === "number" && typeof b === "number") return a - b;
  return String(a) < String(b) ? -1 : 1;
}

function numericValue(row: VizRow, column: string): number {
  const value = row[column];
  if (value === null || value === undefined || value === "" || !Number.isFinite(Number(value))) throw new Error(`Non-finite value in column ${column}`);
  return Number(value);
}

function aggregateValues(rows: readonly VizRow[], column: string, operation: string): number | null {
  if (operation === "count") return rows.length;
  const values = rows.filter((row) => row[column] !== null).map((row) => numericValue(row, column));
  switch (operation) {
    case "sum": return sum(values);
    case "mean": return mean(values) ?? null;
    case "min": return extent(values)[0] ?? null;
    case "max": return extent(values)[1] ?? null;
    case "median": return values.length ? median(values) : null;
    case "variance": return variance(values) ?? null;
    case "deviation": return deviation(values) ?? null;
    default: throw new Error(`Unknown aggregate ${operation}`);
  }
}

function inferHierarchyPath<T>(source: hierarchy.VizHierarchyNode<T>, target: hierarchy.VizHierarchyNode<T>): hierarchy.VizHierarchyNode<T>[] {
  const a = source.ancestors(), b = target.ancestors(), targetAncestors = new Set(b);
  const common = a.find((node) => targetAncestors.has(node));
  if (!common) throw new Error("Cannot bundle nodes of different hierarchies");
  return [...a.slice(0, a.indexOf(common) + 1), ...b.slice(0, b.indexOf(common)).reverse()];
}

function groupRows(table: VizTable, columns: readonly string[]): VizRow[][] {
  for (const column of columns) columnName(table, { column });
  if (!columns.length) return [[...table.rows]];
  type GroupNode = { children: Map<VizRow[string] | undefined, GroupNode>; rows?: VizRow[] };
  const root: GroupNode = { children: new Map() }, groups: VizRow[][] = [];
  for (const row of table.rows) {
    let node = root;
    for (const column of columns) {
      const key = row[column];
      let child = node.children.get(key);
      if (child === undefined) { child = { children: new Map() }; node.children.set(key, child); }
      node = child;
    }
    if (node.rows === undefined) groups.push(node.rows = []);
    node.rows.push(row);
  }
  return groups;
}

function inferTransform(spec: VizChartSpecification, table: VizTable, kind: string, options: LayerOptions): VizTable {
  if (INFERRED_TRANSFORM_OPTIONS[kind] === undefined) throw new Error(`Unknown transform ${kind}`);
  validateOptions(options, INFERRED_TRANSFORM_OPTIONS[kind], kind);
  const result = (rows: readonly VizRow[], columns: readonly string[] = table.columns) => inferredTable(table.name, rows, columns);
  const output = String(options.as ?? options.column ?? "value");
  if (kind === "filter") {
    const column = columnName(table, options);
    const operator = optionEnum(options, "operator", ["eq", "ne", "lt", "lte", "gt", "gte", "in", "valid"] as const, "eq");
    const value = options.value;
    return result(table.rows.filter((row) => {
      const current = row[column];
      switch (operator) {
        case "eq": return current === value;
        case "ne": return current !== value;
        case "in": return Array.isArray(value) && value.includes(current as VizOptionValue);
        case "valid": return current !== null && current !== undefined && !(typeof current === "number" && !Number.isFinite(current));
        case "lt": return current !== null && current !== undefined && compareValues(current, value as VizOptionValue) < 0;
        case "lte": return current !== null && current !== undefined && compareValues(current, value as VizOptionValue) <= 0;
        case "gt": return current !== null && current !== undefined && compareValues(current, value as VizOptionValue) > 0;
        case "gte": return current !== null && current !== undefined && compareValues(current, value as VizOptionValue) >= 0;
      }
    }));
  }
  if (kind === "sort") {
    const columns = optionList(options.columns ?? options.column);
    if (!columns.length) throw new Error("Sort requires columns");
    for (const column of columns) columnName(table, { column });
    const descending = options.descending === true || optionEnum(options, "order", ["ascending", "descending"] as const, "ascending") === "descending";
    return result([...table.rows].sort((a, b) => {
      for (const column of columns) {
        const av = a[column], bv = b[column];
        const delta = compareValues(av, bv);
        if (delta) return descending ? -delta : delta;
      }
      return 0;
    }));
  }
  if (kind === "group") {
    const columns = optionList(options.columns ?? options.column);
    if (!columns.length) throw new Error("Group requires columns");
    return result(groupRows(table, columns).flatMap((rows, index) => rows.map((row) => ({ ...row, [String(options.as ?? "group")]: index }))));
  }
  if (kind === "aggregate" || kind === "rollup" || kind === "summary") {
    const columns = optionList(options.groupby ?? options.group);
    const column = columnName(table, options);
    const operation = String(options.operation ?? "sum");
    return result(groupRows(table, columns).map((rows) => ({ ...Object.fromEntries(columns.map((key) => [key, rows[0]?.[key] ?? null])), [output]: aggregateValues(rows, column, operation) })), [...columns, output]);
  }
  if (kind === "fold") {
    const columns = optionList(options.columns);
    for (const column of columns) columnName(table, { column });
    if (!columns.length) throw new Error("Fold requires columns");
    const folded = fold(table, columns, String(options.key ?? "key"), String(options.value ?? "value"));
    return { ...folded, name: table.name };
  }
  if (kind === "pivot") {
    const index = columnName(table, options, "index", "id"), key = columnName(table, options, "key", "key"), value = columnName(table, options, "value");
    const keys = [...new Set(table.rows.map((row) => String(row[key])))];
    const rows = groupRows(table, [index]).map((bucket) => ({ [index]: bucket[0]![index]!, ...Object.fromEntries(keys.map((k) => [k, aggregateValues(bucket.filter((row) => String(row[key]) === k), value, String(options.operation ?? "sum"))])) }));
    return result(rows, [index, ...keys]);
  }
  if (kind === "join") {
    const right = spec.tables?.find((entry) => entry.name === options.table);
    if (!right) throw new Error(`Unknown join table ${options.table}`);
    const leftKey = columnName(table, options, "left", "id"), rightKey = columnName(right, options, "right", leftKey);
    const prefix = String(options.prefix ?? "");
    const rows = table.rows.flatMap((row) => right.rows.filter((other) => other[rightKey] === row[leftKey]).map((other) => ({ ...row, ...Object.fromEntries(Object.entries(other).filter(([key]) => key !== rightKey).map(([key, value]) => [prefix + key, value])) })));
    return result(rows);
  }
  if (kind === "window" || kind === "normalize" || kind === "cumulative") {
    const column = columnName(table, options), rows: VizRow[] = [];
    for (const bucket of groupRows(table, optionList(options.groupby))) {
      const values = bucket.map((row) => numericValue(row, column));
      let computed: readonly (number | null)[];
      if (kind === "cumulative") computed = cumulativeSum(values);
      else if (kind === "normalize") {
        const mode = optionEnum(options, "mode", ["sum", "extent"] as const, "sum");
        const total = sum(values);
        computed = mode === "extent" ? normalize(values) : values.map((value) => total ? value / total : 0);
      } else {
        const size = optionNumber(options, "size", 3);
        if (!Number.isInteger(size) || size < 1) throw new Error("Window size must be a positive integer");
        computed = window(values, size, (frame) => aggregateValues(frame.map((value) => ({ value })), "value", String(options.operation ?? "mean")) ?? 0, options.centred === true);
      }
      bucket.forEach((row, index) => rows.push({ ...row, [output]: computed[index]! }));
    }
    const positions = new Map(table.rows.map((row, index) => [row, index]));
    const ordered = groupRows(table, optionList(options.groupby)).flat();
    return result(rows.map((row, index) => ({ row, index: positions.get(ordered[index]!)! })).sort((a, b) => a.index - b.index).map(({ row }) => row));
  }
  if (kind === "bin") {
    const column = columnName(table, options);
    const thresholds = Array.isArray(options.thresholds) ? options.thresholds.map(Number) : options.thresholds as number | VizThresholdRule | undefined;
    const domain = options.min !== undefined || options.max !== undefined ? [optionNumber(options, "min", 0), optionNumber(options, "max", 1)] as const : undefined;
    if (domain && domain[1] < domain[0]) throw new Error("Bin domain is reversed");
    return result(bin(table.rows, { value: (row) => numericValue(row, column), domain, thresholds }).map((bucket) => ({ x: bucket.x0, x2: bucket.x1, y: bucket.values.length, x0: bucket.x0, x1: bucket.x1, count: bucket.values.length })), ["x", "x2", "y", "x0", "x1", "count"]);
  }
  if (kind === "stack") {
    const keys = optionList(options.keys);
    if (!keys.length) throw new Error("Stack requires keys");
    for (const column of keys) columnName(table, { column });
    return result(stack(table.rows, keys, { value: (row, key) => numericValue(row, key), order: optionEnum(options, "order", VIZ_STACK_ORDERS, "none"), offset: optionEnum(options, "offset", VIZ_STACK_OFFSETS, "none") }).flatMap((series) => series.points.map((point) => ({ ...table.rows[point.index], key: series.key, index: series.index, y: point[1], y2: point[0], y0: point[0], y1: point[1], value: numericValue(table.rows[point.index]!, series.key) }))));
  }
  if (kind === "quantile") {
    const column = columnName(table, options), values = table.rows.map((row) => numericValue(row, column));
    const probabilities = options.probabilities === undefined ? [0, 0.25, 0.5, 0.75, 1] : optionList(options.probabilities).map(Number);
    if (probabilities.some((p) => !Number.isFinite(p) || p < 0 || p > 1)) throw new Error("Invalid quantile probabilities");
    return result(probabilities.map((p) => ({ p, value: values.length ? quantile(values, p) : null })), ["p", "value"]);
  }
  if (kind === "kde") {
    const column = columnName(table, options), values = table.rows.map((row) => numericValue(row, column));
    if (!values.length) return result([], ["x", "y", "density"]);
    const [lo, hi] = extent(values), min = optionNumber(options, "min", lo!), max = optionNumber(options, "max", hi!);
    const count = optionNumber(options, "samples", 100), bandwidth = optionNumber(options, "bandwidth", silvermanBandwidth(values));
    if (!Number.isInteger(count) || count < 2 || bandwidth <= 0 || max < min) throw new Error("Invalid density grid or bandwidth");
    const grid = Array.from({ length: count }, (_, i) => min + (max - min) * i / (count - 1));
    const densities = kernelDensity1d(values, grid, { bandwidth, kernel: optionEnum(options, "kernel", VIZ_KERNELS, "gaussian") });
    return result(grid.map((x, i) => ({ x, y: densities[i]!, density: densities[i]! })), ["x", "y", "density"]);
  }
  if (kind === "regression") {
    const x = columnName(table, options, "x", "x"), y = columnName(table, options, "y", "y");
    const points = table.rows.map((row) => [numericValue(row, x), numericValue(row, y)] as const);
    if (!points.length) return result([], ["x", "y"]);
    const method = optionEnum(options, "method", ["linear", "poly", "exp", "log", "pow"] as const, "linear");
    if (points.some(([xv, yv]) => ((method === "log" || method === "pow") && xv <= 0) || ((method === "exp" || method === "pow") && yv <= 0))) throw new Error(`Invalid ${method} regression domain`);
    const order = optionNumber(options, "order", 2);
    if (method === "poly" && (!Number.isInteger(order) || order < 0 || order >= points.length)) throw new Error("Invalid regression order");
    const transformed = points.map(([xv, yv]) => [method === "log" || method === "pow" ? Math.log(xv) : xv, method === "exp" || method === "pow" ? Math.log(yv) : yv] as const);
    const fit = linearRegression(transformed), coefficients = method === "poly" ? polynomialRegression(points, order) : [];
    return result(points.map(([xv]) => {
      const prediction = method === "poly" ? coefficients.reduce((total, c, i) => total + c * xv ** i, 0) : fit.predict(method === "log" || method === "pow" ? Math.log(xv) : xv);
      return { x: xv, y: method === "exp" || method === "pow" ? Math.exp(prediction) : prediction, slope: fit.slope, intercept: fit.intercept, r2: fit.r2 };
    }), ["x", "y", "slope", "intercept", "r2"]);
  }
  throw new Error(`Unknown transform ${kind}`);
}

/** 🧮️ Resolves authored transforms and layout geometry as the chart's pure layer inference. */
export function inferVizLayerTable(spec: VizChartSpecification, layer: VizLayerSpec, control: VizLayerInferenceControl = {}): VizTable {
  control.signal?.throwIfAborted();
  const source = layer.data === undefined ? spec.tables?.[0] : spec.tables?.find((table) => table.name === layer.data);
  if (!source) {
    if (layer.data !== undefined || layer.transform?.length || layer.layout) throw new Error(`Unknown layer table ${layer.data ?? "<first>"}`);
    return { name: "", columns: [], rows: [] };
  }
  const forceUnits = layer.layout?.algorithm === "force" ? optionNumber(layer.layout.options ?? {}, "iterations", 300) : 0;
  const total = source.rows.length + (layer.transform?.length ?? 0) + 1 + forceUnits;
  let completed = 0;
  const checkpoint = () => { control.signal?.throwIfAborted(); control.onProgress?.(completed, total); control.signal?.throwIfAborted(); };
  let table: VizTable = { ...source, columns: [...source.columns], rows: source.rows.map((row, index) => { completed += 1; if (index % 256 === 0) checkpoint(); return { ...row }; }) };
  for (const transform of layer.transform ?? []) { checkpoint(); table = inferTransform(spec, table, transform.kind, transform.options ?? {}); completed += 1; checkpoint(); }
  checkpoint();
  if (!layer.layout) { completed = total; checkpoint(); return table; }
  const { algorithm } = layer.layout, options: LayerOptions = layer.layout.options ?? {};
  if (!VIZ_LAYOUT_ALGORITHMS.includes(algorithm)) throw new Error(`Unknown layout ${algorithm}`);
  if (algorithm === "bin" || algorithm === "stack") { const derived = inferTransform(spec, table, algorithm, options); completed = total; checkpoint(); return derived; }
  validateOptions(options, "width height " + INFERRED_LAYOUT_OPTIONS[algorithm], algorithm);
  const margin = spec.margin ?? { top: 8, right: 8, bottom: 14, left: 16 };
  const width = optionNumber(options, "width", spec.width - margin.left - margin.right), height = optionNumber(options, "height", spec.height - margin.top - margin.bottom);
  if (width <= 0 || height <= 0) throw new Error("Layout extent must be positive");
  const result = (rows: readonly VizRow[], columns: readonly string[] = []) => { completed = total; checkpoint(); return inferredTable(table.name, rows, columns); };
  if (algorithm === "pie") {
    const column = columnName(table, options, "value", layer.encodings?.angle?.column ?? "value");
    const sort = optionEnum(options, "sort", ["ascending", "descending", "none"] as const, "descending");
    return result(vizPie(table.rows, { value: (row) => numericValue(row, column), sortValues: sort === "none" ? null : sort === "ascending" ? (a, b) => a - b : (a, b) => b - a, startAngle: optionNumber(options, "startAngle", 0), endAngle: optionNumber(options, "endAngle", 2 * Math.PI), padAngle: optionNumber(options, "padAngle", 0) }).map((slice) => ({ ...slice.data, index: slice.index, value: slice.value, startAngle: slice.startAngle, endAngle: slice.endAngle, padAngle: slice.padAngle, innerRadius: optionNumber(options, "innerRadius", 0), outerRadius: optionNumber(options, "outerRadius", Math.min(width, height) / 2), x: optionNumber(options, "cx", width / 2), y: optionNumber(options, "cy", height / 2) })));
  }
  if (["tree", "cluster", "treemap", "partition", "pack", "bundling"].includes(algorithm)) {
    if (!table.rows.length) return result([]);
    const id = columnName(table, options, "id", "id"), parent = columnName(table, options, "parent", "parent");
    const root = hierarchy.vizStratify(table.rows, { id: (row) => String(row[id]), parentId: (row) => row[parent] === null || row[parent] === "" ? null : String(row[parent]) });
    const value = String(options.value ?? "value");
    if (table.columns.includes(value)) root.sum((row) => numericValue(row, value)); else root.count();
    const sort = optionEnum(options, "sort", ["none", "ascending", "descending"] as const, "none");
    if (sort !== "none") root.sort((a, b) => (sort === "ascending" ? 1 : -1) * ((a.value ?? 0) - (b.value ?? 0)));
    const size = [width, height] as const;
    if (algorithm === "treemap") vizTreemap(root, { size, tile: optionEnum(options, "tile", VIZ_TREEMAP_TILINGS, "squarify"), round: options.round === true, paddingInner: optionNumber(options, "paddingInner", 0), paddingOuter: optionNumber(options, "paddingOuter", 0), paddingTop: optionNumber(options, "paddingTop", optionNumber(options, "paddingOuter", 0)), paddingRight: optionNumber(options, "paddingRight", optionNumber(options, "paddingOuter", 0)), paddingBottom: optionNumber(options, "paddingBottom", optionNumber(options, "paddingOuter", 0)), paddingLeft: optionNumber(options, "paddingLeft", optionNumber(options, "paddingOuter", 0)) });
    else if (algorithm === "partition") vizPartition(root, { size, padding: optionNumber(options, "padding", 0), round: options.round === true });
    else if (algorithm === "pack") vizPack(root, { size, padding: optionNumber(options, "padding", 0) });
    else (algorithm === "tree" ? vizTree : vizCluster)(root, { size, separation: (a, b) => (a.parent === b.parent ? optionNumber(options, "separation", 1) : optionNumber(options, "cousinSeparation", 2)) });
    const nodes = options.leaves === true ? root.leaves() : root.descendants();
    if (algorithm === "bundling") return result(root.links().flatMap(({ source, target }, detail) => inferHierarchyPath(target, source).map((node, order) => ({ detail, order, x: node.x, y: node.y }))));
    return result(nodes.map((node) => ({ ...node.data, value: node.value ?? 0, depth: node.depth, x: algorithm === "treemap" || algorithm === "partition" ? node.x0 : node.x, y: algorithm === "treemap" || algorithm === "partition" ? node.y0 : node.y, x2: node.x1, y2: node.y1, x0: node.x0, y0: node.y0, x1: node.x1, y1: node.y1, radius: node.r, size: Math.PI * node.r ** 2, parentX: node.parent?.x ?? node.x, parentY: node.parent?.y ?? node.y })));
  }
  if (["arc", "dag", "force", "sankey", "alluvial", "chord"].includes(algorithm)) {
    const sourceColumn = String(options.source ?? "source"), targetColumn = String(options.target ?? "target"), valueColumn = String(options.value ?? "value");
    const hasLinks = table.columns.includes(sourceColumn) && table.columns.includes(targetColumn);
    const names = hasLinks ? [...new Set(table.rows.flatMap((row) => [String(row[sourceColumn]), String(row[targetColumn])]))] : table.rows.map((row, index) => String(row[String(options.id ?? "id")] ?? index));
    const edges = hasLinks ? table.rows.map((row) => [String(row[sourceColumn]), String(row[targetColumn])] as const) : [];
    if (["sankey", "alluvial", "chord"].includes(algorithm) && !hasLinks && algorithm !== "alluvial") throw new Error(`${algorithm} requires source and target columns`);
    const sankeyOptions = { extent: [[0, 0], [width, height]] as const, nodeWidth: optionNumber(options, "nodeWidth", 4), nodePadding: optionNumber(options, "nodePadding", 2), iterations: optionNumber(options, "iterations", 6), align: optionEnum(options, "align", flow.VIZ_SANKEY_ALIGNMENTS, "justify") };
    if (algorithm === "sankey" || algorithm === "alluvial") {
      const stages = optionList(options.stages);
      if (algorithm === "alluvial" && stages.length < 2) throw new Error("Alluvial requires at least two stages");
      for (const column of stages) columnName(table, { column });
      const layout = algorithm === "sankey" ? flow.vizSankey({ nodes: names.map((name) => ({ name })), links: table.rows.map((row) => ({ source: String(row[sourceColumn]), target: String(row[targetColumn]), value: numericValue(row, valueColumn) })) }, sankeyOptions) : flow.vizAlluvial(table.rows.map((row) => ({ stages: stages.map((key) => String(row[key])), value: numericValue(row, valueColumn) })), { ...sankeyOptions, stageNames: stages });
      return options.output === "nodes" ? result(layout.nodes.map((node) => ({ id: node.name, value: node.value, x: node.x0, y: node.y0, x2: node.x1, y2: node.y1, depth: node.depth }))) : result(layout.links.map((link) => ({ source: link.source.name, target: link.target.name, value: link.value, x: link.source.x1, y: link.y0, x2: link.target.x0, y2: link.y1, width: link.width })));
    }
    if (algorithm === "chord") {
      const matrix = names.map(() => names.map(() => 0));
      for (const row of table.rows) matrix[names.indexOf(String(row[sourceColumn]))]![names.indexOf(String(row[targetColumn]))]! += numericValue(row, valueColumn);
      const layout = network.vizChord(matrix, { padAngle: optionNumber(options, "padAngle", 0), transpose: options.transpose === true });
      return options.output === "groups" ? result(layout.groups.map((span) => ({ id: names[span.index]!, value: span.value, startAngle: span.startAngle, endAngle: span.endAngle }))) : result(layout.chords.map((chord) => ({ source: names[chord.source.index]!, target: names[chord.target.index]!, value: chord.source.value, startAngle: chord.source.startAngle, endAngle: chord.source.endAngle, targetStartAngle: chord.target.startAngle, targetEndAngle: chord.target.endAngle, radius: optionNumber(options, "radius", Math.min(width, height) / 2) })));
    }
    let nodes: readonly { id: string; x: number; y: number }[];
    if (algorithm === "arc") nodes = network.vizArcLayout({ nodes: names, edges }, { length: width, origin: optionNumber(options, "origin", 0), vertical: options.vertical === true });
    else if (algorithm === "dag") nodes = network.vizLayeredLayout({ nodes: names, edges }, { layerGap: optionNumber(options, "layerGap", 20), nodeGap: optionNumber(options, "nodeGap", 12), sweeps: optionNumber(options, "sweeps", 4) }).nodes;
    else {
      const placed = names.map((id, index) => ({ id, x: hasLinks ? NaN : Number(table.rows[index]?.x ?? NaN), y: hasLinks ? NaN : Number(table.rows[index]?.y ?? NaN), vx: 0, vy: 0 }));
      const links = edges.map(([a, b]) => ({ source: names.indexOf(a), target: names.indexOf(b) }));
      const simulation = new network.VizForceSimulation(placed, { seed: optionNumber(options, "seed", 1), alpha: optionNumber(options, "alpha", 1), velocityDecay: optionNumber(options, "velocityDecay", 0.4) });
      simulation.force("charge", network.forceVizManyBody({ strength: optionNumber(options, "charge", -30) })).force("center", network.forceVizCenter(width / 2, height / 2));
      if (links.length) simulation.force("link", network.forceVizLink(links, { distance: optionNumber(options, "distance", 30) }));
      if (options.radius !== undefined) simulation.force("collide", network.forceVizCollide({ radius: optionNumber(options, "radius", 1) }));
      const iterations = optionNumber(options, "iterations", 300);
      if (!Number.isInteger(iterations) || iterations < 0) throw new Error("Force iterations must be nonnegative integer");
      for (let iteration = 0; iteration < iterations; iteration += 1) { control.signal?.throwIfAborted(); simulation.tick(); completed += 1; checkpoint(); }
      nodes = placed;
    }
    if (options.output === "links") return result(edges.map(([source, target]) => { const a = nodes.find((node) => node.id === source)!, b = nodes.find((node) => node.id === target)!; return { source, target, x: a.x, y: a.y, x2: b.x, y2: b.y }; }));
    return result(nodes.map((node, index) => ({ ...(hasLinks ? {} : table.rows[index]), ...node })));
  }
  const xColumn = algorithm === "contour" ? "x" : columnName(table, options, "x", layer.encodings?.x?.column ?? "x"), yColumn = algorithm === "contour" ? "y" : columnName(table, options, "y", layer.encodings?.y?.column ?? "y");
  const points: VizPoint[] = algorithm === "contour" ? [] : table.rows.map((row) => [numericValue(row, xColumn), numericValue(row, yColumn)]);
  const bounds = [optionNumber(options, "x0", 0), optionNumber(options, "y0", 0), optionNumber(options, "x1", width), optionNumber(options, "y1", height)] as const;
  if (algorithm === "projection") {
    const projection = vizGeoProjection(optionEnum(options, "projection", VIZ_PROJECTIONS, "equirectangular"), { scale: optionNumber(options, "scale", width / (2 * Math.PI)), translate: [optionNumber(options, "translateX", width / 2), optionNumber(options, "translateY", height / 2)], rotate: [optionNumber(options, "rotateLongitude", 0), optionNumber(options, "rotateLatitude", 0), optionNumber(options, "rotateGamma", 0)], center: [optionNumber(options, "centerLongitude", 0), optionNumber(options, "centerLatitude", 0)], reflectX: options.reflectX === true, reflectY: options.reflectY === true });
    return result(table.rows.map((row, i) => { const [x, y] = projection(points[i]!); return { ...row, x, y }; }));
  }
  if (algorithm === "hexbin") return result(spatial.vizHexbin(table.rows, { radius: optionNumber(options, "radius", 1), x: (row) => numericValue(row, xColumn), y: (row) => numericValue(row, yColumn) }).map((bucket) => ({ x: bucket.x, y: bucket.y, count: bucket.values.length, value: bucket.values.length, radius: optionNumber(options, "radius", 1) })));
  if (algorithm === "jitter") {
    const random = hierarchy.vizLcg(optionNumber(options, "seed", 1)), amount = optionNumber(options, "amount", 1);
    return result(table.rows.map((row, i) => ({ ...row, x: points[i]![0] + (random() - 0.5) * amount, y: points[i]![1] + (random() - 0.5) * amount })));
  }
  if (algorithm === "beeswarm") {
    const radius = optionNumber(options, "radius", 1), placed: VizPoint[] = [], axis = optionEnum(options, "axis", ["x", "y"] as const, "x");
    if (radius <= 0) throw new Error("Beeswarm radius must be positive");
    const rows = table.rows.map((row, i) => {
      const point = points[i]!, value = axis === "x" ? point[0] : point[1], baseline = axis === "x" ? point[1] : point[0];
      const candidates = [baseline];
      for (const prior of placed) {
        const delta = Math.abs(value - prior[0]);
        if (delta < 2 * radius) { const offset = Math.sqrt(4 * radius * radius - delta * delta); candidates.push(prior[1] - offset, prior[1] + offset); }
      }
      candidates.sort((a, b) => Math.abs(a - baseline) - Math.abs(b - baseline) || a - b);
      const offset = candidates.find((candidate) => placed.every((prior) => (value - prior[0]) ** 2 + (candidate - prior[1]) ** 2 >= 4 * radius * radius - 1e-9))!;
      placed.push([value, offset]);
      return { ...row, x: axis === "x" ? value : offset, y: axis === "x" ? offset : value };
    });
    return result(rows);
  }
  const polygons = (shapes: readonly (readonly VizPoint[])[], values?: readonly number[]) => result(shapes.flatMap((polygon, detail) => polygon.map(([x, y], order) => ({ x, y, detail, order, value: values?.[detail] ?? detail }))));
  if (algorithm === "voronoi") return polygons(spatial.vizVoronoi(points, bounds).cells);
  if (algorithm === "delaunay") return polygons(spatial.vizDelaunay(points).triangles.map((triangle) => triangle.map((i) => points[i]!)));
  if (algorithm === "hull") return polygons([spatial.vizConvexHull(points).map((i) => points[i]!)]);
  if (algorithm === "contour" || algorithm === "density") {
    const thresholds = optionNumber(options, "thresholds", 10);
    if (!Number.isInteger(thresholds) || thresholds < 1) throw new Error("Contour thresholds must be a positive integer");
    let contours: spatial.VizContour[];
    if (algorithm === "density") {
      const cellSize = optionNumber(options, "cellSize", 1), bandwidth = optionNumber(options, "bandwidth", 10);
      if (cellSize <= 0 || bandwidth <= 0) throw new Error("Density cellSize and bandwidth must be positive");
      if (!points.length) return result([]);
      const grid = spatial.vizDensity2d(points, { extent: bounds, cellSize, bandwidth, kernel: optionEnum(options, "kernel", VIZ_KERNELS, "gaussian") });
      contours = spatial.vizDensityContours(grid, thresholds).map((contour) => ({ ...contour, coordinates: contour.coordinates.map((polygon) => polygon.map((ring) => ring.map(([x, y]) => [grid.x0 + x * grid.cellSize, grid.y0 + y * grid.cellSize] as VizPoint))) }));
    } else {
      const columns = optionNumber(options, "columns", width), rows = optionNumber(options, "rows", height);
      if (!Number.isInteger(columns) || !Number.isInteger(rows) || columns <= 0 || rows <= 0 || table.rows.length !== columns * rows) throw new Error("Contour requires a complete rectangular grid");
      contours = spatial.vizContours(table.rows.map((row) => numericValue(row, String(options.value ?? "value"))), [columns, rows], thresholds);
    }
    return polygons(contours.flatMap((contour) => contour.coordinates.flatMap((polygon) => polygon)), contours.flatMap((contour) => contour.coordinates.flatMap((polygon) => polygon.map(() => contour.value))));
  }
  throw new Error(`Unknown layout ${algorithm}`);
}
//#endregion 🔖️LayerInference
