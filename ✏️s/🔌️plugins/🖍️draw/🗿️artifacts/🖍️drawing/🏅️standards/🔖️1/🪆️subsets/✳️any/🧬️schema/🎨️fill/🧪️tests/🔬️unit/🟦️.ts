/** 🎨️ Fill edits follow shared cases and an independent Immer document update. */
import { expect, test } from "bun:test";
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
import { PreparedFill } from "../../🎨️sampling/🟦️.ts";
import samplingCases from "../../🎨️sampling/🧫️fixtures/🔣️.json";
import samplingSchema from "../../🎨️sampling/🧬️schema/🔣️.json";

function referencePaint(fill:Fill,box:readonly number[]=[0,0,120,24]):string {
  const bounds=`x="${box[0]}" y="${box[1]}" width="${box[2]}" height="${box[3]}"`;
  const rgb=(color:number[])=>`rgb(${color.slice(0,3).map(value=>value*255).join(",")})`;
  if(fill.kind==="solid") return `<rect ${bounds} fill="${rgb(fill.color)}" fill-opacity="${fill.color[3]}"/>`;
  const tag=fill.kind==="linearGradient" ? "linearGradient" : "radialGradient";
  const coordinates=fill.kind==="linearGradient" ? `x1="${fill.x1}" y1="${fill.y1}" x2="${fill.x2}" y2="${fill.y2}"` : `cx="${fill.cx}" cy="${fill.cy}" r="${fill.r}"`;
  const stops=[...fill.stops].sort((a,b)=>a.offset-b.offset).map(stop=>`<stop offset="${stop.offset}" stop-color="${rgb(stop.color)}" stop-opacity="${stop.color[3]}"/>`).join("");
  return `<defs><${tag} id="paint" gradientUnits="userSpaceOnUse" ${coordinates}>${stops}</${tag}></defs><rect ${bounds} fill="url(#paint)"/>`;
}

test("prepared fill sampling matches shared cases and independent SVG pixels",async()=>{
  expect(new Ajv({strict:true}).compile(samplingSchema)(samplingCases)).toBe(true);
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
        const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="${x} ${y} 0.001 0.001">${referencePaint(fill,[x!,y!,0.001,0.001])}</svg>`;
        const pixel=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
        expect(Math.abs(pixel[3]!/255-color[3]),`${entry.name}: alpha ${pixel} vs ${color}`).toBeLessThanOrEqual(2/255);
        for(let channel=0;channel<3;channel++) expect(Math.abs(pixel[channel]!*pixel[3]!/255-color[channel]!*color[3]*255),`${entry.name}: rgb ${pixel} vs ${color}`).toBeLessThanOrEqual(2);
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
