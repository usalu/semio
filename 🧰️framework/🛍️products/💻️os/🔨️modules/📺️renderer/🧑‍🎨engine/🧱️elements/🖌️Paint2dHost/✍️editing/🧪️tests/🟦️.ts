/** 🧪️ Pixel tools respect nested layer transforms and lossless selection transport. */
import { expect, test } from "bun:test";
import sharp from "sharp";
import { editSelection, maskLayers, pixelLayers, layerPoint, selectionSpans, selectionBounds, type SelectionScanOptions } from "../🟦️.ts";
import { surfacePaintEditingFixture as fixtures, paintStrokeRevisionFixture as revisions, paintMaskStrokeFixture as maskStrokes } from "@semio-tech/framework-surface-ts/testing/fixtures";
import sourceCases from "../📥️source/🧪️tests/🔣️.json";
import {preparePixelSourceBytes, prepareBlankPixelSource, pixelReadbackView} from "../📥️source/🟦️.ts";
import base64 from "base64-js";
import editingFixture from "../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎚️paint2d-editing/🔣️.json";
import editingSchema from "../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🖼️plane/🎨️paint2d-scene/🎚️editing/🧬️schema/🔣️.json";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import type {Paint2dEditingOptions} from "@semio-tech/framework";

test("editing options agree with the neutral scene schema",()=>{
  const validate=semioSchemaAjvV1({allErrors:true}).compile<Paint2dEditingOptions>(editingSchema);
  expect(validate(editingFixture.initial)).toBe(true);expect(validate(editingFixture.options)).toBe(true);expect(validate(editingFixture.edge)).toBe(true);
  for(const invalid of editingFixture.invalid)expect(validate(invalid)).toBe(false);
});

for(const row of sourceCases.cases)test(`pixel source ${row.name}`,async()=>{
  const progress:number[]=[];
  const bytes=await preparePixelSourceBytes(row.encoded,{onProgress:p=>progress.push(p)});
  expect([...bytes]).toEqual(row.bytes);
  expect([...bytes]).toEqual([...base64.toByteArray(row.encoded)]);
  expect(progress.at(-1)).toBe(1);
  expect(progress.every((value,index)=>value>=0&&value<=1&&(index===0||value>=progress[index-1]!))).toBe(true);
});
for(const encoded of sourceCases.invalid)test(`pixel source refuses ${encoded}`,async()=>{
  await expect(preparePixelSourceBytes(encoded)).rejects.toThrow();
});
for(const row of sourceCases.admission)test(`pixel source respects caller byte admission ${row.maximumOutputBytes}`,async()=>{
  const result=preparePixelSourceBytes("Zg==",{maximumOutputBytes:row.maximumOutputBytes});
  if(row.valid)expect([...await result]).toEqual([...base64.toByteArray("Zg==")]);else await expect(result).rejects.toThrow();
});
test("pixel source preparation yields while retaining exact bytes",async()=>{
  const row=sourceCases.large,encoded=row.pattern.repeat(row.repeat),progress:number[]=[];
  let yielded=false;setTimeout(()=>{yielded=true;},0);
  const bytes=await preparePixelSourceBytes(encoded,{onProgress:p=>progress.push(p)});
  expect(bytes.length).toBe(row.decodedBytes);expect(yielded).toBe(true);
  expect(bytes).toEqual(base64.toByteArray(encoded));
  expect(progress.some(p=>p>0&&p<0.5)).toBe(true);
  expect(progress.some(p=>p>0.5&&p<1)).toBe(true);
  expect(progress.every((value,index)=>index===0||value>=progress[index-1]!)).toBe(true);
});
for(const at of sourceCases.cancelAt)test(`pixel source cancellation ${at} refuses output`,async()=>{
  const controller=new AbortController();if(at==="before")controller.abort();
  await expect(preparePixelSourceBytes(sourceCases.large.pattern.repeat(sourceCases.large.repeat),{
    signal:controller.signal,onProgress:p=>{if(at==="validation"&&p>0&&p<0.5||at==="decoding"&&p>0.5&&p<1||at==="complete"&&p===1)controller.abort();}
  })).rejects.toMatchObject({name:"AbortError"});
});
test("pixel readback preserves the owned browser buffer and exact view extent",()=>{
  const buffer=new ArrayBuffer(20),source=new Uint8ClampedArray(buffer,4,8);source.set([0,255,128,255,3,2,1,0]);
  const view=pixelReadbackView(source);
  expect(view.buffer).toBe(buffer);expect(view.byteOffset).toBe(4);expect(view.byteLength).toBe(8);expect([...view]).toEqual([...source]);
});
for(const row of sourceCases.blank)test(`blank ${row.target} source matches independent image creation`,async()=>{
  const image=await prepareBlankPixelSource(row.width,row.height,row.target as "pixels"|"mask");
  const oracle=await sharp({create:{width:row.width,height:row.height,channels:4,background:{r:row.value,g:row.value,b:row.value,alpha:row.value/255}}}).raw().toBuffer();
  expect(image.width).toBe(row.width);expect(image.height).toBe(row.height);expect([...image.pixels]).toEqual([...oracle]);
});
test("blank mask source initializes in cancellable grants",async()=>{
  const row=sourceCases.blankBoundary,progress:number[]=[];let yielded=false;setTimeout(()=>{yielded=true;},0);
  const image=await prepareBlankPixelSource(row.width,row.height,"mask",{onProgress:p=>progress.push(Math.round(p*row.bytes))});
  expect(progress).toEqual(row.progress);expect(yielded).toBe(true);expect(image.pixels.every(value=>value===255)).toBe(true);
  for(const cancelAt of [0,32768,row.bytes]){
    const controller=new AbortController();
    await expect(prepareBlankPixelSource(row.width,row.height,"mask",{signal:controller.signal,onProgress:p=>{if(Math.round(p*row.bytes)===cancelAt)controller.abort();}})).rejects.toMatchObject({name:"AbortError"});
  }
});
test("blank source rejects invalid targets and extents",async()=>{
  await expect(prepareBlankPixelSource(0,2,"mask")).rejects.toThrow();
  await expect(prepareBlankPixelSource(16384,16384,"mask")).rejects.toThrow();
  await expect(prepareBlankPixelSource(1,1,"other" as "mask")).rejects.toThrow();
});

for(const fixture of fixtures.cases) test(fixture.name,()=>{
  const [layer]=pixelLayers(JSON.stringify(fixture.document),JSON.stringify(fixture.assets));
  const point=layerPoint(layer!,fixture.worldPoint[0]!,fixture.worldPoint[1]!);
  for(let i=0;i<2;i++)expect(point[i]!).toBeCloseTo(fixture.pixelPoint[i]!,10);
});

test("nested transforms map painting into the selected layer", () => {
  const layers=pixelLayers(JSON.stringify({layers:[{kind:"group",id:"g",visible:true,transform:{x:10.0,y:20.0,a:2.0,b:0.0,c:-0.0,d:2.0},children:[{kind:"pixel",id:"p",name:"Pixels",width:20,height:10,transform:{x:3,y:4,a:1,b:0,c:0,d:1}}]}]}));
  expect(layers).toHaveLength(1);
  expect(layerPoint(layers[0]!,18,30)).toEqual([11,6]);
});

test("the compositor centers pixel layers on their transforms",()=>{
  const [layer]=pixelLayers(JSON.stringify({layers:[{kind:"pixel",id:"p",width:512,height:256,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]}));
  expect(layerPoint(layer!,0,0)).toEqual([256,128]);
  expect(layerPoint(layer!,-256,-128)).toEqual([0,0]);
});

test("hidden ancestors prevent painting", () => {
  const layers=pixelLayers(JSON.stringify({layers:[{kind:"group",visible:false,children:[{kind:"pixel",id:"p"}]}]}));
  expect(layers[0]!.visible).toBe(false);
});

test("selection spans retain empty versus unrestricted selection", async () => {
  expect(await selectionSpans(undefined)).toBe(null);
  expect(await selectionSpans(new Uint8Array(4))).toEqual([]);
  expect(await selectionSpans(Uint8Array.of(0,255,255,0,128))).toEqual([{start:1,length:2,coverage:255},{start:4,length:1,coverage:128}]);
});

test("selection bounds use actual mask coverage", async () => {
  expect(await selectionBounds(Uint8Array.of(0,255,0,0,255,0),3)).toEqual({x:1,y:0,width:1,height:2});
  expect(await selectionBounds(new Uint8Array(6),3)).toBe(null);
});

test("singular transforms reject interaction instead of inventing a location", () => {
  const layers=pixelLayers(JSON.stringify({layers:[{kind:"pixel",id:"p",transform:{x:0,y:0,a:0.0,b:0.0,c:-0.0,d:1.0}}]}));
  expect(()=>layerPoint(layers[0]!,0,0)).toThrow();
});

for(const fixture of fixtures.cases) test(`${fixture.name}: SVG oracle maps the fixture pixel to its world coordinate`,async()=>{
  const layer=fixture.document.layers[0]!;
  const transform=layer.transform as {x:number;y:number;a:number;b:number;c:number;d:number};
  const asset="imageKey" in layer?fixture.assets[layer.imageKey as keyof typeof fixture.assets]:undefined;
  const sx=asset?layer.width/asset.width:1,sy=asset?layer.height/asset.height:1;
  const {x,y,zoom}=fixture.camera;
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"><g transform="translate(${50-x*zoom} ${50-y*zoom}) scale(${zoom}) matrix(${transform.a} ${transform.b} ${transform.c} ${transform.d} ${transform.x} ${transform.y}) translate(${-layer.width/2} ${-layer.height/2}) scale(${sx} ${sy})"><rect x="${fixture.pixelPoint[0]!-0.5}" y="${fixture.pixelPoint[1]!-0.5}" width="1" height="1" fill="red"/></g></svg>`;
  const {data}=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer({resolveWithObject:true});
  const px=Math.floor(50+(fixture.worldPoint[0]!-x)*zoom),py=Math.floor(50+(fixture.worldPoint[1]!-y)*zoom),at=(py*100+px)*4;
  expect(data[at]).toBe(255);expect(data[at+3]).toBeGreaterThan(200);
});

for(const fixture of fixtures.selectionTransport.cases) test(fixture.name+": bounded selection transport",async()=>{
  const mask=Uint8Array.from(fixture.mask);
  const spans=(await selectionSpans(mask))!;
  expect(spans).toEqual(fixture.spans);
  expect(await selectionBounds(mask,fixture.width)).toEqual(fixture.bounds);
  const restored=new Uint8Array(mask.length);
  for(const {start,length:count,coverage:value} of spans) restored.fill(value,start,start+count);
  const png=await sharp(restored,{raw:{width:fixture.width,height:mask.length/fixture.width,channels:1}}).png().toBuffer();
  const decoded=await sharp(png).greyscale().raw().toBuffer();
  expect([...decoded]).toEqual(fixture.mask);
});

test("selection transport yields with exact progress across a run boundary",async()=>{
  const f=fixtures.selectionTransport.boundary,mask=new Uint8Array(f.length);mask.fill(f.value,f.start,f.start+f.count);
  const progress:number[]=[];let yielded=false;setTimeout(()=>{yielded=true;},0);
  const result=await selectionSpans(mask,{onProgress:p=>progress.push(p.completed)});
  expect(result).toEqual([{start:f.start,length:f.count,coverage:f.value}]);
  expect(progress).toEqual([32768,65536,f.length]);expect(yielded).toBe(true);
});

test("selection transport and bounds honor cancellation before publishing",async()=>{
  const mask=new Uint8Array(65539).fill(255);
  for(const operation of [selectionSpans,(mask:Uint8Array,options:SelectionScanOptions)=>selectionBounds(mask,1,options)]){
    const controller=new AbortController();let progress=0;
    await expect(operation(mask,{signal:controller.signal,onProgress:(p:{completed:number})=>{progress=p.completed;controller.abort();}})).rejects.toMatchObject({name:"AbortError"});
    expect(progress).toBe(fixtures.selectionTransport.cancellationAfter);
    await expect(operation(mask,{signal:controller.signal})).rejects.toMatchObject({name:"AbortError"});
  }
});

test("fragmented typed coverage survives without an encoded text budget",async()=>{
  const mask=Uint8Array.from({length:131072},(_,index)=>index%2?255:0),progress:number[]=[];
  const spans=await selectionSpans(mask,{onProgress:p=>progress.push(p.completed)});
  expect(spans?.length).toBe(65536);
  expect(progress).toEqual([32768,65536,98304,131072]);
});

test("selection transport cancellation at the final grant cannot publish",async()=>{
  for(const operation of [selectionSpans,(mask:Uint8Array,options:SelectionScanOptions)=>selectionBounds(mask,2,options)]) {
    const controller=new AbortController();
    await expect(operation(Uint8Array.of(0,255),{signal:controller.signal,onProgress:p=>{if(p.done)controller.abort();}})).rejects.toMatchObject({name:"AbortError"});
  }
});

for(const fixture of fixtures.maskCases)test(fixture.name,async()=>{
  const [layer]=maskLayers(JSON.stringify(fixture.document),JSON.stringify(fixture.assets));
  expect(layer?.imageKey).toBe("m");expect(layer?.visible).toBe(true);
  const point=layerPoint(layer!,fixture.worldPoint[0]!,fixture.worldPoint[1]!);
  for(let i=0;i<2;i++)expect(point[i]!).toBeCloseTo(fixture.pixelPoint[i]!,10);
  const svg=`<svg width="100" height="100"><g transform="${fixture.svgTransform}"><rect x="1" y="0" width="1" height="1" fill="red"/></g></svg>`;
  const data=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
  expect(data[(fixture.worldPoint[1]!*100+fixture.worldPoint[0]!)*4+3]).toBe(255);
});

for(const fixture of fixtures.selectionEditing.cases)test(fixture.name,async()=>{
  const source=fixture.mask?Uint8Array.from(fixture.mask):undefined;
  const result=await editSelection(fixture.expected.length,source,fixture.mode as "all"|"invert");
  expect([...result]).toEqual(fixture.expected);
  const oracle=sharp(Buffer.from(source??new Uint8Array(result.length)),{raw:{width:result.length,height:1,channels:1}});
  const expected=await (fixture.mode==="invert"?oracle.negate():oracle.linear(0,255)).greyscale().raw().toBuffer();
  expect([...result]).toEqual([...expected]);
  if(source)expect([...source]).toEqual(fixture.mask!);
});
test("selection editing yields and cancels without publishing partial coverage",async()=>{
  const fixture=fixtures.selectionEditing;
  for(const mode of ["all","invert"] as const){
    const progress:number[]=[];let yielded=false;setTimeout(()=>{yielded=true;},0);
    const result=await editSelection(fixture.length,undefined,mode,{onProgress:p=>progress.push(p.completed)});
    expect(progress).toEqual(fixture.progress);expect(yielded).toBe(true);expect(result.every(v=>v===255)).toBe(true);
    for(const cancelAt of [32768,fixture.length]){
      const controller=new AbortController();
      await expect(editSelection(fixture.length,undefined,mode,{signal:controller.signal,onProgress:p=>{if(p.completed===cancelAt)controller.abort();}})).rejects.toMatchObject({name:"AbortError"});
    }
  }
});
test("selection editing rejects invalid extents before allocating",async()=>{
  for(const length of [-1,0,1.5,16777217,NaN])await expect(editSelection(length,undefined,"all")).rejects.toThrow();
  await expect(editSelection(3,new Uint8Array(2),"invert")).rejects.toThrow();
  const controller=new AbortController();controller.abort();
  await expect(editSelection(3,undefined,"all",{signal:controller.signal})).rejects.toMatchObject({name:"AbortError"});
});

import patch,{type Operation} from "fast-json-patch";
import {pixelGestureRevision} from "../🟦️.ts";
for(const row of revisions.cases)test("stroke revision "+row.name,()=>{
  const before=pixelLayers(JSON.stringify(revisions.document))[0];
  const updated=patch.applyPatch(revisions.document,row.patch as Operation[],true,false).newDocument;
  const after=pixelLayers(JSON.stringify(updated)).find(layer=>layer.id===row.selected[0]);
  expect(pixelGestureRevision(before)!==pixelGestureRevision(after)||row.utility!=="paintBrush").toBe(row.cancel);
});

import {Matrix3,Vector2} from "three";
for(const row of maskStrokes.cases)test("shared mask stroke "+row.name,async()=>{
  const [layer]=maskLayers(JSON.stringify(row.document),JSON.stringify(row.assets));
  expect(layer?.target).toBe("mask");expect(layer?.locked).toBe(false);
  const point=layerPoint(layer!,row.worldPoint[0]!,row.worldPoint[1]!);
  const owner=row.document.layers[0]!.children[0]!,mask=owner.mask,parent=row.document.layers[0]!.transform;
  const affine=(t:{a:number;b:number;c:number;d:number;x:number;y:number})=>new Matrix3().set(t.a,t.c,t.x,t.b,t.d,t.y,0,0,1);
  const matrix=affine(parent);if(mask.linked)matrix.multiply(affine(owner.transform));matrix.multiply(affine(mask.transform));
  const extent=mask.imageKey?row.assets.m:undefined,width=extent?.width??mask.width??("width" in owner?owner.width:undefined)??512,height=extent?.height??mask.height??("height" in owner?owner.height:undefined)??512;
  matrix.multiply(new Matrix3().set((mask.width??width)/width,0,-(mask.width??width)/2,0,(mask.height??height)/height,-(mask.height??height)/2,0,0,1));
  const oracle=new Vector2(row.worldPoint[0],row.worldPoint[1]).applyMatrix3(matrix.invert());
  for(let i=0;i<2;i++){expect(point[i]!).toBeCloseTo(row.pixelPoint[i]!,10);expect(point[i]!).toBeCloseTo(oracle.toArray()[i]!,10);}
  expect(JSON.parse(layer!.maskRevision!)).toEqual(mask);
});
for(const row of maskStrokes.revisions)test("shared mask revision "+row.name,()=>{
  const base=maskStrokes.cases[0]!;
  const [before]=maskLayers(JSON.stringify(base.document),JSON.stringify(base.assets));
  const updated=patch.applyPatch(base.document,row.patch as Operation[],true,false).newDocument;
  const [after]=maskLayers(JSON.stringify(updated),JSON.stringify(base.assets));
  expect(pixelGestureRevision(before)!==pixelGestureRevision(after)).toBe(row.cancel);
  expect(pixelGestureRevision(after)!==null).toBe(row.admit);
});
