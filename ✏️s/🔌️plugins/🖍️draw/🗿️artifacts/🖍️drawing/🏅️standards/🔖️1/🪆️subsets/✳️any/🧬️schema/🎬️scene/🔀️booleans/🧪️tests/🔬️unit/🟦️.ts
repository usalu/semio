const neutral=(value:any):any=>value instanceof Uint8Array?Array.from(value):Array.isArray(value)?value.map(neutral):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,neutral(item)])):value;
/** 🧪️ Actual documents, reference geometry, independent SVG pixels and publication limits. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json" with {type:"json"};
import sharp from "sharp";
import rows from "../../../../../🧪️tests/🧫️fixtures/🎬️scene/🔀️booleans/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import prepSchema from "../../../📋️prepare/🧬️schema/🔣️.json";
import rasterSchema from "../../../📷️raster/🧬️schema/🔣️.json";
import pathSchema from "../../../../🧮️geometry/📷️raster/🧬️schema/🔣️.json";
import { imageDecodeSchema as imageSchema } from "@semio-tech/framework/pixels/image/decode/schema";
import { pathFlattenSchema as flatSchema } from "@semio-tech/framework/2d/path/flatten/schema";
import { booleansSchema as regionSchema } from "@semio-tech/framework/2d/booleans/schema";
import { booleansPathsSchema as curveSchema } from "@semio-tech/framework/2d/booleans/paths/schema";
import {binary64} from "@semio-tech/framework/value/ieee754";
import {DocumentRasterJob,rasterizeDocument,resolvedSceneInput} from "../../../📋️prepare/🟦️.ts";
import {DrawingSceneWorkController} from "../../../🎟️work/🟦️.ts";
import {ScenePlanCloseJob as ScenePlanCloseJobRef} from "../../../🧹️retire/🟦️.ts";
import {stock,drain,drive,plain,sceneOwner,owned,controllerFixture} from "../../../🎨️paint/🧪️tests/🎟️funding/🟦️.ts";
import {RasterSceneJob} from "../../../📷️raster/🟦️.ts";
import {DocumentBooleanJob,resolveDocumentBooleans,type DocumentBooleanInput,type DocumentBooleanProgress} from "../../🟦️.ts";
import type {ScenePlanOwner} from "../../../🗺️plan-model/🎟️owned/🟦️.ts";
import documentTraceSchema from "../../../🔍️trace/🧬️schema/🔣️.json";
import { traceSchema as bitmapTraceSchema } from "@semio-tech/framework/2d/trace/schema";
import traceRows from "../../../../../🧪️tests/🧫️fixtures/🎬️scene/🔍️trace/🔣️.json";
const ajv=new Ajv({strict:true}).addMetaSchema(draft7);for(const s of [pathSchema,imageSchema,rasterSchema,prepSchema,flatSchema,regionSchema,curveSchema,bitmapTraceSchema,documentTraceSchema])ajv.addSchema(s);const valid=ajv.compile(schema),validProgress=ajv.compile({$ref:schema.$id+"#/definitions/progress"}),validResult=ajv.compile({$ref:schema.$id+"#/definitions/result"});
function lift(v:any):any{return typeof v==="number"?binary64(v):Array.isArray(v)?v.map(lift):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).map(([k,v])=>[k,lift(v)])):v;}
const documentOf=(row:typeof rows[number],patch?:(layers:any[])=>void)=>{const layers=lift(row.document.layers);patch?.(layers);return{...row.document,layers};};
const input=(row:typeof rows[number],limits:any=row.limits,patch?:(layers:any[])=>void):DocumentBooleanInput=>({plan:sceneOwner(documentOf(row,patch),row.preparation),limits});
function finish(value:DocumentBooleanInput,grant:number,observer?:(p:DocumentBooleanProgress)=>void){
 const job=new DocumentBooleanJob(value);expect(()=>job.result()).toThrow(/incomplete/i);let work=0,result:any;
 drive(job,grant,p=>{expect(validProgress(p)).toBe(true);work=p.work;observer?.(p);if(!p.done)expect(()=>job.result()).toThrow(/incomplete/i);});
 result=plain(job.result());expect(validResult(neutral(result))).toBe(true);const moved=job.intoRetirement();drain(moved.job,grant);expect(moved.output).not.toBeNull();drain(new ScenePlanCloseJobRef(moved.output!),grant);return{plan:result,work};
}
function area(segments:any[]):number{let origin=[0,0],previous=[0,0],sum=0;for(const s of segments)if(s.kind==="move")origin=previous=s.to;else if(s.kind==="line"){sum+=((previous[0]!-origin[0]!)*(s.to[1]-origin[1]!)-(previous[1]!-origin[1]!)*(s.to[0]-origin[0]!))/2;previous=s.to;}return sum;}
for(const row of rows)test("real document boolean: "+row.name,()=>{
 const value=input(row);expect(valid(neutral({plan:plain(value.plan.borrow()),limits:value.limits}))).toBe(true);drain(new ScenePlanCloseJobRef(value.plan));let previous:any=null;
 for(const grant of [1,7,4096]){const{plan}=finish(input(row),grant),node=plan.nodes.find((n:any)=>n.id==="result")!;expect(node.content.kind).toBe("path");const c=node.content as any;expect(c.segments.filter((s:any)=>s.kind==="move").length).toBe(row.expected.contours);expect(Math.abs(Math.abs(area(c.segments))-row.expected.area)).toBeLessThanOrEqual("tolerance" in row.expected?row.expected.tolerance!:1e-8);if(previous)expect(plan).toEqual(previous);previous=plan;}
});
test("actual document Boolean pixels match independent authored SVG",async()=>{
 for(const row of rows){const job=new DocumentBooleanJob(input(row));drive(job);const raster=new RasterSceneJob(resolvedSceneInput(job.result(),{width:64,height:48,origin:[-32,-8],tolerance:.005,maxPixels:100000,maxSourceBytes:100000}));drive(raster);
  const scale=16,w=64*scale,h=48*scale,raw=await sharp(Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="'+w+'" height="'+h+'" viewBox="-32 -8 64 48"><path d="'+row.oracle.d+'" transform="matrix('+row.oracle.matrix.join(" ")+')" fill="red"/></svg>')).ensureAlpha().raw().toBuffer(),actual=raster.result().pixels;let delta=0;
  for(let at=0;at<64*48;at++){let sum=0;for(let y=0;y<scale;y++)for(let x=0;x<scale;x++)sum+=raw[((Math.floor(at/64)*scale+y)*w+at%64*scale+x)*4+3]!;delta=Math.max(delta,Math.abs(actual[at*4+3]!-Math.round(sum/(scale*scale))));}expect(delta,row.name).toBeLessThanOrEqual(3);
  drain(raster.intoRetirement().job);drain(job.intoRetirement().job);
 }
});
test("document Boolean caps and malformed limits fail privately and sticky",()=>{
 const row=rows[0]!;for(const patch of [{maxWork:0},{maxDepth:0},{maxRetainedSegments:0},{epsilon:0}]){const value=input(row,{...row.limits,...patch});expect(()=>new DocumentBooleanJob(value)).toThrow();drain(new ScenePlanCloseJobRef(value.plan));}
 for(const kind of ["depth","references","retained","work"]){const limits={...row.limits} as any;if(kind==="depth")limits.maxDepth=1;if(kind==="references")limits.maxReferences=1;if(kind==="retained")limits.maxRetainedSegments=1;if(kind==="work")limits.maxWork=1;
  const job=new DocumentBooleanJob(input(row,limits));expect(()=>drive(job)).toThrow();expect(()=>job.result()).toThrow();expect(()=>job.advance(1,stock(1))).toThrow();const moved=job.intoRetirement();expect(moved.output).toBeNull();drain(moved.job);
 }
 for(const quantum of [0,-1,NaN,Infinity,.5]){const job=new DocumentBooleanJob(input(row));expect(()=>job.advance(quantum,stock(1))).toThrow(/quantum/);drain(job.intoRetirement().job);}
});
test("every document Boolean phase cancels and retires its private owners",()=>{
 const row=rows[0]!,seen=new Set<string>();finish(input(row),1,p=>{if(seen.has(p.phase))return;const probe=new DocumentBooleanJob(input(row));let work=0;for(let at=0;at<20000000&&work<p.work;at++)work=probe.advance(1,stock(1)).work;probe.cancel();expect(()=>probe.result()).toThrow(/cancel/i);expect(()=>probe.advance(1,stock(1))).toThrow(/cancel/i);const moved=probe.intoRetirement();expect(moved.output).toBeNull();expect(()=>probe.intoRetirement()).toThrow(/transferred/i);drain(moved.job);seen.add(p.phase);});expect([...seen].sort()).toEqual(["indexing","validation","cycles","requirements","traversal","operands","geometry","remapping","publishing","complete"].sort());
 const job=new DocumentBooleanJob(input(row));drive(job);const result=plain(job.result());job.cancel();expect(()=>job.result()).toThrow(/cancel/i);const moved=job.intoRetirement();expect(moved.output).toBeNull();drain(moved.job);expect(result.nodes.length).toBeGreaterThan(0);
});
test("exact retained and work caps preserve complete output",()=>{const row=rows[0]!;let last!:DocumentBooleanProgress;const expected=finish(input(row),1,p=>last=p).plan;expect(finish(input(row,{...row.limits,maxWork:last.work,maxRetainedSegments:last.segments}),7).plan).toEqual(expected);});
test("unrepresentable result accuracy refuses privately and empty singular results stay empty",()=>{
 const job=new DocumentBooleanJob(input(rows[0]!,rows[0]!.limits,layers=>{const last=layers.at(-1);last.transform={...last.transform,scaleX:binary64(1e8),scaleY:binary64(1e8)};}));expect(()=>drive(job)).toThrow(/precision/);expect(()=>job.result()).toThrow(/precision/);drain(job.intoRetirement().job);
 const empty=finish(input(rows[8]!),7);expect(empty.plan.nodes.at(-1).content.segments).toEqual([]);
});
test("document Boolean async work yields under a caller controller and completion abort prevents publication",async()=>{
 const row=rows[0]!;let tick=false;setTimeout(()=>tick=true,0);const value=input(row),{control}=controllerFixture(value.plan,7);const owner=await resolveDocumentBooleans(value,control);expect(tick).toBe(true);expect(plain(owner.borrow()).nodes.find((n:any)=>n.id==="result").content.kind).toBe("path");drain(new ScenePlanCloseJobRef(owner));
 const abort=new AbortController(),second=input(row),fixture=controllerFixture(second.plan,7,abort.signal);await expect(resolveDocumentBooleans(second,{...fixture.control,onProgress:p=>{if(p.done)abort.abort();}})).rejects.toThrow(/cancel|abort/i);
});
test("real document pipeline resolves all Boolean fixtures under every grant",()=>{
 const viewport={width:64,height:48,origin:[-32,-8] as [number,number],tolerance:.005,maxPixels:100000,maxSourceBytes:100000};
 for(const row of rows)for(const grant of [1,7,4096]){const source=documentOf(row) as any,job=new DocumentRasterJob(owned(source),row.preparation,viewport,{booleans:row.limits,trace:traceRows[0]!.limits,maxWork:10000000});let partial=false;
  drive(job,grant,p=>{if(p.resolution?.geometry&&!p.resolution.geometry.done){partial=true;expect(()=>job.result()).toThrow();}});
  const boolean=new DocumentBooleanJob(input(row)),pixels=job.result().pixels;drive(boolean);const raster=new RasterSceneJob(resolvedSceneInput(boolean.result(),viewport));drive(raster);expect(pixels).toEqual(raster.result().pixels);drain(raster.intoRetirement().job);drain(boolean.intoRetirement().job);drain(job.intoRetirement().job);if(grant===1)expect(partial).toBe(true);}
});
