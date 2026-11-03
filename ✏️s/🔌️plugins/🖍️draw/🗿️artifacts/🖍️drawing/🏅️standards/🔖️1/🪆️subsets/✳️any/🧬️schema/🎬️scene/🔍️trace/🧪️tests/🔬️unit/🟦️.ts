const neutral=(value:any):any=>value instanceof Uint8Array?Array.from(value):Array.isArray(value)?value.map(neutral):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,neutral(item)])):value;
/** 🧪️ Actual encoded documents, native contour coordinates and independent SVG pixels. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json" with {type:"json"};
import sharp from "sharp";
import rows from "../../../../../🧪️tests/🧫️fixtures/🎬️scene/🔍️trace/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import prepSchema from "../../../📋️prepare/🧬️schema/🔣️.json";
import rasterSchema from "../../../📷️raster/🧬️schema/🔣️.json";
import pathSchema from "../../../../🧮️geometry/📷️raster/🧬️schema/🔣️.json";
import { imageDecodeSchema as imageSchema } from "@semio-tech/framework/pixels/image/decode/schema";
import { pathFlattenSchema as flatSchema } from "@semio-tech/framework/2d/path/flatten/schema";
import { booleansSchema as regionSchema } from "@semio-tech/framework/2d/booleans/schema";
import { booleansPathsSchema as curveSchema } from "@semio-tech/framework/2d/booleans/paths/schema";
import { traceSchema as traceSchema } from "@semio-tech/framework/2d/trace/schema";
import booleanSchema from "../../../🔀️booleans/🧬️schema/🔣️.json";
import {binary64} from "@semio-tech/framework/value/ieee754";
import {DocumentRasterJob,rasterizeDocument,resolvedSceneInput} from "../../../📋️prepare/🟦️.ts";
import {ScenePlanCloseJob as ScenePlanCloseJobRef} from "../../../🧹️retire/🟦️.ts";
import {stock,drain,drive,plain,sceneOwner,owned,controllerFixture} from "../../../🎨️paint/🧪️tests/🎟️funding/🟦️.ts";
import {DocumentBooleanJob} from "../../../🔀️booleans/🟦️.ts";
import {RasterSceneJob} from "../../../📷️raster/🟦️.ts";
import {DocumentTraceJob,resolveDocumentTraces,type DocumentTraceInput,type DocumentTraceProgress} from "../../🟦️.ts";
const ajv=new Ajv({strict:true}).addMetaSchema(draft7);for(const s of [pathSchema,imageSchema,rasterSchema,prepSchema,flatSchema,regionSchema,curveSchema,traceSchema,booleanSchema])ajv.addSchema(s);const valid=ajv.compile(schema),progressValid=ajv.compile({$ref:schema.$id+"#/definitions/progress"}),resultValid=ajv.compile({$ref:schema.$id+"#/definitions/result"});
function lift(v:any):any{return typeof v==="number"?binary64(v):Array.isArray(v)?v.map(lift):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).map(([k,v])=>[k,lift(v)])):v;}
const documentOf=(row:typeof rows[number],patch?:(document:any)=>void)=>{const document={...row.document,layers:lift(row.document.layers),assets:structuredClone(row.document.assets)} as any;patch?.(document);return document;};
const input=(row:typeof rows[number],limits:any=row.limits,patch?:(document:any)=>void):DocumentTraceInput=>({plan:sceneOwner(documentOf(row,patch),row.preparation),limits});
function finish(value:DocumentTraceInput,grant:number,observer?:(p:DocumentTraceProgress)=>void){
 const job=new DocumentTraceJob(value);expect(()=>job.result()).toThrow(/incomplete/i);let last!:DocumentTraceProgress;
 drive(job,grant,p=>{expect(progressValid(p)).toBe(true);last=p;observer?.(p);if(!p.done)expect(()=>job.result()).toThrow(/incomplete/i);});
 const result=plain(job.result());expect(resultValid(neutral(result))).toBe(true);const moved=job.intoRetirement();drain(moved.job,grant);drain(new ScenePlanCloseJobRef(moved.output!),grant);return{plan:result,last};
}
function area(segments:any[]):number{let start=[0,0],previous=[0,0],sum=0;for(const s of segments)if(s.kind==="move")start=previous=s.to;else if(s.kind==="line"){sum+=((previous[0]!-start[0]!)*(s.to[1]-start[1]!)-(previous[1]!-start[1]!)*(s.to[0]-start[0]!))/2;previous=s.to;}return Math.abs(sum);}
function foreground(segments:any[],width:number,height:number):number[]{const rings:any[][]=[];for(const s of segments)if(s.kind==="move")rings.push([s.to]);else if(s.kind==="line")rings.at(-1)!.push(s.to);return Array.from({length:width*height},(_,at)=>{const x=at%width+.5,y=Math.floor(at/width)+.5;let winding=0;for(const r of rings)for(let i=0;i<r.length;i++){const a=r[i]!,b=r[(i+1)%r.length]!,cross=(b[0]-a[0])*(y-a[1])-(b[1]-a[1])*(x-a[0]);if(a[1]<=y&&b[1]>y&&cross>0)winding++;else if(a[1]>y&&b[1]<=y&&cross<0)winding--;}return winding?1:0;});}
for(const row of rows)test("actual trace document: "+row.name,()=>{
 const probe=input(row);expect(valid(neutral({plan:plain(probe.plan.borrow()),limits:probe.limits}))).toBe(true);const original=plain(probe.plan.borrow());drain(new ScenePlanCloseJobRef(probe.plan));let previous:any=null;
 for(const grant of [1,7,4096]){const{plan,last}=finish(input(row),grant);expect(last.admittedImages).toBe(row.expected.admittedImages);expect(last.pixels).toBe(row.expected.pixels);for(const e of row.expected.traces){const node=plan.nodes.find((n:any)=>n.id===e.id)!,c=node.content as any;expect(c.kind).toBe("path");expect(area(c.segments)).toBe(e.area);expect(c.segments.filter((s:any)=>s.kind==="move").length).toBe(e.contours);expect(foreground(c.segments,row.source.width,row.source.height)).toEqual(e.foreground);const before=original.nodes.find((n:any)=>n.id===e.id)!;expect({...node,content:null}).toEqual({...before,content:null});for(const key of ["fill","stroke","fillRule"])expect(c[key]).toEqual(before.content[key]);}if(previous)expect(plan).toEqual(previous);previous=plan;}
});
test("real trace and Boolean documents match independent source PNG and authored SVG pixels",async()=>{
 for(const row of rows){const asset=row.document.assets.source,rawPNG=await sharp(Buffer.from(asset.samples.flat()),{raw:{width:asset.width,height:asset.height,channels:4}}).png().toBuffer().then(encoded=>sharp(encoded).ensureAlpha().raw().toBuffer());expect([...rawPNG]).toEqual(row.source.pixels);
  const trace=new DocumentTraceJob(input(row));drive(trace);const moved=trace.intoRetirement();drain(moved.job);const boolean=new DocumentBooleanJob({plan:moved.output!,limits:row.booleans});drive(boolean);const raster=new RasterSceneJob(resolvedSceneInput(boolean.result(),row.viewport as any));drive(raster);const scale=16,w=row.viewport.width,h=row.viewport.height,raw=await sharp(Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="'+w*scale+'" height="'+h*scale+'" viewBox="'+row.viewport.origin.join(" ")+" "+w+" "+h+'">'+row.oracle.paths.map(p=>'<path d="'+p.d+'" transform="matrix('+p.matrix.join(" ")+')" fill="red"/>').join("")+"</svg>")).ensureAlpha().raw().toBuffer();let delta=0;
  for(let at=0;at<w*h;at++){let sum=0;for(let y=0;y<scale;y++)for(let x=0;x<scale;x++)sum+=raw[((Math.floor(at/w)*scale+y)*w*scale+at%w*scale+x)*4+3]!;delta=Math.max(delta,Math.abs(raster.result().pixels[at*4+3]!-Math.round(sum/(scale*scale))));}expect(delta,row.name).toBeLessThanOrEqual(3);
  drain(raster.intoRetirement().job);drain(boolean.intoRetirement().job);
 }
});
test("document trace limits and resource failures stay private and sticky",()=>{
 const row=rows[0]!;for(const patch of [{maxWork:0},{maxPixels:0},{maxAdmittedPixels:0},{maxRetainedSegments:0},{maxSourceBytes:0}]){const value=input(row,{...row.limits,...patch});expect(()=>new DocumentTraceJob(value)).toThrow();drain(new ScenePlanCloseJobRef(value.plan));}
 for(const kind of ["pixels","decoded","source","edges","segments","retained","work"]){const limits={...row.limits} as any;if(kind==="pixels")limits.maxPixels=8;if(kind==="decoded")limits.maxAdmittedPixels=8;if(kind==="source")limits.maxSourceBytes=1;if(kind==="edges")limits.maxEdges=1;if(kind==="segments")limits.maxSegments=1;if(kind==="retained")limits.maxRetainedSegments=1;if(kind==="work")limits.maxWork=1;
  const job=new DocumentTraceJob(input(row,limits));expect(()=>drive(job)).toThrow();expect(()=>job.result()).toThrow();expect(()=>job.advance(1,stock(1))).toThrow();const moved=job.intoRetirement();expect(moved.output).toBeNull();drain(moved.job);
 }
 for(const quantum of [0,-1,NaN,Infinity,.5]){const job=new DocumentTraceJob(input(row));expect(()=>job.advance(quantum,stock(1))).toThrow(/quantum/);drain(job.intoRetirement().job);}
});
test("aggregate source and pixel budgets charge distinct assets once",()=>{
 const row=rows[4]!,bytes=plain(input(row).plan.borrow()).assets[0].image.pixels.length;
 expect(finish(input(row,{...row.limits,maxSourceBytes:bytes,maxAdmittedPixels:9}),7).plan.nodes.length).toBe(2);
 const patch=(document:any)=>{document.assets.second=structuredClone(document.assets.source);document.layers.at(-1).sourceKey="second";};
 for(const cap of ["maxSourceBytes","maxAdmittedPixels"] as const){const limits={...row.limits,[cap]:cap==="maxSourceBytes"?bytes*2-1:17} as any,job=new DocumentTraceJob(input(row,limits,patch));expect(()=>drive(job)).toThrow();expect(()=>job.result()).toThrow();drain(job.intoRetirement().job);}
});
test("every document trace phase cancels with private candidates and retires its owners",()=>{
 const row=rows[0]!,seen=new Set<string>();finish(input(row),1,p=>{if(seen.has(p.phase))return;const probe=new DocumentTraceJob(input(row));let work=0;for(let at=0;at<20000000&&work<p.work;at++)work=probe.advance(1,stock(1)).work;probe.cancel();expect(()=>probe.result()).toThrow(/cancel/i);expect(()=>probe.advance(1,stock(1))).toThrow(/cancel/i);const moved=probe.intoRetirement();expect(moved.output).toBeNull();expect(()=>probe.intoRetirement()).toThrow(/transferred/i);drain(moved.job);seen.add(p.phase);});expect([...seen].sort()).toEqual(["assets","indexing","nodes","luma","tracing","publishing","complete"].sort());
});
test("exact trace budgets preserve complete plans and async progress aborts at completion",async()=>{
 const row=rows[0]!;const{plan:result,last}=finish(input(row),1);expect(finish(input(row,{...row.limits,maxWork:last.work,maxAdmittedPixels:last.pixels,maxRetainedSegments:last.segments,maxSourceBytes:last.sourceBytes}),7).plan).toEqual(result);
 let tick=false;setTimeout(()=>tick=true,0);const value=input(row),{control}=controllerFixture(value.plan,7);const owner=await resolveDocumentTraces(value,control);expect(tick).toBe(true);expect(plain(owner.borrow())).toEqual(result);drain(new ScenePlanCloseJobRef(owner));
 for(const phase of ["luma","tracing","complete"]){const controller=new AbortController(),second=input(row),fixture=controllerFixture(second.plan,1,controller.signal);await expect(resolveDocumentTraces(second,{...fixture.control,onProgress:p=>{if(p.phase===phase)controller.abort();}})).rejects.toThrow(/cancel|abort/i);}
});
test("actual document-to-pixels resolves traced sources before Boolean references under every grant",()=>{
 for(const row of rows)for(const grant of [1,7,4096]){const job=new DocumentRasterJob(owned(documentOf(row)),row.preparation,row.viewport as any,{booleans:row.booleans,trace:row.limits,maxWork:10000000});let decode=false,trace=false;
  drive(job,grant,p=>{if(p.tracing?.phase==="luma"&&!p.tracing.done){decode=true;expect(()=>job.result()).toThrow();}if(p.tracing?.trace&&!p.tracing.trace.done){trace=true;expect(()=>job.result()).toThrow();}});
  const tracer=new DocumentTraceJob(input(row));drive(tracer);const moved=tracer.intoRetirement();drain(moved.job);const boolean=new DocumentBooleanJob({plan:moved.output!,limits:row.booleans});drive(boolean);const raster=new RasterSceneJob(resolvedSceneInput(boolean.result(),row.viewport as any));drive(raster);expect(job.result().pixels).toEqual(raster.result().pixels);drain(raster.intoRetirement().job);drain(boolean.intoRetirement().job);drain(job.intoRetirement().job);if(grant===1){expect(decode).toBe(true);expect(trace).toBe(true);}
 }
});
test("document pipeline cancels decoded trace work and completion observers without publication",async()=>{
 const row=rows[0]!,configuration={booleans:row.booleans,trace:row.limits,maxWork:10000000};
 const job=new DocumentRasterJob(owned(documentOf(row)),row.preparation,row.viewport as any,configuration);while(job.advance(1,stock(1)).phase!=="tracing"){}job.cancel();expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.advance(1,stock(1))).toThrow(/cancel/i);drain(job.intoRetirement().job);
 for(const phase of ["tracing","complete"]){const controller=new AbortController(),document=owned(documentOf(row)),fixture=controllerFixture(document,7,controller.signal);await expect(rasterizeDocument(document,row.preparation,row.viewport as any,configuration,{...fixture.control,signal:controller.signal,onProgress:p=>{if(p.phase===phase)controller.abort();}})).rejects.toThrow(/cancel|abort/i);}
});
test("complete document work admission bounds every stage and permits the exact final grant",()=>{
 const row=rows[0]!,configuration={booleans:row.booleans,trace:row.limits,maxWork:10000000},check=ajv.compile({$ref:prepSchema.$id+"#/definitions/algorithmLimits"});expect(check(configuration)).toBe(true);expect(check({...configuration,maxWork:0})).toBe(false);expect(()=>new DocumentRasterJob(owned(documentOf(row)),row.preparation,row.viewport as any,{...configuration,maxWork:0})).toThrow(/work limit/i);
 const job=new DocumentRasterJob(owned(documentOf(row)),row.preparation,row.viewport as any,configuration),work=drive(job,1).work,expected=job.result().pixels;drain(job.intoRetirement().job);const exact=new DocumentRasterJob(owned(documentOf(row)),row.preparation,row.viewport as any,{...configuration,maxWork:work});drive(exact,7);expect(exact.result().pixels).toEqual(expected);drain(exact.intoRetirement().job);
 for(const maxWork of [1,work-1]){const refused=new DocumentRasterJob(owned(documentOf(row)),row.preparation,row.viewport as any,{...configuration,maxWork});expect(()=>drive(refused)).toThrow(/work limit/i);expect(()=>refused.result()).toThrow(/work limit/i);expect(()=>refused.advance(1,stock(1))).toThrow(/work limit/i);drain(refused.intoRetirement().job);}
});
