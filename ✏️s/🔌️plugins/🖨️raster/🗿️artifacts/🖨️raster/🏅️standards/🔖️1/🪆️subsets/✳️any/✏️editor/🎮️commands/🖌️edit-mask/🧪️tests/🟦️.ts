/** 🖌️ Mask command coverage agrees with the independent libvips oracle. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {editImage,type PixelOperation} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/✍️editing/🟦️.ts";
import {maskCoverage} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🗂️layers/🟦️.ts";
test("mask paint command fixture preserves coverage",async()=>{
  const command={layerId:"paint",expectedMask:JSON.stringify({enabled:true,linked:true,invert:false,imageKey:"coverage",width:3,height:1,transform:{x:0.0,y:0.0,a:1.0,b:0.0,c:-0.0,d:1.0}}),operation:JSON.stringify(fixture.operation),selection:JSON.stringify(fixture.selection)};
  expect(new Ajv({strict:false}).compile(schema)(command)).toBe(true);
  const selection=new Uint8Array(fixture.width*fixture.height);
  for(const [start,length,coverage] of fixture.selection)selection.fill(coverage!,start!,start!+length!);
  const result=await editImage({width:fixture.width,height:fixture.height,pixels:Uint8Array.from(fixture.beforeRgba)},fixture.operation as PixelOperation,{selection});
  expect([...result.pixels]).toEqual(fixture.expectedRgba);
  for(let i=0;i<selection.length;i++){
    const amount=selection[i]!/255*fixture.operation.opacity,alpha=fixture.beforeRgba[i*4+3]!;
    const oracle=await sharp(Uint8Array.of(alpha,alpha,alpha),{raw:{width:1,height:1,channels:3}}).linear(1-amount,fixture.operation.alpha*amount+0.5).extractChannel(0).raw().toBuffer();
    expect(result.pixels[i*4+3]).toBe(oracle[0]);
  }
});

for(const f of fixture.coverageCases)test(f.name,async()=>{
  const pixels=Uint8Array.from(f.beforeRgba);
  const selection=new Uint8Array(f.width*f.height);
  for(const [start,length,value] of f.selection)selection.fill(value!,start!,start!+length!);
  for(let i=0;i<f.coverage.length;i++){
    const offset=i*4,[r,g,b,a]=f.beforeRgba.slice(offset,offset+4);
    const coverage=maskCoverage(r!,g!,b!,a!);
    expect(coverage).toBe(f.coverage[i]);
    const weights=[0.2126,0.7152,0.0722].map(value=>value*a!/255);
    const oracle=await sharp(Uint8Array.of(r!,g!,b!),{raw:{width:1,height:1,channels:3}}).recomb([weights,weights,weights] as [[number,number,number],[number,number,number],[number,number,number]]).linear(1,0.5).raw().toBuffer();
    expect(coverage).toBe(oracle[0]);
    pixels.set([255,255,255,coverage],offset);
  }
  const result=await editImage({width:f.width,height:f.height,pixels},f.operation as PixelOperation,{selection});
  expect([...result.pixels]).toEqual(f.expectedRgba);
});
