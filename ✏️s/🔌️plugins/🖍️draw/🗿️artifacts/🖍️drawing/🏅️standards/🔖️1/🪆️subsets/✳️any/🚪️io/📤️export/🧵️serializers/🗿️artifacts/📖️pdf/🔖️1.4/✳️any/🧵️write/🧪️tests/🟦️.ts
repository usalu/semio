import {bank,close as closeOwner,conserved} from "../../../../../../../../../🧬️schema/🎬️scene/🗺️plan-model/🎟️owned/🧪️tests/owned.test.ts";
import type {ScenePlanRead,ScenePlanNode} from "../../../../../../../../../🧬️schema/🎬️scene/🗺️plan-model/🎟️owned/🟦️.ts";
import {ScenePlanCloseJob} from "../../../../../../../../../🧬️schema/🎬️scene/🧹️retire/🟦️.ts";
import {DocumentSceneJob,type DocumentSceneLimits,type DrawingSceneSource} from "../../../../../../../../../🧬️schema/🎬️scene/📋️prepare/🎟️owned/🟦️.ts";
/** 🧫️ PDF.js renders actual paged writer bytes against neutral pixels and Sharp SVG. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import {pdfNodePixels as renderNode,pdfPixels as render} from "./🔮️oracle/🟦️.ts";
import {PdfWriteJob,type PdfWriteLimits} from "../🟦️.ts";
import cancellation from "./🧫️fixtures/⏹️cancellation/🔣️.json";
import rows from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import documents from "../../../../../../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json";
import paintStyles from "./🧫️fixtures/🎨️paint/🔣️.json";
import commonStyles from "../../../../../../../../../✏️editor/🎮️commands/📤️export-document/🧪️tests/🧫️fixtures/🎨️styles/🔣️.json";
import traces from "../../../../../../../../../🧪️tests/🧫️fixtures/🎬️scene/🔍️trace/🔣️.json";


import {drawingArtifactFixture,drawingLayersFixture} from "../../../../../../../../../🧪️tests/🧫️fixtures/🟦️.ts";
const styles=[...commonStyles,...paintStyles];
const algorithms={maxWork:1e7,trace:traces[0]!.limits,booleans:{tolerance:.005,epsilon:1e-8,maxDepth:32,maxReferences:256,maxEdges:65536,maxParameters:262144,maxAtomicEdges:65536,maxSegments:65536,maxRetainedSegments:262144,maxWork:1e7}};
const limits:PdfWriteLimits={viewBox:[0,0,2,1],maxOutputBytes:65536,maxWork:1e9};
function prepare(document:DrawingSceneSource,limits:DocumentSceneLimits,w:ReturnType<typeof bank>){const job=new DocumentSceneJob(document,limits,algorithms.maxWork);let done=false;for(let at=0;!done&&at<1e6;at++)done=w.run(job,work=>job.advance(1,work)).result.done;expect(done).toBe(true);const moved=job.intoRetirement();closeOwner(moved.job,w);if(moved.output===null)throw Error("Original PDF scene preparation failed");return moved.output;}
function finish(job:PdfWriteJob,grant:number,w:ReturnType<typeof bank>){let work=0;for(let at=0;at<1e6;at++){const p=w.run(job,ledger=>job.advance(grant,ledger),grant).result;expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done){const pages=job.result();return Buffer.concat(Array.from({length:pages.length},(_,index)=>{const page=pages.at(index);if(page===undefined)throw Error("Original PDF page absent");expect(page.length).toBeLessThanOrEqual(3072);return Buffer.from(page);}));}expect(()=>job.result()).toThrow();}throw Error("PDF writer failed to terminate");}
function close(job:PdfWriteJob,w:ReturnType<typeof bank>){const moved=job.intoRetirement();closeOwner(moved.job,w);if(moved.output!==null)closeOwner(moved.output,w);return moved;}
test("neutral PDF bodies and xref pages match independent renderer at every work grant",async()=>{
 const admitLimits=new Ajv({strict:true}).compile(schema);for(const row of rows)expect(admitLimits({viewBox:row.viewBox,maxOutputBytes:row.maxOutputBytes,maxWork:row.maxWork})).toBe(true);for(const row of rows)if(row.styles)expect(styles.some(style=>style.name===row.styles)).toBe(true);
 for(const row of rows)for(const grant of [1,7,4096]){const source=documents.find(item=>item.name===row.source)!,style=styles.find(item=>item.name===row.styles),document={...drawingArtifactFixture(source.document),layers:drawingLayersFixture(style?.layers??source.document.layers)},before=structuredClone(document),w=bank(),owner=prepare(document,{...source.limits,maxNodes:16},w),job=new PdfWriteJob(owner.borrow(),{...row,viewBox:row.viewBox as [number,number,number,number]}),bytes=finish(job,grant,w),pixels=await render(bytes),reference=await sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="2" height="1">${style?.svg??'<rect width="2" height="1" fill="red"/>'}</svg>`)).flatten({background:"white"}).ensureAlpha().raw().toBuffer();expect(pixels.length).toBe(row.expected.length);if(row.styles!=="gradient alpha"){const nodePixels=await renderNode(bytes);for(let at=0;at<pixels.length;at++)expect(Math.abs(pixels[at]!-nodePixels[at]!)).toBeLessThanOrEqual(row.channelTolerance);}for(let at=0;at<pixels.length;at++){expect(Math.abs(pixels[at]!-row.expected[at]!)).toBeLessThanOrEqual(row.channelTolerance);expect(Math.abs(pixels[at]!-reference[at]!)).toBeLessThanOrEqual(row.channelTolerance);}expect(document).toEqual(before);expect(close(job,w).output).not.toBeNull();closeOwner(new ScenePlanCloseJob(owner),w);conserved(w);}
},120000);
test("PDF image channel streams preserve original shared raster and alpha",async()=>{
 const pixels=new Uint8Array([255,0,0,255,0,0,255,128]),image={width:2,height:1,pixels},plan:ScenePlanRead={assets:[{id:"image",image}],nodes:[{id:"i",sourcePath:[0],lockedAncestors:0,groups:[],transform:[1,0,0,1,0,0],visible:true,opacity:1,blendMode:"normal",content:{kind:"image",asset:"image",width:2,height:1}}] satisfies ScenePlanNode[]},w=bank(),job=new PdfWriteJob(plan,limits),bytes=finish(job,1,w),actual=await render(bytes);expect([...actual]).toEqual([255,0,0,255,127,127,255,255]);expect(plan.assets.at(0)!.image.pixels).toBe(pixels);expect(close(job,w).output).not.toBeNull();conserved(w);expect([...pixels]).toEqual([255,0,0,255,0,0,255,128]);
});
test("PDF cancellation and byte/work refusal retain candidates until explicit close",async()=>{
 const source=documents.find(item=>item.name==="rectangle")!;for(const stop of [0,1,7,40,300]){const w=bank(),owner=prepare(drawingArtifactFixture(source.document),source.limits,w),job=new PdfWriteJob(owner.borrow(),limits);if(stop)for(let at=0;at<stop;at++)w.run(job,work=>job.advance(1,work));job.cancel();expect(()=>job.result()).toThrow();expect(close(job,w).output).toBeNull();closeOwner(new ScenePlanCloseJob(owner),w);conserved(w);}for(const patch of [{maxOutputBytes:64},{maxWork:1},{viewBox:[0,0,0,1] as const}]){const w=bank(),owner=prepare(drawingArtifactFixture(source.document),source.limits,w),job=new PdfWriteJob(owner.borrow(),{...limits,...patch});expect(()=>finish(job,1,w)).toThrow();expect(close(job,w).output).toBeNull();closeOwner(new ScenePlanCloseJob(owner),w);conserved(w);}
});

test("shared PDF cancellation phases keep unpublished pages until explicit retirement",async()=>{const source=documents.find(item=>item.name==="rectangle")!,style=styles.find(item=>item.name===cancellation.styles)!;for(const phase of cancellation.phases){const document={...drawingArtifactFixture(source.document),layers:drawingLayersFixture(style.layers)},w=bank(),owner=prepare(document,source.limits,w),job=new PdfWriteJob(owner.borrow(),limits);let reached=false;for(let turn=0;turn<100000;turn++){const progress=w.run(job,work=>job.advance(cancellation.grant,work),cancellation.grant).result;if(progress.phase===phase){reached=true;break;}if(progress.done)break;}expect(reached).toBe(true);job.cancel();expect(()=>job.result()).toThrow();expect(close(job,w).output).toBeNull();closeOwner(new ScenePlanCloseJob(owner),w);conserved(w);}});