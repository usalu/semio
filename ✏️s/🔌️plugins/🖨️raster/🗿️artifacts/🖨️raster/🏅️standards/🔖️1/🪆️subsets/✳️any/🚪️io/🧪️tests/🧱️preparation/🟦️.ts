/** 🧱️ Shared image preparation preserves the complete composite against libvips. */
import {expect,test} from "bun:test";
import sharp from "sharp";
import fixture from "../../🧫️fixtures/🧱️preparation/🔣️.json";
import {RasterStackJob,type RasterStackLayer} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🗂️layers/🟦️.ts";
test("shared source image preparation preserves the composite",async()=>{
  const {width,height,pixel,references,maximumPixelsPerGrant}=fixture;
  const pixels=Uint8Array.from({length:width*height*4},(_,i)=>pixel[i%4]!);
  const transform={x:0.0,y:0.0,a:1.0,b:0.0,c:-0.0,d:1.0};
  const layers:RasterStackLayer[]=Array.from({length:references},(_,i)=>({kind:"pixel",id:String(i),visible:true,opacity:1,blendMode:"normal",width,height,imageKey:"source",transform,mask:null}));
  const job=new RasterStackJob({layers,images:{source:{width,height,pixels}}});
  while(!job.advance(maximumPixelsPerGrant).done){}
  const result=job.result();
  expect(result.origin).toEqual([-width/2,-height/2]);
  expect(result.image.pixels).toEqual(pixels);
  const oracle=await sharp(pixels,{raw:{width,height,channels:4}}).composite(Array.from({length:references-1},()=>({input:Buffer.from(pixels),raw:{width,height,channels:4 as const},blend:"over" as const}))).raw().toBuffer();
  expect(result.image.pixels).toEqual(Uint8Array.from(oracle));
});
