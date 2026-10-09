/** 🧫️ Shared PNG export documents preserve exact samples and independent Sharp SVG pixels. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import rows from "../../🧫️fixtures/🔣️.json";
import styles from "../../🧫️fixtures/🎨️styles/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import documents from "../../../../../🧬️schema/🎬️scene/📋️prepare/🧫️fixtures/🔣️.json";
import traces from "../../../../../🧬️schema/🎬️scene/🔍️trace/🧫️fixtures/🔣️.json";
import {DrawingPngExportJob,exportDrawingPng,DRAWING_PNG_LIMITS,drawingPngExtent} from "../../🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
const lift=(value:any):any=>typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lift(item)])):value;
const algorithms={maxWork:1e7,trace:traces[0]!.limits,booleans:{tolerance:.005,epsilon:1e-8,maxDepth:32,maxReferences:256,maxEdges:65536,maxParameters:262144,maxAtomicEdges:65536,maxSegments:65536,maxRetainedSegments:262144,maxWork:1e7}};
const source=(name="rectangle")=>{const row=documents.find(row=>row.name===name)!;return{row,document:{...structuredClone(row.document),layers:lift(row.document.layers),artboard:{width:binary64(2),height:binary64(1)}}};};
const close=(job:DrawingPngExportJob)=>{const moved=job.intoRetirement();let work=0;for(let at=0;at<1e6;at++){const p=moved.job.advance(1);expect(p.work-work).toBe(1);work=p.work;if(p.done)return moved;}throw Error("PNG retirement did not terminate");};
const finish=(job:DrawingPngExportJob,grant:number)=>{let work=0;for(let at=0;at<1e6;at++){const p=job.advance(grant);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)return job.result();expect(()=>job.result()).toThrow();}throw Error("PNG export did not terminate");};
test("neutral PNG document fixture and Sharp pixels agree at every work grant",async()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.compile({$ref:schema.$id+"#/definitions/fixture"})(rows)).toBe(true);
 for(const row of rows)for(const grant of [1,7,4096]){const {document,row:scene}=source(row.source),before=structuredClone(document),job=new DrawingPngExportJob(document,scene.limits,algorithms,row),bytes=finish(job,grant),decoded=await sharp(bytes).ensureAlpha().raw().toBuffer({resolveWithObject:true});expect(decoded.info.width).toBe(row.width);expect(decoded.info.height).toBe(row.height);expect([...decoded.data]).toEqual(row.expected);const svg=await sharp(Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="2" height="1"><rect width="2" height="1" fill="red"/></svg>')).ensureAlpha().raw().toBuffer();expect([...decoded.data]).toEqual([...svg]);expect(document).toEqual(before);const moved=close(job);expect(moved.output).toBe(bytes);console.log(`[DEBUG] Actual document PNG ${row.name}: grant=${grant} Sharp decoded/SVG samples matched; source unchanged; private close terminal empty`);}
});
test("PNG artboard scaling and white background preserve requested pixel extent",async()=>{
 for(const transparent of [true,false]){const {document,row}=source();(document.layers[0] as any).attributes.fill.color[3]=binary64(.5);const job=new DrawingPngExportJob(document,row.limits,algorithms,{width:4,height:2,transparent}),bytes=finish(job,7),decoded=await sharp(bytes).ensureAlpha().raw().toBuffer();expect(decoded.length).toBe(32);for(let at=0;at<decoded.length;at+=4)expect([...decoded.subarray(at,at+4)]).toEqual(transparent?[255,0,0,128]:[255,127,127,255]);close(job);}
});
test("authoritative PNG retains gradients, evenodd holes, blend modes and isolated opacity",async()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.compile({$ref:schema.$id+"#/definitions/styleFixtures"})(styles)).toBe(true);
 for(const sample of styles){const {document,row}=source();document.layers=lift(sample.layers);const before=structuredClone(document),limits={...row.limits,maxNodes:16},job=new DrawingPngExportJob(document,limits,algorithms),bytes=finish(job,7),actual=await sharp(bytes).ensureAlpha().raw().toBuffer(),expected=await sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="2" height="1">${sample.svg}</svg>`)).ensureAlpha().raw().toBuffer();expect(actual.length).toBe(expected.length);for(let at=0;at<actual.length;at++)expect(Math.abs(actual[at]!-expected[at]!)).toBeLessThanOrEqual(sample.channelTolerance);for(let at=0;at<actual.length;at++)expect(Math.abs(actual[at]!-sample.expected[at]!)).toBeLessThanOrEqual(sample.channelTolerance);expect(document).toEqual(before);close(job);console.log(`[DEBUG] Actual authoritative PNG ${sample.name}: independent Sharp SVG pixels agree within two channel units`);}
});
test("PNG export refuses source, pixel, work and encoded output limits without publication",()=>{
 const {document,row}=source();for(const patch of [{maxPixels:1},{maxWork:1},{maxEncodedBytes:8}]){let job:DrawingPngExportJob|undefined;expect(()=>{job=new DrawingPngExportJob(document,row.limits,algorithms,{}, {...DRAWING_PNG_LIMITS,...patch});finish(job,1);}).toThrow();if(job){expect(()=>job!.result()).toThrow();job.cancel();expect(close(job).output).toBeNull();}}
 const image=source(documents[7]!.name),limited=new DrawingPngExportJob(image.document,image.row.limits,algorithms,{}, {...DRAWING_PNG_LIMITS,maxSourceBytes:1});expect(()=>finish(limited,1)).toThrow();limited.cancel();expect(close(limited).output).toBeNull();
 for(const options of [{width:0},{height:NaN},{width:16385},{height:1.5},{transparent:"yes"}])expect(()=>drawingPngExtent(document,options as any)).toThrow();
},120000);
test("PNG observers cancel before, during and after encoding and never publish partial downloads",async()=>{
 const {document,row}=source();for(const target of ["raster","background","encode","closing","complete"]){const controller=new AbortController();let reached=false;await expect(exportDrawingPng(document,row.limits,algorithms,{transparent:false},{workBudget:1,signal:controller.signal,onProgress:progress=>{if(progress.phase===target){reached=true;controller.abort();}}})).rejects.toThrow(/cancel/i);expect(reached).toBe(true);}
 const controller=new AbortController();controller.abort();await expect(exportDrawingPng(document,row.limits,algorithms,{}, {signal:controller.signal})).rejects.toThrow(/cancel/i);
},120000);

test("neutral PNG admission refusal and cancellation retain actual source and candidate",async()=>{
 const root="../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/📷️png/✍️encode/";
 const fixture=await Bun.file(new URL(root+"🧫️fixtures/🔣️.json",import.meta.url)).json(),encodingSchema=await Bun.file(new URL(root+"🧬️schema/🔣️.json",import.meta.url)).json();
 expect(new Ajv({strict:true}).compile(encodingSchema.definitions.admissionFixture)(fixture)).toBe(true);
 const {PngEncodeJob}=await import("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/📷️png/✍️encode/🟦️.ts");
 const image={width:fixture.width,height:fixture.height,pixels:Uint8Array.from(fixture.pixels)},source=image.pixels;
 expect(()=>new PngEncodeJob(image,fixture.maximumBytes)).toThrow(/byte limit/i);expect(image.pixels).toBe(source);expect([...source]).toEqual(fixture.pixels);
 const job=new PngEncodeJob(image),candidate=(job as any).output as Uint8Array;job.advance(1);const decoded=await sharp(job.result()).ensureAlpha().raw().toBuffer();expect([...decoded]).toEqual(fixture.pixels);
 job.cancel();expect(()=>job.result()).toThrow(/cancel/i);expect((job as any).output).toBe(candidate);expect((job as any).image.pixels).toBe(source);
 const moved=job.intoRetirement();expect(moved.output).toBeNull();for(let at=0;at<4&&!moved.job.terminalIsEmpty();at++)moved.job.advance(1);expect(moved.job.terminalIsEmpty()).toBe(true);
 console.log("[DEBUG] Neutral PNG refusal preserved source and cancellation retained original candidate until explicit close; independent Sharp decoder matched source");
});
test("PNG completed producer custody survives normal publication and cancellation",()=>{
 const {document,row}=source(),job=new DrawingPngExportJob(document,row.limits,algorithms),bytes=finish(job,1),privateJob=job as any;
 expect(privateJob.retiredChildren.length).toBe(2);const children=[...privateJob.retiredChildren];expect(children.every(child=>!child.terminalIsEmpty())).toBe(true);expect(privateJob.output).toBe(bytes);
 job.cancel();expect(privateJob.output).toBe(bytes);expect(privateJob.retiredChildren).toEqual(children);expect(()=>job.result()).toThrow(/cancel/i);expect(close(job).output).toBeNull();expect(children.every(child=>child.terminalIsEmpty())).toBe(true);
 console.log("[DEBUG] PNG retained original completed raster and encoder owners through publication and cancellation until explicit final close");
});
test("neutral source admission refusals preserve unchanged document and no candidate",async()=>{
 const samples=(await import("../../../../../🧬️schema/🎬️scene/📋️prepare/🧫️fixtures/🛂️admission/🔣️.json")).default,preparedSchema=(await import("../../../../../🧬️schema/🎬️scene/📋️prepare/🧬️schema/🔣️.json")).default;
 expect(new Ajv({strict:true}).compile(preparedSchema.definitions.ownedAdmissionFixtures)(samples)).toBe(true);
 const {DocumentVectorJob,DocumentRasterJob}=await import("../../../../../🧬️schema/🎬️scene/📋️prepare/🟦️.ts");
 for(const sample of samples){const {document,row}=source(sample.source),before=structuredClone(document),scene={...row.limits},resolution=structuredClone(algorithms),viewport={width:2,height:1,origin:[0,0] as[number,number],tolerance:.01,maxPixels:100,maxSourceBytes:10000};let job:InstanceType<typeof DocumentVectorJob>|InstanceType<typeof DocumentRasterJob>|undefined;
  switch(sample.limit){case"maxNodes":scene.maxNodes=0;break;case"maxWork":resolution.maxWork=0;break;case"maxEdges":resolution.booleans.maxEdges=0;break;case"width":viewport.width=0;break;case"maxPixels":viewport.maxPixels=0;break;}
  expect(()=>{job=sample.producer==="vector"?new DocumentVectorJob(document,scene,resolution):new DocumentRasterJob(document,scene,viewport,resolution);for(let at=0;at<100000;at++)if(job.advance(1).done)break;}).toThrow();expect(document).toEqual(before);if(job){expect(()=>job!.result()).toThrow();const moved=job.intoRetirement();expect(moved.output).toBeNull();for(let at=0;at<100000&&!moved.job.terminalIsEmpty();at++)moved.job.advance(1);expect(moved.job.terminalIsEmpty()).toBe(true);}
 }
 console.log("[DEBUG] Shared vector/raster source admission refusal fixture preserved original documents and withheld every candidate");
});