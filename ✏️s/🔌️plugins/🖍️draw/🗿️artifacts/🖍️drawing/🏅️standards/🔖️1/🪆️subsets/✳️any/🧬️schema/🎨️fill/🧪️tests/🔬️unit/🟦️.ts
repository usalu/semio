/** 🎨️ Fill edits follow shared cases and an independent Immer document update. */
import { expect, test, spyOn } from "bun:test";
import Ajv from "ajv";
import { produce } from "immer";
import { Vector4 } from "three";
import { editFill, type Fill, type FillEdit } from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import cases from "../../🧫️fixtures/🔣️.json";
test("fill editing preserves stops, geometry, alpha and source on rejection", () => {
  const validate = new Ajv({strict:true}).compile(schema);
  for (const entry of cases) {
    expect(validate(entry.edit)).toBe(true);
    const source = structuredClone(entry.before) as Fill | null;
    if (entry.error) expect(() => editFill(source, entry.edit as FillEdit)).toThrow();
    else {
      const result = editFill(source, entry.edit as FillEdit);
      expect(result).toEqual(entry.after);
      if (entry.edit.kind === "addStop" && source && "stops" in source && result && "stops" in result) {
        const [left,right] = source.stops;
        const color = new Vector4(...left!.color).lerp(new Vector4(...right!.color), (entry.edit.offset!-left!.offset)/(right!.offset-left!.offset)).toArray();
        expect(result.stops.find(stop => stop.offset === entry.edit.offset)!.color).toEqual(color);
      }
      if (["alpha","offset","removeStop"].includes(entry.edit.kind)) {
        const oracle = produce({fill:source}, draft => {
          const fill=draft.fill!;
          if(entry.edit.kind==="alpha") {
            const color=fill.kind==="solid" ? fill.color : fill.stops[entry.edit.index!]!.color;
            color[3]=entry.edit.value as number;
          } else if("stops" in fill && entry.edit.kind==="offset") {
            fill.stops[entry.edit.index!]!.offset=entry.edit.value as number;
            fill.stops.sort((a,b)=>a.offset-b.offset);
          } else if("stops" in fill) fill.stops.splice(entry.edit.index!,1);
        });
        expect(result).toEqual(oracle.fill);
      }
    }
    expect(source).toEqual(entry.before);
  }
});

import sharp from "sharp";
import { PreparedFill, GradientRamp } from "../../🎨️sampling/🟦️.ts";
import retirementCases from "../../🎨️sampling/🧫️fixtures/🧹️retirement/🔣️.json";
import retirementSchema from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧹️retire/🧬️schema/🔣️.json";
import samplingCases from "../../🎨️sampling/🧫️fixtures/🔣️.json";

/** 🧭️ SVG 1.1 degenerate gradients use their last stop; normalize these before the librsvg oracle. */
function referencePaint(fill:Fill,box:readonly number[]=[0,0,120,24]):string {
  if(fill.kind!=="solid" && (fill.kind==="linearGradient" ? fill.x1===fill.x2&&fill.y1===fill.y2 : fill.r===0)) {
    const last=[...fill.stops].sort((a,b)=>a.offset-b.offset).at(-1);
    return referencePaint({kind:"solid",color:last?.color??[0,0,0,0]},box);
  }
  const bounds=`x="${box[0]}" y="${box[1]}" width="${box[2]}" height="${box[3]}"`;
  const rgb=(color:number[])=>`rgb(${color.slice(0,3).map(value=>value*255).join(",")})`;
  if(fill.kind==="solid") return `<rect ${bounds} fill="${rgb(fill.color)}" fill-opacity="${fill.color[3]}"/>`;
  const tag=fill.kind==="linearGradient" ? "linearGradient" : "radialGradient";
  const coordinates=fill.kind==="linearGradient" ? `x1="${fill.x1}" y1="${fill.y1}" x2="${fill.x2}" y2="${fill.y2}"` : `cx="${fill.cx}" cy="${fill.cy}" r="${fill.r}"`;
  const stops=[...fill.stops].sort((a,b)=>a.offset-b.offset).map(stop=>`<stop offset="${stop.offset}" stop-color="${rgb(stop.color)}" stop-opacity="${stop.color[3]}"/>`).join("");
  return `<defs><${tag} id="paint" gradientUnits="userSpaceOnUse" ${coordinates}>${stops}</${tag}></defs><rect ${bounds} fill="url(#paint)"/>`;
}

test("prepared fill sampling matches shared cases and independent SVG pixels",async()=>{
  let compared=0;
  for(const entry of samplingCases) {
    const fill=structuredClone(entry.fill) as Fill;
    if(entry.error) expect(()=>new PreparedFill(fill)).toThrow();
    else {
      const paint=new PreparedFill(fill);
      for(const sample of entry.samples!) {
        const color=paint.sample(sample.point as [number,number]);
        for(let channel=0;channel<4;channel++) expect(color[channel]).toBeCloseTo(sample.color[channel]!,12);
        const [x,y]=sample.point;
        const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="${x} ${y} 1 1">${referencePaint(fill,[x!,y!,1,1])}</svg>`;
        const pixel=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
        const expected=paint.sample([x!+0.5,y!+0.5]);
        expect(Math.abs(pixel[3]!/255-expected[3]),`${entry.name}: alpha ${pixel} vs ${color}`).toBeLessThanOrEqual(2/255);
        for(let channel=0;channel<3;channel++) expect(Math.abs(pixel[channel]!*pixel[3]!/255-expected[channel]!*expected[3]*255),`${entry.name}: rgb ${pixel} vs ${color}`).toBeLessThanOrEqual(2);
        compared++;
      }
    }
    expect(fill).toEqual(entry.fill);
  }
});

test("prepared fills own source stops and reject nonfinite paint or sample coordinates",()=>{
  const fill:Fill={kind:"linearGradient",x1:0,y1:0,x2:100,y2:0,stops:[{offset:0,color:[1,0,0,1]},{offset:1,color:[0,0,1,1]}]};
  const paint=new PreparedFill(fill);
  fill.stops[0]!.color[0]=0;
  expect(paint.sample([0,0])).toEqual([1,0,0,1]);
  for(const invalid of [NaN,Infinity,-Infinity]) {
    expect(()=>paint.sample([invalid,0])).toThrow();
    expect(()=>new PreparedFill({...fill,x1:invalid})).toThrow();
    expect(()=>new PreparedFill({kind:"solid",color:[0,0,0,invalid]})).toThrow();
  }
});


test("inserting a gradient stop preserves independently rasterized appearance",async()=>{
  let compared=0;
  for(const entry of samplingCases.filter(entry=>!entry.error&&entry.fill.kind!=="solid"&&entry.fill.stops?.length)) {
    const source=entry.fill as Fill;
    const render=async(fill:Fill)=>sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="120" height="24" viewBox="0 0 120 24">${referencePaint(fill)}</svg>`)).ensureAlpha().raw().toBuffer();
    const before=await render(source);
    expect(before.some((value,index)=>index%4===3&&value>0),entry.name).toBe(true);
    for(const offset of [0.25,0.5,0.75]) {
      const result=editFill(source,{kind:"addStop",offset})!;
      const after=await render(result);
      let maximumError=0;
      for(let pixel=0;pixel<before.length;pixel+=4) {
        maximumError=Math.max(maximumError,Math.abs(before[pixel+3]!-after[pixel+3]!));
        for(let channel=0;channel<3;channel++) maximumError=Math.max(maximumError,Math.abs(before[pixel+channel]!*before[pixel+3]!/255-after[pixel+channel]!*after[pixel+3]!/255));
      }
      expect(maximumError,`${entry.name}: inserted stop ${offset}`).toBeLessThanOrEqual(2);
      compared++;
    }
  }
});

test("prepared paint retirement drains actual copied stop owners and composes the real ramp",async()=>{
 const validProgress=new Ajv({strict:true}).compile(retirementSchema),pop=Array.prototype.pop;let comparisons=0;
 for(const row of retirementCases)for(const grant of [1,7,4096]){
  const source=samplingCases.find(sample=>sample.name===row.source)!,fill=structuredClone(source.fill) as Fill;
  if(row.repeat&&fill.kind!=="solid")fill.stops=Array.from({length:row.repeat},()=>structuredClone(fill.stops[0]!));const before=structuredClone(fill),paint=new PreparedFill(fill),samples=source.samples!.map(sample=>paint.sample(sample.point as [number,number])),children:{owner:ReturnType<GradientRamp["intoRetirement"]>;work:number}[]=[],rampOriginal=GradientRamp.prototype.intoRetirement;
  const adopt=spyOn(GradientRamp.prototype,"intoRetirement").mockImplementation(function(this:GradientRamp){const owner=rampOriginal.call(this),record={owner,work:0};children.push(record);const advance=owner.advance.bind(owner);owner.advance=unit=>{expect(unit).toBe(1);const p=advance(unit);expect(p.work-record.work).toBe(1);record.work=p.work;return p;};return owner;});
  let retired:ReturnType<PreparedFill["intoRetirement"]>;try{retired=paint.intoRetirement();}finally{adopt.mockRestore();}
  expect(()=>paint.sample([0,0])).toThrow(/transferred/);expect(()=>paint.constantColor()).toThrow(/transferred/);expect(()=>paint.intoRetirement()).toThrow(/transferred/);
  for(const bad of [0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>retired.advance(bad)).toThrow(/grant/);
  const popped:unknown[]=[];let work=0;
  const spy=spyOn(Array.prototype,"pop").mockImplementation(function(this:unknown[]){const value=pop.call(this);if(value&&typeof value==="object"&&"offset" in value&&"color" in value)popped.push(value);return value;});
  try{for(let at=0;at<=row.work.prepared.typescript;at++){const count=popped.length,p=retired.advance(grant);expect(validProgress(p)).toBe(true);expect(p.work-work).toBeGreaterThan(0);expect(p.work-work).toBeLessThanOrEqual(grant);expect(popped.length-count).toBeLessThanOrEqual(grant);work=p.work;if(p.done)break;}}finally{spy.mockRestore();}
  expect(work).toBe(row.work.prepared.typescript);expect(retired.terminalIsEmpty()).toBe(true);expect(retired.advance(1)).toEqual({phase:"complete",work,done:true});expect(fill).toEqual(before);expect(popped).toHaveLength(fill.kind==="solid"?0:fill.stops.length);expect(children).toHaveLength(fill.kind==="solid"?0:1);for(const child of children){expect(child.work).toBe(row.work.ramp.typescript);expect(child.owner.terminalIsEmpty()).toBe(true);}
  if(fill.kind!=="solid")for(const stop of popped as {offset:number;color:number[]}[]){expect(fill.stops.includes(stop as any)).toBe(false);expect(fill.stops.some(source=>source.color===stop.color)).toBe(false);}
  source.samples!.forEach((sample,at)=>sample.color.forEach((value,c)=>expect(samples[at]![c]).toBeCloseTo(value,12)));
  const sample=source.samples![0]!,[x,y]=sample.point,pixel=await sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="${x} ${y} 1 1">${referencePaint(fill,[x!,y!,1,1])}</svg>`)).ensureAlpha().raw().toBuffer(),expected=new PreparedFill(fill).sample([x!+.5,y!+.5]);expect(Math.abs(pixel[3]!/255-expected[3])).toBeLessThanOrEqual(2/255);for(let c=0;c<3;c++)expect(Math.abs(pixel[c]!*pixel[3]!/255-expected[c]!*expected[3]*255)).toBeLessThanOrEqual(2);comparisons++;
  if(fill.kind!=="solid"){const ramp=new GradientRamp(fill.stops),color=ramp.sample(.5),owner=ramp.intoRetirement();expect(()=>ramp.sample(.5)).toThrow(/transferred/);expect(()=>ramp.constantColor()).toThrow(/transferred/);expect(()=>ramp.intoRetirement()).toThrow(/transferred/);let work=0;for(let at=0;at<=row.work.ramp.typescript;at++){const p=owner.advance(grant);expect(validProgress(p)).toBe(true);expect(p.work-work).toBeLessThanOrEqual(grant);work=p.work;if(p.done)break;}expect(work).toBe(row.work.ramp.typescript);expect(owner.terminalIsEmpty()).toBe(true);expect(color).toEqual(new GradientRamp(fill.stops).sample(.5));}
 }
 process.stderr.write(`[DEBUG] Actual paint retirement: ${comparisons} neutral/grant SVG comparisons; copied stop owners drained through genuine ramp composition, including 4096 stops\n`);
});
