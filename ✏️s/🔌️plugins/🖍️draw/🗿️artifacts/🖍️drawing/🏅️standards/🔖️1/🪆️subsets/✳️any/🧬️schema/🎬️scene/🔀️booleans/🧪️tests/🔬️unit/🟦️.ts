/** 🧪️ Actual documents, reference geometry, independent SVG pixels and publication limits. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
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
import {binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {DocumentSceneJob,DocumentRasterJob,rasterizeDocument,resolvedSceneInput} from "../../../📋️prepare/🟦️.ts";
import {RasterSceneJob} from "../../../📷️raster/🟦️.ts";
import {DocumentBooleanJob,resolveDocumentBooleans,type DocumentBooleanInput,type DocumentBooleanProgress} from "../../🟦️.ts";
import documentTraceSchema from "../../../🔍️trace/🧬️schema/🔣️.json";
import bitmapTraceSchema from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🔍️trace/🧬️schema/🔣️.json";
import traceRows from "../../../🔍️trace/🧫️fixtures/🔣️.json";
const ajv=new Ajv({strict:true});for(const s of [pathSchema,imageSchema,rasterSchema,prepSchema,flatSchema,regionSchema,curveSchema,bitmapTraceSchema,documentTraceSchema])ajv.addSchema(s);const valid=ajv.compile(schema),validProgress=ajv.compile({$ref:schema.$id+"#/definitions/progress"}),validResult=ajv.compile({$ref:schema.$id+"#/definitions/result"});
function lift(v:any):any{return typeof v==="number"?binary64(v):Array.isArray(v)?v.map(lift):v&&typeof v==="object"?Object.fromEntries(Object.entries(v).map(([k,v])=>[k,lift(v)])):v;}
function input(row:typeof rows[number]):DocumentBooleanInput{const job=new DocumentSceneJob({...row.document,layers:lift(row.document.layers)} as any,row.preparation);while(!job.advance(4096).done){}return{plan:job.result(),limits:row.limits};}
function finish(value:DocumentBooleanInput,grant:number,observer?:(p:DocumentBooleanProgress)=>void){
 const job=new DocumentBooleanJob(value);expect(()=>job.result()).toThrow(/incomplete/i);let work=0;
 for(;;){const p=job.advance(grant);expect(validProgress(p)).toBe(true);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;observer?.(p);if(p.done){const result=job.result();expect(validResult(result)).toBe(true);return result;}expect(()=>job.result()).toThrow(/incomplete/i);}
}
function area(segments:any[]):number{let origin=[0,0],previous=[0,0],sum=0;for(const s of segments)if(s.kind==="move")origin=previous=s.to;else if(s.kind==="line"){sum+=((previous[0]!-origin[0]!)*(s.to[1]-origin[1]!)-(previous[1]!-origin[1]!)*(s.to[0]-origin[0]!))/2;previous=s.to;}return sum;}
for(const row of rows)test("real document boolean: "+row.name,()=>{
 const value=input(row),before=structuredClone(value);expect(valid(value)).toBe(true);let previous:any=null;
 for(const grant of [1,7,4096]){const result=finish(value,grant),node=result.nodes.find(n=>n.id==="result")!;expect(node.content.kind).toBe("path");const c=node.content as any;expect(c.segments.filter((s:any)=>s.kind==="move").length).toBe(row.expected.contours);expect(Math.abs(Math.abs(area(c.segments))-row.expected.area)).toBeLessThanOrEqual("tolerance" in row.expected?row.expected.tolerance!:1e-8);if(previous)expect(result).toEqual(previous);previous=result;}expect(value).toEqual(before);
});
test("actual document Boolean pixels match independent authored SVG",async()=>{
 for(const row of rows){const result=finish(input(row),4096),raster=new RasterSceneJob(resolvedSceneInput(result,{width:64,height:48,origin:[-32,-8],tolerance:.005,maxPixels:100000,maxSourceBytes:100000,maxBytes:100000,maxChunks:100}));while(!raster.advance(4096).done){}
  const scale=16,w=64*scale,h=48*scale,raw=await sharp(Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="'+w+'" height="'+h+'" viewBox="-32 -8 64 48"><path d="'+row.oracle.d+'" transform="matrix('+row.oracle.matrix.join(" ")+')" fill="red"/></svg>')).ensureAlpha().raw().toBuffer(),actual=raster.result().pixels;let delta=0;
  for(let at=0;at<64*48;at++){let sum=0;for(let y=0;y<scale;y++)for(let x=0;x<scale;x++)sum+=raw[((Math.floor(at/64)*scale+y)*w+at%64*scale+x)*4+3]!;delta=Math.max(delta,Math.abs(actual[at*4+3]!-Math.round(sum/(scale*scale))));}expect(delta,row.name).toBeLessThanOrEqual(3);
 }console.log("[DEBUG] Complete referenced Boolean documents matched independent SVG pixels, including result movement and nested/group geometry");
});
test("document Boolean dependencies, unsupported operands and caps fail privately",()=>{
 const value=input(rows[0]!);for(const patch of [{maxWork:0},{maxDepth:0},{maxRetainedSegments:0},{epsilon:0}])expect(()=>new DocumentBooleanJob({...value,limits:{...value.limits,...patch}})).toThrow();
 for(const kind of ["missing","cycle","unsupported","duplicate","depth","references","retained","work"]){const v=structuredClone(value);if(kind==="missing")(v.plan.nodes.at(-1)!.content as any).children=["absent"];if(kind==="cycle")(v.plan.nodes.at(-1)!.content as any).children=["result"];if(kind==="unsupported")v.plan.nodes[0]!.content={kind:"image",asset:"x",width:1,height:1};if(kind==="duplicate")v.plan.nodes[0]!.id="result";if(kind==="depth")v.limits.maxDepth=1;if(kind==="references")v.limits.maxReferences=1;if(kind==="retained")v.limits.maxRetainedSegments=1;if(kind==="work")v.limits.maxWork=1;
  const job=new DocumentBooleanJob(v);expect(()=>{while(!job.advance(4096).done){}}).toThrow();expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();
 }for(const budget of [0,-1,NaN,Infinity,.5])expect(()=>new DocumentBooleanJob(value).advance(budget)).toThrow();
});
test("every document Boolean phase cancels and published plans remain owned",()=>{
 const value=input(rows[0]!),seen=new Set<string>();finish(value,1,p=>{if(seen.has(p.phase))return;const job=new DocumentBooleanJob(value);job.advance(p.work);job.cancel();expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.advance(1)).toThrow(/cancel/i);seen.add(p.phase);});expect([...seen].sort()).toEqual(["indexing","validation","cycles","requirements","traversal","operands","geometry","remapping","publishing","complete"].sort());
 const job=new DocumentBooleanJob(value);while(!job.advance(4096).done){}const result=job.result(),before=structuredClone(result);job.cancel();expect(result).toEqual(before);
});
test("exact retained and work caps preserve complete output",()=>{const value=input(rows[0]!);let last!:DocumentBooleanProgress;const expected=finish(value,1,p=>last=p);expect(finish({...value,limits:{...value.limits,maxWork:last.work,maxRetainedSegments:last.segments}},7)).toEqual(expected);});
test("unrepresentable result accuracy refuses privately and empty singular results stay empty",()=>{
 const value=input(rows[0]!);value.plan.nodes.at(-1)!.transform=[1e8,0,0,1e8,0,0];const job=new DocumentBooleanJob(value);expect(()=>{while(!job.advance(4096).done){}}).toThrow(/precision/);expect(()=>job.result()).toThrow(/precision/);
 const empty=input(rows[8]!);(empty.plan.nodes.at(-1)!.content as any).referenceTransform=[0,0,0,0,0,0];expect((finish(empty,7).nodes.at(-1)!.content as any).segments).toEqual([]);
});
test("document Boolean async work yields and completion abort prevents publication",async()=>{const value=input(rows[0]!);let tick=false;setTimeout(()=>tick=true,0);expect(await resolveDocumentBooleans(value,{workBudget:7})).toEqual(finish(value,4096));expect(tick).toBe(true);const abort=new AbortController();await expect(resolveDocumentBooleans(value,{signal:abort.signal,onProgress:p=>{if(p.done)abort.abort();}})).rejects.toThrow(/cancel/i);});
test("real document pipeline resolves all Boolean fixtures under every grant",()=>{
 for(const row of rows)for(const grant of [1,7,4096]){const source={...row.document,layers:lift(row.document.layers)} as any,job=new DocumentRasterJob(source,row.preparation,{width:64,height:48,origin:[-32,-8],tolerance:.005,maxPixels:100000,maxSourceBytes:100000,maxBytes:100000,maxChunks:100},{booleans:row.limits,trace:traceRows[0]!.limits,maxWork:10000000});let partial=false,work=0;while(true){const p=job.advance(grant);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.resolution?.geometry&&!p.resolution.geometry.done){partial=true;expect(()=>job.result()).toThrow();}if(p.done)break;}const plan=finish(input(row),4096),raster=new RasterSceneJob(resolvedSceneInput(plan,{width:64,height:48,origin:[-32,-8],tolerance:.005,maxPixels:100000,maxSourceBytes:100000,maxBytes:100000,maxChunks:100}));while(!raster.advance(4096).done){}expect(job.result().pixels).toEqual(raster.result().pixels);if(grant===1)expect(partial).toBe(true);}
 console.log("[DEBUG] Actual document-to-pixels jobs resolve Boolean references and forward real partial curve/region progress");
});
test("document pipeline cancels real algorithm work and does not publish after complete observers",async()=>{
 const row=rows[0]!,source={...row.document,layers:lift(row.document.layers)} as any,viewport={width:16,height:16,origin:[0,0] as [number,number],tolerance:.005,maxPixels:100000,maxSourceBytes:100000,maxBytes:100000,maxChunks:100};
 const job=new DocumentRasterJob(source,row.preparation,viewport,{booleans:row.limits,trace:traceRows[0]!.limits,maxWork:10000000});while(job.advance(1).phase!=="algorithms"){}job.cancel();expect(()=>job.advance(1)).toThrow(/cancel/i);expect(()=>job.result()).toThrow(/cancel/i);
 for(const phase of ["algorithms","complete"]){const controller=new AbortController();await expect(rasterizeDocument(source,row.preparation,viewport,{booleans:row.limits,trace:traceRows[0]!.limits,maxWork:10000000},{workBudget:7,signal:controller.signal,onProgress:p=>{if(p.phase===phase)controller.abort();}})).rejects.toThrow(/cancel/i);}
});

/** 🧹️ Complete results own their mutable metadata; failed graphs keep private retirement state. */
import retirementRows from "../../🧫️fixtures/🧹️retirement/🔣️.json";
import {ScenePlanCloseJob} from "../../../🧹️retire/🟦️.ts";
const retirementValid=ajv.compile({$ref:schema.$id+"#/definitions/retirementProgress"});
test("document Boolean retirement composes actual children and keeps source and completed plans independent",()=>{
 for(const sample of retirementRows.cases)for(const grant of [1,7,4096]){
  const row=rows.find(r=>r.name===("source" in sample?sample.source:retirementRows.source))!,source=input(row);for(const n of source.plan.nodes)n.groups=[structuredClone(retirementRows.metadata.group)];const c=source.plan.nodes.at(-1)!.content as any;c.fill=structuredClone(retirementRows.metadata.fill);c.stroke=structuredClone(retirementRows.metadata.stroke);source.plan.assets=[structuredClone(retirementRows.metadata.asset)];
  if("maxWork" in sample)source.limits={...source.limits,maxWork:sample.maxWork!};if("maxRetainedSegments" in sample)source.limits={...source.limits,maxRetainedSegments:sample.maxRetainedSegments!};
  const before=structuredClone(source),job=new DocumentBooleanJob(source),state=job as any;
  if(sample.phase==="failure")expect(()=>job.advance(100000)).toThrow();else if(sample.phase!=="fresh"){
   if(sample.phase!=="cancelled"){let reached=false;for(let at=0;at<100000;at++){if(job.advance(1).phase===sample.phase){reached=true;break;}}expect(reached).toBe(true);}
   for(let at=0;at<sample.offset;at++)job.advance(1);
  }
  if(sample.phase==="failure"&&"maxWork" in sample)expect(state.ids.size).toBeGreaterThan(0);if(sample.phase==="cancelled")job.cancel();
  const published=sample.phase==="complete"?job.result():null,transferred=job.intoRetirement();if(sample.phase==="cancelled")expect(transferred.input.nodes).toHaveLength(0);else expect(transferred.input).toBe(source.plan);expect(transferred.output).toBe(published);if(sample.phase!=="cancelled")expect(transferred.input).toEqual(before.plan);
  expect(()=>job.result()).toThrow(/cancel/i);expect(()=>job.advance(1)).toThrow(/cancel/i);expect(()=>job.intoRetirement()).toThrow(/transferred/i);job.cancel();for(const n of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>transferred.job.advance(n)).toThrow(/grant/i);
  let work=0,done=false;for(let at=0;at<100000;at++){const beforeCount=state.operands.length,p=transferred.job.advance(grant);expect(beforeCount-state.operands.length).toBeLessThanOrEqual(grant);expect(retirementValid(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done){done=true;break;}}
  expect(done).toBe(true);expect(work).toBeGreaterThanOrEqual(retirementRows.minimumWork);expect(transferred.job.terminalIsEmpty()).toBe(true);expect(transferred.job.advance(1)).toEqual({phase:"complete",work,done:true});
  for(const field of ["ids","visited","visiting","cache","local","impact","quality"])expect(state[field].size).toBe(0);for(const field of ["stack","order","operands","raw","localResult","worldResult"])expect(state[field]).toHaveLength(0);expect(state.child).toBe(null);expect(state.operand).toBe(null);expect(source).toEqual(before);
  if(published){expect(validResult(published)).toBe(true);expect(published.assets).not.toBe(source.plan.assets);expect(published.nodes[0]!.groups).not.toBe(source.plan.nodes[0]!.groups);const out=published.nodes.at(-1)!.content as any;expect(out.fill.stops).not.toBe(c.fill.stops);expect(out.stroke.dash).not.toBe(c.stroke.dash);expect(out.fill).toEqual(c.fill);expect(out.stroke).toEqual(c.stroke);expect(published.assets[0]).not.toBe(source.plan.assets[0]);expect(published.nodes[0]!.transform).not.toBe(source.plan.nodes[0]!.transform);expect(published.nodes[0]!.groups[0]).not.toBe(source.plan.nodes[0]!.groups[0]);expect(out.fill.stops[0].color).not.toBe(c.fill.stops[0].color);expect(out.stroke.color).not.toBe(c.stroke.color);for(let at=0;at<published.nodes.length;at++){const original=source.plan.nodes[at]!.content,next=published.nodes[at]!.content;if(original.kind==="group"&&next.kind==="group")expect(next.children).not.toBe(original.children);}const independent=finish(source,grant),stable=structuredClone(independent),close=new ScenePlanCloseJob(published);while(!close.advance(grant).done){}expect(source).toEqual(before);expect(independent).toEqual(stable);const sourceClose=new ScenePlanCloseJob(transferred.input);while(!sourceClose.advance(grant).done){}expect(independent).toEqual(stable);const independentClose=new ScenePlanCloseJob(independent);while(!independentClose.advance(grant).done){}}
  console.log(`[DEBUG] Document Boolean ${sample.phase} retired private ownership in ${work} work units at grant ${grant}`);
 }
});
