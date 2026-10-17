/** 🧪️ Compiles actual native inference output and neutral grammar vectors with independent D3 oracles. This module runs the compilations in order; the compilations themselves sit in the child folders `🧰️grammar-kit`, `🧱️basics`, `🎛️family-and-scientific`, `🗺️geo-planar`, `🏭️domain-effects` and `🧠️neural-and-domain`, next to the existing `🎨️paint`, `📥️emission` and `💹️financial`.
 *
 * @see ./📖️source-files/🟦️.ts — the source files the source-text laws of the grammar read
 */
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { capturePrintNativeEmissionV1 } from "./📥️emission/🟦️.ts";
import { withPrintScratchV1, type PrintExecutionV1 } from "../../📦️artifacts/🎛️operation/🟦️.ts";
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { scaleLinear, scaleOrdinal, scaleDiverging, scaleUtc, scalePoint } from "d3-scale";
import { line, area, arc, linkHorizontal } from "d3-shape";
import { ribbon } from "d3-chord";
import { path as d3Path } from "d3-path";
import { rgb } from "d3-color";
import { convert as cssColor } from "@asamuzakjp/css-color";
import { interpolateRgb, interpolateLab, interpolateHcl } from "d3-interpolate";
import { utcDay } from "d3-time";
import { sort } from "d3-array";
import { geoEquirectangular } from "d3-geo";
import { buildVizCoordinate } from "@semio-tech/print-viz-inference/coordinate";
import { compileVizProbeDocument } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
import { nativeTransformGrammarChecks } from "../🧬️chart-transform-grammar/🟦️.ts";
import { nativeLayoutGrammarChecks } from "../🧬️chart-layout-grammar/🟦️.ts";
import fixture from "./🔣️.json";
import { compileNativeFinancialGrammar, compileNativeFinancialStock } from "./💹️financial/🟦️.ts";
import { compileNativeFamilyContainment } from "../🖼️family-containment/🟦️.ts";
import { equal } from "./🧰️grammar-kit/🟦️.ts";
import { compileNativeUnknownGrammar, compileNativeMissingGrammar, compileNativeDirectedGraphGrammar, compileNativeFontMetricsGrammar, compileNativeLegendGrammar, compileNativeSpatialDataGrammar, compileNativeGeoPaletteGrammar, compileNativeScaleNiceGrammar } from "./🧱️basics/🟦️.ts";
import { compileNativeFamilyCustomizationGrammar, compileNativeScientificDefaults, compileNativeScientificControlEffects, compileNativeScientificCanvasControls, compileNativeScientificMixedControls, compileNativeScientificModeDefaults, compileNativeScientificAdmission, compileNativeSpecializedPlacementControls, compileNativeScientificRecordDomain, compileNativeScientificRelationStyles, compileNativeScientificScalarAdmission, compileNativeScientificModeDomain } from "./🎛️family-and-scientific/🟦️.ts";
import { compileNativeGeoPlanarGrammar, compileNativeGeoContourBoundary } from "./🗺️geo-planar/🟦️.ts";
import { compileNativeProcessControlEffects, compileNativeThermodynamicControlEffects, compileNativeListSeatingControlEffects, compileNativePrimitiveHullEffects, compileNativeTimelineControlEffects, compileNativeDensityControlEffects, compileNativeDiagramControlEffects, compileNativeSpatialGroupControls, compileNativePlanningControlEffects, compileNativeLogoRadarControls, compileNativeInheritedCaptionControls, compileNativeBiofabricMatrixControls, compileNativeEngineeringExplodedControls, compileNativeUpSetModeControls } from "./🏭️domain-effects/🟦️.ts";
import { compileNativeNeuralArchitectureControls, nativeNeuralIncidenceChecks, compileNativeGraphReferenceControls, compileNativeNeuralTopologyControls, compileNativeNeuralReferenceControls, compileNativeDomainReferenceControls, compileNativeCriticalPathControls, compileNativeTokenPublication, compileNativeActualMarkControls, compileNativeHiveControls, compileNativeNeuralSelfLoopControls, compileNativeGenomeWindowControls, compileNativeMusicWindowControls, compileNativeMeteogramWindowControls, compileNativeDomainScalarControls, compileNativePhysicsWindowControls, compileNativeSurfaceWindowControls, compileNativeSurfaceRangeControls } from "./🧠️neural-and-domain/🟦️.ts";

/** 🎯️ Executes the registered native inference and checks the resulting compiled mark geometry. */
export async function compileNativeGrammar(execution:PrintExecutionV1, native = true): Promise<void> {if(["PRINT_NATIVE_NEURAL_TIP_SOURCE"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/grammar",async({directory:workDir,execution,destination})=>{
  mkdirSync(workDir, { recursive: true });
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="surface-window-controls"){await compileNativeSurfaceWindowControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="physics-window-controls"){await compileNativePhysicsWindowControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="surface-range-controls"){await compileNativeSurfaceRangeControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="geo-contour-boundary"){await compileNativeGeoContourBoundary(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="domain-scalar-controls"){await compileNativeDomainScalarControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="domain-reference-controls"){await compileNativeDomainReferenceControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="critical-path-controls"){await compileNativeCriticalPathControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="token-publication"){await compileNativeTokenPublication(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-admission"){await compileNativeScientificAdmission(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="upset-controls"){await compileNativeUpSetModeControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-architecture"){await compileNativeNeuralArchitectureControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-topology"){await compileNativeNeuralTopologyControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="graph-reference"){await compileNativeGraphReferenceControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="meteogram-window-controls"){await compileNativeMeteogramWindowControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="music-window-controls"){await compileNativeMusicWindowControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="genome-window-controls"){await compileNativeGenomeWindowControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="hive-controls"){await compileNativeHiveControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="directed-graph-controls"){await compileNativeDirectedGraphGrammar(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-admission"){await nativeNeuralIncidenceChecks();return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-reference"){await compileNativeNeuralReferenceControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-self"){await compileNativeNeuralSelfLoopControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-tip"){if(process.env.PRINT_NATIVE_NEURAL_TIP_SOURCE)throw Error("Actual compiled neural probe required; free replay path refused");await compileNativeNeuralTopologyControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="engineering-exploded"){await compileNativeEngineeringExplodedControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="logo-controls"){await compileNativeLogoRadarControls(execution,"logo");return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="radar-controls"){await compileNativeLogoRadarControls(execution,"radar");return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="density-controls"){await compileNativeDensityControlEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="spatial-controls"){await compileNativeSpatialGroupControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="planning-controls"){await compileNativePlanningControlEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="thermodynamic-controls"){await compileNativeThermodynamicControlEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="timeline-controls"){await compileNativeTimelineControlEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="hull-controls"){await compileNativePrimitiveHullEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="list-seating-controls"){await compileNativeListSeatingControlEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="inherited-caption-controls"){await compileNativeInheritedCaptionControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="biofabric-matrix-controls"){await compileNativeBiofabricMatrixControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="actual-mark-controls"){await compileNativeActualMarkControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="specialized-placement-controls"){await compileNativeSpecializedPlacementControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-defaults"){compileNativeScientificDefaults();return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-canvas"){await compileNativeScientificCanvasControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-mixed-controls"){await compileNativeScientificMixedControls(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-modes"){await compileNativeScientificModeDefaults(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-mode-domain"){await compileNativeScientificModeDomain(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-scalar-admission"){await compileNativeScientificScalarAdmission();return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-styles"){await compileNativeScientificRelationStyles(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-stocks"){await compileNativeScientificModeDefaults(execution,true);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-record-domain"){await compileNativeScientificRecordDomain(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-construction-window"){await compileNativeScientificCanvasControls(execution,fixture.scientificConstructionWindows);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-decision-window"){const neutral=fixture.scientificMixedPhysicalWindows;await compileNativeScientificCanvasControls(execution,{vectors:{...neutral.vectors,cases:neutral.vectors.cases.filter(entry=>entry.kind==="decision-lattice")}});return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="diagram-controls"){await compileNativeDiagramControlEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scientific-controls"){await compileNativeScientificControlEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="process-controls"){await compileNativeProcessControlEffects(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="meaningful-controls"){
    const group=process.env.PRINT_NATIVE_CONTROL_GROUP;
    if(!group||group==="process")await compileNativeTokenPublication(execution);
    if(!group||group==="diagram"){await compileNativeDiagramControlEffects(execution);await compileNativeSpatialGroupControls(execution);await compileNativePlanningControlEffects(execution);}
    if(!group||group==="scientific"){await compileNativeScientificControlEffects(execution);await compileNativeScientificDefaults();await compileNativeScientificCanvasControls(execution);await compileNativeScientificMixedControls(execution);await compileNativeScientificCanvasControls(execution,fixture.scientificConstructionWindows);await compileNativeSurfaceWindowControls(execution);await compileNativeSurfaceRangeControls(execution);await compileNativePhysicsWindowControls(execution);await compileNativeScientificModeDefaults(execution);await compileNativeDensityControlEffects(execution);await compileNativeLogoRadarControls(execution,"logo");await compileNativeLogoRadarControls(execution,"radar");await compileNativeScientificAdmission(execution);await compileNativeUpSetModeControls(execution);await compileNativeScientificRecordDomain(execution);await compileNativeScientificRelationStyles(execution);}
    if(!group||group==="process"){await compileNativeProcessControlEffects(execution);await compileNativeDomainReferenceControls(execution);await compileNativeDomainScalarControls(execution);await compileNativeCriticalPathControls(execution);await compileNativeThermodynamicControlEffects(execution);await compileNativeTimelineControlEffects(execution);await compileNativePrimitiveHullEffects(execution);await compileNativeListSeatingControlEffects(execution);await compileNativeInheritedCaptionControls(execution);await compileNativeBiofabricMatrixControls(execution);await compileNativeSpecializedPlacementControls(execution);await compileNativeActualMarkControls(execution);await compileNativeEngineeringExplodedControls(execution);}
    if(group&&!["diagram","scientific","process"].includes(group))throw Error("Unknown native control group "+group);
    return;
  }
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="geo-planar"){await compileNativeGeoPlanarGrammar(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="spatial-data"){await compileNativeSpatialDataGrammar(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="geo-palette"){await compileNativeGeoPaletteGrammar(execution);return;}
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="scale-options"){await compileNativeScaleNiceGrammar(execution);return;}
  if(!process.env.PRINT_NATIVE_GRAMMAR_PHASE)await compileNativeTokenPublication(execution);
  await compileNativeFamilyCustomizationGrammar(execution);
  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="family-customization")return;
  await compileNativeDiagramControlEffects(execution);
  await compileNativeSpatialGroupControls(execution);
  await compileNativePlanningControlEffects(execution);
  await compileNativeScientificControlEffects(execution);
  await compileNativeScientificDefaults();
  await compileNativeScientificCanvasControls(execution);
  await compileNativeScientificMixedControls(execution);
  await compileNativeScientificCanvasControls(execution,fixture.scientificConstructionWindows);
  await compileNativeSurfaceWindowControls(execution);
  await compileNativeSurfaceRangeControls(execution);
  await compileNativePhysicsWindowControls(execution);
  await compileNativeScientificModeDefaults(execution);
  await compileNativeDensityControlEffects(execution);await compileNativeLogoRadarControls(execution,"logo");await compileNativeLogoRadarControls(execution,"radar");await compileNativeScientificAdmission(execution);await compileNativeUpSetModeControls(execution);await compileNativeScientificRecordDomain(execution);await compileNativeScientificRelationStyles(execution);
  await compileNativeProcessControlEffects(execution);
  await compileNativeDomainReferenceControls(execution);await compileNativeDomainScalarControls(execution);
  await compileNativeCriticalPathControls(execution);
  await compileNativeThermodynamicControlEffects(execution);
  await compileNativeTimelineControlEffects(execution);
  await compileNativePrimitiveHullEffects(execution);
  await compileNativeListSeatingControlEffects(execution);
  await compileNativeInheritedCaptionControls(execution);
  await compileNativeBiofabricMatrixControls(execution);
  await compileNativeSpecializedPlacementControls(execution);await compileNativeActualMarkControls(execution);await compileNativeEngineeringExplodedControls(execution);
  await compileNativeScaleNiceGrammar(execution);
  await compileNativeGeoPaletteGrammar(execution);
  await compileNativeSpatialDataGrammar(execution);
  await compileNativeGeoContourBoundary(execution);
  await compileNativeGeoPlanarGrammar(execution);
  if (native) {
    const emitted=await capturePrintNativeEmissionV1(execution,destination),source=join(workDir,"native-chart.tex"),spatialSources=join(workDir,"native-spatial-sources"),planarSources=join(workDir,"native-planar-sources");
    const{writeFile,mkdir}=await import("node:fs/promises");await mkdir(spatialSources);await mkdir(planarSources);
    for(const file of emitted){const path=file.kind==="main"?source:join(file.kind==="spatial"?spatialSources:planarSources,file.name);if(path.length>256)throw Error("Finite Print emitter destination required");checkResourceLeaseControl(execution.phaseControl);await writeFile(path,file.source,{flag:"wx"});}
    const text = readFileSync(source, "utf8");
    const typedProbes = fixture.typedValues.map((_, index) => `\\ExplSyntaxOn\\semio_viz_table_cell:nnnN{native-typed}{${index+1}}{x}\\l_tmpa_tl\\semio_viz_scale_value:nVN{native-typed-x}\\l_tmpa_tl\\l_tmpb_tl\\semio_viz_probe_values:nV{native/typed/${index}}\\l_tmpb_tl\\semio_viz_scale_value:nVN{native-typed-color}\\l_tmpa_tl\\l_tmpb_tl\\semio_viz_probe_values:nx{native/typed-alpha/${index}}{\\semio_viz_color_alpha:n{\\l_tmpb_tl}}\\ExplSyntaxOff`).join("\n");
    const records = await compileVizProbeDocument({ case: "native-chart-grammar", scenario: "native-emission", documentClass: "semio", documentClassOptions: "type=paper,language=en", packages: ["semio-viz"], geometry: true, preamble: ["\\title{Native Chart Grammar}", "\\author{Semio}", "\\date{}", "\\errorcontextlines=200"], body: [{ raw: text.replace("\\end{VizFigure}", typedProbes + "\n\\end{VizFigure}") }] }, {execution,});
    if (records.filter((r) => r.key.startsWith("geometry/plot/")).length < fixture.marks.length) throw new Error("native emission did not paint all canonical marks");
    const typedScale = scaleLinear([0,3],[0,30]).unknown(-5);
    for (const [index, value] of fixture.typedValues.entries()) {
      equal(records.find(record => record.key === `native/typed/${index}`)!.values.map(Number), [typedScale(value as unknown as number)], 'actual native typed row values');
      equal(records.find(record => record.key === `native/typed-alpha/${index}`)!.values.map(Number), [value === null || value === 'bad' ? .35 : 1], 'actual native functional unknown paint alpha');
    }
    await compileNativeSpatialDataGrammar(execution,spatialSources);
    await compileNativeGeoPlanarGrammar(execution,planarSources);
    console.log("[native-grammar] actual native-emitted TikZ compiled");
  }
  const x = scaleLinear(fixture.xDomain, fixture.xRange);
  const y = scaleLinear(fixture.yDomain, fixture.yRange);
  const fill = scaleOrdinal(fixture.colorDomain, fixture.colorRange);
  const expected = fixture.rows.flatMap((r) => [x(Number(r[0])), y(Number(r[1]))]);
  const rows = fixture.rows.map((r) => `\\SemioVizRow{grammar}{${r[0]},${r[1]},${r[2]},4,black,0.6,0.4,30,circle,label,0,1.4,8,2}`).join("\n");
  const tables = `\\SemioVizTable{grammar}{x,y,category,size,stroke,opacity,strokeWidth,rotation,shape,text,angle,endAngle,radius,innerRadius}\n${rows}\n\\SemioVizScale{authored-x}{linear}{0,10}{20,60}\n\\SemioVizScale{authored-y}{linear}{0,10}{30,10}\n\\SemioVizScale{authored-fill}{ordinal}{a,b}{blue,red}`;
  const marks = fixture.marks.map((mark) => ({ raw: `\\SemioVizProbeBegin{native-chart-grammar}{${mark}}\n\\SemioVizPlot[data=grammar,mark=${mark},x={column=x,scale=authored-x},y={column=y,scale=authored-y},fill={column=category,scale=authored-fill},stroke={column=stroke},size={column=size},opacity={column=opacity},strokeWidth={column=strokeWidth},rotation={column=rotation},shape={column=shape},text={column=text},angle={column=angle},endAngle={column=endAngle},radius={column=radius},innerRadius={column=innerRadius}]` }));
  const records = await compileVizProbeDocument({ case: "native-chart-grammar", scenario: "marks", packages: ["semio-viz-plot"], geometry: true, body: [{ raw: "\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{en}\\bool_set_true:N\\l_semio_viz_in_figure_bool\\fp_set:Nn\\l_semio_viz_width_fp{80}\\fp_set:Nn\\l_semio_viz_height_fp{40}\\ExplSyntaxOff\n\\begin{tikzpicture}[x=1mm,y=1mm]" }, { raw: tables }, ...marks, { raw: "\\end{tikzpicture}" }] }, {execution,scenario: undefined});
  for (const mark of fixture.marks) {
    const own = records.filter((r) => r.scenario === mark);
    equal(own.filter((r) => r.key === "geometry/plot/point").flatMap((r) => r.values.map(Number)), expected, `${mark} positions`);
    if (own.filter((r) => r.key !== "geometry/plot/point" && r.key.startsWith("geometry/")).length === 0) throw new Error(`${mark} emitted no geometry`);
    const styles = own.filter((r) => r.key === "grammar/style");
    if (styles.length !== fixture.rows.length) throw new Error(`${mark} emitted no resolved style probes`);
    const points=fixture.rows.map(r=>[x(Number(r[0])),y(Number(r[1]))] as [number,number]);
    const numeric=(value:string)=>value.match(/[-+]?(?:\d*\.?\d+)(?:e[-+]?\d+)?/gi)!.map(Number);
    const make=(draw:(p:ReturnType<typeof d3Path>)=>void)=>{const p=d3Path();draw(p);return numeric(String(p));};
    const vectors: number[][]=[];
    const rectangles=['bar','rect','heatmap'].includes(mark);
    for(const [i,point] of points.entries()) {
      const [px,py]=point;
      if(rectangles)vectors.push(make(p=>p.rect(px,mark==='bar'?Math.min(py,y(0)):py,6,mark==='bar'?Math.abs(py-y(0)):6)));
      if(['point','circle','symbol'].includes(mark))vectors.push(make(p=>{p.arc(0,0,8,0,2*Math.PI);p.closePath();}));
      if(['arc','pie'].includes(mark)){const p=d3Path();arc<{startAngle:number;endAngle:number}>().innerRadius(2).outerRadius(8).context(p as unknown as CanvasRenderingContext2D)({startAngle:0,endAngle:1.4});vectors.push(numeric(String(p)));}
      if(mark==='ribbon'){const p=d3Path();ribbon().context(p as unknown as CanvasRenderingContext2D)({source:{radius:8,startAngle:0,endAngle:1.4},target:{radius:8,startAngle:2,endAngle:2.8}});vectors.push(numeric(String(p)));}
      if(mark==='rule')vectors.push(make(p=>{p.moveTo(px,py);p.lineTo(78,py);}));
      if(mark==='link'){const p=d3Path();linkHorizontal().context(p as unknown as CanvasRenderingContext2D)({source:point,target:[78,py]});vectors.push(numeric(String(p)));}
      if(['line','path','trail','area','band'].includes(mark)&&i>0){const p=d3Path(),pair=[points[i-1]!,point];if(['area','band'].includes(mark))area().x((v:[number,number])=>v[0]).y1((v:[number,number])=>v[1]).y0(y(0)).context(p as unknown as CanvasRenderingContext2D)(pair);else line().context(p as unknown as CanvasRenderingContext2D)(pair);vectors.push(numeric(String(p)));}
    }
    if(mark==='polygon')vectors.push(make(p=>{points.forEach(([px,py],i)=>i?p.lineTo(px,py):p.moveTo(px,py));p.closePath();}));
    if(mark==='text')points.forEach(v=>vectors.push(v));
    const painted=own.filter(r=>r.key.startsWith('geometry/')&&r.key!=='geometry/plot/point');
    if(painted.length!==vectors.length)throw Error(mark+' painted '+painted.length+' paths, expected '+vectors.length);
    painted.forEach((r,i)=>equal(r.values.filter(v=>typeof v==='number') as number[],vectors[i]!,mark+' D3 primitive geometry'));
    styles.forEach((r, i) => {
      const values = r.values.map(String);
      if (values[0] !== fill(String(fixture.rows[i]![2])) || values[1] !== "black") throw new Error(`${mark} style colors ${values}`);
      equal(values.slice(2).map(Number), [0.6, 0.4, 30], `${mark} style controls`);
    });
  }
  console.log(`[native-grammar] ${fixture.marks.length} canonical marks, named scales and row styles matched D3`);
  const groups = await compileVizProbeDocument({ case: "native-chart-grammar", scenario: "detail-order-color", packages: ["semio-viz-plot"], geometry: true, body: [{ raw: "\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{en}\\bool_set_true:N\\l_semio_viz_in_figure_bool\\ExplSyntaxOff\\begin{tikzpicture}[x=1mm,y=1mm]\\SemioVizTable{groups}{x,y,detail,order}\\SemioVizRow{groups}{60,10,a,2}\\SemioVizRow{groups}{50,15,b,2}\\SemioVizRow{groups}{20,30,a,1}\\SemioVizRow{groups}{30,25,b,1}\\SemioVizPlot[data=groups,mark=line,x={column=x},y={column=y},detail={column=detail},order={column=order},stroke=black,clip=true,x0=10,y0=5,x1=70,y1=35]\\SemioVizScale{color}{linear}{0,10}{blue,red}\\ExplSyntaxOn\\semio_viz_scale_value:nnN{color}{5}\\l_tmpa_tl\\convertcolorspec{named}{\\l_tmpa_tl}{rgb}\\l_tmpb_tl\\semio_viz_probe_values:nV{color/rgb}\\l_tmpb_tl\\SemioVizScale{time}{temporal}{0,86400000}{0,40}\\semio_viz_scale_map:nnN{time}{43200000}\\l_tmpa_fp\\semio_viz_probe_values:nx{time/midpoint}{\\fp_to_decimal:N\\l_tmpa_fp}\\ExplSyntaxOff\\end{tikzpicture}" }] }, {execution,});
  const paths = groups.filter((r) => r.key === "geometry/mark/polyline");
  const expectedPaths = [[[20, 30], [60, 10]], [[30, 25], [50, 15]]].map((points) => line()(points as [number, number][])!.match(/[-+]?\d*\.?\d+/g)!.map(Number));
  if (paths.length !== expectedPaths.length) throw new Error(`detail groups emitted ${paths.length} paths`);
  paths.forEach((path, i) => equal(path.values.filter((n) => typeof n === "number") as number[], expectedPaths[i]!, "ordered grouped paths"));
  const color = rgb(scaleLinear<string>().domain([0, 10]).range(["blue", "red"])(5));
  equal(groups.find((r) => r.key === "color/rgb")!.values.map(Number), [color.r / 255, color.g / 255, color.b / 255], "continuous RGB interpolation");
  equal(groups.find((r) => r.key === "time/midpoint")!.values.map(Number), [20], "numeric temporal midpoint");
  console.log("[native-grammar] ordered detail paths, clipping, continuous color and numeric temporal scales passed");

  const frame=fixture.coordinateFrame,options=fixture.coordinateOptions;
  const coordinateBody: {raw:string}[]=[{raw:'\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{en}\\bool_set_true:N\\l_semio_viz_in_figure_bool\\fp_set:Nn\\l_semio_viz_width_fp{80}\\fp_set:Nn\\l_semio_viz_height_fp{40}\\ExplSyntaxOff\\begin{tikzpicture}[x=1mm,y=1mm]\\SemioVizTable{coordinate}{x,y}\\SemioVizRow{coordinate}{.25,.75}\\SemioVizScale{coord-x}{linear}{0,1}{20,60}\\SemioVizScale{coord-y}{linear}{0,1}{30,10}'}];
  for(const kind of fixture.coordinates)coordinateBody.push({raw:'\\SemioVizProbeBegin{native-chart-grammar}{coord-'+kind+'}\\SemioVizPlot[data=coordinate,mark=point,x={column=x,scale=coord-x},y={column=y,scale=coord-y},coordinate='+kind+',coordinateOptions={normalized=true,startAngle=.3,endAngle=5.1,innerRadius=2,outerRadius=8,base=10,axes={a,b,c},projection=equirectangular},x0=16,y0=8,x1=72,y1=26]'});
  coordinateBody.push({raw:'\\end{tikzpicture}'});
  const coordinateRecords=await compileVizProbeDocument({case:'native-chart-grammar',scenario:'coordinates',packages:['semio-viz-plot','semio-viz-geo-projection'],geometry:true,body:coordinateBody},{execution,scenario:undefined});
  for(const kind of fixture.coordinates){const scaledX=scaleLinear([0,1],[20,60])(.25),scaledY=scaleLinear([0,1],[30,10])(.75);const expected=kind==='geographic'?geoEquirectangular().scale(Math.min(56/(2*Math.PI),18/Math.PI)).translate([44,17])([scaledX,scaledY])!:buildVizCoordinate(kind as Parameters<typeof buildVizCoordinate>[0],frame,options).project((scaledX-20)/40,(scaledY-30)/-20);equal(coordinateRecords.find(r=>r.scenario==='coord-'+kind&&r.key==='geometry/plot/point')!.values.map(Number),expected,'authored '+kind+' coordinates');}
  console.log('[native-grammar] all seven authored coordinate frames matched independent D3 scales and coordinate oracles');
  const scaleBody: {raw:string}[] = [];
  for (const points of fixture.fontPoints) scaleBody.push({ raw: `\\ExplSyntaxOn\\semio_viz_probe_values:nx{font/mm/${points}}{\\fp_eval:n{100*\\dim_to_fp:n{${points}pt}/\\dim_to_fp:n{100mm}}}\\semio_viz_probe_values:nx{font/mm-quantized/${points}}{\\fp_eval:n{\\dim_to_fp:n{${points}pt}/\\dim_to_fp:n{1mm}}}\\ExplSyntaxOff` });
  for (const c of [...fixture.scales, ...fixture.colorScales]) {
    const options = Object.entries(c.options ?? {}).map(([k,v])=>k+'={'+String(v)+'}').join(',');
    scaleBody.push({raw:'\\SemioVizScale{'+c.name+'}{'+c.kind+'}{'+c.domain.join(',')+'}{'+c.range.join(',')+'}['+options+']'});
    for (const [i,value] of c.values.entries()) {
      const color = fixture.colorScales.some(v=>v.name===c.name);
      scaleBody.push({raw:'\\ExplSyntaxOn\\semio_viz_scale_value:nnN{'+c.name+'}{'+value+'}\\l_tmpa_tl'+(color?'\\convertcolorspec{named}{\\l_tmpa_tl}{rgb}\\l_tmpb_tl\\semio_viz_probe_values:nV{scale/'+c.name+'/'+i+'}\\l_tmpb_tl':'\\semio_viz_probe_values:nV{scale/'+c.name+'/'+i+'}\\l_tmpa_tl')+'\\ExplSyntaxOff'});
    }
  }
  scaleBody.push({raw:'\\definecolor{alpha-fill}{HTML}{FF0000}\\definecolor{alpha-stroke}{HTML}{0000FF}\\SemioVizPaintAlpha{alpha-fill}{0.25}\\SemioVizPaintAlpha{alpha-stroke}{0.75}\\ExplSyntaxOn\\tl_set:Nn\\l_semio_viz_mark_fill_tl{alpha-fill}\\tl_set:Nn\\l_semio_viz_mark_stroke_tl{alpha-stroke}\\fp_set:Nn\\l_semio_viz_mark_opacity_fp{0.6}\\semio_viz_mark_options:\\semio_viz_probe_values:nx{paint/alpha}{\\fp_eval:n{\\l_semio_viz_mark_opacity_fp*\\semio_viz_color_alpha:n{\\l_semio_viz_mark_fill_tl}},\\fp_eval:n{\\l_semio_viz_mark_opacity_fp*\\semio_viz_color_alpha:n{\\l_semio_viz_mark_stroke_tl}},\\semio_viz_color_alpha:n{black}}\\semio_viz_scale_ticks:nN{explicit-ticks}\\l_tmpa_seq\\semio_viz_probe_values:nx{scale/ticks}{\\seq_use:Nn\\l_tmpa_seq{,}}\\semio_viz_scale_ticks:nN{time-iso}\\l_tmpa_seq\\seq_map_inline:Nn\\l_tmpa_seq{\\semio_viz_scale_epoch_set:nN{#1}\\l_tmpa_fp\\semio_viz_probe_values:nx{scale/time-ticks}{\\fp_to_decimal:N\\l_tmpa_fp}}\\ExplSyntaxOff'});
  const controls = await compileVizProbeDocument({case:'native-chart-grammar',scenario:'scale-controls',packages:['semio-viz-plot'],body:scaleBody},{execution,});
  for (const c of fixture.scales) {
    const range = c.options && 'reverse' in c.options && c.options.reverse ? [...c.range].reverse() : c.range;
    const scale = c.kind==='temporal'?scaleUtc(c.domain.map(v=>new Date(v)),range):scaleLinear(c.domain.map(Number),range);
    if(c.options && 'round' in c.options && c.options.round) scale.rangeRound(range);
    for(const [i,v] of c.values.entries()) equal(controls.find(r=>r.key==='scale/'+c.name+'/'+i)!.values.map(Number),[range.length===1?range[0]!:scale(c.kind==='temporal'?new Date(v):Number(v))],c.name);
  }
  for (const c of fixture.colorScales) {
    if (c.options && 'interpolator' in c.options && c.options.interpolator==='oklab') {
      const position = scaleLinear(c.domain, [0, 1]);
      for (const [i, value] of c.values.entries()) {
        const fraction = position(value), color = cssColor.colorToRgb(`color-mix(in oklab, ${c.range[0]} ${(1-fraction)*100}%, ${c.range[1]} ${fraction*100}%)`);
        equal(controls.find(r => r.key === 'scale/'+c.name+'/'+i)!.values.map(Number), color.slice(0, 3).map(channel => Number(channel)/255), c.name+' independent CSS OKLab', 1/255+.0001);
      }
      continue;
    }
    const range = c.options && 'reverse' in c.options && c.options.reverse ? [...c.range].reverse() : c.range;
    const interpolation = c.options && 'interpolator' in c.options && c.options.interpolator==='lab' ? interpolateLab : c.options && 'interpolator' in c.options && c.options.interpolator==='hcl' ? interpolateHcl : interpolateRgb;
    const scale = c.kind==='diverging'?scaleDiverging(c.domain as [number,number,number],t=>t<.5?interpolation(range[0]!,range[1]!)(t*2):interpolation(range[1]!,range[2]!)(t*2-1)):scaleLinear(c.domain.map(Number),range).interpolate(interpolation);
    for(const [i,v] of c.values.entries()){const color=rgb(scale(v));equal(controls.find(r=>r.key==='scale/'+c.name+'/'+i)!.values.map(Number),[color.r/255,color.g/255,color.b/255],c.name,1/255+.0001);}
  }
  equal(controls.find(r=>r.key==='paint/alpha')!.values.map(Number),[.15,.45,1],'independent CSS fill/stroke alpha');
  equal(controls.find(r=>r.key==='scale/ticks')!.values.map(Number),[0,3,10],'authored tick values');
  equal(controls.filter(r=>r.key==='scale/time-ticks').flatMap(r=>r.values.map(Number)),utcDay.range(new Date('2025-01-01'),new Date('2025-01-04')).map(Number),'ISOZ calendar ticks');
  for (const points of fixture.fontPoints) equal(controls.find(r => r.key === 'font/mm/' + points)!.values.map(Number), [scaleLinear([0, 72.27], [0, 25.4])(points)], 'TeX point font size to millimetres');
  console.log('[native-grammar] authored reverse, piecewise, Lab/HCL, ISOZ, ticks and independent paint alpha passed');

  const themeBody = [{ raw: `\\SemioVizTheme{default}\\ExplSyntaxOn\\semio_viz_probe_values:nx{theme/default-count}{\\semio_viz_theme_color_count:}\\ExplSyntaxOff\\SemioVizThemeSet[colors={${fixture.theme.colors.join(',')}}]` }];
  for (const slot of fixture.theme.slots) themeBody.push({ raw: `\\ExplSyntaxOn\\semio_viz_theme_color:nN{${slot}}\\l_tmpa_tl\\semio_viz_probe_values:nx{theme/protected/${slot}}{\\l_tmpa_tl}\\semio_viz_probe_values:nx{theme/expandable/${slot}}{\\semio_viz_theme_color:n{${slot}}}\\convertcolorspec{named}{\\l_tmpa_tl}{rgb}\\l_tmpb_tl\\semio_viz_probe_values:nV{theme/rgb/${slot}}\\l_tmpb_tl\\ExplSyntaxOff` });
  themeBody.push({ raw: '\\ExplSyntaxOn\\semio_viz_theme_color_count:N\\l_tmpa_int\\semio_viz_probe_values:nx{theme/count}{\\int_use:N\\l_tmpa_int,\\semio_viz_theme_color_count:}\\ExplSyntaxOff\\SemioVizTheme{default}\\ExplSyntaxOn\\semio_viz_probe_values:nx{theme/reset-count}{\\semio_viz_theme_color_count:}\\ExplSyntaxOff\\SemioVizThemeSet[palette=brand]\\ExplSyntaxOn\\semio_viz_probe_values:nx{theme/brand-count}{\\semio_viz_theme_color_count:}\\ExplSyntaxOff' });
  const themes = await compileVizProbeDocument({ case: 'native-chart-grammar', scenario: 'theme-controls', packages: ['semio-viz-theme'], body: themeBody }, {execution,});
  const palette = scaleOrdinal(fixture.theme.slots, fixture.theme.colors);
  for (const slot of fixture.theme.slots) {
    const color = palette(slot), expected = rgb(color);
    for (const accessor of ['protected', 'expandable']) if (themes.find(r => r.key === `theme/${accessor}/${slot}`)!.values[0] !== color) throw new Error(`theme ${accessor} slot ${slot} did not preserve authored palette`);
    equal(themes.find(r => r.key === `theme/rgb/${slot}`)!.values.map(Number), [expected.r / 255, expected.g / 255, expected.b / 255], 'authored theme palette');
  }
  equal(themes.find(r => r.key === 'theme/count')!.values.map(Number), [fixture.theme.colors.length, fixture.theme.colors.length], 'authored theme palette count');
  equal(themes.find(r => r.key === 'theme/reset-count')!.values.map(Number), themes.find(r => r.key === 'theme/default-count')!.values.map(Number), 'theme preset resets authored colors');
  equal(themes.find(r => r.key === 'theme/brand-count')!.values.map(Number), [fixture.theme.brandCount], 'brand palette retained');
  console.log('[native-grammar] custom palette colors, wrapping, expandable accessors, reset and brand controls matched D3');

  await compileNativeUnknownGrammar(execution);
  await compileNativeMissingGrammar(execution);
  const unknownBody: { raw: string }[] = [];
  for (const scale of fixture.unknownScales) {
    unknownBody.push({ raw: `\\SemioVizScale{${scale.name}}{${scale.kind}}{${scale.domain.join(',')}}{${scale.range.join(',')}}[unknown={${scale.unknown}}]` });
    for (const [index, input] of scale.inputs.entries()) unknownBody.push({ raw: `\\ExplSyntaxOn\\semio_viz_scale_value:nnN{${scale.name}}{${input}}\\l_tmpa_tl\\semio_viz_probe_values:nV{unknown/${scale.name}/${index}}\\l_tmpa_tl\\ExplSyntaxOff` });
  }
  const unknownRecords = await compileVizProbeDocument({ case: 'native-chart-grammar', scenario: 'scale-unknown', packages: ['semio-viz-scale'], body: unknownBody }, {execution,});
  for (const scale of fixture.unknownScales) {
    const reference = scale.kind === 'ordinal'
      ? (value: unknown) => scaleOrdinal<string|number,unknown>().domain(scale.domain).range(scale.range).unknown(scale.unknown)(value as string|number)
      : scale.kind === 'temporal'
      ? (value: unknown) => scaleUtc<unknown>().domain(scale.domain.map(value => new Date(value))).range(scale.range).unknown(scale.unknown)(value as number)
      : (value: unknown) => scaleLinear<unknown>().domain(scale.domain.map(Number)).range(scale.range).unknown(scale.unknown)(value as number);
    for (const [index, input] of scale.inputs.entries()) {
      const value = input === '\\SemioVizNull{}' ? null : input === 'NaN' ? Number.NaN : input;
      const expected = reference(value);
      if (String(unknownRecords.find(record => record.key === `unknown/${scale.name}/${index}`)!.values[0]) !== String(expected)) throw new Error(`${scale.name} fallback disagreed with D3`);
    }
  }
  console.log('[native-grammar] 12 authored numeric, color, ordinal and temporal unknown fallbacks matched D3');
  const missingRows = fixture.missingRows.map(row => `\\SemioVizRow{missing}{${row.map(value => value === null ? '\\SemioVizNull{}' : value).join(',')}}`).join('\n');
  const missingRecords = await compileVizProbeDocument({ case: 'native-chart-grammar', scenario: 'missing-values', packages: ['semio-viz-plot'], geometry: true, body: [{ raw: `\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{en}\\bool_set_true:N\\l_semio_viz_in_figure_bool\\fp_set:Nn\\l_semio_viz_width_fp{40}\\fp_set:Nn\\l_semio_viz_height_fp{50}\\ExplSyntaxOff\\begin{tikzpicture}[x=1mm,y=1mm]\\SemioVizTable{missing}{x,y}${missingRows}\\SemioVizScale{missing-x}{linear}{0,3}{0,30}\\SemioVizScale{missing-y}{linear}{0,4}{0,40}\\SemioVizProbeBegin{native-chart-grammar}{missing-gap}\\SemioVizPlot[data=missing,mark=line,x={column=x,scale=missing-x},y={column=y,scale=missing-y},stroke=black,x0=0,y0=0,x1=30,y1=40]\\SemioVizScale{fallback-x}{linear}{0,3}{0,30}[unknown=-5]\\SemioVizProbeBegin{native-chart-grammar}{fallback-mark}\\SemioVizPlot[data=missing,mark=point,x={column=x,scale=fallback-x},y={column=y,scale=missing-y},x0=0,y0=0,x1=30,y1=40]\\end{tikzpicture}` }] }, {execution,scenario: undefined});
  const missingLine = line<(number | null)[]>().defined(row => row[0] !== null).x(row => Number(row[0])*10).y(row => Number(row[1])*10)(fixture.missingRows)!;
  const expectedSegments = missingLine.split('M').filter(Boolean).map(segment => segment.match(/[-+]?\d*\.?\d+/g)!.map(Number));
  const actualSegments = missingRecords.filter(record => record.scenario === 'missing-gap' && record.key === 'geometry/mark/polyline');
  if (actualSegments.length !== expectedSegments.length) throw new Error('missing positions did not split the native line');
  actualSegments.forEach((record, index) => equal(record.values.filter(value => typeof value === 'number') as number[], expectedSegments[index]!, 'nullable native line segments'));
  const fallbackScale = scaleLinear([0,3],[0,30]).unknown(-5);
  equal(missingRecords.filter(record => record.scenario === 'fallback-mark' && record.key === 'geometry/plot/point').flatMap(record => record.values.map(Number)), fixture.missingRows.flatMap(row => [fallbackScale(row[0] as number), Number(row[1])*10]), 'nullable positions with authored unknown fallback');
  console.log('[native-grammar] nullable line gaps and authored fallback positions matched D3');
  const groupRows = fixture.network.groups;
  const groupValues = [...new Set(fixture.network.rows.flatMap(row => row.slice(0, 2) as string[]))].map(id => groupRows.find(row => row[0] === id)![1]);
  const networkRows = fixture.network.rows.map(row => `\\SemioVizRow{grammar-network}{${row.join(',')}}`).join('\n');
  const networks = await compileVizProbeDocument({ case: 'native-chart-grammar', scenario: 'network-family', packages: ['semio-viz-network-graph','semio-viz-catalog'], geometry: true, body: [{ raw: `\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{en}\\bool_set_true:N\\l_semio_viz_in_figure_bool\\fp_set:Nn\\l_semio_viz_width_fp{80}\\fp_set:Nn\\l_semio_viz_height_fp{40}\\ExplSyntaxOff\\begin{tikzpicture}[x=1mm,y=1mm]\\SemioVizChart{${fixture.network.kind}}\\SemioVizTable{grammar-network}{source,target,weight}${networkRows}\\SemioVizChart{${fixture.network.kind}}[data=grammar-network,order=${fixture.network.order}]\\ExplSyntaxOn\\int_step_inline:nn{\\l_semio_viz_network_n_int}{\\semio_viz_probe_values:xx{network/node/\\semio_viz_network_id:n{#1}}{\\fp_eval:n{\\semio_viz_network_f:nn{x}{#1}},\\fp_eval:n{\\semio_viz_network_f:nn{y}{#1}}}}\\seq_set_from_clist:Nn\\l_semio_viz_network_group_seq{${groupValues.join(',')}}\\semio_viz_network_order_build:n{group}\\seq_map_indexed_inline:Nn\\l_semio_viz_network_sorted_seq{\\semio_viz_probe_values:xx{network/group/#1}{\\semio_viz_network_id:n{#2}}}\\ExplSyntaxOff\\end{tikzpicture}` }] }, {execution,});
  const nodes = [...new Set(fixture.network.rows.flatMap(row => row.slice(0, 2) as string[]))].sort();
  const placement = scalePoint(nodes, [0, nodes.length - 1]);
  for (const node of nodes) equal(networks.find(r => r.key === 'network/node/' + node)!.values.map(Number), [placement(node)!, 0], 'named network family order');
  const groupOrder = sort(groupRows, row => row[1]).map(row => row[0]);
  if (JSON.stringify(networks.filter(r => r.key.startsWith('network/group/')).flatMap(r => r.values)) !== JSON.stringify(groupOrder)) throw new Error('named network group order did not match D3');
  console.log('[native-grammar] stock and authored graph family layouts preserved named node order and matched D3');
  const guide = fixture.guide;
  const guides = await compileVizProbeDocument({ case: 'native-chart-grammar', scenario: 'guide-minor', packages: ['semio-viz-guide'], geometry: true, body: [{ raw: `\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{en}\\bool_set_true:N\\l_semio_viz_in_figure_bool\\fp_set:Nn\\l_semio_viz_width_fp{116}\\fp_set:Nn\\l_semio_viz_height_fp{40}\\ExplSyntaxOff\\begin{tikzpicture}[x=1mm,y=1mm]\\SemioVizScale{minor-x}{linear}{${guide.domain.join(',')}}{${guide.range.join(',')}}\\SemioVizAxis[scale=minor-x,orient=bottom,at=0,tickValues={${guide.ticks.join(',')}},minor=true,minorTicks=${guide.subdivisions},minorSize=${guide.size},labels=false]\\end{tikzpicture}` }] }, {execution,});
  const minorScale = scaleLinear(guide.domain, guide.range);
  const minorValues = guide.ticks.slice(0, -1).flatMap((start, interval) => Array.from({ length: guide.subdivisions - 1 }, (_, step) => start + (step + 1) * (guide.ticks[interval + 1]! - start) / guide.subdivisions));
  equal(guides.filter(r => r.key === 'geometry/axis-tick-minor').flatMap(r => r.values.map(Number)), minorValues.flatMap(value => [minorScale(value), 0]), 'authored guide minor tick positions');
  console.log('[native-grammar] six authored minor tick positions matched independent D3 scale');

  await compileNativeDirectedGraphGrammar(execution);
  await compileNativeHiveControls(execution);
  await compileNativeGenomeWindowControls(execution);
  await compileNativeMusicWindowControls(execution);
  await compileNativeMeteogramWindowControls(execution);
  await compileNativeFontMetricsGrammar(execution);
  await compileNativeLegendGrammar(execution);
  await compileNativeFinancialGrammar(execution);
  await compileNativeFinancialStock(execution);
  const transforms = await nativeTransformGrammarChecks(execution);
  for (const check of transforms) equal(check.subject() as number[], check.oracle() as number[], check.name, check.tolerance);
  console.log(`[native-grammar] ${transforms.length} native transform columns matched independent D3`);
  const layouts = await nativeLayoutGrammarChecks(execution);
  for (const check of layouts) equal(check.subject() as number[], check.oracle() as number[], check.name, check.tolerance);
  console.log(`[native-grammar] ${layouts.length} native layout geometry checks matched independent D3 and dagre`);
  await compileNativeFamilyContainment(execution);
});}
