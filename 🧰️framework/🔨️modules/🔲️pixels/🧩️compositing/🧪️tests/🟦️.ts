/** 🧪️ Shared isolated-layer vectors checked against librsvg and libvips via Sharp. */
import {expect,test,spyOn} from "bun:test";
import Ajv from "ajv";
import sharp from "sharp";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {CompositeJob,compositeImage,type CompositeInput,type CompositeLayer} from "../🟦️.ts";

const validate=new Ajv({strict:false}).compile(schema);
function input(value:unknown):CompositeInput {
  const result=structuredClone(value) as CompositeInput;
  for(const image of Object.values(result.images))image.pixels=new Uint8Array(image.pixels);
  const masks=(layers:CompositeLayer[])=>{for(const layer of layers){if(layer.mask)layer.mask.coverage=new Uint8Array(layer.mask.coverage);if(layer.kind==="group")masks(layer.children);}};
  masks(result.layers);return result;
}
function complete(value:CompositeInput,budget=256) {
  const job=new CompositeJob(value);while(!job.advance(budget).done){}return job.result();
}
const svg=(body:string)=>Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1">${body}</svg>`);
function oracleTree(value:CompositeInput):Buffer {
  let id=0;
  const walk=(layers:CompositeLayer[]):string=>{
    let result="";
    for(const layer of layers){
      if(!layer.visible)continue;
      let body="";
      if(layer.kind==="pixels"){
        const p=value.images[layer.image]!.pixels;
        body=`<rect width="1" height="1" fill="rgb(${p[0]},${p[1]},${p[2]})" fill-opacity="${p[3]!/255}"/>`;
      }else if(layer.kind==="group")body=walk(layer.children);
      else {
        const key=`adjust${id++}`,slope=2**(layer.contrast*4),intercept=(layer.brightness-0.5)*slope+0.5;
        result=`<defs><filter id="${key}" color-interpolation-filters="sRGB"><feComponentTransfer>${["R","G","B"].map(c=>`<feFunc${c} type="linear" slope="${slope}" intercept="${intercept}"/>`).join("")}</feComponentTransfer></filter></defs><g filter="url(#${key})">${result}</g>`;continue;
      }
      if(layer.mask){const key=`mask${id++}`,coverage=layer.mask.invert?255-layer.mask.coverage[0]!:layer.mask.coverage[0]!;body=`<defs><mask id="${key}"><rect width="1" height="1" fill="rgb(${coverage},${coverage},${coverage})"/></mask></defs><g mask="url(#${key})">${body}</g>`;}
      result+=`<g opacity="${layer.opacity}">${body}</g>`;
    }
    return result;
  };
  return svg(walk(value.layers));
}
for(const row of fixture.cases)test(row.name,async()=>{
  expect(validate(row.input)).toBe(true);
  const source=input(row.input),before=structuredClone(source),result=complete(source);
  expect([...result.pixels]).toEqual(row.expected);
  expect(source).toEqual(before);
  const oracle=await sharp(oracleTree(source)).ensureAlpha().raw().toBuffer();
  for(let i=0;i<4;i++)expect(Math.abs(result.pixels[i]!-oracle[i]!)).toBeLessThanOrEqual(1);
});
for(const [index,mode] of fixture.blendModes.entries())test(`${mode} uses straight colors when blending translucent pixels`,async()=>{
  const source=input(fixture.blendInput);source.layers[1]!.blend=mode as CompositeLayer["blend"];
  const result=complete(source),css=mode.replace(/[A-Z]/g,c=>"-"+c.toLowerCase());
  const markup=svg(`<defs><filter id="blend" x="0" y="0" width="1" height="1" color-interpolation-filters="sRGB"><feFlood flood-color="rgb(60,120,180)" flood-opacity="0.8" result="back"/><feFlood flood-color="rgb(210,90,150)" flood-opacity="0.6" result="front"/><feBlend in="front" in2="back" mode="${css}"/></filter></defs><rect width="1" height="1" filter="url(#blend)"/>`);
  const oracle=await sharp(markup).ensureAlpha().raw().toBuffer();
  expect([...oracle]).toEqual(fixture.blendExpected[index]!);
  for(let c=0;c<4;c++)expect(Math.abs(result.pixels[c]!-oracle[c]!)).toBeLessThanOrEqual(fixture.oracleTolerance);
});
for(const row of fixture.spatialCases)test(row.name,async()=>{
  expect(validate(row.input)).toBe(true);
  const source=input(row.input),expected=row.runs.flatMap(run=>Array.from({length:run.count},()=>run.pixel).flat());
  for(const budget of [1,17,256,65536])expect([...complete(source,budget).pixels]).toEqual(expected);
  const group=source.layers[0]!;
  if(group.kind!=="group")throw new Error("Spatial fixture requires a group");
  const mask=group.mask;
  const body=mask?`<defs><mask id="coverage" maskUnits="userSpaceOnUse" x="0" y="0" width="513" height="1">${[...mask.coverage].map((v,i)=>`<rect x="${i*171}" width="171" height="1" fill="rgb(${v},${v},${v})"/>`).join("")}</mask></defs><g opacity="0.5" mask="url(#coverage)"><rect width="513" height="1" fill="red"/></g>`:`<g opacity="0.5"><rect width="4" height="1" fill="red"/><rect x="1" width="2" height="1" fill="rgb(128,0,0)"/></g>`;
  const oracle=await sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${source.width}" height="1">${body}</svg>`)).ensureAlpha().raw().toBuffer();
  for(let i=0;i<expected.length;i++)expect(Math.abs(expected[i]!-oracle[i]!)).toBeLessThanOrEqual(1);
});
test("compositor bounds each grant and never publishes a cancelled candidate",()=>{
  const source=input(fixture.cases[0]!.input);source.width=512;source.height=512;
  const job=new CompositeJob(source);const first=job.advance(17);
  expect(first.completed).toBeLessThanOrEqual(17);expect(first.done).toBe(false);expect(()=>job.result()).toThrow();
  job.cancel();expect(()=>job.advance()).toThrow();expect(()=>job.result()).toThrow();
});
test("compositor refuses invalid geometry, missing images and excessive depth",()=>{
  const source=input(fixture.cases[0]!.input);source.layers[0]!.transform=[0,0,0,0,0,0];expect(()=>new CompositeJob(source)).toThrow();
  const missing=input(fixture.blendInput);delete missing.images.front;expect(()=>new CompositeJob(missing)).toThrow();
  const nested=input(fixture.cases[0]!.input);for(let i=0;i<33;i++)nested.layers=[{...nested.layers[0]!,kind:"group",children:nested.layers}];expect(()=>new CompositeJob(nested)).toThrow();
});
test("async compositor exposes progress and observes cancellation",async()=>{
  const controller=new AbortController(),source=input(fixture.blendInput);source.width=64;source.height=64;
  await expect(compositeImage(source,{signal:controller.signal,chunkPixels:64,onProgress:()=>controller.abort()})).rejects.toThrow();
});


test("compositor retirement retains genuine private owners and preserves shared and published bytes",async()=>{
  const rows=(await import("../🧫️fixtures/🧹️retirement/🔣️.json")).default;
  
  const check=new Ajv({strict:true}).compile({$defs:schema.$defs,$ref:"#/$defs/Retirement"});
  for(const row of rows)for(const grant of [1,7,4096]){
    const sourceRow=[...fixture.cases,...fixture.spatialCases].find(v=>v.name===row.source)!,source=input(sourceRow.input),before=structuredClone(source),job=new CompositeJob(source),state=job as any;
    if(["complete","cancelledComplete","published"].includes(row.mode)){while(!job.advance(4096).done){}}else if(row.steps)job.advance(row.steps);
    const commands=[...state.commands],buffers=[...state.buffers],pixels=state.pixels,published=row.mode==="published"?job.result():null;
    if(row.mode.startsWith("cancelled")||published){job.cancel();expect(state.commands).toEqual(commands);expect(state.buffers).toEqual(buffers);expect(state.pixels).toBe(published?null:pixels);}
    const retired=(job as any).intoRetirement();expect(retired.output!==null).toBe(row.mode==="complete");if(retired.output)expect(retired.output.pixels).toBe(pixels);
    expect(()=>job.result()).toThrow(/cancel/);expect(()=>job.advance(1)).toThrow(/cancel/);expect(()=>(job as any).intoRetirement()).toThrow(/transferred/);
    for(const invalid of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>retired.job.advance(invalid)).toThrow(/grant/i);
    let work=0;for(let at=0;at<100000&&!retired.job.terminalIsEmpty();at++){const count=state.commands.length+state.buffers.length,p=retired.job.advance(grant);expect(check(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);expect(count-state.commands.length-state.buffers.length).toBeLessThanOrEqual(grant);work=p.work;}
    expect(retired.job.terminalIsEmpty()).toBe(true);expect(work).toBe(commands.length+buffers.length+4);expect(state.commands.length).toBe(0);expect(state.buffers.length).toBe(0);expect(state.pixels).toBeNull();expect(state.origin).toBeNull();expect(retired.job.advance(1)).toEqual({phase:"complete",work,done:true});expect(source).toEqual(before);
    const image=retired.output??published;if(image){const expected="expected" in sourceRow?sourceRow.expected:sourceRow.runs.flatMap(run=>Array.from({length:run.count},()=>run.pixel).flat());expect([...image.pixels]).toEqual(expected);if(source.width===1){const oracle=await sharp(oracleTree(source)).ensureAlpha().raw().toBuffer();for(let i=0;i<4;i++)expect(Math.abs(image.pixels[i]!-oracle[i]!)).toBeLessThanOrEqual(1);}}
    console.error(`[DEBUG] Actual compositor retirement ${row.name}: grant=${grant} work=${work} terminal_empty=true commands=${commands.length} buffers=${buffers.length}`);
  }
});
test("async composite finally consumes actual owners on success abort and callback failure",async()=>{
  const original=(CompositeJob.prototype as any).intoRetirement;
  for(const mode of ["success","abort","callback"]){const source=input(fixture.cases[0]!.input),before=structuredClone(source),controller=new AbortController();let adopted=0,work=0,closed:any=null;
    const spy=spyOn(CompositeJob.prototype as any,"intoRetirement").mockImplementation(function(this:CompositeJob){adopted++;closed=original.call(this);const advance=closed.job.advance.bind(closed.job);closed.job.advance=(grant:number)=>{expect(grant).toBe(1);const p=advance(grant);expect(p.work-work).toBe(1);work=p.work;return p;};return closed;});
    try{const pending=compositeImage(source,{signal:controller.signal,chunkPixels:1,onProgress:p=>{if(p.done){if(mode==="abort")controller.abort();if(mode==="callback")throw Error("composite completion callback failed");}}});if(mode==="success"){const image=await pending;expect(image.pixels).toBe(closed.output.pixels);expect([...image.pixels]).toEqual(fixture.cases[0]!.expected);}else{await expect(pending).rejects.toThrow(mode==="abort"?/cancel/:/completion callback failed/);expect(closed.output).toBeNull();}expect(adopted).toBe(1);expect(work).toBe(11);expect(closed.job.terminalIsEmpty()).toBe(true);expect(source).toEqual(before);console.error(`[DEBUG] Actual async composite ${mode}: one adoption and ${work} one-unit retirement grants`);}finally{spy.mockRestore();}
  }
});
