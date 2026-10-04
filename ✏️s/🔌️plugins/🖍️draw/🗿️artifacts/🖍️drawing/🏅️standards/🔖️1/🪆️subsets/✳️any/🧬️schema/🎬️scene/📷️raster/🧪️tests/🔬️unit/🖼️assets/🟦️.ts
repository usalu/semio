/** 🖼️ Shared encoded scene fixtures preserve private image composition and independent source/filter/blend output. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import {PNG} from "pngjs";
import cases from "../../../🧫️fixtures/🖼️assets/🔣️.json";
import invalid from "../../../🧫️fixtures/🖼️assets/⚠️invalid/🔣️.json";
import schema from "../../../🧬️schema/🔣️.json";
import pathSchema from "../../../../../🧮️geometry/📷️raster/🧬️schema/🔣️.json";
import {RasterSceneJob,rasterizeScene,type RasterSceneInput} from "../../../🟦️.ts";
import {areaOracle,canvasOracle,delta} from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🎨️sampling/↗️affine/🧪️tests/🔭️oracles/🟦️.ts";
import imageSchema from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧬️schema/🔣️.json";
const ajv=new Ajv({strict:true});ajv.addSchema(pathSchema);ajv.addSchema(imageSchema);ajv.addSchema(schema);const validate=ajv.compile(schema),validateProgress=ajv.compile({$ref:schema.$id+"#/definitions/progress"});
const complete=(value:RasterSceneInput,grant=4096)=>{const job=new RasterSceneJob(value);let work=0;for(let i=0;i<2000000;i++){const p=job.advance(grant);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)return {image:job.result(),progress:p};}throw Error("Encoded scene did not finish");};
async function source(data:string){
 const comma=data.indexOf(","),bytes=data.slice(0,5).toLowerCase()==="data:"?Buffer.from(await(await fetch(data.slice(0,comma).toLowerCase()+data.slice(comma))).arrayBuffer()):Buffer.from(data,"base64");
 const image=PNG.sync.read(bytes,{skipRescale:bytes[24]===16});return {width:image.width,height:image.height,pixels:image.depth===16?Uint8Array.from(image.data as unknown as Uint16Array,v=>v>>>8):new Uint8Array(image.data)};
}
async function reference(value:RasterSceneInput){
 let body="",groups:string[]=[];
 for(const n of value.nodes){
  let common=0;while(common<groups.length&&common<n.groups.length&&groups[common]===n.groups[common]!.id)common++;
  while(groups.length>common){body+="</g>";groups.pop();}
  for(const g of n.groups.slice(common)){body+=`<g opacity="${g.opacity}" style="isolation:isolate;mix-blend-mode:${g.blendMode}">`;groups.push(g.id);}
  const c=n.content;
  if(c.kind==="path"){const f=c.fill;if(f?.kind!=="solid")throw Error("Expected solid oracle path");const d=c.segments.map(s=>s.kind==="close"?"Z":s.kind==="move"?`M${s.to}`:s.kind==="line"?`L${s.to}`:s.kind==="quad"?`Q${s.ctrl} ${s.to}`:s.kind==="cubic"?`C${s.ctrl1} ${s.ctrl2} ${s.to}`:`A${s.rx} ${s.ry} ${s.rotation} ${Number(s.largeArc)} ${Number(s.sweep)} ${s.to}`).join(" ");body+=`<path d="${d}" transform="matrix(${n.transform.join(" ")})" fill="rgb(${f.color.slice(0,3).map(v=>v*255).join(",")})" opacity="${n.opacity*f.color[3]}"/>`;continue;}
  if(c.kind!=="image"||!n.visible||n.opacity===0||n.groups.some(g=>g.opacity===0)||n.transform[0]*n.transform[3]-n.transform[1]*n.transform[2]===0)continue;
  const image=await source(value.assets.find(a=>a.id===c.asset)!.data),m=n.transform,sx=c.width/image.width,sy=c.height/image.height,transform=[m[0]*sx,m[1]*sx,m[2]*sy,m[3]*sy,m[4],m[5]];
  const det=transform[0]!*transform[3]!-transform[1]!*transform[2]!,u=transform[3]!**2+transform[1]!**2,w=transform[2]!**2+transform[0]!**2,cross=-transform[3]!*transform[2]!-transform[1]!*transform[0]!;
  const shrink=(u+w+Math.hypot(u-w,2*cross))/(2*det*det)>1+1e-12,plane={source:image,width:value.width,height:value.height,origin:value.origin,transform,sampling:"auto" as const};
  const pixels=image.width*image.height===1||shrink?areaOracle(plane):canvasOracle(plane),png=await sharp(pixels,{raw:{width:value.width,height:value.height,channels:4}}).png().toBuffer();
  body+=`<image width="${value.width}" height="${value.height}" opacity="${n.opacity}" style="mix-blend-mode:${n.blendMode}" href="data:image/png;base64,${png.toString("base64")}"/>`;
 }
 while(groups.length){body+="</g>";groups.pop();}
 return sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${value.width}" height="${value.height}">${body}</svg>`)).ensureAlpha().raw().toBuffer();
}
for(const row of cases)test(row.name,async()=>{
 expect(validate(row.input)).toBe(true);const value=row.input as RasterSceneInput,before=structuredClone(value);
 for(const grant of [1,7,4096]){const result=complete(value,grant);expect(Array.from(result.image.pixels)).toEqual(row.expected);expect(result.progress.decodes).toBe(row.expectedDecodes);expect(result.progress.sourceBytes).toBe(row.expectedSourceBytes);expect(result.progress.assets).toBe(value.assets.length);if("expectedAllocatedPixels" in row)expect(result.progress.pixels).toBe(row.expectedAllocatedPixels);}
 expect(value).toEqual(before);expect(delta(new Uint8Array(row.expected),await reference(value))).toBeLessThanOrEqual(2);
});
for(const row of invalid)test("refuse encoded scene "+row.name,()=>{
 let job:RasterSceneJob|undefined;expect(()=>{job=new RasterSceneJob(row.input as RasterSceneInput);while(!job.advance(4096).done){}job.result();}).toThrow();
 if(job){expect(()=>job!.result()).toThrow();expect(()=>job!.advance(1)).toThrow();}
});
test("asset admission counts UTF-8 source bytes and abort covers every live preparation phase",async()=>{
 const value=cases[5]!.input as RasterSceneInput,probe=new RasterSceneJob(value),seen=new Set<string>();let steps=0;
 for(;;){const p=probe.advance(1);steps++;if(!seen.has(p.phase)&&!p.done){seen.add(p.phase);const job=new RasterSceneJob(value);job.advance(steps);job.cancel();expect(()=>job.result()).toThrow(/cancel/);expect(()=>job.advance(1)).toThrow(/cancel/);}if(p.done)break;}
 expect(seen.has("assets")).toBe(true);expect(seen.has("source")).toBe(true);expect(seen.has("image")).toBe(true);expect(seen.has("compositing")).toBe(true);
 const unused={...cases[0]!.input,nodes:[],assets:[{id:"unicode",mime:"image/png",data:"é😀"}],maxSourceBytes:6} as RasterSceneInput;
 expect(complete(unused,1).progress.sourceBytes).toBe(6);expect(()=>complete({...unused,maxSourceBytes:5},1)).toThrow(/source/i);
 const controller=new AbortController();let timer=false;setTimeout(()=>{timer=true;},0);
 await expect(rasterizeScene(value,{signal:controller.signal,workBudget:7,onProgress:p=>{if(p.phase==="source")controller.abort();}})).rejects.toThrow(/cancel/);expect(timer).toBe(true);
 const late=new AbortController();await expect(rasterizeScene(value,{signal:late.signal,onProgress:p=>{if(p.done)late.abort();}})).rejects.toThrow(/cancel/);
 console.info("[DEBUG] Encoded scene admission/source/filter/composition phases cancelled without publication; actual UTF-8 budgets verified");
});

test("encoded image layers preserve every blend mode above a painted path",async()=>{
 const base=cases[cases.length-1]!.input as RasterSceneInput;
 const css:Record<string,string>={colorDodge:"color-dodge",colorBurn:"color-burn",hardLight:"hard-light",softLight:"soft-light"};
 for(const blendMode of ["normal","multiply","screen","overlay","darken","lighten","colorDodge","colorBurn","hardLight","softLight","difference","exclusion","hue","saturation","color","luminosity"] as const){
  const nodes=base.nodes.map((n,i)=>i?{...n,blendMode}:{...n,content:{kind:"path" as const,segments:n.content.kind==="path"?n.content.segments:[],fillRule:"nonzero" as const,fill:{kind:"solid" as const,color:[.25,.5,.75,1] as [number,number,number,number]},stroke:null}}),value={...base,nodes};
  const actual=complete(value,7).image.pixels,data=base.assets[0]!.data,svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"><rect width="1" height="1" fill="rgb(63.75,127.5,191.25)"/><image width="1" height="1" opacity=".5" style="mix-blend-mode:${css[blendMode]??blendMode}" href="data:image/png;base64,${data}"/></svg>`,expected=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
  expect(delta(actual,expected),blendMode).toBeLessThanOrEqual(2);
 }
 console.info("[DEBUG] Encoded assets above painted paths matched independent SVG for all sixteen blend modes");
});

test("scene forwards source decoding and partial PNG progress without publishing pixels",()=>{
 const row=cases[31]!,job=new RasterSceneJob(row.input as RasterSceneInput),phases=new Set<string>();let partial=false;
 for(;;){const p=job.advance(1);expect(validateProgress(p)).toBe(true);if(p.decoding){phases.add(p.decoding.phase);if(p.decoding.phase==="png"&&p.decoding.pixels>0&&p.decoding.pixels<p.decoding.totalPixels){partial=true;expect(()=>job.result()).toThrow();}}if(p.done)break;}
 expect([...phases]).toEqual(row.expectedDecodePhases);expect(partial).toBe(true);
 console.info("[DEBUG] Scene observers received header/validation/decode/PNG progress while incomplete image pixels stayed private");
});
