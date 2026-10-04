/** 🖼️ Filtered image scenes agree with neutral pixels and independent filtering references. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import cases from "../../../🧫️fixtures/🖼️images/🔣️.json";
import schema from "../../../🧬️schema/🔣️.json";
import pathSchema from "../../../../../🧮️geometry/📷️raster/🧬️schema/🔣️.json";
import {RasterSceneJob,rasterizeScene,type RasterSceneInput} from "../../../🟦️.ts";
import {areaOracle,canvasOracle,delta} from "../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🎨️sampling/↗️affine/🧪️tests/🔭️oracles/🟦️.ts";
const ajv=new Ajv({strict:true});ajv.addSchema(pathSchema);const validate=ajv.compile(schema);
function input(v:any):RasterSceneInput{return {...v,nodes:v.nodes.map((n:any)=>({...n,content:{kind:"pixels",image:{...n.content.image,pixels:new Uint8Array(n.content.image.pixels)}}}))};}
function complete(v:RasterSceneInput,grant:number){const job=new RasterSceneJob(v);let work=0;for(let step=0;step<2000000;step++){const p=job.advance(grant);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)return {image:job.result(),pixels:p.pixels};}throw Error("Scene did not complete");}
for(const row of cases)test(row.name,()=>{
 expect(validate(row.input)).toBe(true);const v=input(row.input),before=structuredClone(v);for(const grant of [1,7,4096])expect([...complete(v,grant).image.pixels]).toEqual(row.expected);expect(v).toEqual(before);
 const n=v.nodes[0]!,source=n.content.kind==="pixels"?n.content.image:null;expect(source).not.toBeNull();const sample={source:source!,width:v.width,height:v.height,origin:v.origin,transform:n.transform,sampling:"auto" as const};
 const m=n.transform,det=m[0]*m[3]-m[1]*m[2],u=m[3]*m[3]+m[1]*m[1],w=m[2]*m[2]+m[0]*m[0],cross=-m[3]*m[2]-m[1]*m[0],shrink=det!==0&&(u+w+Math.hypot(u-w,2*cross))/(2*det*det)>1+1e-12;
 const reference=source!.width*source!.height===1||shrink?areaOracle(sample):canvasOracle(sample);
 if(v.nodes.length!==1||n.opacity!==1||n.groups.length)for(let at=0;at<reference.length;at+=4){const a=n.groups.length?Math.round((1-(1-reference[at+3]!/255)**v.nodes.length)*n.groups[0]!.opacity*255):Math.round(reference[at+3]!*n.opacity);if(a===0)reference.fill(0,at,at+4);else reference[at+3]=a;}
 expect(delta(new Uint8Array(row.expected),reference)).toBeLessThanOrEqual(2);
});
test("filtered scenes admit cropped output plus source and preserve grid transforms",()=>{
 const v=input(cases[0]!.input);expect(complete({...v,maxPixels:1},7).pixels).toBe(1);
 const shifted={...v,width:2,nodes:[{...v.nodes[0]!,transform:[1,0,0,1,.5,0] as const}]};expect(()=>complete({...shifted,maxPixels:2},7)).toThrow(/pixel/i);expect(complete({...shifted,maxPixels:3},7).pixels).toBe(3);
 expect(complete({...shifted,nodes:[{...shifted.nodes[0]!,transform:[1,0,0,1,1e8,0]}],maxPixels:1},7).pixels).toBe(0);
});
test("filtered scenes expose image progress and cancel private reduction",async()=>{
 const v=input(cases[0]!.input);const controller=new AbortController();let seen=false;
 const source={width:64,height:64,pixels:new Uint8Array(64*64*4).fill(255)},nodes=[{...v.nodes[0]!,transform:[1/64,0,0,1/64,0,0] as const,content:{kind:"pixels" as const,image:source}}];
 await expect(rasterizeScene({...v,maxPixels:4097,nodes},{workBudget:64,signal:controller.signal,onProgress:p=>{if(p.phase==="image"){seen=true;controller.abort();}}})).rejects.toThrow(/cancel/);expect(seen).toBe(true);
 console.error("[DEBUG] Filtered scene reductions exposed cancellable image work");
});
