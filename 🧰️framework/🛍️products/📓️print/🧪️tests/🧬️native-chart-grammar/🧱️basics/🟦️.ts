/** 🧱️ Native grammar compilations, basics: unknown and missing values, directed graphs, font metrics, legends, spatial data, geo palettes, nice scales and the vector paint, base and flow admissions.
 *
 * @see ../🧰️grammar-kit/🟦️.ts — the equality check
 * @see ../🟦️.ts — the native grammar run that calls these compilations
 */
import { nativeControlPdfPaint } from "../🎨️paint/🟦️.ts";
import { PRINT_PDF_OPS_V1 as OPS, printPdfPageV1, printPdfNameV1 } from "../../../📦️artifacts/📖️pdf/🟦️.ts";
import type { CompiledPrintProbeV1 } from "../../../🔨️modules/🖨️tectonic-template-compilation/🟦️.ts";
import { withPrintScratchV1, type PrintExecutionV1 } from "../../../📦️artifacts/🎛️operation/🟦️.ts";
import { deepStrictEqual } from "node:assert";
import type { VizChartSnapshot } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { changeVizChartValue } from "../../../🧬️schema/🧬️mutations/🟦️.ts";
import { applyVizChartDiff, inverseVizChartDiff } from "../../../🧬️schema/🔀️diff/🟦️.ts";
import legendVectors from "../../🎬️render-scene/🔣️legend.json";
import { legendSpecifications } from "../../🎬️render-scene/🧭️legend/🟦️.ts";
import { inferVizChart } from "../../../🔨️modules/🏠️host/💡️inferences/🟦️.ts";
import { validateVizChartSpecification } from "@semio-tech/print-viz-inference";
import fontVectors from "../../../🔨️modules/🔤print-font-catalog/📏️metrics/🧪️tests/🔣️.json";
import { measurePrintSans } from "../../../🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { scaleLinear, scaleDiverging, scaleSqrt, scaleThreshold } from "d3-scale";
import { line } from "d3-shape";
import { path as d3Path } from "d3-path";
import { rgb, color as d3Color } from "d3-color";
import unknownRender from "../../🎨️authored-color-scales/❔️render.json";
import unknownVectors from "../../🎨️authored-color-scales/❔️unknown.json";
import { authoredUnknownScaleChecks } from "../../🎨️authored-color-scales/🟦️.ts";
import { interpolateRgb, interpolateNumber } from "d3-interpolate";
import { sort, max } from "d3-array";
import { polygonContains } from "d3-polygon";
import { geoEquirectangular } from "d3-geo";
import { compileVizProbeDocument, withCompiledVizProbeDocumentV1 } from "../../../🔨️modules/🧪️viz-probe/🟦️.ts";
import fixture from "../🔣️.json";
import chartSchema from "../../../🧬️schema/🔣️.json";
import nativeControlSchema from "../../../🧬️schema/✅️native-controls/🔣️.json";
import Ajv2020 from "ajv/dist/2020.js";
import arrowVectors from "../➡️arrows.json";
import { equal } from "../🧰️grammar-kit/🟦️.ts";

/** ❔️ Executes the same typed unknown-scale vectors through TeX and the independent D3 reference. */
export async function compileNativeUnknownGrammar(execution:PrintExecutionV1): Promise<void> {return withPrintScratchV1(execution,"native/unknown-grammar",async({directory:workDir,execution})=>{
  const references = await authoredUnknownScaleChecks();
  const body: { raw: string }[] = [];
  let paintIndex = 0;
  const typed = (value: unknown): string => value === undefined ? '\\SemioVizUndefined{}' : value === null ? '\\SemioVizNull{}' : typeof value === 'boolean' ? `\\SemioVizBoolean{${value}}` : typeof value === 'string' ? `\\SemioVizString{${value}}` : String(value);
  const paint = (value: unknown): string => {
    if (typeof value !== 'string' || !d3Color(value)) return typed(value);
    const color = rgb(value), alias = `grammar-unknown-paint-${paintIndex++}`;
    body.push({ raw: `\\definecolor{${alias}}{HTML}{${color.formatHex().slice(1)}}\\SemioVizPaintAlpha{${alias}}{${color.opacity}}` });
    return alias;
  };
  const expected = new Map<string, unknown>();
  for (const entry of unknownVectors.cases) {
    const spec = entry.spec;
    const values = references.find(check => check.name === 'unknown/' + entry.name)!.oracle() as unknown[];
    const categorical = ['ordinal','band','point'].includes(spec.kind);
    const domain = spec.domain.map(value => categorical ? typed(value) : String(value));
    const range = spec.range.map(value => typeof value === 'string' && d3Color(value) ? paint(value) : String(value));
    const options = Object.entries(spec.options).map(([key,value]) => `${key}={${key === 'unknown' ? paint(value) : String(value)}}`).join(',');
    body.push({ raw: `\\SemioVizScale{${entry.name}}{${spec.kind}}{${domain.join(',')}}{${range.join(',')}}[${options}]` });
    for (const [index, input] of entry.inputs.entries()) {
      const value = input.kind === 'undefined' ? '\\SemioVizUndefined{}' : input.kind === 'nan' || input.kind === 'invaliddate' ? '\\c_nan_fp' : typed('value' in input ? input.value : undefined);
      const key = `shared/${entry.name}/${index}`;
      expected.set(key, values[index]);
      const measurement = Array.isArray(values[index]) ? `\\convertcolorspec{named}{\\l_tmpa_tl}{rgb}\\l_tmpb_tl\\semio_viz_probe_values:nx{${key}}{\\clist_map_function:nN{\\l_tmpb_tl}\\use:n}` : `\\semio_viz_probe_values:nV{${key}}\\l_tmpa_tl`;
      body.push({ raw: `\\ExplSyntaxOn\\semio_viz_scale_value:nnN{${entry.name}}{${value}}\\l_tmpa_tl${Array.isArray(values[index]) ? `\\convertcolorspec{named}{\\l_tmpa_tl}{rgb}\\l_tmpb_tl\\semio_viz_probe_values:nx{${key}}{\\l_tmpb_tl,\\semio_viz_color_alpha:n{\\l_tmpa_tl}}` : measurement}\\ExplSyntaxOff` });
    }
  }
  const records = await compileVizProbeDocument({ case: 'native-chart-grammar', scenario: 'shared-unknown', packages: ['semio-viz-plot'], body }, {execution,});
  for (const [key, reference] of expected) {
    const record = records.find(item => item.key === key)!;
    const raw = record.values[0];
    const value = raw === '\\SemioVizUndefined {}' || raw === '\\SemioVizUndefined{}' ? { missing: true } : raw === '\\SemioVizNull {}' || raw === '\\SemioVizNull{}' ? null : typeof raw === 'string' && raw.startsWith('\\SemioVizBoolean') ? raw.includes('true') : typeof raw === 'string' && raw.startsWith('\\SemioVizString') ? raw.slice(raw.indexOf('{') + 1, -1) : raw === 'nan' || raw === '\\SemioVizNaN {}' || raw === '\\SemioVizNaN{}' ? { nan: true } : raw;
    if (Array.isArray(reference)) equal(record.values.map(Number).map((value,index) => index < 3 ? value*255 : value), reference as number[], key, 1.0001);
    else if (typeof reference === 'number') equal([Number(value)], [reference], key);
    else if (JSON.stringify(value) !== JSON.stringify(reference)) throw new Error(`${key}: actual=${JSON.stringify(value)}, expected=${JSON.stringify(reference)}`);
  }
  console.log(`[native-grammar] ${expected.size} shared typed unknown scale inputs matched independent D3`);
});}

/** \u{1f573}\uFE0F Checks shared renderer gap and nullable paint vectors against D3 coordinates. */
export async function compileNativeMissingGrammar(execution:PrintExecutionV1): Promise<void> {return withPrintScratchV1(execution,"native/missing-grammar",async({directory:workDir,execution})=>{
  const typed = (value: unknown): string => value === undefined ? '\\SemioVizUndefined{}' : value === null ? '\\SemioVizNull{}' : String(value);
  const rows = unknownRender.rows.map(row => '\\SemioVizRow{shared-missing}{' + ['x','y','paint'].map(key => typed((row as Record<string,unknown>)[key])).join(',') + '}').join('\n');
  const chunks = unknownRender.cases.map((entry,index) => {
    const spec = entry as typeof entry & {missingColumn?:boolean,paintFallback?:null,paintDefault?:boolean};
    const domain = spec.kind === 'diverging' ? '0,5,10' : '0,10';
    const range = spec.kind === 'diverging' ? '0,50,100' : '0,100';
    const fallback = 'unknown' in spec.options ? '[unknown=' + typed(spec.options.unknown) + ']' : '';
    const paint = 'paintFallback' in spec || spec.paintDefault ? ',fill={column=paint,scale=shared-paint},stroke={column=paint,scale=shared-paint}' : '';
    const paintScale = '\\SemioVizScale{shared-paint}{linear}{0,10}{blue,red}' + ('paintFallback' in spec ? '[unknown=\\SemioVizNull{}]' : '');
    return '\\SemioVizScale{shared-position}{' + spec.kind + '}{' + domain + '}{' + range + '}' + fallback + paintScale + '\\SemioVizProbeBegin{native-chart-grammar}{' + spec.name + '}\\SemioVizPlot[data=shared-missing,mark=' + spec.mark + ',x={column=' + (spec.missingColumn ? 'absent' : 'x') + ',scale=shared-position},y={column=y,scale=shared-y},x0=0,y0=0,x1=100,y1=50' + paint + ']';
  }).join('\n');
  const body = '\\ExplSyntaxOn\\tl_set:Nn\\l_semio_language_tl{en}\\bool_set_true:N\\l_semio_viz_in_figure_bool\\fp_set:Nn\\l_semio_viz_width_fp{100}\\fp_set:Nn\\l_semio_viz_height_fp{50}\\ExplSyntaxOff\\begin{tikzpicture}[x=1mm,y=1mm]\\SemioVizTable{shared-missing}{x,y,paint}' + rows + '\\SemioVizScale{shared-y}{linear}{0,5}{0,50}' + chunks + '\\end{tikzpicture}';
  const records = await compileVizProbeDocument({case:'native-chart-grammar',scenario:'shared-missing',packages:['semio-viz-plot'],geometry:true,body:[{raw:body}]},{execution,scenario:undefined});
  for (const entry of unknownRender.cases) {
    const spec = entry as typeof entry & {missingColumn?:boolean,paintFallback?:null,paintDefault?:boolean};
    const scale = spec.kind === 'diverging' ? scaleDiverging([0,5,10],t => 100*t) : scaleLinear([0,10],[0,100]);
    const positions = unknownRender.rows.flatMap(row => {
      if (spec.missingColumn) return [];
      const x = row.x == null ? ('unknown' in spec.options ? spec.options.unknown : undefined) : scale(row.x);
      return typeof x === 'number' ? [x,row.y*10] : [];
    });
    const found = records.filter(record => record.scenario === spec.name && record.key === 'geometry/plot/point');
    equal(found.flatMap(record => record.values.map(Number)),positions,'shared missing renderer/' + spec.name);
    if ('paintFallback' in spec || spec.paintDefault) {
      const styles = records.filter(record => record.scenario === spec.name && record.key === 'grammar/style');
      for (const [index,row] of unknownRender.rows.entries()) if (row.paint == null && styles[index] && styles[index]!.values[spec.mark === 'line' ? 1 : 0] !== 'none') throw new Error(spec.name + ': nullable fill was painted');
    }
  }
  console.log('[native-grammar] 10 shared missing-position and nullable-paint cases matched D3');
});}

/** ➡️ Checks graph arrow boundaries and angular titles against independent D3 geometry. */
export async function compileNativeDirectedGraphGrammar(execution:PrintExecutionV1, loopOnly = false): Promise<void> {return withPrintScratchV1(execution,"native/directed-graph-grammar",async({directory:workDir,execution})=>{
  const rows = loopOnly ? arrowVectors.rows.filter(row=>row[0]===row[1]) : arrowVectors.rows;
  const cases = loopOnly ? arrowVectors.cases.filter(entry=>["circle-straight","degree-curved","box-curved"].includes(entry.name)) : arrowVectors.cases;
  const bodies = cases.map(entry => ({raw: '\\begin{VizFigure}[width=80,height=40]\\SemioVizTable{arrow-data}{source,target,weight}'+rows.map(row=>'\\SemioVizRow{arrow-data}{'+row.join(',')+'}').join('')+'\\SemioVizProbeBegin{native-chart-grammar}{'+entry.name+'}\\SemioVizRunFamily{graph}[data=arrow-data,source=source,target=target,weight=weight,layout=arc,order=name,directed=true,labels=false,shape='+entry.shape+',sizeBy='+entry.sizeBy+',curvature='+entry.curvature+']\\end{VizFigure}'}));
  bodies.push({raw:'\\ExplSyntaxOn\\cs_if_exist:NF\\semio_viz_guide_title_position:{\\cs_set_eq:NN\\semio_viz_test_title_draw:\\semio_viz_guide_title_draw:\\cs_set_protected:Npn\\semio_viz_guide_title_draw:{\\semio_viz_test_title_draw:\\semio_viz_probe_geometry:nx{axis-title}{\\fp_eval:n{\\l_semio_viz_guide_ax_fp+\\l_semio_viz_guide_a_fp*(\\l_semio_viz_guide_bx_fp-\\l_semio_viz_guide_ax_fp)+\\l_semio_viz_guide_nx_fp*(\\l_semio_viz_axis_tick_size_fp+\\l_semio_viz_axis_title_gap_fp)},\\fp_eval:n{\\l_semio_viz_guide_ay_fp+\\l_semio_viz_guide_a_fp*(\\l_semio_viz_guide_by_fp-\\l_semio_viz_guide_ay_fp)+\\l_semio_viz_guide_ny_fp*(\\l_semio_viz_axis_tick_size_fp+\\l_semio_viz_axis_title_gap_fp)}}}}\\ExplSyntaxOff'});
  const a = arrowVectors.angularTitle;
  for(const anchor of loopOnly ? [] : a.anchors)bodies.push({raw:'\\begin{VizFigure}[width=100,height=80]\\SemioVizScale{angular-title}{linear}{0,10}{0,180}\\SemioVizProbeBegin{native-chart-grammar}{angular-title-'+anchor+'}\\SemioVizAxis[scale=angular-title,orient=angular,center={'+a.center.join(',')+'},outerRadius='+a.radius+',startAngle='+a.startAngle+',endAngle='+a.endAngle+',tickValues={0,5,10},tickSize='+a.tickSize+',titleGap='+a.titleGap+',title=Angular,titleAnchor='+anchor+',domainLine=false]\\end{VizFigure}'});
  const records = await compileVizProbeDocument({case:'native-chart-grammar',scenario:'directed-graph',documentClass:'semio',documentClassOptions:'type=paper,language=en',packages:['semio-viz'],geometry:true,preamble:['\\title{Directed Graph Grammar}','\\author{Semio}','\\date{}','\\errorcontextlines=200'],body:bodies},{execution,scenario:undefined});
  const failures:string[]=[];
  const ids=sort([...new Set(rows.flatMap(row=>[String(row[0]),String(row[1])]))]);
  const clearance=.75*25.4/72.27/2;
  const distance=(dx:number,dy:number,r:number,shape:string):number=>{
    if(shape==='box'){
      const corner=Math.min(.4,r),qx=Math.abs(dx)-(2.1*r-corner),qy=Math.abs(dy)-(r-corner);
      return Math.hypot(Math.max(qx,0),Math.max(qy,0))+Math.min(Math.max(qx,qy),0)-corner;
    }
    if(shape!=='square'&&shape!=='diamond')return Math.hypot(dx,dy)-r;
    const polygon:[number,number][]=shape==='square'?[[-r,-r],[r,-r],[r,r],[-r,r]]:[[-1.3*r,0],[0,-1.3*r],[1.3*r,0],[0,1.3*r]];
    const nearest=Math.min(...polygon.map(([x,y],i)=>{
      const [bx,by]=polygon[(i+1)%polygon.length]!,vx=bx-x,vy=by-y;
      const t=scaleLinear([0,vx*vx+vy*vy],[0,1]).clamp(true)((dx-x)*vx+(dy-y)*vy);
      return Math.hypot(dx-x-t*vx,dy-y-t*vy);
    }));
    return polygonContains(polygon,[dx,dy])?-nearest:nearest;
  };
  for(const entry of cases){
    const own=records.filter(record=>record.scenario===entry.name);
    const nodes=own.filter(record=>record.key==='geometry/node');
    const links=own.filter(record=>['geometry/link','geometry/curve','geometry/directed-curve','geometry/loop','geometry/directed-loop'].includes(record.key));
    if(nodes.length!==ids.length||links.length!==rows.length)throw Error(entry.name+': graph probe inventory differs');
    const pairs=new Map<string,number>();
    rows.forEach((row,index)=>{
      const source=nodes[ids.indexOf(String(row[0]))]!.values.map(Number),target=nodes[ids.indexOf(String(row[1]))]!.values.map(Number);
      const [sx,sy]=source,[tx,ty,r]=target;
      if(row[0]===row[1]){
        const cy=ty!+1.4*r!,radius=1.1*r!;
        let lo=180,hi=270;for(let k=0;k<64;k++){const angle=(lo+hi)/2*Math.PI/180;if(distance(radius*Math.cos(angle),cy+radius*Math.sin(angle)-ty!,r!,entry.shape)>clearance)lo=(lo+hi)/2;else hi=(lo+hi)/2;}
        const angle=(lo+hi)/2;
        try{equal(links[index]!.values.slice(0,5).map(Number),[tx!,cy,radius,540-angle,360+angle],entry.name+" directed self-loop",5e-4);}catch(error){failures.push(String(error));}
        return;
      }
      const key=[String(row[0]),String(row[1])].sort().join('/'),slot=pairs.get(key)??0;pairs.set(key,slot+1);
      const bend=entry.curvature+.16*Math.ceil(slot/2)*(slot%2?1:-1);
      const cx=(sx!+tx!)/2-bend*(ty!-sy!),cy=(sy!+ty!)/2+bend*(tx!-sx!);
      const point=(t:number):[number,number]=>[0,1].map(axis=>{const start=source[axis]!,end=target[axis]!,control=[cx,cy][axis]!;if(Math.abs(bend)<1e-9)return interpolateNumber(start,end)(t);const a=interpolateNumber(start,control)(t),c=interpolateNumber(control,end)(t);return interpolateNumber(interpolateNumber(a,control)(t),interpolateNumber(control,c)(t))(t);})as[number,number];
      let lo=0,hi=1;for(let k=0;k<64;k++){const t=(lo+hi)/2,[x,y]=point(t);if(distance(x-tx!,y-ty!,r!,entry.shape)>clearance)lo=t;else hi=t;}
      const t=(lo+hi)/2,tip=point(t),record=links[index]!;
      const p=d3Path();p.moveTo(sx!,sy!);
      if(Math.abs(bend)<1e-9)p.lineTo(...tip);
      else p.bezierCurveTo(sx!+t*(cx-sx!),sy!+t*(cy-sy!),(1-t)**2*sx!+(2*t-t*t)*cx,(1-t)**2*sy!+(2*t-t*t)*cy,...tip);
      const expected=String(p).match(/[-+]?(?:\d*\.?\d+)(?:e[-+]?\d+)?/gi)!.map(Number);
      const actual=record.values.slice(0,Math.abs(bend)<1e-9?4:8).map(Number);
      try{equal(actual,expected,entry.name+'/edge-'+index,5e-4);}catch(error){failures.push(String(error));}
    });
  }
  (loopOnly ? [] : a.anchors).forEach(anchor=>{
    const t=anchor==='start'?0:anchor==='end'?1:.5,angle=scaleLinear([0,1],[a.startAngle,a.endAngle])(t)*Math.PI/180,r=a.radius+a.tickSize+a.titleGap;
    const record=records.find(record=>record.scenario==='angular-title-'+anchor&&record.key==='geometry/axis-title')!;
    try{equal(record.values.map(Number),[a.center[0]!+r*Math.cos(angle),a.center[1]!+r*Math.sin(angle)],'angular title '+anchor,1e-4);}catch(error){failures.push(String(error));}
  });
  if(failures.length)throw Error(failures.length+' directed graph/title mismatches:\n'+failures.join('\n'));
  console.log('[native-grammar] '+(cases.length*rows.length)+' directed graph boundaries including '+(cases.length*rows.filter(row=>row[0]===row[1]).length)+' loops and '+(loopOnly?0:a.anchors.length)+' angular title anchors matched D3 geometry');
});}


/** 🔤️ Measures authored SemioSans shaping with the actual native hbox and tracked font data. */
export async function compileNativeFontMetricsGrammar(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/font-metrics-grammar",async({directory:workDir,execution})=>{
  const bodies:{raw:string}[]=[{raw:"\\begin{VizFigure}[width=80,height=30]"}];
  for(const size of fontVectors.sizes){
    bodies.push({raw:"\\ExplSyntaxOn\\semio_viz_guide_custom_reset:\\keys_set:nn{semio/viz/legend}{labelSize="+size+"}\\ExplSyntaxOff"});
    fontVectors.cases.forEach((entry,index)=>bodies.push({raw:"\\ExplSyntaxOn\\semio_viz_legend_text_width:nN{"+entry.text+"}\\l_tmpa_fp\\semio_viz_probe_values:nx{font/advance/"+size+"/"+index+"}{\\fp_use:N\\l_tmpa_fp}\\ExplSyntaxOff"}));
  }
  bodies.push({raw:"\\end{VizFigure}"});
  const records=await compileVizProbeDocument({case:"native-chart-grammar",scenario:"font-metrics",documentClass:"semio",documentClassOptions:"type=paper,language=en",packages:["semio-viz"],preamble:["\\title{Font Metrics Grammar}","\\author{Semio}","\\date{}"],body:bodies},{execution,});
  const failures:string[]=[];
  for(const size of fontVectors.sizes)fontVectors.cases.forEach((entry,index)=>{
    try{equal(records.find(record=>record.key==="font/advance/"+size+"/"+index)!.values.map(Number),[measurePrintSans(entry.text,size)],"actual SemioSans "+entry.language+"/"+entry.text+"/"+size,1e-4);}catch(error){failures.push(String(error));}
  });
  if(failures.length)throw Error(failures.length+" actual font advance mismatches:\n"+failures.join("\n"));
  console.log("[native-grammar] "+(fontVectors.sizes.length*fontVectors.cases.length)+" actual SemioSans advances matched independent Canvas and tracked Latin shaping");
});}


/** 🏷️ Compiles the neutral legend controls and compares their native probes with canonical inference. */
export async function compileNativeLegendGrammar(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/legend-grammar",async({directory:workDir,execution})=>{
  const bodies:{raw:string}[]=[],specifications=legendSpecifications();
  for(const entry of specifications){
    const guide=entry.spec.guides![0]!,options={scale:guide.scale,...guide.options,...(guide.tickValues===undefined?{}:{tickValues:guide.tickValues.join(",")}),...(guide.title===undefined?{}:{title:guide.title.en}),...guide.options};
    const scales=legendVectors.scales.map(scale=>"\\SemioVizScale{"+scale.name+"}{"+scale.kind+"}{"+scale.domain.join(",")+"}{"+scale.range.join(",")+"}").join("");
    const keys=Object.entries(options).map(([key,value])=>key+"={"+String(value)+"}").join(",");
    bodies.push({raw:"\\begin{VizFigure}[width="+legendVectors.width+",height="+legendVectors.height+"]"+scales+"\\SemioVizProbeBegin{native-chart-grammar}{"+entry.id+"}\\SemioVizLegend["+keys+"]\\end{VizFigure}"});
  }
  for(const entry of specifications.filter(entry=>["horizontal-metrics","title","style"].includes(entry.id))){const result=await inferVizChart({chart:entry.spec});if(!result.complete)throw Error(JSON.stringify(result.diagnostics));bodies.push({raw:result.tikz});}
  await withCompiledVizProbeDocumentV1({case:"native-chart-grammar",scenario:"legends",documentClass:"semio",documentClassOptions:"type=paper,language=en",packages:["semio-viz"],geometry:true,preamble:["\\title{Legend Grammar}","\\author{Semio}","\\date{}","\\errorcontextlines=200"],body:bodies},{execution,scenario:undefined},async({records,probe})=>{
  let count=0;
  for(const entry of specifications){
    const result=await inferVizChart({chart:entry.spec});if(!result.complete||!result.plan)throw Error(JSON.stringify(result.diagnostics));
    const guide=entry.spec.guides![0]!,o=guide.options??{},channel=String(o.channel??""),kind=channel==="fill"?"swatch":channel==="stroke"||channel==="dash"?"line":channel==="shape"?"symbol":channel==="size"?"size":String(o.kind??"swatch"),own=records.filter(record=>record.scenario===entry.id),native=own.filter(record=>record.key==="geometry/legend-item"),swatch=Number(o.swatchSize??2.6),labelGap=Number(o.labelGap??1.2),items=result.plan.items;
    if(kind==="gradient"){
      const first=items.find(item=>item.kind==="rect")!;if(first.kind!=="rect")throw Error("missing gradient bar");
      equal(own.find(record=>record.key==="geometry/legend-gradient")!.values.map(Number),[first.x,legendVectors.height-first.y-first.height,Number(o.length??24),first.height],entry.id+" gradient bar",1e-4);count++;
      const labels=items.filter(item=>item.kind==="text").filter(item=>item.baseline==="top"&&item.anchor==="middle");
      if(labels.length!==native.length)throw Error(entry.id+" gradient item inventory differs");
      labels.forEach((item,index)=>{equal(native[index]!.values.map(Number),[item.x,legendVectors.height-item.y+.5],entry.id+" gradient tick "+index,1e-4);count++;});
    }else if(kind==="size"){
      const circles=items.filter(item=>item.kind==="circle");if(circles.length!==native.length)throw Error(entry.id+" size inventory differs");
      circles.forEach((item,index)=>{equal(native[index]!.values.map(Number),[item.cx,legendVectors.height-item.cy,item.r],entry.id+" size tick "+index,1e-4);count++;});
    }else{
      const labels=items.filter(item=>item.kind==="text").filter(item=>item.baseline==="middle");if(labels.length!==native.length)throw Error(entry.id+" categorical inventory differs");
      labels.forEach((item,index)=>{equal(native[index]!.values.slice(1).map(Number),[item.x-swatch-labelGap,legendVectors.height-item.y-swatch/2],entry.id+" categorical item "+index,1e-4);count++;});
    }
  }
  await verifyNativeLegendFonts(probe);
  console.log("[native-grammar] "+count+" native legend geometry records across "+specifications.length+" neutral controls matched canonical inference and independent D3/Canvas; 3 canonical font-selected TikZ plans compiled");
});});}

/** 🔤️ Checks actual native and emitted PDF text faces against the neutral tracked font roles. */
export async function verifyNativeLegendFonts(probe:CompiledPrintProbeV1):Promise<void>{
  const pdf=await probe.inspect(),expected=legendVectors.nativeFontFaces,failures:string[]=[];let titles=0,labels=0;
  {for(let page=1;page<=pdf.pages.length;page++){
    const proxy=printPdfPageV1(pdf,page);proxy.operators;
    for(const item of (proxy).items){
      if(!("str" in item))continue;
      const title=expected.titles.includes(item.str),label=expected.labels.includes(item.str);if(!title&&!label)continue;
      const face=proxy.fonts[item.fontName]?.name??"",wanted=title?expected.title:expected.label;
      if(title)titles++;else labels++;if(!face.endsWith("+"+wanted)&&face!==wanted)failures.push("page "+page+" "+item.str+": "+face+" expected "+wanted);
    }
  }}
  if(titles<4||labels<4)throw Error("native font role inventory is incomplete");
  if(failures.length)throw Error(failures.length+" native font role mismatches:\n"+failures.join("\n"));
  console.log("[native-grammar] "+titles+" title and "+labels+" label PDF text faces matched tracked native font roles");
}

/** 🗺️ Checks authored spatial table views through event replay, native registries and independent D3 outputs. */
export async function compileNativeSpatialDataGrammar(execution:PrintExecutionV1,rustDirectory?:string):Promise<void>{if(["PRINT_NATIVE_SPATIAL_SOURCE","PRINT_NATIVE_SPATIAL_TIKZ"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");if(rustDirectory!==undefined)throw Error("Actual retained native emission capability required");return withPrintScratchV1(execution,"native/spatial-data-grammar",async({directory:workDir,execution})=>{
  const control=fixture.nativeSpatialData;
  const emissionDirectory=rustDirectory??(process.env.PRINT_NATIVE_SPATIAL_SOURCE==="rust"?process.env.PRINT_NATIVE_SPATIAL_TIKZ:undefined);
  if(process.env.PRINT_NATIVE_SPATIAL_SOURCE==="rust"&&!emissionDirectory)throw Error("Rust spatial emission directory is required");
  for(const appearance of ["light","dark"] as const){
    const zero=await inferVizChart({chart:{...control.snapshot.chart,theme:{appearance},tables:[control.zeroExtent.table],presets:[control.zeroExtent.preset]}} as VizChartSnapshot);
    if(!zero.complete)throw Error("Constant geographic extent inference: "+JSON.stringify(zero.diagnostics));
    const records=await compileVizProbeDocument({case:"spatial-data-zero",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+appearance,packages:["semio-viz"],preamble:["\\title{Constant Geographic Extent}","\\author{Semio}","\\date{}"],body:[{raw:"\\SemioVizProbeBegin{spatial-data-zero}{constant}\n"+zero.tikz+"\\ExplSyntaxOn\\fp_set:Nn\\l_semio_viz_geo_vmin_fp{1}\\fp_set:Nn\\l_semio_viz_geo_vmax_fp{1}\\semio_viz_probe_values:nx{spatial-data/zero}{\\semio_viz_geo_norm:n{1}}\\ExplSyntaxOff"}]},{execution,scenario:undefined});
    equal(records.find(record=>record.key==="spatial-data/zero")!.values.map(Number),[scaleLinear([1,1],[0,1])(1)],"D3 constant geographic extent");
  }
  const ajv=new Ajv2020({strict:false});ajv.addSchema(chartSchema);const validate=ajv.compile({$ref:chartSchema.$id+"#/$defs/ChartSpecification"});
  for(const entry of control.cases){const chart={...control.snapshot.chart,tables:[entry.table],presets:[entry.preset]},owned=validateVizChartSpecification(chart),external=validate(chart);if(owned.length||!external)throw Error(entry.id+" spatial structure admission: "+JSON.stringify({owned,external:validate.errors}));}
  for(const entry of control.badCases){const source=control.cases.find(value=>value.id===entry.caseId)!;const base={chart:{...control.snapshot.chart,tables:[source.table],presets:[source.preset]}} as unknown as VizChartSnapshot;const outcome=changeVizChartValue(base,entry.mutation);if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" bad spatial mutation rejected prematurely");const replay=applyVizChartDiff(base,outcome.diff);if(replay.messages.length)throw Error(entry.id+" bad spatial replay rejected");const result=await inferVizChart(replay.snapshot);if(result.complete||result.tikz!==""||result.plan!==undefined||result.scene!==undefined)throw Error(entry.id+" invalid spatial view was published");}
  const before=JSON.stringify(control);
  for(const appearance of ["light","dark"] as const){
    const body:{raw:string}[]=[],snapshots=new Map<string,VizChartSnapshot>();
    for(const entry of control.cases)for(const language of ["en","de"] as const){
      const base={chart:{...control.snapshot.chart,language,theme:{appearance},tables:[entry.table],presets:[entry.preset]}} as unknown as VizChartSnapshot;
      const outcome=changeVizChartValue(base,entry.mutation);
      if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" spatial mutation rejected: "+JSON.stringify(outcome.messages));
      const replay=applyVizChartDiff(base,outcome.diff);if(replay.messages.length)throw Error(entry.id+" spatial replay rejected");
      const result=await inferVizChart(replay.snapshot);if(!result.complete)throw Error(entry.id+" structured table inference rejected: "+JSON.stringify(result.diagnostics));
      const emitted=emissionDirectory?readFileSync(join(emissionDirectory,`spatial-${entry.id}-${language}-${appearance}.tex`),"utf8"):result.tikz;
      if(!emitted.includes("\\"+entry.command+"{"+entry.table.name+"}"))throw Error(entry.id+" structured declaration was omitted");
      const name=entry.table.name,id=entry.id+"-"+language;snapshots.set(id,replay.snapshot);
      let probe="";
      if(entry.table.structure.kind==="geometry")probe="\\semio_viz_geom_map_inline:nn{"+name+"}{\\semio_viz_control_spatial_part:nnnn#1}";
      else if(entry.table.structure.kind==="grid")probe="\\tl_set:Nn\\l_semio_viz_geofld_grid_tl{"+name+"}\\semio_viz_geofld_load:\\semio_viz_probe_values:nx{spatial-data/grid}{\\int_use:N\\l_semio_viz_geofld_w_int,\\int_use:N\\l_semio_viz_geofld_h_int,\\seq_use:cn{g_semio_viz_grid_"+name+"_seq}{,}}\\seq_map_inline:cn{g_semio_viz_grid_"+name+"_seq}{\\semio_viz_probe_values:nx{spatial-data/grid-norm}{\\semio_viz_geo_norm:n{#1}}}";
      else if(entry.table.structure.kind==="points")probe="\\semio_viz_probe_values:nx{spatial-data/points}{\\seq_use:cn{g_semio_viz_points_"+name+"_seq}{;}}";
      else probe="\\semio_viz_vector_sample:nnn{"+name+"}{0}{0}\\semio_viz_probe_values:nx{spatial-data/vector}{\\fp_use:N\\l_semio_viz_vector_u_fp,\\fp_use:N\\l_semio_viz_vector_v_fp}";
      body.push({raw:"\\SemioVizProbeBegin{spatial-data}{"+id+"}\n"+emitted+"\\ExplSyntaxOn"+probe+"\\ExplSyntaxOff"});
    }
    await withCompiledVizProbeDocumentV1({case:"spatial-data",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+appearance,packages:["semio-viz"],geometry:true,preamble:["\\title{Authored Spatial Data}","\\author{Semio}","\\date{}","\\ExplSyntaxOn\\cs_new_protected:Npn\\semio_viz_control_spatial_part:nnnn#1#2#3#4{\\semio_viz_probe_values:nx{spatial-data/part}{#1,#3,#4}}\\cs_new_eq:NN\\semio_viz_control_spatial_sample:nn\\semio_viz_georte_sample:nn\\cs_set_protected:Npn\\semio_viz_georte_sample:nn#1#2{\\semio_viz_control_spatial_sample:nn{#1}{#2}\\semio_viz_probe_values:nx{spatial-data/vector-render}{\\fp_eval:n{#1},\\fp_eval:n{#2},\\fp_use:N\\l_semio_viz_georte_fu_fp,\\fp_use:N\\l_semio_viz_georte_fv_fp}}\\cs_new_eq:NN\\semio_viz_control_spatial_rk:\\semio_viz_georte_rk_step:\\cs_set_protected:Npn\\semio_viz_georte_rk_step:{\\semio_viz_control_spatial_rk:\\semio_viz_probe_values:nx{spatial-data/vector-rk}{\\fp_use:N\\l_semio_viz_georte_sx_fp,\\fp_use:N\\l_semio_viz_georte_sy_fp}}\\cs_new_eq:NN\\semio_viz_control_spatial_stream:nn\\semio_viz_georte_streamline:nn\\cs_set_protected:Npn\\semio_viz_georte_streamline:nn#1#2{\\semio_viz_control_spatial_stream:nn{#1}{#2}\\semio_viz_probe_values:nx{spatial-data/vector-path}{\\seq_use:Nn\\l_tmpa_seq{;}}}\\ExplSyntaxOff"],body},{execution,scenario:undefined},async({records:records,probe})=>{
    for(const entry of control.cases)for(const language of ["en","de"] as const){
      const id=entry.id+"-"+language,own=records.filter(record=>record.scenario===id&&["spatial-data/part","spatial-data/grid","spatial-data/points","spatial-data/vector"].includes(record.key));
      if(JSON.stringify(own.map(record=>record.values.map(value=>String(value).replace(/\s/g,""))))!==JSON.stringify(entry.expected))throw Error(id+" native spatial values: "+JSON.stringify(own));
      const rows=snapshots.get(id)!.chart.tables![0]!.rows,structure=entry.table.structure;
      if(structure.kind==="vector-field"){
        const component=(key:string)=>interpolateNumber(interpolateNumber(Number(rows[0]![key]),Number(rows[1]![key]))(.5),interpolateNumber(Number(rows[2]![key]),Number(rows[3]![key]))(.5))(.5);
        equal(own[0]!.values.map(Number),[component("vx"),component("vy")],id+" D3 bilinear vector interpolation");
        const rendered=records.filter(record=>record.scenario===id&&record.key==="spatial-data/vector-render");
        if(entry.id==="vector-streamline"){
          const states=records.filter(record=>record.scenario===id&&record.key==="spatial-data/vector-rk"),unit=scaleLinear([-1,1],[0,1]).clamp(true),sample=(x:number,y:number)=>["vx","vy"].map(key=>interpolateNumber(interpolateNumber(Number(rows[0]![key]),Number(rows[1]![key]))(unit(x)),interpolateNumber(Number(rows[2]![key]),Number(rows[3]![key]))(unit(x)))(unit(y))),path:number[][]=[];
          if(rendered.length!==12||states.length!==3)throw Error(id+" RK4 sample/state inventory");let state=rendered[0]!.values.slice(0,2).map(Number);
          for(let step=0;step<3;step++){
            path.push([...state]);const[x,y]=state as[number,number],a=sample(x,y),bx=x+.1*a[0]!,by=y+.1*a[1]!,b=sample(bx,by),cx=x+.1*b[0]!,cy=y+.1*b[1]!,c=sample(cx,cy),dx=x+.2*c[0]!,dy=y+.2*c[1]!,d=sample(dx,dy),expected=[[x,y,...a],[bx,by,...b],[cx,cy,...c],[dx,dy,...d]];
            for(let stage=0;stage<4;stage++)equal(rendered[step*4+stage]!.values.map(Number),expected[stage]!,id+" D3 sampled RK4 stage "+step+"/"+stage,1e-6);
            state=[x+.2*(a[0]!+2*b[0]!+2*c[0]!+d[0]!)/6,y+.2*(a[1]!+2*b[1]!+2*c[1]!+d[1]!)/6].map(value=>Math.max(-1,Math.min(1,value)));equal(states[step]!.values.map(Number),state,id+" D3 sampled RK4 state "+step,1e-6);
          }
          const native=records.find(record=>record.scenario===id&&record.key==="spatial-data/vector-path")!.values.join(",").replace(/[()\s]/g,"").split(";").map(value=>value.split(",").map(Number));
          const page=path.map(point=>point.map(value=>unit(value)*4));for(const[index,point]of page.entries())equal(native[index]!,point,id+" named-field page placement "+index,1e-6);
        }else{if(rendered.length!==rows.length)throw Error(id+" rendered vector inventory");for(const[index,row]of rows.entries())equal(rendered[index]!.values.map(Number),[Number(row.px),Number(row.py),Number(row.vx),Number(row.vy)],id+" rendered named vector sample "+index);}
      }else if(structure.kind==="grid"){
        const normalized=records.filter(record=>record.scenario===id&&record.key==="spatial-data/grid-norm").flatMap(record=>record.values.map(Number));
        equal(normalized,[4,1,2,3].map(scaleLinear([1,4],[0,1])),id+" D3 scalar value normalization");
      }else{
        const native=own.flatMap(record=>(structure.kind==="geometry"?record.values.slice(2):record.values).join(",").split(";").map(value=>value.split(",").map(Number) as [number,number]));
        const authored=rows.map(row=>[Number(row[structure.kind==="geometry"?"longitude":"px"]),Number(row[structure.kind==="geometry"?"latitude":"py"])] as [number,number]);
        if(line().digits(12)(native)!==line().digits(12)(authored))throw Error(id+" D3 spatial path reconstruction");
        if(structure.kind==="geometry"){
          const projection=geoEquirectangular().fitSize([60,30],{type:"MultiPoint",coordinates:authored}),projected=authored.map(point=>projection(point)!);
          if(structure.shape==="point"){
            const symbols=records.filter(record=>record.scenario===id&&record.key==="geometry/geo/symbol"),values=rows.map(row=>Number(row[structure.value!])),normalize=scaleLinear([Math.min(...values),max(values)!],[0,1]),size=scaleSqrt([0,1],[0,2]);
            if(symbols.length!==rows.length)throw Error(id+" authored geographic symbol inventory");
            for(const[index,point]of projected.entries())equal(symbols[index]!.values.map(Number),[...point,.8+size(normalize(values[index]!)),values[index]!],id+" D3 authored geographic symbol "+index,1e-4);
          }else{
            const paths=records.filter(record=>record.scenario===id&&record.key==="geometry/path");if(paths.length!==1)throw Error(id+" authored projected path inventory");equal(paths[0]!.values.map(Number),projected.flat(),id+" D3 authored projected path",1e-4);
          }
        }
      }
    }
    const pdf=await probe.inspect();
    {for(let page=1;page<=pdf.pages.length;page++)await(printPdfPageV1(pdf,page)).operators;}
  });}
  if(JSON.stringify(control)!==before)throw Error("Spatial event replay mutated authored controls");
  console.log("[spatial-data] "+control.cases.length*4+" replayed spatial table views compiled across EN/de and light/dark with independent D3 output checks");
});}

/** 🎨️ Checks independent palette and integration mutations through actual full native colors and D3 ramps. */
export async function compileNativeGeoPaletteGrammar(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/geo-palette-grammar",async({directory:workDir,execution})=>{
  const control=fixture.nativeGeoPalette,body:{raw:string}[]=[],phase=process.env.PRINT_NATIVE_GEO_PALETTE_PHASE;
  if(phase==="legend")return compileNativeGeoPaletteLegend(execution);
  if(phase==="vector-paint")return compileNativeVectorPaintControls(execution);
  if(phase==="vector-admission")return compileNativeVectorBaseAdmission();
  if(phase==="flow-admission")return compileNativeGeoFlowAdmission();
  const before=JSON.stringify(control.snapshot);
  for(const entry of control.cases){
    let snapshot=control.snapshot as VizChartSnapshot;
    const mutations:{path:string[];value:number|string}[]=[{path:["presets","0","options","seeds"],value:1}];
    if(entry.paletteSteps!==undefined)mutations.push({path:["presets","0","options","paletteSteps"],value:entry.paletteSteps});
    if(entry.steps!==undefined)mutations.push({path:["presets","0","options","steps"],value:entry.steps});
    if(entry.palette!==undefined)mutations.push({path:["presets","0","options","palette"],value:entry.palette});
    if(entry.language!==snapshot.chart.language)mutations.push({path:["language"],value:entry.language});
    for(const mutation of mutations){
      const outcome=changeVizChartValue(snapshot,mutation);
      if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" palette mutation rejected: "+JSON.stringify(outcome.messages));
      const replay=applyVizChartDiff(snapshot,outcome.diff);
      if(replay.messages.length)throw Error(entry.id+" palette replay rejected");
      snapshot=replay.snapshot;
    }
    const result=await inferVizChart(snapshot);
    if(!result.complete)throw Error(entry.id+" palette inference failed: "+JSON.stringify(result.diagnostics));
    body.push({raw:"\\SemioVizProbeBegin{geo-palette}{"+entry.id+"}\n"+result.tikz});
  }
  if(JSON.stringify(control.snapshot)!==before)throw Error("Palette mutation changed authored base");
  const rampProbes=control.inputs.map((value,index)=>"\\semio_viz_probe_values:nx{geo/palette/pct/"+index+"}{\\semio_viz_geo_ramp_pct:n{"+value+"}}\\tl_set:Nx\\l_tmpa_tl{\\semio_viz_geo_ramp:n{"+value+"}}\\semio_viz_probe_values:nV{geo/palette/full/"+index+"}\\l_tmpa_tl\\convertcolorspec{named}{\\l_tmpa_tl}{rgb}\\l_tmpb_tl\\semio_viz_probe_values:nV{geo/palette/rgb/"+index+"}\\l_tmpb_tl").join("");
  const themeProbes=[0,1].map(slot=>"\\tl_set:Nx\\l_tmpa_tl{\\semio_viz_theme_color:n{"+slot+"}}\\semio_viz_probe_values:nV{geo/palette/theme/"+slot+"}\\l_tmpa_tl\\convertcolorspec{named}{\\l_tmpa_tl}{rgb}\\l_tmpb_tl\\semio_viz_probe_values:nV{geo/palette/theme-rgb/"+slot+"}\\l_tmpb_tl").join("")+"\\convertcolorspec{named}{semio-chrome-canvas}{rgb}\\l_tmpb_tl\\semio_viz_probe_values:nV{geo/palette/canvas}\\l_tmpb_tl";
  const hook="\\ExplSyntaxOn\\cs_new_eq:NN\\semio_viz_control_palette_streamline:nn\\semio_viz_georte_streamline:nn\\cs_set_protected:Npn\\semio_viz_georte_streamline:nn#1#2{\\semio_viz_control_palette_streamline:nn{#1}{#2}\\semio_viz_probe_values:nx{geo/palette/points}{\\seq_count:N\\l_tmpa_seq}"+rampProbes+themeProbes+"}\\ExplSyntaxOff";
  const records=await compileVizProbeDocument({case:"geo-palette",scenario:"controls",documentClass:"semio",documentClassOptions:"type=paper,language=en",packages:["semio-viz"],preamble:["\\title{Geographic Palette Controls}","\\author{Semio}","\\date{}",hook],body},{execution,scenario:undefined});
  const failures:string[]=[];
  for(const entry of control.cases){
    const count=entry.expectedPaletteSteps,position=scaleThreshold<number,number>().domain(Array.from({length:count-1},(_,index)=>(index+.5)/(count-1))).range(count===1?[.5]:Array.from({length:count},(_,index)=>index/(count-1)));
    const own=records.filter(record=>record.scenario===entry.id&&record.key.startsWith("geo/palette/")),one=(key:string)=>{const matches=own.filter(record=>record.key===key);if(matches.length!==1)throw Error(entry.id+" palette probe inventory "+key);return matches[0]!.values;};
    const theme=[0,1].map(slot=>one("geo/palette/theme/"+slot).join(",").replace(/\s/g,""));
    const colors=new Set<string>();
    try{
      equal(one("geo/palette/points").map(Number),[entry.expectedSteps],entry.id+" integrated native path points");
      const canvas=rgb(control.canvas);equal(one("geo/palette/canvas").map(Number),[canvas.r,canvas.g,canvas.b].map(value=>value/255),entry.id+" source-owned light canvas");
      for(const slot of [0,1]){const color=rgb(control.snapshot.chart.theme.palette[slot]!);equal(one("geo/palette/theme-rgb/"+slot).map(Number),[color.r,color.g,color.b].map(value=>value/255),entry.id+" authored palette slot "+slot);}
    }catch(error){failures.push(String(error));}
    for(const[index,value]of control.inputs.entries()){
      const q=position(value),diverging=entry.palette==="diverging",hue=diverging&&q<.5?1:0,mix=Math.round(scaleLinear([0,1],[18,100])(diverging?Math.abs(2*q-1):q));
      const full=one("geo/palette/full/"+index).join(",").replace(/\s/g,"");colors.add(full);
      const expected=rgb(interpolateRgb(control.canvas,control.snapshot.chart.theme.palette[hue]!)(mix/100));
      try{equal(one("geo/palette/pct/"+index).map(Number),[mix],entry.id+" D3 whole-position mix "+index);if(full!==theme[hue]+"!"+mix+"!semio-chrome-canvas")throw Error(entry.id+" full ramp "+index+": "+full+" expected hue"+hue+" mix"+mix);equal(one("geo/palette/rgb/"+index).map(Number),[expected.r,expected.g,expected.b].map(value=>value/255),entry.id+" D3 full ramp RGB "+index,3e-3);}catch(error){failures.push(String(error));}
    }
    if(colors.size!==count)failures.push(entry.id+" full palette class count "+colors.size+" expected "+count);
  }
  if(failures.length)throw Error(failures.length+" whole palette failures:\n"+failures.join("\n"));
  console.log("[geo-palette] "+control.cases.length+" mutation/inference controls, "+control.cases.length*control.inputs.length+" D3 full hue/mix/RGB checks, exact palette classes and native integration counts");
  if(phase!=="ramp"){await compileNativeGeoPaletteLegend(execution);await compileNativeVectorBaseAdmission();await compileNativeGeoFlowAdmission();await compileNativeVectorPaintControls(execution);}
});}

/** 🏷️ Compiles the actual one-level diverging choropleth legend in each authored language. */
async function compileNativeGeoPaletteLegend(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/geo-palette-legend",async({directory:workDir,execution})=>{
  const control=fixture.nativeGeoPalette,body:{raw:string}[]=[];
  for(const entry of control.legendCases){
    const before=JSON.stringify(control.snapshot);let snapshot=control.snapshot as VizChartSnapshot;
    for(const mutation of [{path:["presets"],value:[entry.preset]},{path:["language"],value:entry.language}]){
      const outcome=changeVizChartValue(snapshot,mutation);if(outcome.messages.length)throw Error(entry.id+" legend mutation "+JSON.stringify(outcome.messages));const replay=applyVizChartDiff(snapshot,outcome.diff);if(replay.messages.length)throw Error(entry.id+" legend replay "+JSON.stringify(replay.messages));snapshot=replay.snapshot;
    }
    const result=await inferVizChart(snapshot);if(!result.complete)throw Error(entry.id+" legend inference "+JSON.stringify(result.diagnostics));if(JSON.stringify(control.snapshot)!==before)throw Error(entry.id+" mutated palette snapshot");body.push({raw:"\\SemioVizProbeBegin{geo-palette-legend}{"+entry.id+"}\n"+result.tikz});
  }
  const hook="\\ExplSyntaxOn\\cs_new_eq:NN\\semio_viz_control_palette_legend:\\semio_viz_chor_legend:\\cs_set_protected:Npn\\semio_viz_chor_legend:{\\semio_viz_control_palette_legend:\\semio_viz_probe_values:nx{geo/palette/legend-count}{\\int_use:N\\l_semio_viz_geo_f_palette_steps_int}}\\ExplSyntaxOff";
  const records=await compileVizProbeDocument({case:"geo-palette-legend",scenario:"controls",documentClass:"semio",documentClassOptions:"type=paper,language=en",packages:["semio-viz"],preamble:["\\title{One-Level Diverging Legend}","\\author{Semio}","\\date{}",hook],body},{execution,scenario:undefined});
  for(const entry of control.legendCases){const own=records.filter(record=>record.scenario===entry.id&&record.key==="geo/palette/legend-count");if(own.length!==1)throw Error(entry.id+" one-level legend missing");equal(own[0]!.values.map(Number),[1],entry.id+" one-level legend");}
  console.log("[geo-palette] "+control.legendCases.length+" actual one-level diverging legends compiled in en/de");
});}

/** ✨️ Verifies authored nice switches and counts through mutations, inference and the native scale consumer. */
export async function compileNativeScaleNiceGrammar(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/scale-nice-grammar",async({directory:workDir,execution})=>{
  const control=fixture.nativeScaleNice,body:{raw:string}[]=[];
  const before=JSON.stringify(control.snapshot);
  for(const entry of control.cases){
    let snapshot=control.snapshot as VizChartSnapshot;
    const mutations:{path:string[];value:number|boolean|string|undefined}[]=[{path:["scales","0","options","nice"],value:entry.value}];
    if(entry.language!==snapshot.chart.language)mutations.push({path:["language"],value:entry.language});
    for(const mutation of mutations){
      const outcome=changeVizChartValue(snapshot,mutation);
      if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" nice mutation rejected: "+JSON.stringify(outcome.messages));
      const replay=applyVizChartDiff(snapshot,outcome.diff);
      if(replay.messages.length)throw Error(entry.id+" nice replay rejected");
      snapshot=replay.snapshot;
    }
    const result=await inferVizChart(snapshot);
    if(!result.complete)throw Error(entry.id+" nice inference failed: "+JSON.stringify(result.diagnostics));
    const values=control.inputs.map((value,index)=>"\\semio_viz_scale_value:nnN{nice-control}{"+value+"}\\l_tmpa_tl\\semio_viz_probe_values:nV{scale/nice/"+index+"}\\l_tmpa_tl").join("");
    const probes="\\ExplSyntaxOn\\semio_viz_probe_values:nx{scale/nice/domain}{\\clist_item:cn{g_semio_viz_scale_nice-control_domain_clist}{1},\\clist_item:cn{g_semio_viz_scale_nice-control_domain_clist}{-1}}"+values+"\\ExplSyntaxOff\n";
    if(result.tikz.split("\\end{VizFigure}").length!==2)throw Error(entry.id+" nice figure inventory");
    body.push({raw:"\\SemioVizProbeBegin{native-scale-options}{"+entry.id+"}\n"+result.tikz.replace("\\end{VizFigure}",probes+"\\end{VizFigure}")});
  }
  if(JSON.stringify(control.snapshot)!==before)throw Error("Nice mutation changed authored base");
  const records=await compileVizProbeDocument({case:"native-scale-options",scenario:"controls",documentClass:"semio",documentClassOptions:"type=paper,language=en",packages:["semio-viz"],preamble:["\\title{Native Scale Controls}","\\author{Semio}","\\date{}"],body},{execution,scenario:undefined});
  for(const entry of control.cases){
    const spec=control.snapshot.chart.scales[0]!,reference=scaleLinear(spec.domain,spec.range);
    if(entry.value!==undefined&&entry.value!==false)reference.nice(entry.value===true?10:entry.value);
    const own=records.filter(record=>record.scenario===entry.id);
    if(own.filter(record=>record.key.startsWith("scale/nice/")).length!==control.inputs.length+1)throw Error(entry.id+" native nice probe inventory");
    equal(own.find(record=>record.key==="scale/nice/domain")!.values.map(Number),reference.domain(),entry.id+" native nice domain");
    for(const [index,value] of control.inputs.entries())equal(own.find(record=>record.key==="scale/nice/"+index)!.values.map(Number),[reference(value)],entry.id+" native nice map");
  }
  console.log("[native-scale-options] "+control.cases.length+" mutation/inference controls and "+control.cases.length*(control.inputs.length+1)+" independent D3 native checks");
});}

/** 🎏️ Measures authored plane-vector paint controls through canonical replay and actual PDF operators. */
export async function compileNativeVectorPaintControls(execution:PrintExecutionV1):Promise<void>{if(["PRINT_NATIVE_VECTOR_PAINT_SOURCE"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/vector-paint-controls",async({directory:workDir,execution})=>{
  const neutral=fixture.nativeVectorPaintControls,c=neutral.vectors,before=JSON.stringify(c),failures:string[]=[],color=rgb(c.stroke),expected=[color.r,color.g,color.b].map(value=>value/255),cases=c.renders.flatMap(render=>c.opacities.flatMap(opacity=>c.languages.map(language=>({id:render+"-"+opacity+"-"+language,render,opacity,language}))));
  
  const source=process.env.PRINT_NATIVE_VECTOR_PAINT_SOURCE,themes=(["light","dark"]as const).filter(theme=>!source||existsSync(join(source,theme,"🧪️probe-out",theme+".pdf")));
  if(!themes.length)throw Error("Vector paint source has no theme PDF");
  for(const appearance of themes){
    const body:{raw:string}[]=[],directory=join(workDir,appearance);
    for(const entry of cases){
      const base={chart:{language:entry.language,theme:{appearance},width:c.canvas[0],height:c.canvas[1],margin:{top:0,right:0,bottom:0,left:0},layers:[],tables:[],presets:[{kind:"quiver-plot",options:{field:"source",render:entry.render,columns:2,rows:2,cell:5,arrow:1,seeds:2,steps:4,dt:.04,stroke:"#000000",lineWidth:c.lineWidth+"mm",opacity:entry.opacity}}]}} as unknown as VizChartSnapshot,original=JSON.stringify(base),outcome=changeVizChartValue(base,{path:["presets","0","options","stroke"],value:c.stroke});
      if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" vector paint mutation refused");const replay=applyVizChartDiff(base,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(base,outcome.diff));if(replay.messages.length||inverse.messages.length)throw Error(entry.id+" vector paint replay/inverse refused");deepStrictEqual(inverse.snapshot,base);if(JSON.stringify(base)!==original)throw Error(entry.id+" vector paint input changed");const result=await inferVizChart(replay.snapshot);if(!result.complete)throw Error(entry.id+" vector paint inference "+JSON.stringify(result.diagnostics));mkdirSync(join(directory,"emissions"),{recursive:true});await Bun.write(join(directory,"emissions",entry.id+".tex"),result.tikz);
      body.push({raw:"\\clearpage\\SemioVizProbeBegin{vector-paint}{"+entry.id+"}\\ExplSyntaxOn\\semio_viz_probe_values:nn{vector/paint}{1}\\ExplSyntaxOff\n"+result.tikz.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1\\draw[line width=.01mm] (0,0) rectangle (80,40);\\special{pdf:literal direct /SemioVizControl"+entry.id+" BMC}\n").replace("\\end{VizFigure}","\\special{pdf:literal direct EMC}\\end{VizFigure}")});
    }
    await withCompiledVizProbeDocumentV1({case:"vector-paint",scenario:appearance,geometry:true,documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+appearance,packages:["semio-viz"],preamble:["\\title{Plane Vector Paint Controls}","\\author{Semio}","\\date{}","\\errorcontextlines=200"],body},{execution,scenario:undefined},async({records:records,probe})=>{
    const pdf=await probe.inspect(),paints=new Map<string,Awaited<ReturnType<typeof nativeControlPdfPaint>>>();
    {for(let page=1;page<=pdf.pages.length;page++){const operators=await(printPdfPageV1(pdf,page)).operators;for(const entry of cases)if(operators.argsArray.some((args,index)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps].includes(operators.fnArray[index]!)&&printPdfNameV1(args?.[0])==="SemioVizControl"+entry.id))paints.set(entry.id,await nativeControlPdfPaint(operators,c.canvas,"SemioVizControl"+entry.id,probe.control));}}
    await Bun.write(join(directory,"owned-paint.json"),JSON.stringify(Object.fromEntries(paints),null,2));
    for(const entry of cases){try{
      const paint=paints.get(entry.id);if(!paint)throw Error(entry.id+" vector paint marker absent");if(entry.opacity===0){if(paint.paths.length||paint.glyphs.length||paint.images.length)throw Error(entry.id+" zero-opacity vector emitted visible ink");continue;}const paths=paint.paths.filter(path=>entry.render==="particle"?path.closed:!path.closed&&path.points.length>=2);if(!paths.length)throw Error(entry.id+" vector paint absent");
      if(entry.render==="quiver"){const heads=paint.paths.filter(path=>path.closed);if(heads.length!==4)throw Error(entry.id+" quiver head inventory differs");for(const head of heads){equal(head.fill,expected,entry.id+" actual D3 arrowhead color",.005);equal([head.fillOpacity],[entry.opacity],entry.id+" actual arrowhead alpha",.002);}}
      for(const path of paths){equal(entry.render==="particle"?path.fill:path.stroke,expected,entry.id+" actual D3 CSS paint",.005);equal([entry.render==="particle"?path.fillOpacity:path.opacity],[entry.opacity*(entry.render==="lic"?.55:entry.render==="particle"?.7:1)],entry.id+" actual vector alpha",.002);if(entry.render!=="particle")equal([path.width],[scaleLinear([0,1],[0,c.lineWidth])(1)],entry.id+" actual physical stroke width",.005);}
    }catch(error){failures.push(appearance+" "+String(error));}}
  });}
  if(JSON.stringify(c)!==before)throw Error("Vector paint neutral fixture changed");if(failures.length)throw Error("Native vector paint effects:\n"+failures.join("\n"));
});}

/** 🧭️ Rejects geographic map controls on a plane vector field through canonical inference and independent JSON Schema admission. */
export async function compileNativeVectorBaseAdmission():Promise<void>{
  const c=fixture.nativeVectorBaseAdmission,validate=new Ajv2020({strict:false}).compile(nativeControlSchema.$defs.VectorOptions),before=JSON.stringify(c),failures:string[]=[];
  for(const language of c.vectors.languages){
    const base={chart:{language,width:80,height:40,margin:{top:0,right:0,bottom:0,left:0},layers:[],tables:[],presets:[{kind:"quiver-plot",options:c.vectors.accepted}]}} as unknown as VizChartSnapshot,valid=await inferVizChart(base);
    if(!validate(c.vectors.accepted)||!valid.complete)throw Error(language+" admitted plane-vector options rejected");
    for(const entry of c.vectors.rejected){
      const original=JSON.stringify(base),outcome=changeVizChartValue(base,{path:["presets","0","options",entry.key],value:entry.value});if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.key+" vector applicability mutation refused");const replay=applyVizChartDiff(base,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(base,outcome.diff));if(replay.messages.length||inverse.messages.length)throw Error(entry.key+" vector applicability replay refused");deepStrictEqual(inverse.snapshot,base);if(JSON.stringify(base)!==original)throw Error("Vector applicability base changed");if(validate({...c.vectors.accepted,[entry.key]:entry.value}))throw Error("Independent JSON Schema admitted map-only vector option "+entry.key);const result=await inferVizChart(replay.snapshot);if(result.complete||result.tikz!==""||result.scene!==undefined||result.plan!==undefined)failures.push(language+" plane-vector map-only control published "+entry.key);
    }
  }
  if(JSON.stringify(c)!==before)throw Error("Vector applicability fixture changed");if(failures.length)throw Error("Vector applicability failures:\n"+failures.join("\n"));
}

/** 🌊️ Rejects route-only controls and invalid flow style selectors on a flow map through canonical inference and independent JSON Schema admission. */
export async function compileNativeGeoFlowAdmission():Promise<void>{
  const c=fixture.nativeGeoFlowAdmission,validate=new Ajv2020({strict:false}).compile(nativeControlSchema.$defs.GeoFlowOptions),before=JSON.stringify(c),failures:string[]=[];
  for(const language of c.vectors.languages){
    const base={chart:{language,width:80,height:40,margin:{top:0,right:0,bottom:0,left:0},layers:[],tables:[],presets:[{kind:"flow-map",options:c.vectors.accepted}]}} as unknown as VizChartSnapshot,valid=await inferVizChart(base);
    if(!validate(c.vectors.accepted)||!valid.complete)throw Error(language+" admitted geographic-flow options rejected");
    for(const entry of c.vectors.rejected){
      const original=JSON.stringify(base),outcome=changeVizChartValue(base,{path:["presets","0","options",entry.key],value:entry.value});if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.key+" vector applicability mutation refused");const replay=applyVizChartDiff(base,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(base,outcome.diff));if(replay.messages.length||inverse.messages.length)throw Error(entry.key+" vector applicability replay refused");deepStrictEqual(inverse.snapshot,base);if(JSON.stringify(base)!==original)throw Error("Flow applicability base changed");if(validate({...c.vectors.accepted,[entry.key]:entry.value}))throw Error("Independent JSON Schema admitted inapplicable vector option "+entry.key);const result=await inferVizChart(replay.snapshot);if(result.complete||result.tikz!==""||result.scene!==undefined||result.plan!==undefined)failures.push(language+" geographic-flow inapplicable control published "+entry.key);
    }
  }
  if(JSON.stringify(c)!==before)throw Error("Flow applicability fixture changed");if(failures.length)throw Error("Flow applicability failures:\n"+failures.join("\n"));
}
