/** ⚛️ Verifies physical diagram containment and text separation against independent D3 and PDF.js. */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { getDocument, OPS, Util } from "pdfjs-dist/legacy/build/pdf.mjs";
import { scaleLinear } from "d3-scale";
import { geoOrthographic } from "d3-geo";
import { compileVizProbeDocument } from "../../../🔨️modules/🧪️viz-probe/🟦️.ts";
import fixture from "./🔣️.json";

/** 🔬️ Compiles stock and authored physics parameters without weakening their physical values. */
export async function compileNativePhysicsContainment(workDir: string): Promise<void> {
  const cases = process.env.PRINT_NATIVE_PHYSICS_PHASE === "potential" ? fixture.cases.filter(entry => "potential" in entry) : process.env.PRINT_NATIVE_PHYSICS_PHASE === "transitions" ? fixture.cases.filter(entry => "transitionLabels" in entry) : process.env.PRINT_NATIVE_PHYSICS_PHASE === "captions" ? fixture.cases.filter(entry => "mathGlyphs" in entry) : fixture.cases;
  const failures: string[] = [];
  for (const theme of ["light", "dark"]) {
    const root = join(workDir, theme), body = cases.map(entry => ({ raw: `\\clearpage\\SemioVizProbeBegin{physics-containment}{${entry.id}}\\begin{VizFigure}[title={${entry.id}},width=${fixture.frame[0]},height=${fixture.frame[1]}]\\begin{scope}[local bounding box=physics-result]\\PhysicsMeasure\\SemioVizChart{${entry.kind}}[${entry.options}]\\end{scope}\\ExplSyntaxOn\\pgfextractx\\l_tmpa_dim{\\pgfpointanchor{physics-result}{south~west}}\\pgfextracty\\l_tmpb_dim{\\pgfpointanchor{physics-result}{south~west}}\\semio_viz_probe_values:nx{physics/min}{\\fp_eval:n{\\dim_to_fp:n{\\l_tmpa_dim}/(\\dim_to_fp:n{100mm}/100)},\\fp_eval:n{\\dim_to_fp:n{\\l_tmpb_dim}/(\\dim_to_fp:n{100mm}/100)}}\\pgfextractx\\l_tmpa_dim{\\pgfpointanchor{physics-result}{north~east}}\\pgfextracty\\l_tmpb_dim{\\pgfpointanchor{physics-result}{north~east}}\\semio_viz_probe_values:nx{physics/max}{\\fp_eval:n{\\dim_to_fp:n{\\l_tmpa_dim}/(\\dim_to_fp:n{100mm}/100)},\\fp_eval:n{\\dim_to_fp:n{\\l_tmpb_dim}/(\\dim_to_fp:n{100mm}/100)}}\\ExplSyntaxOff\\end{VizFigure}` }));
    const records = await compileVizProbeDocument({ case: "physics-containment", scenario: "cases", documentClass: "semio", documentClassOptions: `type=paper,language=en,theme=${theme}`, packages: ["semio-viz"], geometry: true, preamble: ["\\makeatletter\\newcommand\\PhysicsMeasure{\\pgf@relevantforpicturesizetrue}\\makeatother", "\\title{Physics Containment}", "\\author{Semio}", "\\date{}"], body }, { workDir: root, scenario: undefined, keepWorkDir: true });
    let referenceCount = 0;
    for (const entry of cases) {
      const own = records.filter(record => record.scenario === entry.id), min = own.find(record => record.key === "physics/min")!.values.map(Number), max = own.find(record => record.key === "physics/max")!.values.map(Number);
      for (let axis = 0; axis < 2; axis++) if (min[axis]! < -fixture.tolerance || max[axis]! > entry.canvas[axis]! + fixture.tolerance) failures.push(`${theme}/${entry.id}: actual bounds ${min} .. ${max} exceed ${entry.canvas}`);
      if ("energies" in entry && entry.energies) {
        const values = [...entry.energies];
        if ("potential" in entry && entry.potential) {
          const [coefficient, intercept, from, to] = entry.potential, sample = scaleLinear([0, 120], [from!, to!]);
          for (let index = 0; index <= 120; index++) values.push(coefficient! * sample(index) ** 2 + intercept!);
        }
        const extend = scaleLinear([0, 1], [Math.min(...values), Math.max(...values)]), expected = "explicitRange" in entry && entry.explicitRange ? entry.explicitRange : [extend(-0.12), extend(1.12)], actual = own.find(record => record.key === "geometry/levels/window")!.values.slice(2).map(Number);
        if (actual.some((value, axis) => Math.abs(value - expected[axis]!) > 1e-4)) failures.push(`${theme}/${entry.id}: energy viewport differs from D3 complete authored range`);
        if ("transitionPairs" in entry && entry.transitionPairs) {
          const actual = own.filter(record => record.key === "geometry/levels/transition");
          for (const [index, pair] of entry.transitionPairs.entries()) {
            const energy = scaleLinear([0, 1], [entry.energies[pair[0]! - 1]!, entry.energies[pair[1]! - 1]!]);
            if (!actual[index] || Math.abs(Number(actual[index]!.values[0]) - (energy(0) - energy(1))) > 1e-4) failures.push(`${theme}/${entry.id}: transition energy differs from independent D3 endpoints`);
            referenceCount++;
          }
        }
        referenceCount++;
      }
      if ("forces" in entry && entry.forces && entry.forceScale !== undefined) {
        const magnitude = scaleLinear([0, 1], [0, entry.forceScale]), actual = own.filter(record => record.key === "geometry/force/vector");
        for (const [index, force] of entry.forces.entries()) {
          const expected = [magnitude(force[0]!) * Math.cos(force[1]! * Math.PI / 180), magnitude(force[0]!) * Math.sin(force[1]! * Math.PI / 180)];
          if (!actual[index] || actual[index]!.values.some((value, axis) => Math.abs(Number(value) - expected[axis]!) > 1e-4)) failures.push(`${theme}/${entry.id}: force components differ from D3 magnitude scale`);
          referenceCount++;
        }
      }
      if ("optics" in entry && entry.optics) {
        const [focal, distance, height] = entry.optics, image = focal! * distance! / (distance! - focal!), expected = [image, scaleLinear([0, distance!], [0, -height!])(image)], actual = own.find(record => record.key === "geometry/optics/image")!.values.map(Number);
        if (actual.some((value, axis) => Math.abs(value - expected[axis]!) > 1e-4)) failures.push(`${theme}/${entry.id}: thin-element image differs from independent D3 magnification`);
        referenceCount++;
      }
      if ("sphere" in entry && entry.sphere && entry.states) {
        const [radius, azimuth, elevation] = entry.sphere, project = geoOrthographic().rotate([-azimuth!, -elevation!]).translate([0, 0]).scale(radius!), actual = own.filter(record => record.key === "geometry/poincare/state");
        for (const [index, state] of entry.states.entries()) {
          const length = Math.hypot(...state), projected = length === 0 ? [0, 0] : project([Math.atan2(state[1]!, state[0]!) * 180 / Math.PI, Math.asin(state[2]! / length) * 180 / Math.PI])!, expected = [projected[0]!, -projected[1]!];
          if (!actual[index] || actual[index]!.values.some((value, axis) => Math.abs(Number(value) - expected[axis]!) > 1e-4)) failures.push(`${theme}/${entry.id}: Stokes projection differs from independent D3 orthographic projection`);
          referenceCount++;
        }
      }
    }
    const out = join(root, "🧪️probe-out"), pdf = await getDocument({ data: new Uint8Array(readFileSync(join(out, readdirSync(out).find(name => name.endsWith(".pdf"))!))) }).promise;
    let pages = 0, potentialReferences = 0, potentialCaptions = 0;
    try {
      for (let page = 1; page <= pdf.numPages; page++) {
        const pdfPage = await pdf.getPage(page), items = (await pdfPage.getTextContent()).items.flatMap(item => "str" in item ? [item] : []), text = items.map(item => item.str).join(" "), entry = cases.find(entry => text.includes(entry.id));
        if (entry && "potential" in entry && entry.potential && entry.labels) {
          const operators = await pdfPage.getOperatorList(), stack: number[][] = [], curves: { local: number[][]; page: number[][] }[] = [];
          let matrix = [1, 0, 0, 1, 0, 0];
          for (const [index, operation] of operators.fnArray.entries()) {
            const args = operators.argsArray[index];
            if (operation === OPS.save) stack.push([...matrix]);
            else if (operation === OPS.restore) matrix = stack.pop() ?? [1, 0, 0, 1, 0, 0];
            else if (operation === OPS.transform) matrix = Util.transform(matrix, args);
            else if (operation === OPS.constructPath) {
              const path = args[1]?.[0];
              if (path?.length === fixture.potentialPathPoints * 3 && path[0] === 0 && Array.from({ length: fixture.potentialPathPoints - 1 }, (_, point) => path[(point + 1) * 3]).every(value => value === 1)) {
                const local = Array.from({ length: fixture.potentialPathPoints }, (_, point) => [Number(path[point * 3 + 1]), Number(path[point * 3 + 2])]);
                curves.push({ local, page: local.map(point => { const projected = [...point]; Util.applyTransform(projected, matrix); return projected; }) });
              }
            }
          }
          if (curves.length !== 1) failures.push(`${theme}/${entry.id}: expected one actual ${fixture.potentialPathPoints}-point potential stroke, found ${curves.length}`);
          for (const curve of curves) {
            const [coefficient, intercept, from, to] = entry.potential, sample = scaleLinear([0, fixture.potentialPathPoints - 1], [from!, to!]), values = [...entry.energies, ...curve.local.map((_, index) => coefficient! * sample(index) ** 2 + intercept!)], extend = scaleLinear([0, 1], [Math.min(...values), Math.max(...values)]), ordinate = scaleLinear([extend(-0.12), extend(1.12)], [0, entry.canvas[1]! * 72 / 25.4]), abscissa = scaleLinear([from!, to!], [0, entry.canvas[0]! * fixture.potentialViewportFraction * 72 / 25.4]);
            for (const [index, point] of curve.local.entries()) {
              const x = sample(index), expected = [abscissa(x), ordinate(coefficient! * x ** 2 + intercept!)];
              if (point.some((value, axis) => Math.abs(value - expected[axis]!) > 0.1)) failures.push(`${theme}/${entry.id}: PDF potential point ${index} differs from independent D3 viewport projection`);
              potentialReferences++;
            }
            for (const name of entry.labels) {
              const label = items.find(item => item.str.trim() === name), gap = fixture.potentialCaptionGap * 72 / 25.4;
              if (!label) continue;
              const left = label.transform[4] - gap, right = label.transform[4] + label.width + gap, bottom = label.transform[5] - gap, top = label.transform[5] + label.height + gap;
              const intersects = curve.page.slice(1).some((end, index) => {
                const start = curve.page[index]!, from = Math.max(start[0]!, left), to = Math.min(end[0]!, right);
                if (from > to) return false;
                const segment = scaleLinear([start[0]!, end[0]!], [start[1]!, end[1]!]), ys = [segment(from), segment(to)];
                return Math.min(...ys) <= top && Math.max(...ys) >= bottom;
              });
              if (intersects) failures.push(`${theme}/${entry.id}: actual potential stroke intersects authored caption ${name}`);
              potentialCaptions++;
            }
          }
        }
        if (entry && "labels" in entry && entry.labels) {
          const names = [...entry.labels, ...("transitionLabels" in entry && entry.transitionLabels ? entry.transitionLabels : [])];
          const labels = names.map(label => items.find(item => item.str.trim() === label || item.str.startsWith(label + " ")));
          for (const [index, label] of labels.entries()) {
            if (!label) failures.push(`${theme}/${entry.id}: missing authored physics label ${names[index]}`);
            else for (const other of labels.slice(index + 1)) if (other && Math.min(label.transform[4] + label.width, other.transform[4] + other.width) > Math.max(label.transform[4], other.transform[4]) && Math.min(label.transform[5] + label.height, other.transform[5] + other.height) + fixture.minimumLabelGap * 72 / 25.4 > Math.max(label.transform[5], other.transform[5])) failures.push(`${theme}/${entry.id}: PDF.js authored label rectangles overlap`);
          }
        }
        if (entry && "mathGlyphs" in entry && entry.mathGlyphs) for (const glyph of new Set(entry.mathGlyphs)) if ([...text].filter(value => value === glyph).length < entry.mathGlyphs.filter(value => value === glyph).length) failures.push(`${theme}/${entry.id}: missing authored mathematical transition glyph ${glyph}`);
        if (/[\uFFFD\uFFFF]/u.test(text)) failures.push(`${theme}/page${page}: invalid extracted physics glyph`);
        pages++;
      }
    } finally { await pdf.destroy(); }
    console.log(`[native-physics] ${theme}: ${cases.length} compiled cases, ${referenceCount} D3 physical projections, ${potentialReferences} D3 PDF potential points, ${potentialCaptions} actual path/caption checks, ${pages} PDF.js pages consumed`);
  }
  if (failures.length) throw Error(`${failures.length} physics containment/projection failures:\n${failures.join("\n")}`);
  console.log("[native-physics] stock/authored forces, motion, levels, bands, optics and Stokes containment PASS");
}
