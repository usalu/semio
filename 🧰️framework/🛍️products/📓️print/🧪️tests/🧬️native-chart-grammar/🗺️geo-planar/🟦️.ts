/** 🗺️ Native grammar compilations, geo planar: the planar geo grammar with its admission, guard and pdf paths, and the contour boundary.
 *
 * @see ../🧰️grammar-kit/🟦️.ts — the equality check
 * @see ../🟦️.ts — the native grammar run that calls these compilations
 */
import { PRINT_PDF_OPS_V1 as OPS, printPdfPageV1, printPdfTransformV1, printPdfApplyV1, printPdfInverseV1, printPdfArrayV1, printPdfVectorV1, printPdfNameV1, type PrintPdfPageV1 } from "../../../📦️artifacts/📖️pdf/🟦️.ts";
import { withPrintScratchV1, type PrintExecutionV1 } from "../../../📦️artifacts/🎛️operation/🟦️.ts";
import { deepStrictEqual } from "node:assert";
import type { VizChartSnapshot } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { changeVizChartValue } from "../../../🧬️schema/🧬️mutations/🟦️.ts";
import { applyVizChartDiff, inverseVizChartDiff } from "../../../🧬️schema/🔀️diff/🟦️.ts";
import { inferVizChart } from "../../../🔨️modules/🏠️host/💡️inferences/🟦️.ts";
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { scaleLinear } from "d3-scale";
import { sum } from "d3-array";
import { hexbin } from "d3-hexbin";
import { contours } from "d3-contour";
import { parseProbeJsonLines, compileVizProbeDocument, inspectVizProbeDocumentRefusalV1, withCompiledVizProbeDocumentV1 } from "../../../🔨️modules/🧪️viz-probe/🟦️.ts";
import fixture from "../🔣️.json";
import chartSchema from "../../../🧬️schema/🔣️.json";
import Ajv2020 from "ajv/dist/2020.js";
import { equal } from "../🧰️grammar-kit/🟦️.ts";

/** 🐝️ Checks planar binning and KDE separately from fitted native rendering and authored paint controls. */
export async function compileNativeGeoPlanarGrammar(execution:PrintExecutionV1,rustDirectory?:string):Promise<void>{if(["PRINT_NATIVE_GEO_PLANAR_REPLAY","PRINT_NATIVE_GEO_PLANAR_SOURCE","PRINT_NATIVE_GEO_PLANAR_TIKZ"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");if(rustDirectory!==undefined)throw Error("Actual retained native emission capability required");return withPrintScratchV1(execution,"native/geo-planar-grammar",async({directory:workDir,execution})=>{
  if(process.env.PRINT_NATIVE_GEO_PLANAR_PHASE==="guard"){await compileNativeGeoPlanarGuard(execution);return;}
  if(process.env.PRINT_NATIVE_GEO_PLANAR_PHASE==="admission"){await compileNativeGeoPlanarAdmission(execution);return;}
  if(process.env.PRINT_NATIVE_GEO_PLANAR_REPLAY)workDir=process.env.PRINT_NATIVE_GEO_PLANAR_REPLAY;
  await compileNativeGeoPlanarAdmission(execution);
  const emissionDirectory=rustDirectory??(process.env.PRINT_NATIVE_GEO_PLANAR_SOURCE==="rust"?process.env.PRINT_NATIVE_GEO_PLANAR_TIKZ:undefined);
  if(process.env.PRINT_NATIVE_GEO_PLANAR_SOURCE==="rust"&&!emissionDirectory)throw Error("Rust planar emission directory is required");
  const control=fixture.nativeGeoPlanar,before=JSON.stringify(control),failures:string[]=[];
  const raw=(entry:typeof control.cases[number])=>{
    const rawBins=hexbin<[number,number]>().radius(entry.radius)(entry.points as [number,number][]),vertices=rawBins.map(bin=>[30,90,150,210,270,330].map(angle=>[bin.x+entry.hexScale*entry.radius*Math.cos(angle*Math.PI/180),bin.y+entry.hexScale*entry.radius*Math.sin(angle*Math.PI/180)]));
    const values=Array.from({length:entry.gridWidth*entry.gridHeight},(_,index)=>sum(entry.points,point=>Math.exp(-((entry.gridOrigin[0]!+index%entry.gridWidth*entry.gridCell-point[0]!)**2+(entry.gridOrigin[1]!+Math.floor(index/entry.gridWidth)*entry.gridCell-point[1]!)**2)/(2*entry.bandwidth**2))/(2*Math.PI*entry.bandwidth**2)));
    const thresholds=entry.thresholds.length?entry.thresholds:Array.from({length:entry.levels},(_,index)=>Math.min(...values)+(index+1)/(entry.levels+1)*(Math.max(...values)-Math.min(...values))),rings=contours().size([entry.gridWidth,entry.gridHeight]).thresholds(thresholds)(values).flatMap(shape=>shape.coordinates.flatMap(polygon=>polygon.map(ring=>ring.map(point=>[entry.gridOrigin[0]!+(point[0]!-.5)*entry.gridCell,entry.gridOrigin[1]!+(point[1]!-.5)*entry.gridCell]))));
    return{rawBins,vertices,values,rings};
  };
  const bounds=(points:number[][])=>[Math.min(...points.map(point=>point[0]!)),Math.min(...points.map(point=>point[1]!)),Math.max(...points.map(point=>point[0]!)),Math.max(...points.map(point=>point[1]!))];
  const references=control.cases.map((_,index)=>[100+index,90+index]);
  for(const appearance of ["light","dark"] as const){
    const body:{raw:string}[]=[],expected:{entry:typeof control.cases[number];paths:number[][][];bounds:number[]}[]=[];
    for(const[index,entry]of control.cases.entries()){
      const {vertices,rings}=raw(entry);
      const base={chart:{...control.snapshot.chart,language:entry.language,theme:{appearance},tables:[{...control.table,rows:entry.points.map(point=>({x:point[0]!,y:point[1]!}))}],presets:[{kind:entry.kind,options:{points:control.table.name,radius:entry.radius,bandwidth:entry.bandwidth,gridWidth:16,gridHeight:11,gridOrigin:"0,0",gridCell:5,hexScale:.92,heatOpacity:.3,thresholds:"",levels:entry.levels,width:entry.width,height:entry.height,opacity:1,lineWidth:".1mm"}}]}} as unknown as VizChartSnapshot;
      let current=base;
      for(const [key,value]of Object.entries({gridWidth:entry.gridWidth,gridHeight:entry.gridHeight,gridOrigin:entry.gridOrigin.join(","),gridCell:entry.gridCell,hexScale:entry.hexScale,heatOpacity:entry.heatOpacity,thresholds:entry.thresholds.join(","),opacity:entry.opacity,lineWidth:entry.lineWidth}).filter(([key,value])=>base.chart.presets![0]!.options?.[key]!==value)){const mutation={path:["presets","0","options",key],value};
        const outcome=changeVizChartValue(current,mutation);if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" planar mutation rejected");const replay=applyVizChartDiff(current,outcome.diff);if(replay.messages.length)throw Error(entry.id+" planar replay rejected");const inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(current,outcome.diff));if(inverse.messages.length)throw Error(entry.id+" planar inverse rejected");deepStrictEqual(inverse.snapshot,current);current=replay.snapshot;
      }
      const result=await inferVizChart(current);if(!result.complete)throw Error(entry.id+" planar inference rejected: "+JSON.stringify(result.diagnostics));
      const paths=entry.render==="hex"?vertices:rings,extent=paths.length?bounds(paths.flat()):[0,0,0,0],scale=paths.length?Math.min((entry.width-Number.parseFloat(entry.lineWidth))/(extent[2]!-extent[0]!),(entry.height-Number.parseFloat(entry.lineWidth))/(extent[3]!-extent[1]!)):0,x=scaleLinear([extent[0]!,extent[2]!],[(entry.width-scale*(extent[2]!-extent[0]!))/2,(entry.width+scale*(extent[2]!-extent[0]!))/2]),y=scaleLinear([extent[1]!,extent[3]!],[(entry.height-scale*(extent[3]!-extent[1]!))/2,(entry.height+scale*(extent[3]!-extent[1]!))/2]);
      const fitted=paths.map(path=>path.map(point=>[x(point[0]!),y(point[1]!)]));expected.push({entry,paths:fitted,bounds:bounds(fitted.flat())});
      const marker=`\\draw[line width=.01mm] (0,0) rectangle (${references[index]![0]},${references[index]![1]});\\special{pdf:literal direct /SemioVizPlanar${entry.id} BMC}`;
      const emitted=emissionDirectory?readFileSync(join(emissionDirectory,`geo-planar-${entry.id}-${appearance}.tex`),"utf8"):result.tikz;
      mkdirSync(join(workDir,"emissions"),{recursive:true});await Bun.write(join(workDir,"emissions",`geo-planar-${entry.id}-${appearance}.tex`),emitted);
      body.push({raw:"\\SemioVizProbeBegin{geo-planar}{"+entry.id+"}\n"+emitted.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1"+marker+"\n").replace("\\end{VizFigure}","\\special{pdf:literal direct EMC}\\end{VizFigure}")});
    }
    await withCompiledVizProbeDocumentV1({case:"geo-planar",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+appearance,packages:["semio-viz"],preamble:["\\title{Planar Geographic Controls}","\\author{Semio}","\\date{}","\\ExplSyntaxOn\\cs_new_protected:Npn\\semio_viz_control_planar_bin:nnn#1#2#3{\\semio_viz_probe_values:nx{geo-planar/bin}{#2,#3,\\semio_viz_spatial_bin_count:n{#1}}}\\cs_new_eq:NN\\semio_viz_control_planar_hexes:\\semio_viz_geosym_hexes:\\cs_set_protected:Npn\\semio_viz_geosym_hexes:{\\semio_viz_control_planar_hexes:\\seq_map_inline:Nn\\g_semio_viz_spatial_bins_seq{\\semio_viz_control_planar_bin:nnn##1}}\\cs_new_eq:NN\\semio_viz_control_planar_density:\\semio_viz_geosym_density:\\cs_set_protected:Npn\\semio_viz_geosym_density:{\\semio_viz_control_planar_density:\\semio_viz_grid_require:n{planar-points-density}\\semio_viz_probe_values:nx{geo-planar/grid}{\\seq_use:cn{g_semio_viz_grid_\\l_semio_viz_store_name_tl _seq}{,}}}\\ExplSyntaxOff"],body},{execution,scenario:undefined},async({records:records,probe})=>{
    for(const entry of control.cases.filter(entry=>entry.render==="hex")){
      const actual=records.filter(record=>record.scenario===entry.id&&record.key==="geo-planar/bin").map(record=>record.values.map(Number)).sort((a,b)=>a[0]!-b[0]!||a[1]!-b[1]!);
      const bins=raw(entry).rawBins.map(bin=>[bin.x,bin.y,bin.length]).sort((a,b)=>a[0]!-b[0]!||a[1]!-b[1]!);equal(actual.flat(),bins.flat(),entry.id+" raw D3 hexbin geometry");
    }
    for(const entry of control.cases.filter(entry=>entry.render!=="hex"))equal(records.find(record=>record.scenario===entry.id&&record.key==="geo-planar/grid")!.values.map(Number),raw(entry).values,entry.id+" unchanged raw Gaussian KDE grid",1e-7);
    const pdf=await probe.inspect();
    const paths=new Map<number,{points:number[][];width:number;opacity:number;fillOpacity:number;join:number}[]>();
    {for(let page=1;page<=pdf.pages.length;page++)nativeGeoPlanarPdfPaths(await(printPdfPageV1(pdf,page)).operators,references,paths,control.cases.map(entry=>"SemioVizPlanar"+entry.id));}
    await Bun.write(join(workDir,appearance,"owned-planar-paths.json"),JSON.stringify(Object.fromEntries(paths),null,2));
    for(const[index,reference]of expected.entries()){
      const actual=paths.get(index)??[];
      try{if(actual.length!==reference.paths.length)throw Error(reference.entry.id+" PDF path count "+actual.length+" expected "+reference.paths.length);if(reference.paths.length)equal(bounds(actual.flatMap(path=>path.points)),reference.bounds,reference.entry.id+" fitted PDF frame",.02);const unique=(points:number[][])=>[...new Map(points.map(point=>[point.map(value=>value.toFixed(4)).join(","),point])).values()].sort((a,b)=>a[0]!-b[0]!||a[1]!-b[1]!);const unmatched=reference.paths.flatMap(unique),observed=actual.flatMap(path=>unique(path.points));if(unmatched.length!==observed.length)throw Error(reference.entry.id+" complete D3 vertex inventory "+observed.length+" expected "+unmatched.length);for(const point of observed){const match=unmatched.findIndex(expected=>Math.abs(point[0]!-expected[0]!)<=.02&&Math.abs(point[1]!-expected[1]!)<=.02);if(match<0)throw Error(reference.entry.id+" unmatched fitted D3 vertex "+JSON.stringify(point));unmatched.splice(match,1);}}catch(error){failures.push(appearance+" "+String(error));}
      for(const path of actual){try{equal([path.width,path.opacity,path.join],[Number.parseFloat(reference.entry.lineWidth),reference.entry.opacity,1],reference.entry.id+" actual lineWidth/opacity/roundjoin",.002);if(reference.entry.render!=="density")equal([path.fillOpacity],[reference.entry.render==="heat"?reference.entry.heatOpacity*reference.entry.opacity:reference.entry.opacity],reference.entry.id+" actual fill opacity",.002);}catch(error){failures.push(appearance+" "+String(error));}}
    }
  });}
  if(JSON.stringify(control)!==before)throw Error("Planar controls mutated authored input");
  if(failures.length)throw Error("Planar frame/paint failures:\n"+failures.join("\n"));
  console.log("[DEBUG] Geo planar "+control.cases.length*2+" actual EN/de light/dark controls matched raw D3 hexbin and fitted contour/PDF frame/paint outputs");
});}

/** 📐️ Reads straight closed PDF paths relative to declared per-case physical canvas markers. */
function nativeGeoPlanarPdfPaths(operators:PrintPdfPageV1["operators"],references:number[][],result:Map<number,{points:number[][];width:number;opacity:number;fillOpacity:number;join:number}[]>,markers:string[]):void{
  let matrix=[1,0,0,1,0,0],width=1,opacity=1,fillOpacity=1,lineJoin=0,reference:number[]|undefined,owner:number|undefined;
  const stack:{matrix:number[];width:number;opacity:number;fillOpacity:number;join:number}[]=[],content:string[]=[],mm=72/25.4;
  for(const[index,id]of operators.fnArray.entries()){
    const args=printPdfArrayV1(operators.argsArray[index]);
    if(id===OPS.beginMarkedContent||id===OPS.beginMarkedContentProps)content.push(printPdfNameV1(args?.[0]));
    else if(id===OPS.endMarkedContent){if(!content.length)throw Error("Planar PDF ownership stack underflow");content.pop();}
    else if(id===OPS.save||id===OPS.paintFormXObjectBegin){stack.push({matrix:[...matrix],width,opacity,fillOpacity,join:lineJoin});if(id===OPS.paintFormXObjectBegin&&args[0])matrix=printPdfTransformV1(matrix,printPdfVectorV1(args[0],6));}
    else if(id===OPS.restore||id===OPS.paintFormXObjectEnd){const value=stack.pop();if(!value)throw Error("Planar PDF stack underflow");matrix=value.matrix;width=value.width;opacity=value.opacity;fillOpacity=value.fillOpacity;lineJoin=value.join;}
    else if(id===OPS.transform)matrix=printPdfTransformV1(matrix,printPdfVectorV1(args,6));
    else if(id===OPS.setLineWidth)width=Number(args[0]);
    else if(id===OPS.setLineJoin)lineJoin=Number(args[0]);
    else if(id===OPS.setGState){for(const item of printPdfArrayV1(args[0]).map(printPdfArrayV1)){if(item[0]==="LW")width=Number(item[1]);if(item[0]==="CA")opacity=Number(item[1]);if(item[0]==="ca")fillOpacity=Number(item[1]);}}
    else if(id===OPS.constructPath&&args[2]){
      const packed=printPdfVectorV1(printPdfArrayV1(args[1])[0]),points:number[][]=[];let cursor=0,closed=0,valid=true;
      while(cursor<packed.length){const command=packed[cursor++]!;if(command===0||command===1)points.push([packed[cursor++]!,packed[cursor++]!]);else if(command===4)closed++;else{valid=false;break;}}
      if(!valid||closed!==1)continue;
      for(let point=points.length-1;point>0;point--)if(points[point]![0]===points[point-1]![0]&&points[point]![1]===points[point-1]![1])points.splice(point,1);
      const found=new Set(points.map(point=>point.join(","))).size===4&&Math.abs(width-.01*mm)<.002?references.findIndex(canvas=>printPdfVectorV1(args[2]).every((value,axis)=>Math.abs(value-[0,0,canvas[0]!*mm,canvas[1]!*mm][axis]!)<.03)):-1;
      if(found>=0){owner=found;reference=[...matrix];if(result.has(owner))throw Error("Planar duplicate canvas marker");result.set(owner,[]);}
      else if(owner!==undefined&&reference&&content.includes(markers[owner]!)&&points.length>=3){const transform=printPdfTransformV1(printPdfInverseV1(reference),matrix);points.forEach(point=>printPdfApplyV1(point,transform));result.get(owner)!.push({points:points.map(point=>point.map(value=>value/mm)),width:width/mm,opacity,fillOpacity,join:lineJoin});}
    }
  }
  if(stack.length)throw Error("Planar unbalanced PDF stack");
  if(content.length)throw Error("Planar unbalanced PDF ownership stack");
}

/** 🛂️ Adjudicates closed planar controls and positive native carriers with owned inference and independent AJV. */
export async function compileNativeGeoPlanarAdmission(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/geo-planar-admission",async({directory:workDir,execution})=>{
  const control=fixture.nativeGeoPlanar,failures:string[]=[],source=chartSchema["x-semio-family-options"]["geo-hexbin"].options;
  const properties=Object.fromEntries(Object.entries(source).map(([key,descriptor])=>{const contract=Object.fromEntries(Object.entries(descriptor).filter(([name])=>["type","enum","minimum","exclusiveMinimum","maximum"].includes(name))),types=Array.isArray(descriptor.type)?descriptor.type:[descriptor.type];return[key,descriptor.syntax==="expression"&&types.some(type=>type==="number"||type==="integer")?{anyOf:[contract,{type:"string",minLength:1}]}:contract];}));
  const ajv=new Ajv2020({strict:false}),validate=ajv.compile({type:"object",properties,additionalProperties:false});
  for(const entry of control.admissionCases){
    const base={chart:{...control.snapshot.chart,language:entry.language,tables:[control.table],presets:[{kind:"hexbin-map",options:{points:control.table.name,width:60,height:30,radius:5,bandwidth:5,lineWidth:".3mm",opacity:.5}}]}} as unknown as VizChartSnapshot;
    const outcome=changeVizChartValue(base,entry.mutation);if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" planar admission mutation refused");const replay=applyVizChartDiff(base,outcome.diff);if(replay.messages.length)throw Error(entry.id+" planar admission replay refused");
    const result=await inferVizChart(replay.snapshot),options={...replay.snapshot.chart.presets![0]!.options};
    for(const[key,value]of Object.entries(options)){const descriptor=source[key as keyof typeof source];if(descriptor?.syntax==="expression"&&typeof value==="string"&&/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(value.trim()))options[key]=Number(value);}
    let external=validate(options);if(entry.parsed!==null){const descriptor=source[entry.mutation.path.at(-1)! as keyof typeof source],count="itemCount" in descriptor?Number(descriptor.itemCount):undefined;const list=ajv.compile({type:"array",items:{type:"number"},...(count===undefined?{}:{minItems:count,maxItems:count})});external=Boolean(external&&list(entry.parsed));}if(result.complete!==entry.valid||external!==entry.valid)failures.push(entry.id+" owned="+result.complete+" AJV="+external+" expected="+entry.valid+" "+JSON.stringify({diagnostics:result.diagnostics,external:validate.errors}));
    if(!entry.valid&&(!result.complete)&&(result.tikz!==""||result.scene!==undefined||result.plan!==undefined))throw Error(entry.id+" invalid planar result published an output");
    if(entry.valid&&result.complete){mkdirSync(workDir,{recursive:true});await Bun.write(join(workDir,entry.id+".tex"),result.tikz);}
    if(JSON.stringify(base.chart.presets![0]!.options)!==JSON.stringify({points:control.table.name,width:60,height:30,radius:5,bandwidth:5,lineWidth:".3mm",opacity:.5}))throw Error(entry.id+" planar mutation changed authored input");
  }
  if(failures.length)throw Error("Planar source-owned admission failures:\n"+failures.join("\n"));
  console.log("[DEBUG] Geo planar "+control.admissionCases.length+" neutral closed-vocabulary/positive/numeric-literal/expression mutation controls matched owned inference and AJV");
});}

/** 🛑️ Exercises evaluated TeX dimensions and native kernel/frame predicates before drawing. */
export async function compileNativeGeoPlanarGuard(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/geo-planar-guard",async({directory:workDir,execution})=>{
  
  const rejectDir=join(workDir,"zero-width");
  {const refusal=await inspectVizProbeDocumentRefusalV1({case:"geo-planar-domain",scenario:"evaluated-zero-width",documentClass:"semio",documentClassOptions:"type=paper,language=en,theme=light",packages:["semio-viz"],preamble:["\\title{Native Planar Domain}","\\author{Semio}","\\date{}"],body:[{raw:"\\begin{VizFigure}[width=100,height=90]\\SemioVizChart{hexbin-map}[width={1-1},height=30,radius=5,lineWidth=.3mm]\\end{VizFigure}"},{values:{key:"native/accepted",clist:"1"}}]},{execution,});const log=refusal.log;if(!log.includes("Planar display width"))throw Error("Unexpected actual compiler diagnostic: "+log);}
  
  const body=fixture.nativeGeoPlanar.evaluatedCases.map(entry=>({raw:"\\SemioVizProbeBegin{geo-planar-domain}{"+entry.id+"}\\ExplSyntaxOn\\semio_viz_geosym_reset:\\keys_set:nn{semio/viz/geo/planar}{width=60,height=30,lineWidth=.3mm,"+entry.options+"}\\keys_set:nn{semio/viz/family/geo-hexbin}{"+entry.options+"}\\semio_viz_geosym_validate:\\semio_viz_probe_values:nx{native/valid}{\\bool_if:NTF\\l_semio_viz_geosym_valid_bool{1}{0}}\\ExplSyntaxOff"}));
  const records=await compileVizProbeDocument({case:"geo-planar-domain",scenario:"predicates",documentClass:"semio",documentClassOptions:"type=paper,language=en,theme=light",packages:["semio-viz"],preamble:["\\title{Native Planar Predicates}","\\author{Semio}","\\date{}","\\ExplSyntaxOn\\msg_redirect_name:nnn{semio-viz}{planar-domain}{warning}\\ExplSyntaxOff"],body},{execution,scenario:undefined});
  for(const entry of fixture.nativeGeoPlanar.evaluatedCases)equal(records.find(record=>record.scenario===entry.id&&record.key==="native/valid")!.values.map(Number),[entry.valid?1:0],entry.id+" independent evaluated finite/domain predicate");
  console.log("[DEBUG] Native "+fixture.nativeGeoPlanar.evaluatedCases.length+" evaluated finite/positive/physical-stroke predicates and actual hard compiler rejection PASS");
});}

/** 🧊️ Compares complete native exterior and hole rings to independent D3 contour vertices. */
export async function compileNativeGeoContourBoundary(execution:PrintExecutionV1):Promise<void>{if(["PRINT_NATIVE_GEO_BOUNDARY_REPLAY"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/geo-contour-boundary",async({directory:workDir,execution})=>{
  if(process.env.PRINT_NATIVE_GEO_BOUNDARY_REPLAY)workDir=process.env.PRINT_NATIVE_GEO_BOUNDARY_REPLAY;
  const control=fixture.nativeGeoContourBoundary;
  
  const body=control.cases.map(entry=>({raw:`\\SemioVizProbeBegin{geo-contour-boundary}{${entry.id}}\\SemioVizValueGrid{boundary-${entry.id}}{${entry.width}}{${entry.height}}{${entry.values.join(",")}}\\ExplSyntaxOn\\semio_viz_spatial_contour:nn{boundary-${entry.id}}{${entry.threshold}}\\semio_viz_probe_values:nx{boundary/count}{\\seq_count:N\\g_semio_viz_spatial_rings_seq}\\seq_map_inline:Nn\\g_semio_viz_spatial_rings_seq{\\seq_set_split:Nnn\\l_tmpa_seq{;}{#1}\\seq_map_inline:Nn\\l_tmpa_seq{\\semio_viz_probe_values:nn{boundary/vertex}{##1}}}\\ExplSyntaxOff`}));
  const records=process.env.PRINT_NATIVE_GEO_BOUNDARY_REPLAY?parseProbeJsonLines(readFileSync(join(workDir,"🧪️probe-out/raw-rings.probe.jsonl"),"utf8")):await compileVizProbeDocument({case:"geo-contour-boundary",scenario:"raw-rings",documentClass:"semio",documentClassOptions:"type=paper,language=en",packages:["semio-viz"],preamble:["\\title{Contour Boundaries}","\\author{Semio}","\\date{}","\\errorcontextlines=200"],body:[...body,{raw:"\\SemioVizProbePage"}]},{execution,scenario:undefined});
  const ordered=(points:number[][])=>points.sort((a,b)=>a[0]!-b[0]!||a[1]!-b[1]!);
  for(const entry of control.cases){
    if(entry.values.length!==entry.width*entry.height)throw Error(entry.id+" neutral grid cardinality");
    const rings=contours().size([entry.width,entry.height]).thresholds([entry.threshold])(entry.values).flatMap(shape=>shape.coordinates.flatMap(polygon=>polygon)),expected=rings.map(ring=>ordered(ring.slice(0,-1).map(point=>point.map(value=>value-.5)))),actual:number[][][]=[],vertices=records.filter(record=>record.scenario===entry.id&&record.key==="boundary/vertex").map(record=>record.values.map(Number));
    let current:number[][]=[];for(const vertex of vertices){if(current.length&&vertex.every((value,axis)=>Math.abs(value-current[0]![axis]!)<1e-9)){actual.push(ordered(current));current=[];}else current.push(vertex);}if(current.length)throw Error(entry.id+" native contour ring not closed");
    equal(records.find(record=>record.scenario===entry.id&&record.key==="boundary/count")!.values.map(Number),[rings.length],entry.id+" D3 complete boundary/hole ring count");
    if(actual.length!==expected.length)throw Error(entry.id+" closed ring inventory mismatch");
    for(const ring of actual){const index=expected.findIndex(candidate=>candidate.length===ring.length&&candidate.every((point,vertex)=>point.every((value,axis)=>Math.abs(value-ring[vertex]![axis]!)<1e-7)));if(index<0)throw Error(entry.id+" unmatched complete D3 exterior/hole ring "+JSON.stringify(ring));expected.splice(index,1);}
  }
  console.log("[DEBUG] Native five single/row/boundary/hole/saddle grid rings matched complete independent D3 contours");
});}
