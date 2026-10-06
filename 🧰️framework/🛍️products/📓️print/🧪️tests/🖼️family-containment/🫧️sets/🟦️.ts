/** 🫧️ Checks native proportional circles against independent D3, polygon clipping and actual PDF paths. */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { getDocument, OPS, Util } from "pdfjs-dist/legacy/build/pdf.mjs";
import { scaleLinear, scaleSqrt } from "d3-scale";
import { polygonArea } from "d3-polygon";
import polygonClipping from "polygon-clipping";
import { compileVizProbeDocument } from "../../../🔨️modules/🧪️viz-probe/🟦️.ts";
import fixture from "./🔣️.json";

type Point = [number, number];

/** 📐️ Measures the actual four cubic PDF circle through its complete graphics transform. */
function circleBounds(path: number[], matrix: number[]): number[] {
  const points: number[][] = [];
  let start = [path[4]!, path[5]!];
  for (const offset of [6, 13, 20, 27]) {
    const curve = [start, [path[offset + 1]!, path[offset + 2]!], [path[offset + 3]!, path[offset + 4]!], [path[offset + 5]!, path[offset + 6]!]];
    for (let index = 0; index <= 128; index++) {
      const t = index / 128, u = 1 - t, point = [0, 1].map(axis => u ** 3 * curve[0]![axis]! + 3 * u * u * t * curve[1]![axis]! + 3 * u * t * t * curve[2]![axis]! + t ** 3 * curve[3]![axis]!);
      Util.applyTransform(point, matrix); points.push(point);
    }
    start = curve[3]!;
  }
  return [Math.min(...points.map(point => point[0]!)), Math.min(...points.map(point => point[1]!)), Math.max(...points.map(point => point[0]!)), Math.max(...points.map(point => point[1]!))];
}

/** 🧭️ Independently projects the expected circular geometry to an ellipse in final page coordinates. */
function expectedBounds(circle: number[], matrix: number[]): number[] {
  const project = scaleLinear([0, 1], [0, 72 / 25.4]), centre = [project(circle[0]!), project(circle[1]!)], radius = project(circle[2]!);
  Util.applyTransform(centre, matrix);
  const x = radius * Math.hypot(matrix[0]!, matrix[2]!), y = radius * Math.hypot(matrix[1]!, matrix[3]!);
  return [centre[0]! - x, centre[1]! - y, centre[0]! + x, centre[1]! + y];
}

/** 🧮️ Numerically intersects independently sampled circles with the existing third-party polygon engine. */
function overlapArea(radii: number[], distance: number): number {
  const rings = radii.map((radius, circle) => Array.from({ length: fixture.polygonSamples }, (_, index): Point => {
    const angle = 2 * Math.PI * index / fixture.polygonSamples;
    return [radius * Math.cos(angle) + circle * distance, radius * Math.sin(angle)];
  }));
  return polygonClipping.intersection([rings[0]!], [rings[1]!]).reduce((sum, polygon) => sum + polygon.reduce((area, ring, index) => area + (index ? -1 : 1) * Math.abs(polygonArea(ring)), 0), 0);
}

/** ⚖️ Derives independent radii, overlap distance and the declared uniform canvas fit. */
function reference(entry: typeof fixture.cases[number]) {
  const radius = scaleSqrt([0, Math.max(...entry.sizes)], [0, entry.radius]), radii = entry.sizes.map(size => radius(size)), target = Math.PI * radii[0]! ** 2 * entry.overlap / entry.sizes[0]!;
  let low = Math.abs(radii[0]! - radii[1]!), high = radii[0]! + radii[1]!;
  if (entry.overlap === 0) low = high;
  else if (entry.overlap === Math.min(...entry.sizes)) high = low;
  else for (let iteration = 0; iteration < fixture.distanceIterations; iteration++) {
    const middle = (low + high) / 2;
    if (overlapArea(radii, middle) > target) low = middle; else high = middle;
  }
  const distance = (low + high) / 2, left = Math.min(-radii[0]!, distance - radii[1]!), right = Math.max(radii[0]!, distance + radii[1]!), margin = entry.padding + fixture.strokeWidthPt * 25.4 / 72.27 / 2, fit = entry.fit ? Math.min(1, (entry.canvas[0]! - 2 * margin) / (right - left), (entry.canvas[1]! - 2 * margin) / (2 * Math.max(...radii))) : 1;
  const circles = radii.map((value, index) => [entry.canvas[0]! / 2 + (index * distance - (left + right) / 2) * fit, entry.canvas[1]! / 2, value * fit]);
  return { circles, distance: distance * fit, overlap: target * fit * fit, fit };
}

/** 🔬️ Compiles declared boundary, size, canvas and policy controls in both print themes. */
export async function compileNativeSetContainment(workDir: string): Promise<void> {
  const cases = process.env.PRINT_NATIVE_SET_PHASE === "baseline" ? fixture.cases.filter(entry => entry.id === "set-stock") : fixture.cases;
  const expected = new Map(cases.map(entry => [entry.id, reference(entry)])), failures: string[] = [];
  for (const theme of ["light", "dark"]) {
    const body = cases.map(entry => ({ raw: `\\clearpage\\SemioVizProbeBegin{set-containment}{${entry.id}}\\begin{VizFigure}[title={${entry.id}},width=${entry.canvas[0]},height=${entry.canvas[1]}]\\draw[opacity=0.05,line width=0.01mm] (0,0) rectangle (${entry.canvas[0]},${entry.canvas[1]});\\begin{scope}[local bounding box=set-result]\\SetMeasure\\SemioVizChart{area-proportional-venn-diagram}[${entry.options}]\\end{scope}\\ExplSyntaxOn\\pgfextractx\\l_tmpa_dim{\\pgfpointanchor{set-result}{south~west}}\\pgfextracty\\l_tmpb_dim{\\pgfpointanchor{set-result}{south~west}}\\semio_viz_probe_values:nx{set/min}{\\fp_eval:n{\\dim_to_fp:n{\\l_tmpa_dim}/(\\dim_to_fp:n{100mm}/100)},\\fp_eval:n{\\dim_to_fp:n{\\l_tmpb_dim}/(\\dim_to_fp:n{100mm}/100)}}\\pgfextractx\\l_tmpa_dim{\\pgfpointanchor{set-result}{north~east}}\\pgfextracty\\l_tmpb_dim{\\pgfpointanchor{set-result}{north~east}}\\semio_viz_probe_values:nx{set/max}{\\fp_eval:n{\\dim_to_fp:n{\\l_tmpa_dim}/(\\dim_to_fp:n{100mm}/100)},\\fp_eval:n{\\dim_to_fp:n{\\l_tmpb_dim}/(\\dim_to_fp:n{100mm}/100)}}\\ExplSyntaxOff\\end{VizFigure}` }));
    const root = join(workDir, theme), records = await compileVizProbeDocument({ case: "set-containment", scenario: "cases", documentClass: "semio", documentClassOptions: `type=paper,language=en,theme=${theme}`, packages: ["semio-viz"], geometry: true, preamble: ["\\makeatletter\\newcommand\\SetMeasure{\\pgf@relevantforpicturesizetrue}\\makeatother", "\\title{Set Containment}", "\\author{Semio}", "\\date{}"], body }, { workDir: root, scenario: undefined, keepWorkDir: true });
    let values = 0, paths = 0, pages = 0;
    for (const entry of cases) {
      const own = records.filter(record => record.scenario === entry.id), circles = own.filter(record => record.key === "geometry/set/circle").map(record => record.values.map(Number)), distance = Number(own.find(record => record.key === "geometry/set/distance")?.values[0]), oracle = expected.get(entry.id)!;
      if (circles.length !== 2) failures.push(`${theme}/${entry.id}: expected two circle records`);
      for (const [index, circle] of circles.entries()) for (const [axis, value] of circle.entries()) {
        if (!Number.isFinite(value) || Math.abs(value - oracle.circles[index]![axis]!) > fixture.geometryTolerance) failures.push(`${theme}/${entry.id}: circle ${index} field ${axis} differs from independent geometry`);
        values++;
      }
      if (!Number.isFinite(distance) || Math.abs(distance - oracle.distance) > fixture.geometryTolerance) failures.push(`${theme}/${entry.id}: distance differs from independent polygon intersection`);
      if (circles.length === 2 && circles.every(circle => circle[2]! > 0)) {
        const actualArea = overlapArea(circles.map(circle => circle[2]!), distance);
        if (Math.abs(actualArea - oracle.overlap) > fixture.overlapTolerance) failures.push(`${theme}/${entry.id}: actual radii/distance do not preserve declared overlap area`);
      } else failures.push(`${theme}/${entry.id}: positive set sizes require positive radii`);
      if (entry.fit) {
        const min = own.find(record => record.key === "set/min")?.values.map(Number), max = own.find(record => record.key === "set/max")?.values.map(Number);
        for (let axis = 0; axis < 2; axis++) if (!min || !max || min[axis]! < entry.padding - fixture.geometryTolerance || max[axis]! > entry.canvas[axis]! - entry.padding + fixture.geometryTolerance) failures.push(`${theme}/${entry.id}: actual native bounds exceed declared padded canvas`);
      }
      values += 2;
    }
    const out = join(root, "🧪️probe-out"), pdf = await getDocument({ data: new Uint8Array(readFileSync(join(out, readdirSync(out).find(name => name.endsWith(".pdf"))!))) }).promise, seen = new Set<string>();
    try {
      for (let index = 1; index <= pdf.numPages; index++) {
        const page = await pdf.getPage(index), text = (await page.getTextContent()).items.flatMap(item => "str" in item ? [item.str] : []).join(" "), entry = cases.find(entry => text.includes(entry.id));
        if (/[\uFFFD\uFFFF]/u.test(text)) failures.push(`${theme}/page${index}: invalid published glyph`);
        if (entry) {
          if (seen.has(entry.id)) failures.push(`${theme}/${entry.id}: duplicate actual case page`);
          seen.add(entry.id);
          const operators = await page.getOperatorList(), circles: { path: number[]; matrix: number[]; width: number }[] = [], stack: { matrix: number[]; width: number }[] = [], project = scaleLinear([0, 1], [0, 72 / 25.4]);
          let matrix = [1, 0, 0, 1, 0, 0], width = 1, canvasMatrix: number[] | undefined;
          for (const [operation, id] of operators.fnArray.entries()) {
            const args = operators.argsArray[operation];
            if (id === OPS.save || id === OPS.paintFormXObjectBegin) {
              stack.push({ matrix: [...matrix], width });
              if (id === OPS.paintFormXObjectBegin && args[0]) matrix = Util.transform(matrix, args[0]);
            } else if (id === OPS.restore || id === OPS.paintFormXObjectEnd) {
              const saved = stack.pop();
              if (!saved) failures.push(`${theme}/${entry.id}: invalid PDF graphics stack`);
              else { matrix = saved.matrix; width = saved.width; }
            }
            else if (id === OPS.transform) matrix = Util.transform(matrix, args);
            else if (id === OPS.setLineWidth) width = Number(args[0]);
            else if (id === OPS.setGState) {
              for (const state of args[0]) if (state[0] === "LW") width = Number(state[1]);
            }
            else if (id === OPS.constructPath && args[2]) {
              const path = args[1]?.[0], bounds = Array.from(args[2], Number);
              if (args[0] === OPS.stroke && path?.length === 38 && [6, 13, 20, 27].every(position => path[position] === 2) && path[34] === 4) circles.push({ path: Array.from(path, Number), matrix: [...matrix], width });
              if (path?.length === 19 && [6, 9, 12].every(position => path[position] === 1) && path[15] === 4 && bounds.every((value, axis) => Math.abs(value - [0, 0, project(entry.canvas[0]!), project(entry.canvas[1]!)][axis]!) < project(fixture.geometryTolerance))) {
                if (canvasMatrix) failures.push(`${theme}/${entry.id}: duplicate independent PDF canvas reference`);
                canvasMatrix = [...matrix];
              }
            }
          }
          if (circles.length !== 2) failures.push(`${theme}/${entry.id}: expected two actual four-cubic circle strokes, found ${circles.length}`);
          if (!canvasMatrix) failures.push(`${theme}/${entry.id}: independent actual PDF canvas reference missing`);
          const oracle = expected.get(entry.id)!;
          for (const [index, circle] of circles.entries()) {
            const canvas = canvasMatrix ?? [1, 0, 0, 1, 0, 0], bounds = circleBounds(circle.path, circle.matrix), target = expectedBounds(oracle.circles[index]!, canvas), tolerance = project(fixture.geometryTolerance) * Math.max(Math.hypot(canvas[0]!, canvas[2]!), Math.hypot(canvas[1]!, canvas[3]!));
            if (bounds.some((value, axis) => Math.abs(value - target[axis]!) > tolerance)) failures.push(`${theme}/${entry.id}: actual PDF circle ${index} differs from independent final-page D3 projection`);
            if (Math.abs(circle.width - fixture.strokeWidthPt * 72 / 72.27) > 0.002) failures.push(`${theme}/${entry.id}: actual PDF stroke differs from neutral theme token`);
            if (entry.fit && canvasMatrix) {
              const relative = Util.transform(Util.inverseTransform(canvas), circle.matrix), local = circleBounds(circle.path, relative), half = [circle.width / 2 * Math.hypot(relative[0]!, relative[2]!), circle.width / 2 * Math.hypot(relative[1]!, relative[3]!)];
              for (let axis = 0; axis < 2; axis++) if (local[axis]! - half[axis]! < project(entry.padding - fixture.geometryTolerance) || local[axis + 2]! + half[axis]! > project(entry.canvas[axis]! - entry.padding + fixture.geometryTolerance)) failures.push(`${theme}/${entry.id}: actual transformed PDF stroke exceeds padded canvas`);
            }
            paths++;
          }
          if (stack.length) failures.push(`${theme}/${entry.id}: unbalanced PDF graphics stack`);
        }
        page.cleanup(); pages++;
      }
    } finally { await pdf.destroy(); }
    for (const entry of cases) if (!seen.has(entry.id)) failures.push(`${theme}/${entry.id}: missing actual PDF case`);
    console.log(`[native-sets] ${theme}: ${cases.length} cases, ${values} independent values, ${paths} actual PDF circle strokes, ${pages} PDF.js pages consumed`);
  }
  if (failures.length) throw Error(`${failures.length} proportional set failures:\n${failures.join("\n")}`);
  console.log("[native-sets] D3 radii, polygon-clipping overlap/distance and actual PDF containment PASS");
}
