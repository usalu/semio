/** 🧬️ Authored transformation columns adjudicated by native TeX and independent D3. */
import { join, resolve } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";
import { defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { compileVizProbe, type ProbeRecord } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
import vectors from "./🔣️.json";

type Check = { readonly module: string; readonly name: string; readonly subject: () => unknown; readonly oracle: () => unknown; readonly tolerance: number };

function source(): string {
  const table = (name: string, rows: readonly Record<string, unknown>[]) => {
    const columns = Object.keys(rows[0]!);
    return "\\SemioVizTable{" + name + "}{" + columns.join(",") + "}\n" + rows.map(row => "\\SemioVizRow{" + name + "}{" + columns.map(column => "{" + String(row[column]) + "}").join(",") + "}").join("\n");
  };
  const commands = [...vectors.cases, ...vectors.extras].map(entry => {
    const id = "id" in entry ? entry.id : entry.kind;
    const options = Object.entries(entry.options).map(([key, value]) => key + "={" + (Array.isArray(value) ? value.join(",") : value) + "}").join(",");
    return "\\SemioVizTransform{out-" + id + "}{data}[kind=" + entry.kind + "," + options + "]\n" + entry.columns.map(column => "\\ProbeChartColumn{" + id + "/" + column + "}{out-" + id + "}{" + column + "}").join("\n");
  });
  return String.raw`\documentclass{article}
\usepackage{semio-viz-transform}
\usepackage{semio-viz-probe}
\ExplSyntaxOn
\seq_new:N \l_chart_probe_seq
\tl_new:N \l_chart_probe_cell_tl
\NewDocumentCommand \ProbeChartColumn { m m m } {
  \semio_viz_tr_load:n {#2}
  \seq_clear:N \l_chart_probe_seq
  \seq_map_inline:Nn \l_semio_viz_tr_rows_seq {
    \semio_viz_tr_get:nnN {##1} {#3} \l_chart_probe_cell_tl
    \seq_put_right:NV \l_chart_probe_seq \l_chart_probe_cell_tl
  }
  \semio_viz_probe_values:nx {#1} { \seq_use:Nn \l_chart_probe_seq { , } }
}
\ExplSyntaxOff
\begin{document}
\SemioVizProbeBegin{chart-transform-grammar}{all-transform-kinds}
` + table("data", vectors.rows) + "\n" + table("lookup", vectors.lookup) + "\n" + commands.join("\n") + "\n\\SemioVizProbeEnd\n\\end{document}\n";
}

type RegressionBuilder<T> = ((rows: T[]) => {a:number;b:number;rSquared:number;predict(value:number):number}) & {x(accessor:(row:T)=>number):RegressionBuilder<T>;y(accessor:(row:T)=>number):RegressionBuilder<T>};
const regressionPackage: string = "d3-regression";

async function reference(): Promise<Record<string, readonly number[]>> {
  const [array, shape, regression] = await Promise.all([import("d3-array"), import("d3-shape"), import(regressionPackage) as Promise<{regressionLinear<T>():RegressionBuilder<T>}>]);
  const rows = vectors.rows;
  const values = rows.map(row => row.value);
  const groups = [...array.group(rows, row => row.group).values()];
  const fit = regression.regressionLinear<typeof rows[number]>().x((row: typeof rows[number])=>row.x).y((row: typeof rows[number])=>row.y)(rows);
  const result: Record<string, readonly number[]> = {
    "filter/value": array.filter(rows, row => row.value > 2).map(row => row.value),
    "sort/value": array.sort(rows, (a, b) => array.descending(a.value, b.value)).map(row => row.value),
    "group/value": groups.flatMap(group => group.map(row => row.value)),
    "group/group": groups.flatMap((group, index) => group.map(() => index)),
    "fold/value": array.cross(rows, ["a", "b"], (row, key) => row[key as "a" | "b"]),
    "join/value": array.cross(rows, vectors.lookup).filter(([row, other]) => row.id === other.id).map(([row]) => row.value),
    "join/other-score": array.cross(rows, vectors.lookup).filter(([row, other]) => row.id === other.id).map(([, other]) => other.score),
    "window/value": values.map((_, index) => array.mean(values.slice(Math.max(0, index - 1), index + 1))!),
    "normalize/value": values.map(value => value / array.sum(values)),
    "cumulative/value": Array.from(array.cumsum(values)),
    "quantile/p": [0, .25, .5, .75, 1],
    "quantile/value": [0, .25, .5, .75, 1].map(p => array.quantile(values, p)!),
    "kde/x": [0, 2, 4, 6, 8],
    "kde/density": [0, 2, 4, 6, 8].map(x => array.mean(values, value => Math.exp(-.5 * (x - value) ** 2) / Math.sqrt(2 * Math.PI))!),
    "regression/x": rows.map(row => row.x),
    "regression/y": rows.map(row => fit.predict(row.x)),
    "regression/slope": rows.map(() => fit.a),
    "regression/intercept": rows.map(() => fit.b),
    "regression/r2": rows.map(() => fit.rSquared),
    "grouped-window/value": values,
    "grouped-window/smooth": rows.map(row=>{const group=groups.find(bucket=>bucket.includes(row))!,i=group.indexOf(row);return array.sum(group.slice(i,i+2),entry=>entry.value);}),
    "grouped-normalize/value": values,
    "grouped-normalize/fraction": rows.map(row=>{const group=groups.find(bucket=>bucket.includes(row))!,extent=array.extent(group,entry=>entry.value);return extent[0]===extent[1]?0:(row.value-extent[0]!)/(extent[1]!-extent[0]!);}),
    "stable-sort/value": array.sort(rows,(a,b)=>array.ascending(a.group,b.group)||array.ascending(a.value,b.value)).map(row=>row.value),
    "grouped-median/median": groups.map(group=>array.median(group,row=>row.value)!),
    "kde-uniform/density": [0,2,4,6,8].map(x=>array.mean(values,value=>Math.abs(x-value)<=1?0.5:0)!),
  };
  for (const kind of ["aggregate", "rollup", "summary"]) result[kind + "/value"] = [...array.rollup(rows, group => array.sum(group, row => row.value), row => row.group).values()];
  for (const key of ["a", "b", "c"]) result["pivot/" + key] = groups.map(group => array.sum(group.filter(row => row.id === key), row => row.value));
  const bins = array.bin<number, number>().domain([0, 8]).thresholds([3, 5])(values);
  result["bin/x0"] = bins.map(bin => bin.x0!); result["bin/x1"] = bins.map(bin => bin.x1!); result["bin/count"] = bins.map(bin => bin.length);
  const stack = shape.stack<typeof rows[number]>().keys(["a", "b"]).offset(shape.stackOffsetExpand)(rows);
  result["stack/y0"] = stack.flatMap(series => series.map(point => point[0])); result["stack/y1"] = stack.flatMap(series => series.map(point => point[1]));
  return result;
}

/** ⚖️ Compiles every authored transform once and returns checks for each inferred numeric column. */
export async function nativeTransformGrammarChecks(workDir: string): Promise<readonly Check[]> {
  workDir = resolve(workDir);
  mkdirSync(workDir, { recursive: true });
  const input = join(workDir, "chart-transform-grammar.tex");
  writeFileSync(input, source());
  const [records, oracle] = await Promise.all([compileVizProbe(input, { workDir: join(workDir, "compiled"), keepWorkDir: true, caseName: "chart-transform-grammar", scenario: "all-transform-kinds" }), reference()]);
  return Object.entries(oracle).map(([key, values]) => ({ module: "native-transform", name: key, subject: () => records.filter((record: ProbeRecord) => record.key === key).flatMap(record => record.values.map(Number)), oracle: () => values, tolerance: 2e-6 }));
}

export default defineTestAdapter({ implementation: "typescript", scenarios: { "all-transform-kinds": { subject: async context => ({ projection: Object.fromEntries((await nativeTransformGrammarChecks(context.workDir)).map(check => [check.name, check.subject() as number[]])) }), oracle: async () => ({ projection: await reference() }) } } });
