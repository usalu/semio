/** 🧫️ Neutral scene RGBA, independent SVG groups and exact uncropped path equivalence. */
import {test,expect,spyOn} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import cases from "../../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import pathHandoffCases from "../../🧫️fixtures/🧹️path/🔣️.json";
import pathHandoffSchema from "../../🧬️schema/🧹️path/🔣️.json";
import imageHandoffCases from "../../🧫️fixtures/🧹️image/🔣️.json";
import imageHandoffSchema from "../../🧬️schema/🧹️image/🔣️.json";
import imageSources from "../../🧫️fixtures/🖼️images/🔣️.json";
import {AffineImageJob} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🎨️sampling/↗️affine/🟦️.ts";
import pathSchema from "../../../../🧮️geometry/📷️raster/🧬️schema/🔣️.json";
import compositingCases from "../../../🧩️compositing/🧫️fixtures/🔣️.json";
import {RasterSceneJob,rasterizeScene,type RasterSceneInput,type RasterSceneNode} from "../../🟦️.ts";
import {PathRasterJob,type PathRasterInput} from "../../../../🧮️geometry/📷️raster/🟦️.ts";
const ajv=new Ajv({strict:true});ajv.addSchema(pathSchema);const validate=ajv.compile(schema);
test("scene path pixels wait for the actual whole parent retirement before compositing",async()=>{
 expect(new Ajv({strict:true}).compile(pathHandoffSchema)(pathHandoffCases)).toBe(true);
 const original=PathRasterJob.prototype.intoRetirement;
 for(const handoff of pathHandoffCases)for(const grant of [1,7,4096]){const name=handoff.source,row=cases.find(c=>c.name===name)!,value=input(row.input),before=structuredClone(value),job=new RasterSceneJob(value),state=job as any,records:{job:any;output:any;work:number}[]=[];
  const spy=spyOn(PathRasterJob.prototype,"intoRetirement").mockImplementation(function(this:PathRasterJob){const result=original.call(this),record={...result,work:0};records.push(record);expect(result.output).not.toBeNull();const nodes=state.nodes,advance=result.job.advance.bind(result.job);result.job.advance=unit=>{expect(unit).toBe(1);expect(state.phase).toBe("pathCleanup");expect(state.painter).toBeNull();expect(state.painted).toBe(result.output);expect(state.nodes).toBe(nodes);const p=advance(unit);expect(p.work-record.work).toBe(1);record.work=p.work;return p;};return result;});
  try{let done=false;for(let at=0;at<2000000;at++){const p=job.advance(grant);if(p.done){done=true;break;}}expect(done).toBe(true);}finally{spy.mockRestore();}expect(records.length).toBe(handoff.parents);records.forEach(record=>{expect(record.job.terminalIsEmpty()).toBe(true);expect(record.work).toBeGreaterThanOrEqual(16);});expect(state.painterRetirement).toBeNull();expect(state.painted).toBeNull();expect([...job.result().pixels]).toEqual(row.expected);const reference=await sharp(Buffer.from(await svg(value))).ensureAlpha().raw().toBuffer();for(let at=0;at<reference.length;at++)expect(Math.abs(reference[at]!-job.result().pixels[at]!)).toBeLessThanOrEqual(2);expect(value).toEqual(before);process.stderr.write(`[DEBUG] Actual scene whole-path handoff ${name}: grant=${grant}, children=${records.length}, RGBA matched SVG\n`);
 }
});
test("scene sampled pixels wait for actual affine retirement before layer publication",async()=>{
 expect(new Ajv({strict:true}).compile(imageHandoffSchema)(imageHandoffCases)).toBe(true);const original=AffineImageJob.prototype.intoRetirement;
 for(const handoff of imageHandoffCases)for(const grant of [1,7,4096]){
  const row=imageSources.find(c=>c.name===handoff.source)!,value=input(row.input),before=structuredClone(value),job=new RasterSceneJob(value),state=job as any,records:{job:any;output:any;work:number}[]=[];
  const spy=spyOn(AffineImageJob.prototype,"intoRetirement").mockImplementation(function(this:AffineImageJob){const retired=original.call(this),record={...retired,work:0};records.push(record);expect(retired.output).not.toBeNull();const nodes=state.nodes,advance=retired.job.advance.bind(retired.job);retired.job.advance=unit=>{expect(unit).toBe(1);expect(state.phase).toBe("imageCleanup");expect(state.sampler).toBeNull();expect(state.sampled).toBe(retired.output);expect(state.nodes).toBe(nodes);const p=advance(unit);expect(p.work-record.work).toBe(1);record.work=p.work;return p;};return retired;});
  try{let done=false;for(let at=0;at<2000000;at++){if(job.advance(grant).done){done=true;break;}}expect(done).toBe(true);}finally{spy.mockRestore();}
  expect(records.length).toBe(handoff.parents);for(const record of records){expect(record.job.terminalIsEmpty()).toBe(true);expect(record.work).toBeGreaterThanOrEqual(11);}expect(state.samplerRetirement).toBeNull();expect(state.sampled).toBeNull();expect([...job.result().pixels]).toEqual(row.expected);expect(value).toEqual(before);const reference=await sharp(Buffer.from(await svg(value))).ensureAlpha().raw().toBuffer();expect(delta(job.result().pixels,reference)).toBeLessThanOrEqual(2);console.error(`[DEBUG] Actual scene affine handoff ${handoff.source}: grant=${grant}, children=${records.length}, RGBA matched independent SVG`);
 }
});
function input(value:any):RasterSceneInput {return {...value,nodes:value.nodes.map((n:any)=>({...n,content:n.content.kind==="pixels"?{kind:"pixels",image:{...n.content.image,pixels:new Uint8Array(n.content.image.pixels)}}:n.content}))};}
function complete(value:RasterSceneInput,budget=4096) {const job=new RasterSceneJob(value);let work=0;for(let steps=0;steps<2000000;steps++){const p=job.advance(budget);expect(p.work-work).toBeLessThanOrEqual(budget);work=p.work;if(p.done)return job.result();}throw Error("Scene did not terminate");}
function d(segments:PathRasterInput["segments"]):string {return segments.map((s:any)=>s.kind==="close"?"Z":s.kind==="move"?`M${s.to}`:s.kind==="line"?`L${s.to}`:s.kind==="quad"?`Q${s.ctrl} ${s.to}`:s.kind==="cubic"?`C${s.ctrl1} ${s.ctrl2} ${s.to}`:`A${s.rx} ${s.ry} ${s.rotation} ${Number(s.largeArc)} ${Number(s.sweep)} ${s.to}`).join(" ");}
async function svg(value:RasterSceneInput):Promise<string>{
 let body="",groups:string[]=[];let serial=0;const rgb=(c:readonly number[])=>`rgb(${c.slice(0,3).map(v=>v*255).join(",")})`;
 const css:Record<string,string>={colorDodge:"color-dodge",colorBurn:"color-burn",hardLight:"hard-light",softLight:"soft-light"};
 for(const n of value.nodes){
  let common=0;while(common<groups.length&&common<n.groups.length&&groups[common]===n.groups[common]!.id)common++;
  while(groups.length>common){body+="</g>";groups.pop();}
  for(const g of n.groups.slice(common)){body+=`<g opacity="${g.opacity}" style="isolation:isolate;mix-blend-mode:${css[g.blendMode]??g.blendMode}">`;groups.push(g.id);}
  if(!n.visible)continue;
  const m=[...n.transform];m[4]-=value.origin[0];m[5]-=value.origin[1];const c=n.content;
  const props=`transform="matrix(${m.join(" ")})" opacity="${n.opacity}" style="mix-blend-mode:${css[n.blendMode]??n.blendMode}"`;
  if(c.kind==="pixels"){const bytes=await sharp(c.image.pixels,{raw:{width:c.image.width,height:c.image.height,channels:4}}).png().toBuffer();body+=`<image ${props} width="${c.image.width}" height="${c.image.height}" href="data:image/png;base64,${bytes.toString("base64")}"/>`;}
  else {const f=c.fill,s=c.stroke;let paint="none",alpha=1;
   if(f?.kind==="solid"){paint=rgb(f.color);alpha=f.color[3];}
   else if(f){const key="ramp"+serial++,tag=f.kind==="linearGradient"?"linearGradient":"radialGradient",pos=f.kind==="linearGradient"?`x1="${f.x1}" y1="${f.y1}" x2="${f.x2}" y2="${f.y2}"`:`cx="${f.cx}" cy="${f.cy}" r="${f.r}"`;body+=`<defs><${tag} id="${key}" gradientUnits="userSpaceOnUse" ${pos}>${f.stops.map(stop=>`<stop offset="${stop.offset}" stop-color="${rgb(stop.color)}" stop-opacity="${stop.color[3]}"/>`).join("")}</${tag}></defs>`;paint=`url(#${key})`;}
   body+=`<path ${props} d="${d(c.segments)}" fill="${paint}" fill-opacity="${alpha}" fill-rule="${c.fillRule}" ${s?`stroke="${rgb(s.color)}" stroke-opacity="${s.color[3]}" stroke-width="${s.width}" stroke-linecap="${s.cap}" stroke-linejoin="${s.join}" stroke-miterlimit="4" ${s.dash?.some(v=>v>0)?`stroke-dasharray="${s.dash.join(" ")}"`:""}`:""}/>`;
  }
 }
 while(groups.length){body+="</g>";groups.pop();}
 return `<svg xmlns="http://www.w3.org/2000/svg" width="${value.width}" height="${value.height}">${body}</svg>`;
}
const delta=(a:Uint8Array,b:Uint8Array)=>{let max=0;for(let at=0;at<a.length;at+=4){max=Math.max(max,Math.abs(a[at+3]!-b[at+3]!));for(let c=0;c<3;c++)max=Math.max(max,Math.abs(a[at+c]!*a[at+3]!/255-b[at+c]!*b[at+3]!/255));}return max;};
for(const row of cases)test(row.name,async()=>{expect(validate(row.input)).toBe(true);const value=input(row.input),before=structuredClone(value);for(const grant of [1,7,4096])expect([...complete(value,grant).pixels]).toEqual(row.expected);expect(value).toEqual(before);const reference=await sharp(Buffer.from(await svg(value))).ensureAlpha().raw().toBuffer();expect(delta(new Uint8Array(row.expected),reference)).toBeLessThanOrEqual(2);});
test("painted scenes bound grants, cancel, keep published ownership and refuse failures",()=>{
 const value=input(cases[5]!.input);
 for(const steps of [0,1,3,10,30,100]){const job=new RasterSceneJob(value);expect(()=>job.result()).toThrow();for(let i=0;i<steps;i++)job.advance(1);job.cancel();expect(()=>job.advance(1)).toThrow(/cancel/);expect(()=>job.result()).toThrow(/cancel/);}
 const job=new RasterSceneJob(value);while(!job.advance(4096).done){}const pixels=job.result().pixels;job.cancel();expect([...pixels]).toEqual(cases[5]!.expected);
 const overflow=new RasterSceneJob({...value,maxPixels:1});expect(()=>overflow.advance(100000)).toThrow(/pixel/i);expect(()=>overflow.result()).toThrow();expect(()=>overflow.advance(1)).toThrow();
 for(const grant of [0,-1,1.5,NaN,Infinity])expect(()=>new RasterSceneJob(value).advance(grant)).toThrow();
 const invalids=[{width:0},{origin:[NaN,0]},{maxPixels:0},{maxPixels:67108865},{nodes:new Array(1025).fill(value.nodes[0])}];
 for(const patch of invalids)expect(()=>new RasterSceneJob({...value,...patch} as RasterSceneInput)).toThrow();
});
test("scene group scopes refuse reopening, inconsistent style and malformed content",()=>{
 const node=input(cases[4]!.input).nodes[0]!,plain={...node,id:"plain",groups:[]};
 for(const nodes of [[node,plain,{...node,id:"again"}],[node,{...node,id:"changed",groups:[{...node.groups[0]!,opacity:.75}]}],[{...node,content:{kind:"text"}}],[{...node,transform:[NaN,0,0,1,0,0]}],[{...node,blendMode:"bogus"}]]){const job=new RasterSceneJob({...input(cases[0]!.input),nodes} as RasterSceneInput);expect(()=>job.advance(100000)).toThrow();expect(()=>job.result()).toThrow();}
});
test("scene crops match uncropped affine curves, gradients and stroke regions",()=>{
 const paths:PathRasterInput["segments"][]=[
  [{kind:"move",to:[2,3]},{kind:"cubic",ctrl1:[13,-6],ctrl2:[-4,16],to:[10,9]},{kind:"close"}],
  [{kind:"move",to:[2,3]},{kind:"quad",ctrl:[9,15],to:[12,3]}],
  ...[false,true].flatMap(largeArc=>[false,true].map(sweep=>[{kind:"move",to:[2,3]},{kind:"arc",rx:7,ry:2,rotation:35,largeArc,sweep,to:[12,3]},{kind:"close"}] as PathRasterInput["segments"]))
 ];let count=0;
 for(const segments of paths)for(const cap of ["butt","round","square"] as const)for(const join of ["miter","round","bevel"] as const){
  const content={kind:"path" as const,segments,fillRule:"evenodd" as const,fill:{kind:"linearGradient" as const,x1:0,y1:0,x2:20,y2:12,stops:[{offset:0,color:[1,0,0,1] as [number,number,number,number]},{offset:1,color:[0,0,1,1] as [number,number,number,number]}]},stroke:{color:[.2,.6,.1,1] as [number,number,number,number],width:.75,cap,join,dash:[.8,.5]}};
  const value:RasterSceneInput={width:32,height:24,origin:[-.25,.5],tolerance:.001,maxPixels:768,maxSourceBytes:268439552,maxBytes:67108864,maxChunks:65536,assets:[],nodes:[{id:"path",groups:[],transform:count%2?[1,.2,.4,.8,5,4]:[-.8,.2,.3,.7,18,3],opacity:1,blendMode:"normal",visible:true,content}]};
  const full=new PathRasterJob({...value,...content,transform:value.nodes[0]!.transform});while(!full.advance(4096).done){}expect([...complete(value).pixels]).toEqual([...full.result().pixels]);count++;
 }
 console.error(`[DEBUG] ${count} cropped curve/stroke scenes matched their uncropped path output`);
},120000);
test("all authored blend scopes match the independent SVG compositing corpus",async()=>{
 for(const row of compositingCases){
  const value:RasterSceneInput={width:24,height:16,origin:[0,0],tolerance:.001,maxPixels:4096,maxSourceBytes:268439552,maxBytes:67108864,maxChunks:65536,assets:[],nodes:row.nodes.map((n:any)=>({id:n.id,groups:n.groups??[],transform:n.transform,opacity:n.opacity,blendMode:n.blendMode,visible:n.visible,content:{kind:"path",segments:n.segments,fillRule:n.fillRule??"nonzero",fill:n.fill??null,stroke:n.stroke??null}}))};
  const image=complete(value),actual=await sharp(image.pixels,{raw:{width:24,height:16,channels:4}}).flatten({background:"white"}).ensureAlpha().raw().toBuffer(),reference=await sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="24" height="16">${row.svg}</svg>`)).flatten({background:"white"}).ensureAlpha().raw().toBuffer();
  expect(delta(actual,reference),row.name).toBeLessThanOrEqual(2);
 }
 console.error(`[DEBUG] ${compositingCases.length} isolated/layer blend scenes matched independent SVG output`);
},120000);
test("async scenes yield progress and honor abort without publishing",async()=>{
 const value=input(cases[5]!.input),events:string[]=[];const timer=setTimeout(()=>events.push("timer"),0);const image=await rasterizeScene(value,{workBudget:7,onProgress:p=>events.push(p.phase)});clearTimeout(timer);expect([...image.pixels]).toEqual(cases[5]!.expected);expect(events.indexOf("timer")).toBeGreaterThanOrEqual(0);expect(events.indexOf("timer")).toBeLessThan(events.indexOf("complete"));
 const controller=new AbortController();await expect(rasterizeScene(value,{signal:controller.signal,workBudget:1,onProgress:()=>controller.abort()})).rejects.toThrow(/cancel/);let seen=false;await expect(rasterizeScene(value,{signal:controller.signal,onProgress:()=>{seen=true;}})).rejects.toThrow(/cancel/);expect(seen).toBe(false);
});
test("conservative ellipse bounds do not reject a visible short arc with large radii",()=>{
 const content={kind:"path" as const,segments:[{kind:"move" as const,to:[2,3] as [number,number]},{kind:"arc" as const,rx:1e9,ry:1e9,rotation:0,largeArc:false,sweep:true,to:[12,3] as [number,number]}],fillRule:"nonzero" as const,fill:null,stroke:{color:[1,0,0,1] as [number,number,number,number],width:.75,cap:"round" as const,join:"miter" as const}};
 const value:RasterSceneInput={width:16,height:8,origin:[0,0],tolerance:.001,maxPixels:128,maxSourceBytes:268439552,maxBytes:67108864,maxChunks:65536,assets:[],nodes:[{id:"short-arc",groups:[],transform:[1,0,0,1,0,0],opacity:1,blendMode:"normal",visible:true,content}]};
 const full=new PathRasterJob({...value,...content,transform:value.nodes[0]!.transform});while(!full.advance(4096).done){}
 expect([...complete(value).pixels]).toEqual([...full.result().pixels]);
});


test("scene waits for actual compositor retirement before completing output",async()=>{
 const rows=(await import("../../🧫️fixtures/🧹️compositing/🔣️.json")).default;
 
 const {CompositeJob}=await import("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🟦️.ts"),original=CompositeJob.prototype.intoRetirement;
 for(const handoff of rows)for(const grant of [1,7,4096]){
  const row=cases.find(v=>v.name===handoff.source)!,value=input(row.input),before=structuredClone(value),job=new RasterSceneJob(value),state=job as any;let closed:any=null,adopted=0,work=0;
  const spy=spyOn(CompositeJob.prototype,"intoRetirement").mockImplementation(function(this:CompositeJob){adopted++;closed=original.call(this);expect(closed.output).not.toBeNull();const advance=closed.job.advance.bind(closed.job);closed.job.advance=(unit:number)=>{expect(unit).toBe(1);expect(state.phase).toBe("compositingCleanup");expect(state.compositor).toBeNull();expect(state.output).toBe(closed.output);expect(()=>job.result()).toThrow(/incomplete/);const p=advance(unit);expect(p.work-work).toBe(1);work=p.work;return p;};return closed;});
  try{let done=false;for(let at=0;at<2000000;at++){const p=job.advance(grant);if(p.done){done=true;break;}}expect(done).toBe(true);expect(adopted).toBe(1);expect(work).toBeGreaterThanOrEqual(6);expect(closed.job.terminalIsEmpty()).toBe(true);expect(state.compositorRetirement).toBeNull();expect(job.result()).toBe(closed.output);expect([...job.result().pixels]).toEqual(row.expected);expect(value).toEqual(before);const reference=await sharp(Buffer.from(await svg(value))).ensureAlpha().raw().toBuffer();expect(delta(job.result().pixels,reference)).toBeLessThanOrEqual(2);console.error(`[DEBUG] Actual scene compositor handoff ${row.name}: grant=${grant} work=${work} RGBA matched independent SVG`);}finally{spy.mockRestore();}
 }
});
