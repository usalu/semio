/** 🧪️ Language-neutral pixel editing cases and independent libvips image oracles. */
import { describe, expect, test } from "bun:test";
import sharp from "sharp";
import Ajv from "ajv";
import fixtures from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import { editImage, selectionMask, selectPixels, combineSelections, SelectionCombineJob, type SelectionMerge, floodSelection, paintStroke, PixelEditJob, PixelSelectionJob, strokeBounds, validateImage, type PixelOperation, type PixelPoint, type SelectionShape } from "../🟦️.ts";

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
    if("width" in fixture && fixture.width !== undefined && fixture.height !== undefined) expect([result.width,result.height]).toEqual([fixture.width,fixture.height]);
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
  test("selection merge operations preserve coverage", async () => {
    const a = Uint8Array.of(255,0,128), b = Uint8Array.of(0,255,255);
    expect([...await combineSelections(a,b,"add")]).toEqual([255,255,255]);
    expect([...await combineSelections(a,b,"subtract")]).toEqual([255,0,0]);
    expect([...await combineSelections(a,b,"intersect")]).toEqual([0,0,128]);
    expect([...await combineSelections(a,b,"replace")]).toEqual([...b]);
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

for (const fixture of fixtures.cases.filter(row=>row.operation.kind==="alphaStroke")) test(`${fixture.name}: retained job and libvips oracle`,async()=>{
  const source=fixture.image!;
  const selection="selection" in fixture?Uint8Array.from(fixture.selection!):undefined;
  const job=new PixelEditJob({...source,pixels:Uint8Array.from(source.pixels)},fixture.operation as PixelOperation,selection);
  while(!job.advance(1).done)expect(()=>job.result()).toThrow();
  expect([...job.result().pixels]).toEqual(fixture.expected);
  const operation=fixture.operation as {alpha:number;opacity:number};
  const coverage=(fixture as {brushCoverage:number[]}).brushCoverage;
  for(let index=0;index<coverage.length;index++) {
    const amount=coverage[index]!/255*(selection?.[index]??255)/255*operation.opacity;
    const alpha=source.pixels[index*4+3]!;
    const oracle=await sharp(Uint8Array.of(alpha,alpha,alpha),{raw:{width:1,height:1,channels:3}}).linear(1-amount,operation.alpha*amount+0.5).extractChannel(0).raw().toBuffer();
    expect(fixture.expected[index*4+3]).toBe(oracle[0]);
  }
});

test("alpha strokes validate input and cancel without publishing",async()=>{
  const operation={kind:"alphaStroke",points:[[0.5,0.5]],size:2,opacity:1,hardness:1,alpha:255} as const;
  for(const alpha of [-1,256,0.5,NaN])await expect(editImage(image(),{...operation,alpha} as PixelOperation)).rejects.toThrow();
  const controller=new AbortController();
  await expect(editImage(image(),operation as PixelOperation,{signal:controller.signal,onProgress:()=>controller.abort()})).rejects.toThrow();
  const job=new PixelEditJob(image(),operation as PixelOperation);
  job.advance(1);job.cancel();
  expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();
});

test("output progress grants preserve the independent image result",async()=>{
  const f=fixtures.outputGrants,total=f.width*f.height;
  const pixels=Uint8Array.from({length:total*4},(_,i)=>f.sourcePixel[i%4]!);
  const job=new PixelEditJob({width:f.width,height:f.height,pixels},f.operation as PixelOperation);
  expect(()=>job.result()).toThrow();
  for(let i=0;i<f.grants.length;i++){
    const completed=f.completed[i]!;
    expect(job.advance(f.grants[i])).toEqual({completed,total,done:completed===total});
    if(completed<total)expect(()=>job.result()).toThrow();
  }
  const oracle=await sharp(pixels,{raw:{width:f.width,height:f.height,channels:4}}).negate({alpha:false}).raw().toBuffer();
  expect(job.result().pixels).toEqual(Uint8Array.from(oracle));
  expect(job.result().pixels).toEqual(Uint8Array.from({length:total*4},(_,i)=>f.expectedPixel[i%4]!));
});

 test("alpha fill agrees with libvips interpolation and cancels privately",async()=>{
  for(const fixture of fixtures.cases.filter(value=>value.operation.kind==="alphaFill")) {
    const source=fixture.image!,operation=fixture.operation as unknown as {kind:"alphaFill";alpha:number;opacity:number};
    const result=await editImage({...source,pixels:Uint8Array.from(source.pixels)},operation,{selection:"selection" in fixture?Uint8Array.from(fixture.selection!):undefined});
    for(let i=0;i<source.width*source.height;i++) {
      const coverage=("selection" in fixture?fixture.selection![i]!:255)/255*operation.opacity,alpha=source.pixels[i*4+3]!;
      const oracle=await sharp(Uint8Array.of(alpha,alpha,alpha),{raw:{width:1,height:1,channels:3}}).linear(1-coverage,operation.alpha*coverage+0.5).extractChannel(0).raw().toBuffer();
      expect(result.pixels[i*4+3]).toBe(oracle[0]);
    }
  }
  for(const alpha of [-1,256,1.5,NaN]) await expect(editImage(image(),{kind:"alphaFill",alpha,opacity:1})).rejects.toThrow();
  for(const opacity of [-1,2,NaN]) await expect(editImage(image(),{kind:"alphaFill",alpha:128,opacity})).rejects.toThrow();
  const controller=new AbortController();
  await expect(editImage(image(),{kind:"alphaFill",alpha:0,opacity:1},{signal:controller.signal,chunkPixels:1,onProgress:()=>controller.abort()})).rejects.toThrow();
});

for(const row of fixtures.selectionCombinations) test(row.name+" bounded combination and libvips oracle",async()=>{
  const validate=new Ajv().addSchema(schema).compile({$ref:schema.$id+"#/$defs/SelectionCombination"});
  const {name,expected,...input}=row;expect(validate(input)).toBe(true);
  const current="current" in row?Uint8Array.from(row.current!):undefined,next=Uint8Array.from(row.next);
  const job=new SelectionCombineJob(current,next,row.mode as SelectionMerge);
  expect(()=>job.result()).toThrow();expect(job.advance(0).completed).toBe(0);
  while(!job.advance(1).done) {}
  expect([...job.result()]).toEqual(expected);
  expect([...await combineSelections(current,next,row.mode as SelectionMerge,{chunkPixels:1})]).toEqual(expected);
  expect([...next]).toEqual(row.next);if(current)expect([...current]).toEqual(row.current!);
  const oracle:number[]=[];
  for(let i=0;i<next.length;i++) {
    const a=current?.[i]??0,b=next[i]!;
    let pipeline=sharp(Buffer.from([row.mode==="replace"?b:a]),{raw:{width:1,height:1,channels:1}});
    if(row.mode==="subtract") pipeline=pipeline.linear(1,-b);
    if(row.mode==="add"||row.mode==="intersect") pipeline=pipeline.composite([{input:Buffer.from([b,b,b,255]),raw:{width:1,height:1,channels:4},blend:row.mode==="add"?"lighten":"darken"}]);
    oracle.push((await pipeline.raw().toBuffer())[0]!);
  }
  expect([...job.result()]).toEqual(oracle);
});
test("selection combination bounds work and cancellation never publishes",async()=>{
  const row=fixtures.selectionCombinationWork,next=new Uint8Array(row.length).fill(128),current=new Uint8Array(row.length).fill(200),progress:number[]=[];
  const combined=await combineSelections(current,next,"subtract",{chunkPixels:Number.MAX_SAFE_INTEGER,onProgress:p=>progress.push(p.completed)});
  expect(progress).toEqual(row.completed);expect(combined.every(v=>v===72)).toBe(true);
  for(const final of [false,true]) {
    const controller=new AbortController();
    await expect(combineSelections(current,next,"add",{signal:controller.signal,onProgress:p=>{if(p.done===final)controller.abort();}})).rejects.toThrow();
  }
  const controller=new AbortController();
  await expect(combineSelections(current,next,"add",{signal:controller.signal,onProgress:p=>{if(p.completed===row.grant)setTimeout(()=>controller.abort(),0);}})).rejects.toThrow();
  const job=new SelectionCombineJob(current,next,"add");job.advance(1);job.cancel();expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();
  expect(()=>new SelectionCombineJob(undefined,new Uint8Array(),"add")).toThrow();
  expect(()=>new SelectionCombineJob(Uint8Array.of(1),next,"add")).toThrow();
  expect(()=>new SelectionCombineJob(undefined,next,"unknown" as SelectionMerge)).toThrow();
  expect(()=>new SelectionCombineJob(undefined,new Uint8Array(16777217),"add")).toThrow();
  expect(()=>new SelectionCombineJob(undefined,next,"add").advance(-1)).toThrow();
  expect(current.every(v=>v===200)).toBe(true);expect(next.every(v=>v===128)).toBe(true);
});

test("selection row grants match the shared contract and SVG oracle",async()=>{
  const f=fixtures.selectionRasterization;
  const validate=new Ajv().addSchema(schema).compile<{width:number;height:number;shape:SelectionShape}>({$ref:schema.$id+"#/$defs/SelectionRasterization"});
  const input={width:f.width,height:f.height,shape:f.shape};
  if(!validate(input))throw new Error(JSON.stringify(validate.errors));
  const job=new PixelSelectionJob(input.width,input.height,input.shape);expect(()=>job.result()).toThrow();
  for(let i=0;i<f.grants.length;i++){const progress=job.advance(f.grants[i]);expect(progress).toEqual({completed:f.completed[i],total:f.height,done:f.completed[i]===f.height});if(!progress.done)expect(()=>job.result()).toThrow();}
  const oracle=await sharp(Buffer.from(f.svg)).ensureAlpha().extractChannel(3).raw().toBuffer();expect([...oracle]).toEqual(f.expected);expect([...job.result()]).toEqual([...oracle]);
  for(const rows of f.cancelAfterRows){const job=new PixelSelectionJob(input.width,input.height,input.shape);if(rows)job.advance(rows);job.cancel();expect(()=>job.result()).toThrow();expect(()=>job.advance(1)).toThrow();}
  for(const rows of f.invalidGrants){const job=new PixelSelectionJob(input.width,input.height,input.shape);expect(()=>job.advance(rows)).toThrow();expect(job.advance(1).completed).toBe(1);}
});

test("stroke bounds match the shared contract", () => {
  for (const row of fixtures.strokeBounds) {
    const points=row.points.map((point):PixelPoint=>{const [x,y]=point;if(point.length!==2||typeof x!=="number"||typeof y!=="number")throw Error("Invalid pixel fixture point");return [x,y];});
    expect(strokeBounds(points, row.size, row.width, row.height)).toEqual(row.expected);
  }
});

test("flood selections match the shared contract (the Python BFS oracle that wrote it, and the Rust twin)", async () => {
  for (const row of fixtures.floodSelections) {
    const input = { ...row.image, pixels: Uint8Array.from(row.image.pixels) };
    const selection = "selection" in row && row.selection ? Uint8Array.from(row.selection) : undefined;
    expect([...await floodSelection(input, row.seed[0]!, row.seed[1]!, row.tolerance, { selection })], row.name).toEqual(row.expected);
  }
});
