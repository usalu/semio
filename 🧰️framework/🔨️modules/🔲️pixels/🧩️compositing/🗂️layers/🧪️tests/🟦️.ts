/** 🗂️ Shared layer-stack vectors checked against independent SVG mask compositing. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {RasterStackJob,type RasterStackLayer} from "../🟦️.ts";
const images=()=>Object.fromEntries(Object.entries(fixture.images).map(([key,image])=>[key,{...image,pixels:Uint8Array.from(image.pixels)}]));
const validate=new Ajv({strict:false}).compile<{layers:RasterStackLayer[]}>(schema);
/** 🗂️ Admits one fixture layer list through the shared schema, so the job receives exactly the validated stack. */
const admitted=(layers:unknown):RasterStackLayer[]=>{const input={layers,images:fixture.images};if(!validate(input))throw new Error(JSON.stringify(validate.errors));return input.layers;};
for(const row of fixture.cases)test(row.name,async()=>{
  expect(validate({layers:row.layers,images:fixture.images})).toBe(true);
  const shapes=fixture.images.coverage.pixels.flatMap((_value,index,bytes)=>index%4?[]:[`<rect x="${index/4+row.svgMaskX}" width="1" height="1" fill="rgb(${Array(3).fill(row.invert?255-bytes[index]!*bytes[index+3]!/255:bytes[index]!*bytes[index+3]!/255).join(",")})"/>`]).join("");
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="3" height="1"><defs><mask id="m" maskUnits="userSpaceOnUse" x="0" y="0" width="3" height="1" color-interpolation="sRGB">${row.invert?'<rect width="3" height="1" fill="white"/>':""}${shapes}</mask></defs><g opacity="${row.opacity}" mask="url(#m)"><rect width="3" height="1" fill="red"/>${row.layers[0]!.kind==="group"?'<rect width="3" height="1" fill="red"/>':""}</g></svg>`;
  const reference=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
  for(const grant of [1,17,256]){
    const job=new RasterStackJob({layers:admitted(row.layers),images:images()});
    expect(()=>job.result()).toThrow();
    let done=false;
    for(let step=0;step<1000&&!done;step++)done=job.advance(grant).done;
    expect(done).toBe(true);
    const result=job.result();expect(result.origin).toEqual(row.origin);expect(result.image.width).toBe(3);expect(result.image.height).toBe(1);
    for(let pixel=0;pixel<3;pixel++){
      expect(result.image.pixels[pixel*4+3]).toBe(row.alpha[pixel]);
      expect(Math.abs(result.image.pixels[pixel*4+3]!-reference[pixel*4+3]!)).toBeLessThanOrEqual(1);
    }
  }
});
test("mask preparation is cancellable and never publishes a partial image",()=>{
  const job=new RasterStackJob({layers:admitted(fixture.cases[0]!.layers),images:images()});
  expect(job.advance(1).done).toBe(false);job.cancel();
  expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();
});
const layer=():RasterStackLayer=>structuredClone(fixture.cases[0]!.layers[0]) as RasterStackLayer;
const finish=(job:RasterStackJob)=>{for(let i=0;i<10000;i++)if(job.advance(256).done)return job.result();throw new Error("Stack job did not finish");};
test("grants include deduplicated mask preparation with monotonic fixed totals",()=>{
  const first=layer(),second=layer();second.id="second";
  const job=new RasterStackJob({layers:[first,second],images:images()});
  let progress=job.advance(1);expect(progress).toEqual({completed:1,total:12,done:false});
  for(let completed=2;completed<=12;completed++){
    progress=job.advance(1);expect(progress).toEqual({completed,total:12,done:completed===12});
  }
  expect(job.advance(1)).toEqual(progress);
});
test("layer metadata is owned while immutable image bytes remain borrowed",()=>{
  const source=layer(),assets=images();
  const job=new RasterStackJob({layers:[source],images:assets});
  source.transform.x=100;source.opacity=0;
  if(source.kind!=="pixel")throw new Error("Expected pixel fixture");
  source.mask!.invert=true;source.mask!.transform.x=100;assets.paint!.width=100;
  const result=finish(job);expect(result.origin).toEqual([0,0]);expect([...result.image.pixels].filter((_value,index)=>index%4===3)).toEqual([255,128,0]);
});
test("blank and empty documents publish transparent finite extents",()=>{
  const blank=layer();if(blank.kind!=="pixel")throw new Error("Expected pixel fixture");
  blank.imageKey=null;blank.mask=null;
  const result=finish(new RasterStackJob({layers:[blank],images:{}}));
  expect(result.empty).toBe(false);expect(result.image).toEqual({width:3,height:1,pixels:new Uint8Array(12)});
  const empty=finish(new RasterStackJob({layers:[],images:{}}));
  expect(empty).toEqual({empty:true,origin:[0,0],image:{width:1,height:1,pixels:new Uint8Array(4)}});
});
test("constant inverted and disabled masks preserve their distinct meaning",()=>{
  const source=layer();if(source.kind!=="pixel")throw new Error("Expected pixel fixture");
  source.mask!.imageKey=null;source.mask!.invert=true;
  expect(finish(new RasterStackJob({layers:[source],images:images()})).image.pixels).toEqual(new Uint8Array(12));
  source.mask!.enabled=false;
  expect(finish(new RasterStackJob({layers:[source],images:images()})).image.pixels).toEqual(images().paint!.pixels);
});
test("invalid identifiers transforms assets and grants are rejected",()=>{
  expect(()=>new RasterStackJob({layers:[layer(),layer()],images:images()})).toThrow();
  const singular=layer();singular.transform.a=0;
  expect(()=>new RasterStackJob({layers:[singular],images:images()})).toThrow();
  expect(()=>new RasterStackJob({layers:[layer()],images:{}})).toThrow();
  const job=new RasterStackJob({layers:[layer()],images:images()});
  for(const grant of [0,-1,1.5,NaN,1048577])expect(()=>job.advance(grant)).toThrow();
  expect(finish(job).image.width).toBe(3);
});
test("blank pixels remain valid at the maximum group depth",()=>{
  let source=layer();if(source.kind!=="pixel")throw new Error("Expected pixel fixture");source.imageKey=null;source.mask=null;
  for(let depth=0;depth<32;depth++)source={kind:"group",id:`group-${depth}`,visible:true,opacity:1,blendMode:"normal",transform:{x:0.0,y:0.0,a:1.0,b:0.0,c:-0.0,d:1.0},children:[source]};
  const result=finish(new RasterStackJob({layers:[source],images:{}}));expect(result.image.pixels).toEqual(new Uint8Array(12));
});
