/** 🧫️ Shared scene fixtures agree with independent SVG pixels and XML scalar decoding. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import {DOMParser} from "@xmldom/xmldom";
import {SvgWriteJob,type SvgWriteLimits} from "../🟦️.ts";
import rows from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import documents from "../../../../../../../../../🧬️schema/🎬️scene/📋️prepare/🧫️fixtures/🔣️.json";
import styles from "../../../../../../../../../✏️editor/🎮️commands/📤️export-document/🧫️fixtures/🎨️styles/🔣️.json";
import traces from "../../../../../../../../../🧬️schema/🎬️scene/🔍️trace/🧫️fixtures/🔣️.json";
import {prepareDocumentVector,type DocumentScenePlan} from "../../../../../../../../../🧬️schema/🎬️scene/📋️prepare/🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
const lift=(value:any):any=>typeof value==="number"?binary64(value):Array.isArray(value)?value.map(lift):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lift(item)])):value;
const algorithms={maxWork:1e7,trace:traces[0]!.limits,booleans:{tolerance:.005,epsilon:1e-8,maxDepth:32,maxReferences:256,maxEdges:65536,maxParameters:262144,maxAtomicEdges:65536,maxSegments:65536,maxRetainedSegments:262144,maxWork:1e7}};
const limits:SvgWriteLimits={viewBox:[0,0,2,1],maxOutputBytes:65536,maxWork:1e9};
const finish=(job:SvgWriteJob,grant:number)=>{let work=0;for(let at=0;at<1e6;at++){const progress=job.advance(grant);expect(progress.work-work).toBeLessThanOrEqual(grant);work=progress.work;if(progress.done){const pages=job.result();for(const page of pages)expect(page.length).toBeLessThanOrEqual(3072);return Buffer.concat(pages.map(page=>Buffer.from(page)));}expect(()=>job.result()).toThrow();}throw Error("SVG writer failed to terminate");};
const close=(job:SvgWriteJob)=>{const moved=job.intoRetirement();let work=0;for(let at=0;at<1e6;at++){const p=moved.job.advance(1);expect(p.work-work).toBe(1);work=p.work;if(p.done)return moved;}throw Error("SVG writer close failed to terminate");};
test("neutral authoritative SVG page writers match Sharp at every work grant",async()=>{
 const admitLimits=new Ajv({strict:true}).compile(schema);for(const row of rows)expect(admitLimits({viewBox:row.viewBox,maxOutputBytes:row.maxOutputBytes,maxWork:row.maxWork})).toBe(true);
 for(const row of rows)if(row.styles)expect(styles.some(style=>style.name===row.styles)).toBe(true);for(const row of rows)for(const grant of [1,7,4096]){const source=documents.find(item=>item.name===row.source)!,style=styles.find(item=>item.name===row.styles),document={...structuredClone(source.document),layers:lift(style?.layers??source.document.layers)},before=structuredClone(document),plan=await prepareDocumentVector(document,{...source.limits,maxNodes:16},algorithms),job=new SvgWriteJob(plan,{...row,viewBox:row.viewBox as [number,number,number,number]}),svg=finish(job,grant),pixels=await sharp(svg).ensureAlpha().raw().toBuffer();expect(pixels.length).toBe(row.expected.length);for(let at=0;at<pixels.length;at++)expect(Math.abs(pixels[at]!-row.expected[at]!)).toBeLessThanOrEqual(row.channelTolerance);expect(document).toEqual(before);expect(new DOMParser().parseFromString(svg.toString(),"image/svg+xml").documentElement.localName).toBe("svg");expect(close(job).output).not.toBeNull();console.log(`[DEBUG] Authoritative SVG ${row.styles||row.source}: grant=${grant}, bounded pages, Sharp pixels, XML root and unchanged source matched`);}
});
test("shared authoritative image pixels become bounded PNG data pages without source copies",async()=>{
 const pixels=new Uint8Array([255,0,0,255,0,0,255,128]),image={width:2,height:1,pixels},plan:DocumentScenePlan={assets:[{id:"image",image}],nodes:[{id:"i",sourcePath:[0],lockedAncestors:0,groups:[],transform:[1,0,0,1,0,0],visible:true,opacity:1,blendMode:"normal",content:{kind:"image",asset:"image",width:2,height:1}}]},job=new SvgWriteJob(plan,limits),svg=finish(job,1),xml=new DOMParser().parseFromString(svg.toString(),"image/svg+xml"),href=xml.getElementsByTagName("image")[0]!.getAttribute("href")!;expect([...await sharp(Buffer.from(href.slice(href.indexOf(",")+1),"base64")).ensureAlpha().raw().toBuffer()]).toEqual([...pixels]);expect(plan.assets[0]!.image.pixels).toBe(pixels);expect(close(job).output).not.toBeNull();expect([...pixels]).toEqual([255,0,0,255,0,0,255,128]);
});
test("scalar XML text and identifiers preserve escaped Unicode and multiline baselines",()=>{
 const content='A<&"🙂\r\nB',id='i<&"🙂',plan:DocumentScenePlan={assets:[],nodes:[{id,sourcePath:[0],lockedAncestors:0,groups:[],transform:[1,0,0,1,0,0],visible:true,opacity:1,blendMode:"normal",content:{kind:"text",content,x:4,y:5,size:10,fillRule:"nonzero",fill:{kind:"solid",color:[0,0,0,1]},stroke:null}}]},job=new SvgWriteJob(plan,limits),svg=finish(job,1),xml=new DOMParser().parseFromString(svg.toString(),"image/svg+xml"),spans=xml.getElementsByTagName("tspan");expect(xml.getElementsByTagName("g")[0]!.getAttribute("data-layer-id")).toBe(id);expect(spans.length).toBe(2);expect(spans[0]!.textContent).toBe('A<&"🙂');expect(spans[1]!.textContent).toBe("B");expect(spans[0]!.getAttribute("x")).toBe("4");expect(spans[1]!.getAttribute("y")).toBe("27");close(job);
});
test("every public writer phase refuses cancellation or capacity without partial publication",async()=>{
 const source=documents.find(item=>item.name==="rectangle")!;for(const stop of [0,1,7,40]){const plan=await prepareDocumentVector({...structuredClone(source.document),layers:lift(source.document.layers)},source.limits,algorithms),job=new SvgWriteJob(plan,limits);if(stop)job.advance(stop);job.cancel();expect(()=>job.result()).toThrow();expect(close(job).output).toBeNull();}for(const patch of [{maxOutputBytes:64},{maxWork:1},{viewBox:[0,0,0,1] as const}]){const plan=await prepareDocumentVector({...structuredClone(source.document),layers:lift(source.document.layers)},source.limits,algorithms),job=new SvgWriteJob(plan,{...limits,...patch});expect(()=>finish(job,1)).toThrow();expect(close(job).output).toBeNull();}
});
