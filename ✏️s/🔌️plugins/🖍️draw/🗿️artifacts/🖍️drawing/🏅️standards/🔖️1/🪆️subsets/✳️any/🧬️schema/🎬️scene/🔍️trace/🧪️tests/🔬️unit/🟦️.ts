const neutral=(value:any):any=>value instanceof Uint8Array?Array.from(value):Array.isArray(value)?value.map(neutral):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,neutral(item)])):value;
/** 🧪️ Actual encoded documents, native contour coordinates and independent SVG pixels. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json" with {type:"json"};
import sharp from "sharp";
import rows from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import prepSchema from "../../../📋️prepare/🧬️schema/🔣️.json";
import rasterSchema from "../../../📷️raster/🧬️schema/🔣️.json";
import pathSchema from "../../../../🧮️geometry/📷️raster/🧬️schema/🔣️.json";
import imageSchema from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧬️schema/🔣️.json";
import flatSchema from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🧬️schema/🔣️.json";
import regionSchema from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🔀️booleans/🧬️schema/🔣️.json";
import curveSchema from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🔀️booleans/🛤️paths/🧬️schema/🔣️.json";
import traceSchema from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🔍️trace/🧬️schema/🔣️.json";
import booleanSchema from "../../../🔀️booleans/🧬️schema/🔣️.json";
import {binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {DocumentSceneJob,DocumentRasterJob,rasterizeDocument,resolvedSceneInput} from "../../../📋️prepare/🟦️.ts";
import {DocumentBooleanJob} from "../../../🔀️booleans/🟦️.ts";
import {RasterSceneJob} from "../../../📷️raster/🟦️.ts";
import {DocumentTraceJob,resolveDocumentTraces,type DocumentTraceInput,type DocumentTraceProgress} from "../../🟦️.ts";
const ajv=new Ajv({strict:true}).addMetaSchema(draft7);for(const s of [pathSchema,imageSchema,rasterSchema,prepSchema,flatSchema,regionSchema,curveSchema,traceSchema,booleanSchema])ajv.addSchema(s);const valid=ajv.compile(schema),progressValid=ajv.compile({$ref:schema.$id+"#/definitions/progress"}),resultValid=ajv.compile({$ref:schema.$id+"#/definitions/result"});
function lift(v:any):any{return typeof v==="number"?binary64(v):Array.isArray(v)?v.map(lift):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).map(([k,v])=>[k,lift(v)])):v;}
function input(row:typeof rows[number]):DocumentTraceInput{const job=new DocumentSceneJob({...row.document,layers:lift(row.document.layers)}as any,row.preparation);while(!job.advance(4096).done){}return{plan:job.result(),limits:row.limits};}
function finish(value:DocumentTraceInput,grant:number,observer?:(p:DocumentTraceProgress)=>void){
 const job=new DocumentTraceJob(value);expect(()=>job.result()).toThrow(/incomplete/i);let work=0;
 for(;;){const p=job.advance(grant);expect(progressValid(p)).toBe(true);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;observer?.(p);if(p.done){const result=job.result();expect(resultValid(neutral(result))).toBe(true);return result;}expect(()=>job.result()).toThrow(/incomplete/i);}
}
function area(segments:any[]):number{let start=[0,0],previous=[0,0],sum=0;for(const s of segments)if(s.kind==="move")start=previous=s.to;else if(s.kind==="line"){sum+=((previous[0]!-start[0]!)*(s.to[1]-start[1]!)-(previous[1]!-start[1]!)*(s.to[0]-start[0]!))/2;previous=s.to;}return Math.abs(sum);}
function foreground(segments:any[],width:number,height:number):number[]{const rings:any[][]=[];for(const s of segments)if(s.kind==="move")rings.push([s.to]);else if(s.kind==="line")rings.at(-1)!.push(s.to);return Array.from({length:width*height},(_,at)=>{const x=at%width+.5,y=Math.floor(at/width)+.5;let winding=0;for(const r of rings)for(let i=0;i<r.length;i++){const a=r[i]!,b=r[(i+1)%r.length]!,cross=(b[0]-a[0])*(y-a[1])-(b[1]-a[1])*(x-a[0]);if(a[1]<=y&&b[1]>y&&cross>0)winding++;else if(a[1]>y&&b[1]<=y&&cross<0)winding--;}return winding?1:0;});}
for(const row of rows)test("actual trace document: "+row.name,()=>{
 const value=input(row),before=structuredClone(value);expect(valid(neutral(value))).toBe(true);let previous:any=null;
 for(const grant of [1,7,4096]){let last!:DocumentTraceProgress;const result=finish(value,grant,p=>last=p);expect(last.admittedImages).toBe(row.expected.admittedImages);expect(last.pixels).toBe(row.expected.pixels);for(const e of row.expected.traces){const node=result.nodes.find(n=>n.id===e.id)!,c=node.content as any;expect(c.kind).toBe("path");expect(area(c.segments)).toBe(e.area);expect(c.segments.filter((s:any)=>s.kind==="move").length).toBe(e.contours);expect(foreground(c.segments,row.source.width,row.source.height)).toEqual(e.foreground);const original=value.plan.nodes.find(n=>n.id===e.id)!;expect({...node,content:null}).toEqual({...original,content:null});for(const key of ["fill","stroke","fillRule"])expect(c[key]).toEqual((original.content as any)[key]);}if(previous)expect(result).toEqual(previous);previous=result;}expect(value).toEqual(before);
});
test("real trace and Boolean documents match independent source PNG and authored SVG pixels",async()=>{
 for(const row of rows){const asset=row.document.assets.source,rawPNG=await sharp(Buffer.from(asset.samples.flat()),{raw:{width:asset.width,height:asset.height,channels:4}}).png().toBuffer().then(encoded=>sharp(encoded).ensureAlpha().raw().toBuffer());expect([...rawPNG]).toEqual(row.source.pixels);
  const plan=finish(input(row),4096),boolean=new DocumentBooleanJob({plan,limits:row.booleans});while(!boolean.advance(4096).done){}const raster=new RasterSceneJob(resolvedSceneInput(boolean.result(),row.viewport as any));while(!raster.advance(4096).done){}const scale=16,w=row.viewport.width,h=row.viewport.height,raw=await sharp(Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="'+w*scale+'" height="'+h*scale+'" viewBox="'+row.viewport.origin.join(" ")+" "+w+" "+h+'">'+row.oracle.paths.map(p=>'<path d="'+p.d+'" transform="matrix('+p.matrix.join(" ")+')" fill="red"/>').join("")+"</svg>")).ensureAlpha().raw().toBuffer();let delta=0;
  for(let at=0;at<w*h;at++){let sum=0;for(let y=0;y<scale;y++)for(let x=0;x<scale;x++)sum+=raw[((Math.floor(at/w)*scale+y)*w*scale+at%w*scale+x)*4+3]!;delta=Math.max(delta,Math.abs(raster.result().pixels[at*4+3]!-Math.round(sum/(scale*scale))));}expect(delta,row.name).toBeLessThanOrEqual(3);
 }console.log("[DEBUG] Actual encoded image trace documents matched independent PNG bytes and authored SVG pixels including Boolean operands and native dimensions");
});
test("document trace source, parameters and resource failures stay private",()=>{
 const source=input(rows[0]!);for(const patch of [{maxWork:0},{maxPixels:0},{maxAdmittedPixels:0},{maxRetainedSegments:0},{maxSourceBytes:0}])expect(()=>new DocumentTraceJob({...source,limits:{...source.limits,...patch}})).toThrow();
 for(const kind of ["missing","duplicate","extent","samples","threshold","epsilon","pixels","decoded","source","edges","segments","retained","work"]){const v=structuredClone(source);const c=v.plan.nodes[0]!.content as any;if(kind==="missing")c.source="absent";if(kind==="duplicate")v.plan.assets.push(v.plan.assets[0]!);if(kind==="extent")v.plan.assets[0]!.image.width=0;if(kind==="samples")v.plan.assets[0]!.image.pixels=new Uint8Array(1);if(kind==="threshold")c.threshold=NaN;if(kind==="epsilon")c.simplifyEpsilon=8193;if(kind==="pixels")v.limits.maxPixels=8;if(kind==="decoded")v.limits.maxAdmittedPixels=8;if(kind==="source")v.limits.maxSourceBytes=1;if(kind==="edges")v.limits.maxEdges=1;if(kind==="segments")v.limits.maxSegments=1;if(kind==="retained")v.limits.maxRetainedSegments=1;if(kind==="work")v.limits.maxWork=1;
  const job=new DocumentTraceJob(v);expect(()=>{while(!job.advance(4096).done){}}).toThrow();expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();
 }for(const grant of [0,-1,NaN,Infinity,.5])expect(()=>new DocumentTraceJob(source).advance(grant)).toThrow();
});
test("aggregate source and pixel budgets charge distinct assets once",()=>{
 const row=rows[4]!,source=input(row);expect(finish({...source,limits:{...source.limits,maxSourceBytes:source.plan.assets[0]!.image.pixels.length,maxAdmittedPixels:9}},7).nodes.length).toBe(2);
 for(const cap of ["maxSourceBytes","maxAdmittedPixels"]){const value=structuredClone(source);value.plan.assets.push({...value.plan.assets[0]!,id:"second"});(value.plan.nodes[1]!.content as any).source="second";value.limits[cap]=cap==="maxSourceBytes"?value.plan.assets[0]!.image.pixels.length*2-1:17;const job=new DocumentTraceJob(value);expect(()=>{while(!job.advance(4096).done){}}).toThrow();expect(()=>job.result()).toThrow();}
});
test("every document trace phase cancels with private candidates and owned published output",()=>{
 const value=input(rows[0]!),seen=new Set<string>();finish(value,1,p=>{if(seen.has(p.phase))return;const job=new DocumentTraceJob(value);job.advance(p.work);job.cancel();expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.advance(1)).toThrow(/cancel/i);seen.add(p.phase);});expect([...seen].sort()).toEqual(["assets","indexing","nodes","luma","tracing","publishing","complete"].sort());
 const job=new DocumentTraceJob(value);while(!job.advance(4096).done){}const result=job.result(),before=structuredClone(result);job.cancel();expect(result).toEqual(before);
});
test("exact trace budgets preserve complete plans and async progress aborts at completion",async()=>{
 const value=input(rows[0]!);let last!:DocumentTraceProgress;const result=finish(value,1,p=>last=p);expect(finish({...value,limits:{...value.limits,maxWork:last.work,maxAdmittedPixels:last.pixels,maxRetainedSegments:last.segments,maxSourceBytes:last.sourceBytes}},7)).toEqual(result);
 let tick=false;setTimeout(()=>tick=true,0);expect(await resolveDocumentTraces(value,{workBudget:7})).toEqual(result);expect(tick).toBe(true);for(const phase of ["luma","tracing","complete"]){const controller=new AbortController();await expect(resolveDocumentTraces(value,{workBudget:1,signal:controller.signal,onProgress:p=>{if(p.phase===phase)controller.abort();}})).rejects.toThrow(/cancel/i);}
});
test("actual document-to-pixels resolves traced sources before Boolean references under every grant",()=>{
 for(const row of rows)for(const grant of [1,7,4096]){const document={...row.document,layers:lift(row.document.layers)}as any,job=new DocumentRasterJob(document,row.preparation,row.viewport as any,{booleans:row.booleans,trace:row.limits,maxWork:10000000});let decode=false,trace=false,work=0;
  for(;;){const p=job.advance(grant);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.tracing?.phase==="luma"&&!p.tracing.done){decode=true;expect(()=>job.result()).toThrow();}if(p.tracing?.trace&&!p.tracing.trace.done){trace=true;expect(()=>job.result()).toThrow();}if(p.done)break;}
  const boolean=new DocumentBooleanJob({plan:finish(input(row),4096),limits:row.booleans});while(!boolean.advance(4096).done){}const raster=new RasterSceneJob(resolvedSceneInput(boolean.result(),row.viewport as any));while(!raster.advance(4096).done){}expect(job.result().pixels).toEqual(raster.result().pixels);if(grant===1){expect(decode).toBe(true);expect(trace).toBe(true);}
 }console.log("[DEBUG] Actual document-to-pixels jobs prepare encoded trace sources before resolving their Boolean consumers and forward partial decode/contour progress");
});
test("document pipeline cancels decoded trace work and completion observers without publication",async()=>{
 const row=rows[0]!,document={...row.document,layers:lift(row.document.layers)}as any,configuration={booleans:row.booleans,trace:row.limits,maxWork:10000000};
 const job=new DocumentRasterJob(document,row.preparation,row.viewport as any,configuration);while(job.advance(1).phase!=="tracing"){}job.cancel();expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.advance(1)).toThrow(/cancel/i);
 for(const phase of ["tracing","complete"]){const controller=new AbortController();await expect(rasterizeDocument(document,row.preparation,row.viewport as any,configuration,{signal:controller.signal,workBudget:7,onProgress:p=>{if(p.phase===phase)controller.abort();}})).rejects.toThrow(/cancel/i);}
});
test("complete document work admission bounds every stage and permits the exact final grant",()=>{
 const row=rows[0]!,document={...row.document,layers:lift(row.document.layers)}as any,configuration={booleans:row.booleans,trace:row.limits,maxWork:10000000},check=ajv.compile({$ref:prepSchema.$id+"#/definitions/algorithmLimits"});expect(check(configuration)).toBe(true);expect(check({...configuration,maxWork:0})).toBe(false);expect(()=>new DocumentRasterJob(document,row.preparation,row.viewport as any,{...configuration,maxWork:0})).toThrow(/work limit/i);
 const job=new DocumentRasterJob(document,row.preparation,row.viewport as any,configuration);let work=0;for(;;){const p=job.advance(1);work=p.work;if(p.done)break;}const expected=job.result().pixels,exact=new DocumentRasterJob(document,row.preparation,row.viewport as any,{...configuration,maxWork:work});while(!exact.advance(7).done){}expect(exact.result().pixels).toEqual(expected);
 for(const maxWork of [1,work-1]){const refused=new DocumentRasterJob(document,row.preparation,row.viewport as any,{...configuration,maxWork});expect(()=>{while(!refused.advance(4096).done){}}).toThrow(/work limit/i);expect(()=>refused.result()).toThrow(/work limit/i);expect(()=>refused.advance(1)).toThrow(/work limit/i);}
});

/** 🧹️ Transfer private trace owners without sharing mutable completed scene metadata. */
import retirementRows from "../../🧫️fixtures/🧹️retirement/🔣️.json";
import {ScenePlanCloseJob} from "../../../🧹️retire/🟦️.ts";
const retirementValid=ajv.compile({$ref:schema.$id+"#/definitions/retirementProgress"});
test("document trace retirement composes active kernels and independently owned scene plans",()=>{
 for(const sample of retirementRows.cases)for(const grant of [1,7,4096]){
  const row=rows.find(r=>r.name===("source" in sample?sample.source:retirementRows.source))!,source=input(row);for(const n of source.plan.nodes)n.groups.push(structuredClone(retirementRows.metadata.group));const c=source.plan.nodes.find(n=>n.content.kind==="trace")!.content as any;c.fill=structuredClone(retirementRows.metadata.fill);c.stroke=structuredClone(retirementRows.metadata.stroke);
  if("maxWork" in sample)source.limits={...source.limits,maxWork:sample.maxWork!};if("maxRetainedSegments" in sample)source.limits={...source.limits,maxRetainedSegments:sample.maxRetainedSegments!};
  const before=structuredClone(source),job=new DocumentTraceJob(source),state=job as any;
  if(sample.phase==="failure")expect(()=>job.advance(100000)).toThrow();else if(sample.phase!=="fresh"){if(sample.phase!=="cancelled"){let reached=false;for(let at=0;at<100000;at++){if(job.advance(1).phase===sample.phase&&(!("checkpoint" in sample)||state.closingTracer!==null)){reached=true;break;}}expect(reached).toBe(true);}for(let at=0;at<sample.offset;at++)job.advance(1);}
  if(sample.phase==="failure"&&"maxWork" in sample)expect(state.catalog.size).toBeGreaterThan(0);if(sample.phase==="cancelled")job.cancel();const published=sample.phase==="complete"?job.result():null,transferred=job.intoRetirement();expect(transferred.output).toBe(published);if(sample.phase==="cancelled")expect(transferred.input.nodes).toHaveLength(0);else expect(transferred.input).toBe(source.plan);
  expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.advance(1)).toThrow(/cancel/i);expect(()=>job.intoRetirement()).toThrow(/transferred/i);job.cancel();for(const n of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>transferred.job.advance(n)).toThrow(/grant/i);
  let work=0,done=false;for(let at=0;at<100000;at++){const count=state.masks.size,p=transferred.job.advance(grant);expect(count-state.masks.size).toBeLessThanOrEqual(grant);expect(retirementValid(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done){done=true;break;}}
  expect(done).toBe(true);expect(work).toBeGreaterThanOrEqual(retirementRows.minimumWork);expect(transferred.job.terminalIsEmpty()).toBe(true);expect(transferred.job.advance(1)).toEqual({phase:"complete",work,done:true});for(const field of ["catalog","ids","masks","dimensions"])expect(state[field].size).toBe(0);for(const field of ["raw","candidate"])expect(state[field]).toHaveLength(0);for(const field of ["tracer","closingTracer","mask","image","current","copyNode"])expect(state[field]).toBe(null);expect(source).toEqual(before);
  if(published){expect(resultValid(neutral(published))).toBe(true);const live=finish(source,grant),stable=structuredClone(live);for(let at=0;at<published.nodes.length;at++){const original=source.plan.nodes[at]!.content,next=published.nodes[at]!.content;expect(published.nodes[at]!.groups).not.toBe(source.plan.nodes[at]!.groups);if(original.kind==="boolean"&&next.kind==="boolean"){expect(next.children).not.toBe(original.children);expect(next.referenceTransform).not.toBe(original.referenceTransform);}}const outputClose=new ScenePlanCloseJob(published);while(!outputClose.advance(grant).done){}expect(source).toEqual(before);expect(live).toEqual(stable);const inputClose=new ScenePlanCloseJob(transferred.input);while(!inputClose.advance(grant).done){}expect(live).toEqual(stable);const liveClose=new ScenePlanCloseJob(live);while(!liveClose.advance(grant).done){}}
  console.log(`[DEBUG] Document trace ${sample.phase} retired private owners in ${work} work units at grant ${grant}`);
 }
});
