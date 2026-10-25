/** 🧲️ Native grammar compilations, family customization and scientific controls: the family customization and its pdf shapes, the scientific defaults, control effects, canvas and mixed controls, mode defaults and the admissions of records, relations, scalars, modes and specialized placement.
 *
 * @see ../🧰️grammar-kit/🟦️.ts — the equality check
 * @see ../🟦️.ts — the native grammar run that calls these compilations
 */
import { nativeControlPdfPaint, type NativeControlPdfPath } from "../🎨️paint/🟦️.ts";
import { PRINT_PDF_OPS_V1 as OPS, printPdfPageV1, printPdfTransformV1, printPdfApplyV1, printPdfInverseV1, printPdfArrayV1, printPdfVectorV1, printPdfNameV1, type PrintPdfPageV1 } from "../../../📦️artifacts/📖️pdf/🟦️.ts";
import { basename } from "node:path";
import { withPrintScratchV1, type PrintExecutionV1 } from "../../../📦️artifacts/🎛️operation/🟦️.ts";
import { deepStrictEqual } from "node:assert";
import type { VizChartSnapshot } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { changeVizChartValue } from "../../../🧬️schema/🧬️mutations/🟦️.ts";
import { applyVizChartDiff, inverseVizChartDiff } from "../../../🧬️schema/🔀️diff/🟦️.ts";
import { inferVizChart } from "../../../🔨️modules/🏠️host/💡️inferences/🟦️.ts";
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { scaleLinear, scaleSqrt, scaleBand } from "d3-scale";
import { line, pointRadial } from "d3-shape";
import { path as d3Path } from "d3-path";
import { rgb } from "d3-color";
import { sum, max, group as d3Group } from "d3-array";
import { compileVizProbeDocument, inspectVizProbeDocumentRefusalV1, withCompiledVizProbeDocumentV1 } from "../../../🔨️modules/🧪️viz-probe/🟦️.ts";
import fixture from "../🔣️.json";
import chartSchema from "../../../🧬️schema/🔣️.json";
import Ajv2020 from "ajv/dist/2020.js";
import { equal } from "../🧰️grammar-kit/🟦️.ts";

/** 🎛️ Compiles semantic mutation controls through existing inference and compares native output with D3 and PDF.js. */
export async function compileNativeFamilyCustomizationGrammar(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/family-customization-grammar",async({directory:workDir,execution})=>{
  const controls=fixture.familyCustomization;
  const failures:string[]=[];
  let checks=0,pages=0,visible=0;
  const expected=new Map<string,{mode:string;values:number[][];canvas:number[]}>();
  for(const theme of controls.contract.themes){
    const body:{raw:string}[]=[];
    for(const entry of controls.cases){
      let snapshot=entry.snapshot as VizChartSnapshot;
      const before=JSON.stringify(snapshot);
      for(const mutation of entry.mutations){
        const outcome=changeVizChartValue(snapshot,mutation);
        if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" mutation rejected: "+JSON.stringify(outcome.messages));
        const applied=applyVizChartDiff(snapshot,outcome.diff);
        if(applied.messages.length)throw Error(entry.id+" replay rejected: "+JSON.stringify(applied.messages));
        snapshot=applied.snapshot;
      }
      if(JSON.stringify(entry.snapshot)!==before)throw Error(entry.id+" mutated its authored base");
      const result=await inferVizChart({chart:{...snapshot.chart,theme:{name:"semio",appearance:theme as "light"|"dark"}}});
      if(!result.complete)throw Error(entry.id+" inference failed: "+JSON.stringify(result.diagnostics));
      const canvas=[snapshot.chart.width,snapshot.chart.height],marker="\\draw[opacity=0.05,line width=0.01mm] (0,0) rectangle ("+canvas.join(",")+");\n";
      if(result.tikz.split("\\end{VizFigure}").length!==2)throw Error(entry.id+" native figure inventory");
      body.push({raw:"\\clearpage\\noindent\\texttt{Case~"+entry.id+"}\\par\n\\SemioVizProbeBegin{family-customization}{"+entry.id+"}\n"+result.tikz.replace("\\end{VizFigure}",marker+"\\end{VizFigure}")});
    }
    const root=join(workDir,theme);
    await withCompiledVizProbeDocumentV1({case:"family-customization",scenario:"controls",documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+theme,packages:["semio-viz"],geometry:true,preamble:["\\title{Family Customization}","\\author{Semio}","\\date{}","\\errorcontextlines=200"],body},{execution,scenario:undefined},async({records:records,probe})=>{
    for(const entry of controls.cases){
      const own=records.filter(record=>record.scenario===entry.id);
      const reference=entry.reference as {mode:string;weights?:number[];size?:number;split?:boolean;nodeCount?:number;padding?:number;cellGap?:number;nodeWidth?:number|null;nodeHeight?:number|null;gap?:number};
      const numeric=(key:string)=>own.filter(record=>record.key===key).map(record=>record.values.map(Number));
      try{
        if(reference.mode==="weight"){
          const table=entry.snapshot.chart.tables[0]!;
          const ids=[...new Set(table.rows.flatMap(row=>[String("source" in row?row.source:""),String("target" in row?row.target:"")]))];
          const strengths=ids.map(id=>sum(table.rows.map(row=>("source" in row&&row.source===id?Math.abs(Number("weight" in row?row.weight:0)):0)+("target" in row&&row.target===id?Math.abs(Number("weight" in row?row.weight:0)):0))));
          equal(strengths,reference.weights!,entry.id+" neutral weighted strengths");
          const scale=scaleSqrt([0,max(strengths)||1],[0,1]);
          const radii=strengths.map(value=>reference.size!*(.55+.75*scale(value)));
          expected.set(entry.id,{mode:reference.mode,values:radii.map(value=>[value]),canvas:[entry.snapshot.chart.width,entry.snapshot.chart.height]});
          equal(numeric("geometry/node").map(row=>row[2]!),radii,entry.id+" weighted radii",1e-4);
          checks+=strengths.length;
        }else if(reference.mode==="matrix"){
          const count=3,pad=reference.padding!,gap=reference.cellGap!,width=entry.snapshot.chart.width-2*pad,height=entry.snapshot.chart.height-2*pad;
          const side=Math.min(width*(reference.split?.52:1),height),pitch=side/count;
          const band=scaleBand(["a","b","c"],[pad,pad+side-gap]).paddingInner(gap/pitch);
          expected.set(entry.id,{mode:reference.mode,values:Array.from({length:6},()=>[band.bandwidth()]),canvas:[entry.snapshot.chart.width,entry.snapshot.chart.height]});
          const cells=numeric("geometry/poly");
          if(cells.length!==6)throw Error(entry.id+" matrix cell inventory "+cells.length);
          equal(cells.map(row=>row[2]!),cells.map(()=>band.bandwidth()),entry.id+" matrix cell width",1e-4);
          if(numeric("geometry/node").length!==reference.nodeCount)throw Error(entry.id+" split node inventory "+numeric("geometry/node").length);
          checks+=cells.length+1;
        }else if(reference.mode==="diagram"){
          const pad=reference.padding!,gap=reference.gap!,nodes=entry.snapshot.chart.tables[0]!.rows;
          const columns=["1","2"],rows=["1","2"],w=entry.snapshot.chart.width-2*pad,h=entry.snapshot.chart.height-2*pad;
          const width=reference.nodeWidth??scaleBand(columns,[0,w]).paddingInner(gap/((w+gap)/columns.length)).bandwidth();
          const height=reference.nodeHeight??scaleBand(rows,[0,h]).paddingInner(gap/((h+gap)/rows.length)).bandwidth();
          const x=scaleBand(columns,[pad,pad+columns.length*width+(columns.length-1)*gap]).paddingInner(gap/(width+gap));
          const y=scaleBand(rows,[entry.snapshot.chart.height-pad,entry.snapshot.chart.height-pad-rows.length*height-(rows.length-1)*gap]).paddingInner(gap/(height+gap));
          const actual=numeric("geometry/diagram-node");
          if(actual.length!==nodes.length)throw Error(entry.id+" node inventory "+actual.length);
          const values=nodes.map(node=>[x(String("col" in node?node.col:""))!+width/2,y(String("row" in node?node.row:""))!+height/2,width,height]);
          expected.set(entry.id,{mode:reference.mode,values,canvas:[entry.snapshot.chart.width,entry.snapshot.chart.height]});
          nodes.forEach((_,index)=>equal(actual[index]!.slice(0,4),values[index]!,entry.id+" independent dimensions "+index,1e-4));
          checks+=nodes.length*4;
        }else if(reference.mode==="mark"){
          const width=120,height=60,pad=reference.padding!,x=scaleLinear([0,10],[pad,width-pad]),y=scaleLinear([0,10],[pad,height-pad]);
          const actual=own.filter(record=>/^geometry\/mark\/[^/]+\/at$/.test(record.key));
          const rows=entry.snapshot.chart.tables[0]!.rows;
          if(actual.length!==rows.length)throw Error(entry.id+" mark inventory "+actual.length);
          const values=rows.map(row=>[x(Number("t" in row?row.t:0)),y(Number("val" in row?row.val:0))]);
          expected.set(entry.id,{mode:reference.mode,values,canvas:[width,height]});
          rows.forEach((_,index)=>equal(actual[index]!.values.slice(0,2).map(Number),values[index]!,entry.id+" enclosing frame "+index,1e-4));
          checks+=rows.length*2;
        }
      }catch(error){failures.push(theme+"/"+entry.id+": "+String(error));}
    }
    const out=join(root,"🧪️probe-out"),file=basename(probe.pdfPath);
    if(!file)throw Error(theme+" customization emitted no PDF");
    const pdf=await probe.inspect();
    {
      pages+=pdf.pages.length;
      const texts:string[]=[];
      for(let index=1;index<=pdf.pages.length;index++){const content=await (printPdfPageV1(pdf,index));texts.push(content.items.flatMap(item=>"str" in item?[item.str]:[]).join(" "));}
      for(const entry of controls.cases){
        const selected=texts.map((text,index)=>({text,page:index+1})).filter(value=>value.text.replace(/\s/g,"").includes("Case"+entry.id));
        if(selected.length!==1){failures.push(theme+"/"+entry.id+" PDF case page inventory "+selected.length);continue;}
        checks++;
        const oracle=expected.get(entry.id);
        if(oracle){
          try{
            const shapes=nativeFamilyPdfShapes(await (printPdfPageV1(pdf,selected[0]!.page)).operators,oracle.canvas);
            const circle=oracle.mode==="weight"||oracle.mode==="mark",filled=new Set([OPS.fill,OPS.eoFill,OPS.fillStroke,OPS.eoFillStroke]);
            const own=shapes.filter(shape=>shape.kind===(circle?"circle":"rect")&&(oracle.mode==="mark"||filled.has(shape.paint))&&shape.bounds[0]!>=-.01&&shape.bounds[1]!>=-.01&&shape.bounds[2]!<=oracle.canvas[0]!+.01&&shape.bounds[3]!<=oracle.canvas[1]!+.01);
            if(own.length!==oracle.values.length)throw Error(entry.id+" visible shape inventory "+own.length+" expected "+oracle.values.length);
            own.forEach((shape,index)=>{const [x0,y0,x1,y1]=shape.bounds,radius=(x1!-x0!)/2;
              const value=oracle.mode==="weight"?[radius]:oracle.mode==="matrix"?[x1!-x0!]:oracle.mode==="mark"?[(x0!+x1!)/2,(y0!+y1!)/2]:[(x0!+x1!)/2,(y0!+y1!)/2,x1!-x0!,y1!-y0!];
              equal(value,oracle.values[index]!,entry.id+" visible PDF shape "+index,.01);visible++;});
          }catch(error){failures.push(theme+"/"+entry.id+": "+String(error));}
        }
        if(entry.reference.mode!=="text")continue;
        const text=selected[0]!.text;
        const reference=entry.reference as {texts:string[];absent:string[]};
        for(const expected of reference.texts){if(!text.includes(expected))failures.push(theme+"/"+entry.id+" missing PDF text "+expected);checks++;}
        for(const absent of reference.absent){if(text.includes(absent))failures.push(theme+"/"+entry.id+" wrong locale text "+absent);checks++;}
      }
    }
  });}
  if(failures.length)throw Error(failures.length+" native customization mismatches:\n"+failures.join("\n"));
  console.log("[native-customization] "+controls.cases.length+" mutation/inference cases, "+checks+" independent D3/PDF.js checks, "+pages+" actual pages and "+visible+" visible PDF shapes across both themes");
});}

type NativeFamilyPdfShape={kind:"circle"|"rect";bounds:number[];paint:string};

/** 🔬️ Measures visible native glyphs relative to an independently declared PDF canvas reference. */
function nativeFamilyPdfShapes(operators:PrintPdfPageV1["operators"],canvas:number[]):NativeFamilyPdfShape[]{
  const shapes:{kind:"circle"|"rect";points:number[][];matrix:number[];paint:string}[]=[],stack:{matrix:number[];width:number}[]=[];
  const point=scaleLinear([0,1],[0,72/25.4]);
  let matrix=[1,0,0,1,0,0],width=1,reference:number[]|undefined;
  for(const [index,id]of operators.fnArray.entries()){
    const args=printPdfArrayV1(operators.argsArray[index]);
    if(id===OPS.save||id===OPS.paintFormXObjectBegin){stack.push({matrix:[...matrix],width});if(id===OPS.paintFormXObjectBegin&&args[0])matrix=printPdfTransformV1(matrix,printPdfVectorV1(args[0],6));}
    else if(id===OPS.restore||id===OPS.paintFormXObjectEnd){const saved=stack.pop();if(!saved)throw Error("native PDF graphics stack underflow");matrix=saved.matrix;width=saved.width;}
    else if(id===OPS.transform)matrix=printPdfTransformV1(matrix,printPdfVectorV1(args,6));
    else if(id===OPS.setLineWidth)width=Number(args[0]);
    else if(id===OPS.setGState){for(const state of printPdfArrayV1(args[0]).map(printPdfArrayV1))if(state[0]==="LW")width=Number(state[1]);}
    else if(id===OPS.constructPath&&args[2]){
      const path=printPdfVectorV1(printPdfArrayV1(args[1])[0]);
      let cursor=0,start:number[]|undefined,lines=0,curves=0,closed=0;
      const points:number[][]=[],corners:number[][]=[];
      while(cursor<path.length){
        const command=path[cursor++]!;
        if(command===0){start=[path[cursor++]!,path[cursor++]!];if(!lines&&!curves){corners.length=0;corners.push([...start]);}}
        else if(command===1){start=[path[cursor++]!,path[cursor++]!];corners.push([...start]);lines++;}
        else if(command===2){
          if(!start)throw Error("native PDF curve without a starting point");
          const curve=[start,[path[cursor++]!,path[cursor++]!],[path[cursor++]!,path[cursor++]!],[path[cursor++]!,path[cursor++]!]];
          for(let sample=0;sample<=128;sample++){const t=sample/128,u=1-t;points.push([0,1].map(axis=>u**3*curve[0]![axis]!+3*u*u*t*curve[1]![axis]!+3*u*t*t*curve[2]![axis]!+t**3*curve[3]![axis]!));}
          start=curve[3]!;curves++;
        }else if(command===4)closed++;
        else{curves=-1;break;}
      }
      const kind=closed<=1&&curves===4&&lines===0?"circle":closed===1&&curves===0&&lines===3?"rect":undefined;
      if(kind==="rect"&&args[0]===OPS.stroke&&Math.abs(width-point(.01))<.002&&printPdfVectorV1(args[2]).every((value,axis)=>Math.abs(value-[0,0,point(canvas[0]!),point(canvas[1]!)][axis]!)<point(.01))){
        if(reference)throw Error("duplicate native PDF canvas reference");reference=[...matrix];
      }else if(kind)shapes.push({kind,points:kind==="circle"?points:corners,matrix:[...matrix],paint:printPdfNameV1(args[0])});
    }
  }
  if(stack.length)throw Error("unbalanced native PDF graphics stack");
  if(!reference)throw Error("native PDF canvas reference missing");
  const inverse=printPdfInverseV1(reference);
  return shapes.map(shape=>{
    const relative=printPdfTransformV1(inverse,shape.matrix);
    shape.points.forEach(value=>printPdfApplyV1(value,relative));
    const bounds=[Math.min(...shape.points.map(value=>value[0]!)),Math.min(...shape.points.map(value=>value[1]!)),Math.max(...shape.points.map(value=>value[0]!)),Math.max(...shape.points.map(value=>value[1]!))].map(point.invert);
    return {kind:shape.kind,bounds,paint:shape.paint};
  });
}


/** 🏠️ Matches per-family native defaults to the canonical schema using neutral AJV vectors. */
export function compileNativeScientificDefaults():void{
  const control=fixture.scientificFamilyDefaults,ajv=new Ajv2020({strict:false}),failures:string[]=[];
  
  for(const entry of control.vectors){const value=(chartSchema["x-semio-family-options"] as Record<string,{options:Record<string,{default?:unknown}>}>)[entry.family]!.options[entry.key]!.default,independent=ajv.compile({const:entry.expected})(value);if(independent!==(value===entry.expected)||!independent)failures.push(entry.family+"/"+entry.key+" schema default "+JSON.stringify(value)+" expected native "+entry.expected);}
  if(failures.length)throw Error("Scientific per-family default contracts:\n"+failures.join("\n"));
}

/** 🧫️ Verifies scientific customization through canonical mutations, D3 and owned actual PDF paint. */
export async function compileNativeScientificControlEffects(execution:PrintExecutionV1):Promise<void>{if(["PRINT_NATIVE_SCIENTIFIC_REPLAY"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/scientific-control-effects",async({directory:workDir,execution})=>{
  const neutral=fixture.nativeScientificControlEffects,controls=neutral.vectors,before=JSON.stringify(controls),failures:string[]=[];
  
  
  const filled=new Set([OPS.fill,OPS.eoFill,OPS.fillStroke,OPS.eoFillStroke,OPS.closeFillStroke,OPS.closeEOFillStroke]);
  let painted=0;
  const themes=(["light","dark"] as const).filter(value=>!process.env.PRINT_NATIVE_CONTROL_THEME||value===process.env.PRINT_NATIVE_CONTROL_THEME);
  if(!themes.length)throw Error("Unknown scientific control theme");
  for(const appearance of themes){
    const body:{raw:string}[]=[],options=new Map<string,Record<string,string|number|boolean>>();
    for(const entry of controls.cases){
      let snapshot={chart:{width:controls.canvas[0],height:controls.canvas[1],language:entry.language,theme:{appearance},layers:[],presets:[{kind:entry.kind,options:{width:80,height:48,axes:false,grid:false,...entry.options}}]}} as unknown as VizChartSnapshot;
      const original=JSON.stringify(snapshot);
      for(const mutation of entry.mutations){
        const outcome=changeVizChartValue(snapshot,mutation);
        if(outcome.messages.length||outcome.diff.edits.length>1)throw Error(entry.id+" scientific mutation rejected: "+JSON.stringify(outcome.messages));
        const replay=applyVizChartDiff(snapshot,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(snapshot,outcome.diff));
        if(replay.messages.length||inverse.messages.length)throw Error(entry.id+" scientific guarded replay rejected");
        deepStrictEqual(inverse.snapshot,snapshot);snapshot=replay.snapshot;
      }
      if(original!==JSON.stringify({chart:{width:controls.canvas[0],height:controls.canvas[1],language:entry.language,theme:{appearance},layers:[],presets:[{kind:entry.kind,options:{width:80,height:48,axes:false,grid:false,...entry.options}}]}}))throw Error(entry.id+" scientific mutation changed base");
      const result=await inferVizChart(snapshot);
      if(!result.complete)throw Error(entry.id+" scientific inference rejected: "+JSON.stringify(result.diagnostics));
      options.set(entry.id,snapshot.chart.presets![0]!.options as Record<string,string|number|boolean>);
      const reference="\\draw[line width=.01mm] (0,0) rectangle ("+controls.canvas.join(",")+");\n\\special{pdf:literal direct /SemioVizControl"+entry.id+" BMC}\n";
      const emitted=result.tikz.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1"+reference).replace("\\end{VizFigure}","\\special{pdf:literal direct EMC}\n\\end{VizFigure}");
      body.push({raw:"\\clearpage\\noindent\\texttt{Case~"+entry.id+"}\\par\n\\SemioVizProbeBegin{scientific-controls}{"+entry.id+"}\n"+emitted});
    }
    const root=join(workDir,appearance);
    await withCompiledVizProbeDocumentV1({case:"scientific-controls",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language=en,theme="+appearance,packages:["semio-viz"],geometry:true,preamble:["\\title{Scientific Control Effects}","\\author{Semio}","\\date{}","\\errorcontextlines=200"],body},{execution,scenario:undefined},async({records:records,probe})=>{
    const pdfRoot=process.env.PRINT_NATIVE_SCIENTIFIC_REPLAY?join(process.env.PRINT_NATIVE_SCIENTIFIC_REPLAY,appearance):root;
    const pdf=await probe.inspect();
    {
      const pages:{text:string;page:number}[]=[];
      for(let page=1;page<=pdf.pages.length;page++){const content=await(printPdfPageV1(pdf,page));pages.push({page,text:content.items.flatMap(item=>"str"in item?[item.str]:[]).join("").replace(/\s/g,"")});}
      for(const entry of controls.cases){
        try{
          const selected=pages.filter(page=>page.text.includes("Case"+entry.id));
          if(selected.length!==1)throw Error("Actual scientific case page count "+selected.length);
          const {paths}=await nativeControlPdfPaint(await(printPdfPageV1(pdf,selected[0]!.page)).operators,controls.canvas,"SemioVizControl"+entry.id,probe.control),o=options.get(entry.id)!;
          const numeric=(key:string)=>Number(o[key]),project=(point:number[],domain:number[],range:number[])=>[scaleLinear(domain,[0,numeric("width")])(point[0]!),scaleLinear(range,[0,numeric("height")])(point[1]!)];
          const match=(actual:NativeControlPdfPath[],expected:number[][][],label:string)=>{
            if(actual.length!==expected.length)throw Error(label+" actual painted paths "+actual.length+" expected "+expected.length);
            const pending=[...actual];
            for(const points of expected){
              const drawing=line()(points as [number,number][]);
              if(!drawing)throw Error("Independent D3 path missing");
              const vertices=drawing.match(/[-+]?(?:\d*\.?\d+)(?:e[-+]?\d+)?/gi)!.map(Number),oracle=Array.from({length:vertices.length/2},(_,index)=>vertices.slice(index*2,index*2+2));
              const found=pending.findIndex(path=>path.points.length===oracle.length&&path.points.every((point,index)=>point.every((value,axis)=>Math.abs(value-oracle[index]![axis]!)<=.025)));
              if(found<0)throw Error(label+" unmatched independent D3 path "+JSON.stringify(oracle)+"; observed "+JSON.stringify(pending.slice(0,4).map(path=>path.points)));
              pending.splice(found,1);painted++;
            }
          };
          if(entry.mode==="bracket"){
            const rounds=numeric("rounds"),rw=numeric("roundWidth"),sh=numeric("slotHeight"),records=String(o.entries).split(",").map((label,index)=>[1,index+1,label]).concat(String(o.results).split(",").map(value=>value.split("/").map((part,index)=>index<2?Number(part):part)).filter(value=>Number(value[0])<=rounds+1));
            const boxes=paths.filter(path=>path.closed&&filled.has(path.paint)&&Math.abs(path.bounds[2]!-path.bounds[0]!-.72*rw)<.025);
            if(boxes.length!==records.length)throw Error("Round-controlled boxes "+boxes.length+" expected "+records.length);
            const centers=boxes.map(path=>[(path.bounds[0]!+path.bounds[2]!)/2,(path.bounds[1]!+path.bounds[3]!)/2]);
            records.forEach(record=>{const center=[(Number(record[0])-1)*rw+.36*rw,-((Number(record[1])-.5)*2**(Number(record[0])-1)-.5)*sh];if(!centers.some(point=>point.every((value,axis)=>Math.abs(value-center[axis]!)<.025)))throw Error("Round-controlled box center missing "+JSON.stringify(center));});painted+=boxes.length;
          }else if(entry.mode==="grid"){
            const domain=String(o.domain).split(",").map(Number),range=String(o.range).split(",").map(Number),beta=numeric("beta"),n=numeric("gridLines"),expected:number[][][]=[];
            for(const axis of [0,1]){
              const corners=domain.flatMap(x=>range.map(y=>axis===0?x-beta*y:y-beta*x)),levels=scaleLinear([0,n+1],[Math.min(...corners),Math.max(...corners)]);
              for(let index=1;index<=n;index++){
                const k=levels(index),candidates=axis===0?[...range.map(y=>[k+beta*y,y]),...(beta?domain.map(x=>[x,(x-k)/beta]):[])]:[...domain.map(x=>[x,k+beta*x]),...(beta?range.map(y=>[(y-k)/beta,y]):[])];
                const endpoints=[...new Map(candidates.filter(([x,y])=>x!>=domain[0]!-1e-8&&x!<=domain[1]!+1e-8&&y!>=range[0]!-1e-8&&y!<=range[1]!+1e-8).map(point=>[point.map(value=>value.toFixed(8)).join(","),point])).values()].sort((a,b)=>a[axis===0?1:0]!-b[axis===0?1:0]!);
                expected.push(endpoints.map(point=>project(point,domain,range)));
              }
            }
            match(paths.filter(path=>!path.closed&&Math.abs(path.opacity-.3)<.001),expected,"Primed grid");
          }else if(entry.mode==="beams"){
            let x=6,previous:number[]|undefined;const expected:number[][][]=[];
            for(const event of String(o.events).split(",")){const [step,duration]=event.split("/").map(Number),tip=[x+1.6,step!*numeric("spacing")/2+9];if(duration===.5){if(previous&&o.beams)expected.push([previous,tip]);previous=tip;}else previous=undefined;x+=numeric("stride")*duration!;}
            match(paths.filter(path=>!path.closed&&Math.abs(path.width-2.25*25.4/72.27)<.003),expected,"Staff beams");
          }else if(entry.mode==="eye"){
            const span=numeric("span"),samples=numeric("samples"),domain=o.domain?String(o.domain).split(",").map(Number):[-span/2,span/2],range=String(o.range).split(",").map(Number),expected:number[][][]=[];let seed=numeric("seed");
            const random=()=>{seed=seed*48271%2147483647;return seed/2147483647;};
            for(let trace=0;trace<24;trace++){const polarity=random()<.5?-1:1,jitter=(random()-.5)*.16;expected.push(Array.from({length:span*samples+1},(_,index)=>{const t=-span/2+index/samples;return project([t,polarity*Math.cos(Math.PI*t)**2+jitter],domain,range);}));}
            match(paths.filter(path=>!path.closed&&Math.abs(path.opacity-.45)<.001),expected,"Eye symbol span");
          }else if(entry.mode==="pole"){
            const sx=numeric("sigmaX"),sy=numeric("sigmaY"),tau=numeric("tauXY"),center=(sx+sy)/2,radius=Math.hypot((sx-sy)/2,tau),domain=[center-1.25*radius,center+1.25*radius],range=[-1.25*radius,1.25*radius],pole=project([sy,tau],domain,range),segments=o.pole?[[project([sx,tau],domain,range),pole],[project([sy,-tau],domain,range),pole]]:[];
            match(paths.filter(path=>!path.closed&&Math.abs(path.opacity-.6)<.001),segments,"Mohr pole construction");
            const dots=paths.filter(path=>path.closed&&filled.has(path.paint)&&Math.abs(path.bounds[2]!-path.bounds[0]!-1.4)<.025);
            if(dots.length!==(o.pole?1:0))throw Error("Mohr pole marker count "+dots.length);
            if(dots.length)equal([(dots[0]!.bounds[0]!+dots[0]!.bounds[2]!)/2,(dots[0]!.bounds[1]!+dots[0]!.bounds[3]!)/2],pole,"Mohr pole circle incidence",.025);
          }else{
            const columns=numeric("columns"),rows=numeric("rows"),size=numeric("size"),node=(x:number,y:number)=>[x*size,y*size+x*size/4+y*size/9],expected:number[][][]=[];
            if(o.fillCells)for(let x=0;x<columns;x++)for(let y=0;y<rows;y++){const a=node(x,y),b=node(x+1,y),c=node(x+1,y+1),d=node(x,y+1);if(o.mode==="triangulation")expected.push([a,b,c,a],[a,c,d,a]);else expected.push([a,b,c,d,a]);}
            const fills=paths.filter(path=>path.closed&&filled.has(path.paint));
            match(fills.map(path=>({...path,points:[...path.points,path.points[0]!]})),expected,"Warped cell fills");
            if(o.fillCells&&new Set(fills.map(path=>path.fill.join(","))).size!==columns*rows)throw Error("Cell index colors are not distinct");
          }
        }catch(error){failures.push(appearance+"/"+entry.id+": "+String(error));}
      }
    }
  });}
  if(JSON.stringify(controls)!==before)throw Error("Scientific neutral vectors changed");
  if(failures.length)throw Error("Scientific control effects:\n"+failures.join("\n"));
});}

/** 🏷️ Supplies independent body/chrome ownership for actual scientific PDF probes. */
function scientificCanvasProbePreamble():string[]{return["\\title{Scientific Viewports}","\\author{Semio}","\\date{}","\\errorcontextlines=200",String.raw`\ExplSyntaxOn
\tl_new:N \l_semio_viz_test_canvas_case_tl
\cs_new_eq:NN \semio_viz_test_canvas_frame: \semio_viz_sci_frame:
\cs_set_protected:Npn \semio_viz_sci_frame: { \exp_args:Nx \special {pdf:literal~direct~/SemioVizChrome\tl_use:N \l_semio_viz_test_canvas_case_tl \c_space_tl BMC} \semio_viz_test_canvas_frame: \special{pdf:literal~direct~EMC} }
\cs_if_exist:NT \semio_viz_sci_physical_begin: {
\cs_new_eq:NN \semio_viz_test_canvas_begin: \semio_viz_sci_physical_begin:
\cs_new_eq:NN \semio_viz_test_canvas_end: \semio_viz_sci_physical_end:
\cs_set_protected:Npn \semio_viz_sci_physical_begin: { \semio_viz_test_canvas_begin: \exp_args:Nx \special{pdf:literal~direct~/SemioVizBody\tl_use:N \l_semio_viz_test_canvas_case_tl \c_space_tl BMC} }
\cs_set_protected:Npn \semio_viz_sci_physical_end: { \special{pdf:literal~direct~EMC} \semio_viz_test_canvas_end: }
}
\ExplSyntaxOff`];}

/** 🪟️ Compares physical scientific viewports with independent D3 affine paint and isolated chrome. */
export async function compileNativeScientificCanvasControls(execution:PrintExecutionV1,neutral:{vectors:{canvas:number[];familyFrame:number[];domain:number[];range:number[];tickDivisions:number[];variants?:number[];cases:{family:string;kind:string;options:Record<string,unknown>;physicalOutline?:number[][];intrinsicGrid?:{kind:string;radius:number;labels:string[]}}[]}}=fixture.scientificCanvasControlEffects):Promise<void>{if(["PRINT_NATIVE_CONSTRUCTION_TIKZ","PRINT_NATIVE_CANVAS_REPLAY"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/scientific-canvas-controls",async({directory:workDir,execution})=>{
  const control=neutral.vectors,before=JSON.stringify(control),failures:string[]=[];
  
  const cases=control.cases.flatMap((entry,index)=>(control.variants??[0,1,2]).map(mode=>({entry,mode,id:"Cv"+index+"M"+mode}))),[width,height]=control.familyFrame,{domain,range}=control,preamble=scientificCanvasProbePreamble();
  let matched=0;
  const themes=(["light","dark"] as const).filter(theme=>!process.env.PRINT_NATIVE_CONTROL_THEME||theme===process.env.PRINT_NATIVE_CONTROL_THEME);if(!themes.length)throw Error("Unknown scientific canvas theme");
  for(const appearance of themes){
    const language=appearance==="light"?"en":"de",body:{raw:string}[]=[];
    for(const entry of cases){
      const initial={chart:{width:control.canvas[0],height:control.canvas[1],language,theme:{appearance},layers:[],presets:[{kind:entry.entry.kind,options:{...entry.entry.options,...(control.variants&&typeof entry.entry.options.points==="string"?{points:"O/0/0,Q/1/0,R/0/1"}:{}),width,height,...(entry.mode===3?{}:{axes:false,grid:false}),title:"",xlabel:"",ylabel:"",domain:"",range:""}}]}} as unknown as VizChartSnapshot;
      let snapshot=initial;const source=JSON.stringify(initial),changes:Record<string,string|number|boolean>=entry.mode&&entry.mode!==3?{axes:true,grid:true,ticks:control.tickDivisions[entry.mode===1?0:1]!,title:"Cvtitle",xlabel:"Cvx",ylabel:"Cvy",...([2,4,6].includes(entry.mode)?{domain:(entry.mode===6?[...domain].reverse():domain).join(",")}:{}),...([2,5,6].includes(entry.mode)?{range:(entry.mode===6?[...range].reverse():range).join(",")}: {})}:{};
      if(control.variants&&typeof entry.entry.options.points==="string")changes.points=entry.entry.options.points;
      for(const[key,value]of Object.entries(changes)){
        const mutation=changeVizChartValue(snapshot,{path:["presets","0","options",key],value});if(mutation.messages.length||mutation.diff.edits.length>1)throw Error(entry.id+" canvas mutation rejected: "+JSON.stringify(mutation.messages));
        const replay=applyVizChartDiff(snapshot,mutation.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(snapshot,mutation.diff));if(replay.messages.length||inverse.messages.length)throw Error(entry.id+" canvas replay rejected");deepStrictEqual(inverse.snapshot,snapshot);snapshot=replay.snapshot;
      }
      if(JSON.stringify(initial)!==source)throw Error(entry.id+" canvas mutation changed input");const result=await inferVizChart(snapshot);if(!result.complete)throw Error(entry.id+" canvas inference rejected: "+JSON.stringify(result.diagnostics));
      const nativeSource=process.env.PRINT_NATIVE_CONSTRUCTION_TIKZ?readFileSync(join(process.env.PRINT_NATIVE_CONSTRUCTION_TIKZ,entry.id+"-"+appearance+".tex"),"utf8"):result.tikz;if(!nativeSource.includes("{"+entry.entry.kind+"}"))throw Error("Construction native source kind mismatch "+entry.id);
      const reference="\\draw[line width=.01mm] (0,0) rectangle ("+control.canvas.join(",")+");\n\\special{pdf:literal direct /SemioVizControl"+entry.id+" BMC}\n",emitted=nativeSource.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1"+reference).replace("\\end{VizFigure}","\\special{pdf:literal direct EMC}\n\\end{VizFigure}");
      body.push({raw:"\\clearpage\\noindent\\texttt{Case~"+entry.id+"}\\par\\ExplSyntaxOn\\tl_set:Nn\\l_semio_viz_test_canvas_case_tl{"+entry.id+"}\\ExplSyntaxOff\n\\SemioVizProbeBegin{scientific-canvas}{"+entry.id+"}\\SemioVizProbeValues{canvas/observer}{1}\n"+emitted});
    }
    const directory=join(workDir,appearance);await withCompiledVizProbeDocumentV1({case:"scientific-canvas",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],geometry:true,preamble,body},{execution,scenario:undefined},async({records:records,probe})=>{
    const pdfDirectory=process.env.PRINT_NATIVE_CANVAS_REPLAY?join(process.env.PRINT_NATIVE_CANVAS_REPLAY,appearance):directory,pdf=await probe.inspect(),paint=new Map<string,{body:Awaited<ReturnType<typeof nativeControlPdfPaint>>;chrome?:Awaited<ReturnType<typeof nativeControlPdfPaint>>}>();
    {
      for(let page=1;page<=pdf.pages.length;page++){
        const operators=await(printPdfPageV1(pdf,page)).operators,has=(marker:string)=>operators.argsArray.some((args,index)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps].includes(operators.fnArray[index]!)&&printPdfNameV1(args?.[0])===marker);
        for(const entry of cases)if(has("SemioVizControl"+entry.id)){
          if(paint.has(entry.id))throw Error("Scientific canvas duplicate case page "+entry.id);
          paint.set(entry.id,{body:await nativeControlPdfPaint(operators,control.canvas,(has("SemioVizBody"+entry.id)?"SemioVizBody":"SemioVizControl")+entry.id,probe.control,["SemioVizChrome"+entry.id]),...(has("SemioVizChrome"+entry.id)?{chrome:await nativeControlPdfPaint(operators,control.canvas,"SemioVizChrome"+entry.id,probe.control)}:{})});
        }
      }
      for(const entry of cases)try{
        const actual=paint.get(entry.id),baseline=paint.get(entry.id.replace(/M\d$/,"M0"));if(!actual||!baseline)throw Error("Scientific canvas actual case body missing");if(!baseline.body.paths.length&&!baseline.body.glyphs.length)throw Error("Scientific canvas baseline has no actual body paint");
        const dx=[2,4,6].includes(entry.mode)?(entry.mode===6?[...domain].reverse():domain):[0,width!],dy=[2,5,6].includes(entry.mode)?(entry.mode===6?[...range].reverse():range):[0,height!],x=scaleLinear(dx,[0,width!]),y=scaleLinear(dy,[0,height!]),scale=Math.sqrt(Math.abs(width!/(dx[1]!-dx[0]!)*height!/(dy[1]!-dy[0]!)));
        if(entry.entry.physicalOutline){const expected=entry.entry.physicalOutline.map(point=>[scaleLinear([0,width!],[0,width!])(point[0]!),scaleLinear([0,height!],[0,height!])(point[1]!)]),outlines=baseline.body.paths.filter(path=>path.closed&&(path.paint===OPS.stroke||path.paint===OPS.closeStroke)&&path.points.length===expected.length);if(outlines.length!==1)throw Error("Authored construction outline count "+outlines.length);outlines[0]!.points.forEach((point,index)=>equal(point,expected[index]!,"Independent D3 authored construction outline",.025));const dots=baseline.body.paths.filter(path=>path.closed&&[OPS.fill,OPS.eoFill,OPS.fillStroke,OPS.eoFillStroke,OPS.closeFillStroke,OPS.closeEOFillStroke].includes(path.paint));if(dots.length!==expected.length)throw Error("Authored construction dot count "+dots.length);for(const point of expected)if(!dots.some(dot=>Math.abs((dot.bounds[0]!+dot.bounds[2]!)/2-point[0]!)<.025&&Math.abs((dot.bounds[1]!+dot.bounds[3]!)/2-point[1]!)<.025))throw Error("Independent authored construction dot missing");}
        const intrinsic=entry.entry.intrinsicGrid,isIntrinsic=(value:NativeControlPdfPath)=>Boolean(intrinsic&&!value.closed&&value.points.length===2&&Math.abs(value.opacity-.3)<.001),originalPaths=baseline.body.paths.filter(value=>!isIntrinsic(value)),actualPaths=actual.body.paths.filter(value=>!isIntrinsic(value));
        if(intrinsic){const spokes=actual.body.paths.filter(isIntrinsic),expected=entry.mode?intrinsic.labels.map((_,index)=>{const point=pointRadial(2*Math.PI*index/intrinsic.labels.length,intrinsic.radius);return[[x(0),y(0)],[x(point[0]),y(-point[1])]];}):[];if(spokes.length!==expected.length)throw Error("Scientific intrinsic radial grid count "+spokes.length+" expected "+expected.length);const pending=[...spokes];for(const oracle of expected){const at=pending.findIndex(value=>value.points.every((point,index)=>point.every((coordinate,axis)=>Math.abs(coordinate-oracle[index]![axis]!)<.035)));if(at<0)throw Error("Independent D3 radial grid projection missing "+JSON.stringify(oracle));const spoke=pending.splice(at,1)[0]!;if(spoke.width<=0)throw Error("Intrinsic radial grid stroke missing");matched++;}}
        if(actualPaths.length!==originalPaths.length)throw Error("Scientific body paths "+actualPaths.length+" expected "+originalPaths.length);
        for(const[index,original]of originalPaths.entries()){
          const value=actualPaths[index]!;if(value.paint!==original.paint||value.closed!==original.closed||value.points.length!==original.points.length)throw Error("Scientific body topology changed at "+index);
          for(const[vertex,point]of original.points.entries())equal(value.points[vertex]!,[x(point[0]!),y(point[1]!)],"D3 scientific affine body "+index+"/"+vertex,.035);
          equal(value.bounds,[Math.min(x(original.bounds[0]!),x(original.bounds[2]!)),Math.min(y(original.bounds[1]!),y(original.bounds[3]!)),Math.max(x(original.bounds[0]!),x(original.bounds[2]!)),Math.max(y(original.bounds[1]!),y(original.bounds[3]!))],"D3 scientific exact affine bounds "+index,.035);
          equal([value.width,value.opacity,value.fillOpacity],[original.width*scale,original.opacity,original.fillOpacity],"Scientific affine paint state "+index,.005);equal(value.stroke,original.stroke,"Scientific stroke colour",.002);equal(value.fill,original.fill,"Scientific fill colour",.002);matched++;
        }
        if(actual.body.glyphs.length!==baseline.body.glyphs.length)throw Error("Scientific body glyph count changed");for(const[index,original]of baseline.body.glyphs.entries()){const value=actual.body.glyphs[index]!;if(value.text!==original.text||!value.origin||!original.origin||!value.advance||!original.advance)throw Error("Scientific body glyph content/geometry missing");equal(value.origin,[x(original.origin[0]!),y(original.origin[1]!)],"D3 scientific affine glyph origin",.035);equal(value.advance,[x(original.advance[0]!)-x(0),y(original.advance[1]!)-y(0)],"D3 scientific affine glyph advance",.035);}
        if(!actual.chrome){if(!entry.mode||entry.mode===3)continue;throw Error("Enabled scientific chrome ownership missing");}const chrome=actual.chrome;
        if(!entry.mode||entry.mode===3){if(chrome.paths.length||chrome.glyphs.length)throw Error("Disabled scientific chrome paints");continue;}
        const ticks=control.tickDivisions[entry.mode===1?0:1]!,grid=chrome.paths.filter(path=>Math.abs(path.opacity-.3)<.001&&!path.closed),axes=chrome.paths.filter(path=>path.closed&&Math.abs(path.opacity-1)<.001);
        if(grid.length!==2*(ticks+1)||axes.length!==1)throw Error("Scientific chrome grid/axis paths "+grid.length+"/"+axes.length+" expected "+2*(ticks+1)+"/1");
        for(let tick=0;tick<=ticks;tick++){const px=scaleLinear([0,ticks],[0,width!])(tick),py=scaleLinear([0,ticks],[0,height!])(tick);for(const expected of [[[px,0],[px,height!]],[[0,py],[width!,py]]])if(!grid.some(path=>path.points.length===2&&path.points.every((point,index)=>point.every((value,axis)=>Math.abs(value-expected[index]![axis]!)<.025))))throw Error("Independent D3 scientific grid missing "+JSON.stringify(expected));}
        equal(axes[0]!.bounds,[0,0,width!,height!],"Scientific canvas axis bounds",.025);const text=chrome.glyphs.map(glyph=>glyph.text).join("").replace(/\s/g,"");for(const label of ["Cvtitle","Cvx","Cvy"])if(!text.includes(label))throw Error("Scientific chrome label missing "+label);
        const tickX=scaleLinear([0,ticks],dx),tickY=scaleLinear([0,ticks],dy),format=(value:number)=>String(Number(value.toFixed(2))).replace(".",language==="de"?",":"."),labels=Array.from({length:ticks+1},(_,index)=>format(tickX(index))+format(tickY(index))).join("");if(!text.replace(/[−–]/g,"-").startsWith(labels))throw Error("Independent localized D3 tick labels "+text+" expected prefix "+labels);
      }catch(error){failures.push(appearance+"/"+entry.entry.family+"/"+entry.id+": "+String(error));}
    }
  });}
  if(JSON.stringify(control)!==before)throw Error("Scientific canvas neutral vectors changed");if(failures.length)throw Error("Scientific physical canvas effects:\n"+failures.join("\n"));
});}

/** 🧭️ Checks numeric and physical branch windows without changing their domain-specific coordinates. */
export async function compileNativeScientificMixedControls(execution:PrintExecutionV1):Promise<void>{if(["PRINT_NATIVE_MIXED_REPLAY","PRINT_NATIVE_CANVAS_REPLAY"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/scientific-mixed-controls",async({directory:workDir,execution})=>{
  const neutral=fixture.scientificMixedNumericWindows,control=neutral.vectors,original=JSON.stringify(control),failures:string[]=[];
  
  const cases=control.cases.flatMap((entry,index)=>[0,1,2].map(mode=>({entry,mode,id:"Mx"+index+"M"+mode}))),[width,height]=control.frame,themes=(["light","dark"] as const).filter(theme=>!process.env.PRINT_NATIVE_CONTROL_THEME||theme===process.env.PRINT_NATIVE_CONTROL_THEME);
  if(!themes.length)throw Error("Unknown mixed scientific theme");let matched=0;
  for(const appearance of themes){
    const language=appearance==="light"?"en":"de",body:{raw:string}[]=[];
    for(const entry of cases){
      const options={mode:entry.entry.mode,width,height,axes:false,grid:false,title:"",xlabel:"",ylabel:"",...(entry.entry.projection==="orbital"?{levels:control.levels.map(row=>row.join("/")).join(",")}: {surface:"generic",columns:2,rows:2,events:control.events.map(row=>row.join("/")).join(","),nodes:control.nodes.map(row=>row.join("/")).join(","),edges:control.edges.map(row=>row.join("/")+"/").join(",")})};
      const initial={chart:{width:control.canvas[0],height:control.canvas[1],language,theme:{appearance},layers:[],presets:[{kind:entry.entry.kind,options}]}} as unknown as VizChartSnapshot;let snapshot=initial;const before=JSON.stringify(initial),changes:Record<string,string|number|boolean>=entry.mode?{axes:true,grid:true,ticks:entry.mode===1?2:5,title:"Cvtitle",xlabel:"Cvx",ylabel:"Cvy",...(entry.mode===2?{domain:entry.entry.windowDomain.join(","),range:entry.entry.windowRange.join(",")}: {})}:{};
      for(const[key,value]of Object.entries(changes)){const outcome=changeVizChartValue(snapshot,{path:["presets","0","options",key],value});if(outcome.messages.length||outcome.diff.edits.length>1)throw Error(entry.id+" mixed scientific mutation rejected");const replay=applyVizChartDiff(snapshot,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(snapshot,outcome.diff));if(replay.messages.length||inverse.messages.length)throw Error(entry.id+" mixed scientific replay rejected");deepStrictEqual(inverse.snapshot,snapshot);snapshot=replay.snapshot;}
      if(JSON.stringify(initial)!==before)throw Error("Mixed scientific input changed");const result=await inferVizChart(snapshot);if(!result.complete)throw Error(entry.id+" mixed scientific inference rejected: "+JSON.stringify(result.diagnostics));
      const reference="\\draw[line width=.01mm] (0,0) rectangle ("+control.canvas.join(",")+");\n\\special{pdf:literal direct /SemioVizControl"+entry.id+" BMC}\n",emitted=result.tikz.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1"+reference).replace("\\end{VizFigure}","\\special{pdf:literal direct EMC}\n\\end{VizFigure}");
      body.push({raw:"\\clearpage\\ExplSyntaxOn\\tl_set:Nn\\l_semio_viz_test_canvas_case_tl{"+entry.id+"}\\ExplSyntaxOff\\SemioVizProbeBegin{scientific-mixed-controls}{"+entry.id+"}\n"+emitted});
    }
    const directory=join(workDir,"numeric",appearance);await withCompiledVizProbeDocumentV1({case:"scientific-mixed-controls",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],geometry:true,preamble:scientificCanvasProbePreamble(),body},{execution,scenario:undefined},async({records:records,probe})=>{
    const pdfDirectory=process.env.PRINT_NATIVE_MIXED_REPLAY?join(process.env.PRINT_NATIVE_MIXED_REPLAY,"numeric",appearance):directory,pdf=await probe.inspect(),paints=new Map<string,{body:Awaited<ReturnType<typeof nativeControlPdfPaint>>;chrome?:Awaited<ReturnType<typeof nativeControlPdfPaint>>}>();
    {
      for(let page=1;page<=pdf.pages.length;page++){const operators=await(printPdfPageV1(pdf,page)).operators,has=(marker:string)=>operators.argsArray.some((args,index)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps].includes(operators.fnArray[index]!)&&printPdfNameV1(args?.[0])===marker);for(const entry of cases)if(has("SemioVizControl"+entry.id)){if(paints.has(entry.id))throw Error("Mixed scientific duplicate actual case "+entry.id);paints.set(entry.id,{body:await nativeControlPdfPaint(operators,control.canvas,"SemioVizControl"+entry.id,probe.control),...(has("SemioVizChrome"+entry.id)?{chrome:await nativeControlPdfPaint(operators,control.canvas,"SemioVizChrome"+entry.id,probe.control)}:{})});}}
      for(const entry of cases)try{
        const actual=paints.get(entry.id);if(!actual)throw Error("Mixed scientific actual case missing");const domain=entry.mode===2?entry.entry.windowDomain:entry.entry.domain,range=entry.mode===2?entry.entry.windowRange:entry.entry.range,x=scaleLinear(domain,[0,width!]),y=scaleLinear(range,[0,height!]),paths=actual.body.paths;
        const match=(values:NativeControlPdfPath[],expected:number[][][],label:string)=>{if(values.length!==expected.length)throw Error(label+" actual path count "+values.length+" expected "+expected.length);const pending=[...values];for(const points of expected){const independent=line()(points as[number,number][]);if(!independent)throw Error("Independent D3 mixed path missing");const numbers=independent.match(/[-+]?(?:\d*\.?\d+)(?:e[-+]?\d+)?/gi)!.map(Number),oracle=Array.from({length:numbers.length/2},(_,index)=>numbers.slice(2*index,2*index+2)),found=pending.findIndex(path=>path.points.length===oracle.length&&path.points.every((point,index)=>point.every((value,axis)=>Math.abs(value-oracle[index]![axis]!)<.035)));if(found<0)throw Error(label+" actual path differs from independent D3 "+JSON.stringify(oracle));pending.splice(found,1);matched++;}};
        if(entry.entry.projection==="orbital"){const mx=scaleLinear(domain,[6,width!-6]);match(paths.filter(path=>!path.closed&&path.points.length===2&&Math.abs(path.bounds[2]!-path.bounds[0]!-12)<.035&&Math.abs(path.bounds[3]!-path.bounds[1]!)<.035),control.levels.map(row=>[[mx(Number(row[0]))-6,y(Number(row[1]))],[mx(Number(row[0]))+6,y(Number(row[1]))]]),"Molecular orbital numeric window");}
        else if(entry.entry.projection==="trajectory")match(paths.filter(path=>!path.closed&&path.points.length===control.events.length&&Math.abs(path.opacity-1)<.001),[control.events.map(row=>[x(Number(row[0])),y(Number(row[1]))])],"Pitch trajectory numeric window");
        else if(entry.entry.projection==="heat"){
          const bins=[...new Set(control.events.map(row=>Math.floor(Number(row[0])*2)+","+Math.floor(Number(row[1])*2)))].map(value=>value.split(",").map(Number)),fills=paths.filter(path=>path.closed&&Math.abs(path.fillOpacity-.7)<.001);if(fills.length!==bins.length)throw Error("Pitch heat bin count");for(const[column,row]of bins)if(!fills.some(path=>path.bounds.every((value,axis)=>Math.abs(value-[x(column!/2),y(row!/2),x((column!+1)/2),y((row!+1)/2)][axis]!)<.035)))throw Error("Pitch heat bins ignore independent D3 numeric window");matched+=fills.length;
        }else{
          const circles=paths.filter(path=>path.closed&&Math.abs(path.fillOpacity-1)<.001&&path.bounds[2]!-path.bounds[0]!<15&&path.bounds[3]!-path.bounds[1]!<15&&(entry.entry.projection!=="events"||Math.abs(path.bounds[2]!-path.bounds[0]!-2.4)<.035)),points=entry.entry.projection==="events"?control.events.map(row=>[Number(row[0]),Number(row[1])]):control.nodes.map(row=>[Number(row[1]),Number(row[2])]);
          if(circles.length!==points.length)throw Error("Pitch event/player circle count "+circles.length+" expected "+points.length);for(const point of points)if(!circles.some(path=>Math.abs((path.bounds[0]!+path.bounds[2]!)/2-x(point[0]!))<.035&&Math.abs((path.bounds[1]!+path.bounds[3]!)/2-y(point[1]!))<.035))throw Error("Pitch event/player centre ignores independent D3 numeric window");matched+=circles.length;
          if(entry.entry.projection==="passes"){const nodes=new Map(control.nodes.map(row=>[String(row[0]),[x(Number(row[1])),y(Number(row[2]))]]));match(paths.filter(path=>!path.closed&&path.points.length===2&&path.width>.1&&Math.abs(path.opacity-1)<.001),control.edges.map(row=>[nodes.get(String(row[0]))!,nodes.get(String(row[1]))!]),"Pitch pass numeric endpoints");}
        }
        if(!actual.chrome)throw Error("Mixed scientific shared chrome missing");if(!entry.mode){if(actual.chrome.paths.length||actual.chrome.glyphs.length)throw Error("Disabled mixed scientific chrome paints");}else{const ticks=entry.mode===1?2:5,grid=actual.chrome.paths.filter(path=>!path.closed&&Math.abs(path.opacity-.3)<.001);if(grid.length!==2*(ticks+1))throw Error("Mixed scientific grid divisions ignored");const text=actual.chrome.glyphs.map(glyph=>glyph.text).join("").replace(/\s/g,"");for(const label of ["Cvtitle","Cvx","Cvy"])if(!text.includes(label))throw Error("Mixed scientific chrome label missing "+label);}
      }catch(error){failures.push(appearance+"/"+entry.entry.kind+"/"+entry.id+": "+String(error));}
    }
  });}
  const previous=process.env.PRINT_NATIVE_CANVAS_REPLAY;if(process.env.PRINT_NATIVE_MIXED_REPLAY)process.env.PRINT_NATIVE_CANVAS_REPLAY=join(process.env.PRINT_NATIVE_MIXED_REPLAY,"physical");
  try{await compileNativeScientificCanvasControls(execution,fixture.scientificMixedPhysicalWindows);}catch(error){failures.push(String(error));}finally{if(previous===undefined)delete process.env.PRINT_NATIVE_CANVAS_REPLAY;else process.env.PRINT_NATIVE_CANVAS_REPLAY=previous;}
  if(JSON.stringify(control)!==original)throw Error("Mixed scientific vectors changed");if(failures.length)throw Error("Mixed scientific control effects:\n"+failures.join("\n"));
});}

/** 🧬️ Checks mode-conditioned defaults with explicit neutral vectors and actual D3 relations. */
export async function compileNativeScientificModeDefaults(execution:PrintExecutionV1,stock=Boolean(process.env.PRINT_NATIVE_MODE_STOCKS)):Promise<void>{if(["PRINT_NATIVE_MODE_REPLAY"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/scientific-mode-defaults",async({directory:workDir,execution})=>{
  const neutral=fixture.scientificModeDefaults,control=neutral.vectors,original=JSON.stringify(control),failures:string[]=[];
  
  const stocks=fixture.scientificStockSemanticVectors;const families=stock?stocks.vectors.cases:control.families, cases=families.flatMap((family,fi)=>family.modes.flatMap((vector,mi)=>[false,true].map(explicit=>({family,vector,explicit,id:"Md"+fi+"M"+mi+(explicit?"X":"O")})))),themes=(["light","dark"] as const).filter(value=>!process.env.PRINT_NATIVE_CONTROL_THEME||value===process.env.PRINT_NATIVE_CONTROL_THEME);
  if(!themes.length)throw Error("Unknown scientific mode theme");let compared=0,relations=0;
  for(const appearance of themes){
    const language=appearance==="light"?"en":"de",body:{raw:string}[]=[],directory=join(workDir,appearance);
    for(const entry of cases){
      const options:Record<string,string|number|boolean>={mode:entry.family.modes[0]!.mode,width:80,height:48,axes:false,grid:false,title:"",xlabel:"",ylabel:"",...(entry.family.family!=="sci-circuit"?{unit:entry.family.unit,nodeShape:entry.family.nodeShape}:{})},initial={chart:{width:control.canvas[0],height:control.canvas[1],language,theme:{appearance},layers:[],presets:[{kind:entry.family.kind,options}]}} as unknown as VizChartSnapshot,before=JSON.stringify(initial);
      let snapshot=initial;const changes:Record<string,string|number|boolean>={mode:entry.vector.mode,...entry.explicit?Object.fromEntries(Object.entries(entry.vector).filter(([key])=>key!=="mode")):{}};
      for(const [key,value]of Object.entries(changes)){const outcome=changeVizChartValue(snapshot,{path:["presets","0","options",key],value});if(outcome.messages.length||outcome.diff.edits.length>1)throw Error(entry.id+" mode mutation refused");const replay=applyVizChartDiff(snapshot,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(snapshot,outcome.diff));if(replay.messages.length||inverse.messages.length)throw Error(entry.id+" mode replay refused");deepStrictEqual(inverse.snapshot,snapshot);snapshot=replay.snapshot;}
      if(JSON.stringify(initial)!==before)throw Error("Mode mutation changed input");const result=await inferVizChart(snapshot);if(!result.complete)throw Error(entry.id+" mode inference refused "+JSON.stringify(result.diagnostics));
      const reference="\\draw[line width=.01mm] (0,0) rectangle ("+control.canvas.join(",")+");\n",emitted=result.tikz.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1"+reference);
      body.push({raw:"\\clearpage\\ExplSyntaxOn\\tl_set:Nn\\l_semio_viz_test_canvas_case_tl{"+entry.id+"}\\int_gzero:N\\g_semio_viz_test_mode_element_int\\ExplSyntaxOff\\SemioVizProbeBegin{scientific-mode-defaults}{"+entry.id+"}\n"+emitted});
    }
    const preamble=[...scientificCanvasProbePreamble(),["\\ExplSyntaxOn","\\int_new:N\\g_semio_viz_test_mode_element_int","\\cs_new_eq:NN\\semio_viz_test_mode_symbol:\\semio_viz_eng_symbol:","\\cs_set_protected:Npn\\semio_viz_eng_symbol:{","\\int_gincr:N\\g_semio_viz_test_mode_element_int","\\use:x{\\exp_not:N\\special{pdf:literal~direct~/SemioVizElement\\tl_use:N\\l_semio_viz_test_canvas_case_tl E\\int_use:N\\g_semio_viz_test_mode_element_int\\c_space_tl BMC}}","\\semio_viz_test_mode_symbol:\\special{pdf:literal~direct~EMC}}","\\ExplSyntaxOff"].join("\n")];
    await withCompiledVizProbeDocumentV1({case:"scientific-mode-defaults",scenario:appearance,geometry:true,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],preamble,body},{execution,scenario:undefined},async({records:records,probe})=>{
    const pdfDirectory=process.env.PRINT_NATIVE_MODE_REPLAY?join(process.env.PRINT_NATIVE_MODE_REPLAY,appearance):directory,pdf=await probe.inspect(),paints=new Map<string,Awaited<ReturnType<typeof nativeControlPdfPaint>>>(),elements=new Map<string,Awaited<ReturnType<typeof nativeControlPdfPaint>>>();
    {for(let page=1;page<=pdf.pages.length;page++){const operators=await(printPdfPageV1(pdf,page)).operators,names=operators.argsArray.filter((_,index)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps].includes(operators.fnArray[index]!)).map(args=>printPdfNameV1(args?.[0]));for(const entry of cases){const marker="SemioVizBody"+entry.id;if(names.includes(marker)){if(paints.has(entry.id))throw Error("Duplicate native mode body");paints.set(entry.id,await nativeControlPdfPaint(operators,control.canvas,marker,probe.control));}for(const name of names.filter(name=>name.startsWith("SemioVizElement"+entry.id+"E")))elements.set(name,await nativeControlPdfPaint(operators,control.canvas,name,probe.control));}}}
    await Bun.write(join(directory,"actual-owned-paint.json"),JSON.stringify({bodies:Object.fromEntries(paints),elements:Object.fromEntries(elements)},null,2));
    const same=(a:Awaited<ReturnType<typeof nativeControlPdfPaint>>,b:Awaited<ReturnType<typeof nativeControlPdfPaint>>,label:string)=>{
      if(a.paths.length!==b.paths.length||a.glyphs.length!==b.glyphs.length)throw Error(label+" default paint inventory differs from explicit neutral vector");
      for(const [index,path]of a.paths.entries()){const expected=b.paths[index]!;if(path.paint!==expected.paint||path.closed!==expected.closed||path.points.length!==expected.points.length)throw Error(label+" default path topology differs");equal(path.points.flat(),expected.points.flat(),label+" actual path",.025);equal([path.width,path.opacity,path.fillOpacity,...path.fill,...path.stroke],[expected.width,expected.opacity,expected.fillOpacity,...expected.fill,...expected.stroke],label+" paint state",.001);}
      for(const [index,glyph]of a.glyphs.entries()){const expected=b.glyphs[index]!;if(glyph.text!==expected.text)throw Error(label+" default literal glyph differs");equal([...(glyph.origin??[]),...(glyph.advance??[])],[...(expected.origin??[]),...(expected.advance??[])],label+" actual glyph",.025);}
    };
    for(const entry of cases.filter(entry=>!entry.explicit))try{
      const paint=paints.get(entry.id),target=paints.get(entry.id.slice(0,-1)+"X");if(!paint||!target)throw Error("Missing native mode body");same(paint,target,entry.id);compared++;
    }catch(error){failures.push(appearance+"/"+entry.id+": "+String(error));}
    for(const entry of cases.filter(entry=>entry.explicit))try{
      const paint=paints.get(entry.id);if(!paint)throw Error("Missing explicit native mode body");
      if("nodes" in entry.vector&&"edges" in entry.vector){
        const nodes=new Map(String(entry.vector.nodes).split(",").filter(Boolean).map(record=>{const[id,x,y,label]=record.split("/");return[id!,{point:[Number(x)*entry.family.unit,Number(y)*entry.family.unit],label:label!}] as const;})),edges=String(entry.vector.edges).split(",").filter(Boolean).map(record=>record.split("/")),incidence=d3Group(edges,edge=>edge[0]!);
        for(const [from,group]of incidence)if(!nodes.has(from)||group.some(edge=>!nodes.has(edge[1]!)))throw Error("Independent D3 incidence has dangling endpoint");
        for(const [from,to,style]of edges){const source=nodes.get(from!)!.point,target=nodes.get(to!)!.point,path=d3Path();path.moveTo(...source as[number,number]);path.lineTo(...target as[number,number]);if(!path.toString())throw Error("Independent D3 mode relation empty");const segment=paint.paths.find(actual=>!actual.closed&&actual.points.length===2&&Math.hypot(actual.points[0]![0]!-source[0]!,actual.points[0]![1]!-source[1]!)<.04&&Math.hypot(actual.points[1]![0]!-target[0]!,actual.points[1]![1]!-target[1]!)<(["plain","wire","leg","order"].includes(style!)?.04:1.8));if(!segment)throw Error("Mode relation ignores independent D3 endpoints "+from+"/"+to+"/"+style);relations++;}
        for(const node of nodes.values())if(node.label){const glyph=paint.glyphs.find(value=>value.text===node.label&&value.origin&&value.advance&&Math.abs(value.origin[0]!+value.advance[0]!/2-node.point[0])<.12);if(!glyph?.origin||!glyph.advance)throw Error("Mode node literal glyph missing "+node.label);equal([glyph.origin[0]!+glyph.advance[0]!/2],[node.point[0]!],"D3 node horizontal centre",.12);}
        if(entry.vector.mode==="dynkin"&&entry.family.family==="sci-abstract-diagram"&&paint.paths.some(path=>!path.closed&&path.points.length===3&&path.bounds[3]!-path.bounds[1]!>.1))throw Error("Undirected default Dynkin bond has an arrowhead");
      }else if("elements" in entry.vector){
        const records=String(entry.vector.elements).split(",").filter(Boolean).map(record=>record.split("/"));
        for(const [index,record]of records.entries()){const[type,x1,y1,x2,y2]=record,start=[Number(x1),Number(y1)],end=[Number(x2),Number(y2)],element=elements.get("SemioVizElement"+entry.id+"E"+(index+1));if(!element)throw Error("Missing actual circuit element "+index);const dx=end[0]!-start[0]!,dy=end[1]!-start[1]!,length=Math.hypot(dx,dy),projection=(point:number[])=>((point[0]!-start[0]!)*dx+(point[1]!-start[1]!)*dy)/length;
          if(["and","or","xor","nand"].includes(type!)){const endpoints=element.paths.flatMap(path=>path.points).filter(point=>Math.abs((point[0]!-start[0]!)*dy-(point[1]!-start[1]!)*dx)<.05*length);if(!endpoints.some(point=>Math.abs(projection(point)-length)<.04))failures.push(appearance+"/"+entry.id+"/E"+(index+1)+": Logic gate output terminal disconnected "+type);}
          if(type==="contact"&&(element.paths.some(path=>path.closed)||element.paths.filter(path=>path.points.length===2&&Math.abs(path.bounds[3]!-path.bounds[1]!-4)<.04).length!==2))failures.push(appearance+"/"+entry.id+"/E"+(index+1)+": Ladder contact is not two separated contact bars");
          if(type==="coil"&&(element.paths.some(path=>path.closed)||element.paths.filter(path=>!path.closed&&path.points.length>20).length!==2))failures.push(appearance+"/"+entry.id+"/E"+(index+1)+": Ladder coil is not paired coil arcs");
          if(type==="signal"&&!element.paths.some(path=>!path.closed&&path.points.length===3&&Math.abs(Math.max(...path.points.map(projection))-length)<path.width+.05&&Math.abs(Math.max(...path.points.map(projection))-Math.min(...path.points.map(projection))-1.1)<.04))failures.push(appearance+"/"+entry.id+"/E"+(index+1)+": Signal connector has no declared target head");
        }
      }
    }catch(error){failures.push(appearance+"/"+entry.id+": "+String(error));}
  });}
  if(JSON.stringify(control)!==original)throw Error("Mode neutral vector changed");if(failures.length)throw Error("Scientific mode defaults:\n"+failures.join("\n"));
  if(!stock)await compileNativeScientificModeDefaults(execution,true);
});}


/** 🧾️ Adjudicates closed scientific mode and resolved record contracts through independent AJV and D3 incidence before canonical admission. */
export async function compileNativeScientificAdmission(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/scientific-admission",async({directory:workDir,execution})=>{
  const neutral=fixture.nativeScientificAdmission,before=JSON.stringify(neutral),failures:string[]=[],ajv=new Ajv2020({strict:false}),expression={anyOf:[{type:"number"},{type:"string",minLength:1,pattern:"^(?![+-]?(?:[iI][nN][fF](?:[iI][nN][iI][tT][yY])?|[nN][aA][nN])$).+"}]},identifier={type:"string",minLength:1,pattern:"\\S"},text={type:"string"},row=(fields:unknown[])=>({type:"array",minItems:fields.length,maxItems:fields.length,prefixItems:fields,items:false});
  mkdirSync(workDir,{recursive:true});const results:unknown[]=[];
  for(const family of neutral.families){const options=chartSchema["x-semio-family-options"][family.name as "sci-pathway"].options as Record<string,{enum?:string[];recordFields?:{enum?:string[]}[];defaultsByMode?:Record<string,string>}>;deepStrictEqual(options.mode?.enum,family.modes);if(family.styles.length){deepStrictEqual(options.nodes?.recordFields?.length,4);deepStrictEqual(options.edges?.recordFields?.length,4);deepStrictEqual(options.edges?.recordFields?.[2]?.enum,family.styles);}if(family.types.length){deepStrictEqual(options.elements?.recordFields?.length,6);deepStrictEqual(options.elements?.recordFields?.[0]?.enum,family.types);}}
  for(const family of fixture.scientificModeDefaults.vectors.families){const options=chartSchema["x-semio-family-options"][family.family as "sci-pathway"].options as Record<string,{defaultsByMode?:Record<string,string>}>;for(const mode of family.modes)for(const key of ["nodes","edges","elements"]as const)if(key in mode)deepStrictEqual(options[key]?.defaultsByMode?.[mode.mode],(mode as unknown as Record<string,string>)[key]);}
  for(const entry of neutral.cases){
    const family=neutral.families.find(family=>family.name===entry.family)!;if(!family)throw Error(entry.id+" unknown neutral owner");
    const schema={type:"object",required:["mode","nodes","edges","elements"],additionalProperties:false,properties:{mode:{enum:family.modes},nodes:{type:"array",items:row([identifier,expression,expression,text])},edges:{type:"array",items:row([identifier,identifier,family.styles.length?{enum:family.styles}:{not:{}},text])},elements:{type:"array",items:row([family.types.length?{enum:family.types}:{not:{}},expression,expression,expression,expression,text])}}},validate=ajv.compile(schema),structural=validate(entry.oracle),groups=d3Group(entry.oracle.nodes,node=>node[0]),ids=new Set(entry.oracle.nodes.map(node=>node[0])),finite=(value:unknown)=>typeof value==="number"?Number.isFinite(value):typeof value==="string"&&value.trim()!==""&&(!/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/.test(value.trim())||Number.isFinite(Number(value))),numeric=entry.oracle.nodes.every(node=>finite(node[1])&&finite(node[2]))&&entry.oracle.elements.every(element=>element.slice(1,5).every(finite)),incidence=[...groups.values()].every(nodes=>nodes.length===1)&&entry.oracle.edges.every(edge=>ids.has(edge[0])&&ids.has(edge[1])),expected=Boolean(structural&&numeric&&incidence);
    if(expected!==entry.valid)throw Error(entry.id+" independent neutral oracle mismatch "+JSON.stringify(validate.errors));
    for(const language of ["en","de"]as const){const options={...entry.options,mode:entry.options.mode===family.modes[0]?family.modes[1]:family.modes[0]},base={chart:{...neutral.snapshot.chart,language,presets:[{kind:entry.kind,options}]}}as unknown as VizChartSnapshot,original=JSON.stringify(base),outcome=changeVizChartValue(base,{path:["presets","0","options","mode"],value:entry.options.mode});if(outcome.messages.length||outcome.diff.edits.length!==1)throw Error(entry.id+" admission mutation refused");const replay=applyVizChartDiff(base,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(base,outcome.diff));deepStrictEqual(inverse.snapshot,base);if(replay.messages.length||inverse.messages.length||JSON.stringify(base)!==original)throw Error(entry.id+" admission inverse/purity");const result=await inferVizChart(replay.snapshot);results.push({id:entry.id,language,valid:entry.valid,complete:result.complete,diagnostics:result.diagnostics});if(result.complete!==entry.valid)failures.push(entry.id+" "+language+" admitted="+result.complete+" expected="+entry.valid);if(entry.valid&&result.complete)await Bun.write(join(workDir,entry.id+"-"+language+".tex"),result.tikz);if(!result.complete&&(result.tikz||!result.diagnostics.length))throw Error(entry.id+" rejected admission has partial/undiagnosed output");}
  }
  await Bun.write(join(workDir,"admission-results.json"),JSON.stringify(results,null,2));if(JSON.stringify(neutral)!==before)throw Error("Admission neutral changed");if(failures.length)throw Error("Scientific canonical admission:\n"+failures.join("\n"));
});}

/** 🆚️ Compares specialized placement and fabric permutations with actual D3 closed-form and force outputs. */
export async function compileNativeSpecializedPlacementControls(execution:PrintExecutionV1,owner?:string):Promise<void>{if(["PRINT_NATIVE_PLACEMENT_REPLAY"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/specialized-placement-controls",async({directory:workDir,execution})=>{
  if(process.env.PRINT_NATIVE_PLACEMENT_PHASE==="guard"){await compileNativeSpecializedPlacementAdmission(execution);return;}
  if(!owner&&!process.env.PRINT_NATIVE_PLACEMENT_FAMILY&&!process.env.PRINT_NATIVE_PLACEMENT_REPLAY){for(const family of ["biofabric","data-structure","commit-graph"]){await compileNativeSpecializedPlacementControls(execution,family);}await compileNativeSpecializedPlacementAdmission(execution);return;}
  const contract=fixture.nativeSpecializedPlacementControls,control={...contract.vectors,cases:contract.vectors.cases.filter(entry=>(!process.env.PRINT_NATIVE_PLACEMENT_BASELINE||entry.layout!=="force")&&(!process.env.PRINT_NATIVE_PLACEMENT_CASE||entry.id===process.env.PRINT_NATIVE_PLACEMENT_CASE))},{forceSimulation,forceLink,forceManyBody,forceCenter}=await import("d3-force"),{randomLcg}=await import("d3-random"),{pointRadial}=await import("d3-shape"),{default:dagre}=await import("dagre");const failures:string[]=[],oracles:unknown[]=[],families=["biofabric","data-structure","commit-graph"].filter(family=>!(owner??process.env.PRINT_NATIVE_PLACEMENT_FAMILY)||family===(owner??process.env.PRINT_NATIVE_PLACEMENT_FAMILY));
  const oracle=(entry:typeof control.cases[number],family:string)=>{const ids=control.nodes.map(row=>row.id),edges=family==="data-structure"?control.nodes.flatMap((row,index)=>{const parent=entry.parents.length?entry.parents[index]!:row.parent;return parent?[[parent,row.id]]:[]}):(entry.edges.length?entry.edges:control.edges),points=ids.map((id,index)=>({id,x:entry.initial.length?entry.initial[index]![0]!:NaN,y:entry.initial.length?entry.initial[index]![1]!:NaN})),order=entry.order==="name"?[...ids].sort():ids;
    if(entry.layout==="force"){const simulation=forceSimulation(points).stop().randomSource(randomLcg(entry.seed));for(const name of entry.forces)simulation.force(name,name==="link"?forceLink(edges.map(([source,target])=>({source:source!,target:target!}))).id(node=>String((node as typeof points[number]).id)).distance(entry.linkDistance):name==="many-body"?forceManyBody().strength(entry.charge).theta(0):forceCenter(0,0));simulation.tick(entry.iterations);}
    else if(entry.layout==="random"){const random=randomLcg(entry.seed);for(const point of points){point.x=random();point.y=random();}}
    else if(entry.layout==="partite"){const groups=d3Group(points,point=>entry.groups.length?entry.groups[ids.indexOf(point.id)]!:control.nodes[ids.indexOf(point.id)]!.group);for(const[groupIndex,rows]of [...groups.values()].entries())for(const[index,point]of rows.entries()){point.x=groupIndex+1;point.y=-index-1;}}
    else if(entry.layout==="radial"){const graph=new dagre.graphlib.Graph({directed:false});for(const id of ids)graph.setNode(id,{});for(const[source,target]of edges)graph.setEdge(source!,target!);const distances=dagre.graphlib.alg.dijkstra(graph,ids[entry.root-1]!,()=>1,id=>edges.filter(([source,target])=>source===id||target===id).map(([source,target])=>({v:source!,w:target!,points:[]}))),rings=d3Group(points,point=>distances[point.id]!.distance);for(const[radius,rows]of rings)for(const[index,point]of rows.entries()){const xy=pointRadial(Math.PI/2+2*Math.PI*index/rows.length,radius);point.x=xy[0];point.y=xy[1];}}
    else if(entry.layout==="layered"){const graph=new dagre.graphlib.Graph().setGraph({ranker:"longest-path",rankdir:"LR",nodesep:1,ranksep:1}).setDefaultEdgeLabel(()=>({}));for(const id of ids)graph.setNode(id,{width:0,height:0});for(const[source,target]of edges)graph.setEdge(source!,target!);dagre.layout(graph);const ranks=[...new Set(ids.map(id=>Number(graph.node(id).x)))].sort((a,b)=>a-b),layers=d3Group(points,point=>ranks.indexOf(Number(graph.node(point.id).x)));for(const[layer,rows]of layers){if(entry.sweeps)rows.sort((a,b)=>Number(graph.node(a.id).y)-Number(graph.node(b.id).y));for(const[index,point]of rows.entries()){const cross=index-(rows.length-1)/2;point.x=entry.orientation==="horizontal"?layer:cross;point.y=entry.orientation==="horizontal"?cross:-layer;}}}
    else for(const[rank,id]of order.entries()){const point=points.find(point=>point.id===id)!;if(entry.layout==="grid"){point.x=rank%entry.columns;point.y=-Math.floor(rank/entry.columns);}else{const xy=pointRadial(Math.PI/2+2*Math.PI*rank/ids.length,1);point.x=xy[0];point.y=xy[1];}}return points;};
  for(const[language,appearance]of ([["en","light"],["de","dark"]]as const).filter(([,appearance])=>!process.env.PRINT_NATIVE_PLACEMENT_THEME||appearance===process.env.PRINT_NATIVE_PLACEMENT_THEME)){const directory=join(workDir,appearance),body:{raw:string}[]=[];
    for(const family of families)for(const entry of control.cases){const force=entry.layout==="force",numeric=(key:string,value:number)=>chartSchema["x-semio-family-options"][family as "biofabric"].options[key as "padding"]?.type==="string"?String(value):value,options:Record<string,string|number|boolean>={data:family==="data-structure"?"placement-nodes":"placement-edges",group:"group",label:"id",labels:true,padding:numeric("padding",3),size:numeric("size",control.size),layout:entry.layout,order:entry.order,seed:numeric("seed",entry.seed),root:numeric("root",entry.root),sweeps:numeric("sweeps",entry.sweeps),orientation:entry.orientation,...(force?{iterations:numeric("iterations",entry.iterations),forces:entry.forces.join(","),charge:numeric("charge",entry.charge),linkDistance:numeric("linkDistance",entry.linkDistance)}:{columns:numeric("columns",entry.columns)})};if(family==="data-structure")Object.assign(options,{idColumn:"id",parentColumn:"parent",valueColumn:"value",mode:"bst"});else Object.assign(options,{nodes:"placement-nodes",id:"id",group:"group",source:"source",target:"target",...(family==="biofabric"?{representation:"fabric"}:{colorBy:"group"})});const key=entry.layout==="random"?"seed":entry.layout==="partite"?"group":entry.layout==="radial"?"root":entry.layout==="layered"?(entry.id==="layered-vertical"?"orientation":"sweeps"):entry.id.startsWith("grid")?"columns":entry.id.startsWith("circle")?"order":entry.id==="force-iterations"?"iterations":entry.id==="force-forces"?"forces":entry.id==="force-charge"?"charge":entry.id==="force-distance"?"linkDistance":entry.id.startsWith("force-seed")?"seed":"layout",value=options[key]!;if(value===undefined)throw Error("Placement target value absent: "+entry.id+"/"+key);options[key]=key==="group"?"other":key==="root"?numeric("root",entry.root===1?4:1):key==="sweeps"?numeric("sweeps",entry.sweeps===0?4:0):key==="orientation"?(entry.orientation==="horizontal"?"vertical":"horizontal"):key==="columns"?numeric("columns",entry.columns===2?3:2):key==="order"?(entry.order==="index"?"name":"index"):key==="iterations"?numeric("iterations",24):key==="forces"?"link,many-body,center":key==="charge"?numeric("charge",-30):key==="linkDistance"?numeric("linkDistance",30):key==="seed"?numeric("seed",entry.seed===1?1000:1):"grid";const base={chart:{language,theme:{appearance},width:80,height:40,margin:{top:0,right:0,bottom:0,left:0},layers:[],tables:[{name:"placement-nodes",columns:["id","parent","group","other","value",...(entry.initial.length?["x","y"]:[])],rows:control.nodes.map((row,index)=>({...row,parent:entry.parents.length?entry.parents[index]!:row.parent,group:entry.groups.length?entry.groups[index]!:row.group,other:row.group,...(entry.initial.length?{x:entry.initialType==="string"?String(entry.initial[index]![0]!):entry.initial[index]![0]!,y:entry.initialType==="string"?String(entry.initial[index]![1]!):entry.initial[index]![1]!}:{})}))},{name:"placement-edges",columns:["source","target"],rows:(entry.edges.length?entry.edges:control.edges).map(row=>({source:row[0]!,target:row[1]!}))}],presets:[{kind:family==="biofabric"?"biofabric-style-graph":family==="data-structure"?"binary-search-tree":"commit-graph",options}]}}as unknown as VizChartSnapshot,outcome=changeVizChartValue(base,{path:["presets","0","options",key],value}),replay=applyVizChartDiff(base,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(base,outcome.diff));if(outcome.messages.length||outcome.diff.edits.length!==1||replay.messages.length||inverse.messages.length)throw Error("Placement canonical mutation/replay refused");deepStrictEqual(inverse.snapshot,base);const inferred=await inferVizChart(replay.snapshot);if(inferred.diagnostics.some(diagnostic=>diagnostic.code!=="print.chart.scene-unavailable"))throw Error(family+"/"+entry.id+" placement emission rejected "+JSON.stringify(inferred.diagnostics));if(!inferred.tikz.includes(key+"={"+value+"}")&&!(key==="forces"&&inferred.tikz.includes(key+"={"+String(value).split(",").map(item=>"{"+item+"}").join(",")+"}")))throw Error("Placement native target option absent: "+entry.id+"/"+key+" expected "+String(value)+"\n"+inferred.tikz);const id=family+"-"+entry.id;body.push({raw:"\\clearpage\\SemioVizProbeBegin{specialized-placement-controls}{"+id+"}\n"+inferred.tikz.replace(/(\\SemioVizTheme\{default\}\[[^\]]*\])/g,"$1\\SemioVizThemeSet[colors={"+control.palette.map((_,index)=>"placement-color"+index).join(",")+"}]").replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1\\draw[line width=.01mm] (0,0) rectangle (80,40);\\special{pdf:literal direct /SemioVizControl"+id+" BMC}\n").replace("\\end{VizFigure}","\\special{pdf:literal direct EMC}\\end{VizFigure}")});oracles.push({language,appearance,family,id,raw:oracle(entry,family)});}
    await withCompiledVizProbeDocumentV1({case:"specialized-placement-controls",scenario:appearance,documentClass:"semio",documentClassOptions:`type=paper,language=${language},theme=${appearance}`,packages:["semio-viz"],geometry:true,preamble:["\\errorcontextlines=200","\\title{Specialized Placement}","\\author{Semio}","\\date{}",...control.palette.map((color,index)=>"\\definecolor{placement-color"+index+"}{HTML}{"+color.slice(1)+"}")],body:[...body,{values:{key:"native/placement",clist:"1"}}]},{execution,scenario:undefined},async({records:records,probe})=>{const out=join(directory,"🧪️probe-out"),name=basename(probe.pdfPath);if(!name)throw Error("Specialized placement produced no actual PDF");const pdf=await probe.inspect(),paints=new Map<string,Awaited<ReturnType<typeof nativeControlPdfPaint>>>();{for(let page=1;page<=pdf.pages.length;page++){const operators=await(printPdfPageV1(pdf,page)).operators;for(const family of families)for(const entry of control.cases){const id=family+"-"+entry.id;if(operators.argsArray.some((args,index)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps].includes(operators.fnArray[index]!)&&printPdfNameV1(args?.[0])==="SemioVizControl"+id))paints.set(id,await nativeControlPdfPaint(operators,[80,40],"SemioVizControl"+id,probe.control));}}}
    for(const family of families)for(const entry of control.cases)try{const id=family+"-"+entry.id,paint=paints.get(id);if(!paint)throw Error("Actual owned placement body missing");const points=oracle(entry,family);if(family==="biofabric"){const permutation=[...points].sort((a,b)=>Number(a.x.toFixed(12))-Number(b.x.toFixed(12))||Number(a.y.toFixed(12))-Number(b.y.toFixed(12))||a.id.localeCompare(b.id)),band=scaleBand<string>().domain(permutation.map(point=>point.id)).range([37,3]);for(const[rowIndex,row]of control.nodes.entries()){const color=rgb(control.palette[rowIndex]!),glyph=paint.glyphs.find(glyph=>glyph.text===row.id),path=paint.paths.find(path=>!path.closed&&path.points.length===2&&Math.abs(path.bounds[2]!-path.bounds[0]!-65)<.04&&(entry.groups.length?!!glyph?.origin&&Math.abs(path.bounds[1]!-glyph.origin[1]!)<2:path.stroke.every((value,index)=>Math.abs(value-[color.r,color.g,color.b][index]!/255)<.004)));if(!path)throw Error(row.id+" actual identified fabric row missing");equal([path.bounds[1]!],[band(row.id)!+band.bandwidth()/2],id+" independent D3 layout-derived permutation for "+row.id,control.toleranceMm);}}
    else{const margin=2.6*control.size,xmin=Math.min(...points.map(point=>point.x)),xmax=Math.max(...points.map(point=>point.x)),ymin=Math.min(...points.map(point=>point.y)),ymax=Math.max(...points.map(point=>point.y)),scale=Math.min((74-2*margin)/Math.max(1e-6,xmax-xmin),(34-2*margin)/Math.max(1e-6,ymax-ymin)),x=scaleLinear([xmin,xmax],[(80-scale*(xmax-xmin))/2,(80+scale*(xmax-xmin))/2]),y=scaleLinear([ymin,ymax],[(40-scale*(ymax-ymin))/2,(40+scale*(ymax-ymin))/2]),circles=paint.paths.filter(path=>path.closed&&path.points.length>20&&Math.abs(path.bounds[2]!-path.bounds[0]!-2*control.size)<.04);if(circles.length!==points.length)throw Error(id+" actual node inventory "+circles.length);for(const[index,point]of points.entries()){const circle=circles[index]!;equal([(circle.bounds[0]!+circle.bounds[2]!)/2,(circle.bounds[1]!+circle.bounds[3]!)/2],[x(point.x),y(point.y)],id+" independent D3 node "+point.id,control.toleranceMm);const glyph=paint.glyphs.find(glyph=>glyph.text===point.id);if(!glyph?.origin||!glyph.advance)throw Error(id+" stable ID caption missing");equal([glyph.origin[0]!+glyph.advance[0]!/2],[x(point.x)],id+" actual stable caption x for "+point.id,control.toleranceMm);}}
    }catch(error){failures.push(language+"/"+appearance+" "+family+"/"+entry.id+" "+String(error));}
  });}
  mkdirSync(workDir,{recursive:true});await Bun.write(join(workDir,"placement-oracle.json"),JSON.stringify(oracles,null,2));if(failures.length)throw Error("Specialized placement controls:\n"+failures.join("\n"));
});}

/** 🛡️ Checks native scientific record domains against AJV rows and D3 incidence. */
export async function compileNativeScientificRecordDomain(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/scientific-record-domain",async({directory:workDir,execution})=>{
  const neutral=fixture.nativeScientificRecordDomain,control=neutral.vectors,before=JSON.stringify(control),failures:string[]=[];
  
  const hard=async(appearance:"light"|"dark",entries:typeof control.hardCases,label:string)=>{
    const directory=join(workDir,label,appearance),language=appearance==="light"?"en":"de";
    {const refusal=await inspectVizProbeDocumentRefusalV1({case:"scientific-record-domain",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],preamble:["\\title{Scientific Record Domain}","\\author{Semio}","\\date{}"],body:[...entries.map(entry=>({raw:"\\begin{VizFigure}[width=80,height=48]\\SemioVizRunFamily{"+entry.family+"}[axes=false,grid=false,"+entry.options+"]\\end{VizFigure}"})),{values:{key:"native/accepted",clist:"1"}}]},{execution,scenario:undefined});const log=refusal.log;if(!log.includes("Scientific record domain"))throw Error("Unexpected actual compiler diagnostic: "+log);}
    
  };
  if(process.env.PRINT_NATIVE_RECORD_PHASE==="baseline"){await hard("light",control.hardCases,"baseline");return;}
  const ajv=new Ajv2020({strict:false}),row=(fields:unknown[])=>({type:"array",minItems:fields.length,maxItems:fields.length,prefixItems:fields,items:false}),id={type:"string",minLength:1},number={type:"number"},text={type:"string"};
  for(const entry of control.cases){
    const family=control.families.find(family=>family.name===entry.family)!;
    const schema={type:"object",required:["nodes","edges","elements","unit","nodeShape","symbolSize"],additionalProperties:false,properties:{nodes:{type:"array",items:row([id,number,number,text])},edges:{type:"array",items:row([id,id,family.styles.length?{enum:family.styles}:{not:{}},text])},elements:{type:"array",items:row([family.types.length?{enum:family.types}:{not:{}},number,number,number,number,text])},unit:{type:"number",exclusiveMinimum:0},symbolSize:{type:"number",exclusiveMinimum:0},nodeShape:{enum:entry.family==="sci-pathway"?["box","circle"]:["none","box","circle"]}}};
    const valid=ajv.compile(schema)(entry.oracle),groups=d3Group(entry.oracle.nodes,node=>node[0]),ids=new Set(entry.oracle.nodes.map(node=>node[0])),incidence=[...groups.values()].every(group=>group.length===1)&&entry.oracle.edges.every(edge=>ids.has(edge[0])&&ids.has(edge[1]));
    if(Boolean(valid&&incidence)!==entry.valid)throw Error(entry.id+" independent native domain oracle mismatch");
  }
  for(const appearance of (["light","dark"]as const).filter(value=>!process.env.PRINT_NATIVE_CONTROL_THEME||value===process.env.PRINT_NATIVE_CONTROL_THEME)){
    const body=control.cases.map(entry=>{const family=control.families.find(family=>family.name===entry.family)!;return{raw:"\\SemioVizProbeBegin{scientific-record-domain}{"+entry.id+"}\\ExplSyntaxOn\\semio_viz_sci_enter:nnn{"+family.name+"}{"+family.defaults+"}{"+entry.options+"}\\"+family.defaultsMacro+"\\"+family.validateMacro+"\\semio_viz_probe_values:nx{native/valid}{\\bool_if:NTF\\l_semio_viz_sci_records_valid_bool{1}{0}}\\ExplSyntaxOff"};});
    const records=await compileVizProbeDocument({case:"scientific-record-domain",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language="+(appearance==="light"?"en":"de")+",theme="+appearance,packages:["semio-viz"],preamble:["\\title{Scientific Record Predicates}","\\author{Semio}","\\date{}","\\ExplSyntaxOn\\msg_redirect_name:nnn{semio-viz-sci}{invalid-record}{warning}\\ExplSyntaxOff"],body},{execution,scenario:undefined});
    for(const entry of control.cases)try{equal(records.find(record=>record.scenario===entry.id&&record.key==="native/valid")!.values.map(Number),[entry.valid?1:0],entry.id+" native evaluated validity");}catch(error){failures.push(appearance+"/"+String(error));}
    for(const entry of control.hardCases)try{await hard(appearance,[entry],entry.id);}catch(error){failures.push(appearance+"/"+String(error));}
  }
  if(JSON.stringify(control)!==before)throw Error("Native scientific neutral changed");if(failures.length)throw Error("Scientific native domains:\n"+failures.join("\n"));
  await compileNativeScientificScalarAdmission();
  await compileNativeScientificModeDomain(execution);
});}

/** 🪝️ Checks every mathematical relation style using actual PDF state and D3 endpoints. */
export async function compileNativeScientificRelationStyles(execution:PrintExecutionV1):Promise<void>{if(["PRINT_NATIVE_STYLE_REPLAY"].some(key=>process.env[key]!==undefined))throw Error("Actual owned probe required; environment path replay refused");return withPrintScratchV1(execution,"native/scientific-relation-styles",async({directory:workDir,execution})=>{
  const neutral=fixture.scientificRelationStyles,control=neutral.vectors,original=JSON.stringify(control),failures:string[]=[];
  
  for(const appearance of (["light","dark"]as const).filter(value=>!process.env.PRINT_NATIVE_CONTROL_THEME||value===process.env.PRINT_NATIVE_CONTROL_THEME)){
    const language=appearance==="light"?"en":"de",directory=join(workDir,appearance),body:{raw:string}[]=[];
    for(const entry of control.cases){
      const options={mode:"commutative",unit:control.unit,nodeShape:"none",nodes:"a/"+control.origin[0]+"/"+control.origin[1]+"/,b/"+(control.origin[0]!+1)+"/"+control.origin[1]+"/",edges:"a/b/"+(entry.style==="arrow"?"wire":"arrow")+"/",width:80,height:48,axes:false,grid:false,title:"",xlabel:"",ylabel:""},base={chart:{width:control.canvas[0],height:control.canvas[1],language,theme:{appearance},layers:[],presets:[{kind:"commutative-diagram",options}]}}as unknown as VizChartSnapshot,before=JSON.stringify(base),outcome=changeVizChartValue(base,{path:["presets","0","options","edges"],value:"a/b/"+entry.style+"/"}),replay=applyVizChartDiff(base,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(base,outcome.diff));
      if(outcome.messages.length||outcome.diff.edits.length!==1||replay.messages.length||inverse.messages.length||JSON.stringify(base)!==before)throw Error("Relation style mutation refused");deepStrictEqual(inverse.snapshot,base);const result=await inferVizChart(replay.snapshot);if(!result.complete)throw Error("Relation style inference refused");
      body.push({raw:"\\clearpage\\ExplSyntaxOn\\tl_set:Nn\\l_semio_viz_test_canvas_case_tl{Rs"+entry.style+"}\\ExplSyntaxOff\\SemioVizProbeBegin{scientific-relation-styles}{"+entry.style+"}\n"+result.tikz.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1\\draw[line width=.01mm] (0,0) rectangle ("+control.canvas.join(",")+");\n")});
    }
    await withCompiledVizProbeDocumentV1({case:"scientific-relation-styles",scenario:appearance,geometry:true,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],preamble:scientificCanvasProbePreamble(),body},{execution,scenario:undefined},async({records:records,probe})=>{
    const source=process.env.PRINT_NATIVE_STYLE_REPLAY?join(process.env.PRINT_NATIVE_STYLE_REPLAY,appearance):directory,pdf=await probe.inspect(),paints=new Map<string,Awaited<ReturnType<typeof nativeControlPdfPaint>>>();
    {for(let page=1;page<=pdf.pages.length;page++){const operators=await(printPdfPageV1(pdf,page)).operators;for(const entry of control.cases){const marker="SemioVizBodyRs"+entry.style;if(operators.argsArray.some((args,index)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps].includes(operators.fnArray[index]!)&&printPdfNameV1(args?.[0])===marker))paints.set(entry.style,await nativeControlPdfPaint(operators,control.canvas,marker,probe.control));}}}
    await Bun.write(join(directory,"actual-owned-styles.json"),JSON.stringify(Object.fromEntries(paints),null,2));
    for(const entry of control.cases)try{
      const actual=paints.get(entry.style);if(!actual)throw Error("Missing actual style body");if(actual.paths.some(path=>path.closed)!==control.emptyNodePaint||actual.glyphs.length)throw Error("Empty unshaped nodes paint opaque covers or glyphs");if(actual.paths.some(path=>path.bounds.some((value,axis)=>value<0||value>control.canvas[axis%2]!)))throw Error("Relation path is outside the visible authored frame");const origin=control.origin.map(value=>value*control.unit),paint={...actual,paths:actual.paths.map(path=>({...path,points:path.points.map(point=>point.map((value,axis)=>value-origin[axis]!)),bounds:path.bounds.map((value,axis)=>value-origin[axis%2]!)}))};const line=d3Path();line.moveTo(0,0);line.lineTo(control.unit,0);if(line.toString()!=="M0,0L"+control.unit+",0")throw Error("Independent D3 line changed");
      const shafts=paint.paths.filter(path=>!path.closed&&path.points.length===2&&Math.abs(path.bounds[1]!)<.02&&Math.abs(path.bounds[3]!)<.02),heads=paint.paths.filter(path=>!path.closed&&path.points.length===3&&path.bounds[3]!-path.bounds[1]!>.5),hooks=paint.paths.filter(path=>!path.closed&&path.points.length>20&&path.bounds[0]!<1.2&&path.bounds[2]!<1.3);
      if(!shafts.length)throw Error("D3 relation shaft missing");if(entry.double?shafts.length!==2:shafts.length!==1)throw Error("Declared relation shaft multiplicity differs "+shafts.length);
      for(const shaft of shafts){if(!entry.hook)equal([shaft.points[0]![0]!],[0],"D3 source",.03);if(entry.hook&&!(shaft.points[0]![0]!>0&&shaft.points[0]![0]!<1.3))throw Error("Injection hook source does not connect to shaft");equal([shaft.points[1]![0]!],[control.unit],"D3 target",entry.directed?1.8:.03);equal(shaft.dash,entry.dashPt.map(value=>value*72/72.27),"Actual dash pattern",.01);equal([shaft.dashPhase],[0],"Actual dash phase",.001);}
      if(heads.length!==(entry.directed?1:0))throw Error("Declared target arrowhead count differs");for(const head of heads){equal([head.bounds[2]!-head.bounds[0]!],[1.1],"Straight Barb length",.04);if(Math.abs(head.bounds[2]!-control.unit)>head.width+.05)throw Error("Declared target lies outside head stroke envelope");}
      if(entry.hook?hooks.length!==1:hooks.length!==0)throw Error("Injection source hook geometry differs "+hooks.length);if(entry.double&&Math.max(...shafts.map(path=>path.width))-Math.min(...shafts.map(path=>path.width))<.12)throw Error("Double relation has no separated visible strokes");
    }catch(error){failures.push(appearance+"/"+entry.style+": "+String(error));}
  });}
  if(JSON.stringify(control)!==original)throw Error("Style vector changed");if(failures.length)throw Error("Scientific relation styles:\n"+failures.join("\n"));
});}

/** 📏️ Verifies canonical shape vocabularies and positive scientific body scales. */
export async function compileNativeScientificScalarAdmission():Promise<void>{
  const neutral=fixture.scientificScalarAdmission,control=neutral.vectors,failures:string[]=[],original=JSON.stringify(control),ajv=new Ajv2020({strict:false});
  
  for(const entry of control.cases){
    const schema=entry.key==="nodeShape"?{enum:entry.family==="sci-pathway"?["box","circle"]:["none","box","circle"]}:{type:"number",exclusiveMinimum:0};
    if(Boolean(ajv.compile(schema)(entry.value))!==entry.valid)throw Error("Scientific scalar independent oracle mismatch");
    for(const language of ["en","de"]as const){
      const options={[entry.key]:entry.key==="nodeShape"?(entry.value==="circle"?"box":"circle"):1},base={chart:{width:80,height:48,language,theme:{appearance:language==="en"?"light":"dark"},layers:[],presets:[{kind:entry.kind,options}]}}as unknown as VizChartSnapshot,before=JSON.stringify(base),outcome=changeVizChartValue(base,{path:["presets","0","options",entry.key],value:entry.value}),replay=applyVizChartDiff(base,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(base,outcome.diff));
      if(outcome.messages.length||outcome.diff.edits.length!==1||replay.messages.length||inverse.messages.length||JSON.stringify(base)!==before)throw Error("Scientific scalar mutation refused");deepStrictEqual(inverse.snapshot,base);const result=await inferVizChart(replay.snapshot);if(result.complete!==entry.valid)failures.push(entry.id+"/"+language+" admitted="+result.complete+" expected="+entry.valid);if(!result.complete&&(result.tikz||!result.diagnostics.length))throw Error("Scientific scalar rejected with partial/undiagnosed output");
    }
  }
  if(JSON.stringify(control)!==original)throw Error("Scientific scalar neutral mutated");if(failures.length)throw Error("Scientific scalar admission:\n"+failures.join("\n"));
}

/** 🔒️ Verifies native mode admission against the authored eight-family vocabulary. */
export async function compileNativeScientificModeDomain(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/scientific-mode-domain",async({directory:workDir,execution})=>{
  const neutral=fixture.scientificNativeModeDomain,control=neutral.vectors,original=JSON.stringify(control),failures:string[]=[],ajv=new Ajv2020({strict:false}),themes=(["light","dark"]as const).filter(value=>!process.env.PRINT_NATIVE_CONTROL_THEME||value===process.env.PRINT_NATIVE_CONTROL_THEME);
  
  for(const entry of control.cases){const family=control.families.find(family=>family.name===entry.family)!;if(Boolean(ajv.compile({enum:family.modes})(entry.mode))!==entry.valid)throw Error("Independent native mode oracle mismatch");}
  for(const appearance of themes){
    const language=appearance==="light"?"en":"de",directory=join(workDir,"predicates",appearance),body=control.cases.map(entry=>({raw:"\\SemioVizProbeBegin{scientific-mode-domain}{"+entry.id+"}\\ExplSyntaxOn\\bool_set_true:N\\l_semio_viz_sci_records_valid_bool\\keys_set:nn{semio/viz/family/"+entry.family+"}{mode="+entry.mode+"}\\semio_viz_probe_values:nx{native/valid}{\\bool_if:NTF\\l_semio_viz_sci_records_valid_bool{1}{0}}\\ExplSyntaxOff"}));
    const records=await compileVizProbeDocument({case:"scientific-mode-domain",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],preamble:["\\title{Scientific Mode Domain}","\\author{Semio}","\\date{}","\\ExplSyntaxOn\\msg_redirect_name:nnn{semio-viz-sci}{invalid-record}{warning}\\ExplSyntaxOff"],body},{execution,scenario:undefined});
    for(const entry of control.cases)try{equal(records.find(record=>record.scenario===entry.id&&record.key==="native/valid")!.values.map(Number),[entry.valid?1:0],entry.id+" native mode predicate");}catch(error){failures.push(appearance+"/"+String(error));}
  }
  if(failures.length)throw Error("Scientific native modes:\n"+failures.join("\n"));
  for(const appearance of themes)for(const entry of control.cases.filter(entry=>!entry.valid)){
    const directory=join(workDir,entry.id,appearance),language=appearance==="light"?"en":"de";
    {const refusal=await inspectVizProbeDocumentRefusalV1({case:"scientific-mode-domain",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],preamble:["\\title{Scientific Mode Rejection}","\\author{Semio}","\\date{}"],body:[{raw:"\\ExplSyntaxOn\\keys_set:nn{semio/viz/family/"+entry.family+"}{mode="+entry.mode+"}\\ExplSyntaxOff"},{values:{key:"native/accepted",clist:"1"}}]},{execution,scenario:undefined});const log=refusal.log;if(!log.includes("Scientific record domain")||!log.includes(entry.family))throw Error("Unexpected actual compiler diagnostic: "+log);}
    
  }
  if(JSON.stringify(control)!==original)throw Error("Native mode vector changed");
});}

/** 🏁️ Requires finite complete public force-start pairs before the existing native solver. */
export async function compileNativeSpecializedPlacementAdmission(execution:PrintExecutionV1):Promise<void>{return withPrintScratchV1(execution,"native/specialized-placement-admission",async({directory:workDir,execution})=>{
  const contract=fixture.nativeSpecializedPlacementAdmission,{csvParse}=await import("d3-dsv"),failures:string[]=[];let executed=0;
  
  const rowSchema=new Ajv2020({strict:false,strictNumbers:true}).compile({type:"object",required:["x","y"],properties:{x:{type:"number"},y:{type:"number"}}});
  for(const entry of contract.vectors.cases.filter(entry=>!process.env.PRINT_NATIVE_PLACEMENT_CASE||entry.id===process.env.PRINT_NATIVE_PLACEMENT_CASE)){
    const parsed=csvParse([entry.columns.join(","),...entry.rows.map(row=>row.join(","))].join("\n")),valid=parsed.every(row=>rowSchema(Object.fromEntries(["x","y"].filter(key=>entry.columns.includes(key)).map(key=>[key,row[key]?.trim()?Number(row[key]):NaN]))));
    if(valid!==entry.valid)throw Error(entry.id+" independent D3 CSV/AJV finite pair contract differs");
    for(const family of ["biofabric","data-structure","commit-graph"].filter(family=>!process.env.PRINT_NATIVE_PLACEMENT_FAMILY||family===process.env.PRINT_NATIVE_PLACEMENT_FAMILY))for(const[language,appearance]of [["en","light"],["de","dark"]]as const){
      if(process.env.PRINT_NATIVE_PLACEMENT_THEME&&appearance!==process.env.PRINT_NATIVE_PLACEMENT_THEME)continue;
      const directory=join(workDir,family,entry.id,appearance),columns=[...entry.columns,"parent",...(family==="data-structure"?["value"]:[])],rows=entry.rows.map((row,index)=>Object.fromEntries(columns.map((key,column)=>[key,key==="parent"?(index?"A":""):key==="value"?index:row[column]!]))) as Record<string,string|number>[],numeric=(key:string,value:number)=>chartSchema["x-semio-family-options"][family as "biofabric"].options[key as "padding"]?.type==="string"?String(value):value,options={layout:"grid",iterations:numeric("iterations",1),labels:false,...family==="data-structure"?{data:"xy-nodes",idColumn:"id",parentColumn:"parent",valueColumn:"value",mode:"bst"}:{data:"xy-edges",nodes:"xy-nodes",id:"id",source:"source",target:"target"}},base={chart:{language,theme:{appearance},width:80,height:40,margin:{top:0,right:0,bottom:0,left:0},layers:[],tables:[{name:"xy-nodes",columns,rows},{name:"xy-edges",columns:["source","target"],rows:[{source:"A",target:"B"}]}],presets:[{kind:family==="biofabric"?"biofabric-style-graph":family==="data-structure"?"binary-search-tree":"commit-graph",options}]}}as unknown as VizChartSnapshot,before=JSON.stringify(base),outcome=changeVizChartValue(base,{path:["presets","0","options","layout"],value:"force"}),replay=applyVizChartDiff(base,outcome.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(base,outcome.diff));
      if(outcome.messages.length||outcome.diff.edits.length!==1||replay.messages.length||inverse.messages.length||JSON.stringify(base)!==before)throw Error("Public XY mutation/purity refused");deepStrictEqual(inverse.snapshot,base);const inferred=await inferVizChart(replay.snapshot);if(inferred.diagnostics.some(diagnostic=>diagnostic.code!=="print.chart.scene-unavailable")||!inferred.tikz.includes("layout={force}"))throw Error("Public XY canonical emission refused "+JSON.stringify(inferred.diagnostics));mkdirSync(directory,{recursive:true});await Bun.write(join(directory,"authored-emission.tex"),inferred.tikz);executed++;
      if(entry.valid){await compileVizProbeDocument({case:"public-xy-admission",scenario:appearance,documentClass:"semio",documentClassOptions:`type=paper,language=${language},theme=${appearance}`,packages:["semio-viz"],preamble:["\\title{Public Force Starts}","\\author{Semio}","\\date{}"],body:[{raw:inferred.tikz},{values:{key:"native/accepted",clist:"1"}}]},{execution,scenario:undefined});}else {const refusal=await inspectVizProbeDocumentRefusalV1({case:"public-xy-admission",scenario:appearance,documentClass:"semio",documentClassOptions:`type=paper,language=${language},theme=${appearance}`,packages:["semio-viz"],preamble:["\\title{Public Force Starts}","\\author{Semio}","\\date{}"],body:[{raw:inferred.tikz},{values:{key:"native/accepted",clist:"1"}}]},{execution,scenario:undefined});const log=refusal.log;if(!log.replace(/\r?\n\(semio-viz\)\s+/g," ").includes("Authored force nodes require both finite x and y columns")){failures.push(family+"/"+entry.id+"/"+appearance+" failed outside the public coordinate domain diagnostic");continue;}}

    }
  }
  if(failures.length)throw Error("Public force starts:\n"+failures.join("\n"));
});}
