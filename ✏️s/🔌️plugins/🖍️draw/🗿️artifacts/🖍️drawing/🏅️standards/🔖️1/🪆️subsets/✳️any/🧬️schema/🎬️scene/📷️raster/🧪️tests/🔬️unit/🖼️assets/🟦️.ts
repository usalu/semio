const neutral=(value:any):any=>value instanceof Uint8Array?Array.from(value):Array.isArray(value)?value.map(neutral):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,neutral(item)])):value;
/** 🖼️ Shared encoded scene fixtures preserve private image composition and independent source/filter/blend output. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import draft7 from "ajv/dist/refs/json-schema-draft-07.json" with {type:"json"};
import sharp from "sharp";
import {PNG} from "pngjs";
import cases from "../../../🧫️fixtures/🖼️assets/🔣️.json";
import invalid from "../../../🧫️fixtures/🖼️assets/⚠️invalid/🔣️.json";
import schema from "../../../🧬️schema/🔣️.json";
import pathSchema from "../../../../../🧮️geometry/📷️raster/🧬️schema/🔣️.json";
import {RasterSceneJob,rasterizeScene,type RasterSceneInput} from "../../../🟦️.ts";
import {areaOracle,canvasOracle,delta} from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🎨️sampling/↗️affine/🧪️tests/🔭️oracles/🟦️.ts";
import imageSchema from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧬️schema/🔣️.json";
const ajv=new Ajv({strict:true}).addMetaSchema(draft7);ajv.addSchema(pathSchema);ajv.addSchema(imageSchema);ajv.addSchema(schema);const validate=ajv.compile(schema),validateProgress=ajv.compile({$ref:schema.$id+"#/definitions/progress"});
const complete=(value:RasterSceneInput,grant=4096)=>{const job=new RasterSceneJob(value);let work=0;for(let i=0;i<2000000;i++){const p=job.advance(grant);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)return {image:job.result(),progress:p};}throw Error("Encoded scene did not finish");};
async function reference(value:RasterSceneInput){
 let body="",groups:string[]=[];
 for(const n of value.nodes){
  let common=0;while(common<groups.length&&common<n.groups.length&&groups[common]===n.groups[common]!.id)common++;
  while(groups.length>common){body+="</g>";groups.pop();}
  for(const g of n.groups.slice(common)){body+=`<g opacity="${g.opacity}" style="isolation:isolate;mix-blend-mode:${g.blendMode}">`;groups.push(g.id);}
  const c=n.content;
  if(c.kind==="path"){const f=c.fill;if(f?.kind!=="solid")throw Error("Expected solid oracle path");const d=c.segments.map(s=>s.kind==="close"?"Z":s.kind==="move"?`M${s.to}`:s.kind==="line"?`L${s.to}`:s.kind==="quad"?`Q${s.ctrl} ${s.to}`:s.kind==="cubic"?`C${s.ctrl1} ${s.ctrl2} ${s.to}`:`A${s.rx} ${s.ry} ${s.rotation} ${Number(s.largeArc)} ${Number(s.sweep)} ${s.to}`).join(" ");body+=`<path d="${d}" transform="matrix(${n.transform.join(" ")})" fill="rgb(${f.color.slice(0,3).map(v=>v*255).join(",")})" opacity="${n.opacity*f.color[3]}"/>`;continue;}
  if(c.kind!=="image"||!n.visible||n.opacity===0||n.groups.some(g=>g.opacity===0)||n.transform[0]*n.transform[3]-n.transform[1]*n.transform[2]===0)continue;
  const image=value.assets.find(a=>a.id===c.asset)!.image,m=n.transform,sx=c.width/image.width,sy=c.height/image.height,transform=[m[0]*sx,m[1]*sx,m[2]*sy,m[3]*sy,m[4],m[5]];
  const det=transform[0]!*transform[3]!-transform[1]!*transform[2]!,u=transform[3]!**2+transform[1]!**2,w=transform[2]!**2+transform[0]!**2,cross=-transform[3]!*transform[2]!-transform[1]!*transform[0]!;
  const shrink=(u+w+Math.hypot(u-w,2*cross))/(2*det*det)>1+1e-12,plane={source:{...image,pixels:Uint8Array.from(image.pixels)},width:value.width,height:value.height,origin:value.origin,transform,sampling:"auto" as const};
  const pixels=image.width*image.height===1||shrink?areaOracle(plane):canvasOracle(plane),png=await sharp(pixels,{raw:{width:value.width,height:value.height,channels:4}}).png().toBuffer();
  body+=`<image width="${value.width}" height="${value.height}" opacity="${n.opacity}" style="mix-blend-mode:${n.blendMode}" href="data:image/png;base64,${png.toString("base64")}"/>`;
 }
 while(groups.length){body+="</g>";groups.pop();}
 return sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${value.width}" height="${value.height}">${body}</svg>`)).ensureAlpha().raw().toBuffer();
}
for(const row of cases)test(row.name,async()=>{
 expect(validate(row.input)).toBe(true);const value={...row.input,assets:row.input.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))} as RasterSceneInput,before=structuredClone(value);
 for(const grant of [1,7,4096]){const result=complete(value,grant);expect(Array.from(result.image.pixels)).toEqual(row.expected);expect(result.progress.admittedImages).toBe(row.expectedAdmittedImages);expect(result.progress.sourceBytes).toBe(row.expectedSourceBytes);expect(result.progress.assets).toBe(value.assets.length);if("expectedAllocatedPixels" in row)expect(result.progress.pixels).toBe(row.expectedAllocatedPixels);}
 expect(value).toEqual(before);expect(delta(new Uint8Array(row.expected),await reference(value))).toBeLessThanOrEqual(2);
});
for(const row of invalid)test("refuse encoded scene "+row.name,()=>{
 let job:RasterSceneJob|undefined;expect(()=>{job=new RasterSceneJob(row.input as RasterSceneInput);while(!job.advance(4096).done){}job.result();}).toThrow();
 if(job){expect(()=>job!.result()).toThrow();expect(()=>job!.advance(1)).toThrow();}
});
test("asset admission counts UTF-8 source bytes and abort covers every live preparation phase",async()=>{
 const original=cases[5]!.input,value={...original,assets:original.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))} as RasterSceneInput,probe=new RasterSceneJob(value),seen=new Set<string>();let steps=0;
 for(;;){const p=probe.advance(1);steps++;if(!seen.has(p.phase)&&!p.done){seen.add(p.phase);const job=new RasterSceneJob(value);job.advance(steps);job.cancel();expect(()=>job.result()).toThrow(/cancel/);expect(()=>job.advance(1)).toThrow(/cancel/);}if(p.done)break;}
 expect(seen.has("assets")).toBe(true);expect(seen.has("nodes")).toBe(true);expect(seen.has("compositing")).toBe(true);
 const unused={...cases[0]!.input,nodes:[],assets:[{id:"é😀",image:{width:1,height:1,pixels:new Uint8Array([255,0,0,255])}}],maxSourceBytes:4} as RasterSceneInput;
 expect(complete(unused,1).progress.sourceBytes).toBe(4);expect(()=>complete({...unused,maxSourceBytes:3},1)).toThrow(/sample byte/i);
 const controller=new AbortController();let timer=false;setTimeout(()=>{timer=true;},0);
 await expect(rasterizeScene(value,{signal:controller.signal,workBudget:7,onProgress:p=>{if(p.phase==="compositing")controller.abort();}})).rejects.toThrow(/cancel/);expect(timer).toBe(true);
 const late=new AbortController();await expect(rasterizeScene(value,{signal:late.signal,onProgress:p=>{if(p.done)late.abort();}})).rejects.toThrow(/cancel/);
 console.info("[DEBUG] Encoded scene admission/source/filter/composition phases cancelled without publication; actual UTF-8 budgets verified");
});

test("encoded image layers preserve every blend mode above a painted path",async()=>{
 const original=cases[cases.length-1]!.input,base={...original,assets:original.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))} as RasterSceneInput;
 const css:Record<string,string>={colorDodge:"color-dodge",colorBurn:"color-burn",hardLight:"hard-light",softLight:"soft-light"};
 for(const blendMode of ["normal","multiply","screen","overlay","darken","lighten","colorDodge","colorBurn","hardLight","softLight","difference","exclusion","hue","saturation","color","luminosity"] as const){
  const nodes=base.nodes.map((n,i)=>i?{...n,blendMode}:{...n,content:{kind:"path" as const,segments:n.content.kind==="path"?n.content.segments:[],fillRule:"nonzero" as const,fill:{kind:"solid" as const,color:[.25,.5,.75,1] as [number,number,number,number]},stroke:null}}),value={...base,nodes};
  const actual=complete(value,7).image.pixels,data=PNG.sync.write({width:base.assets[0]!.image.width,height:base.assets[0]!.image.height,data:Buffer.from(base.assets[0]!.image.pixels)} as PNG).toString("base64"),svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"><rect width="1" height="1" fill="rgb(63.75,127.5,191.25)"/><image width="1" height="1" opacity=".5" style="mix-blend-mode:${css[blendMode]??blendMode}" href="data:image/png;base64,${data}"/></svg>`,expected=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
  expect(delta(actual,expected),blendMode).toBeLessThanOrEqual(2);
 }
 console.info("[DEBUG] Encoded assets above painted paths matched independent SVG for all sixteen blend modes");
});

test("scene retains admitted images while partial composition remains private",()=>{
 const row=cases[31]!,value={...row.input,assets:row.input.assets.map(a=>({...a,image:{...a.image,pixels:Uint8Array.from(a.image.pixels)}}))} as RasterSceneInput,before=structuredClone(value),job=new RasterSceneJob(value);let partial=false;
 for(;;){const p=job.advance(1);expect(validateProgress(neutral(p))).toBe(true);if(p.phase==="compositing"&&!p.done){partial=true;expect(()=>job.result()).toThrow();}if(p.done)break;}
 expect(partial).toBe(true);expect(value).toEqual(before);expect([...job.result().pixels]).toEqual(row.expected);
 console.info("[DEBUG] Admitted image owners remain intact through private bounded scene composition");
});
