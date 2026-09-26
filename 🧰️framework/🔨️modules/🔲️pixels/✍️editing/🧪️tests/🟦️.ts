/** 🧪️ Language-neutral pixel editing cases and independent libvips image oracles. */
import { describe, expect, test } from "bun:test";
import sharp from "sharp";
import Ajv from "ajv";
import fixtures from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import { editImage, selectionMask, selectPixels, combineSelections, floodSelection, paintStroke, PixelEditJob, validateImage, type PixelOperation, type SelectionShape } from "../🟦️.ts";

const image = () => ({ ...fixtures.image, pixels: Uint8Array.from(fixtures.image.pixels) });
describe("pixel editing contract", () => {
  const validate = new Ajv().compile(schema);
  for (const fixture of fixtures.cases) test(fixture.name, async () => {
    const source="image" in fixture?fixture.image!:fixtures.image;
    expect(validate({image: source, operation: fixture.operation, ...("selection" in fixture ? {selection: fixture.selection} : {})})).toBe(true);
    const input = {...source,pixels:Uint8Array.from(source.pixels)};
    const result = await editImage(input, fixture.operation as PixelOperation, { selection: "selection" in fixture ? Uint8Array.from(fixture.selection!) : undefined });
    expect([...result.pixels]).toEqual(fixture.expected);
    expect([...input.pixels]).toEqual(source.pixels);
    if("width" in fixture) expect([result.width,result.height]).toEqual([fixture.width,fixture.height]);
  });
  for (const fixture of fixtures.selections) test(fixture.name, () => {
    expect([...selectionMask("width" in fixture ? fixture.width! : 2, "height" in fixture ? fixture.height! : 2, fixture.shape as SelectionShape)]).toEqual(fixture.expected);
  });
  test("libvips independently verifies flips, rotations, crops and inversion", async () => {
    for (const [kind, apply] of [
      ["flipHorizontal", (s: sharp.Sharp) => s.flop()],
      ["flipVertical", (s: sharp.Sharp) => s.flip()],
      ["rotateClockwise", (s: sharp.Sharp) => s.rotate(90)],
      ["rotateCounterclockwise", (s: sharp.Sharp) => s.rotate(270)],
      ["invert", (s: sharp.Sharp) => s.negate({alpha: false})],
    ] as const) {
      const source = image();
      const oracle = await apply(sharp(source.pixels, {raw:{width:2,height:2,channels:4}})).raw().toBuffer();
      expect([...((await editImage(source, {kind})).pixels)]).toEqual([...oracle]);
    }
    const oracle = await sharp(image().pixels, {raw:{width:2,height:2,channels:4}}).extract({left:1,top:0,width:1,height:1}).raw().toBuffer();
    expect([...((await editImage(image(), {kind:"crop",x:1,y:0,width:1,height:1})).pixels)]).toEqual([...oracle]);
  });
  test("selection merge operations preserve coverage", () => {
    const a = Uint8Array.of(255,0,128), b = Uint8Array.of(0,255,255);
    expect([...combineSelections(a,b,"add")]).toEqual([255,255,255]);
    expect([...combineSelections(a,b,"subtract")]).toEqual([255,0,0]);
    expect([...combineSelections(a,b,"intersect")]).toEqual([0,0,128]);
    expect([...combineSelections(a,b,"replace")]).toEqual([...b]);
  });
  test("libvips independently verifies box convolution",async()=>{
    const source={width:3,height:1,pixels:Uint8Array.of(0,0,0,255,90,90,90,255,180,180,180,255)};
    const oracle=await sharp(source.pixels,{raw:{width:3,height:1,channels:4}}).convolve({width:3,height:3,kernel:Array(9).fill(1),scale:9}).raw().toBuffer();
    expect([...((await editImage(source,{kind:"blur",value:1})).pixels)]).toEqual([...oracle]);
  });
  test("asynchronous selections agree with fixtures and cancel before publication", async()=>{
    for(const fixture of fixtures.selections) {
      expect([...await selectPixels("width" in fixture?fixture.width!:2,"height" in fixture?fixture.height!:2,fixture.shape as SelectionShape)]).toEqual(fixture.expected);
    }
    const controller=new AbortController();
    await expect(selectPixels(1024,1024,{kind:"rectangle",x:0,y:0,width:1024,height:1024},{signal:controller.signal,onProgress:()=>controller.abort()})).rejects.toThrow();
  });
  test("selection rasterization agrees with an independent SVG renderer", async()=>{
    const source=Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40"><path fill="white" d="M0 0H20V40H0Z"/></svg>');
    const oracle=await sharp(source).ensureAlpha().raw().toBuffer();
    const expected=[0,1,2,3].map(index=>oracle[((Math.floor(index/2)*20+10)*40+(index%2*20+10))*4+3]);
    expect([...await selectPixels(2,2,{kind:"polygon",points:[[0,0],[1,0],[1,2],[0,2]]})]).toEqual(expected);
  });
  test("magic wand is contiguous and includes alpha in distance", async () => {
    const input = {width:3,height:1,pixels:Uint8Array.of(255,0,0,255,0,0,0,255,255,0,0,255)};
    expect([...await floodSelection(input,0,0,0)]).toEqual([255,0,0]);
    expect([...await floodSelection(input,0,0,255)]).toEqual([255,255,255]);
  });
  test("jobs yield, cancel atomically and never expose partial output", () => {
    const job = new PixelEditJob(image(),{kind:"invert"});
    expect(job.advance(1)).toEqual({completed:1,total:4,done:false});
    expect(() => job.result()).toThrow();
    job.cancel();
    expect(() => job.advance(1)).toThrow();
    expect(() => job.result()).toThrow();
  });
  test("AbortSignal prevents publication", async () => {
    const controller = new AbortController();
    await expect(editImage(image(),{kind:"invert"},{signal:controller.signal,chunkPixels:1,onProgress:() => controller.abort()})).rejects.toThrow();
  });
  test("invalid dimensions, masks, coordinates and operation arguments are rejected", async () => {
    expect(() => validateImage({width:2,height:2,pixels:new Uint8Array(3)})).toThrow();
    expect(() => validateImage({width:Infinity,height:2,pixels:new Uint8Array()})).toThrow();
    for (const operation of [{kind:"gamma",value:0},{kind:"blur",value:-1},{kind:"crop",x:1,y:1,width:2,height:2},{kind:"resize",width:0,height:2,sampling:"nearest"}]) {
      await expect(editImage(image(),operation as PixelOperation)).rejects.toThrow();
    }
    await expect(editImage(image(),{kind:"invert"},{selection:Uint8Array.of(255)})).rejects.toThrow();
    await expect(floodSelection(image(),-1,0,0)).rejects.toThrow();
  });
  test("bilinear resize does not leak hidden RGB into visible pixels", async () => {
    const input = {width:2,height:1,pixels:Uint8Array.of(255,0,0,255,0,0,255,0)};
    const result = await editImage(input,{kind:"resize",width:3,height:1,sampling:"bilinear"});
    expect([...result.pixels.slice(4,8)]).toEqual([255,0,0,128]);
  });
  test("painting uses source-over, interpolation, selection and erasure", async () => {
    const input = {width:5,height:1,pixels:new Uint8Array(20)};
    const painted = await paintStroke(input,[[0.5,0.5],[4.5,0.5]],{size:1,opacity:0.5,color:[255,0,0,255],hardness:1});
    expect([...painted.pixels]).toEqual(Array.from({length:5},()=>[255,0,0,128]).flat());
    const erased = await paintStroke(painted,[[0.5,0.5],[4.5,0.5]],{size:1,opacity:1,color:[0,0,0,255],hardness:1,erase:true},{selection:Uint8Array.of(255,0,0,0,0)});
    expect([...erased.pixels.slice(0,8)]).toEqual([0,0,0,0,255,0,0,128]);
  });
  test("completed operation progress is monotonic and ends at total", async () => {
    const progress:number[] = [];
    await editImage(image(),{kind:"invert"},{chunkPixels:1,onProgress:p => progress.push(p.completed)});
    expect(progress).toEqual([1,2,3,4]);
  });
});
